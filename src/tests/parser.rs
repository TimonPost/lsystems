use std::vec;

use crate::{
    lexer::{Lexer, Token},
    parser::{
        parse, Action, ActionParam, BinOpKind, ExprKind, Item, ItemKind, LexedTokens, ParsedToken,
        StatementKind, P,
    },
};

#[test]
fn interpret_simple_action() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            interpret A as DrawForward(1);
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    vec!["A".into()],
                    Action::new("DrawForward".into(), vec![ActionParam::Number(1.0)])
                )]
            )
        }
    );
}

#[test]
fn interpret_action_addition() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            interpret A as DrawForward(1+1);
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let expression = ActionParam::Expression(ExprKind::Binary(
        BinOpKind::Add,
        P::new(ActionParam::Number(1.0)),
        P::new(ActionParam::Number(1.0)),
    ));

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    vec!["A".into()],
                    Action::new("DrawForward".into(), vec![expression])
                )]
            )
        }
    );
}

#[test]
fn interpret_action_two_additions() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            interpret A as DrawForward(1+1, 2+2);
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let expression1 = ActionParam::Expression(ExprKind::Binary(
        BinOpKind::Add,
        P::new(ActionParam::Number(1.0)),
        P::new(ActionParam::Number(1.0)),
    ));

    let expression2 = ActionParam::Expression(ExprKind::Binary(
        BinOpKind::Add,
        P::new(ActionParam::Number(2.0)),
        P::new(ActionParam::Number(2.0)),
    ));

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    vec!["A".into()],
                    Action::new("DrawForward".into(), vec![expression1, expression2])
                )]
            )
        }
    );
}

#[test]
fn interpret_action_decimal() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            interpret A as DrawForward(1.5, 2.5);
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let expression1 = ActionParam::Number(1.5);

    let expression2 = ActionParam::Number(2.5);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    vec!["A".into()],
                    Action::new("DrawForward".into(), vec![expression1, expression2])
                )]
            )
        }
    );
}

#[test]
fn replace_single_const() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            replace A by B;
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Replace(
                    vec![Token::Ident("A".into())],
                    vec![Token::Ident("B".into())]
                )]
            )
        }
    );
}

#[test]
fn replace_multi_const() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            replace A B C by B;
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Replace(
                    vec![
                        Token::Ident("A".into()),
                        Token::Ident("B".into()),
                        Token::Ident("C".into())
                    ],
                    vec![Token::Ident("B".into())]
                )]
            )
        }
    );
}

#[test]
fn axiom() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            axiom F+A;
        }",
    );

    let lex = lexer.lex(string);

    let mut tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Axiom("F+A".into())]
            )
        }
    );
}

#[test]
fn fractal_plant() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem FractalPlant {
            axiom X;

            replace F by FF;
        }",
    );

    let lex = lexer.lex(string);

    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "FractalPlant".into(),
                vec![
                    StatementKind::Axiom("X".into()),
                    StatementKind::Replace(
                        vec![Token::Ident("F".into()),],
                        vec![Token::Ident("FF".into())]
                    )
                ]
            )
        }
    );
}

#[test]
fn koch_curve() {
    let definition = format!(
        "   lsystem KochCurve {{
            axiom F;

            replace F by F+F;
            interpret F as KochDrawF({:.1},{:.1},{:.1},{:.1});
            interpret + as RotateZAction(3.14/2);
        }}
    ",
        5, 0.2, 0.2, 0.2
    );

    let lexer = Lexer::new();

    let lex = lexer.lex(definition);
    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "KochCurve".into(),
                vec![
                    StatementKind::Axiom("F".into()),
                    StatementKind::Replace(
                        vec![Token::Ident("F".into()),],
                        vec![Token::Ident("F+F".into())]
                    ),
                    StatementKind::Interpret(
                        vec!["F".into()],
                        Action::new(
                            "KochDrawF".into(),
                            vec![
                                ActionParam::Number(5.0),
                                ActionParam::Number(0.2),
                                ActionParam::Number(0.2),
                                ActionParam::Number(0.2)
                            ]
                        )
                    ),
                    StatementKind::Interpret(
                        vec!["+".into()],
                        Action::new("RotateZAction".into(), vec![ActionParam::Expression(ExprKind::Binary(BinOpKind::Div, P::new(ActionParam::Number(3.14)), P::new(ActionParam::Number(2.0))))])
                    )
                ]
            )
        }
    );
}
