use std::collections::VecDeque;

use crate::lexer::Token;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParsedToken {
    Function,
    Add,
    Sub,
    Mul,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    pub item_kind: ItemKind,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ItemKind {
    LSystem(String, Vec<StatementKind>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum StatementKind {
    Axiom(String),
    DefineVariable,
    Replace(Vec<Token>, Vec<Token>),
    Interpret(Vec<Constant>, Action),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Action {
    name: String,
    params: Vec<ActionParam>,
}

impl Action {
    pub fn new(name: String, params: Vec<ActionParam>) -> Self {
        Self {
            name:name.into(),
            params,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ActionParam {
    Number(Number),
    Constant(Constant),
    Expression(ExprKind),
    None,
}

type Constant = String;
type Number = f32;

#[derive(PartialEq, Clone, Debug)]
pub enum ExprKind {
    Binary(BinOpKind, P<ActionParam>, P<ActionParam>),
}

#[derive(PartialEq, Clone, Debug)]
pub struct P<T: ?Sized + PartialEq + Clone> {
    ptr: Box<T>,
}

impl<T: PartialEq + Clone> P<T> {
    pub fn new(ptr: T) -> Self {
        Self { ptr: Box::new(ptr) }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitXor,
    BitAnd,
    BitOr,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}

pub enum ExpressionKind {}

pub struct LexedTokens {
    tokens: Vec<Token>,
    index: usize,
}

impl LexedTokens {
    pub fn new(input: Vec<Token>) -> Self {
        LexedTokens {
            tokens: input,
            index: 0,
        }
    }
    pub fn finished(&self) -> bool {
        self.index > self.tokens.len() - 1
    }

    pub fn current_token_ref(&self) -> Option<&Token> {
        return self.tokens.get(self.index);
    }

    pub fn current_token(&self) -> Option<Token> {
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
                return Item { item_kind };
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

        return ItemKind::LSystem(l_system_name.into(), statements);
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

            return StatementKind::Interpret(
                action_tokens,
                Action::new(action_name.into(), params),
            );
        } else {
            panic!("Expected left parameter '(' after action found no parameter. Expected: 'interpret X as Y(Z); {:?}'",tokens.current_token_ref());
        }
    } else {
        panic!("Expected action identity.")
    }
}

fn parse_module_parameters(tokens: &mut LexedTokens) -> Vec<ActionParam> {
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
        params.push(parse_parameters(&mut tokens, &ActionParam::None));
    }

    params
}

fn parse_parameters(tokens: &mut LexedTokens, prev_parsed: &ActionParam) -> ActionParam {
    if tokens.finished() {
        panic!("No more tokens in param list.");
    }

    println!("{:?}", tokens.current_token_ref());
    
    match tokens.current_token().unwrap() {
        Token::Number(number) => {
            let param = ActionParam::Number(number as f32);

            if !tokens.finished() {
                tokens.advance();
                let rh = parse_parameters(tokens, &param);
                return rh;
            } else {
                return param;
            }
        }
        Token::Ident(ident) => {
            let param = ActionParam::Constant(ident);

            if !tokens.finished() {
                tokens.advance();
                let rh = parse_parameters(tokens, &param);
                return rh;
            } else {
                return param;
            }
        }
        Token::Symbol(symbol) => {
            tokens.advance();

            match symbol {
                '*' => {
                    let rh = parse_parameters(tokens, prev_parsed);
                    return ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Mul,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ));
                }
                '+' => {
                    let rh = parse_parameters(tokens, prev_parsed);
                    return ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Add,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ));
                }
                '-' => {
                    let rh = parse_parameters(tokens, prev_parsed);
                    return ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Sub,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ));
                }
                '/' => {
                    let rh = parse_parameters(tokens, prev_parsed);
                    return ActionParam::Expression(ExprKind::Binary(
                        BinOpKind::Div,
                        P::new(prev_parsed.clone()),
                        P::new(rh),
                    ));
                }
                '.' => {
                    if let (ActionParam::Number(lh), Some(Token::Number(rh))) =
                        (prev_parsed, tokens.current_token())
                    {
                        tokens.advance();

                        let decimal = lh + rh as f32 / 10.0;
                        return ActionParam::Number(decimal);
                    } else {
                        panic!("Expected number after '.'. Expected: 'interpret X as Y(10.20));'");
                    }
                }
                ',' => {                    
                    return prev_parsed.clone();
                }
                _ => panic!("Unexpected symbol: {:?}", symbol),
            }
        }
        Token::Param(param) => {
            if param == '(' {
                tokens.advance();
                let rh = parse_parameters(tokens, prev_parsed);
                return rh;
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

    StatementKind::Replace(lh_tokens, rh_tokens)
}
