use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{ArrowExpr, BlockStmt, BlockStmtOrExpr, Expr, Ident},
        utils::ExprFactory,
    },
};

use crate::hoist::Hoist;

#[derive(Debug)]
pub struct Scope<'s> {
    hoist: Hoist<'s>,
}

impl<'s> Scope<'s> {
    pub fn new(id: &'s str) -> Self {
        Self {
            hoist: Hoist::new(id),
        }
    }

    pub fn get_or_insert(&mut self, expr: Expr) -> &Ident {
        self.hoist.get_or_insert(expr)
    }

    pub fn create_render_expr(&mut self, expr: Expr) -> Expr {
        if let Some(decl) = self.hoist.decl() {
            ArrowExpr {
                body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![decl.into(), expr.into_return_stmt().into()],
                }),
                ..Take::dummy()
            }
            .as_iife()
            .into()
        } else {
            expr
        }
    }
}
