use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{ArrowExpr, BlockStmt, BlockStmtOrExpr, Expr},
        utils::ExprFactory,
    },
};

use crate::{context::Context, hoist::Hoist};

pub trait ExprExtend {
    fn hoist_to_module<'a>(self, ctx: &mut impl Context<'a>) -> Expr;

    fn hoist_to_scope<'a>(self, ctx: &mut impl Context<'a>) -> Expr;

    fn with_hoist(self, hoist: &mut Hoist) -> Expr;
}

impl ExprExtend for Expr {
    fn hoist_to_module<'a>(self, ctx: &mut impl Context<'a>) -> Expr {
        ctx.hoist_to_module(self).into()
    }

    fn hoist_to_scope<'a>(self, ctx: &mut impl Context<'a>) -> Expr {
        ctx.hoist_to_scope(self).into()
    }

    fn with_hoist(self, hoist: &mut Hoist) -> Expr {
        match hoist.get_var_decl() {
            Some(decl) => {
                ArrowExpr {
                    body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                        span: DUMMY_SP,
                        stmts: vec![decl.into(), self.into_return_stmt().into()],
                    }),
                    ..Take::dummy()
                }
                .as_iife()
                .into()
            },
            None => self,
        }
    }
}
