use indexmap::IndexMap;
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{Expr, Ident, Module, ModuleItem, VarDecl, VarDeclKind, VarDeclarator},
        utils::private_ident,
    },
};

use crate::utils::ast::is_import_decl_module_item;

#[derive(Debug)]
pub struct Hoist<'s> {
    id: &'s str,
    declaration: IndexMap<Expr, Ident>,
}

impl<'s> Hoist<'s> {
    pub fn new(id: &'s str) -> Self {
        Self {
            id,
            declaration: IndexMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, expr: Expr) -> &Ident {
        let Self { id, declaration } = self;

        declaration
            .entry(expr)
            .or_insert_with(|| private_ident!(*id))
    }

    fn inject_location(body: &[ModuleItem]) -> usize {
        body.iter()
            .position(|item| !is_import_decl_module_item(item))
            .unwrap_or(body.len() - 1)
    }

    pub fn decl(&mut self) -> Option<VarDecl> {
        let Self { declaration, .. } = self;

        if declaration.is_empty() {
            None
        } else {
            Some(VarDecl {
                kind: VarDeclKind::Const,
                decls: declaration
                    .drain(..)
                    .map(|(expr, ident)| {
                        VarDeclarator {
                            name: ident.into(),
                            init: Some(box expr),
                            ..Take::dummy()
                        }
                    })
                    .collect(),
                ..Take::dummy()
            })
        }
    }

    pub fn inject(&mut self, module: &mut Module) {
        if let Some(decl) = self.decl() {
            let Module { body, .. } = module;

            let loc = Self::inject_location(body);

            body.insert(loc, decl.into())
        }
    }
}
