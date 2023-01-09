use indexmap::IndexMap;
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{Expr, Ident, VarDeclarator},
        utils::private_ident,
    },
};

pub type DeclMap = IndexMap<Expr, Ident>;

pub trait Declarator {
    fn get_or_decl(&mut self, expr: Expr, name: &str) -> &Ident;

    fn decls(&mut self) -> Vec<VarDeclarator>;
}

impl Declarator for DeclMap {
    fn get_or_decl(&mut self, expr: Expr, name: &str) -> &Ident {
        self.entry(expr).or_insert_with(|| private_ident!(name))
    }

    fn decls(&mut self) -> Vec<VarDeclarator> {
        self.drain(..)
            .map(|(expr, ident)| {
                VarDeclarator {
                    name: ident.into(),
                    init: Some(box expr),
                    ..Take::dummy()
                }
            })
            .collect()
    }
}
