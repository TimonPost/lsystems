use std::slice::Iter;

/*
    - Ignore constant symbols when context matching.
    - Take in mind [] when context matching
    - Perhaps transition to some linkedlinst

*/

/// A symbol in the alphabet defining an l-system.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Symbol {
    /// Variable symbols can be replaced and execute actions.
    Variable(char),
    /// Constant symbols can only perform actions.
    Constant(char),
    // A module is a symbol with a list of parameters.
    Module(char, Vec<char>),
}

pub enum Params {
    Param(char),
    Seperator(char),
}

/// A set of symbols containing both elements that can be replaced (variables) and those which cannot be replaced (constants or terminals).
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Alphabet {
    /// The symbols making up the alphabet.
    pub symbols: Vec<Symbol>,
    /// The number of symbols depend up on the depth of lsystem generation.
    pub generation: u8,
}

impl Alphabet {
    /// Creates an alphabet with the given capacity.
    fn with_capacity(capacity: usize, generation: u8) -> Alphabet {
        Alphabet {
            symbols: Vec::with_capacity(capacity),
            generation,
        }
    }

    /// Creates an alphabet from a string definition.
    /// This will panic if the given alphabet definition does not define a symbol for some character in the string.
    pub fn from_string<A: SymbolDefiner>(
        alphabet_string: String,
        generations: u8,
        alphabet_definition: &A,
    ) -> Alphabet {
        let mut alphabet = Alphabet::with_capacity(alphabet_string.len(), generations);

        alphabet_string
            .chars()
            .map(|c| alphabet_definition.into_symbol(c))
            .for_each(|letter| {
                alphabet.add_symbol(letter);
            });

        alphabet
    }

    pub fn to_string(&self) -> String {
        use std::fmt::Write;
        let mut out_string = String::with_capacity(self.symbols.len());

        for l in &self.symbols {
            match l {
                Symbol::Variable(v) => write!(out_string, "{v}").unwrap(),
                Symbol::Constant(c) => write!(out_string, "{c}").unwrap(),
                Symbol::Module(c, _) => write!(out_string, "{c}").unwrap(),
            };
        }
        out_string
    }

    pub fn add_symbol(&mut self, letter: Symbol) {
        self.symbols.push(letter);
    }

    pub fn iter(&self) -> Iter<'_, Symbol> {
        self.symbols.iter()
    }
}

/// Definer of alphabet symbols from chars.
pub trait SymbolDefiner {
    /// Returns the `Symbol` for the given char.
    fn into_symbol(&self, char: char) -> Symbol;
}

/// Default alphabet symbol definer, maps:
/// A-Z and 0-1 to `Symbol::Variable`
/// ∧, \\, /, &, '+', '-', '[', and ']' to `Symbol::Constant`
///
/// Using any other character with this definition will panic.
pub struct DefaultAlphabetSymbolDefiner;

impl SymbolDefiner for DefaultAlphabetSymbolDefiner {
    fn into_symbol(&self, char: char) -> Symbol {
        match char {
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
            | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'f' => {
                Symbol::Variable(char)
            }
            '0' | '1' => Symbol::Variable(char),
            '∧' | '\\' | '/' | '|' | '&' | '+' | '-' | '[' | ']' => Symbol::Constant(char),
            _ => panic!("Non supported char '{char}'"),
        }
    }
}

/// A string of symbols from Alphabet defining the initial state of the system.
pub type Axiom = &'static str;
