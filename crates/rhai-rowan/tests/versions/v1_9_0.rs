use logos::Logos;
use rhai::Token;
use rhai_rowan::ast::{AstNode, Expr, Rhai};
use rhai_rowan::syntax::SyntaxKind;
use rhai_rowan::Parser;

// `testdata/versions/v1-9-0/switch-multi-values.rhai`
#[test]
fn switch_multiple_values() {
    let script = "switch x { 1 | 2 | 3 => 1, _ => 3 }";
    let parse = Parser::new(script).parse_script();
    assert!(parse.errors.is_empty(), "{:#?}", parse);
    let rhai = Rhai::cast(parse.into_syntax()).unwrap();
    let stmt = rhai.statements().next().expect("one statement");
    let expr = stmt
        .item()
        .and_then(|item| item.expr())
        .expect("an expression");

    // switch x { 1 | 2 | 3 => 1, _ => 3 }
    let Expr::Switch(switch) = expr else {
        panic!("expected a switch expression, got {expr:#?}");
    };

    // 1 | 2 | 3 => 1
    let arm = switch
        .switch_arm_list()
        .and_then(|list| list.arms().next())
        .expect("first switch arm");
    // 1 | 2 | 3
    let pattern = arm.syntax().first_child().expect("a pattern");

    // Should be Syntax::SWITCH_ARM_PATTERNS or something similar for 1 | 2 | 3 not regular expr
    assert!(
        !matches!(pattern.kind(), SyntaxKind::EXPR),
        "pattern was {pattern:#?}"
    );
}

// `testdata/versions/v1-9-0/reserved-left-right-pipe.rhai`
#[test]
fn reserved_left_pipe() {
    let mut lexer = SyntaxKind::lexer("<|");
    let first = lexer.next();
    let second = lexer.next();
    assert!(first.is_some());
    assert!(
        first.unwrap().is_reserved_keyword(),
        "first token is not reserved: {first:#?}"
    );
    assert!(second.is_none(), "second token was {second:#?}");
}

// `testdata/versions/v1-9-0/reserved-left-right-pipe.rhai`
#[test]
fn reserved_right_pipe() {
    let mut lexer = SyntaxKind::lexer("|>");
    let first = lexer.next();
    let second = lexer.next();
    assert!(first.is_some());
    assert!(
        first.unwrap().is_reserved_keyword(),
        "first token is not reserved: {first:#?}"
    );
    assert!(second.is_none(), "second token was {second:#?}");
}

// `testdata/versions/v1-9-0/module-doc.rhai`
#[test]
fn module_doc() {
    let parse = Parser::new("//! Module docs.\n").parse_script();
    assert!(parse.errors.is_empty());
    let rhai = Rhai::cast(parse.into_syntax()).unwrap();
    assert_eq!(rhai.script_docs(), "Module docs.\n");

    let mut lexer = SyntaxKind::lexer("//!");
    let first = lexer.next();
    assert!(first.is_some());
    let first = first.unwrap();
    assert_ne!(
        first,
        SyntaxKind::COMMENT_LINE,
        "//! should be module documentation: {first:#?}"
    );
}

// `testdata/versions/v1-9-0/switch-duplicate.rhai`
#[test]
fn switch_duplicate() {
    let parse = Parser::new("switch 1 { 1 => 1, 1 => 2 }").parse_script();
    assert!(parse.errors.is_empty(), "{:#?}", parse);
    let rhai = Rhai::cast(parse.into_syntax()).unwrap();
    let stmt = rhai.statements().next().expect("no statement");
    let expr = stmt
        .item()
        .and_then(|item| item.expr())
        .expect("an expression");

    // switch 1 { 1 => 1, 1 => 2 }
    let Expr::Switch(switch) = expr else {
        panic!("expected a switch expression, got {expr:#?}");
    };

    // 1 => 1, 1 => 2
    let arms = switch.switch_arm_list().unwrap();
    assert_eq!(arms.arms().count(), 2);
}

// `testdata/versions/v1-9-0/function-pointer-shorthand.rhai`
#[test]
fn function_pointer_shorthand() {
    let parse = Parser::new("fn foo(z) { z } \n let f = foo").parse_script();
    assert!(parse.errors.is_empty(), "{:#?}", parse);

    let rhai = Rhai::cast(parse.into_syntax()).unwrap();
    let mut stmt = rhai.statements();
    let expr = stmt
        .next()
        .unwrap()
        .item()
        .expect("an item")
        .expr()
        .expect("an expression");
    let Expr::Fn(func_expr) = expr else {
        panic!("expected a function pointer expression, got {expr:#?}");
    };

    // fn foo(z) { z }
    assert_eq!(func_expr.syntax().kind(), SyntaxKind::EXPR_FN);

    let stmt = stmt.next().expect("expect second statement");
    let expr = stmt.item().expect("an item").expr().expect("an expression");
    let Expr::Let(assign) = expr else {
        panic!("expected an assignment expression, got {expr:#?}");
    };
    // let f = foo
    assert_eq!(assign.syntax().kind(), SyntaxKind::EXPR_LET);
    let let_expr = assign.expr().expect("an expression");
    let Expr::Ident(ident) = let_expr else {
        panic!("expected an identifier expression, got {let_expr:#?}");
    };
    assert_eq!(ident.ident_token().unwrap().text(), "foo");
}
