#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(is_some_and)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use options::PluginOptions;
use shared::{parse::Parse, state::State};
use swc_core::{
    common::{chain, util::take::Take},
    ecma::{
        ast::{Expr, Module, Program},
        transforms::base::hygiene::hygiene,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{metadata::TransformPluginProgramMetadata as Metadata, plugin_transform},
};

use crate::{import_helper::ImportHelper, utils::clean_jsx_text::clean_jsx_text, vnode::VNode};

mod constant;
#[path = "import-helper/mod.rs"]
mod import_helper;
mod options;
mod patch_flag;
mod shared;
mod utils;
#[path = "VNode/mod.rs"]
mod vnode;

#[plugin_transform]
pub fn process_transform(program: Program, metadata: Metadata) -> Program {
    let opts = PluginOptions::from(&metadata);
    program.fold_with(&mut chain!(as_folder(VueJSX::new(opts)), hygiene()))
}

pub struct VueJSX {
    pub opts: PluginOptions,
}

impl VueJSX {
    pub fn new(opts: PluginOptions) -> Self {
        Self { opts }
    }
}

impl State for VueJSX {
    fn is_custom_element(&self, text: &str) -> bool {
        regex_set!(&self.opts.custom_element_patterns).is_match(text)
    }

    fn is_transform_on(&self) -> bool {
        self.opts.transform_on
    }
}

impl VisitMut for VueJSX {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let mut import_helper = ImportHelper::new(module);

        import_helper.get_or_insert("ref", "./a");
        import_helper.get_or_insert("a", "./a");

        module.visit_mut_children_with(self);

        import_helper.inject_import_decls()
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match &expr {
            Expr::JSXElement(box element) => {
                let mut vnode = VNode::parse(element);
                vnode.analyze(self);
                println!("{}", clean_jsx_text("a  b \r d  \n  e\n  \n \nf"));

                println!("======{:#?}", vnode);
            },
            Expr::JSXFragment(fragment) => {
                let mut vnode = VNode::parse(fragment);
                println!("======{:#?}", vnode);
            },
            _ => {},
        }

        expr.visit_mut_children_with(self);
    }

    // fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
    //     let mut vnode = VNode::parse(&*element);
    //     vnode.fix(self);
    //
    //     println!("======{:#?}", vnode);
    //
    //     element.visit_mut_children_with(self);
    // }
}
