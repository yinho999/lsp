use rhai_rowan::ast::{AstNode, Rhai};
use rhai_rowan::Parser;

#[test]
fn test_is_empty() {
    let script = include_str!("../../../../testdata/versions/v1-10-0/is_empty.rhai");

    let parse = Parser::new(script).parse_script();

    assert!(parse.errors.is_empty(), "{:#?}", parse);
}
