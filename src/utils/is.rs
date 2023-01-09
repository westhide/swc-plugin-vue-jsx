use crate::{constant::V_MODEL_NATIVE_ELEMENT, regex};

pub fn is_directive(text: &str) -> bool {
    regex!("^v-").is_match(text)
}

#[allow(dead_code)]
pub fn is_native_v_model_tag(tag: &str) -> bool {
    V_MODEL_NATIVE_ELEMENT.contains(&tag)
}
