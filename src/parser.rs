use core::panic;
use std::{collections::VecDeque, vec};

use crate::{abs::*, lexer::Token, DefaultAlphabetSymbolDefiner, LSystem};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParsedToken {
    Function,
    Add,
    Sub,
    Mul,
}

#[derive(PartialEq, Clone, Debug)]
pub struct LexedTokens {
    pub tokens: Vec<Token>,
    index: usize,
}

impl LexedTokens {
    pub fn new(input: Vec<Token>) -> Self {
        LexedTokens {
            tokens: input
                .into_iter()
                .filter(|x| !matches!(x, Token::Space))
                .collect(),
            index: 0,
        }
    }
    pub fn finished(&self) -> bool {
        self.index > self.tokens.len() - 1
    }

    pub fn current_token_ref(&mut self) -> Option<&Token> {
        return self.tokens.get(self.index);
    }

    pub fn current_token(&mut self) -> Option<Token> {
        return self.tokens.get(self.index).cloned();
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn advance_by(&mut self, count: usize) {
        self.index += count;
    }
}

pub fn parse(mut tokens: LexedTokens) -> Item {
    let current_token = tokens.current_token_ref();

    match current_token {
        Some(Token::Ident(ident)) => match ident.as_str() {
            "lsystem" => {
                tokens.advance();
                let item_kind = parse_lsystem(tokens);
                 Item { item_kind }
            }
            _ => {
                panic!("Expected lsystem keyword found {:?}", current_token);
            }
        },
        _ => {
            panic!("Expected lsystem keyword found {:?}", current_token);
        }
    }
}

fn parse_lsystem(mut tokens: LexedTokens) -> ItemKind {
    if let Some(Token::Ident(l_system_name)) = tokens.current_token() {
        tokens.advance();
        tokens.advance();
        let mut statements = Vec::new();

        while !tokens.finished() {
            if let Some(Token::Parentesis('}')) = tokens.current_token_ref() {
                break;
            }

            let statement = parse_statement(&mut tokens);

            statements.push(statement);
        }

        ItemKind::LSystem(l_system_name, statements)
    } else {
        panic!("Expected lsystem name after 'lsystem' keyworld. Expected: 'lsystem MyLSystem {{ .. }}'");
    }
}

fn parse_statement(tokens: &mut LexedTokens) -> StatementKind {
    let statement = match tokens.current_token_ref() {
        Some(Token::Ident(ident)) => match ident.as_str() {
            "replace" => parse_replace(tokens),
            "interpret" => parse_interpret(tokens),
            "let" => StatementKind::DefineVariable,
            "axiom" => parse_axiom(tokens),
            _ => panic!(
                "Expected 'let' or 'interpret' or 'replace' keyword found {:?}",
                ident
            ),
        },
        Some(t) => panic!("Token '{:?}' not expected.", t),
        None => panic!("Not found"),
    };

    tokens.advance();

    statement
}

fn parse_axiom(tokens: &mut LexedTokens) -> StatementKind {
    tokens.advance();

    let mut symbols = Vec::new();
    while let Some(token) = tokens.current_token() {
        match token {
            Token::Symbol(symbol) => {
                symbols.push(symbol.to_string());
            }
            Token::Ident(symbol) => {
                symbols.push(symbol);
            }
            Token::Break => return StatementKind::Axiom(String::from_iter(symbols.into_iter())),
            _ => {
                panic!("Non supported symbol after keyworld 'axiom'. {:?}", token);
            }
        }
        tokens.advance();
    }

    panic!("No break found after 'axiom' keyword. Expected: 'axiom AB;'");
}

fn parse_let_statement() {}

fn parse_interpret(tokens: &mut LexedTokens) -> StatementKind {
    tokens.advance();

    let mut action_tokens = Vec::new();

    while let Some(token) = tokens.current_token() {
        match token {
            Token::Symbol(symbol) => {
                tokens.advance();
                action_tokens.push(symbol.to_string());
            }
            Token::Ident(string) => {
                tokens.advance();

                if string == "as" {
                    break;
                }

                action_tokens.push(string.clone());
            }
            token => panic!(
                "Unexpected token: {:?}. Expected: 'interpret X as Y(Z);'",
                token
            ),
        }
    }

    if tokens.current_token_ref().is_none() {
        panic!("Unfinished interpret statement. Could not find 'as' keyword. Expected: 'interpret X as Y(Z);'");
    }

    if let Some(Token::Ident(action_name)) = tokens.current_token() {
        tokens.advance();

        if let Some(Token::Param(lh_param)) = tokens.current_token_ref() {
            if *lh_param != '(' {
                panic!("Unexpected parameter character: {:?}.", lh_param);
            }

            let params = parse_module_parameters(tokens);

            assert!(
                action_tokens.len() == 1,
                "At the moment only one interpret symbol allowed."
            );
            return StatementKind::Interpret(
                action_tokens
                    .first()
                    .expect("Expect at least on interpret symbol.")
                    .clone(),
                Action::new(action_name, params),
            );
        } else {
            panic!("Expected left parameter '(' after action found no parameter. Expected: 'interpret X as Y(Z); {:?}'",tokens.current_token_ref());
        }
    } else {
        panic!("Expected action identity.")
    }
}

pub fn parse_module_parameters(tokens: &mut LexedTokens) -> Vec<ActionParam> {
    let mut params = Vec::new();
    let mut param_stack = VecDeque::new();

    while let Some(token) = tokens.current_token() {
        if let Token::Param(ident) = token {
            params.push(token.clone());
            tokens.advance();

            if ident == ')' {
                let param = param_stack.pop_back().expect("msg");
                assert_eq!(param, '(');

                if param_stack.is_empty() {
                    break;
                }
            } else if ident == '(' {
                param_stack.push_back('(');
            }
        } else {
            params.push(token.clone());
            tokens.advance();
        }
    }

    let mut tokens = LexedTokens::new(params);
    let mut params = Vec::new();

    while !tokens.finished() {
        let parsed_token = parse_parameters(&mut tokens, &ActionParam::None);
        if parsed_token != ActionParam::None {
            params.push(parsed_token);
        }
    }

    params
}

pub fn parse_parameters(tokens: &mut LexedTokens, prev_parsed: &ActionParam) -> ActionParam {
    if tokens.finished() {
        panic!("No more tokens in param list.");
    }

    match tokens.current_token().unwrap() {
        Token::Number(number) => {
            let param = ActionParam::Number(number);

            // Perhaps operator, comma, decimal.
            if !tokens.finished() {
                tokens.advance();
                let parsed_parameter = parse_parameters(tokens, &param);
                parsed_parameter
            } else {
                // Just a single number.
                param
            }
        }
        Token::Ident(ident) => {
            let param = ActionParam::Constant(ident);

            if !tokens.finished() {
                tokens.advance();
                let rh = parse_parameters(tokens, &param);
                rh
            } else {
                param
            }
        }
        Token::Symbol(symbol) => {
            tokens.advance();

            match symbol {
                '*' => {
                    // fetch the right hand side.
                    let rh = parse_parameters(tokens, prev_parsed);
                    ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Mul,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ))
                }
                '+' => {
                    // fetch the right hand side.
                    let rh = parse_parameters(tokens, prev_parsed);
                    ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Add,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ))
                }
                '-' => {
                    let rh = parse_parameters(tokens, prev_parsed);
                    ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Sub,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ))
                }
                '/' => {
                    // fetch the right hand side.
                    let rh = parse_parameters(tokens, prev_parsed);
                    ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Div,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ))
                }
                ',' => {
                    // return as we reached the end of the parameter expression.
                    prev_parsed.clone()
                }
                _ => panic!("Unexpected symbol: {:?}", symbol),
            }
        }
        Token::Param(param) => {
            if param == '(' {
                tokens.advance();
                let rh = parse_parameters(tokens, prev_parsed);
                rh
            } else if param == ')' {
                tokens.advance();
                return prev_parsed.clone();
            } else {
                panic!();
            }
        }
        _ => panic!("Not expected"),
    }
}

fn parse_replace(tokens: &mut LexedTokens) -> StatementKind {
    tokens.advance();

    let mut lh_tokens = Vec::new();
    let mut rh_tokens = Vec::new();

    while let Some(Token::Ident(ident)) = tokens.current_token_ref() {
        if ident == "by" {
            tokens.advance();
            break;
        }
        lh_tokens.push(Token::Ident(ident.clone()));
        tokens.advance();
    }

    if tokens.current_token_ref().is_none() {
        panic!("Unfinished replace statement. Could not find 'by' keyworld. Expected: 'replace X by Y;'");
    }

    while tokens.current_token_ref() != Some(&Token::Break) {
        rh_tokens.push(tokens.current_token_ref().unwrap().clone());
        tokens.advance();
    }

    if tokens.finished() {
        panic!("Unfinished replace statement. Could not find ';' after replace statement. Expected: 'replace X by Y;'");
    }

    parse_replace_statement(lh_tokens, rh_tokens)
}

fn parse_replace_statement(replace: Vec<Token>, by: Vec<Token>) -> StatementKind {
    let replace = replace
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>()
        .join("");
    let by = by
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>()
        .join("");

    StatementKind::Replace(replace, by)
}

pub struct LSystemParser {
    item: Item,
}

impl LSystemParser {
    pub fn lsystem_name(&self) -> String {
        let crate::parser::ItemKind::LSystem(name, _) = &self.item.item_kind;
        name.to_string()
    }

    pub fn axiom(&self) -> String {
        let crate::parser::ItemKind::LSystem(_, statements) = &self.item.item_kind;

        for statement in statements {
            if let crate::parser::StatementKind::Axiom(axiom) = statement {
                return axiom.to_string();
            }
        }

        panic!("No axiom found!");
    }

    pub fn replacement_rules(&mut self, lsystem: &mut LSystem<DefaultAlphabetSymbolDefiner>) {
        let crate::parser::ItemKind::LSystem(_, statements) = &self.item.item_kind;

        for statement in statements {
            if let crate::parser::StatementKind::Replace(replace, by) = statement {
                let replace = replace.to_string();
                let by = by.to_string();

                println!("{replace} by {by}");

                lsystem.add_dynamic_stochastic_rule(replace, by)
            }
        }
    }

    pub fn interpret_rules(&mut self) -> Vec<(String, Action)> {
        let crate::parser::ItemKind::LSystem(_, statements) = &self.item.item_kind;

        let mut interprets = vec![];
        for statement in statements {
            if let crate::parser::StatementKind::Interpret(interpret, by) = statement {
                interprets.push((interpret.clone(), by.clone()));
            }
        }

        interprets
    }

    pub fn parse(item: Item) -> LSystem<DefaultAlphabetSymbolDefiner> {
        let mut builder = LSystemParser { item };

        let mut lsystem = LSystem::<DefaultAlphabetSymbolDefiner>::new(
            builder.axiom(),
            DefaultAlphabetSymbolDefiner,
        );
        lsystem.name = builder.lsystem_name();
        lsystem.action_rules = builder.interpret_rules();
        builder.replacement_rules(&mut lsystem);
        lsystem
    }
}
