use swc_core::ecma::ast::{JSXAttrName, JSXNamespacedName};

use crate::{
    constant::{CLASS, EMPTY_STR, INNER_HTML, KEY, ON_CLICK, REF, STYLE, TEXT_CONTENT},
    shared::transform::Transform,
    utils::pattern::{is_directive, is_event},
};

/// ## [Key]
/// - dynamic attribute is terrible for compiler optimize
///   - [TransformOn](https://github.com/vuejs/babel-plugin-jsx/blob/dev/packages/babel-helper-vue-transform-on/README.md) Event no more support in Vue JSX
///   - [Spread Attribute](Key::Spread) not recommend to use for better static optimize
#[derive(Debug)]
pub enum Key<'a> {
    /// special Attribute
    /// - [REF], [KEY], [CLASS], [STYLE],
    Ref,
    Key,
    Class,
    Style,
    /// [ON_CLICK]
    /// - `onClick`, `on:click`
    OnClick,
    /// [MODEL]
    /// - `v-model`, `v:model`, `v-model:*`<sup>arg</sup>
    Model(Option<&'a str>),
    Event(&'a str),
    Directive(&'a str),
    /// - [TEXT_CONTENT] : `v-text`
    /// - [INNER_HTML] : `v-html`
    Attr(&'a str),
    NSAttr {
        ns: &'a str,
        name: &'a str,
    },
    Spread,
}

impl<'a> Key<'a> {
    // fn v_model(value: Value<'a>, arg: Option<&'a str>) -> Self {
    //     match value {
    //         Value::Expr(expr) => Self::Model { expr, arg },
    //         _ => panic!("v-model must have expr value"),
    //     }
    // }
    pub fn may_static(&self) -> bool {
        matches!(
            self,
            Self::Key | Self::Class | Self::Style | Self::Attr(_) | Self::NSAttr { .. }
        )
    }

    fn specialize_directive(name: &'a str) -> Self {
        match name {
            EMPTY_STR => panic!("Forbidden: Empty directive name"),
            "model" => Self::Model(None),
            "text" => Self::Attr(TEXT_CONTENT),
            "html" => Self::Attr(INNER_HTML),
            name => Self::Directive(name),
        }
    }

    fn specialize_event(name: &'a str) -> Self {
        match name {
            "click" => Self::OnClick,
            name => Self::Event(name),
        }
    }
}

impl<'a> Transform<'a, Key<'a>> for JSXAttrName {
    fn transform(&'a self) -> Key<'a> {
        match self {
            JSXAttrName::Ident(ident) => {
                match ident.as_ref() {
                    REF => Key::Ref,
                    KEY => Key::Key,
                    CLASS => Key::Class,
                    STYLE => Key::Style,
                    ON_CLICK => Key::OnClick,
                    name if is_event(name) => Key::Event(&name[2..]),
                    name if is_directive(name) => Key::specialize_directive(&name[2..]),
                    name => Key::Attr(name),
                }
            },
            JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name }) => {
                let name = name.as_ref();

                match ns.as_ref() {
                    "on" => Key::specialize_event(name),
                    "v" => Key::specialize_directive(name),
                    "v-model" => Key::Model(Some(name)),
                    ns => Key::NSAttr { ns, name },
                }
            },
        }
    }
}
