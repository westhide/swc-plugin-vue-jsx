use indexmap::IndexMap;
use swc_core::{
    common::util::take::Take,
    ecma::ast::{Expr, Ident, Module, ModuleItem, VarDecl, VarDeclKind},
};

use crate::hoist::decl_map::Declarator;

mod decl_map;

#[derive(Debug)]
pub struct Hoist<'a> {
    name: &'a str,
    decl_map: IndexMap<Expr, Ident>,
}

impl<'a> Hoist<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            decl_map: IndexMap::new(),
        }
    }

    pub fn get_or_decl(&mut self, expr: Expr) -> &Ident {
        let Self { name, decl_map } = self;

        decl_map.get_or_decl(expr, name)
    }

    pub fn get_var_decl(&mut self) -> Option<VarDecl> {
        let Self { decl_map, .. } = self;

        if decl_map.is_empty() {
            None
        } else {
            Some(VarDecl {
                kind: VarDeclKind::Const,
                decls: decl_map.decls(),
                ..Take::dummy()
            })
        }
    }

    pub fn add_to_module(&mut self, module: &mut Module) {
        if let Some(decl) = self.get_var_decl() {
            let Module { body, .. } = module;

            let mut idx = 0;

            for item in body.iter() {
                match item {
                    ModuleItem::ModuleDecl(_) => idx += 1,
                    _ => break,
                }
            }

            body.insert(idx, decl.into())
        }
    }
}
