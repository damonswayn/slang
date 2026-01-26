use super::Parser;
use crate::ast::Statement;
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

#[test]
fn test_class_statement_parsing() {
    let input = r#"
        class Point {
            function getX() { this.x; }
            function getY() { this.y; }
        }
    "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_errors(&p);

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Class(cs) => {
            assert_eq!(cs.name.value, "Point");
            assert_eq!(cs.methods.len(), 2);
            assert_eq!(cs.methods[0].name.value, "getX");
            assert_eq!(cs.methods[1].name.value, "getY");
        }
        other => panic!("expected Class statement, got {:?}", other),
    }
}

#[test]
fn test_class_with_constructor_parsing() {
    let input = r#"
        class Rectangle {
            function construct(w, h) {
                this.width = w;
                this.height = h;
            }
            function area() { this.width * this.height; }
        }
    "#;

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_errors(&p);

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Class(cs) => {
            assert_eq!(cs.name.value, "Rectangle");
            assert_eq!(cs.methods.len(), 2);
            assert_eq!(cs.methods[0].name.value, "construct");
            assert_eq!(cs.methods[0].literal.params.len(), 2);
            assert_eq!(cs.methods[0].literal.params[0].value, "w");
            assert_eq!(cs.methods[0].literal.params[1].value, "h");
            assert_eq!(cs.methods[1].name.value, "area");
        }
        other => panic!("expected Class statement, got {:?}", other),
    }
}

#[test]
fn test_new_expression_parsing() {
    let tests = vec![
        ("new Foo();", "Foo", 0),
        ("new Bar(1, 2);", "Bar", 2),
        ("new Point(x, y, z);", "Point", 3),
    ];

    for (input, expected_name, expected_args) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_errors(&p);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(es) => match &es.expression {
                crate::ast::Expression::New(ne) => {
                    assert_eq!(ne.class_name.value, expected_name);
                    assert_eq!(ne.arguments.len(), expected_args);
                }
                other => panic!("expected New expression, got {:?}", other),
            },
            other => panic!("expected Expression statement, got {:?}", other),
        }
    }
}
