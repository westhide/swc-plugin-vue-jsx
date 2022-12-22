use swc_core::ecma::ast::{Expr, JSXAttrValue, JSXExpr, Lit};

use crate::{
    constant::EMPTY_STRING_EXPR,
    shared::{convert::Convert, transform::Transform},
    state::State,
    utils::ast::is_constant_expr,
    vnode::{element::Element, fragment::Fragment},
};

/// ## [Value]
#[derive(Debug)]
pub enum Value<'a> {
    Lit(&'a Lit),
    /// TODO: [is_constant_expr] Non-strict
    Const(&'a Expr),
    Expr(&'a Expr),
    Element(Box<Element<'a>>),
    Fragment(Box<Fragment<'a>>),
    Empty,
}

impl<'a> Value<'a> {
    fn specialize_expr(expr: &'a Expr) -> Self {
        match expr {
            Expr::Lit(lit) => Self::Lit(lit),
            expr if is_constant_expr(expr) => Self::Const(expr),
            expr => Self::Expr(expr),
        }
    }

    pub fn is_dyn(&self) -> bool {
        matches!(self, Self::Expr(_) | Self::Element(_) | Self::Fragment(_))
    }
}

impl<'a> Transform<'a, Value<'a>> for JSXAttrValue {
    fn transform(&'a self) -> Value<'a> {
        match self {
            JSXAttrValue::Lit(lit) => Value::Lit(lit),
            JSXAttrValue::JSXExprContainer(container) => {
                match &container.expr {
                    JSXExpr::Expr(expr) => Value::specialize_expr(expr),
                    JSXExpr::JSXEmptyExpr(_) => {
                        panic!("JSXAttrValue::JSXExprContainer can not empty")
                    },
                }
            },
            JSXAttrValue::JSXElement(box element) => Value::Element(box element.transform()),
            JSXAttrValue::JSXFragment(fragment) => Value::Fragment(box fragment.transform()),
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Value<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        match self {
            Self::Expr(&ref expr) => expr.clone(),
            Self::Lit(&ref lit) => lit.clone().into(),
            Self::Const(&ref expr) => expr.clone(),
            Self::Empty => EMPTY_STRING_EXPR,
            Self::Element(element) => element.convert(state),
            Self::Fragment(fragment) => fragment.convert(state),
        }
    }
}
