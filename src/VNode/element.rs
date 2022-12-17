use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{Expr, JSXElement, JSXElementName, JSXMemberExpr},
        utils::ExprFactory,
    },
};

use crate::{
    shared::{convert::Convert, state::State, transform::Transform},
    utils::{
        ast::{create_vnode_expr, string_lit_expr},
        pattern::is_native_tag,
    },
    vnode::{prop::PropStore, VNode},
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
            JSXElementName::JSXNamespacedName(_) => {
                panic!("Tag.parse(): JSXNamespacedName Element is not supported")
            },
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Tag<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        match self {
            Self::Native(name) => string_lit_expr(name),
            Self::Ext(name) => {
                todo!()
            },
            Self::MemberExpr(member_expr) => todo!(),
        }
    }
}

/// ## [Element]
///
/// ---
#[derive(Debug)]
pub struct Element<'a> {
    pub tag: Tag<'a>,
    pub prop_store: PropStore<'a>,
    pub children: Vec<VNode<'a>>,

    pub raw: &'a JSXElement,

    pub patch_flag: isize,
    pub is_const_props: bool,
    pub is_static: bool,
}

impl<'a> Element<'a> {
    // fn check_v_model<S: State>(&self, state: &S) {
    //     if self.props.has_v_model() && self.not_allow_v_model(state) {
    //         panic!("v-model can only be used on <input>, <textarea> and <select> elements")
    //     }
    // }

    // TODO: static custom element tag
    pub fn is_static(is_const_props: bool, tag: &Tag, children: &[VNode]) -> bool {
        is_const_props && tag.is_native() && children.iter().all(VNode::is_static)
    }
}

impl<'a> Transform<'a, Element<'a>> for JSXElement {
    fn transform(&'a self) -> Element<'a> {
        let Self {
            opening, children, ..
        } = self;

        let tag = opening.name.transform();

        let prop_store = opening.attrs.transform();

        let children: Vec<VNode> = children.iter().filter_map(Transform::transform).collect();

        let (patch_flag, is_const_props) = prop_store.inform();

        let is_static = Element::is_static(is_const_props, &tag, &children);

        Element {
            tag,
            prop_store,
            children,
            raw: self,

            patch_flag,
            is_const_props,
            is_static,
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Element<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        println!("{:?}", self.tag);
        println!("{:#?}", self.prop_store);
        println!("patch_flag:{:?}", self.patch_flag);
        println!("is_const_props:{:?}", self.is_const_props);
        println!("is_static:{:?}", self.is_static);

        let Self {
            tag, prop_store, ..
        } = self;

        let tag_arg = tag.convert(state).as_arg();

        let prop_obj = prop_store.convert(state);
        // TODO: spreads, models, directives
        //       dynamic key

        // if spreads.is_empty() {
        //     Expr::Object(prop_obj)
        // } else {
        //     create_merge_props(spreads, prop_obj, state)
        // }

        create_vnode_expr(vec![tag_arg, prop_arg], state)
    }
}
