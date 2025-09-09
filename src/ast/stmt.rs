use crate::ast::expr::Expr;
use crate::util::{Location, Spannable};

use crate::type_::Type;

struct Param {
    loc: Location,
    name: String,
}

enum Statement {
    VarDecl {
        loc: Location,
        name: String,
        expr: Expr,
    },
    ParamDecl(Param),
    FunDecl {
        loc: Location,
        name: String,
        params: Vec<Param>,
    },
}

impl Spannable for Statement {
    fn get_loc(&self) -> Location {
        match self {
            Statement::VarDecl { loc, .. }
            | Statement::ParamDecl(Param { loc, .. })
            | Statement::FunDecl { loc, .. } => *loc,
        }
    }
}
