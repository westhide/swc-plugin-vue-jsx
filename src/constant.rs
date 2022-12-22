use phf::{phf_set, Set};
use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{Expr, Ident, Lit, Null, PropName, Str},
        atoms::{js_word, JsWord},
    },
};

/// ### VNode Prop [Key]
pub const REF: &str = "ref";
pub const KEY: &str = "key";
pub const CLASS: &str = "class";
pub const STYLE: &str = "style";
pub const ON_CLICK: &str = "onClick";

pub const TEXT_CONTENT: &str = "textContent";
pub const INNER_HTML: &str = "innerHTML";
pub const MODEL: &str = "model";
pub const MODEL_VALUE: &str = "modelValue";

pub const JSW_KEY: JsWord = js_word!("key");
pub const JSW_CLASS: JsWord = js_word!("class");
pub const JSW_STYLE: JsWord = js_word!("style");

pub const PROP_NAME_KEY: PropName = PropName::Ident(Ident::new(JSW_KEY, DUMMY_SP));
pub const PROP_NAME_CLASS: PropName = PropName::Ident(Ident::new(JSW_CLASS, DUMMY_SP));
pub const PROP_NAME_STYLE: PropName = PropName::Ident(Ident::new(JSW_STYLE, DUMMY_SP));

pub const EMPTY_STRING_LIT: Lit = Lit::Str(Str {
    span: DUMMY_SP,
    value: js_word!(""),
    raw: None,
});
pub const EMPTY_STRING_EXPR: Expr = Expr::Lit(EMPTY_STRING_LIT);

pub const NULL_EXPR: Expr = Expr::Lit(Lit::Null(Null { span: DUMMY_SP }));

/// ## KEY WORD
pub const JSX_HELPER_KEY: &str = "JSX_HELPER_KEY";
pub const FRAGMENT: &str = "Fragment";
pub const KEEP_ALIVE: &str = "KeepAlive";

pub const EMPTY_STR: &str = "";
pub const UNDEFINED: &str = "undefined";

pub const V_MODEL_NATIVE_ELEMENT: &[&str; 3] = &["input", "textarea", "select"];

/// ## [HTML ELEMENT](https://html.spec.whatwg.org/multipage/indices.html#elements-3)
pub const HTML_ELEMENT: &Set<&str> = &phf_set! {
    "a","abbr","address","area","article","aside","audio",
    "b","base","bdi","bdo","blockquote","body","br","button",
    "canvas","caption","cite","code","col","colgroup",
    "data","datalist","dd","del","details","dfn","dialog","div","dl","dt",
    "em","embed",
    "fieldset","figcaption","figure","footer","form",
    "h1","h2","h3","h4","h5","h6",
    "head","header","hgroup","hr","html",
    "i","iframe","img","input","ins",
    "kbd",
    "label","legend","li","link",
    "main","map","mark","menu","meta","meter",
    "nav","noscript",
    "object","ol","optgroup","option","output",
    "p","picture","pre","progress",
    "q",
    "rp","rt","ruby",
    "s","samp","script","section","select","slot","small","source",
    "span","strong","style","sub","summary","sup","svg",
    "table","tbody","td","template","textarea","tfoot",
    "th","thead","time","title","tr","track",
    "u","ul",
    "var","video",
    "wbr",
};

/// ## [SVG Element](https://svgwg.org/svg2-draft/eltindex.html)
pub const SVG_ELEMENT: &Set<&str> = &phf_set! {
    "a","animate","animateMotion","animateTransform","circle","clipPath",
    "defs","desc","discard","ellipse",
    "feBlend","feColorMatrix","feComponentTransfer","feComposite","feConvolveMatrix",
    "feDiffuseLighting","feDisplacementMap","feDistantLight","feDropShadow",
    "feFlood","feFuncA","feFuncB","feFuncG","feFuncR","feGaussianBlur",
    "feImage","feMerge","feMergeNode","feMorphology","feOffset","fePointLight",
    "feSpecularLighting","feSpotLight","feTile","feTurbulence","filter","foreignObject",
    "g","image","line","linearGradient",
    "marker","mask","metadata","mpath",
    "path","pattern","polygon","polyline",
    "radialGradient","rect",
    "script","set","stop","style","svg","switch","symbol",
    "text","textPath","title","tspan",
    "use","view",
};

/// ## [Boolean attributes](https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#boolean-attributes)
/// [Attributes](https://html.spec.whatwg.org/multipage/indices.html#attributes-3)
pub const BOOLEAN_ATTRIBUTE: &Set<&str> = &phf_set! {
    "allowfullscreen","async","autofocus","autoplay",
    "checked","controls",
    "default","defer","disabled",
    "formnovalidate","hidden",
    "inert","isMap","itemscope",
    "loop",
    "multiple","muted",
    "nomodule","novalidate",
    "open",
    "playsinline",
    "readonly","required","reversed",
    "selected",
};
