use std::collections::HashMap;

use crate::{
    abs::{Action, ActionParam},
    ExecuteContext, Symbol,
};

pub struct ActionResolver {
    pub actions: HashMap<(String, char), Box<dyn Fn(&Action) -> Option<Box<dyn LSystemAction>>>>,
}

impl ActionResolver {
    pub fn add_action_resolver<A: LSystemAction + 'static>(&mut self, trigger: Symbol) {
        let trigger_move = trigger.clone();
        let resolver: Box<dyn Fn(&Action) -> Option<Box<dyn LSystemAction>>> = Box::new(move |action| {
            let resolver_action = A::from_params(trigger_move.clone(),&action.params);
            let result = resolver_action.map(|a| Box::new(a) as Box<dyn LSystemAction>);

            result
        });

        if let Symbol::Constant(char) | Symbol::Variable(char) = trigger {
            self.actions.insert((A::name().to_owned(), char), resolver);
        }
    }

    pub fn resolve(&self, trigger: &Symbol, action: &Action) -> Option<Box<dyn LSystemAction>> {
        if let Symbol::Constant(char) | Symbol::Variable(char) = trigger {
            self.actions.get(&(action.name.clone(), *char)).and_then(|cb| cb(action))
        } else {
            None
        }
    }
}

pub trait LSystemAction {
    fn from_params(symbol: Symbol,params: &ParamsResolver) -> Option<Self>
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
    // Accepts a string with values separated by ','. Will try to resolve the params.
    pub fn from_string(params: String) -> Self {
        let mut action_params = vec![];
        for split in params.split(",") {
            if let Ok(value) = split.parse::<f32>() {
                action_params.push(ActionParam::Number(value))
            } else if let Ok(value) = split.parse::<usize>() {
                action_params.push(ActionParam::Number(value as f32))
            } else {
                action_params.push(ActionParam::Constant(split.to_string()))
            }
            // TODO: add support for expressions.
        }

        Self {
            params: action_params,
        }
    }

    pub fn get(&self, index: usize) -> Option<f32> {
        if let Some(param) = self.params.get(index) {
            self.action_param(param)
        } else {
            None
        }
    }

    fn action_param(&self, param: &ActionParam) -> Option<f32> {
        match param {
            ActionParam::Number(number) => Some(*number),
            ActionParam::Constant(_constant) => {
                panic!("The usage of constants/variables is not yet supported.")
            }
            ActionParam::Expression(kind) => match kind {
                crate::ExprKind::Binary(opt, lh, rh) => {
                    let lh = self.action_param(lh)?;
                    let rh = self.action_param(rh)?;

                    Some(match opt {
                        crate::BinOpKind::Add => lh + rh,
                        crate::BinOpKind::Sub => lh - rh,
                        crate::BinOpKind::Mul => lh * rh,
                        crate::BinOpKind::Div => lh / rh,
                        _ => {
                            panic!("The binary operation '{}' is not supported yet as action parameter.", opt.to_string());
                        }
                    })
                }
                crate::ExprKind::Random(range) => {
                    let mut rng = perchance::global();
                    let rand = rng.uniform_range_f32(range.clone());
                    Some(rand)
                }
            },
            ActionParam::None => None,
        }
    }
}
