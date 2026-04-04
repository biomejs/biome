use std::{borrow::Cow, collections::HashMap, sync::LazyLock};

use biome_html_syntax::{AnyHtmlElement, AnyHtmlTagName, HtmlAttributeName, HtmlTagName};
use biome_string_case::{StrLikeExtension, StrOnlyExtension};

use crate::{
    HtmlFormatter,
    utils::css_display::{CssDisplay, get_css_display},
};

/// HTML tags that have a "block" layout, or anything that is not inline by default.
///
/// **NOTE**: This list is kept for reference but is no longer used directly.
/// Use [`crate::utils::css_display::get_css_display`] instead for accurate CSS display values.
///
/// ### References
///  - <https://html.spec.whatwg.org/#flow-content-3>
///  - <https://html.spec.whatwg.org/#sections-and-headings>
///  - <https://html.spec.whatwg.org/#lists>
///  - <https://github.com/prettier/prettier/blob/af6e7215ce0e0d243cb34a85174af65ab4177f47/src/language-html/constants.evaluate.js>
#[deprecated]
pub const HTML_BLOCK_TAGS: &[&str] = &[
    // These have `block` explicitly
    "html",
    "body",
    "address",
    "blockquote",
    "center",
    "dialog",
    "div",
    "figure",
    "figcaption",
    "footer",
    "form",
    "header",
    "hr",
    "legend",
    "listing",
    "main",
    "p",
    "plaintext",
    "pre",
    "search",
    "xmp",
    "article",
    "aside",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hgroup",
    "nav",
    "section",
    "dir",
    "dd",
    "dl",
    "dt",
    "menu",
    "ol",
    "ul",
    "details",
    "summary",
    // These have display: none
    "area",
    "base",
    "basefont",
    "datalist",
    "head",
    "link",
    "meta",
    "noembed",
    "noframes",
    "param",
    "rp",
    "script",
    "style",
    "template",
    "title",
    "source",
    "track",
    "param",
    // These have others, but for our purposes, we consider them block-level elements
    "li",
    "table",
    "caption",
    "option",
    "optgroup",
    "video",
    "audio",
    "select",
    "object",
    "meter",
    "progress",
];

/// A list of all HTML tags that are or have been specified in the HTML specification. Although technically an element can have any name, this is necessary to know whether or not the formatter should normalize the casing of the tag name, and it's attribute names.
///
/// This does contain some elements that are deprecated, but browsers still support for backwards compatibility.
///
/// TODO: Ideally, this would be codegened from the actual HTML spec.
///
/// Prettier grabs it's known tag names from the `html-tag-names` npm package.
/// - <https://github.com/prettier/prettier/blob/main/src/language-html/utils/html-tag-names.evaluate.js>
/// - <https://www.npmjs.com/package/html-tag-names>
///
/// Ref: <https://html.spec.whatwg.org/multipage/semantics.html#semantics>
pub const HTML_ALL_TAGS: &[&str] = &[
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "basefont",
    "bdi",
    "bdo",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "em",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "kbd",
    "label",
    "legend",
    "li",
    "li",
    "link",
    "listing",
    "main",
    "map",
    "mark",
    "menu",
    "meta",
    "meter",
    "nav",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "plaintext",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "search",
    "section",
    "select",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
];

/// Canonical attributes that are global to all HTML elements.
///
/// "Canonical" refers to a non-wildcard name that has been a part of web standards, regardless of it's current deprecation status.
///
/// Prettier normalizes the casing of attributes if the tag name is known, and the attribute name is known.
///
/// Prettier grabs it's known attributes from the `html-element-attributes` npm package.
/// - <https://github.com/prettier/prettier/blob/main/src/language-html/utils/html-elements-attributes.evaluate.js>
/// - <https://www.npmjs.com/package/html-element-attributes>
/// - <https://github.com/wooorm/html-element-attributes/blob/main/index.js>
pub const HTML_GLOBAL_ATTRIBUTES: &[&str] = &[
    "accesskey",
    "autocapitalize",
    "autofocus",
    "class",
    "contenteditable",
    "dir",
    "draggable",
    "enterkeyhint",
    "hidden",
    "id",
    "inert",
    "inputmode",
    "is",
    "itemid",
    "itemprop",
    "itemref",
    "itemscope",
    "itemtype",
    "lang",
    "nonce",
    "popover",
    "slot",
    "spellcheck",
    "style",
    "tabindex",
    "title",
    "translate",
    "writingsuggestions",
];

/// Canonical attributes that are known per tag name.
///
/// "Canonical" refers to a non-wildcard name that has been a part of web standards, regardless of it's current deprecation status.
///
/// Prettier normalizes the casing of attributes if the tag name is known, and the attribute name is known.
///
/// Prettier grabs it's known attributes from the `html-element-attributes` npm package.
/// - <https://github.com/prettier/prettier/blob/main/src/language-html/utils/html-elements-attributes.evaluate.js>
/// - <https://www.npmjs.com/package/html-element-attributes>
/// - <https://github.com/wooorm/html-element-attributes/blob/main/index.js>
pub static HTML_ATTRIBUTES_BY_TAG: LazyLock<HashMap<&str, &[&str]>> = LazyLock::new(|| {
    let attributes_by_tag: &[(&str, &[&str])] = &[
        (
            "a",
            &[
                "charset",
                "coords",
                "download",
                "href",
                "hreflang",
                "name",
                "ping",
                "referrerpolicy",
                "rel",
                "rev",
                "shape",
                "target",
                "type",
            ],
        ),
        (
            "applet",
            &[
                "align", "alt", "archive", "code", "codebase", "height", "hspace", "name",
                "object", "vspace", "width",
            ],
        ),
        (
            "area",
            &[
                "alt",
                "coords",
                "download",
                "href",
                "hreflang",
                "nohref",
                "ping",
                "referrerpolicy",
                "rel",
                "shape",
                "target",
                "type",
            ],
        ),
        (
            "audio",
            &[
                "autoplay",
                "controls",
                "crossorigin",
                "loop",
                "muted",
                "preload",
                "src",
            ],
        ),
        ("base", &["href", "target"]),
        ("basefont", &["color", "face", "size"]),
        ("blockquote", &["cite"]),
        (
            "body",
            &["alink", "background", "bgcolor", "link", "text", "vlink"],
        ),
        ("br", &["clear"]),
        (
            "button",
            &[
                "disabled",
                "form",
                "formaction",
                "formenctype",
                "formmethod",
                "formnovalidate",
                "formtarget",
                "name",
                "popovertarget",
                "popovertargetaction",
                "type",
                "value",
            ],
        ),
        ("canvas", &["height", "width"]),
        ("caption", &["align"]),
        (
            "col",
            &["align", "char", "charoff", "span", "valign", "width"],
        ),
        (
            "colgroup",
            &["align", "char", "charoff", "span", "valign", "width"],
        ),
        ("data", &["value"]),
        ("del", &["cite", "datetime"]),
        ("details", &["name", "open"]),
        ("dialog", &["open"]),
        ("dir", &["compact"]),
        ("div", &["align"]),
        ("dl", &["compact"]),
        ("embed", &["height", "src", "type", "width"]),
        ("fieldset", &["disabled", "form", "name"]),
        ("font", &["color", "face", "size"]),
        (
            "form",
            &[
                "accept",
                "accept-charset",
                "action",
                "autocomplete",
                "enctype",
                "method",
                "name",
                "novalidate",
                "target",
            ],
        ),
        (
            "frame",
            &[
                "frameborder",
                "longdesc",
                "marginheight",
                "marginwidth",
                "name",
                "noresize",
                "scrolling",
                "src",
            ],
        ),
        ("frameset", &["cols", "rows"]),
        ("h1", &["align"]),
        ("h2", &["align"]),
        ("h3", &["align"]),
        ("h4", &["align"]),
        ("h5", &["align"]),
        ("h6", &["align"]),
        ("head", &["profile"]),
        ("hr", &["align", "noshade", "size", "width"]),
        ("html", &["manifest", "version"]),
        (
            "iframe",
            &[
                "align",
                "allow",
                "allowfullscreen",
                "allowpaymentrequest",
                "allowusermedia",
                "frameborder",
                "height",
                "loading",
                "longdesc",
                "marginheight",
                "marginwidth",
                "name",
                "referrerpolicy",
                "sandbox",
                "scrolling",
                "src",
                "srcdoc",
                "width",
            ],
        ),
        (
            "img",
            &[
                "align",
                "alt",
                "border",
                "crossorigin",
                "decoding",
                "fetchpriority",
                "height",
                "hspace",
                "ismap",
                "loading",
                "longdesc",
                "name",
                "referrerpolicy",
                "sizes",
                "src",
                "srcset",
                "usemap",
                "vspace",
                "width",
            ],
        ),
        (
            "input",
            &[
                "accept",
                "align",
                "alt",
                "autocomplete",
                "checked",
                "dirname",
                "disabled",
                "form",
                "formaction",
                "formenctype",
                "formmethod",
                "formnovalidate",
                "formtarget",
                "height",
                "ismap",
                "list",
                "max",
                "maxlength",
                "min",
                "minlength",
                "multiple",
                "name",
                "pattern",
                "placeholder",
                "popovertarget",
                "popovertargetaction",
                "readonly",
                "required",
                "size",
                "src",
                "step",
                "type",
                "usemap",
                "value",
                "width",
            ],
        ),
        ("ins", &["cite", "datetime"]),
        ("isindex", &["prompt"]),
        ("label", &["for", "form"]),
        ("legend", &["align"]),
        ("li", &["type", "value"]),
        (
            "link",
            &[
                "as",
                "blocking",
                "charset",
                "color",
                "crossorigin",
                "disabled",
                "fetchpriority",
                "href",
                "hreflang",
                "imagesizes",
                "imagesrcset",
                "integrity",
                "media",
                "referrerpolicy",
                "rel",
                "rev",
                "sizes",
                "target",
                "type",
            ],
        ),
        ("map", &["name"]),
        ("menu", &["compact"]),
        (
            "meta",
            &[
                "charset",
                "content",
                "http-equiv",
                "media",
                "name",
                "scheme",
            ],
        ),
        ("meter", &["high", "low", "max", "min", "optimum", "value"]),
        (
            "object",
            &[
                "align",
                "archive",
                "border",
                "classid",
                "codebase",
                "codetype",
                "data",
                "declare",
                "form",
                "height",
                "hspace",
                "name",
                "standby",
                "type",
                "typemustmatch",
                "usemap",
                "vspace",
                "width",
            ],
        ),
        ("ol", &["compact", "reversed", "start", "type"]),
        ("optgroup", &["disabled", "label"]),
        ("option", &["disabled", "label", "selected", "value"]),
        ("output", &["for", "form", "name"]),
        ("p", &["align"]),
        ("param", &["name", "type", "value", "valuetype"]),
        ("pre", &["width"]),
        ("progress", &["max", "value"]),
        ("q", &["cite"]),
        (
            "script",
            &[
                "async",
                "blocking",
                "charset",
                "crossorigin",
                "defer",
                "fetchpriority",
                "integrity",
                "language",
                "nomodule",
                "referrerpolicy",
                "src",
                "type",
            ],
        ),
        (
            "select",
            &[
                "autocomplete",
                "disabled",
                "form",
                "multiple",
                "name",
                "required",
                "size",
            ],
        ),
        ("slot", &["name"]),
        (
            "source",
            &["height", "media", "sizes", "src", "srcset", "type", "width"],
        ),
        ("style", &["blocking", "media", "type"]),
        (
            "table",
            &[
                "align",
                "bgcolor",
                "border",
                "cellpadding",
                "cellspacing",
                "frame",
                "rules",
                "summary",
                "width",
            ],
        ),
        ("tbody", &["align", "char", "charoff", "valign"]),
        (
            "td",
            &[
                "abbr", "align", "axis", "bgcolor", "char", "charoff", "colspan", "headers",
                "height", "nowrap", "rowspan", "scope", "valign", "width",
            ],
        ),
        (
            "template",
            &[
                "shadowrootclonable",
                "shadowrootdelegatesfocus",
                "shadowrootmode",
            ],
        ),
        (
            "textarea",
            &[
                "autocomplete",
                "cols",
                "dirname",
                "disabled",
                "form",
                "maxlength",
                "minlength",
                "name",
                "placeholder",
                "readonly",
                "required",
                "rows",
                "wrap",
            ],
        ),
        ("tfoot", &["align", "char", "charoff", "valign"]),
        (
            "th",
            &[
                "abbr", "align", "axis", "bgcolor", "char", "charoff", "colspan", "headers",
                "height", "nowrap", "rowspan", "scope", "valign", "width",
            ],
        ),
        ("thead", &["align", "char", "charoff", "valign"]),
        ("time", &["datetime"]),
        ("tr", &["align", "bgcolor", "char", "charoff", "valign"]),
        ("track", &["default", "kind", "label", "src", "srclang"]),
        ("ul", &["compact", "type"]),
        (
            "video",
            &[
                "autoplay",
                "controls",
                "crossorigin",
                "height",
                "loop",
                "muted",
                "playsinline",
                "poster",
                "preload",
                "src",
                "width",
            ],
        ),
    ];
    attributes_by_tag.iter().map(|(k, v)| (*k, *v)).collect()
});

pub static SVG_ALL_TAGS: &[&str] = &[
    "animate",
    "animatemotion",
    "animatetransform",
    "circle",
    "clippath",
    "defs",
    "desc",
    "ellipse",
    "feblend",
    "fecolormatrix",
    "fecomponenttransfer",
    "fecomposite",
    "feconvolvematrix",
    "fediffuselighting",
    "fedisplacementmap",
    "fedistantlight",
    "fedropshadow",
    "feflood",
    "fefunca",
    "fefuncb",
    "fefuncg",
    "fefuncr",
    "fegaussianblur",
    "feimage",
    "femerge",
    "femergenode",
    "femorphology",
    "feoffset",
    "fepointlight",
    "fespecularlighting",
    "fespotlight",
    "fetile",
    "feturbulence",
    "filter",
    "foreignobject",
    "g",
    "image",
    "line",
    "lineargradient",
    "marker",
    "mask",
    "metadata",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialgradient",
    "rect",
    "set",
    "stop",
    "switch",
    "symbol",
    "text",
    "textpath",
    "tspan",
    "use",
    "view",
];

pub static MATHML_ALL_TAGS: &[&str] = &[
    "annotation",
    "annotation-xml",
    "maction",
    "math",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mi",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mspace",
    "msqrt",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "semantics",
];

/// Whether the given tag name is a known HTML element. See also: [`HTML_ALL_TAGS`].
pub(crate) fn is_canonical_html_tag_name(tag_name: &str) -> bool {
    match tag_name.to_ascii_lowercase_cow() {
        Cow::Owned(name) => HTML_ALL_TAGS.binary_search(&name.as_str()).is_ok(),
        Cow::Borrowed(name) => HTML_ALL_TAGS.binary_search(&name).is_ok(),
    }
}

/// Whether the given tag name is a known HTML element. See also: [`HTML_ALL_TAGS`].
pub(crate) fn is_canonical_html_tag(tag_name: &HtmlTagName) -> bool {
    let Ok(tag_name) = tag_name.value_token() else {
        return false;
    };
    is_canonical_html_tag_name(tag_name.text_trimmed())
}

/// Whether a tag should be lowercased in the current formatting context.
///
/// Returns `true` only for canonical HTML tags in pure HTML files (.html).
/// Component frameworks preserve tag name casing.
pub(crate) fn should_lowercase_html_tag(f: &HtmlFormatter, tag_name: &HtmlTagName) -> bool {
    f.options().file_source().is_html() && is_canonical_html_tag(tag_name)
}

/// Whether the given attribute name is a known HTML attribute for the given tag name.
///
/// See [`HTML_ATTRIBUTES_BY_TAG`], [`HTML_GLOBAL_ATTRIBUTES`].
pub(crate) fn is_canonical_html_attribute_name(tag_name: &str, attribute_name: &str) -> bool {
    let attribute_name = attribute_name.to_ascii_lowercase_cow();
    let is_global = match &attribute_name {
        Cow::Owned(name) => HTML_GLOBAL_ATTRIBUTES.binary_search(&name.as_str()).is_ok(),
        Cow::Borrowed(name) => HTML_GLOBAL_ATTRIBUTES.binary_search(name).is_ok(),
    };
    if is_global {
        return true;
    }
    let tag_name = tag_name.to_lowercase_cow();
    let Some(tag_attributes) = (match tag_name {
        Cow::Owned(name) => HTML_ATTRIBUTES_BY_TAG.get(name.as_str()),
        Cow::Borrowed(name) => HTML_ATTRIBUTES_BY_TAG.get(&name),
    }) else {
        return false;
    };
    match &attribute_name {
        Cow::Owned(name) => tag_attributes.binary_search(&name.as_str()).is_ok(),
        Cow::Borrowed(name) => tag_attributes.binary_search(name).is_ok(),
    }
}

/// Whether the given attribute name is a known HTML attribute for the given tag name.
///
/// See [`HTML_ATTRIBUTES_BY_TAG`], [`HTML_GLOBAL_ATTRIBUTES`].
pub(crate) fn is_canonical_html_attribute(
    tag_name: &AnyHtmlTagName,
    attribute_name: &HtmlAttributeName,
) -> bool {
    // Only check for canonical attributes on regular HTML tags, not components
    match tag_name {
        AnyHtmlTagName::HtmlTagName(tag) => {
            let Ok(tag_token) = tag.value_token() else {
                return false;
            };
            let tag_name_text = tag_token.text_trimmed();

            let Ok(attribute_token) = attribute_name.value_token() else {
                return false;
            };
            let attribute_name_text = attribute_token.text_trimmed();

            is_canonical_html_attribute_name(tag_name_text, attribute_name_text)
        }
        // Component names and member names are not canonical HTML
        AnyHtmlTagName::HtmlComponentName(_) | AnyHtmlTagName::HtmlMemberName(_) => false,
    }
}

/// Gets the CSS display value for an HTML element.
pub(crate) fn get_element_css_display(element: &AnyHtmlElement) -> CssDisplay {
    use biome_html_syntax::AnySvelteBlock;

    // Check for Svelte blocks and classify them appropriately
    if let Some(svelte_block) = element.clone().as_svelte_block() {
        return match svelte_block {
            // {@html ...} and {@render ...} are "tag" expressions that inject content inline
            // They should be treated as inline elements for whitespace sensitivity purposes
            AnySvelteBlock::SvelteHtmlBlock(_) | AnySvelteBlock::SvelteRenderBlock(_) => {
                CssDisplay::Inline
            }
            // Control flow blocks ({#if}, {#each}, etc.) and other constructs are block-level
            AnySvelteBlock::SvelteIfBlock(_)
            | AnySvelteBlock::SvelteEachBlock(_)
            | AnySvelteBlock::SvelteAwaitBlock(_)
            | AnySvelteBlock::SvelteKeyBlock(_)
            | AnySvelteBlock::SvelteSnippetBlock(_)
            | AnySvelteBlock::SvelteConstBlock(_)
            | AnySvelteBlock::SvelteDebugBlock(_)
            | AnySvelteBlock::SvelteBogusBlock(_) => CssDisplay::Block,
        };
    }

    if let Some(tag_name) = element.name() {
        get_css_display(&tag_name)
    } else {
        CssDisplay::Inline
    }
}

/// CSS whitespace handling modes.
///
/// Directly corresponds to [`white-space` CSS property values](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/white-space).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[expect(dead_code)]
pub(crate) enum CssWhitespace {
    #[default]
    Normal,
    Pre,
    PreWrap,
    PreLine,
    Wrap,
    Collapse,
    PreserveNoWrap,
}

#[cfg_attr(not(test), expect(dead_code))]
pub(crate) fn get_css_whitespace(tag_name: &str) -> CssWhitespace {
    // Mirrors Prettier's CSS white-space lookup:
    // prettier/src/language-html/constants.evaluate.js
    // prettier/src/language-html/utilities/index.js#getNodeCssStyleWhiteSpace
    //
    // Final tag mapping in Prettier:
    // - listing, plaintext, pre, xmp: "pre"
    // - textarea: "pre-wrap"
    // - nobr: "nowrap"
    // - table: "initial" (effectively treated as default "normal")

    // The cow makes this have a lower allocation chance. Also, this intentionally
    // avoids using multiple `eq_ignore_ascii_case` checks because this optimizes
    // into SIMD instructions, and overall less CPU instructions.
    let tag_name = tag_name.to_ascii_lowercase_cow();
    let tag_name = tag_name.as_ref();
    match tag_name {
        "listing" | "plaintext" | "pre" | "xmp" => CssWhitespace::Pre,
        "textarea" => CssWhitespace::PreWrap,
        "nobr" => CssWhitespace::PreserveNoWrap,
        // In Prettier this is "initial", which computes to the initial value (`normal`).
        "table" => CssWhitespace::Normal,
        _ => CssWhitespace::Normal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Enforce this invariant to allow for binary search.
    #[test]
    fn html_all_tags_is_sorted() {
        if !HTML_ALL_TAGS.is_sorted() {
            let mut sorted = HTML_ALL_TAGS.to_vec();
            sorted.sort_unstable();
            panic!("All tags array is not sorted. Here it is sorted {sorted:?}");
        }
    }

    // Enforce this invariant to allow for binary search.
    #[test]
    fn global_attributes_is_sorted() {
        if !HTML_GLOBAL_ATTRIBUTES.is_sorted() {
            let mut sorted = HTML_GLOBAL_ATTRIBUTES.to_vec();
            sorted.sort_unstable();
            panic!("Global attributes array is not sorted. Here it is sorted {sorted:?}",);
        }
    }

    // Enforce this invariant to allow for binary search.
    #[test]
    fn tag_attributes_are_sorted() {
        HTML_ATTRIBUTES_BY_TAG.iter().for_each(|(tag, attributes)| {
            if !attributes.is_sorted() {
                let mut sorted = attributes.to_vec();
                sorted.sort_unstable();
                panic!(
                    "Attributes for tag '{tag}' are not sorted. Here they are sorted {sorted:?}",
                );
            }
        });
    }

    // Enforce this invariant to allow for binary search.
    #[test]
    fn svg_all_tags_is_sorted() {
        if !SVG_ALL_TAGS.is_sorted() {
            let mut sorted = SVG_ALL_TAGS.to_vec();
            sorted.sort_unstable();
            panic!("SVG tags array is not sorted. Here it is sorted {sorted:?}");
        }
    }

    // Enforce this invariant to allow for binary search.
    #[test]
    fn mathml_all_tags_is_sorted() {
        if !MATHML_ALL_TAGS.is_sorted() {
            let mut sorted = MATHML_ALL_TAGS.to_vec();
            sorted.sort_unstable();
            panic!("MathML tags array is not sorted. Here it is sorted {sorted:?}");
        }
    }

    #[test]
    fn test_is_canonical_html_tag_name_should_match_case_insensitive() {
        let cases = ["div", "DIV", "Div"];
        for case in cases {
            assert!(
                is_canonical_html_tag_name(case),
                "Did not recognize '{case}' as a canonical HTML tag name, but it should be."
            );
        }
    }

    #[test]
    fn test_get_css_whitespace_matches_prettier() {
        // From Prettier's computed `CSS_WHITE_SPACE_TAGS` mapping:
        // prettier/src/language-html/constants.evaluate.js
        assert_eq!(get_css_whitespace("pre"), CssWhitespace::Pre);
        assert_eq!(get_css_whitespace("listing"), CssWhitespace::Pre);
        assert_eq!(get_css_whitespace("plaintext"), CssWhitespace::Pre);
        assert_eq!(get_css_whitespace("xmp"), CssWhitespace::Pre);

        assert_eq!(get_css_whitespace("textarea"), CssWhitespace::PreWrap);
        assert_eq!(get_css_whitespace("nobr"), CssWhitespace::PreserveNoWrap);

        // Prettier returns the CSS value "initial" for table, which computes to `normal`.
        assert_eq!(get_css_whitespace("table"), CssWhitespace::Normal);

        // Default value
        assert_eq!(get_css_whitespace("div"), CssWhitespace::Normal);

        // Case-insensitive
        assert_eq!(get_css_whitespace("PRE"), CssWhitespace::Pre);
        assert_eq!(get_css_whitespace("NoBr"), CssWhitespace::PreserveNoWrap);
    }
}
