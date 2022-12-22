use indexmap::IndexMap;
use swc_core::{
    common::util::take::Take,
    ecma::ast::{Expr, Ident, VarDecl, VarDeclKind, VarDeclarator},
};

pub trait Declare {
    fn decl(&mut self) -> VarDecl;
}

impl Declare for IndexMap<Expr, Ident> {
    fn decl(&mut self) -> VarDecl {
        let decls = self
            .drain(..)
            .map(|(expr, ident)| {
                VarDeclarator {
                    name: ident.into(),
                    init: Some(box expr),
                    ..Take::dummy()
                }
            })
            .collect();

        VarDecl {
            kind: VarDeclKind::Const,
            decls,
            ..Take::dummy()
        }
    }
}
