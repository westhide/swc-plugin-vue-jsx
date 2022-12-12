use std::collections::HashMap;

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

#[derive(Debug)]
pub struct ImportHelper<'a> {
    module: *mut Module,
    /// HashMap<[path], HashMap<[name], [ident]>>
    ///  - e.g.
    /// import-helper { createVNode as _createVNode } from "vue" => <br>
    /// HashMap<["vue"], HashMap<["createVNode"], ["_createVNode"]<sup>Ident</sup>>>
    store: HashMap<&'a str, HashMap<&'a str, &'a Ident>>,
    /// Import Declaration should inject to Module
    injects: HashMap<&'a str, HashMap<&'a str, Ident>>,
}

impl<'a> ImportHelper<'a> {
    pub fn new(module: *mut Module) -> Self {
        let body = unsafe { &(*module).body };

        let mut store = HashMap::new();

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
                        match imported {
                            Some(ModuleExportName::Ident(imported)) => {
                                imports.insert(&*imported.sym, local);
                            },
                            None => {
                                imports.insert(&*local.sym, local);
                            },
                            Some(ModuleExportName::Str(_)) => {},
                        }
                    }
                })
            };
        });

        ImportHelper {
            module,
            store,
            injects: HashMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, name: &'a str, path: &'a str) -> &Ident {
        match self.store.entry(path).or_insert(HashMap::new()).get(name) {
            Some(&ident) => ident,
            None => {
                self.injects
                    .entry(path)
                    .or_insert(HashMap::new())
                    .entry(name)
                    .or_insert(private_ident!(name))
            },
        }
    }

    pub fn import_decls(self) -> Vec<ModuleItem> {
        self.injects
            .into_iter()
            .map(|(path, mapper)| {
                let import_decl = ImportDecl {
                    specifiers: mapper
                        .into_values()
                        .map(|local| {
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

    pub fn inject_import_decls(self) {
        if self.injects.is_empty() {
            return;
        }

        let module = self.module;

        let decls = self.import_decls();

        unsafe { (*module).body.splice(0..0, decls) };
    }
}
