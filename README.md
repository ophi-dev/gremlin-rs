# gremlin-rs

A Rust **parser** for the [Gremlin graph traversal language][gremlin], generated
from Apache TinkerPop™'s **official** [`Gremlin.g4`][grammar] grammar via
[`antlr-rust-runtime`](https://crates.io/crates/antlr-rust-runtime).

```rust
use gremlin_rs::parse;

let tree = parse("g.V().has('name','marko').out('knows').values('name')")?;
println!("{}", tree.tree().text());
# Ok::<(), gremlin_rs::GremlinParseError>(())
```

## Parser, not a client

This crate parses Gremlin **text** into a parse tree. It is *not* a database
client and does not talk to a Gremlin Server — for that, see
[`gremlin-client`](https://crates.io/crates/gremlin-client). Use `gremlin-rs`
when you need to read, analyze, translate, lint, or transform Gremlin *source*:

- tooling (formatters, linters, syntax highlighting, LSPs)
- query translation / rewriting
- static analysis of stored or user-submitted traversals

## Why generated from the official grammar

TinkerPop maintains one grammar, [`Gremlin.g4`][grammar], and generates its JVM
and JavaScript language variants' text parsers from it. `gremlin-rs` is generated
from that same grammar, so it tracks the official language definition rather than
a hand-maintained re-implementation that can drift as Gremlin evolves.

## API

- [`parse`] — parse a `;`-separated query list (grammar entry rule `queryList`).
- [`parse_query`] — parse a single traversal/expression (entry rule `query`).

Both return [`ParsedFile`] on success. Invalid input returns a
`GremlinParseError` — `Syntax { count, tree }` when the parser recovered from
syntax errors (the best-effort tree is still provided), or `Runtime` on an
unrecoverable error.

## Provenance

| | |
|---|---|
| Grammar | [`apache/tinkerpop` `gremlin-language/src/main/antlr4/Gremlin.g4`][grammar] |
| Grammar commit | `5dcf41fe6161` (2026-07-09) |
| Runtime | [`antlr-rust-runtime`](https://github.com/ophi-dev/antlr-rust-runtime) `2f46383a84ba7efbe10ed25bd1ff1d46d8bfccb7` |
| Generated with | `antlr4-rust-gen` directly from `grammar/Gremlin.g4` |

The generated lexer/parser modules under `src/generated/` are committed so the
crate builds without the generator. To regenerate against a newer `Gremlin.g4`,
see [`grammar/REGENERATE.md`](grammar/REGENERATE.md).

## License

Licensed under the [Apache License, Version 2.0](LICENSE). The `Gremlin.g4`
grammar this parser is generated from is © the Apache Software Foundation,
licensed under Apache-2.0; see [`NOTICE`](NOTICE). "Gremlin", "TinkerPop", and
"Apache TinkerPop" are trademarks of the Apache Software Foundation. This is an
independent project and is not affiliated with or endorsed by the ASF.

[gremlin]: https://tinkerpop.apache.org/gremlin.html
[grammar]: https://github.com/apache/tinkerpop/blob/master/gremlin-language/src/main/antlr4/Gremlin.g4
