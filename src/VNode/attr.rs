use swc_core::{
    common::util::take::Take,
    ecma::ast::{JSXAttr, JSXAttrOrSpread, SpreadElement},
};

use crate::{
    shared::{convert::Convert, transform::Transform},
    utils::pattern::is_bool_attr,
    vnode::{attr_key::Key, attr_value::Value},
};

#[derive(Debug)]
pub struct Attr<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

impl<'a> Attr<'a> {
    pub fn is_static(&self) -> bool {
        let Self { key, value } = self;
        key.may_static() && !value.is_dyn()
    }
}

impl<'a> Transform<'a, Attr<'a>> for JSXAttrOrSpread {
    fn transform(&'a self) -> Attr<'a> {
        match self {
            JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) => {
                let key = name.transform();

                match value.as_ref() {
                    Some(attr_value) => {
                        let value = attr_value.transform();

                        Attr { key, value }
                    }
                    None if let Key::Attr(name) = key && is_bool_attr(name) => {
                        Attr {
                            key,
                            value: Value::Empty,
                        }
                    },
                    None => panic!("Forbidden: Empty JSXAttr")
                }
            },
            JSXAttrOrSpread::SpreadElement(SpreadElement { expr, .. }) => {
                Attr {
                    key: Key::Spread,
                    value: Value::Expr(expr),
                }
            },
        }
    }
}
