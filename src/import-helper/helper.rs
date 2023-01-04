use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, Module, ModuleDecl,
            ModuleExportName, ModuleItem,
        },
        utils::private_ident,
    },
};

#[derive(Debug, Default)]
pub struct ImportHelper<'a> {
    /// HashMap<`path`, HashMap<`name`, `ident`>>
    ///  - e.g., import-helper { createVNode as _createVNode } from "vue"
    ///    - HashMap<"vue", HashMap<"createVNode", "_createVNode"<sup>[Ident]</sup>>>
    store: HashMap<&'a str, HashMap<&'a str, &'a Ident>>,

    loc: usize,
    /// Import Declaration should add to Module
    declaration: IndexMap<&'a str, IndexMap<&'a str, Ident>>,
}

impl<'a> ImportHelper<'a> {
    pub fn init(&mut self, module: *mut Module) {
        let Self { store, .. } = self;

        let body = unsafe { &(*module).body };

        let position = body.iter().position(|item: &ModuleItem| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                src, specifiers, ..
            })) = item
            {
                let imports = store.entry(&*src.value).or_insert_with(HashMap::new);

                specifiers.iter().for_each(|specifier: &ImportSpecifier| {
                    if let ImportSpecifier::Named(ImportNamedSpecifier {
                        local, imported, ..
                    }) = specifier
                    {
                        let name = match imported {
                            Some(ModuleExportName::Ident(imported)) => imported.as_ref(),
                            Some(ModuleExportName::Str(imported)) => &*imported.value,
                            None => local.as_ref(),
                        };

                        imports.insert(name, local);
                    }
                });

                false
            } else {
                true
            }
        });

        self.loc = position.unwrap_or(body.len() - 1)
    }

    pub fn get_or_insert(&mut self, name: &'a str, path: &'a str) -> &Ident {
        match self
            .store
            .entry(path)
            .or_insert_with(HashMap::new)
            .get(name)
        {
            Some(&ident) => ident,
            None => {
                self.declaration
                    .entry(path)
                    .or_insert_with(IndexMap::new)
                    .entry(name)
                    .or_insert_with_key(|&name| private_ident!(name))
            },
        }
    }

    fn import_decls(&mut self) -> Vec<ModuleItem> {
        self.declaration
            .drain(..)
            .map(|(path, mut mapper)| {
                let import_decl = ImportDecl {
                    specifiers: mapper
                        .into_iter()
                        .map(|(_, local)| {
                            ImportSpecifier::Named(ImportNamedSpecifier {
                                span: DUMMY_SP,
                                local,
                                imported: None,
                                is_type_only: Default::default(),
                            })
                        })
                        .collect(),
                    src: box path.into(),
                    ..Take::dummy()
                };

                ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl))
            })
            .collect()
    }

    pub fn inject(&mut self, module: &mut Module) {
        if !self.declaration.is_empty() {
            let loc = self.loc;
            let decls = self.import_decls();

            module.body.splice(loc..loc, decls);
        }
    }
}
