# Regenerating the parser

The Rust lexer/parser under `src/generated/` is mechanically generated from
`grammar/Gremlin.g4` (Apache TinkerPop's official Gremlin grammar). To update it
against a newer grammar revision:

```bash
# 1. Fetch the latest official grammar
curl -fLo grammar/Gremlin.g4 \
  https://raw.githubusercontent.com/apache/tinkerpop/master/gremlin-language/src/main/antlr4/Gremlin.g4

# 2. Emit ANTLR .interp metadata (requires the ANTLR 4.13.2 tool jar + Java)
#    Gremlin.g4 is a combined grammar, so ANTLR emits Gremlin.interp + GremlinLexer.interp
java -jar antlr-4.13.2-complete.jar -Dlanguage=Java -o build grammar/Gremlin.g4

# 3. Generate the Rust modules (requires antlr-rust-runtime's generator)
cargo install antlr-rust-runtime --bin antlr4-rust-gen   # once
antlr4-rust-gen \
  --lexer  build/GremlinLexer.interp \
  --parser build/Gremlin.interp \
  --out-dir src/generated

# 4. Verify
cargo test
```

Record the source grammar commit in `README.md` (the "Provenance" table) after
regenerating, and re-run the test suite to confirm real traversals still parse.
