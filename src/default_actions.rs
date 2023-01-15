///! This module defines some very common actions that can be reused by various lsystems.
///! For example many lsystems use rotation or transform stacking.
use crate::{ExecuteContext, LSystemAction, Symbol};

/// Rotation action arround the z axis.
pub struct RotateZAction(pub f32, pub char);

impl<T> LSystemAction<T> for RotateZAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext<T>) {
        context.turtle.rotate_z(self.0);
    }
}

/// Rotation action arround the x axis.
pub struct RotateXAction(pub f32, pub char);

impl<T> LSystemAction<T> for RotateXAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext<T>) {
        context.turtle.rotate_x(self.0);
    }
}

/// Rotation action arround the z axis.
pub struct RotateYAction(pub f32, pub char);

impl<T> LSystemAction<T> for RotateYAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(self.1)
    }

    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext<T>) {
        context.turtle.rotate_y(self.0);
    }
}

/// L systems commonly saves transforms while generating actions.
/// This action saves the current turret transform.
/// The transform can be popped with `PopTransformFromStackAction`
/// This action triggers on `]`.
pub struct PushTranformToStackAction;

impl<T> LSystemAction<T> for PushTranformToStackAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant('[')
    }

    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext<T>) {
        context.transform_stack.push(context.turtle.clone());
    }
}

/// L systems commonly saves transforms while generating actions.
/// This action pops a saved transform at the end of a recursive path.
/// This action triggers on `]`.
pub struct PopTransformFromStackAction;

impl<T> LSystemAction<T> for PopTransformFromStackAction {
    fn trigger(&self) -> Symbol {
        Symbol::Constant(']')
    }

    fn execute(&self, symbol: &Symbol, context: &mut ExecuteContext<T>) {
        context.turtle = context.transform_stack.pop();
    }
}
