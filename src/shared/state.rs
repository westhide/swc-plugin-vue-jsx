pub trait State {
    fn is_custom_element(&self, text: &str) -> bool;

    fn is_transform_on(&self) -> bool;
}
