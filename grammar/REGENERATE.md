# Regenerating the parser

The Rust lexer/parser under `src/generated/` is mechanically generated from
`grammar/Gremlin.g4` (Apache TinkerPop's official Gremlin grammar). To update it
against a newer grammar revision:

```bash
# 1. Fetch the latest official grammar
curl -fLo grammar/Gremlin.g4 \
  https://raw.githubusercontent.com/apache/tinkerpop/master/gremlin-language/src/main/antlr4/Gremlin.g4

# 2. Generate the Rust modules directly from the combined grammar
cargo install antlr-rust-runtime --features codegen --bin antlr4-rust-gen
antlr4-rust-gen grammar/Gremlin.g4 --out-dir src/generated

# 3. Verify
cargo test
```

Record the source grammar commit in `README.md` (the "Provenance" table) after
regenerating, and re-run the test suite to confirm real traversals still parse.
