use std::collections::HashMap;

use crate::{
    abs::{Action, ActionParam},
    ExecuteContext, Symbol,
};

pub struct ActionResolver {
    pub actions: HashMap<String, Box<dyn Fn(&Action) -> Option<Box<dyn LSystemAction>>>>,
}

impl ActionResolver {
    pub fn add_action_resolver<A: LSystemAction + 'static>(&mut self) {
        let resolver: Box<dyn Fn(&Action) -> Option<Box<dyn LSystemAction>>> = Box::new(|action| {
            let resolver_action = A::from_params(&action.params);
            let result = resolver_action.map(|a| Box::new(a) as Box<dyn LSystemAction>);

            result
        });

        self.actions.insert(A::name().to_owned(), resolver);
    }

    pub fn resolve(&self, action: &Action) -> Option<Box<dyn LSystemAction>> {
        self.actions.get(&action.name).and_then(|cb| cb(action))
    }
}

pub trait LSystemAction {
    fn from_params(params: &ParamsResolver) -> Option<Self>
    where
        Self: Sized;

    fn name() -> &'static str
    where
        Self: Sized;

    /// Returns the trigger letter for this action.
    fn trigger(&self) -> Symbol;

    /// Executes the given action.
    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext);
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParamsResolver {
    pub params: Vec<ActionParam>,
}

impl ParamsResolver {
    pub fn number(&self, index: usize) -> Option<f32> {
        if let Some(ActionParam::Number(number)) = self.params.get(index) {
            Some(*number)
        } else {
            None
        }
    }
}
