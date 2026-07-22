//! # gremlin-rs
//!
//! A parser for the [Gremlin graph traversal language][gremlin], generated from
//! Apache TinkerPop's **official** `Gremlin.g4` grammar via
//! [`antlr-rust-runtime`](https://crates.io/crates/antlr-rust-runtime).
//!
//! Unlike a Gremlin *client* (which builds traversals programmatically and sends
//! bytecode to a server), this crate parses Gremlin **text** — e.g.
//! `g.V().has('name','marko').out('knows').values('name')` — into a parse tree
//! you can walk, translate, lint, or analyze.
//!
//! Because the parser is generated from the same grammar TinkerPop uses for its
//! JVM and JavaScript language variants, it tracks the official language rather
//! than a hand-maintained approximation.
//!
//! ## Example
//!
//! ```
//! use gremlin_rs::parse;
//!
//! let tree = parse("g.V().has('name','marko').out('knows').values('name')")?;
//! println!("{}", tree.tree().text());
//! # Ok::<(), gremlin_rs::GremlinParseError>(())
//! ```
//!
//! [gremlin]: https://tinkerpop.apache.org/gremlin.html

#![forbid(unsafe_code)]

mod generated {
    #![allow(warnings)]
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    pub mod gremlin_lexer;
    pub mod gremlin_parser;
}

pub use antlr4_runtime::ParsedFile;
use antlr4_runtime::{InputStream, Parser};

use generated::gremlin_lexer::GremlinLexer;
use generated::gremlin_parser::{GremlinParser, GremlinParserParseOutput};

/// An error produced while parsing Gremlin text.
#[derive(Debug)]
pub enum GremlinParseError {
    /// The lexer or parser failed outright (e.g. an unrecoverable error).
    Runtime(antlr4_runtime::AntlrError),
    /// The parser produced a tree but recovered from `count` syntax error(s);
    /// the input is not valid Gremlin. The recovered tree is provided for
    /// callers that want to inspect a best-effort result anyway.
    Syntax { count: usize, tree: Box<ParsedFile> },
}

impl core::fmt::Display for GremlinParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Runtime(e) => write!(f, "gremlin parse failed: {e}"),
            Self::Syntax { count, .. } => {
                write!(f, "gremlin input has {count} syntax error(s)")
            }
        }
    }
}

impl std::error::Error for GremlinParseError {}

impl From<antlr4_runtime::AntlrError> for GremlinParseError {
    fn from(e: antlr4_runtime::AntlrError) -> Self {
        Self::Runtime(e)
    }
}

/// Parses a Gremlin query list — one or more `;`-separated traversals — the
/// grammar's top-level `queryList` entry rule.
///
/// Returns the parse tree on success. If the input is syntactically invalid,
/// returns [`GremlinParseError::Syntax`] (which still carries the recovered
/// tree), or [`GremlinParseError::Runtime`] on an unrecoverable error.
///
/// # Errors
/// Returns [`GremlinParseError`] if the input is not valid Gremlin.
pub fn parse(input: &str) -> Result<ParsedFile, GremlinParseError> {
    parse_with(input, GremlinParser::query_list)
}

/// Parses a single Gremlin query (the grammar's `query` entry rule) rather than
/// a `;`-separated list. Use this when you know the input is exactly one
/// traversal or expression.
///
/// # Errors
/// Returns [`GremlinParseError`] if the input is not a valid single Gremlin query.
pub fn parse_query(input: &str) -> Result<ParsedFile, GremlinParseError> {
    parse_with(input, GremlinParser::query)
}

/// Shared driver: build lexer + token stream + parser, invoke `entry`, and
/// promote recovered syntax errors to a hard error so callers get a clean
/// valid/invalid signal by default.
fn parse_with<F>(input: &str, entry: F) -> Result<ParsedFile, GremlinParseError>
where
    F: FnOnce(
        &mut GremlinParser<GremlinLexer<InputStream>>,
    ) -> Result<antlr4_runtime::NodeId, antlr4_runtime::AntlrError>,
{
    let GremlinParserParseOutput { result, parser } =
        generated::gremlin_parser::parse_with_parser(input, GremlinLexer::new, entry)?;
    let count = parser.number_of_syntax_errors();
    let tree = parser.into_parsed_file(result);
    if count > 0 {
        return Err(GremlinParseError::Syntax {
            count,
            tree: Box::new(tree),
        });
    }
    Ok(tree)
}
