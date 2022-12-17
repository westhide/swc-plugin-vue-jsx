/// ## Clean JSX Text
/// - Babel implementation [cleanJSXElementLiteralChild](https://github.com/babel/babel/blob/f5b52208f05157a348fdfaa0222c07a9a83fb101/packages/babel-types/src/utils/react/cleanJSXElementLiteralChild.ts#L5)
/// - SWC Impl [jsx_text_to_str](https://github.com/swc-project/swc/blob/b97655106525536af62ddd53780e0dbdf752b545/crates/swc_ecma_transforms_react/src/jsx/mod.rs#L1264)
/// - [Unicode PropList](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)
/// - [DOM Parser](https://html.spec.whatwg.org/multipage/dynamic-markup-insertion.html#dom-parsing-and-serialization())
pub fn clean_jsx_text(text: &str) -> String {
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

#[cfg(test)]
mod tests {
    // extern crate test;

    use super::*;

    macro_rules! test {
        ($name:ident, $input:literal, $expected:literal) => {
            #[test]
            fn $name() {
                let text = &clean_jsx_text($input);
                assert_eq!(text, $expected)
            }
        };
    }

    test!(case1, "a", "a");
    test!(case2, "a b", "a b");
    test!(case3, " a ", " a ");
    test!(case4, "\ta", " a");
    test!(case5, "\n  a ", "a ");
    test!(case6, "\r\n  a ", "a ");
    test!(case7, "a \n", "a ");
    test!(case8, "a \n ", "a");
}
