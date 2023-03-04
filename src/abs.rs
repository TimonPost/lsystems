use crate::action::ParamsResolver;

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

pub type Constant = String;
pub type Number = f32;

#[derive(PartialEq, Clone, Debug)]
pub enum ExprKind {
    Binary(BinOpKind, P<ActionParam>, P<ActionParam>),
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

#[derive(PartialEq, Clone, Debug)]
pub struct P<T: ?Sized + PartialEq + Clone> {
    ptr: Box<T>,
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
                let x = match x {
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
                };

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
