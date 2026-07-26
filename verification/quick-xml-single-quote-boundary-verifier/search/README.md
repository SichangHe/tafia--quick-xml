# Pre-edit counterexample search

This executable checks the selected transition before any proof or production-source change.

- production scanner: public `ElementParser::feed`
- ordinary bridge: public slice `Reader::from_str(...).read_event()`
- domain: every `<a x='PAYLOAD'>T` with payload length `0..=6` over `a`, space, `>`, `"`, `/`, `=`
- assertions: direct delimiter index, `Event::Start` raw bytes, and exact `buffer_position`
- EOF partition: the corresponding missing single quote yields `UnclosedSingleQuotedAttributeValue`; a matching quote without `>` yields `UnclosedTag`
- checkout gate: exact detached commit and tree, clean status, and disabled push URL

Run with `cargo run --release --locked`. A clean result prints `status=no-counterexample` and exact case totals.

This is exhaustive only for the finite template domain. It does not prove arbitrary tag syntax, double-quoted attributes, other events, other readers, validation, or whole-document correctness.
