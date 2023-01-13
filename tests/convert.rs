use insta::assert_snapshot;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{
            base::{fixer::fixer, hygiene::hygiene, resolver},
            testing::Tester,
        },
        visit::as_folder,
    },
};
use swc_plugin_vue_jsx::{PluginOptions, VueJSX};

const TSX_SYNTAX: Syntax = Syntax::Typescript(TsConfig {
    tsx: true,
    decorators: false,
    dts: false,
    no_early_errors: false,
});

macro_rules! test {
    ($name:ident, $src:literal, $opts:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            Tester::run(|tester| {
                let unresolved_mark = Mark::new();

                let module = tester.apply_transform(
                    chain!(
                        resolver(unresolved_mark, Mark::new(), false),
                        as_folder(VueJSX::new($opts, None, unresolved_mark)),
                        hygiene(),
                        fixer(Some(&tester.comments.clone()))
                    ),
                    "test.tsx",
                    TSX_SYNTAX,
                    $src,
                )?;

                let code = tester.print(&module, &tester.comments.clone());

                assert_snapshot!(stringify!($name), code);

                Ok(())
            });
        }
    };

    ($name:ident, $src:literal) => {
        test!($name, $src, PluginOptions::from("{}"));
    };

    ($($mod:ident:{
        $($name:ident : $src:literal),+ $(,)?
    }),+ $(,)?) => {
        $(
        #[allow(non_snake_case)]
        mod $mod {
            use super::*;
            $(test!($name,$src);)+
        }
        )+
    };
}

test!(
    Tag:{
        native_div: r#"<div></div>"#,
        extra_component: r#"let A; <A></A>"#,
        extra_resolve_component: r#"<A></A>"#,
        extra_member: r#"<A.b></A.b>"#,
    },
    Prop:{
        r#ref: r#"<div ref={a}></div>"#,
        key: r#"<div key={a}></div>"#,
        namespace: r#"<div ns:name={a}></div>"#,
        spread: r#"<div {...a}></div>"#,
        spread_with_prop: r#"<div {...a} class="cls"></div>"#,
        verify_symbol: r#"<div prop-name={a}></div>"#,
    },
    Event:{
        onClick: r#"<div onClick={fn}></div>"#,
        prefix_on: r#"<div onEvent={fn}></div>"#,
        namespace: r#"<div on:event={fn}></div>"#,
    },
    Directive:{
        vText: r#"<div v-text="text1"></div>"#,
        vHtml: r#"<div v-html="<div></div>"></div>"#,
        vShow: r#"<div v-show={a}></div>"#,
        vSlots: r#"<div v-slots={slots}></div>"#,
        vModel: r#"<input v-model={a} />"#,
        vModel_with_key: r#"<A v-model:key={a} />"#,
        custom: r#"<div v-custom={a}></div>"#,
    },
    Element:{
        expr_child: r#"<div>{a}</div>"#,
        spread_child: r#"<div>{...a}</div>"#,
        jsx_child: r#"<div>{<div></div>}</div>"#,
        fragment_child: r#"<div>{<></>}</div>"#,
    },
    PatchFlag:{
        dyn_class: r#"<div class={a}></div>"#,
        dyn_style: r#"<div style={a}></div>"#,
        dyn_prop: r#"<div prop={a}></div>"#,
        full_props: r#"<div {...a}></div>"#,
        hydration_event: r#"<div onEvent={fn}></div>"#,
        need_patch: r#"<div ref={fn}></div>"#,
    },
    Fragment:{
        children: r#"
          <>
            <div></div>
            <div></div>
          </>
        "#,
    },
    Text:{
        clean_text: r#"
            <div>
             text1
             <br/>
       text2
             </div>
        "#,
    },
    StaticVNode:{
        below_threshold: r#"
          <>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
          </>
        "#,
        above_threshold: r#"
          <>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
          </>
        "#
    }
);

test!(
    Tag_custom_element,
    r#"<custom-tag></custom-tag>"#,
    PluginOptions::from(r#"{ "customElementPatterns":["custom-tag"] }"#)
);
