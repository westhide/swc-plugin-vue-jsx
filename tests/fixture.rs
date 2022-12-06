use std::path::PathBuf;

use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{
            base::{hygiene::hygiene, resolver},
            testing::{test, test_fixture},
        },
        // visit::as_folder,
    },
};
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
        &|_| chain!(resolver(Mark::new(), Mark::new(), false), hygiene()),
        &input,
        &output,
        Default::default(),
    )
}
