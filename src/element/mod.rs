use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            op, ArrayLit, ArrowExpr, AssignExpr, Expr, ExprOrSpread, KeyValueProp, ObjectLit, Prop,
            PropOrSpread,
        },
        utils::{quote_ident, ExprFactory},
    },
};
use swc_helper_jsx_transform::{
    attr::{key::Key, Attr},
    element::{tag::Tag, Element},
};

use crate::{
    args,
    constant::{
        CLASS, INNER_HTML, KEY, MODEL_VALUE, NULL_EXPR, ON_CLICK, REF, STYLE, TEXT_CONTENT, V_HTML,
        V_MODEL, V_SLOTS, V_TEXT,
    },
    context::Context,
    element::directive::Directive,
    patch_flag::PatchFlag,
    shared::{add::Add, convert::Convert, expr::ExprExtend},
    utils::is::is_directive,
};

pub mod attr_value;
pub mod directive;
pub mod tag;

fn is_component<'a>(tag: &Tag, ctx: &mut impl Context<'a>) -> bool {
    match tag {
        Tag::Native(_) => false,
        Tag::Extra(name) => !ctx.is_custom_element(name),
        Tag::Member(_) => true,
    }
}

#[derive(Debug)]
pub struct State<'s> {
    props: Vec<PropOrSpread>,
    spreads: Vec<ExprOrSpread>,

    slots: Option<Expr>,

    flag: isize,
    dyn_keys: Vec<Option<ExprOrSpread>>,

    directives: Vec<Directive<'s>>,

    raw: &'s Element<'s>,
}

impl<'s> State<'s> {
    pub fn new(element: &'s Element<'s>) -> Self {
        Self {
            props: Vec::new(),
            spreads: Vec::new(),
            slots: None,
            flag: 0,
            dyn_keys: Vec::new(),
            directives: Vec::new(),
            raw: element,
        }
    }
}

impl<'s> State<'s> {
    fn has_dyn_class(&mut self) {
        self.flag |= PatchFlag::CLASS
    }

    fn has_dyn_style(&mut self) {
        self.flag |= PatchFlag::STYLE
    }

    fn has_dyn_prop(&mut self) {
        self.flag |= PatchFlag::PROPS
    }

    fn has_dyn_slot(&mut self) {
        self.flag |= PatchFlag::DYNAMIC_SLOTS
    }

    fn has_hydration_event(&mut self) {
        self.flag |= PatchFlag::HYDRATE_EVENTS
    }

    fn need_patch(&mut self) {
        self.flag |= PatchFlag::NEED_PATCH
    }

    fn has_full_props(&mut self) {
        self.flag |= PatchFlag::FULL_PROPS
    }

    fn add_prop(&mut self, name: &str, value: Expr) {
        let prop = Prop::KeyValue(KeyValueProp {
            key: quote_ident!(name).into(),
            value: box value,
        });

        self.props.push(prop.into())
    }

    fn add_spread(&mut self, value: Expr) {
        self.spreads.add(value)
    }

    fn add_dyn_key(&mut self, name: &str) {
        self.has_dyn_prop();

        self.dyn_keys.push(Some(name.as_arg()))
    }

    fn add_directive(&mut self, name: &'s str, value: Expr) {
        self.directives.push(Directive { name, value })
    }
}

impl<'a, 's> State<'s> {
    fn add_on_update(&mut self, key: &str, value: Expr, ctx: &mut impl Context<'a>) {
        let param = ctx.get_ident("$v");

        let listener = ArrowExpr {
            params: vec![param.clone().into()],
            body: AssignExpr {
                span: DUMMY_SP,
                left: value.as_pat_or_expr(),
                op: op!("="),
                right: param.into(),
            }
            .into(),
            ..Take::dummy()
        }
        .into();

        self.add_prop(&format!("onUpdate:{key}"), listener)
    }

    pub fn convert_into_expr(self, ctx: &mut impl Context<'a>) -> Expr {
        let Self {
            props,
            mut spreads,
            slots,
            flag,
            dyn_keys,
            directives,
            raw:
                Element {
                    tag,
                    children,
                    is_static,
                    ..
                },
        } = self;

        let tag_expr = tag.convert(ctx);

        let props_expr = if props.is_empty() {
            if spreads.is_empty() {
                NULL_EXPR
            } else {
                ctx.merge_props(spreads)
            }
        } else {
            let props_obj = ObjectLit {
                span: DUMMY_SP,
                props,
            };

            if spreads.is_empty() {
                props_obj.into()
            } else {
                spreads.add(props_obj);

                ctx.merge_props(spreads)
            }
        };

        let children_or_slots = slots.unwrap_or_else(|| {
            if children.is_empty() {
                NULL_EXPR
            } else {
                children.convert(ctx)
            }
        });

        let mut args = args![tag_expr, props_expr, children_or_slots];

        if *is_static {
            args.add(PatchFlag::HOISTED as f64);

            return ctx.create_element_vnode(args).hoist_to_module(ctx);
        }

        if flag != 0 {
            args.add(flag as f64);

            if !dyn_keys.is_empty() {
                args.add(ArrayLit {
                    span: DUMMY_SP,
                    elems: dyn_keys,
                })
            }
        }

        let element_expr = ctx.create_vnode(args);

        if directives.is_empty() {
            element_expr
        } else {
            let directives_expr: Expr = ArrayLit {
                span: DUMMY_SP,
                elems: directives
                    .into_iter()
                    .map(|directive| directive.convert_into_expr(ctx))
                    .map(ExprFactory::as_arg)
                    .map(Some)
                    .collect(),
            }
            .into();

            ctx.with_directive(args![element_expr, directives_expr])
        }
    }
}

impl<'a, 'b> Convert<'a, Expr> for Element<'b> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> Expr {
        let Self { tag, attrs, .. } = self;

        let is_cmpt = is_component(tag, ctx);

        let mut state = State::new(self);

        attrs.iter().for_each(|Attr { key, value }| {
            let is_dyn = !value.is_static();

            let value = value.convert(ctx);

            match key {
                Key::Attr(REF) => {
                    if is_dyn {
                        state.need_patch()
                    }

                    state.add_prop(REF, value)
                },
                Key::Attr(KEY) => state.add_prop(KEY, value),
                Key::Attr(CLASS) => {
                    if is_dyn {
                        if is_cmpt {
                            state.add_dyn_key(CLASS)
                        } else {
                            state.has_dyn_class()
                        }
                    }

                    state.add_prop(CLASS, value)
                },
                Key::Attr(STYLE) => {
                    if is_dyn {
                        if is_cmpt {
                            state.add_dyn_key(STYLE)
                        } else {
                            state.has_dyn_style()
                        }
                    }

                    state.add_prop(STYLE, value)
                },

                Key::Attr(V_TEXT) => {
                    if is_dyn {
                        state.add_dyn_key(TEXT_CONTENT)
                    }

                    state.add_prop(TEXT_CONTENT, value);
                },
                Key::Attr(V_HTML) => {
                    if is_dyn {
                        state.add_dyn_key(INNER_HTML)
                    }

                    state.add_prop(INNER_HTML, value);
                },

                Key::Attr(V_SLOTS) => {
                    state.has_dyn_slot();

                    state.slots = Some(value)
                },

                Key::Attr(V_MODEL) => {
                    if is_cmpt {
                        state.add_dyn_key(MODEL_VALUE);

                        state.add_prop(MODEL_VALUE, value.clone())
                    } else {
                        state.need_patch();

                        state.add_directive(V_MODEL, value.clone())
                    }

                    state.add_on_update(MODEL_VALUE, value, ctx)
                },

                Key::Attr(name) if is_directive(name) => {
                    state.need_patch();

                    state.add_directive(name, value)
                },

                Key::Attr(name) => {
                    if is_dyn {
                        state.add_dyn_key(name)
                    }

                    state.add_prop(name, value);
                },

                Key::Event("click" | "Click") => {
                    if is_cmpt {
                        state.add_dyn_key(ON_CLICK)
                    }

                    state.add_prop(ON_CLICK, value)
                },
                Key::Event(name) => {
                    let event_name = format!("on{}{}", name[..1].to_uppercase(), &name[1..]);

                    if is_cmpt {
                        state.add_dyn_key(&event_name)
                    } else {
                        state.has_hydration_event()
                    }

                    state.add_prop(&event_name, value);
                },

                Key::NSAttr {
                    ns: V_MODEL,
                    name: key,
                } => {
                    state.add_dyn_key(key);

                    state.add_prop(key, value.clone());

                    state.add_on_update(key, value, ctx)
                },

                Key::NSAttr { ns, name } => {
                    let ns_name = format!("{ns}:{name}");

                    if is_dyn {
                        state.add_dyn_key(&ns_name)
                    }

                    state.add_prop(&ns_name, value);
                },
                Key::Spread => {
                    state.has_full_props();

                    state.add_spread(value)
                },
            }
        });

        state.convert_into_expr(ctx)
    }
}
