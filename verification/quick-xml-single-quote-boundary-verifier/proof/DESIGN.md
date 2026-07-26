# Executable Verus harness

- modeled transition
  - the three `ElementParser` states and its significant-byte step for `>`, `'`, and `"`
  - entry into a single-quoted value, up to six arbitrary non-single-quote payload bytes, the matching quote, and the following outside `>`
  - no boundary while the state remains `SingleQ`
  - exact relative consumption from the closing quote through the outside `>`
  - EOF classification before and after the matching quote
- universal inputs
  - arbitrary payload length `0..=6`
  - arbitrary six payload bytes, each constrained only to differ from the single quote
  - this includes arbitrary literal `>` and double-quote bytes inside the single-quoted value
- production binding limit
  - the step function mirrors pinned `ElementParser::feed`
  - the separate finite host harness executes both that public parser and ordinary slice `Reader::read_event`
  - the proof does not verify `memchr`, the reader implementation, other tags or events, arbitrary-length payloads, other quote mode, validation, or whole documents
- executable gate
  - run the staged verifier with `--no-cheating --compile`
  - execute the produced native binary
  - reject `assume`, `admit`, `external_body`, and `assume_specification`
