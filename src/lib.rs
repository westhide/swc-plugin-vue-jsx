#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(is_some_and)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
// TODO: Debug only
#![allow(unused)]
#![feature(test)]
// #![feature(associated_type_defaults)]

extern crate core;

use std::fmt::Debug;

pub use options::PluginOptions;
use shared::state::State;
use swc_core::{
    common::{chain, comments::Comments, util::take::Take, Mark},
    ecma::{
        ast::{Expr, Ident, Module, Program},
        transforms::base::hygiene::hygiene,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{metadata::TransformPluginProgramMetadata as Metadata, plugin_transform},
};

use crate::{
    import_helper::ImportHelper,
    shared::{convert::Convert, transform::Transform},
    vnode::VNode,
};

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

    let Metadata {
        comments,
        unresolved_mark,
        ..
    } = metadata;

    program.fold_with(&mut chain!(
        as_folder(VueJSX::new(opts, comments, unresolved_mark)),
        hygiene()
    ))
}

pub struct VueJSX<'s, C: Comments> {
    opts: PluginOptions,
    comments: Option<C>,
    unresolved_mark: Mark,

    import_helper: ImportHelper<'s>,
}

impl<'s, C: Comments> VueJSX<'s, C> {
    pub fn new(opts: PluginOptions, comments: Option<C>, unresolved_mark: Mark) -> Self {
        Self {
            opts,
            comments,
            unresolved_mark,
            import_helper: ImportHelper::default(),
        }
    }
}

impl<'s, C: Comments> State<'s> for VueJSX<'s, C> {
    fn is_custom_element(&self, text: &str) -> bool {
        regex_set!(&self.opts.custom_element_patterns).is_match(text)
    }

    fn is_transform_on(&self) -> bool {
        self.opts.transform_on
    }

    fn import_from_vue(&mut self, name: &'s str) -> &Ident {
        self.import_helper.get_or_insert(name, "vue")
    }
}

impl<'s, C: Comments> VisitMut for VueJSX<'s, C> {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let Self { import_helper, .. } = self;

        import_helper.init(module);

        module.visit_mut_children_with(self);

        self.import_helper.inject_import_decls();
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match &expr {
            Expr::JSXElement(box element) => {
                let mut vnode: VNode = element.transform();
                let vnode_expr = vnode.convert(self);
                *expr = vnode_expr
            },
            Expr::JSXFragment(fragment) => {
                let mut vnode = fragment.transform();
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
