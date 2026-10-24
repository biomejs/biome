use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsSyntaxToken, JsxAttribute, JsxElement, T, jsx_ext::AnyJsxElement, static_value::StaticValue,
};
use biome_rowan::AstNode;
use biome_rule_options::no_obsolete_attributes::NoObsoleteAttributesOptions;

declare_lint_rule! {
    /// Disallow obsolete attributes on HTML elements.
    ///
    /// The HTML specification marks these attributes as obsolete, so they should
    /// no longer be used. Browsers might ignore them or handle them differently
    /// from each other. Most of them changed how a page looks, which is now done
    /// with CSS. Others have a modern replacement, such as the `id` attribute
    /// instead of `name` on `<a>` elements.
    ///
    /// Element and attribute names must match exactly, including uppercase and
    /// lowercase letters. The rule reports both the HTML spelling of an attribute,
    /// such as `cellpadding`, and its React spelling, such as `cellPadding`.
    /// Components such as `<Table />` and elements inside `<svg>` are ignored.
    ///
    /// Attributes with the value `false`, `null`, or `undefined` are also ignored,
    /// because they are not added to the page.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <table bgcolor="#ffffff"></table>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <table cellPadding="5"></table>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a name="top">Back to top</a>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe src="/page.html" frameBorder="0"></iframe>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <table style={{ backgroundColor: "#ffffff" }}></table>;
    /// <a id="top">Back to top</a>;
    /// <iframe src="/page.html" style={{ border: 0 }}></iframe>;
    /// <input name="email" />;
    /// <Table bgcolor="#ffffff" />;
    /// <a name={undefined}>Back to top</a>;
    /// ```
    ///
    /// ## Resources
    ///
    /// - [HTML specification: Non-conforming features](https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features)
    ///
    /// ## See Also
    ///
    /// - To disallow obsolete elements, such as `<font>` and `<center>`, see [`noObsoleteTags`](https://biomejs.dev/linter/rules/no-obsolete-tags/).
    ///
    pub NoObsoleteAttributes {
        version: "2.6.3",
        name: "noObsoleteAttributes",
        language: "jsx",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::HtmlEslint("no-obsolete-attrs").same()],
    }
}

impl Rule for NoObsoleteAttributes {
    type Query = Ast<JsxAttribute>;
    type State = ObsoleteAttribute;
    type Signals = Option<Self::State>;
    type Options = NoObsoleteAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let index = OBSOLETE_ATTRIBUTES
            .binary_search_by(|(obsolete, _)| (*obsolete).cmp(name.text_trimmed()))
            .ok()?;

        let element = AnyJsxElement::cast(attribute.syntax().grand_parent()?)?;
        // Components don't have a plain JSX name, so they are never matched.
        let element_name = element.name().ok()?.as_jsx_name()?.value_token().ok()?;

        let (_, usages) = OBSOLETE_ATTRIBUTES[index];
        let suggestion = usages
            .iter()
            .find(|(elements, _)| elements.contains(element_name.text_trimmed()))
            .map(|(_, suggestion)| *suggestion)?;

        if is_never_rendered(attribute) || is_inside_svg(&element) {
            return None;
        }

        Some(ObsoleteAttribute {
            element_name,
            suggestion,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let attribute = ctx.query();
        let name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                attribute.range(),
                markup! {
                    "The "<Emphasis>{name.text_trimmed()}</Emphasis>" attribute on "<Emphasis>"<"{state.element_name.text_trimmed()}">"</Emphasis>" is obsolete."
                },
            )
            .note(markup! {
                "Obsolete attributes should no longer be used. Browsers might ignore them or handle them differently from each other."
            })
            .note(markup! {
                {state.suggestion}
            }),
        )
    }
}

/// An obsolete attribute found on an HTML element.
pub struct ObsoleteAttribute {
    /// The tag name of the element that has the attribute.
    element_name: JsSyntaxToken,
    suggestion: Suggestion,
}

/// What to use instead of an obsolete attribute.
#[derive(Clone, Copy)]
enum Suggestion {
    UseCss,
    CharsetOnLink,
    CharsetOnScript,
    ImageMapOnLink,
    Methods,
    UseId,
    Rev,
    Urn,
    FormAccept,
    NoEffect,
    ContextMenu,
    MenuType,
    StyleType,
    Manifest,
    UseMap,
    LongDescription,
    LowQualitySource,
    MetaScheme,
    ObjectPlugin,
    ObjectDeclare,
    ObjectStandby,
    ObjectTypeMustMatch,
    ScriptLanguage,
    ScriptEvent,
    TableSummary,
    CellAbbreviation,
    CellAxis,
    CellScope,
    DataBinding,
    DropZone,
}

impl biome_console::fmt::Display for Suggestion {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::UseCss => fmt.write_markup(markup! { "Use CSS instead." }),
            Self::CharsetOnLink => fmt.write_markup(markup! {
                "Remove it. The server that hosts the linked file should send its character encoding in the "<Emphasis>"Content-Type"</Emphasis>" header instead."
            }),
            Self::CharsetOnScript => fmt.write_markup(markup! {
                "Remove it. Scripts use the same character encoding as the page."
            }),
            Self::ImageMapOnLink => fmt.write_markup(markup! {
                "Use an "<Emphasis>"<area>"</Emphasis>" element inside a "<Emphasis>"<map>"</Emphasis>" element to make clickable regions in an image."
            }),
            Self::Methods => fmt.write_markup(markup! {
                "Remove it. Send an HTTP "<Emphasis>"OPTIONS"</Emphasis>" request to find out which methods a URL supports."
            }),
            Self::UseId => fmt.write_markup(markup! {
                "Use the "<Emphasis>"id"</Emphasis>" attribute instead."
            }),
            Self::Rev => fmt.write_markup(markup! {
                "Use the "<Emphasis>"rel"</Emphasis>" attribute with the opposite meaning instead. For example, replace "<Emphasis>"rev=\"made\""</Emphasis>" with "<Emphasis>"rel=\"author\""</Emphasis>"."
            }),
            Self::Urn => fmt.write_markup(markup! {
                "Put the permanent address of the resource in the "<Emphasis>"href"</Emphasis>" attribute instead."
            }),
            Self::FormAccept => fmt.write_markup(markup! {
                "Add the "<Emphasis>"accept"</Emphasis>" attribute to each file "<Emphasis>"<input>"</Emphasis>" element in the form instead."
            }),
            Self::NoEffect => fmt.write_markup(markup! { "Remove it. It has no effect." }),
            Self::ContextMenu => fmt.write_markup(markup! {
                "Custom context menus are no longer supported. Handle the "<Emphasis>"contextmenu"</Emphasis>" event in a script instead."
            }),
            Self::MenuType => fmt.write_markup(markup! {
                "Remove it. For context menus, handle the "<Emphasis>"contextmenu"</Emphasis>" event in a script instead."
            }),
            Self::StyleType => fmt.write_markup(markup! {
                "Remove it. Styles are always CSS. To embed other data, use a "<Emphasis>"<script>"</Emphasis>" element instead."
            }),
            Self::Manifest => fmt.write_markup(markup! {
                "Use a service worker to make the page work offline instead."
            }),
            Self::UseMap => fmt.write_markup(markup! {
                "Use an "<Emphasis>"<img>"</Emphasis>" element to make clickable regions in an image instead."
            }),
            Self::LongDescription => fmt.write_markup(markup! {
                "Link to the description with a regular "<Emphasis>"<a>"</Emphasis>" element instead."
            }),
            Self::LowQualitySource => fmt.write_markup(markup! {
                "Use a single image that loads progressively, such as a progressive JPEG, in the "<Emphasis>"src"</Emphasis>" attribute instead."
            }),
            Self::MetaScheme => fmt.write_markup(markup! {
                "Include the scheme in the value of the "<Emphasis>"content"</Emphasis>" attribute instead."
            }),
            Self::ObjectPlugin => fmt.write_markup(markup! {
                "Use the "<Emphasis>"data"</Emphasis>" and "<Emphasis>"type"</Emphasis>" attributes instead."
            }),
            Self::ObjectDeclare => fmt.write_markup(markup! {
                "Repeat the whole "<Emphasis>"<object>"</Emphasis>" element each time you want to reuse the resource."
            }),
            Self::ObjectStandby => fmt.write_markup(markup! {
                "Remove it, and make the resource load quickly or in smaller parts instead."
            }),
            Self::ObjectTypeMustMatch => fmt.write_markup(markup! {
                "Remove it, and avoid using "<Emphasis>"<object>"</Emphasis>" elements to embed resources you don't trust."
            }),
            Self::ScriptLanguage => fmt.write_markup(markup! {
                "Remove it for JavaScript. To embed other data, use the "<Emphasis>"type"</Emphasis>" attribute instead."
            }),
            Self::ScriptEvent => fmt.write_markup(markup! {
                "Register event listeners with "<Emphasis>"addEventListener()"</Emphasis>" in the script instead."
            }),
            Self::TableSummary => fmt.write_markup(markup! {
                "Describe the table with a "<Emphasis>"<caption>"</Emphasis>" element or with text next to the table instead."
            }),
            Self::CellAbbreviation => fmt.write_markup(markup! {
                "Start the cell text with a short description and put the details after it. For header cells, use a "<Emphasis>"<th>"</Emphasis>" element, which supports the "<Emphasis>"abbr"</Emphasis>" attribute."
            }),
            Self::CellAxis => fmt.write_markup(markup! {
                "Use the "<Emphasis>"scope"</Emphasis>" attribute on the related "<Emphasis>"<th>"</Emphasis>" element instead."
            }),
            Self::CellScope => fmt.write_markup(markup! {
                "Use a "<Emphasis>"<th>"</Emphasis>" element for header cells instead."
            }),
            Self::DataBinding => fmt.write_markup(markup! {
                "Load the data with a script instead, for example with "<Emphasis>"fetch()"</Emphasis>"."
            }),
            Self::DropZone => fmt.write_markup(markup! {
                "Handle the "<Emphasis>"dragenter"</Emphasis>" and "<Emphasis>"dragover"</Emphasis>" events in a script instead."
            }),
        }
    }
}

/// The elements that an obsolete attribute applies to.
#[derive(Clone, Copy)]
enum Elements {
    All,
    /// Lowercase tag names, sorted for binary search.
    Only(&'static [&'static str]),
}

impl Elements {
    fn contains(self, name: &str) -> bool {
        match self {
            Self::All => true,
            Self::Only(names) => names.binary_search(&name).is_ok(),
        }
    }
}

/// Returns `true` if the value of `attribute` is `false`, `null`, or `undefined`,
/// which means the attribute is not added to the page.
fn is_never_rendered(attribute: &JsxAttribute) -> bool {
    attribute
        .as_static_value()
        .is_some_and(|value| match value {
            StaticValue::Boolean(token) => token.kind() == T![false],
            value => value.is_null_or_undefined(),
        })
}

/// Returns `true` if `element` is part of an inline SVG image, where attributes
/// such as `type` on `<style>` are still valid.
///
/// `<foreignObject>` puts HTML content back inside an SVG image, so the closest
/// of these two elements decides whether an element is an SVG element.
fn is_inside_svg(element: &AnyJsxElement) -> bool {
    // An opening element is wrapped in the node of the whole element, which is
    // not an ancestor of the element either.
    let own_nodes = match element {
        AnyJsxElement::JsxOpeningElement(_) => 2,
        AnyJsxElement::JsxSelfClosingElement(_) => 1,
    };
    element
        .syntax()
        .ancestors()
        .skip(own_nodes)
        .filter_map(JsxElement::cast)
        .filter_map(|ancestor| {
            let name = ancestor.opening_element().ok()?.name().ok()?;
            name.as_jsx_name()?.value_token().ok()
        })
        .find_map(|name| match name.text_trimmed() {
            "svg" => Some(true),
            "foreignObject" => Some(false),
            _ => None,
        })
        .unwrap_or(false)
}

const ALIGNED_ELEMENTS: &[&str] = &[
    "caption", "col", "div", "embed", "h1", "h2", "h3", "h4", "h5", "h6", "hr", "iframe", "img",
    "input", "legend", "object", "p", "table", "tbody", "td", "tfoot", "th", "thead", "tr",
];

const TABLE_SECTIONS_AND_CELLS: &[&str] = &["col", "tbody", "td", "tfoot", "th", "thead", "tr"];

const EMBEDDED_ELEMENTS: &[&str] = &["embed", "iframe", "img", "input", "object"];

const DATAFLD_ELEMENTS: &[&str] = &[
    "a", "button", "div", "fieldset", "frame", "iframe", "img", "input", "label", "legend",
    "marquee", "object", "select", "span", "textarea",
];

const DATAFORMATAS_ELEMENTS: &[&str] = &[
    "button", "div", "input", "label", "legend", "marquee", "object", "option", "select", "span",
    "table",
];

const DATASRC_ELEMENTS: &[&str] = &[
    "a", "button", "div", "frame", "iframe", "img", "input", "label", "legend", "marquee",
    "object", "option", "select", "span", "table", "textarea",
];

const CHARSET: &[(Elements, Suggestion)] = &[
    (Elements::Only(&["a", "link"]), Suggestion::CharsetOnLink),
    (Elements::Only(&["script"]), Suggestion::CharsetOnScript),
];

/// Obsolete attribute names, sorted by name for binary search. Each name maps to
/// the elements where it is obsolete and what to use instead.
///
/// Names are matched exactly, so attributes that React spells differently from
/// HTML, such as `cellPadding`, are listed under both spellings.
const OBSOLETE_ATTRIBUTES: &[(&str, &[(Elements, Suggestion)])] = &[
    (
        "abbr",
        &[(Elements::Only(&["td"]), Suggestion::CellAbbreviation)],
    ),
    (
        "accept",
        &[(Elements::Only(&["form"]), Suggestion::FormAccept)],
    ),
    (
        "align",
        &[(Elements::Only(ALIGNED_ELEMENTS), Suggestion::UseCss)],
    ),
    ("alink", &[(Elements::Only(&["body"]), Suggestion::UseCss)]),
    (
        "allowTransparency",
        &[(Elements::Only(&["iframe"]), Suggestion::UseCss)],
    ),
    (
        "allowtransparency",
        &[(Elements::Only(&["iframe"]), Suggestion::UseCss)],
    ),
    (
        "archive",
        &[(Elements::Only(&["object"]), Suggestion::ObjectPlugin)],
    ),
    (
        "axis",
        &[(Elements::Only(&["td", "th"]), Suggestion::CellAxis)],
    ),
    (
        "background",
        &[(
            Elements::Only(&["body", "table", "tbody", "td", "tfoot", "th", "thead", "tr"]),
            Suggestion::UseCss,
        )],
    ),
    (
        "bgcolor",
        &[(
            Elements::Only(&["body", "table", "td", "th", "tr"]),
            Suggestion::UseCss,
        )],
    ),
    (
        "border",
        &[(
            Elements::Only(&["img", "input", "object", "table"]),
            Suggestion::UseCss,
        )],
    ),
    (
        "bordercolor",
        &[(Elements::Only(&["table"]), Suggestion::UseCss)],
    ),
    (
        "bottommargin",
        &[(Elements::Only(&["body"]), Suggestion::UseCss)],
    ),
    (
        "cellPadding",
        &[(Elements::Only(&["table"]), Suggestion::UseCss)],
    ),
    (
        "cellSpacing",
        &[(Elements::Only(&["table"]), Suggestion::UseCss)],
    ),
    (
        "cellpadding",
        &[(Elements::Only(&["table"]), Suggestion::UseCss)],
    ),
    (
        "cellspacing",
        &[(Elements::Only(&["table"]), Suggestion::UseCss)],
    ),
    (
        "char",
        &[(Elements::Only(TABLE_SECTIONS_AND_CELLS), Suggestion::UseCss)],
    ),
    ("charSet", CHARSET),
    (
        "charoff",
        &[(Elements::Only(TABLE_SECTIONS_AND_CELLS), Suggestion::UseCss)],
    ),
    ("charset", CHARSET),
    (
        "classID",
        &[(Elements::Only(&["object"]), Suggestion::ObjectPlugin)],
    ),
    (
        "classid",
        &[(Elements::Only(&["object"]), Suggestion::ObjectPlugin)],
    ),
    ("clear", &[(Elements::Only(&["br"]), Suggestion::UseCss)]),
    (
        "code",
        &[(Elements::Only(&["object"]), Suggestion::ObjectPlugin)],
    ),
    (
        "codebase",
        &[(Elements::Only(&["object"]), Suggestion::ObjectPlugin)],
    ),
    (
        "codetype",
        &[(Elements::Only(&["object"]), Suggestion::ObjectPlugin)],
    ),
    ("color", &[(Elements::Only(&["hr"]), Suggestion::UseCss)]),
    (
        "compact",
        &[(
            Elements::Only(&["dl", "menu", "ol", "ul"]),
            Suggestion::UseCss,
        )],
    ),
    ("contextMenu", &[(Elements::All, Suggestion::ContextMenu)]),
    ("contextmenu", &[(Elements::All, Suggestion::ContextMenu)]),
    (
        "coords",
        &[(Elements::Only(&["a"]), Suggestion::ImageMapOnLink)],
    ),
    (
        "datafld",
        &[(Elements::Only(DATAFLD_ELEMENTS), Suggestion::DataBinding)],
    ),
    (
        "dataformatas",
        &[(
            Elements::Only(DATAFORMATAS_ELEMENTS),
            Suggestion::DataBinding,
        )],
    ),
    (
        "datapagesize",
        &[(Elements::Only(&["table"]), Suggestion::NoEffect)],
    ),
    (
        "datasrc",
        &[(Elements::Only(DATASRC_ELEMENTS), Suggestion::DataBinding)],
    ),
    (
        "declare",
        &[(Elements::Only(&["object"]), Suggestion::ObjectDeclare)],
    ),
    ("dropzone", &[(Elements::All, Suggestion::DropZone)]),
    (
        "event",
        &[(Elements::Only(&["script"]), Suggestion::ScriptEvent)],
    ),
    (
        "for",
        &[(Elements::Only(&["script"]), Suggestion::ScriptEvent)],
    ),
    ("frame", &[(Elements::Only(&["table"]), Suggestion::UseCss)]),
    (
        "frameBorder",
        &[(Elements::Only(&["iframe"]), Suggestion::UseCss)],
    ),
    (
        "frameborder",
        &[(Elements::Only(&["iframe"]), Suggestion::UseCss)],
    ),
    (
        "framespacing",
        &[(Elements::Only(&["iframe"]), Suggestion::UseCss)],
    ),
    (
        "height",
        &[(
            Elements::Only(&["table", "tbody", "td", "tfoot", "th", "thead", "tr"]),
            Suggestion::UseCss,
        )],
    ),
    (
        "hrefLang",
        &[(Elements::Only(&["area"]), Suggestion::NoEffect)],
    ),
    (
        "hreflang",
        &[(Elements::Only(&["area"]), Suggestion::NoEffect)],
    ),
    (
        "hspace",
        &[(Elements::Only(EMBEDDED_ELEMENTS), Suggestion::UseCss)],
    ),
    (
        "htmlFor",
        &[(Elements::Only(&["script"]), Suggestion::ScriptEvent)],
    ),
    (
        "ismap",
        &[(Elements::Only(&["input"]), Suggestion::NoEffect)],
    ),
    (
        "label",
        &[(Elements::Only(&["menu"]), Suggestion::ContextMenu)],
    ),
    (
        "language",
        &[(Elements::Only(&["script"]), Suggestion::ScriptLanguage)],
    ),
    (
        "leftmargin",
        &[(Elements::Only(&["body"]), Suggestion::UseCss)],
    ),
    ("link", &[(Elements::Only(&["body"]), Suggestion::UseCss)]),
    (
        "longdesc",
        &[(
            Elements::Only(&["iframe", "img"]),
            Suggestion::LongDescription,
        )],
    ),
    (
        "lowsrc",
        &[(Elements::Only(&["img"]), Suggestion::LowQualitySource)],
    ),
    (
        "manifest",
        &[(Elements::Only(&["html"]), Suggestion::Manifest)],
    ),
    (
        "marginHeight",
        &[(Elements::Only(&["body", "iframe"]), Suggestion::UseCss)],
    ),
    (
        "marginWidth",
        &[(Elements::Only(&["body", "iframe"]), Suggestion::UseCss)],
    ),
    (
        "marginheight",
        &[(Elements::Only(&["body", "iframe"]), Suggestion::UseCss)],
    ),
    (
        "marginwidth",
        &[(Elements::Only(&["body", "iframe"]), Suggestion::UseCss)],
    ),
    (
        "methods",
        &[(Elements::Only(&["a", "link"]), Suggestion::Methods)],
    ),
    (
        "name",
        &[(
            Elements::Only(&["a", "embed", "img", "option"]),
            Suggestion::UseId,
        )],
    ),
    (
        "nohref",
        &[(Elements::Only(&["area"]), Suggestion::NoEffect)],
    ),
    ("noshade", &[(Elements::Only(&["hr"]), Suggestion::UseCss)]),
    (
        "nowrap",
        &[(Elements::Only(&["td", "th"]), Suggestion::UseCss)],
    ),
    ("onshow", &[(Elements::All, Suggestion::ContextMenu)]),
    (
        "profile",
        &[(Elements::Only(&["head"]), Suggestion::NoEffect)],
    ),
    ("rev", &[(Elements::Only(&["a", "link"]), Suggestion::Rev)]),
    (
        "rightmargin",
        &[(Elements::Only(&["body"]), Suggestion::UseCss)],
    ),
    ("rules", &[(Elements::Only(&["table"]), Suggestion::UseCss)]),
    (
        "scheme",
        &[(Elements::Only(&["meta"]), Suggestion::MetaScheme)],
    ),
    ("scope", &[(Elements::Only(&["td"]), Suggestion::CellScope)]),
    (
        "scrolling",
        &[(Elements::Only(&["iframe"]), Suggestion::UseCss)],
    ),
    (
        "shape",
        &[(Elements::Only(&["a"]), Suggestion::ImageMapOnLink)],
    ),
    ("size", &[(Elements::Only(&["hr"]), Suggestion::UseCss)]),
    (
        "standby",
        &[(Elements::Only(&["object"]), Suggestion::ObjectStandby)],
    ),
    (
        "summary",
        &[(Elements::Only(&["table"]), Suggestion::TableSummary)],
    ),
    (
        "target",
        &[(Elements::Only(&["link"]), Suggestion::NoEffect)],
    ),
    ("text", &[(Elements::Only(&["body"]), Suggestion::UseCss)]),
    (
        "topmargin",
        &[(Elements::Only(&["body"]), Suggestion::UseCss)],
    ),
    (
        "type",
        &[
            (Elements::Only(&["area"]), Suggestion::NoEffect),
            (Elements::Only(&["li", "ul"]), Suggestion::UseCss),
            (Elements::Only(&["menu"]), Suggestion::MenuType),
            (Elements::Only(&["style"]), Suggestion::StyleType),
        ],
    ),
    (
        "typemustmatch",
        &[(Elements::Only(&["object"]), Suggestion::ObjectTypeMustMatch)],
    ),
    ("urn", &[(Elements::Only(&["a", "link"]), Suggestion::Urn)]),
    (
        "useMap",
        &[(Elements::Only(&["input", "object"]), Suggestion::UseMap)],
    ),
    (
        "usemap",
        &[(Elements::Only(&["input", "object"]), Suggestion::UseMap)],
    ),
    (
        "valign",
        &[(Elements::Only(TABLE_SECTIONS_AND_CELLS), Suggestion::UseCss)],
    ),
    (
        "version",
        &[(Elements::Only(&["html"]), Suggestion::NoEffect)],
    ),
    ("vlink", &[(Elements::Only(&["body"]), Suggestion::UseCss)]),
    (
        "vspace",
        &[(Elements::Only(EMBEDDED_ELEMENTS), Suggestion::UseCss)],
    ),
    (
        "width",
        &[(
            Elements::Only(&["col", "hr", "pre", "table", "td", "th"]),
            Suggestion::UseCss,
        )],
    ),
];

#[cfg(test)]
mod tests {
    use super::{Elements, OBSOLETE_ATTRIBUTES};

    #[test]
    fn obsolete_attributes_are_sorted() {
        for pair in OBSOLETE_ATTRIBUTES.windows(2) {
            let (left, _) = pair[0];
            let (right, _) = pair[1];
            assert!(left < right, "`{left}` must come before `{right}`");
        }
    }

    #[test]
    fn obsolete_attribute_elements_are_sorted() {
        for (attribute, usages) in OBSOLETE_ATTRIBUTES {
            for (elements, _) in *usages {
                let Elements::Only(names) = elements else {
                    continue;
                };
                for pair in names.windows(2) {
                    assert!(
                        pair[0] < pair[1],
                        "`{}` must come before `{}` in the elements of `{attribute}`",
                        pair[0],
                        pair[1]
                    );
                }
            }
        }
    }
}
