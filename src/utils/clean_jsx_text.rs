use crate::regex;

/// ## Clean JSX Text
/// - Babel implementation [cleanJSXElementLiteralChild](https://github.com/babel/babel/blob/f5b52208f05157a348fdfaa0222c07a9a83fb101/packages/babel-types/src/utils/react/cleanJSXElementLiteralChild.ts#L5)
/// - SWC Impl [jsx_text_to_str](https://github.com/swc-project/swc/blob/b97655106525536af62ddd53780e0dbdf752b545/crates/swc_ecma_transforms_react/src/jsx/mod.rs#L1264)
/// - [Unicode PropList](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)
/// - [DOM Parser](https://html.spec.whatwg.org/multipage/dynamic-markup-insertion.html#dom-parsing-and-serialization())
pub fn clean_jsx_text(text: &str) -> String {
    let text = text.replace("\t", " ");

    regex!(r"(?:[^ ])[ ]*(\r\n|\n|\r)[ ]*")
        .replace_all(&text, "-")
        .to_string()
}
