use crate::util::{Location, Spannable};

pub enum Lit {
    Int { loc: Location, value: i32 },
    String { loc: Location, value: String },
    Float { loc: Location, value: f32 },
}

pub enum Op {
    Add,
    Sub,
    Div,
    Mult,
}

pub enum Expr {
    Identifier {
        loc: Location,
        name: String,
    },
    Literal(Lit),
    BinOp(Op, Box<Expr>, Box<Expr>),
    FunCall {
        loc: Location,
        name: String,
        args: Vec<Box<Expr>>,
    },
}

impl Spannable for Expr {
    fn get_loc(&self) -> Location {
        match self {
            Expr::Identifier { loc, .. }
            | Expr::Literal(Lit::Int { loc, .. })
            | Expr::Literal(Lit::String { loc, .. })
            | Expr::Literal(Lit::Float { loc, .. })
            | Expr::FunCall { loc, .. } => *loc,

            Expr::BinOp(_, expr, _) => expr.get_loc(),
        }
    }
}
