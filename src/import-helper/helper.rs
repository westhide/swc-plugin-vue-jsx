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
    module: Option<*mut Module>,
    /// HashMap<`path`, HashMap<`name`, `ident`>>
    ///  - e.g., import-helper { createVNode as _createVNode } from "vue"
    ///    - HashMap<"vue", HashMap<"createVNode", "_createVNode"<sup>[Ident]</sup>>>
    store: HashMap<&'a str, HashMap<&'a str, &'a Ident>>,
    /// Import Declaration should inject to Module
    injects: IndexMap<&'a str, IndexMap<&'a str, Ident>>,
}

impl<'a> ImportHelper<'a> {
    pub fn init(&mut self, module: *mut Module) {
        self.module = Some(module);

        let Self { store, .. } = self;

        let body = unsafe { &(*module).body };

        body.iter().for_each(|item: &ModuleItem| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                specifiers, src, ..
            })) = item
            {
                let imports = store.entry(&*src.value).or_insert(HashMap::new());

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
                })
            };
        });
    }

    pub fn get_or_insert(&mut self, name: &'a str, path: &'a str) -> &Ident {
        match self.store.entry(path).or_insert(HashMap::new()).get(name) {
            Some(&ident) => ident,
            None => {
                self.injects
                    .entry(path)
                    .or_insert(IndexMap::new())
                    .entry(name)
                    .or_insert(private_ident!(name))
            },
        }
    }

    fn import_decls(&mut self) -> Vec<ModuleItem> {
        self.injects
            .drain(..)
            .map(|(path, mut mapper)| {
                let import_decl = ImportDecl {
                    specifiers: mapper
                        .drain(..)
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

    pub fn inject_import_decls(&mut self) {
        if self.injects.is_empty() {
            return;
        }

        let Some(module) = self.module else { panic!("Forbidden: inject_import_decls() before init ImportHelper") };

        let decls = self.import_decls();

        unsafe { (*module).body.splice(0..0, decls) };
    }
}
