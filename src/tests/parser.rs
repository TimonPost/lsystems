use std::{vec};



use crate::{abs::*, lexer::*, parser::*};

#[test]
fn interpret_simple_action() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            interpret A as DrawForward(1);
        }",
    );

    let lex = lexer.lex(string);

    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    "A".into(),
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

    let tokens = LexedTokens::new(lex);

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
                    "A".into(),
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

    let tokens = LexedTokens::new(lex);

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
                    "A".into(),
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

    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let expression1 = ActionParam::Number(1.5);

    let expression2 = ActionParam::Number(2.5);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    "A".into(),
                    Action::new("DrawForward".into(), vec![expression1, expression2])
                )]
            )
        }
    );
}

#[test]
fn interpret_action_decimal_division() {
    let lexer = Lexer::new();
    let string = String::from(
        "lsystem LSystemName {
            interpret A as DrawForward(3.141592653589793238 / 3.141592653589793238, 1.5);
        }",
    );

    let lex = lexer.lex(string);

    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let expression1 = ActionParam::Expression(ExprKind::Binary(
        BinOpKind::Div,
        P::new(ActionParam::Number(3.141592653589793238)),
        P::new(ActionParam::Number(3.141592653589793238)),
    ));

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Interpret(
                    "A".into(),
                    Action::new(
                        "DrawForward".into(),
                        vec![expression1, ActionParam::Number(1.5)]
                    )
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

    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Replace(String::from("A"), String::from("B"))]
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

    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    assert_eq!(
        item,
        Item {
            item_kind: ItemKind::LSystem(
                "LSystemName".into(),
                vec![StatementKind::Replace(
                    String::from("ABC"),
                    String::from("B")
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

    let tokens = LexedTokens::new(lex);

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
                    StatementKind::Replace(String::from("F"), String::from("FF"))
                ]
            )
        }
    );
}

#[test]
fn koch_curve() {
    let definition = format!(
        "lsystem KochCurve {{
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
                    StatementKind::Replace(String::from("F"), String::from("F+F")),
                    StatementKind::Interpret(
                        "F".into(),
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
                        "+".into(),
                        Action::new(
                            "RotateZAction".into(),
                            vec![ActionParam::Expression(ExprKind::Binary(
                                BinOpKind::Div,
                                P::new(ActionParam::Number(3.14)),
                                P::new(ActionParam::Number(2.0))
                            ))]
                        )
                    )
                ]
            )
        }
    );
}

#[test]
fn parse_parameter_integer_number() {
    let mut tokens = LexedTokens::new(vec![
        Token::Param('('),
        Token::Number(1.0),
        Token::Symbol(','),
        Token::Number(20.0),
        Token::Symbol(','),
        Token::Number(300.0),
        Token::Symbol(','),
        Token::Number(301.0),
        Token::Param(')'),
    ]);

    let parsed = parse_module_parameters(&mut tokens);

    assert_eq!(parsed[0], ActionParam::Number(1.0));
    assert_eq!(parsed[1], ActionParam::Number(20.0));
    assert_eq!(parsed[2], ActionParam::Number(300.0));
    assert_eq!(parsed[3], ActionParam::Number(301.0));
    assert_eq!(parsed.get(4), None);
}

#[test]
fn parse_parameter_integer_flaot_1() {
    let mut tokens = LexedTokens::new(vec![
        Token::Param('('),
        //Token::Number(0), Token::Symbol('.'),Token::Number(1),
        Token::Number(0.01),
        Token::Param(')'),
    ]);

    println!("{:?}", tokens.tokens[2]);
    let parsed = parse_module_parameters(&mut tokens);

    assert_eq!(parsed[0], ActionParam::Number(0.01));
    assert_eq!(parsed.get(4), None);
}
