use std::ops::{Deref, Range};

use crate::{action::ParamsResolver, Symbol};

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
    Replace(String, String),
    Interpret(Constant, Action),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Action {
    pub name: String,
    pub params: ParamsResolver,
}

impl Action {
    pub fn new(name: String, params: Vec<ActionParam>) -> Self {
        Self {
            name: name.into(),
            params: ParamsResolver { params },
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

impl ActionParam {
    pub fn to_string(&self) -> String {
        match self {
            ActionParam::Number(number) => number.to_string(),
            ActionParam::Constant(c) => c.to_owned(),
            ActionParam::Expression(e) => e.to_string(),
            ActionParam::None => todo!(),
        }
    }
}

pub type Constant = String;
pub type Number = f32;

#[derive(PartialEq, Clone, Debug)]
pub enum ExprKind {
    Binary(BinOpKind, P<ActionParam>, P<ActionParam>),
    Random(Range<f32>),
}

impl ExprKind {
    pub fn to_string(&self) -> String {
        match self {
            ExprKind::Binary(op, lh, rh) => {
                let op = op.to_string();

                let lh = lh.ptr.to_string();
                let rh = rh.ptr.to_string();

                format!("{op}{lh}{rh}")
            }
            ExprKind::Random(range) => {
                format!("{range:?}")
            }
        }
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

impl BinOpKind {
    pub fn to_string(&self) -> String {
        match self {
            BinOpKind::Add => "+",
            BinOpKind::Sub => "-",
            BinOpKind::Mul => "*",
            BinOpKind::Div => "/",
            BinOpKind::Rem => "%",
            BinOpKind::BitXor => "^",
            BinOpKind::BitAnd => "&",
            BinOpKind::BitOr => "|",
            BinOpKind::Lt => "<",
            BinOpKind::Le => "<=",
            BinOpKind::Ne => "!=",
            BinOpKind::Ge => ">=",
            BinOpKind::Gt => ">",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct P<T: ?Sized + PartialEq + Clone> {
    ptr: Box<T>,
}

impl<T: ?Sized + PartialEq + Clone> Deref for P<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.ptr
    }
}

impl<T: PartialEq + Clone> P<T> {
    pub fn new(ptr: T) -> Self {
        Self { ptr: Box::new(ptr) }
    }
}

pub enum ExpressionKind {}

#[derive(PartialEq, Clone, Debug)]
pub enum ReplaceExprKind {
    Binary(BinOpKind, P<ReplaceKind>, P<ReplaceKind>),
}

impl ToString for ReplaceExprKind {
    fn to_string(&self) -> String {
        match self {
            ReplaceExprKind::Binary(x, y, z) => {
                let x = x.to_string();

                let y = y.ptr.to_string();
                let z = z.ptr.to_string();

                format!("{y}{x}{z}")
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ReplaceKind {
    Number(Number),
    Constant(Constant),
    Expression(ReplaceExprKind),
}

impl ToString for ReplaceKind {
    fn to_string(&self) -> String {
        match self {
            ReplaceKind::Number(n) => n.to_string(),
            ReplaceKind::Constant(c) => c.to_string(),
            ReplaceKind::Expression(e) => e.to_string(),
        }
    }
}
