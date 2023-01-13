use swc_core::{
    common::{
        comments::{Comment, Comments},
        BytePos,
    },
    ecma::{
        ast::{Expr, ExprOrSpread, Ident},
        utils::private_ident,
    },
};

use crate::{
    args, regex_set,
    shared::{expr::ExprExtend, ident::IdentExtend},
    VueJSX,
};

pub trait Context {
    fn is_unresolved(&self, ident: &Ident) -> bool;

    fn is_custom_element(&self, text: &str) -> bool;

    fn import_from_vue(&mut self, name: &'static str) -> Ident;

    fn get_ident(&mut self, name: &'static str) -> Ident;

    fn add_pure_comment(&self, pos: BytePos);

    fn add_leading_comment(&self, pos: BytePos, cmt: Comment);

    fn static_threshold(&self) -> usize;

    fn hoist_to_module(&mut self, expr: Expr) -> Ident;

    fn hoist_to_scope(&mut self, expr: Expr) -> Ident;

    fn invoke(&mut self, func: &'static str, args: Vec<ExprOrSpread>) -> Expr {
        self.import_from_vue(func).call(args)
    }

    fn resolve<T: Into<Expr>>(&mut self, func: &'static str, target: T) -> Expr
    where
        Self: Sized,
    {
        self.invoke(func, args![target]).hoist_to_scope(self)
    }

    fn create_element_vnode(&mut self, args: Vec<ExprOrSpread>) -> Expr {
        self.invoke("createElementVNode", args)
    }

    fn create_vnode(&mut self, args: Vec<ExprOrSpread>) -> Expr {
        self.invoke("createVNode", args)
    }

    fn create_static_vnode(&mut self, args: Vec<ExprOrSpread>) -> Expr
    where
        Self: Sized,
    {
        self.invoke("createStaticVNode", args).hoist_to_module(self)
    }

    fn create_text_vnode(&mut self, args: Vec<ExprOrSpread>) -> Expr
    where
        Self: Sized,
    {
        self.invoke("createTextVNode", args).hoist_to_module(self)
    }

    fn merge_props(&mut self, args: Vec<ExprOrSpread>) -> Expr {
        self.invoke("mergeProps", args)
    }

    fn with_directive(&mut self, args: Vec<ExprOrSpread>) -> Expr {
        self.invoke("withDirectives", args)
    }
}

impl<'a> Context for VueJSX<'a> {
    fn is_unresolved(&self, ident: &Ident) -> bool {
        ident.span.has_mark(self.unresolved_mark)
    }

    fn is_custom_element(&self, text: &str) -> bool {
        regex_set!(&self.opts.custom_element_patterns).is_match(text)
    }

    fn import_from_vue(&mut self, name: &'static str) -> Ident {
        self.import_helper.get_or_import(name, "vue").clone()
    }

    fn get_ident(&mut self, name: &'static str) -> Ident {
        self.ident_map
            .entry(name)
            .or_insert_with(|| private_ident!(name))
            .clone()
    }

    fn add_pure_comment(&self, pos: BytePos) {
        self.comments.add_pure_comment(pos)
    }

    fn add_leading_comment(&self, pos: BytePos, cmt: Comment) {
        self.comments.add_leading(pos, cmt)
    }

    fn static_threshold(&self) -> usize {
        self.opts.static_threshold
    }

    fn hoist_to_module(&mut self, expr: Expr) -> Ident {
        self.module_hoist.get_or_decl(expr).clone()
    }

    fn hoist_to_scope(&mut self, expr: Expr) -> Ident {
        self.scope_hoist.get_or_decl(expr).clone()
    }
}
