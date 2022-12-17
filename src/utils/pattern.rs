use once_cell::sync::Lazy;
use regex::Regex;
use swc_core::ecma::ast::Ident;

use crate::{
    constant::{BOOLEAN_ATTRIBUTE, HTML_ELEMENT, SVG_ELEMENT, V_MODEL_NATIVE_ELEMENT},
    regex,
};

pub fn is_directive(text: &str) -> bool {
    regex!("^[vV]-").is_match(text)
}

pub fn is_event(text: &str) -> bool {
    regex!("^on[^a-z]").is_match(text)
}

static XLINK_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^xlink([A-Z])").unwrap());

pub fn is_camelcase_xlink(text: &str) -> bool {
    XLINK_RE.is_match(text)
}

pub fn xlink_to_namespace(text: &str) -> String {
    XLINK_RE.replace(text, "xlink:$1").to_string()
}

pub fn is_native_tag(tag: &str) -> bool {
    HTML_ELEMENT.contains(tag) || SVG_ELEMENT.contains(tag)
}

pub fn is_native_v_model_tag(tag: &str) -> bool {
    V_MODEL_NATIVE_ELEMENT.contains(&tag)
}

pub fn is_bool_attr(ident: &Ident) -> bool {
    BOOLEAN_ATTRIBUTE.contains(ident.as_ref())
}
