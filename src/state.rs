use swc_core::{
    common::{
        comments::{Comment, Comments},
        BytePos,
    },
    ecma::{
        ast::{Expr, Ident},
        utils::private_ident,
    },
};

use crate::{regex_set, VueJSX};

pub trait State<'s> {
    fn is_custom_element(&self, text: &str) -> bool;

    fn import_from_vue(&mut self, name: &'s str) -> Ident;

    fn hoist_expr(&mut self, expr: Expr) -> Ident;

    fn scope_expr(&mut self, expr: Expr) -> Ident;

    fn get_private_ident(&mut self, name: &'s str) -> Ident;

    fn add_pure_comment(&self, pos: BytePos);

    fn add_leading_comment(&self, pos: BytePos, cmt: Comment);

    fn static_threshold(&self) -> usize;
}

impl<'s, C: Comments> State<'s> for VueJSX<'s, C> {
    fn is_custom_element(&self, text: &str) -> bool {
        regex_set!(&self.opts.custom_element_patterns).is_match(text)
    }

    fn import_from_vue(&mut self, name: &'s str) -> Ident {
        self.import_helper.get_or_insert(name, "vue").clone()
    }

    fn hoist_expr(&mut self, expr: Expr) -> Ident {
        self.hoist.get_or_insert(expr).clone()
    }

    fn scope_expr(&mut self, expr: Expr) -> Ident {
        self.scope.get_or_insert(expr).clone()
    }

    fn get_private_ident(&mut self, name: &'s str) -> Ident {
        self.private_idents
            .entry(name)
            .or_insert_with_key(|&name| private_ident!(name))
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
}
