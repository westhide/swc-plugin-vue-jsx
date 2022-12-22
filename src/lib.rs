#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(is_some_and)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
// TODO: Debug only
#![allow(unused)]
#![feature(test)]

use std::collections::HashMap;

pub use options::PluginOptions;
use state::State;
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
    hoist_helper::HoistHelper,
    import_helper::ImportHelper,
    scope_helper::ScopeHelper,
    shared::{convert::Convert, transform::Transform},
    vnode::{element::Element, VNode},
};

mod constant;
mod hoist_helper;
#[path = "import-helper/mod.rs"]
mod import_helper;
mod options;
mod patch_flag;
mod scope_helper;
mod shared;
mod state;
mod static_vnode_helper;
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

    hoist_helper: HoistHelper,

    private_idents: HashMap<&'s str, Ident>,

    scope: ScopeHelper,
}

impl<'s, C: Comments> VueJSX<'s, C> {
    pub fn new(opts: PluginOptions, comments: Option<C>, unresolved_mark: Mark) -> Self {
        Self {
            opts,
            comments,
            unresolved_mark,
            import_helper: ImportHelper::default(),
            hoist_helper: HoistHelper::default(),
            private_idents: HashMap::new(),
            scope: ScopeHelper::default(),
        }
    }
}

impl<'s, C: Comments> VisitMut for VueJSX<'s, C> {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let Self { import_helper, .. } = self;

        import_helper.init(module);

        module.visit_mut_children_with(self);

        self.import_helper.inject(module);
        self.hoist_helper.inject(module)
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match &expr {
            Expr::JSXElement(box element) => {
                let mut elm: Element = element.transform();

                let elm_expr = elm.convert(self);

                let render_expr = self.scope.create_render_expr(elm_expr);

                *expr = render_expr
            },
            Expr::JSXFragment(fragment) => {
                let mut vnode: VNode = fragment.transform();
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
