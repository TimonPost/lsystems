use std::ops::Range;

use regex::Regex;

struct LanguageRegex {
    operator_regex: Regex,
    char_regex: Regex,
    symbol_regex: Regex,
    branching_regex: Regex,
    param_regex: Regex,
    whitespace_regex: Regex,
    break_regex: Regex,
    parentesis_regex: Regex,
    number_regex: Regex,
}

impl LanguageRegex {
    pub fn new() -> Self {
        let operator_regex = Regex::new(r"\+|-|/|\*|%").unwrap();
        let char_regex = Regex::new(r"[a-zA-Z]").unwrap();
        let symbol_regex = Regex::new(r"\+|-|\*|/|>|<|&|\||\\|\^|=|,").unwrap();
        let branching_regex = Regex::new(r"\[|\]").unwrap();
        let param_regex = Regex::new(r"\(|\)").unwrap();
        let whitespace_regex = Regex::new(r"\s").unwrap();
        let break_regex = Regex::new(r";").unwrap();
        let parentesis_regex = Regex::new(r"\{|\}").unwrap();
        let number_regex = Regex::new(r"\d+.\d+|\d+").unwrap();

        LanguageRegex {
            operator_regex,
            char_regex,
            symbol_regex,
            branching_regex,
            param_regex,
            whitespace_regex,
            break_regex,
            parentesis_regex,
            number_regex,
        }
    }
}

struct UnlexedTokens {
    tokens: String,
    index: usize,
}

impl UnlexedTokens {
    pub fn new(input: String) -> Self {
        UnlexedTokens {
            tokens: input,
            index: 0,
        }
    }
    pub fn finished(&self) -> bool {
        self.index > self.tokens.len() - 1
    }

    pub fn current_token(&self) -> &str {
        return &self.tokens[self.index..self.index + 1];
    }

    pub fn first_char(token: &str) -> char {
        token.chars().next().unwrap()
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn advance_by(&mut self, count: usize) {
        self.index += count;
    }
}

pub struct Lexer {
    regex: LanguageRegex,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            regex: LanguageRegex::new(),
        }
    }

    pub fn lex(&self, input: String) -> Vec<Token> {
        let unlexed_tokens = UnlexedTokens::new(input);
        let mut lexed_tokens = Vec::new();

        self.lex_next_char(unlexed_tokens, &mut lexed_tokens);

        lexed_tokens
    }

    fn lex_next_char(&self, mut unlexed_tokens: UnlexedTokens, tokens: &mut Vec<Token>) {
        if unlexed_tokens.finished() {
            return;
        }

        let current_symbol = unlexed_tokens.current_token();
        let current_char = UnlexedTokens::first_char(current_symbol);

        if self.regex.symbol_regex.is_match(current_symbol) {
            tokens.push(Token::Symbol(current_char));
            unlexed_tokens.advance();
        } else if self.regex.break_regex.is_match(current_symbol) {
            tokens.push(Token::Break);
            unlexed_tokens.advance();
        } else if self.regex.parentesis_regex.is_match(current_symbol) {
            tokens.push(Token::Parentesis(current_char));
            unlexed_tokens.advance();
        } else if self.regex.branching_regex.is_match(current_symbol) {
            tokens.push(Token::Bracket(current_char));
            unlexed_tokens.advance();
        } else if self.regex.param_regex.is_match(current_symbol) {
            tokens.push(Token::Param(current_char));
            unlexed_tokens.advance();
        } else if self.regex.char_regex.is_match(current_symbol) {
            let mut string = Vec::new();
            self.lex_string(&mut unlexed_tokens, &mut string);
            let ident = string.join("");
            tokens.push(Token::Ident(ident));
        } else if self.regex.number_regex.is_match(current_symbol) {
            let mut number = String::new();
            self.lex_number(&mut unlexed_tokens, &mut number);

            if number.contains("..") {
                let mut split = number.split("..");
                let start_range = split.next().expect("Expected a (half-open) range bounded inclusively below and exclusively above (`start..end`). Found no 'start'");
                let end_range = split.next().expect("Expected a (half-open) range bounded inclusively below and exclusively above (`start..end`). Found only 'start'");

                let start_range = start_range
                    .parse::<f32>()
                    .expect("could not parse start of the range.");
                let end_range = end_range
                    .parse::<f32>()
                    .expect("could not parse start of the range.");

                tokens.push(Token::Range(start_range..end_range));
            } else {
                let number = number.parse::<f32>().expect("could not parse number");
                tokens.push(Token::Number(number));
            }

            unlexed_tokens.advance();
        } else if self.regex.whitespace_regex.is_match(current_symbol) {
            tokens.push(Token::Space);
            unlexed_tokens.advance();
        } else {
            panic!("Unknown char: {current_symbol}")
        }

        self.lex_next_char(unlexed_tokens, tokens);
    }

    fn lex_string(&self, unlexed_tokens: &mut UnlexedTokens, chars: &mut Vec<String>) {
        if unlexed_tokens.finished() {
            return;
        }

        let current_token = unlexed_tokens.current_token();

        if self.regex.char_regex.is_match(current_token) {
            chars.push(current_token.to_string())
        } else {
            return;
        }

        unlexed_tokens.advance();

        self.lex_string(unlexed_tokens, chars)
    }

    fn lex_number(&self, unlexed_tokens: &mut UnlexedTokens, number: &mut String) {
        if unlexed_tokens.finished() {
            return;
        }

        let current_token = unlexed_tokens.current_token();

        if self.regex.number_regex.is_match(current_token) || current_token == "." {
            number.push_str(current_token);
        } else {
            unlexed_tokens.index -= 1;
            return;
        }

        unlexed_tokens.advance();

        self.lex_number(unlexed_tokens, number);
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Action names / variables
    Ident(String),
    // Constant(char),
    Number(f32),
    // A number range from x to y.
    Range(Range<f32>),
    // * | + | - | / | . | ,
    Symbol(char),
    // ( | )
    Param(char),
    // [ | ]
    Bracket(char),
    // ;
    Break,
    // { | }
    Parentesis(char),
    //Paramter(String)
    Space,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Ident(ident) => ident.to_string(),
            Token::Number(n) => n.to_string(),
            Token::Symbol(s) => s.to_string(),
            Token::Param(param) => param.to_string(),
            Token::Bracket(b) => b.to_string(),
            Token::Break => ";".to_string(),
            Token::Parentesis(p) => p.to_string(),
            Token::Space => " ".to_string(),
            Token::Range(range) => format!("{range:?}"),
        }
    }
}

pub enum Statements {
    Action(),
}
