use crate::{DefaultAlphabetSymbolDefiner, LSystemAction, SymbolDefiner};
use crate::{LSystem, Symbol};

struct DefaultAlphabet;

impl SymbolDefiner for DefaultAlphabet {
    fn into_symbol(&self, char: char) -> Symbol {
        match char {
            'F' => Symbol::Variable(char),
            'G' => Symbol::Variable(char),
            'X' => Symbol::Variable(char),
            '0' => Symbol::Variable(char),
            '1' => Symbol::Variable(char),
            '+' => Symbol::Constant(char),
            '-' => Symbol::Constant(char),
            'A' => Symbol::Constant(char),
            'B' => Symbol::Constant(char),
            '[' => Symbol::Constant(char),
            ']' => Symbol::Constant(char),
            'a' => Symbol::Module(char, vec![]),
            '(' => Symbol::Constant(char),
            ')' => Symbol::Constant(char),
            ',' => Symbol::Constant(char),
            _ => panic!("Non supported char '{char}'"),
        }
    }
}

struct ParameticAction;

impl<T> LSystemAction<T> for ParameticAction {
    fn trigger(&self) -> Symbol {
        Symbol::Module('a', vec!['x', 'y', 'z'])
    }

    fn execute(&self, symbol: &Symbol, context: &mut crate::ExecuteContext<T>) {
        if let Symbol::Module(name, params) = symbol {
            println!("{name} params: {:?}", params);
        }
    }
}

#[test]
fn parametic_rule() {
    let mut lsystem = LSystem::<(), DefaultAlphabet>::new("a(0,1,2)", DefaultAlphabet);
    lsystem.add_parametic_production_rule('a', |symbol, params| return Some("a(0+1,0+1,0+1)"));
    lsystem.add_action(ParameticAction);

    let alphabet = lsystem.generate(1);

    //assert_eq!(alphabet.symbols.len(), 14);
    assert_eq!(alphabet.to_string(), "a(0+1,0+1,0+1)");
}

#[test]
fn context_sensitive_rule() {
    let mut lsystem = LSystem::<()>::new("BAC", DefaultAlphabetSymbolDefiner);
    lsystem.add_context_sensitive_rule('A', |symbol, index, chars| {
        if chars[index - 1] == 'B' && chars[index + 1] == 'C' && symbol == 'A' {
            return Some("AA");
        } else {
            return None;
        }
    });

    let alphabet = lsystem.generate(1);
    assert_eq!(alphabet.symbols.len(), 4);
    assert_eq!(alphabet.to_string(), "BAAC");
}

#[test]
fn algae_test() {
    let mut lsystem = LSystem::<()>::new("A", DefaultAlphabetSymbolDefiner);
    lsystem.add_stochastic_rule('A', "AB");
    lsystem.add_stochastic_rule('B', "A");

    let alphabet = lsystem.generate(7);
    assert_eq!(alphabet.symbols.len(), 34);
    assert_eq!(alphabet.to_string(), "ABAABABAABAABABAABABAABAABABAABAAB");
}

#[test]
fn fractal_binary_tree_test() {
    let mut lsystem = LSystem::<()>::new("0", DefaultAlphabetSymbolDefiner);
    lsystem.add_stochastic_rule('1', "11");
    lsystem.add_stochastic_rule('0', "1[0]0");

    let alphabet = lsystem.generate(3);
    assert_eq!(alphabet.to_string(), "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0");
    assert_eq!(alphabet.symbols.len(), 34);
}

#[test]
fn koch_curve() {
    let mut lsystem = LSystem::<()>::new("F", DefaultAlphabetSymbolDefiner);
    lsystem.add_stochastic_rule('F', "F+F-F-F+F");

    let alphabet = lsystem.generate(3);
    assert_eq!(alphabet.to_string(), "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F");
    assert_eq!(alphabet.symbols.len(), 249);
}

#[test]
fn sierpinski_curve() {
    let mut lsystem = LSystem::<()>::new("F-G-G", DefaultAlphabetSymbolDefiner);
    lsystem.add_stochastic_rule('F', "F-G+F+G-F");
    lsystem.add_stochastic_rule('G', "GG");

    let alphabet = lsystem.generate(2);
    assert_eq!(
        alphabet.to_string(),
        "F-G+F+G-F-GG+F-G+F+G-F+GG-F-G+F+G-F-GGGG-GGGG"
    );
    assert_eq!(alphabet.symbols.len(), 45);
}

#[test]
fn dragon_curve() {
    let mut lsystem = LSystem::<()>::new("F", DefaultAlphabetSymbolDefiner);
    lsystem.add_stochastic_rule('F', "F+G");
    lsystem.add_stochastic_rule('G', "F-G");

    let alphabet = lsystem.generate(3);
    assert_eq!(alphabet.to_string(), "F+G+F-G+F+G-F-G");
    assert_eq!(alphabet.symbols.len(), 15);
}

#[test]
fn fractal_plant() {
    let mut lsystem = LSystem::<()>::new("X", DefaultAlphabetSymbolDefiner);

    lsystem.add_stochastic_rule('X', "F+[[X]-X]-F[-FX]+X");
    lsystem.add_stochastic_rule('F', "FF");

    let alphabet = lsystem.generate(2);
    assert_eq!(
        alphabet.to_string(),
        "FF+[[F+[[X]-X]-F[-FX]+X]-F+[[X]-X]-F[-FX]+X]-FF[-FFF+[[X]-X]-F[-FX]+X]+F+[[X]-X]-F[-FX]+X"
    );
    assert_eq!(alphabet.symbols.len(), 89);
}
