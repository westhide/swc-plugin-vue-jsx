#![feature(box_syntax)]
#![feature(box_patterns)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod helper;
mod regex_macro;

use swc_core::{
    common::chain,
    ecma::{
        ast::{Module, Program},
        transforms::base::hygiene::hygiene,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut},
    },
    plugin::{metadata::TransformPluginProgramMetadata as Metadata, plugin_transform},
};

#[plugin_transform]
pub fn process_transform(program: Program, _: Metadata) -> Program {
    program.fold_with(&mut chain!(as_folder(VueJSX), hygiene()))
}

use helper::import::ImportHelper;

pub struct VueJSX;

impl VisitMut for VueJSX {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let import_helper = ImportHelper::new(module);
        println!("========{:?}", import_helper.store.keys())
    }
}
