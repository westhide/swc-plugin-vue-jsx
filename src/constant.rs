use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Expr, Lit, Null},
};

pub const NULL_EXPR: Expr = Expr::Lit(Lit::Null(Null { span: DUMMY_SP }));

pub const REF: &str = "ref";
pub const KEY: &str = "key";
pub const CLASS: &str = "class";
pub const STYLE: &str = "style";
pub const ON_CLICK: &str = "onClick";

pub const V_TEXT: &str = "v-text";
pub const TEXT_CONTENT: &str = "textContent";

pub const V_HTML: &str = "v-html";
pub const INNER_HTML: &str = "innerHTML";

pub const V_MODEL: &str = "v-model";
pub const MODEL_VALUE: &str = "modelValue";

pub const V_SLOTS: &str = "v-slots";

pub const V_SHOW: &str = "v-show";

pub const FRAGMENT: &str = "Fragment";

// pub const KEEP_ALIVE: &str = "KeepAlive";

pub const V_MODEL_NATIVE_ELEMENT: &[&str; 3] = &["input", "textarea", "select"];
