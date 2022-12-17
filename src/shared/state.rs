use swc_core::ecma::ast::Ident;

pub trait State<'s> {
    fn is_custom_element(&self, text: &str) -> bool;

    fn is_transform_on(&self) -> bool;

    fn import_from_vue(&mut self, name: &'s str) -> &Ident;
}
