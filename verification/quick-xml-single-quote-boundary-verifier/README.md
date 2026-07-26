# Bounded quick-xml single-quote boundary verifier

This portable experiment is fixed to `tafia/quick-xml` release `0.41.0` at
commit `4deda08abeffdc188c269360229cf47e12a77a9f` (tree
`8a2bd1c856a33135fd9738e632a27c4d9b9af6ef`). It changes no production source
and makes no defect, full-parser, or whole-document claim.

## Boundary

The experiment covers the ordinary slice-reader `InsideMarkup -> InsideText`
transition reached through public `Reader::read_event`. It is limited to the
generic start-tag branch with one single-quoted attribute whose value has at
most six bytes and may contain literal `>` bytes. It checks the returned
`Event::Start`, exact reader position, and the two scoped EOF outcomes.

The production link at the fixed base is:

- `src/parser/element.rs`, blob `db99111f500b04177ef8dfdbb3ca8b0ca0c67832`
- `src/reader/mod.rs`, blob `c359c5890870c324953385ddc8f5e0bd0e14f553`
- `src/reader/slice_reader.rs`, blob `ffc8708e26b30890ac07ffe338d40fdfcbf4bbd9`
- `src/reader/state.rs`, blob `a0ee3b963394bd6f0ebd8a2e3f8904a72f9ee8d2`

`SCOPE.md` preserves the reviewed boundary and exclusions byte-for-byte.

## Published-source custody limitation

The released `quick-xml` 0.41.0 archive has SHA-256
`e660451e55124f798a69a5af3f49ccfbefbd41910eefd25caf2393e1f3473ec1`, and its
`.cargo_vcs_info.json` names the fixed commit. The four scoped Rust members in
that package use CRLF line endings, while the corresponding Git-tree blobs use
LF. Their raw hashes therefore differ. Removing only the terminal carriage
return from each package line made each scoped member byte-identical to its
Git-tree file in the frozen reviewed custody check.

This commit binds execution directly to the Git tree and records that
normalization bridge; it does not claim raw-byte equality between the package
and Git representations. The package archive is not duplicated in this
portable subtree, so reproduction of that archive-specific custody check
requires the checksum-pinned first-party package separately.

## Preserved reviewed content

The scope, proof, design note, finite production harness, Cargo manifest,
lockfile, and harness note are byte-for-byte copies of the frozen reviewed
packet. `reviewed-source.sha256` records their identities.

The harness intentionally retains its reviewed `../checkout` layout and
checkout guard. `check.sh` creates that layout only in temporary storage by
placing the unchanged harness beside a clean detached local clone of this
commit's parent. This makes the path portable without changing the reviewed
Rust or Cargo content.

## Bounded result

The reviewed finite production search found no mismatch in 55,987 complete
payloads, including 36,456 with at least one literal `>` inside the
single-quoted value, or in the corresponding 55,987 missing-quote and 55,987
missing-close cases.

The executable Verus model verified the standalone significant-byte transition
for arbitrary in-scope model inputs: `11 verified, 0 errors` with
`--no-cheating`; its compiled witness exited zero. This universal model result
has only a finite differential binding to the exact public `ElementParser` and
ordinary slice `Reader::read_event` path. It is not a formal proof of `memchr`,
the reader implementation, other syntax or events, validation, or whole
documents.

## Reproduce

Use x86-64 Linux and the exact Verus release
[`0.2025.11.30.840fa61`](https://github.com/verus-lang/verus/releases/tag/release/0.2025.11.30.840fa61).
The official `verus-0.2025.11.30.840fa61-x86-linux.zip` asset has SHA-256
`c08b8959a98b15519a07f1eb813db8c34a6311606e4ff7dda7e8ae135b5cfcb2`;
its `verus` executable has SHA-256
`59659856b1cc28264a8d21dc11707feb435f801ff386949d60c7ab5a995e7365`.

Run from a clean checkout of this exact commit:

```console
./verification/quick-xml-single-quote-boundary-verifier/check.sh /path/to/verus
```

The checker validates ancestry and production blobs, stages the unchanged
reviewed harness in temporary storage, repeats the finite production search,
runs formatting, tests, and warnings-denied no-dependency Clippy, verifies the
proof with `--no-cheating`, and executes the compiled proof witness. Build,
verifier, and temporary Git state remain outside the checkout.

This is a local candidate requiring a fresh exact-commit review before any
publication. It does not authorize a push, contact, issue, or pull request.
