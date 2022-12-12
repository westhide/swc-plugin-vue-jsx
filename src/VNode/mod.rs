use swc_core::ecma::ast::{
    Expr, JSXElement, JSXElementChild, JSXExpr, JSXExprContainer, JSXFragment, JSXSpreadChild,
    JSXText,
};

use crate::{
    constant::EMPTY_STR,
    shared::{parse::Parse, state::State},
    vnode::element::Element,
};

mod element;
mod props;

#[derive(Debug)]
pub enum VNode<'a> {
    Text(&'a str),
    Element(Box<Element<'a>>),
    Expr(&'a Expr),
    Spread(&'a Expr),
    Fragment(Vec<VNode<'a>>),
}

impl<'a> VNode<'a> {
    pub fn is_empty_text(&self) -> bool {
        matches!(self, VNode::Text(EMPTY_STR))
    }
}

impl<'a> VNode<'a> {
    pub fn analyze<S: State>(&mut self, state: &'a S) {
        match self {
            Self::Element(elm) => elm.analyze(state),
            _ => {
                todo!()
            },
        }
    }
}

impl<'a> Parse<&'a JSXText> for VNode<'a> {
    fn parse(jsx_text: &'a JSXText) -> Self {
        Self::Text(jsx_text.value.trim())
    }
}

impl<'a> Parse<&'a JSXElement> for VNode<'a> {
    fn parse(element: &'a JSXElement) -> Self {
        Self::Element(box Element::parse(element))
    }
}

impl<'a> Parse<&'a JSXExprContainer> for VNode<'a> {
    fn parse(container: &'a JSXExprContainer) -> Self {
        match &container.expr {
            JSXExpr::JSXEmptyExpr(_) => panic!("JSXExprContainer can not empty"),
            JSXExpr::Expr(expr) => Self::Expr(expr),
        }
    }
}

impl<'a> Parse<&'a JSXSpreadChild> for VNode<'a> {
    fn parse(spread_child: &'a JSXSpreadChild) -> Self {
        Self::Spread(&spread_child.expr)
    }
}

impl<'a> Parse<&'a JSXFragment> for VNode<'a> {
    fn parse(fragment: &'a JSXFragment) -> Self {
        Self::Fragment(
            fragment
                .children
                .iter()
                .map(|child| Self::parse(child))
                .filter(|vnode| !vnode.is_empty_text())
                .collect(),
        )
    }
}

impl<'a> Parse<&'a JSXElementChild> for VNode<'a> {
    fn parse(child: &'a JSXElementChild) -> Self {
        match child {
            JSXElementChild::JSXText(jsx_text) => Self::parse(jsx_text),
            JSXElementChild::JSXExprContainer(container) => Self::parse(container),
            JSXElementChild::JSXSpreadChild(spread_child) => Self::parse(spread_child),
            JSXElementChild::JSXElement(box element) => Self::parse(element),
            JSXElementChild::JSXFragment(fragment) => Self::parse(fragment),
        }
    }
}
