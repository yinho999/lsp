use rhai_hir::Hir;
use rhai_rowan::Parser;

// `testdata/versions/v1-10-0/typed_constants_variables.d.rhai`
#[test]
fn test_typed_constants_variables() {
    let typed_constants_variables = include_str!("../../../../testdata/versions/v1-10-0/typed_constants_variables.d.rhai");
    let def = Parser::new(typed_constants_variables).parse_def();
    assert!(def.errors.is_empty(), "{:#?}", def);
    // Hir test
    let mut hir = Hir::new();
    hir.add_source(
        &"test:///typed_constants_variables.d.rhai".parse().unwrap(),
        &Parser::new(typed_constants_variables).parse_def().into_syntax(),
    );
    hir.resolve_all();
    assert!(hir.errors().is_empty(), "{:#?}", hir);
    let (_, data) = hir
        .symbols()
        .find(|(_, data)| data.kind.as_decl().is_some_and(|d| d.name == "CONSTANT"))
        .expect("a declaration named CONSTANT");

    assert_eq!(data.ty.fmt(&hir).to_string(), "int");

    let (_, data) = hir
        .symbols()
        .find(|(_, data)| data.kind.as_decl().is_some_and(|d| d.name == "hello_there"))
        .expect("a declaration named hello_there");

    assert_eq!(data.ty.fmt(&hir).to_string(), "string");

    let (_, data) = hir
        .symbols()
        .find(|(_, data)| data.kind.as_decl().is_some_and(|d| d.name == "HELLO"))
        .expect("a declaration named HELLO");

    assert_eq!(data.ty.fmt(&hir).to_string(), "string");
}


