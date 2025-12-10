use crate::ast::Statement;
use super::Parser;
use crate::lexer::Lexer;
use crate::test_support::check_errors;

#[test]
fn test_let_statements() {
    let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_errors(&p);

    assert_eq!(program.statements.len(), 3);

    let names = vec!["x", "y", "foobar"];

    for (i, name) in names.iter().enumerate() {
        match &program.statements[i] {
            Statement::Let(ls) => assert_eq!(ls.name.value, *name),
            _ => panic!("statement {} is not a LetStatement", i),
        }
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("1 + 2 * 3;", "(1 + (2 * 3))"),
        ("1 * 2 + 3;", "((1 * 2) + 3)"),
        ("(1 + 2) * 3;", "((1 + 2) * 3)"),
        ("a.b.c;", "a.b.c"),
        ("{ x: 1, y: 2 }.x;", "{x: 1, y: 2}.x"),
        ("x++;", "(x++)"),
        ("++x;", "(++x)"),
        ("x++ + 1;", "((x++) + 1)"),
        ("++x + 1;", "((++x) + 1)"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_errors(&p);

        let actual = program.to_string();
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_namespace_statement_parsing() {
    let input = r#"
        namespace Utils {
            let x = 1;
            function add(a, b) { a + b; }
        }
    "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_errors(&p);

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Namespace(ns) => {
            assert_eq!(ns.name.value, "Utils");
            assert_eq!(ns.body.statements.len(), 2);
        }
        other => panic!("expected Namespace statement, got {:?}", other),
    }
}

#[test]
fn test_import_statement_parsing() {
    let input = r#"
        import "foo.sl";
    "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_errors(&p);

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Import(is) => assert_eq!(is.path, "foo.sl"),
        other => panic!("expected Import statement, got {:?}", other),
    }
}


