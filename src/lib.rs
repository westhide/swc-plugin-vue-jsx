#![feature(box_syntax)]
#![feature(box_patterns)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_core::{
    common::chain,
    ecma::{
        ast::Program,
        transforms::base::hygiene::hygiene,
        visit::{as_folder, FoldWith},
    },
    plugin::{metadata::TransformPluginProgramMetadata as Metadata, plugin_transform},
};

mod regex_macro;

#[plugin_transform]
pub fn process_transform(program: Program, _: Metadata) -> Program {
    program.fold_with(
        &mut hygiene(), // &mut chain!(hygiene())
    )
}
