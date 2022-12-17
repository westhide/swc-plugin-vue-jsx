use swc_core::ecma::ast::{
    Expr, JSXElement, JSXElementChild, JSXExpr, JSXExprContainer, JSXFragment, JSXSpreadChild,
    JSXText,
};

use crate::{
    shared::{convert::Convert, state::State, transform::Transform},
    utils::clean_jsx_text::clean_jsx_text,
    vnode::element::Element,
};

#[derive(Debug)]
pub enum VNode<'a> {
    Text(String),
    Element(Box<Element<'a>>),
    Expr(&'a Expr),
    Spread(&'a Expr),
    Fragment(Vec<VNode<'a>>),
}

impl<'a> VNode<'a> {
    pub fn is_static(&self) -> bool {
        match self {
            Self::Text(_) => true,
            Self::Element(box element) => element.is_static,
            Self::Expr(_) => false,
            Self::Spread(_) => false,
            Self::Fragment(fragment) => fragment.iter().all(VNode::is_static),
        }
    }
}

impl<'a> Transform<'a, Option<VNode<'a>>> for JSXText {
    fn transform(&'a self) -> Option<VNode<'a>> {
        let text = clean_jsx_text(&self.value);

        if text.is_empty() {
            None
        } else {
            Some(VNode::Text(text))
        }
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXElement {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Element(box self.transform())
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXExprContainer {
    fn transform(&'a self) -> VNode<'a> {
        match &self.expr {
            JSXExpr::JSXEmptyExpr(_) => panic!("JSXExprContainer can not empty"),
            // TODO: specialize Expr::Lit
            JSXExpr::Expr(expr) => VNode::Expr(expr),
        }
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXSpreadChild {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Spread(&self.expr)
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXFragment {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Fragment(
            self.children
                .iter()
                .filter_map(Transform::transform)
                .collect(),
        )
    }
}

impl<'a> Transform<'a, Option<VNode<'a>>> for JSXElementChild {
    fn transform(&'a self) -> Option<VNode<'a>> {
        match self {
            JSXElementChild::JSXText(jsx_text) => jsx_text.transform(),
            JSXElementChild::JSXExprContainer(container) => Some(container.transform()),
            JSXElementChild::JSXSpreadChild(spread_child) => Some(spread_child.transform()),
            JSXElementChild::JSXElement(box element) => Some(element.transform()),
            JSXElementChild::JSXFragment(fragment) => Some(fragment.transform()),
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for VNode<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        match self {
            Self::Element(element) => element.convert(state),
            _ => {
                todo!()
            },
        }
    }
}
