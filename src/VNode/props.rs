use swc_core::ecma::ast::{
    Expr, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXExpr, JSXNamespacedName, Lit,
    SpreadElement,
};

use crate::{
    constant::{BOOLEAN_ATTRIBUTE, CLASS, INNER_HTML, KEY, ON_CLICK, REF, STYLE, TEXT_CONTENT},
    shared::parse::Parse,
    utils::{
        ast::is_constant_expr,
        pattern::{is_camelcase_xlink, is_directive, is_event, xlink_to_namespace},
    },
    vnode::VNode,
};

/// ## [Key]
///
/// ---
#[derive(Debug)]
pub enum Key<'a> {
    /// special Attribute
    /// - [REF], [KEY], [CLASS], [STYLE],
    /// - [ON_CLICK]: onClick, on:click->onClick, on:Click->onClick
    Spec(&'a str),
    Event(&'a str),
    Directive(&'a str),
    /// - [TEXT_CONTENT] : v-text->textContent
    /// - [INNER_HTML] : v-html->innerHTML
    Attr(&'a str),
    NSAttr(String),
    Spread,
}

impl<'a> Key<'a> {
    fn specialize_directive(key: &'a str) -> Self {
        match key {
            "" => panic!("specialize_directive: directive name can not empty"),
            "text" => Self::Attr(TEXT_CONTENT),
            "html" => Self::Attr(INNER_HTML),
            key => Self::Directive(key),
        }
    }

    fn specialize_event(key: &'a str) -> Self {
        match key {
            "click" | "Click" => Self::Spec(ON_CLICK),
            key => Self::Event(key),
        }
    }
}

impl<'a> Parse<&'a JSXAttrName> for Key<'a> {
    fn parse(attr_name: &'a JSXAttrName) -> Self {
        match attr_name {
            JSXAttrName::Ident(name) => {
                match &*name.sym {
                    key @ (REF | KEY | CLASS | STYLE | ON_CLICK) => Self::Spec(key),
                    key if is_event(key) => Self::Event(&key[2..]),
                    key if is_directive(key) => Self::specialize_directive(&key[2..]),
                    key if is_camelcase_xlink(key) => Self::NSAttr(xlink_to_namespace(key)),
                    key => Self::Attr(key),
                }
            },
            JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name }) => {
                let key = &*name.sym;

                match &*ns.sym {
                    "on" => Self::specialize_event(key),
                    "v" => Self::specialize_directive(key),
                    domain => Self::NSAttr(format!("{domain}:{key}")),
                }
            },
        }
    }
}

/// ## [Value]
///
/// ---
#[derive(Debug)]
pub enum Value<'a> {
    True,
    Lit(&'a Lit),
    Const(&'a Expr),
    Expr(&'a Expr),
    VNode(Box<VNode<'a>>),
}

impl<'a> Value<'a> {
    fn specialize_expr(expr: &'a Expr) -> Self {
        match expr {
            Expr::Lit(lit) => Self::Lit(lit),
            expr if is_constant_expr(expr) => Self::Const(expr),
            expr => Self::Expr(expr),
        }
    }

    pub fn is_dyn_expr(&self) -> bool {
        matches!(self, Self::Expr(_))
    }

    pub fn is_vnode(&self) -> bool {
        matches!(self, Self::VNode(_))
    }
}

impl<'a> Parse<&'a JSXAttrValue> for Value<'a> {
    fn parse(attr_value: &'a JSXAttrValue) -> Self {
        match attr_value {
            JSXAttrValue::Lit(lit) => Self::Lit(lit),
            JSXAttrValue::JSXExprContainer(container) => {
                match &container.expr {
                    JSXExpr::Expr(expr) => Self::specialize_expr(expr),
                    JSXExpr::JSXEmptyExpr(_) => {
                        panic!("JSXAttrValue::JSXExprContainer can not empty")
                    },
                }
            },
            JSXAttrValue::JSXElement(box element) => Self::VNode(box VNode::parse(element)),
            JSXAttrValue::JSXFragment(fragment) => Self::VNode(box VNode::parse(fragment)),
        }
    }
}

/// ## [Prop]
///
/// ---
#[derive(Debug)]
pub struct Prop<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

impl<'a> Parse<&'a JSXAttrOrSpread> for Prop<'a> {
    fn parse(attr_or_spread: &'a JSXAttrOrSpread) -> Self {
        match attr_or_spread {
            JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) => {
                let key = Key::parse(name);

                let value = match value.as_ref() {
                    Some(attr_value) => Value::parse(attr_value),
                    None if let Key::Attr(attr) = key && BOOLEAN_ATTRIBUTE.contains(attr) => Value::True,
                    _ => panic!("JSXAttrValue {key:?} can not empty")
                };

                Self { key, value }
            },
            JSXAttrOrSpread::SpreadElement(SpreadElement { expr, .. }) => {
                Self {
                    key: Key::Spread,
                    value: Value::Expr(expr),
                }
            },
        }
    }
}

/// ## [PropCollection]
///
/// ---
#[derive(Debug, Default)]
pub struct PropCollection<'a> {
    pub specs: Vec<&'a Prop<'a>>,
    pub events: Vec<&'a Prop<'a>>,
    pub directives: Vec<&'a Prop<'a>>,
    pub attrs: Vec<&'a Prop<'a>>,
    pub spreads: Vec<&'a Prop<'a>>,
}

impl<'a> PropCollection<'a> {
    pub fn push(&mut self, prop: &'a Prop<'a>) {
        match &prop.key {
            Key::Spec(_) => self.specs.push(prop),
            Key::Event(_) => self.events.push(prop),
            Key::Directive(_) => self.directives.push(prop),
            Key::Attr(_) | Key::NSAttr(_) => self.attrs.push(prop),
            Key::Spread => self.spreads.push(prop),
        };
    }
}

impl<'a> FromIterator<&'a Prop<'a>> for PropCollection<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Prop<'a>>>(iter: T) -> Self {
        let mut collection = Self::default();

        let mut iter = iter.into_iter();

        while let Some(prop) = iter.next() {
            collection.push(prop)
        }

        collection
    }
}
