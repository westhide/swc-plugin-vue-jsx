use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, JSXElementName, JSXMemberExpr, Lit},
        utils::ExprFactory,
    },
};

use crate::{
    shared::{convert::Convert, transform::Transform},
    state::State,
    utils::pattern::is_native_tag,
};

/// ## [Tag]
///
/// ---
#[derive(Debug)]
pub enum Tag<'a> {
    Native(&'a str),
    /// Component or Custom element
    Ext(&'a str),
    MemberExpr(&'a JSXMemberExpr),
}

impl<'a> Tag<'a> {
    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native(_))
    }

    pub fn is_component<'s, S: State<'s>>(&self, state: &S) -> bool {
        match self {
            Self::Native(_) => false,
            Self::MemberExpr(_) => true,
            Self::Ext(name) => !state.is_custom_element(name),
        }
    }
}

impl<'a> Transform<'a, Tag<'a>> for JSXElementName {
    fn transform(&'a self) -> Tag<'a> {
        match self {
            JSXElementName::Ident(ident) => {
                match ident.as_ref() {
                    name if is_native_tag(name) => Tag::Native(name),
                    name => Tag::Ext(name),
                }
            },
            JSXElementName::JSXMemberExpr(member_expr) => Tag::MemberExpr(member_expr),
            JSXElementName::JSXNamespacedName(_) => panic!("Forbidden: JSXNamespacedName Element"),
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Tag<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        match self {
            Self::Native(&ref name) => Lit::from(name).into(),
            Self::Ext(name) => {
                let resolve_component = state.import_from_vue("resolveComponent");

                let resolve_expr = resolve_component.as_call(DUMMY_SP, vec![name.as_arg()]);

                state.scope_expr(resolve_expr).into()
            },
            Self::MemberExpr(member_expr) => todo!(),
        }
    }
}
