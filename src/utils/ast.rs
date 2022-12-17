use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            ArrayLit, CallExpr, Expr, ExprOrSpread, Ident, KeyValueProp, ObjectLit, Prop,
            PropOrSpread, SeqExpr,
        },
        utils::ExprFactory,
    },
};

use crate::{
    constant::UNDEFINED,
    shared::{convert::Convert, state::State},
};

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

pub fn create_vnode_expr<'s, S: State<'s>>(args: Vec<ExprOrSpread>, state: &mut S) -> Expr {
    let callee = state.get_vue_import("create_vnode").clone().as_callee();

    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee,
        args,
        type_args: None,
    })
}

pub fn create_merge_props<'s, S: State<'s>>(
    spreads: &Vec<&Expr>,
    prop_obj: ObjectLit,
    state: &mut S,
) -> Expr {
    let callee = state.get_vue_import("mergeProps").clone().as_callee();

    let mut args: Vec<ExprOrSpread> = spreads.iter().map(|&expr| expr.clone().into()).collect();

    args.push(prop_obj.as_arg());

    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee,
        args,
        type_args: None,
    })
}
