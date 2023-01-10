use insta::assert_snapshot;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{
            base::{fixer::fixer, hygiene::hygiene},
            testing::Tester,
        },
        visit::as_folder,
    },
    plugin::proxies::PluginCommentsProxy,
};
use swc_plugin_vue_jsx::{PluginOptions, VueJSX};

const TSX_SYNTAX: Syntax = Syntax::Typescript(TsConfig {
    tsx: true,
    decorators: false,
    dts: false,
    no_early_errors: false,
});

macro_rules! test {
    ($name:ident, $src:literal) => {
        #[test]
        fn $name() {
            Tester::run(|tester| {
                let module = tester.apply_transform(
                    chain!(
                        as_folder(VueJSX::<PluginCommentsProxy>::new(
                            PluginOptions::default(),
                            None,
                            Mark::new(),
                        )),
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

    ({ $($name:ident : $src:literal),+ $(,)? }) => {
        $(test!($name,$src);)+
    };
}

test!({
    div:r#"const app = <>
            <A></A>
            123
            {d}
            {...e}
            <div frg={<div></div>}></div>

            123
            <span></span>
            <span></span>
            <span></span>
            <span></span>
            <div></div>
            <div></div>
    </>"#
});
