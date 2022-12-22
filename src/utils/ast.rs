use swc_core::{
    common::util::take::Take,
    ecma::ast::{
        ArrayLit, Expr, ExprOrSpread, Ident, KeyValueProp, ObjectLit, Prop, PropOrSpread, SeqExpr,
    },
};

use crate::{
    constant::UNDEFINED,
    shared::{convert::Convert, transform::Transform},
};

/// ## Constant Expr
///
/// ---
pub fn is_undefined_ident(ident: &Ident) -> bool {
    ident.as_ref() == UNDEFINED
}

pub fn is_constant_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Ident(ident) => is_undefined_ident(ident),
        Expr::Array(ArrayLit { elems, .. }) => {
            elems.iter().all(|elem| {
                match elem {
                    None => true,
                    Some(ExprOrSpread {
                        spread: Some(_), ..
                    }) => false,
                    Some(ExprOrSpread { expr, .. }) => is_constant_expr(expr),
                }
            })
        },
        Expr::Object(ObjectLit { props, .. }) => {
            props.iter().all(|prop_or_spread| {
                match prop_or_spread {
                    PropOrSpread::Spread(_) => false,
                    PropOrSpread::Prop(box prop) => {
                        match prop {
                            Prop::Shorthand(ident) => is_undefined_ident(ident),
                            Prop::KeyValue(KeyValueProp { value, .. }) => is_constant_expr(value),
                            _ => false,
                        }
                    },
                }
            })
        },
        Expr::Seq(SeqExpr { exprs, .. }) => exprs.iter().all(|expr| is_constant_expr(expr)),
        _ => false,
    }
}
