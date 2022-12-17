use regex::internal::Input;
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            Expr, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXExpr,
            JSXNamespacedName, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread,
            SpreadElement,
        },
        utils::quote_ident,
    },
};

use crate::{
    constant::{
        CLASS, EMPTY_STR, EMPTY_STRING_EXPR, INNER_HTML, KEY, MODEL, ON_CLICK, PROP_NAME_CLASS,
        PROP_NAME_KEY, PROP_NAME_STYLE, REF, STYLE, TEXT_CONTENT,
    },
    patch_flag::PatchFlag,
    shared::{convert::Convert, state::State, transform::Transform},
    utils::{
        ast::{is_constant_expr, key_value_prop},
        pattern::{is_bool_attr, is_directive, is_event},
    },
    vnode::VNode,
};

/// ## [Value]
///
/// ---
#[derive(Debug)]
pub enum Value<'a> {
    Lit(&'a Lit),
    /// TODO: not strict, [is_constant_expr] check may loose
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

    fn is_dyn(value: &Value<'a>) -> bool {
        matches!(value, Self::Expr(_) | Self::VNode(_))
    }
}

impl<'a> Transform<'a, Value<'a>> for JSXAttrValue {
    fn transform(&'a self) -> Value<'a> {
        match self {
            JSXAttrValue::Lit(lit) => Value::Lit(lit),
            JSXAttrValue::JSXExprContainer(container) => {
                match &container.expr {
                    JSXExpr::Expr(expr) => Value::specialize_expr(expr),
                    JSXExpr::JSXEmptyExpr(_) => {
                        panic!("JSXAttrValue::JSXExprContainer can not empty")
                    },
                }
            },
            JSXAttrValue::JSXElement(box element) => Value::VNode(box element.transform()),
            JSXAttrValue::JSXFragment(fragment) => Value::VNode(box fragment.transform()),
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Value<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        match self {
            Self::Expr(&ref expr) => expr.clone(),
            Self::Lit(&ref lit) => lit.clone().into(),
            Self::Const(&ref expr) => expr.clone(),
            Self::VNode(vnode) => vnode.convert(state),
        }
    }
}

/// ## [PropStore]
/// - dynamic attribute is terrible for compiler optimize
///   - [TransformOn](https://github.com/vuejs/babel-plugin-jsx/blob/dev/packages/babel-helper-vue-transform-on/README.md) Event no more support in Vue JSX
///   - [Spread Attribute](Key::Spread) not recommend to use for better static optimize
/// ---
#[derive(Debug, Default)]
pub struct PropStore<'a> {
    /// special Attribute
    /// - [REF], [KEY], [CLASS], [STYLE],
    r#ref: Option<Value<'a>>,
    key: Option<Value<'a>>,
    class: Option<Value<'a>>,
    style: Option<Value<'a>>,
    /// [ON_CLICK]
    /// - `onClick`, `on:click`
    on_click: Option<Value<'a>>,
    /// [MODEL]
    /// - `v-model`, `v:model`
    models: Vec<(Value<'a>, Option<&'a str>)>,

    events: Vec<(&'a str, Value<'a>)>,
    directives: Vec<(&'a str, Value<'a>)>,

    /// - [TEXT_CONTENT] : `v-text`
    /// - [INNER_HTML] : `v-html`
    attrs: Vec<(&'a str, Value<'a>)>,
    ns_attrs: Vec<(&'a str, &'a str, Value<'a>)>,

    bool_attrs: Vec<&'a Ident>,

    spreads: Vec<&'a Expr>,
}

impl<'a> PropStore<'a> {
    fn specialize_directive(&mut self, name: &'a str, value: Value<'a>) {
        match name {
            EMPTY_STR => panic!("specialize_directive: directive name can not empty"),
            "model" => self.models.push((value, None)),
            "text" => self.attrs.push((TEXT_CONTENT, value)),
            "html" => self.attrs.push((INNER_HTML, value)),
            name => self.directives.push((name, value)),
        }
    }

    fn specialize_event(&mut self, name: &'a str, value: Value<'a>) {
        match name {
            "click" => self.on_click = Some(value),
            name => self.events.push((name, value)),
        }
    }

    pub fn insert(&mut self, attr_name: &'a JSXAttrName, attr_value: &'a JSXAttrValue) {
        let value = attr_value.transform();

        match attr_name {
            JSXAttrName::Ident(ident) => {
                match ident.as_ref() {
                    REF => self.r#ref = Some(value),
                    KEY => self.key = Some(value),
                    CLASS => self.class = Some(value),
                    STYLE => self.style = Some(value),
                    ON_CLICK => self.on_click = Some(value),
                    name if is_event(name) => self.events.push((&name[2..], value)),
                    name if is_directive(name) => self.specialize_directive(&name[2..], value),
                    name => self.attrs.push((name, value)),
                }
            },
            JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name }) => {
                let name = name.as_ref();

                match ns.as_ref() {
                    "on" => self.specialize_event(name, value),
                    "v" => self.specialize_directive(name, value),
                    "v-model" => self.models.push((value, Some(name))),
                    ns => self.ns_attrs.push((ns, name, value)),
                }
            },
        };
    }
}

impl<'a> Transform<'a, PropStore<'a>> for [JSXAttrOrSpread] {
    fn transform(&'a self) -> PropStore<'a> {
        let mut store = PropStore::default();

        self.iter().for_each(|prop_or_spread| {
            match prop_or_spread {
                JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) => {
                    match value.as_ref() {
                        Some(attr_value) => {
                            store.insert(name, attr_value)
                        }
                        None if let JSXAttrName::Ident(ident) = name && is_bool_attr(ident) => {
                            store.bool_attrs.push(ident)
                        },
                        None => panic!("JSXAttr can not empty")
                    }
                },
                JSXAttrOrSpread::SpreadElement(SpreadElement { expr, .. }) => {
                    store.spreads.push(expr)
                },
            };
        });

        store
    }
}

trait TryPushProp<V> {
    fn try_push<'s, S: State<'s>>(&mut self, name: PropName, value: &V, state: &mut S);
}

impl<'a> TryPushProp<Option<Value<'a>>> for Vec<PropOrSpread> {
    fn try_push<'s, S: State<'s>>(
        &mut self,
        key: PropName,
        value: &Option<Value<'a>>,
        state: &mut S,
    ) {
        if let Some(value) = value {
            self.push(
                Prop::KeyValue(KeyValueProp {
                    key,
                    value: box value.convert(state),
                })
                .into(),
            );
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for PropStore<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> ObjectLit {
        let mut props: Vec<PropOrSpread> = Vec::new();

        let Self {
            r#ref,
            key,
            class,
            style,
            on_click,
            events,
            attrs,
            ns_attrs,
            bool_attrs,
            spreads,
            ..
        } = self;

        props.try_push(quote_ident!(REF).into(), r#ref, state);
        props.try_push(PROP_NAME_KEY, key, state);
        props.try_push(PROP_NAME_CLASS, class, state);
        props.try_push(PROP_NAME_STYLE, style, state);
        props.try_push(quote_ident!(ON_CLICK).into(), on_click, state);

        // props.try_push(quote_ident!(MODEL).into(), model, state);
        // props.extend(model.iter.map(|(d, ..)| {}));

        props.extend(events.iter().map(|(&ref name, value)| {
            let event_name = format!("on{}{}", name[..1].to_uppercase(), &name[1..]);
            key_value_prop(event_name, value, state)
        }));

        props.extend(attrs.iter().map(|(&ref name, value)| {
            let attr_name = name.to_string();
            key_value_prop(attr_name, value, state)
        }));

        props.extend(ns_attrs.iter().map(|(&ref ns, &ref name, value)| {
            key_value_prop(format!("{ns}:{name}"), value, state)
        }));

        props.extend(bool_attrs.iter().map(|&ident| {
            Prop::KeyValue(KeyValueProp {
                key: ident.clone().into(),
                value: box EMPTY_STRING_EXPR,
            })
            .into()
        }));

        let prop_obj = ObjectLit {
            span: DUMMY_SP,
            props,
        };
    }
}

/// ### [Props] [Inform](Props::inform)
///
/// ---
pub trait IsPatch {
    fn is_patch(&self) -> bool;
}

impl<'a> IsPatch for Option<Value<'a>> {
    fn is_patch(&self) -> bool {
        self.as_ref().is_some_and(Value::is_dyn)
    }
}

impl<'a> IsPatch for [(&'a str, Value<'a>)] {
    fn is_patch(&self) -> bool {
        self.iter().any(|(_, value)| Value::is_dyn(value))
    }
}

impl<'a> IsPatch for [(&'a str, &'a str, Value<'a>)] {
    fn is_patch(&self) -> bool {
        self.iter().any(|(.., value)| Value::is_dyn(value))
    }
}

impl<'a> PropStore<'a> {
    fn need_patch(&self) -> bool {
        let Self {
            r#ref,
            models,
            directives,
            ..
        } = self;

        r#ref.is_some() || !models.is_empty() || !directives.is_empty()
    }

    /// - Return: ( patch_flag<sup>[PatchFlag]</sup>, is_const<sup>bool</sup>)
    pub fn inform(&self) -> (isize, bool) {
        if !self.spreads.is_empty() {
            return (PatchFlag::FULL_PROPS, false);
        }

        let mut flag = 0isize;

        let Self {
            key,
            class,
            style,
            on_click,
            events,
            attrs,
            ns_attrs,
            ..
        } = self;

        if class.is_patch() {
            flag |= PatchFlag::CLASS
        }

        if style.is_patch() {
            flag |= PatchFlag::STYLE
        }

        if events.is_patch() {
            flag |= PatchFlag::HYDRATE_EVENTS
        }

        if attrs.is_patch() || ns_attrs.is_patch() {
            flag |= PatchFlag::PROPS
        }

        if PatchFlag::is_non_prop(&flag) && self.need_patch() {
            flag |= PatchFlag::NEED_PATCH
        }

        let is_const = flag == 0 && !key.is_patch() && !on_click.is_patch();

        (flag, is_const)
    }
}
