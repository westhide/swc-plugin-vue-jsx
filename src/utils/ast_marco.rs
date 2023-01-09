#[macro_export]
macro_rules! args {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => {{
        use swc_core::ecma::utils::ExprFactory;

        vec![$($x.as_arg()),+]
    }};
}

#[macro_export]
macro_rules! array_lit {
    () => {
        ArrayLit {
            span: DUMMY_SP,
            elems: Vec::new(),
        }
    };
    ($($x:expr),+ $(,)?) => {{
        use swc_core::{
            common::DUMMY_SP,
            ecma::{ast::ArrayLit, utils::ExprFactory}
        };

        ArrayLit {
            span: DUMMY_SP,
            elems: vec![$(Some($x.as_arg())),+],
        }
    }};
}
