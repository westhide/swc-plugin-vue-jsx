use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, JSXText},
        utils::ExprFactory,
    },
};

use crate::{
    shared::{convert::Convert, transform::Transform},
    state::State,
};

#[derive(Debug)]
pub struct Text<'a> {
    pub content: String,

    pub raw: &'a JSXText,
}

impl<'a> Text<'a> {
    /// ## Clean JSX Text
    /// - Babel implementation [cleanJSXElementLiteralChild](https://github.com/babel/babel/blob/f5b52208f05157a348fdfaa0222c07a9a83fb101/packages/babel-types/src/utils/react/cleanJSXElementLiteralChild.ts#L5)
    /// - SWC Impl [jsx_text_to_str](https://github.com/swc-project/swc/blob/b97655106525536af62ddd53780e0dbdf752b545/crates/swc_ecma_transforms_react/src/jsx/mod.rs#L1264)
    /// - [Unicode PropList](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)
    /// - [DOM Parser](https://html.spec.whatwg.org/multipage/dynamic-markup-insertion.html#dom-parsing-and-serialization())
    pub fn clean(text: &str) -> String {
        let replaced = text.replace('\t', " ");

        let mut lines = replaced.lines().enumerate().peekable();

        let mut buf = String::new();

        while let Some((i, mut line)) = lines.next() {
            if i != 0 {
                line = line.trim_start_matches(' ')
            }

            if lines.peek().is_some() {
                line = line.trim_end_matches(' ')
            }

            if line.is_empty() {
                continue;
            }

            if i != 0 && !buf.is_empty() {
                buf.push(' ')
            }

            buf.push_str(line);
        }

        buf
    }
}

impl<'a> Transform<'a, Option<Text<'a>>> for JSXText {
    fn transform(&'a self) -> Option<Text<'a>> {
        let content = Text::clean(self.value.as_ref());

        if content.is_empty() {
            None
        } else {
            Some(Text { content, raw: self })
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Text<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        let Text { content, .. } = self;

        let create_text_node = state.import_from_vue("createTextVNode");

        let hoist_expr = create_text_node.as_call(DUMMY_SP, vec![content.clone().as_arg()]);

        state.hoist_expr(hoist_expr).into()
    }
}

#[cfg(test)]
mod tests {
    // extern crate test;
    use super::*;

    macro_rules! test {
        ($name:ident, $input:literal, $expected:literal) => {
            #[test]
            fn $name() {
                let content = &Text::clean($input);
                assert_eq!(content, $expected)
            }
        };
    }

    test!(clean_text_1, "a", "a");
    test!(clean_text_2, "a b", "a b");
    test!(clean_text_3, " a ", " a ");
    test!(clean_text_4, "\ta", " a");
    test!(clean_text_5, "\n  a ", "a ");
    test!(clean_text_6, "\r\n  a ", "a ");
    test!(clean_text_7, "a \n", "a ");
    test!(clean_text_8, "a \n ", "a");
}
