//! Integration tests exercising the public API against real Gremlin traversals.

use gremlin_rs::{parse, parse_query, GremlinParseError};

/// Canonical traversals from the TinkerPop docs should parse cleanly.
#[test]
fn parses_real_traversals() {
    let queries = [
        "g.V()",
        "g.V().count()",
        "g.V().has('name','marko').out('knows').values('name')",
        "g.V().hasLabel('person').groupCount().by('age')",
        "g.V().repeat(out()).times(2).path()",
        "g.addV('person').property('name','stephen')",
        "g.V().where(__.out('created')).values('name')",
    ];
    for q in queries {
        let tree = parse(q).unwrap_or_else(|e| panic!("expected {q:?} to parse, got {e}"));
        // The reconstructed text should be non-empty (the tree covered the input).
        assert!(!tree.tree().text().is_empty(), "empty tree text for {q:?}");
    }
}

/// A `;`-separated query list is the top-level `parse` entry.
#[test]
fn parses_query_list() {
    let tree = parse("g.V().count();g.E().count()").expect("query list should parse");
    assert!(!tree.tree().text().is_empty());
}

/// `parse_query` accepts a single traversal.
#[test]
fn parse_query_accepts_single() {
    parse_query("g.V().limit(10)").expect("single query should parse");
}

/// Invalid Gremlin should surface as a syntax error, not a panic or false success.
#[test]
fn rejects_invalid_input() {
    let err = parse("g.V(((").expect_err("garbage should not parse cleanly");
    match err {
        GremlinParseError::Syntax { count, .. } => assert!(count > 0),
        GremlinParseError::Runtime(_) => { /* also acceptable */ }
    }
}

/// The error type renders a useful message.
#[test]
fn error_displays() {
    if let Err(e) = parse("g.V(") {
        assert!(!e.to_string().is_empty());
    }
}
