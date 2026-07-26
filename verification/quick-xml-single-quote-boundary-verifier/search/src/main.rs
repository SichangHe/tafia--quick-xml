use quick_xml::errors::SyntaxError;
use quick_xml::events::Event;
use quick_xml::parser::{ElementParser, Parser};
use quick_xml::{Error, Reader};
use std::path::Path;
use std::process::{Command, ExitCode};

const ADMITTED_COMMIT: &str = "4deda08abeffdc188c269360229cf47e12a77a9f";
const ADMITTED_TREE: &str = "8a2bd1c856a33135fd9738e632a27c4d9b9af6ef";
const DISABLED_PUSH_URL: &str = "disabled://quick-xml-local-only";
const ALPHABET: &[u8] = b"a >\"/=";
const MAX_PAYLOAD_LEN: usize = 6;

#[derive(Default)]
struct Summary {
    complete: u64,
    embedded_gt: u64,
    unclosed_single_quote: u64,
    unclosed_tag: u64,
}

fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;
    if !output.status.success() {
        return Err(format!("git {args:?} failed with {}", output.status));
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|error| format!("git output was not UTF-8: {error}"))
}

fn check_checkout() -> Result<(), String> {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../checkout");
    if git(&repo, &["rev-parse", "HEAD"])? != ADMITTED_COMMIT {
        return Err("production checkout does not match the admitted commit".into());
    }
    if git(&repo, &["rev-parse", "HEAD^{tree}"])? != ADMITTED_TREE {
        return Err("production checkout does not match the admitted tree".into());
    }
    if git(&repo, &["rev-parse", "--abbrev-ref", "HEAD"])? != "HEAD" {
        return Err("production checkout is not detached".into());
    }
    if !git(&repo, &["status", "--porcelain=v1"])?.is_empty() {
        return Err("production checkout is not clean".into());
    }
    let push_url = git(&repo, &["remote", "get-url", "--push", "origin"])?;
    if push_url != DISABLED_PUSH_URL {
        return Err(format!("unexpected push URL: {push_url}"));
    }
    Ok(())
}

fn payload(mut ordinal: usize, len: usize) -> Vec<u8> {
    let mut value = vec![ALPHABET[0]; len];
    for byte in &mut value {
        *byte = ALPHABET[ordinal % ALPHABET.len()];
        ordinal /= ALPHABET.len();
    }
    value
}

fn complete_input(payload: &[u8]) -> Vec<u8> {
    let mut input = Vec::with_capacity(payload.len() + 9);
    input.extend_from_slice(b"<a x='");
    input.extend_from_slice(payload);
    input.extend_from_slice(b"'>T");
    input
}

fn input_str(input: &[u8]) -> Result<&str, String> {
    std::str::from_utf8(input).map_err(|error| format!("template was not UTF-8: {error}"))
}

fn check_complete(payload: &[u8]) -> Result<(), String> {
    let input = complete_input(payload);
    let boundary = 7 + payload.len();

    let mut parser = ElementParser::Outside;
    let direct = parser.feed(&input[1..]);
    if direct != Some(boundary - 1) {
        return Err(format!(
            "direct boundary mismatch for payload {payload:?}: expected {}, observed {direct:?}, state {parser:?}",
            boundary - 1
        ));
    }

    let mut reader = Reader::from_str(input_str(&input)?);
    match reader.read_event() {
        Ok(Event::Start(start))
            if *start == input[1..boundary]
                && reader.buffer_position() == (boundary + 1) as u64 =>
        {
            Ok(())
        }
        Ok(event) => Err(format!(
            "reader boundary mismatch for payload {payload:?}: event {event:?}, position {}",
            reader.buffer_position()
        )),
        Err(error) => Err(format!(
            "reader rejected complete payload {payload:?}: {error:?}, position {}",
            reader.buffer_position()
        )),
    }
}

fn check_unclosed_single_quote(payload: &[u8]) -> Result<(), String> {
    let mut input = Vec::with_capacity(payload.len() + 6);
    input.extend_from_slice(b"<a x='");
    input.extend_from_slice(payload);

    let mut parser = ElementParser::Outside;
    if parser.feed(&input[1..]).is_some() || parser != ElementParser::SingleQ {
        return Err(format!(
            "direct parser did not preserve SingleQ at EOF for payload {payload:?}: {parser:?}"
        ));
    }

    let mut reader = Reader::from_str(input_str(&input)?);
    match reader.read_event() {
        Err(Error::Syntax(SyntaxError::UnclosedSingleQuotedAttributeValue))
            if reader.buffer_position() == input.len() as u64
                && reader.error_position() == 0 =>
        {
            Ok(())
        }
        result => Err(format!(
            "unclosed-single-quote mismatch for payload {payload:?}: {result:?}, position {}, error position {}",
            reader.buffer_position(),
            reader.error_position()
        )),
    }
}

fn check_unclosed_tag(payload: &[u8]) -> Result<(), String> {
    let mut input = Vec::with_capacity(payload.len() + 7);
    input.extend_from_slice(b"<a x='");
    input.extend_from_slice(payload);
    input.push(b'\'');

    let mut parser = ElementParser::Outside;
    if parser.feed(&input[1..]).is_some() || parser != ElementParser::Outside {
        return Err(format!(
            "direct parser did not return Outside at EOF for payload {payload:?}: {parser:?}"
        ));
    }

    let mut reader = Reader::from_str(input_str(&input)?);
    match reader.read_event() {
        Err(Error::Syntax(SyntaxError::UnclosedTag))
            if reader.buffer_position() == input.len() as u64
                && reader.error_position() == 0 =>
        {
            Ok(())
        }
        result => Err(format!(
            "unclosed-tag mismatch for payload {payload:?}: {result:?}, position {}, error position {}",
            reader.buffer_position(),
            reader.error_position()
        )),
    }
}

fn run_search() -> Result<Summary, String> {
    check_checkout()?;
    let mut summary = Summary::default();
    for len in 0..=MAX_PAYLOAD_LEN {
        let n_payloads = ALPHABET.len().pow(len as u32);
        for ordinal in 0..n_payloads {
            let payload = payload(ordinal, len);
            check_complete(&payload)?;
            check_unclosed_single_quote(&payload)?;
            check_unclosed_tag(&payload)?;
            summary.complete += 1;
            summary.unclosed_single_quote += 1;
            summary.unclosed_tag += 1;
            if payload.contains(&b'>') {
                summary.embedded_gt += 1;
            }
        }
    }
    Ok(summary)
}

fn main() -> ExitCode {
    match run_search() {
        Ok(summary) => {
            println!("status=no-counterexample");
            println!("complete_cases={}", summary.complete);
            println!("embedded_gt_cases={}", summary.embedded_gt);
            println!(
                "unclosed_single_quote_cases={}",
                summary.unclosed_single_quote
            );
            println!("unclosed_tag_cases={}", summary.unclosed_tag);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("status=counterexample-or-harness-error");
            eprintln!("detail={error}");
            ExitCode::FAILURE
        }
    }
}
