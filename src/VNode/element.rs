use swc_core::ecma::ast::{JSXElement, JSXElementName, JSXMemberExpr};

use crate::{
    constant::{CLASS, REF, STYLE},
    patch_flag::PatchFlag,
    shared::{parse::Parse, state::State},
    utils::pattern::is_native_tag,
    vnode::{
        props::{Key, Prop},
        VNode,
    },
};

/// ## [Tag]
///
/// ---
#[derive(Debug)]
pub enum Tag<'a> {
    Native(&'a str),
    Mixin(&'a str),
    MemberExpr(&'a JSXMemberExpr),
}

impl<'a> Tag<'a> {
    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native(_))
    }
}

impl<'a> Parse<&'a JSXElementName> for Tag<'a> {
    fn parse(element_name: &'a JSXElementName) -> Self {
        match element_name {
            JSXElementName::Ident(ident) => {
                match &*ident.sym {
                    name if is_native_tag(name) => Self::Native(name),
                    name => Self::Mixin(name),
                }
            },
            JSXElementName::JSXMemberExpr(member_expr) => Self::MemberExpr(member_expr),
            JSXElementName::JSXNamespacedName(_) => {
                panic!("Tag.parse(): JSXNamespacedName Element is not supported")
            },
        }
    }
}

/// ## [Element]
///
/// ---
#[derive(Debug)]
pub struct Element<'a> {
    pub tag: Tag<'a>,
    pub props: Vec<Prop<'a>>,
    pub children: Vec<VNode<'a>>,
}

impl<'a> Element<'a> {
    // fn check_v_model<S: State>(&self, state: &S) {
    //     if self.props.has_v_model() && self.not_allow_v_model(state) {
    //         panic!("v-model can only be used on <input>, <textarea> and <select> elements")
    //     }
    // }

    pub fn analyze<S: State>(&mut self, state: &S) {
        println!("{:?}", self.patch_flag());
    }

    pub fn patch_flag(&self) -> isize {
        let mut flag = 0isize;

        let mut need_patch = false;
        let mut hydrate_event = false;
        let mut has_patch_attr = false;

        for Prop { key, value } in &self.props {
            if matches!(key, Key::Directive(_)) {
                need_patch = true;
                continue;
            }

            if value.is_dyn_expr() {
                match key {
                    Key::Spread => return PatchFlag::FULL_PROPS,
                    Key::Spec(REF) => need_patch = true,
                    Key::Spec(CLASS) => flag |= PatchFlag::CLASS,
                    Key::Spec(STYLE) => flag |= PatchFlag::STYLE,
                    Key::Event(_) => hydrate_event = true,
                    Key::Attr(_) | Key::NSAttr(_) => has_patch_attr = true,
                    _ => {},
                }
            }
        }

        if hydrate_event {
            flag |= PatchFlag::HYDRATE_EVENTS
        }

        if has_patch_attr {
            flag |= PatchFlag::PROPS
        }

        if PatchFlag::is_non_prop(&flag) && need_patch {
            flag |= PatchFlag::NEED_PATCH
        }

        flag
    }
}

impl<'a> Parse<&'a JSXElement> for Element<'a> {
    fn parse(element: &'a JSXElement) -> Self {
        let JSXElement {
            opening, children, ..
        } = element;

        let tag = Tag::parse(&opening.name);

        let props = Vec::<Prop>::parse(&opening.attrs);

        let children = children
            .iter()
            .map(|child| VNode::parse(child))
            .filter(|vnode| !vnode.is_empty_text())
            .collect();

        Self {
            tag,
            props,
            children,
        }
    }
}
