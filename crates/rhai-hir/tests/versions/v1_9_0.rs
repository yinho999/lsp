use rhai_hir::Hir;
use rhai_rowan::Parser;

// `testdata/versions/v1-9-0/definition.d.rhai`
#[test]
fn test_definition_api() {
    let definition_str = include_str!("../../../../testdata/versions/v1-9-0/definition.d.rhai");
    let def = Parser::new(definition_str).parse_def();
    assert!(def.errors.is_empty(), "{:#?}", def);

    // Hir test
    let mut hir = Hir::new();
    hir.add_source(
        &"test:///root.rhai".parse().unwrap(),
        &Parser::new(definition_str).parse_def().into_syntax(),
    );
    hir.resolve_all();
    assert!(hir.errors().is_empty(), "{:#?}", hir);
}