use regex::internal::Input;
use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            ArrayLit, ArrowExpr, AssignExpr, AssignOp, Expr, ExprOrSpread, JSXElement,
            KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str,
        },
        atoms::JsWord,
        utils::{quote_ident, ExprFactory},
    },
};

use crate::{
    constant::{
        JSW_CLASS, JSW_STYLE, MODEL, MODEL_VALUE, NULL_EXPR, ON_CLICK, PROP_NAME_CLASS,
        PROP_NAME_KEY, PROP_NAME_STYLE, REF,
    },
    patch_flag::PatchFlag,
    shared::{convert::Convert, transform::Transform},
    state::State,
    vnode::{
        attr::Attr,
        attr_key::Key,
        directive::{Directive, PushDirective},
        element_tag::Tag,
        VNode,
    },
};

/// ## [Element]
///
/// ---
#[derive(Debug)]
pub struct Element<'a> {
    pub tag: Tag<'a>,
    pub attrs: Vec<Attr<'a>>,
    pub children: Vec<VNode<'a>>,

    pub raw: &'a JSXElement,

    pub is_static: bool,
}

impl<'a> Element<'a> {}

impl<'a> Transform<'a, Element<'a>> for JSXElement {
    fn transform(&'a self) -> Element<'a> {
        let Self {
            opening, children, ..
        } = self;

        let tag = opening.name.transform();

        let attrs = opening.attrs.transform();

        let children: Vec<VNode> = children.transform();

        let is_static = tag.is_native()
            && attrs.iter().all(Attr::is_static)
            && children.iter().all(VNode::is_static);

        Element {
            tag,
            attrs,
            children,
            raw: self,

            is_static,
        }
    }
}

/// ## [PushProp]
///
/// ---
trait PushProp {
    fn push_prop(&mut self, key: PropName, value: Box<Expr>);

    fn push_ident_prop<T: Into<JsWord>>(&mut self, name: T, value: Box<Expr>);
}

impl PushProp for Vec<PropOrSpread> {
    fn push_prop(&mut self, key: PropName, value: Box<Expr>) {
        self.push(Prop::KeyValue(KeyValueProp { key, value }).into())
    }

    fn push_ident_prop<T: Into<JsWord>>(&mut self, name: T, value: Box<Expr>) {
        self.push_prop(quote_ident!(name).into(), value)
    }
}

/// ## [v_model_listener]
///
/// ---
fn v_model_listener<'s, S: State<'s>>(value: Box<Expr>, state: &mut S) -> Box<Expr> {
    let ident = state.get_private_ident("$v");

    ArrowExpr {
        params: vec![ident.clone().into()],
        body: AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: value.as_pat_or_expr(),
            right: box Expr::Ident(ident),
        }
        .into(),
        ..Take::dummy()
    }
    .into()
}

/// ## [create_props_expr]
///
/// ---
fn create_props_expr<'s, S: State<'s>>(
    props: Vec<PropOrSpread>,
    mut spreads: Vec<ExprOrSpread>,
    state: &mut S,
) -> Expr {
    let props_expr = if props.is_empty() {
        None
    } else {
        Some(Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props,
        }))
    };

    if spreads.is_empty() {
        props_expr.unwrap_or(NULL_EXPR)
    } else {
        let merge_props = state.import_from_vue("mergeProps");

        if let Some(expr) = props_expr {
            spreads.push(expr.as_arg())
        }

        merge_props.as_call(DUMMY_SP, spreads)
    }
}

/// ## [create_vnode_expr]
///
/// ---
fn create_element_node<'s, S: State<'s>>(
    tag: ExprOrSpread,
    props: ExprOrSpread,
    children: ExprOrSpread,
    patch_flag: isize,
    dyn_keys: Vec<Option<ExprOrSpread>>,
    state: &mut S,
) -> Expr {
    let create_vnode = state.import_from_vue("createVNode");

    let mut args = vec![tag, props, children];

    if patch_flag != 0 {
        args.push((patch_flag as f64).as_arg());

        if !dyn_keys.is_empty() {
            let hoisted = state.hoist_expr(
                ArrayLit {
                    span: DUMMY_SP,
                    elems: dyn_keys,
                }
                .into(),
            );

            args.push(hoisted.as_arg())
        }
    }

    create_vnode.as_call(DUMMY_SP, args)
}

fn create_with_directives<'s, S: State<'s>>(
    expr: Expr,
    directives: Vec<Directive>,
    state: &mut S,
) -> Expr {
    let with_directives = state.import_from_vue("withDirectives");

    with_directives.as_call(DUMMY_SP, vec![
        expr.as_arg(),
        directives.convert(state).as_arg(),
    ])
}

impl<'a, 's> Convert<'s, Expr> for Element<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        let Self {
            tag,
            attrs,
            children,
            is_static,
            ..
        } = self;

        let is_cmpt = tag.is_component(state);

        let mut directives: Vec<Directive> = Vec::new();

        let mut flag = 0isize;
        let mut dyn_keys: Vec<Option<ExprOrSpread>> = Vec::new();

        let mut spreads: Vec<ExprOrSpread> = Vec::new();
        let mut props: Vec<PropOrSpread> = Vec::with_capacity(attrs.len());

        attrs.iter().for_each(|Attr { key, value }| {
            let is_dyn = value.is_dyn();
            let value = box value.convert(state);

            match key {
                Key::Ref => {
                    if is_dyn {
                        flag |= PatchFlag::NEED_PATCH
                    } else {
                        panic!("Forbidden: const ref value")
                    }

                    props.push_ident_prop(REF, value)
                },
                Key::Key => props.push_prop(PROP_NAME_KEY, value),
                Key::Class => {
                    if is_dyn {
                        if is_cmpt {
                            flag |= PatchFlag::PROPS;

                            dyn_keys.push(Some(JSW_CLASS.as_arg()))
                        } else {
                            flag |= PatchFlag::CLASS
                        }
                    }

                    props.push_prop(PROP_NAME_CLASS, value)
                },
                Key::Style => {
                    if is_dyn {
                        if is_cmpt {
                            flag |= PatchFlag::PROPS;

                            dyn_keys.push(Some(JSW_STYLE.as_arg()))
                        } else {
                            flag |= PatchFlag::STYLE
                        }
                    }

                    props.push_prop(PROP_NAME_STYLE, value)
                },
                Key::OnClick => {
                    if is_dyn && is_cmpt {
                        flag |= PatchFlag::PROPS;

                        dyn_keys.push(Some(ON_CLICK.as_arg()))
                    }

                    props.push_ident_prop(ON_CLICK, value)
                },
                Key::Model(arg) => {
                    if !is_dyn {
                        panic!("Forbidden: const v-model value")
                    }

                    let arg = arg.unwrap_or_else(|| MODEL_VALUE);
                    let name = format!("onUpdate:{arg}");

                    if is_cmpt {
                        props.push_ident_prop(name.clone(), value.clone());

                        flag |= PatchFlag::PROPS;
                        dyn_keys.push(Some(arg.as_arg()))
                    } else {
                        flag |= PatchFlag::NEED_PATCH;
                        directives.push_directive(MODEL, value.clone())
                    }

                    let listener = v_model_listener(value, state);

                    props.push_prop(Str::from(name).into(), listener);
                },
                Key::Event(name) => {
                    let event_name = format!("on{}{}", name[..1].to_uppercase(), &name[1..]);

                    if is_dyn {
                        if is_cmpt {
                            flag |= PatchFlag::PROPS;

                            dyn_keys.push(Some(event_name.clone().as_arg()))
                        } else {
                            flag |= PatchFlag::HYDRATE_EVENTS
                        }
                    }

                    props.push_ident_prop(event_name, value);
                },
                Key::Directive(name) => {
                    flag |= PatchFlag::NEED_PATCH;

                    directives.push_directive(name, value);
                },
                Key::Attr(&ref name) => {
                    if is_dyn {
                        flag |= PatchFlag::PROPS;

                        dyn_keys.push(Some(name.as_arg()))
                    }

                    props.push_ident_prop(name, value);
                },
                Key::NSAttr { ns, name } => {
                    let ns_name = format!("{ns}:{name}");

                    if is_dyn {
                        flag |= PatchFlag::PROPS;

                        dyn_keys.push(Some(ns_name.clone().as_arg()))
                    }

                    props.push_ident_prop(ns_name, value);
                },
                Key::Spread => {
                    flag |= PatchFlag::FULL_PROPS;

                    spreads.push(ExprOrSpread::from(value));
                },
            }
        });

        let tag_arg = tag.convert(state).as_arg();

        let props_arg = create_props_expr(props, spreads, state).as_arg();

        let children_arg = children.convert(state).as_arg();

        // TODO: models, directives

        if *is_static {
            let expr = create_element_node(
                tag_arg,
                props_arg,
                children_arg,
                PatchFlag::HOISTED,
                dyn_keys,
                state,
            );

            state.hoist_expr(expr).into()
        } else {
            let expr = create_element_node(tag_arg, props_arg, children_arg, flag, dyn_keys, state);

            if directives.is_empty() {
                expr
            } else {
                create_with_directives(expr, directives, state)
            }
        }
    }
}
