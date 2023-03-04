///! This module defines some very common actions that can be reused by various lsystems.
///! For example many lsystems use rotation or transform stacking.
use crate::{action::LSystemAction, action::ParamsResolver, ExecuteContext, Symbol};

/// Rotation action arround the z axis.
pub struct RotateZAction(pub f32, pub char);

impl LSystemAction for RotateZAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.turtle.rotate_z(self.0);
    }

    fn from_params(params: &ParamsResolver) -> Option<Self> {
        let z = params.number(0).unwrap();

        println!("Interpret {} ({})", "RotateZAction", z);

        Some(RotateZAction(z, 'a'))
    }

    fn name() -> &'static str {
        "RotateZAction"
    }
}

/// Rotation action arround the x axis.
pub struct RotateXAction(pub f32, pub char);

impl LSystemAction for RotateXAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.turtle.rotate_x(self.0);
    }

    fn from_params(params: &ParamsResolver) -> Option<Self> {
        let x = params.number(0).unwrap();

        println!("Interpret {} ({})", "RotateXAction", x);

        Some(RotateXAction(x, 'a'))
    }

    fn name() -> &'static str {
        "RotateXAction"
    }
}

/// Rotation action arround the z axis.
pub struct RotateYAction(pub f32, pub char);

impl LSystemAction for RotateYAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.turtle.rotate_y(self.0);
    }

    fn from_params(params: &ParamsResolver) -> Option<Self> {
        let y = params.number(0).unwrap();

        println!("Interpret {} ({})", "RotateYAction", y);

        Some(RotateYAction(y, 'a'))
    }

    fn name() -> &'static str {
        "RotateYAction"
    }
}

/// L systems commonly saves transforms while generating actions.
/// This action saves the current turret transform.
/// The transform can be popped with `PopTransformFromStackAction`
/// This action triggers on `]`.
pub struct PushTranformToStackAction;

impl LSystemAction for PushTranformToStackAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant('[')
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.transform_stack.push(context.turtle.clone());
    }

    fn from_params(_params: &ParamsResolver) -> Option<Self> {
        println!("PushTranformToStackAction");
        Some(PushTranformToStackAction)
    }

    fn name() -> &'static str {
        "PushTranformToStackAction"
    }
}

/// L systems commonly saves transforms while generating actions.
/// This action pops a saved transform at the end of a recursive path.
/// This action triggers on `]`.
pub struct PopTransformFromStackAction;

impl LSystemAction for PopTransformFromStackAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(']')
    }

    fn execute(&self, _symbol: &Symbol, context: &mut ExecuteContext) {
        context.turtle = context.transform_stack.pop();
    }

    fn from_params(_params: &ParamsResolver) -> Option<Self> {
        println!("PushTranformToStackAction");
        Some(PopTransformFromStackAction)
    }

    fn name() -> &'static str {
        "PushTranformToStackAction"
    }
}
