use swc_core::{
    ecma::{
        ast::{
            Expr, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXExpr,
            JSXNamespacedName, KeyValueProp, Lit, Prop, PropName, PropOrSpread, SpreadElement, Str,
        },
        utils::quote_ident,
    },
    quote,
};

use crate::{
    constant::{
        CLASS, EMPTY_STR, EMPTY_STRING_EXPR, INNER_HTML, KEY, MODEL_VALUE, ON_CLICK,
        PROP_NAME_CLASS, PROP_NAME_KEY, PROP_NAME_STYLE, REF, STYLE, TEXT_CONTENT,
    },
    patch_flag::PatchFlag,
    shared::{convert::Convert, state::State, transform::Transform},
    utils::{
        ast::is_constant_expr,
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
    /// TODO: [is_constant_expr] Non-strict
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

    fn is_static(&self) -> bool {
        matches!(self, Self::Lit(_) | Self::Const(_))
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

/// ## [VProp]
/// - dynamic attribute is terrible for compiler optimize
///   - [TransformOn](https://github.com/vuejs/babel-plugin-jsx/blob/dev/packages/babel-helper-vue-transform-on/README.md) Event no more support in Vue JSX
///   - [Spread Attribute](Key::Spread) not recommend to use for better static optimize
/// ---
#[derive(Debug)]
pub enum VProp<'a> {
    /// special Attribute
    /// - [REF], [KEY], [CLASS], [STYLE],
    Ref(Value<'a>),
    Key(Value<'a>),
    Class(Value<'a>),
    Style(Value<'a>),
    /// [ON_CLICK]
    /// - `onClick`, `on:click`
    OnClick(Value<'a>),
    /// [MODEL]
    /// - `v-model`, `v:model`, `v-model:*`<sup>arg</sup>
    Model {
        expr: &'a Expr,
        arg: Option<&'a str>,
    },
    Event(&'a str, Value<'a>),
    Directive(&'a str, Value<'a>),
    /// - [TEXT_CONTENT] : `v-text`
    /// - [INNER_HTML] : `v-html`
    Attr(&'a str, Value<'a>),
    NSAttr {
        ns: &'a str,
        name: &'a str,
        value: Value<'a>,
    },
    BoolAttr(&'a Ident),
    Spread(&'a Box<Expr>),
}

impl<'a> VProp<'a> {
    pub fn is_static(&self) -> bool {
        match self {
            Self::Ref(value)
            | Self::Key(value)
            | Self::Class(value)
            | Self::Style(value)
            | Self::OnClick(value)
            | Self::Event(_, value)
            | Self::Attr(_, value)
            | Self::NSAttr { value, .. } => value.is_static(),
            Self::Model { .. } | VProp::Directive(..) | VProp::Spread(_) => false,
            Self::BoolAttr(_) => true,
        }
    }

    fn v_model(value: Value<'a>, arg: Option<&'a str>) -> Self {
        match value {
            Value::Expr(expr) => Self::Model { expr, arg },
            _ => panic!("v-model must have expr value"),
        }
    }

    fn specialize_directive(name: &'a str, value: Value<'a>) -> Self {
        match name {
            EMPTY_STR => panic!("specialize_directive: directive name can not empty"),
            "model" => Self::v_model(value, None),
            "text" => Self::Attr(TEXT_CONTENT, value),
            "html" => Self::Attr(INNER_HTML, value),
            name => Self::Directive(name, value),
        }
    }

    fn specialize_event(name: &'a str, value: Value<'a>) -> Self {
        match name {
            "click" => Self::OnClick(value),
            name => Self::Event(name, value),
        }
    }

    fn from_attr(attr_name: &'a JSXAttrName, attr_value: &'a JSXAttrValue) -> Self {
        let value = attr_value.transform();

        match attr_name {
            JSXAttrName::Ident(ident) => {
                match ident.as_ref() {
                    REF => Self::Ref(value),
                    KEY => Self::Key(value),
                    CLASS => Self::Class(value),
                    STYLE => Self::Style(value),
                    ON_CLICK => Self::OnClick(value),
                    name if is_event(name) => Self::Event(&name[2..], value),
                    name if is_directive(name) => Self::specialize_directive(&name[2..], value),
                    name => Self::Attr(name, value),
                }
            },
            JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name }) => {
                let name = name.as_ref();

                match ns.as_ref() {
                    "on" => Self::specialize_event(name, value),
                    "v" => Self::specialize_directive(name, value),
                    "v-model" => Self::v_model(value, Some(name)),
                    ns => Self::NSAttr { ns, name, value },
                }
            },
        }
    }
}

impl<'a> Transform<'a, VProp<'a>> for JSXAttrOrSpread {
    fn transform(&'a self) -> VProp<'a> {
        match self {
            JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) => {
                match value.as_ref() {
                    Some(attr_value) => {
                        VProp::from_attr(name, attr_value)
                    }
                    None if let JSXAttrName::Ident(ident) = name && is_bool_attr(ident) => {
                        VProp::BoolAttr(ident)
                    },
                    None => panic!("JSXAttr can not empty")
                }
            },
            JSXAttrOrSpread::SpreadElement(SpreadElement { expr, .. }) => VProp::Spread(expr),
        }
    }
}

// (,directives,kv_props,dynamic key)
impl<'a, 's> Convert<'s, PropOrSpread> for VProp<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> PropOrSpread {
        let kv = match self {
            Self::Ref(value) => {
                KeyValueProp {
                    key: quote_ident!(REF).into(),
                    value: box value.convert(state),
                }
            },
            Self::Key(value) => {
                KeyValueProp {
                    key: PROP_NAME_KEY,
                    value: box value.convert(state),
                }
            },
            Self::Class(value) => {
                KeyValueProp {
                    key: PROP_NAME_CLASS,
                    value: box value.convert(state),
                }
            },
            Self::Style(value) => {
                KeyValueProp {
                    key: PROP_NAME_STYLE,
                    value: box value.convert(state),
                }
            },
            Self::OnClick(value) => {
                KeyValueProp {
                    key: quote_ident!(ON_CLICK).into(),
                    value: box value.convert(state),
                }
            },
            Self::Model { expr, arg } => {
                let arg = arg.unwrap_or_else(|| MODEL_VALUE);

                let update_name = format!("onUpdate:{arg}");
                let key = PropName::Str(Str::from(update_name));

                let listener = quote!("e=>e=$expr" as Expr, expr: Expr = (*expr).clone());

                KeyValueProp {
                    key,
                    value: box listener,
                }
            },
            Self::Event(name, value) => {
                let event_name = format!("on{}{}", name[..1].to_uppercase(), &name[1..]);
                KeyValueProp {
                    key: quote_ident!(event_name).into(),
                    value: box value.convert(state),
                }
            },
            Self::Directive(name, value) => {
                todo!()
            },
            Self::Attr(&ref name, value) => {
                KeyValueProp {
                    key: quote_ident!(name).into(),
                    value: box value.convert(state),
                }
            },
            Self::NSAttr { ns, name, value } => {
                let ns_name = format!("{ns}:{name}");
                KeyValueProp {
                    key: quote_ident!(ns_name).into(),
                    value: box value.convert(state),
                }
            },
            Self::BoolAttr(&ref ident) => {
                KeyValueProp {
                    key: ident.clone().into(),
                    value: box EMPTY_STRING_EXPR,
                }
            },
            Self::Spread(expr) => {
                todo!()
            },
        };

        Prop::KeyValue(kv).into()
    }
}

/// [PatchFlag]
pub fn patch_flag(props: &[VProp]) -> isize {
    let mut flag = 0isize;

    let mut need_patch = false;

    props.iter().for_each(|prop: &VProp| {
        match prop {
            VProp::Class(value) if Value::is_dyn(value) => flag |= PatchFlag::CLASS,
            VProp::Style(value) if Value::is_dyn(value) => flag |= PatchFlag::STYLE,
            VProp::Event(_, value) if Value::is_dyn(value) => flag |= PatchFlag::HYDRATE_EVENTS,
            VProp::Attr(_, value) | VProp::NSAttr { value, .. } if Value::is_dyn(value) => {
                flag |= PatchFlag::PROPS
            },
            VProp::Ref(_) | VProp::Model { .. } | VProp::Directive(..) => need_patch = true,
            VProp::Spread(_) => flag |= PatchFlag::FULL_PROPS,
            _ => {},
        }
    });

    if PatchFlag::is_non_prop(&flag) && need_patch {
        flag |= PatchFlag::NEED_PATCH
    }

    flag
}
