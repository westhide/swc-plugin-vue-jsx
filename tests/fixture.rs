use std::path::PathBuf;

use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{
            base::resolver,
            testing::{test, test_fixture},
        },
        visit::as_folder,
    },
};
use swc_plugin_vue_jsx::{PluginOptions, VueJSX};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/*.tsx", exclude("output.tsx"))]
fn tsx_fixture(input: PathBuf) {
    let file_stem = input.file_stem().unwrap().to_str().unwrap();

    let filename = format!("{file_stem}.output.tsx");

    let output = input.with_file_name(filename);

    test_fixture(
        syntax(),
        &|tester| {
            let opts = PluginOptions::from(
                r#"{
                    "customElementPatterns":["diva"]
                }"#,
            );

            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(VueJSX::new(
                    opts,
                    Some(tester.comments.as_ref().clone()),
                    Mark::new()
                )),
                // hygiene()
            )
        },
        &input,
        &output,
        Default::default(),
    )
}
