#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(is_some_and)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::collections::HashMap;

pub use options::PluginOptions;
use swc_core::{
    common::{comments::Comments, Mark},
    ecma::{
        ast::{Expr, Ident, Module, Program},
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata as Metadata},
};
use swc_helper_jsx_transform::shared::Transform;
use swc_helper_module_import::ImportHelper;

use crate::{
    hoist::Hoist,
    shared::{convert::Convert, expr::ExprExtend},
};

mod constant;
mod context;
mod element;
mod fragment;
mod hoist;
mod options;
mod patch_flag;
mod shared;
mod split_static;
mod text;
mod utils;
mod vnode;

#[plugin_transform]
pub fn process_transform(program: Program, metadata: Metadata) -> Program {
    let opts = PluginOptions::from(&metadata);

    let Metadata {
        comments,
        unresolved_mark,
        ..
    } = metadata;

    program.fold_with(&mut as_folder(VueJSX::new(opts, comments, unresolved_mark)))
}

#[allow(dead_code)]
pub struct VueJSX<'a, C: Comments> {
    opts: PluginOptions,
    comments: Option<C>,
    unresolved_mark: Mark,

    import_helper: ImportHelper<'a>,

    ident_map: HashMap<&'a str, Ident>,

    module_hoist: Hoist<'a>,

    scope_hoist: Hoist<'a>,
}

impl<'a, C: Comments> VueJSX<'a, C> {
    pub fn new(opts: PluginOptions, comments: Option<C>, unresolved_mark: Mark) -> Self {
        Self {
            opts,
            comments,
            unresolved_mark,
            import_helper: ImportHelper::default(),
            ident_map: HashMap::new(),
            module_hoist: Hoist::new("_hoisted_"),
            scope_hoist: Hoist::new("_v"),
        }
    }

    pub fn store(&mut self, module: &mut Module) {
        self.import_helper.store(module)
    }

    pub fn complete(&mut self, module: &mut Module) {
        self.import_helper.add_to_module(module);
        self.module_hoist.add_to_module(module)
    }
}

impl<'a, 'b, C: Comments> VueJSX<'a, C> {
    pub fn compile<T, U>(&mut self, target: &'b T) -> Expr
    where
        T: Transform<'b, U>,
        U: Convert<'a, Expr>,
    {
        target
            .transform()
            .convert(self)
            .with_hoist(&mut self.scope_hoist)
    }
}

impl<'a, C: Comments> VisitMut for VueJSX<'a, C> {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        self.store(module);

        module.visit_mut_children_with(self);

        self.complete(module)
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match &expr {
            Expr::JSXElement(box element) => {
                *expr = self.compile(element);
            },
            Expr::JSXFragment(fragment) => {
                *expr = self.compile(fragment);
            },
            _ => {},
        }

        expr.visit_mut_children_with(self);
    }
}
