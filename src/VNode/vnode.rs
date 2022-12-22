use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            ArrayLit, Expr, ExprOrSpread, JSXElement, JSXElementChild, JSXExpr, JSXExprContainer,
            JSXFragment, JSXSpreadChild,
        },
        utils::ExprFactory,
    },
};

use crate::{
    constant::NULL_EXPR,
    shared::{convert::Convert, transform::Transform},
    state::State,
    utils::code_emitter::CodeEmitter,
    vnode::{element::Element, fragment::Fragment, text::Text},
};

#[derive(Debug)]
pub enum VNode<'a> {
    Text(Box<Text<'a>>),
    Element(Box<Element<'a>>),
    Expr(&'a Expr),
    Spread(&'a Expr),
    Fragment(Box<Fragment<'a>>),
}

impl<'a> VNode<'a> {
    pub fn is_static(&self) -> bool {
        match self {
            Self::Text(_) => true,
            Self::Element(box element) => element.is_static,
            Self::Expr(_) => false,
            Self::Spread(_) => false,
            Self::Fragment(fragment) => fragment.is_static,
        }
    }

    pub fn stringify(&self, count: &mut usize) -> String {
        match self {
            Self::Text(text) => text.content.clone(),
            Self::Element(element) => CodeEmitter::emit(element.raw),
            Self::Fragment(box Fragment { children, .. }) => {
                *count += children.len();

                children
                    .iter()
                    .map(|child| child.stringify(count))
                    .collect()
            },
            _ => panic!("can not stringify dyn vnode"),
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
        VNode::Fragment(box self.transform())
    }
}

impl<'a> Transform<'a, Option<VNode<'a>>> for JSXElementChild {
    fn transform(&'a self) -> Option<VNode<'a>> {
        match self {
            JSXElementChild::JSXText(jsx_text) => {
                jsx_text.transform().map(|text| VNode::Text(box text))
            },
            JSXElementChild::JSXExprContainer(container) => Some(container.transform()),
            JSXElementChild::JSXSpreadChild(spread_child) => Some(spread_child.transform()),
            JSXElementChild::JSXElement(box element) => Some(element.transform()),
            JSXElementChild::JSXFragment(fragment) => Some(fragment.transform()),
        }
    }
}

impl<'a> Transform<'a, Vec<VNode<'a>>> for [JSXElementChild] {
    fn transform(&'a self) -> Vec<VNode<'a>> {
        self.iter().filter_map(Transform::transform).collect()
    }
}

impl<'a, 's> Convert<'s, ExprOrSpread> for VNode<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> ExprOrSpread {
        match self {
            Self::Text(text) => text.convert(state).as_arg(),
            Self::Element(element) => element.convert(state).as_arg(),
            Self::Expr(&ref expr) => expr.clone().as_arg(),
            Self::Spread(&ref expr) => {
                ExprOrSpread {
                    spread: Some(DUMMY_SP),
                    expr: box expr.clone(),
                }
            },
            Self::Fragment(fragment) => fragment.convert(state).as_arg(),
        }
    }
}

fn stringify_static<'s, S: State<'s>>(
    elems: &mut Vec<Option<ExprOrSpread>>,
    static_nodes: &mut Vec<&VNode>,
    state: &mut S,
) {
    let mut count = static_nodes.len();
    if count >= state.static_vnode_threshold() {
        let inner_html: String = static_nodes
            .iter()
            .map(|vnode| vnode.stringify(&mut count))
            .collect();

        let static_vnode = state.import_from_vue("createStaticVNode");

        let hoist_expr = static_vnode.as_call(DUMMY_SP, vec![inner_html.as_arg(), count.as_arg()]);

        elems.push(Some(state.hoist_expr(hoist_expr).as_arg()))
    } else {
        elems.extend(static_nodes.iter().map(|vnode| Some(vnode.convert(state))));
    }

    static_nodes.truncate(0)
}

impl<'a, 's> Convert<'s, Expr> for [VNode<'a>] {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        if self.is_empty() {
            NULL_EXPR
        } else {
            let static_threshold = state.static_vnode_threshold();

            let mut elems: Vec<Option<ExprOrSpread>> = Vec::with_capacity(self.len());

            let mut static_nodes = Vec::new();

            self.iter().for_each(|vnode| {
                if vnode.is_static() {
                    static_nodes.push(vnode)
                } else {
                    stringify_static(&mut elems, &mut static_nodes, state);

                    elems.push(Some(vnode.convert(state)))
                }
            });

            stringify_static(&mut elems, &mut static_nodes, state);

            ArrayLit {
                span: DUMMY_SP,
                elems,
            }
            .into()
        }
    }
}
