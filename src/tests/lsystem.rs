use macaw::Quat;

use crate::action::ActionResolver;
use crate::default_actions::RotateXAction;
use crate::lexer::Lexer;
use crate::{action::*, parser::*};
use crate::{DefaultAlphabetSymbolDefiner, SymbolDefiner};
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

impl LSystemAction for ParameticAction {
    fn trigger(&self) -> Symbol {
        Symbol::Module('a', vec!['x', 'y', 'z'])
    }

    fn execute(&self, symbol: &Symbol, _context: &mut crate::ExecuteContext) {
        if let Symbol::Module(name, params) = symbol {
            println!("{name} params: {:?}", params);
        }
    }

    fn from_params(symbol: Symbol,_params: &ParamsResolver) -> Option<Self>
    where
        Self: Sized,
    {
        Some(ParameticAction)
    }

    fn name() -> &'static str {
        "ParameticAction"
    }
}

#[test]
fn parametric_rule() {
    let mut lsystem = LSystem::<DefaultAlphabet>::new("a(0,1,2)", DefaultAlphabet);
    lsystem.add_parametic_production_rule('a', |_symbol, _params| {
        return Some("a(0+1,0+1,0+1)".into());
    });

    let alphabet = lsystem.generate(1);

    assert_eq!(alphabet.to_string(), "a(0+1,0+1,0+1)");
}

#[test]
fn context_sensitive_rule() {
    let mut lsystem = LSystem::new("BAC", DefaultAlphabetSymbolDefiner);
    lsystem.add_context_sensitive_rule("A", |symbol, index, chars| {
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
    let mut lsystem = LSystem::new("A", DefaultAlphabetSymbolDefiner);
    lsystem.add_rule('A', "AB");
    lsystem.add_rule('B', "A");

    let alphabet = lsystem.generate(7);
    assert_eq!(alphabet.symbols.len(), 34);
    assert_eq!(alphabet.to_string(), "ABAABABAABAABABAABABAABAABABAABAAB");
}

#[test]
fn fractal_binary_tree_test() {
    let mut lsystem = LSystem::new("0", DefaultAlphabetSymbolDefiner);
    lsystem.add_rule('1', "11");
    lsystem.add_rule('0', "1[0]0");

    let alphabet = lsystem.generate(3);
    assert_eq!(alphabet.to_string(), "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0");
    assert_eq!(alphabet.symbols.len(), 34);
}

#[test]
fn koch_curve() {
    let mut lsystem = LSystem::new("F", DefaultAlphabetSymbolDefiner);
    lsystem.add_rule('F', "F+F-F-F+F");

    let alphabet = lsystem.generate(3);
    assert_eq!(alphabet.to_string(), "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F");
    assert_eq!(alphabet.symbols.len(), 249);
}

#[test]
fn sierpinski_curve() {
    let mut lsystem = LSystem::new("F-G-G", DefaultAlphabetSymbolDefiner);
    lsystem.add_rule('F', "F-G+F+G-F");
    lsystem.add_rule('G', "GG");

    let alphabet = lsystem.generate(2);
    assert_eq!(
        alphabet.to_string(),
        "F-G+F+G-F-GG+F-G+F+G-F+GG-F-G+F+G-F-GGGG-GGGG"
    );
    assert_eq!(alphabet.symbols.len(), 45);
}

#[test]
fn dragon_curve() {
    let mut lsystem = LSystem::new("F", DefaultAlphabetSymbolDefiner);
    lsystem.add_rule('F', "F+G");
    lsystem.add_rule('G', "F-G");

    let alphabet = lsystem.generate(3);
    assert_eq!(alphabet.to_string(), "F+G+F-G+F+G-F-G");
    assert_eq!(alphabet.symbols.len(), 15);
}

#[test]
fn fractal_plant() {
    let mut lsystem = LSystem::new("X", DefaultAlphabetSymbolDefiner);

    lsystem.add_rule('X', "F+[[X]-X]-F[-FX]+X");
    lsystem.add_rule('F', "FF");

    let alphabet = lsystem.generate(2);
    assert_eq!(
        alphabet.to_string(),
        "FF+[[F+[[X]-X]-F[-FX]+X]-F+[[X]-X]-F[-FX]+X]-FF[-FFF+[[X]-X]-F[-FX]+X]+F+[[X]-X]-F[-FX]+X"
    );
    assert_eq!(alphabet.symbols.len(), 89);
}

#[test]
fn parse_simple_lsystem_from_script() {
    let definition = format!(
        "   lsystem KochCurve {{
            axiom F;
        }}
    ",
    );

    let lexer = Lexer::new();

    let lex = lexer.lex(definition);
    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let lsystem = LSystemParser::parse(item);

    assert_eq!(lsystem.axiom, "F");
    assert_eq!(lsystem.name, "KochCurve");
}

#[test]
fn parse_lsystem_from_script_and_generate() {
    let definition = format!(
        "lsystem KochCurve {{
            axiom F;

            replace F by F+F-F-F+F;
        }}
    ",
    );

    let lexer = Lexer::new();

    let lex = lexer.lex(definition);
    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let lsystem = LSystemParser::parse(item);
    let alphabet = lsystem.generate(3);

    assert_eq!(lsystem.axiom, "F");
    assert_eq!(lsystem.name, "KochCurve");

    assert_eq!(
        alphabet.to_string(),
        "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F"
    );
}

#[test]
fn parse_lsystem_from_script_and_action() {
    let definition = format!(
        "lsystem KochCurve {{
            axiom F;

            interpret F as RotateXAction(10);
        }}
    ",
    );

    let lexer = Lexer::new();

    let lex = lexer.lex(definition);
    let tokens = LexedTokens::new(lex);

    let item = parse(tokens);

    let mut lsystem = LSystemParser::parse(item);
    let alphabet = lsystem.generate(2);

    let mut resolver = ActionResolver {
        actions: Default::default(),
    };
    resolver.add_action_resolver::<RotateXAction>();

    let context = lsystem.run(&resolver, &alphabet);

    assert_eq!(
        context.turtle.rotation(),
        Quat::from_mat4(&macaw::Mat4::from_rotation_x(10.0))
    );
}
