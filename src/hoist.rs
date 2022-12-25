use indexmap::IndexMap;
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{Expr, Ident, Module, ModuleDecl, ModuleItem},
        utils::private_ident,
    },
};

use crate::shared::declare::Declare;

#[derive(Debug, Default)]
pub struct Hoist {
    declaration: IndexMap<Expr, Ident>,
}

impl Hoist {
    fn not_import_decl(item: &ModuleItem) -> bool {
        !matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_)))
    }

    pub fn get_or_insert(&mut self, expr: Expr) -> &Ident {
        self.declaration
            .entry(expr)
            .or_insert_with(|| private_ident!("_hoisted_"))
    }

    pub fn inject(&mut self, module: &mut Module) {
        let Self { declaration } = self;

        if declaration.is_empty() {
            return;
        }

        let index = module
            .body
            .iter()
            .position(Self::not_import_decl)
            .unwrap_or(module.body.len() - 1);

        let decl = declaration.decl();

        module.body.insert(index, decl.into())
    }
}
