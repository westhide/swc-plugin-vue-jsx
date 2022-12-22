use indexmap::IndexMap;
use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{ArrowExpr, BlockStmt, BlockStmtOrExpr, Expr, Ident},
        utils::{private_ident, ExprFactory},
    },
};

use crate::shared::declare::Declare;

#[derive(Debug, Default)]
pub struct ScopeHelper {
    declaration: IndexMap<Expr, Ident>,
}

impl ScopeHelper {
    pub fn get_or_insert(&mut self, expr: Expr) -> &Ident {
        self.declaration
            .entry(expr)
            .or_insert_with(|| private_ident!("_v"))
    }

    pub fn create_render_expr(&mut self, expr: Expr) -> Expr {
        let Self { declaration } = self;

        if declaration.is_empty() {
            expr
        } else {
            let decls = declaration.decl();
            let return_stmt = expr.into_return_stmt();

            ArrowExpr {
                body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![decls.into(), return_stmt.into()],
                }),
                ..Take::dummy()
            }
            .as_iife()
            .into()
        }
    }
}
