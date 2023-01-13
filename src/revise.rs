use swc_helper_jsx_transform::{
    attr::{key::Key, Attr},
    element::{tag::Tag, Element},
    fragment::Fragment,
    vnode::VNode,
};

use crate::{context::Context, utils::is::is_directive};

pub trait Revise {
    fn revise<C: Context>(&mut self, ctx: &mut C);
}

impl<T: Revise> Revise for [T] {
    fn revise<C: Context>(&mut self, ctx: &mut C) {
        self.iter_mut().for_each(|item| item.revise(ctx))
    }
}

impl<'a> Revise for Tag<'a> {
    fn revise<C: Context>(&mut self, ctx: &mut C) {
        if let Self::Extra(ident) = self {
            let name = &*ident.sym;

            if ctx.is_custom_element(name) {
                *self = Self::Native(name)
            }
        }
    }
}

fn has_directive(attrs: &[Attr]) -> bool {
    attrs.iter().any(|Attr { key, .. }| {
        if let Key::Attr(name) = key && is_directive(name) {
            true
        } else {
            false
        }
    })
}

fn has_dyn_children(children: &[VNode]) -> bool {
    !children.iter().all(VNode::is_static)
}

impl<'a> Revise for Element<'a> {
    fn revise<C: Context>(&mut self, ctx: &mut C) {
        let Self {
            tag,
            attrs,
            children,
            is_static,
            ..
        } = self;

        tag.revise(ctx);

        children.revise(ctx);

        if *is_static && (has_dyn_children(children) || has_directive(attrs)) {
            *is_static = false
        }
    }
}

impl<'a> Revise for Fragment<'a> {
    fn revise<C: Context>(&mut self, ctx: &mut C) {
        self.children.revise(ctx)
    }
}

impl<'a> Revise for VNode<'a> {
    fn revise<C: Context>(&mut self, ctx: &mut C) {
        match self {
            Self::Element(element) => element.revise(ctx),
            Self::Fragment(fragment) => fragment.revise(ctx),
            _ => {},
        }
    }
}
