use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{CallExpr, Expr, ExprOrSpread, Ident},
        utils::ExprFactory,
    },
};

pub trait IdentExtend {
    fn call(self, args: Vec<ExprOrSpread>) -> Expr;
}

impl IdentExtend for Ident {
    fn call(self, args: Vec<ExprOrSpread>) -> Expr {
        Expr::Call(CallExpr {
            args,
            callee: self.as_callee(),
            ..Take::dummy()
        })
    }
}
