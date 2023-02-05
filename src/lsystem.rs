use std::{collections::HashMap, hash::Hash, vec};

use macaw::Vec3;

use crate::{
    lexer::Lexer,
    parser::{Action, Item},
    Alphabet, Axiom, DefaultAlphabetSymbolDefiner, Symbol, SymbolDefiner, Turtle,
    TurtleTranformStack,
};

/// Production consists of two strings, the predecessor and the successor.
/// For any symbol A which is a member of the set Alphabet which does not appear on the left hand side of a production in P,
/// the identity production A → A is assumed; these symbols are called constants or terminals.
pub struct StochasticProductionRule {
    predecessor: char,
    successor: &'static str,
}

impl StochasticProductionRule {
    pub fn new(predecessor: char, successor: &'static str) -> Self {
        Self {
            predecessor,
            successor,
        }
    }

    pub fn apply(&self, symbols: char) -> Option<&'static str> {
        if symbols == self.predecessor {
            Some(self.successor)
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct GenericStochasticProductionRule {
    predecessor: String,
    successor: String,
}

impl GenericStochasticProductionRule {
    fn new(predecessor: String, successor: String) -> Self {
        Self {
            predecessor,
            successor,
        }
    }

    fn apply(&self, symbols: String) -> Option<String> {
        if symbols == self.predecessor {
            Some(self.successor.clone())
        } else {
            None
        }
    }
}

/// Callback that defines a context sensitive rule.
/// * The symbol being matched.
/// * The index of the symbol being matched.
/// * The symbols buffer with all characters and their indexes.
///
/// With this callback one can write context sensitive grammar rules for the L-system.
pub type ContextSensitiveRuleCB = fn(char, usize, &[char]) -> std::option::Option<&'static str>;

/// A context sensitive production rule takes into account the context of other alphabet symbols.
pub struct ContextSensitiveProductionRule {
    rule_cb: ContextSensitiveRuleCB,
}

impl ContextSensitiveProductionRule {
    pub fn new(rule_cb: ContextSensitiveRuleCB) -> Self {
        Self { rule_cb }
    }

    pub fn apply(&self, symbols: char, index: usize, chars: &[char]) -> Option<&'static str> {
        (self.rule_cb)(symbols, index, chars)
    }
}

pub type ParameticRuleCB = fn(char, &[char]) -> std::option::Option<&'static str>;

pub struct ParametricProductionRule {
    rule_cb: ParameticRuleCB,
}

impl ParametricProductionRule {
    pub fn new(rule_cb: ParameticRuleCB) -> Self {
        Self { rule_cb }
    }

    pub fn apply(&self, symbol: char, params: &[char]) -> Option<&'static str> {
        (self.rule_cb)(symbol, params)
    }
}

/// An L-system or Lindenmayer system is a parallel rewriting system and a type of formal grammar.
/// An L-system consists of an alphabet of symbols that can be used to make strings,
/// a collection of production rules that expand each symbol into some larger string of symbols,
/// an initial "axiom" string from which to begin construction,
/// and a mechanism for translating the generated strings into geometric structures.
pub struct LSystem<T, A: SymbolDefiner = DefaultAlphabetSymbolDefiner> {
    pub axiom: String,
    stochastic_rules: HashMap<char, StochasticProductionRule>,
    generic_stochastic_rules: HashMap<String, GenericStochasticProductionRule>,
    context_sensitive_rules: HashMap<char, ContextSensitiveProductionRule>,
    parametric_production_rules: HashMap<char, ParametricProductionRule>,
    actions: HashMap<Symbol, Box<dyn LSystemAction<T>>>,
    alphabet_definer: A,
    pub name: String,
    pub action_rules: Vec<(String, Action)>,
}

impl<A: SymbolDefiner, T> LSystem<T, A> {
    pub fn new(axiom: impl ToString, alphabet_definer: A) -> Self {
        Self {
            axiom: axiom.to_string(),
            stochastic_rules: HashMap::new(),
            generic_stochastic_rules: HashMap::new(),
            actions: HashMap::new(),
            alphabet_definer,
            context_sensitive_rules: HashMap::new(),
            parametric_production_rules: HashMap::new(),
            name: String::new(),
            action_rules: vec![],
        }
    }

    pub fn run<B, C: LSystemAction<B>, F: Fn(&String, &Action) -> Option<C>>(
        &mut self,
        resolver: &F,
        alphabet: &Alphabet,
    ) -> ExecuteContext<B> {
        let mut context = ExecuteContext::<B>::new();

        for token in alphabet.iter() {
            match token {
                Symbol::Variable(var) => {
                    if let Some((interpret, by)) =
                        self.action_rules.iter().find(|x| x.0 == var.to_string())
                    {
                        if let Some(action) = resolver(interpret, by) {
                            action.execute(token, &mut context)
                        }
                    }
                }
                Symbol::Constant(constant) => {
                    if let Some((interpret, by)) = self
                        .action_rules
                        .iter()
                        .find(|x| x.0 == constant.to_string())
                    {
                        if let Some(action) = resolver(interpret, by) {
                            action.execute(token, &mut context)
                        }
                    }
                }
                Symbol::Module(x, params) => todo!(),
            }
        }
        context
    }

    /// The rules of the L-system grammar are applied iteratively starting from the initial state.
    /// As many rules as possible are applied simultaneously, per iteration
    pub fn generate(&self, generations: u8) -> Alphabet {
        let mut result = String::new();

        // Apply grammar rules recursive.
        // Can be parralelized.
        Self::apply_rules_recursive(
            self.axiom.clone(),
            &mut result,
            &self.stochastic_rules,
            &self.context_sensitive_rules,
            &self.parametric_production_rules,
            &self.generic_stochastic_rules,
            generations,
        );

        // Kindof syntax tree containing the letters with the generated symbols.
        // Not the most efficient, could perhaps be constructed during recursive rule applying,
        // or removed entirely.
        Alphabet::from_string(result, generations, &self.alphabet_definer)
    }

    fn recursively_iterate_params(symbols: &Vec<char>, symbol_index: &mut usize) -> Vec<char> {
        let mut params = Vec::new();
        loop {
            *symbol_index += 1;
            let current_symbol = symbols[*symbol_index];

            if current_symbol == ')' {
                *symbol_index += 1;
                return params;
            } else if current_symbol != ',' {
                params.push(current_symbol);
            }
        }
    }

    fn apply_rules_recursive(
        symbols: String,
        string_result: &mut String,
        grammar_rules: &HashMap<char, StochasticProductionRule>,
        context_sensitive_rules: &HashMap<char, ContextSensitiveProductionRule>,
        parametic_rules: &HashMap<char, ParametricProductionRule>,
        generic_rules: &HashMap<String, GenericStochasticProductionRule>,
        generations_left: u8,
    ) {
        // If no more generations to generate, stop, and append leave symbols.
        if generations_left == 0 {
            string_result.push_str(&symbols);
        }

        let symbols = symbols.chars().collect::<Vec<char>>();

        let mut symbol_index = 0;

        if generations_left == 0 || symbols.len() == 0 {
            return;
        }

        loop {
            let symbol = symbols[symbol_index];
            let next_symbol = symbols.get(symbol_index + 1);

            // Check if current symbol is start of parametric module.
            if let Some('(') = next_symbol {
                symbol_index += 2;
                let args = Self::recursively_iterate_params(&symbols, &mut symbol_index);

                if let Some(rule) = parametic_rules.get(&symbol) {
                    if let Some(result) = rule.apply(symbol, &args) {
                        string_result.push_str(result);
                    }
                }
                symbol_index += 1;
                if symbol_index > symbols.len() - 1 {
                    break;
                }
            }

            // Check if current rule is a stochastic production rule.
            if let Some(rule) = grammar_rules.get(&symbol) {
                if let Some(result) = rule.apply(symbol) {
                    Self::apply_rules_recursive(
                        result.to_string(),
                        string_result,
                        grammar_rules,
                        context_sensitive_rules,
                        parametic_rules,
                        generic_rules,
                        generations_left - 1,
                    );
                }
            } else if let Some(rule) = context_sensitive_rules.get(&symbol) {
                // Check if current rule is a context sensitive production rule.
                if let Some(result) = rule.apply(symbol, symbol_index, symbols.as_slice()) {
                    Self::apply_rules_recursive(
                        result.to_string(),
                        string_result,
                        grammar_rules,
                        context_sensitive_rules,
                        parametic_rules,
                        generic_rules,
                        generations_left - 1,
                    );
                }
            } else if let Some(rule) = generic_rules.get(&symbol.to_string()) {
                // Check if current rule is a context sensitive production rule.
                if let Some(result) = rule.apply(symbol.to_string()) {
                    Self::apply_rules_recursive(
                        result.to_string(),
                        string_result,
                        grammar_rules,
                        context_sensitive_rules,
                        parametic_rules,
                        generic_rules,
                        generations_left - 1,
                    );
                }
            } else {
                // If there is no rule for the symbol, then its the end of recurion, append symbol.
                string_result.push_str(&symbol.to_string());
            }

            symbol_index += 1;

            if symbol_index > symbols.len() - 1 {
                break;
            }
        }
    }

    pub fn execute<'a>(
        &self,
        origin: Vec3,
        scale: f32,
        rotation: Vec3,
        alphabet: &Alphabet,
    ) -> Vec<T> {
        let mut context = ExecuteContext {
            elements: Vec::<T>::new(),
            transform_stack: TurtleTranformStack::new(),
            turtle: Turtle::new(),
        };

        context.turtle.scale(scale);
        context.turtle.set_origin(origin);
        context.turtle.rotate_z(rotation.z);
        context.turtle.rotate_x(rotation.x);
        context.turtle.rotate_y(rotation.y);

        for letter in alphabet.iter() {
            if let Some(action) = self.actions.get(letter) {
                action.execute(letter, &mut context);
            }
        }

        context.elements
    }

    pub fn add_action<D: LSystemAction<T> + 'static>(&mut self, action: D) {
        self.actions.insert(action.trigger(), Box::from(action));
    }

    pub fn add_stochastic_rule(&mut self, predecessor: char, successor: &'static str) {
        self.stochastic_rules.insert(
            predecessor,
            StochasticProductionRule::new(predecessor, successor),
        );
    }

    pub fn add_dynamic_stochastic_rule(&mut self, predecessor: String, successor: String) {
        self.generic_stochastic_rules.insert(
            predecessor.clone(),
            GenericStochasticProductionRule::new(predecessor, successor),
        );
    }

    pub fn add_context_sensitive_rule(
        &mut self,
        predecessor: char,
        rule_cb: ContextSensitiveRuleCB,
    ) {
        self.context_sensitive_rules
            .insert(predecessor, ContextSensitiveProductionRule::new(rule_cb));
    }

    pub fn add_parametic_production_rule(&mut self, predecessor: char, rule_cb: ParameticRuleCB) {
        self.parametric_production_rules
            .insert(predecessor, ParametricProductionRule::new(rule_cb));
    }
}

pub trait LSystemAction<T> {
    /// Returns the trigger letter for this action.
    fn trigger(&self) -> Symbol;

    /// Executes the given action.
    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext<T>);
}

pub struct LSystemBuilder<T, A: SymbolDefiner = DefaultAlphabetSymbolDefiner> {
    lsystem: LSystem<T, A>,
}

impl<T, A: SymbolDefiner> LSystemBuilder<T, A> {
    pub fn new(axiom: &'static str, alphabet_definer: A) -> Self {
        Self {
            lsystem: LSystem::new(axiom, alphabet_definer),
        }
    }

    pub fn with_stochastic_rules(mut self, rules: &[(char, &'static str)]) -> Self {
        for (predecessor, successor) in rules {
            self.lsystem.add_stochastic_rule(*predecessor, successor);
        }
        self
    }

    pub fn with_context_sensitive_rules(
        mut self,
        predecessor: char,
        rules: &[ContextSensitiveRuleCB],
    ) -> Self {
        for rule in rules {
            self.lsystem.add_context_sensitive_rule(predecessor, *rule);
        }
        self
    }

    pub fn with_action<LA: LSystemAction<T> + 'static>(mut self, action: LA) -> Self {
        self.lsystem.add_action(action);
        self
    }

    pub fn build(self) -> LSystem<T, A> {
        self.lsystem
    }
}

pub trait LSystemDefinition<T> {
    fn new() -> Self;
    fn name(&self) -> &'static str;
    fn lsystem(&self) -> &LSystem<T>;
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct LSystemKey(&'static str, u8);

pub struct LSystemFactory<T> {
    lsystems_alphabets: HashMap<LSystemKey, Alphabet>,
    lsystems_meshes: HashMap<LSystemKey, Vec<T>>,
}

impl<T> LSystemFactory<T> {
    pub fn new() -> Self {
        Self {
            lsystems_alphabets: HashMap::new(),
            lsystems_meshes: HashMap::new(),
        }
    }

    pub fn generate<L: LSystemDefinition<T>>(
        &mut self,
        definition: &L,
        generation: u8,
    ) -> &Alphabet {
        let key = LSystemKey(definition.name(), generation);

        self.lsystems_alphabets.entry(key).or_insert_with(|| {
            let lsystem = definition.lsystem();
            lsystem.generate(generation)
        })
    }

    pub fn render<L: LSystemDefinition<T>>(
        &mut self,
        definition: &L,
        generation: u8,
        position: Vec3,
        rotation: Vec3,
        scale: f32,
    ) -> Option<&Vec<T>> {
        let key = LSystemKey(definition.name(), generation);
        if let Some(alphabet) = self.lsystems_alphabets.get(&key) {
            let entities = self.lsystems_meshes.entry(key).or_insert_with(|| {
                let lsystem = definition.lsystem();
                lsystem.execute(position, scale, rotation, &alphabet)
            });

            return Some(entities);
        }
        None
    }
}

pub struct ExecuteContext<T> {
    /// Elements generated by the lsystem.
    pub elements: Vec<T>,
    /// Used for saving transforms during lsystem generation.
    pub transform_stack: TurtleTranformStack,
    /// Used for turlte graphics.
    pub turtle: Turtle,
}

impl<T> ExecuteContext<T> {
    pub fn new() -> Self {
        Self {
            elements: vec![],
            transform_stack: TurtleTranformStack::new(),
            turtle: Turtle::new(),
        }
    }
}
