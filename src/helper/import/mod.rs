pub mod builder;

use std::collections::HashMap;

use swc_core::ecma::ast::{
    Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleExportName,
    ModuleItem, Str,
};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Key<'a> {
    id: &'a Ident,
    src: &'a Str,
}

pub struct ImportHelper<'a> {
    pub store: HashMap<Key<'a>, &'a Ident>,
}

impl<'a> ImportHelper<'a> {
    pub fn new(module: &'a Module) -> Self {
        let mut store = HashMap::new();

        module.body.iter().for_each(|item: &ModuleItem| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                specifiers, src, ..
            })) = item
            {
                specifiers.iter().for_each(|specifier: &ImportSpecifier| {
                    if let ImportSpecifier::Named(ImportNamedSpecifier {
                        local, imported, ..
                    }) = specifier
                    {
                        match imported {
                            Some(ModuleExportName::Ident(imported)) => {
                                store.insert(Key { id: imported, src }, local);
                            },
                            None => {
                                store.insert(Key { id: local, src }, local);
                            },
                            Some(ModuleExportName::Str(_)) => {},
                        }
                    }
                })
            };
        });

        Self { store }
    }
}
