# Scope

Status: one transition selected after bounded source inspection and before the pre-edit search.

## Included

- canonical `tafia/quick-xml` release `0.41.0` at tag commit `4deda08abeffdc188c269360229cf47e12a77a9f`
- the ordinary slice-reader `InsideMarkup -> InsideText` transition reached through public `Reader::read_event`
- its generic start-tag branch with `ElementParser::Outside`, limited to one single-quoted attribute whose value may contain literal `>` bytes
- the returned `Event::Start` bytes and `Reader::buffer_position()` immediately after the first `>` observed outside that attribute
- a standalone executable Verus transition model plus focused host checks against the pinned production path

## Excluded

- buffered and asynchronous readers
- double-quoted attributes, empty-element recognition, end tags, XML declarations, processing instructions, comments, CDATA, doctypes, and text-event scanning
- namespace resolution, entity expansion or unescaping, encodings other than the default byte path, DTD semantics, validation, and whole-document well-formedness
- serialization, downstream application behavior, I/O scheduling, performance, security, and end-to-end claims
- unrelated source changes, publication, contact, forks, commits, pushes, issues, pull requests, reviewers, and evaluators

The bounded bridge domain is `<a x='PAYLOAD'>T`, where `PAYLOAD` has length at most six over `a`, space, `>`, `"`, `/`, and `=`. The production parser is also checked on representative missing-quote and missing-close truncations. The proof models only the quote-aware boundary scan used inside this transition.
