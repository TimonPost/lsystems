use crate::lexer::{Lexer, Token};

#[test]
fn number_lexer() {
    let lexer = Lexer::new();
    let string = String::from("1 23 456");
    let mut tokens = lexer.lex(string).into_iter();

    assert_eq!(tokens.next().unwrap(), Token::Number(1));
    assert_eq!(tokens.next().unwrap(), Token::Space);
    assert_eq!(tokens.next().unwrap(), Token::Number(23));
    assert_eq!(tokens.next().unwrap(), Token::Space);
    assert_eq!(tokens.next().unwrap(), Token::Number(456));
    assert!(tokens.next().is_none());
}

#[test]
fn ident_lexer() {
    let lexer = Lexer::new();
    let string = String::from("a bc def");
    let mut tokens = lexer.lex(string).into_iter();

    assert_eq!(tokens.next().unwrap(), Token::Ident("a".into()));
    assert_eq!(tokens.next().unwrap(), Token::Space);
    assert_eq!(tokens.next().unwrap(), Token::Ident("bc".into()));
    assert_eq!(tokens.next().unwrap(), Token::Space);
    assert_eq!(tokens.next().unwrap(), Token::Ident("def".into()));
    assert!(tokens.next().is_none());
}

#[test]
fn bracket_lexer() {
    let lexer = Lexer::new();
    let string = String::from("[]");
    let mut tokens = lexer.lex(string).into_iter();

    assert_eq!(tokens.next().unwrap(), Token::Bracket('['));
    assert_eq!(tokens.next().unwrap(), Token::Bracket(']'));
    assert!(tokens.next().is_none());
}

#[test]
fn param_lexer() {
    let lexer = Lexer::new();
    let string = String::from("()");
    let mut tokens = lexer.lex(string).into_iter();

    assert_eq!(tokens.next().unwrap(), Token::Param('('));
    assert_eq!(tokens.next().unwrap(), Token::Param(')'));
    assert!(tokens.next().is_none());
}

#[test]
fn parentesis_lexer() {
    let lexer = Lexer::new();
    let string = String::from("{}");
    let mut tokens = lexer.lex(string).into_iter();

    assert_eq!(tokens.next().unwrap(), Token::Parentesis('{'));
    assert_eq!(tokens.next().unwrap(), Token::Parentesis('}'));
    assert!(tokens.next().is_none());
}

#[test]
fn symbol_lexer() {
    let lexer = Lexer::new();
    let string = String::from("+ - * / > < & | \\ ^ = .");
    let mut tokens = lexer
        .lex(string)
        .into_iter()
        .filter(|x| !matches!(x, Token::Space));

    assert_eq!(tokens.next().unwrap(), Token::Symbol('+'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('-'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('*'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('/'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('>'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('<'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('&'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('|'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('\\'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('^'));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('='));
    assert_eq!(tokens.next().unwrap(), Token::Symbol('.'));
    assert!(tokens.next().is_none());
}

#[test]
fn break_lexer() {
    let lexer = Lexer::new();
    let string = String::from(";");
    let mut tokens = lexer.lex(string).into_iter();

    assert_eq!(tokens.next().unwrap(), Token::Break);
    assert!(tokens.next().is_none());
}

#[test]
fn lsystem_lexer() {
    let lexer = Lexer::new();
    let string = String::from(
        "
    lsystem LSystemName {
        replace F by F[F];
        interpret A as Test(5);
    }",
    );

    let mut tokens = lexer
        .lex(string)
        .into_iter()
        .filter(|x| !matches!(x, Token::Space));

    assert_eq!(tokens.next().unwrap(), Token::Ident("lsystem".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("LSystemName".into()));
    assert_eq!(tokens.next().unwrap(), Token::Parentesis('{'));

    assert_eq!(tokens.next().unwrap(), Token::Ident("replace".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("F".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("by".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("F".into()));
    assert_eq!(tokens.next().unwrap(), Token::Bracket('['));
    assert_eq!(tokens.next().unwrap(), Token::Ident("F".into()));
    assert_eq!(tokens.next().unwrap(), Token::Bracket(']'));
    assert_eq!(tokens.next().unwrap(), Token::Break);

    assert_eq!(tokens.next().unwrap(), Token::Ident("interpret".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("A".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("as".into()));
    assert_eq!(tokens.next().unwrap(), Token::Ident("Test".into()));
    assert_eq!(tokens.next().unwrap(), Token::Param('('));
    assert_eq!(tokens.next().unwrap(), Token::Number(5));
    assert_eq!(tokens.next().unwrap(), Token::Param(')'));
    assert_eq!(tokens.next().unwrap(), Token::Break);
    assert_eq!(tokens.next().unwrap(), Token::Parentesis('}'));
    assert!(tokens.next().is_none());
}
