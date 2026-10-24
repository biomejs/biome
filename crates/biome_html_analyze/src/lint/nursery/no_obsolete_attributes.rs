use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlElement, HtmlAttribute, HtmlSyntaxKind, HtmlSyntaxToken, T,
    element_ext::AnyHtmlTagElement,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::AstNode;
use biome_rule_options::no_obsolete_attributes::NoObsoleteAttributesOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow obsolete attributes on HTML elements.
    ///
    /// The HTML specification marks these attributes as obsolete, so they should
    /// no longer be used. Browsers might ignore them or handle them differently
    /// from each other. Most of them changed how a page looks, which is now done
    /// with CSS. Others have a modern replacement, such as the `id` attribute
    /// instead of `name` on `<a>` elements.
    ///
    /// Attribute names are case-insensitive. Elements inside `<svg>` and custom
    /// components in template languages, such as `<Table />`, are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <table bgcolor="#ffffff"></table>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div align="center">Content</div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a name="top">Back to top</a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <iframe src="/page.html" frameborder="0"></iframe>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <table style="background-color: #ffffff"></table>
    /// <div style="text-align: center">Content</div>
    /// <a id="top">Back to top</a>
    /// <iframe src="/page.html" style="border: 0"></iframe>
    /// <input name="email" />
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
        language: "html",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::HtmlEslint("no-obsolete-attrs").same()],
    }
}

impl Rule for NoObsoleteAttributes {
    type Query = Ast<HtmlAttribute>;
    type State = ObsoleteAttribute;
    type Signals = Option<Self::State>;
    type Options = NoObsoleteAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?.value_token().ok()?;
        let index = OBSOLETE_ATTRIBUTES
            .binary_search_by(|(obsolete, _)| obsolete.cmp_ignore_ascii_case(name.text_trimmed()))
            .ok()?;

        let element = AnyHtmlTagElement::cast(attribute.syntax().grand_parent()?)?;
        // Components have no tag name, so they are never matched.
        let element_name = element.name_value_token()?;
        let element_kind = element.tag_name_kind()?;

        let (_, usages) = OBSOLETE_ATTRIBUTES[index];
        let suggestion = usages
            .iter()
            .find(|(elements, _)| elements.contains(element_kind))
            .map(|(_, suggestion)| *suggestion)?;

        if is_inside_svg(&element) {
            return None;
        }

        Some(ObsoleteAttribute {
            element_name,
            suggestion,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let attribute = ctx.query();
        let name = attribute.name().ok()?.value_token().ok()?;
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
    element_name: HtmlSyntaxToken,
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
                "Use an "<Emphasis>"<img>"</Emphasis>" element with a "<Emphasis>"usemap"</Emphasis>" attribute to make clickable regions in an image."
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
    Only(TokenSet<HtmlSyntaxKind>),
}

impl Elements {
    fn contains(self, kind: HtmlSyntaxKind) -> bool {
        match self {
            Self::All => true,
            Self::Only(kinds) => kinds.contains(kind),
        }
    }
}

/// Returns `true` if `element` is part of an inline SVG image, where attributes
/// such as `type` on `<style>` are still valid.
fn is_inside_svg(element: &AnyHtmlTagElement) -> bool {
    // An opening element is wrapped in the node of the whole element, which is
    // not an ancestor of the element either.
    let own_nodes = match element {
        AnyHtmlTagElement::HtmlOpeningElement(_) => 2,
        AnyHtmlTagElement::HtmlSelfClosingElement(_) => 1,
    };
    element
        .syntax()
        .ancestors()
        .skip(own_nodes)
        .filter_map(AnyHtmlElement::cast)
        .filter_map(|ancestor| ancestor.tag_name_kind())
        .find(|kind| SVG_BOUNDARIES.contains(*kind))
        == Some(T![svg])
}

/// `<foreignObject>` puts HTML content back inside an SVG image, so the closest
/// of these two elements decides whether an element is an SVG element.
const SVG_BOUNDARIES: TokenSet<HtmlSyntaxKind> = token_set![T![svg], T![foreignObject]];

const TABLE_SECTIONS_AND_CELLS: TokenSet<HtmlSyntaxKind> = token_set![
    T![col],
    T![tbody],
    T![thead],
    T![tfoot],
    T![td],
    T![th],
    T![tr]
];

const REPLACED_ELEMENTS: TokenSet<HtmlSyntaxKind> =
    token_set![T![embed], T![iframe], T![input], T![img], T![object]];

/// Obsolete attribute names, sorted by name for binary search. Each name maps to
/// the elements where it is obsolete and what to use instead.
const OBSOLETE_ATTRIBUTES: &[(&str, &[(Elements, Suggestion)])] = &[
    (
        "abbr",
        &[(
            Elements::Only(token_set![T![td]]),
            Suggestion::CellAbbreviation,
        )],
    ),
    (
        "accept",
        &[(Elements::Only(token_set![T![form]]), Suggestion::FormAccept)],
    ),
    (
        "align",
        &[(
            Elements::Only(token_set![
                T![caption],
                T![col],
                T![div],
                T![embed],
                T![h1],
                T![h2],
                T![h3],
                T![h4],
                T![h5],
                T![h6],
                T![hr],
                T![iframe],
                T![input],
                T![img],
                T![legend],
                T![object],
                T![p],
                T![table],
                T![tbody],
                T![thead],
                T![tfoot],
                T![td],
                T![th],
                T![tr]
            ]),
            Suggestion::UseCss,
        )],
    ),
    (
        "alink",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "allowtransparency",
        &[(Elements::Only(token_set![T![iframe]]), Suggestion::UseCss)],
    ),
    (
        "archive",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectPlugin,
        )],
    ),
    (
        "axis",
        &[(
            Elements::Only(token_set![T![td], T![th]]),
            Suggestion::CellAxis,
        )],
    ),
    (
        "background",
        &[(
            Elements::Only(token_set![
                T![body],
                T![table],
                T![thead],
                T![tbody],
                T![tfoot],
                T![tr],
                T![td],
                T![th]
            ]),
            Suggestion::UseCss,
        )],
    ),
    (
        "bgcolor",
        &[(
            Elements::Only(token_set![T![body], T![table], T![td], T![th], T![tr]]),
            Suggestion::UseCss,
        )],
    ),
    (
        "border",
        &[(
            Elements::Only(token_set![T![input], T![img], T![object], T![table]]),
            Suggestion::UseCss,
        )],
    ),
    (
        "bordercolor",
        &[(Elements::Only(token_set![T![table]]), Suggestion::UseCss)],
    ),
    (
        "bottommargin",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "cellpadding",
        &[(Elements::Only(token_set![T![table]]), Suggestion::UseCss)],
    ),
    (
        "cellspacing",
        &[(Elements::Only(token_set![T![table]]), Suggestion::UseCss)],
    ),
    (
        "char",
        &[(Elements::Only(TABLE_SECTIONS_AND_CELLS), Suggestion::UseCss)],
    ),
    (
        "charoff",
        &[(Elements::Only(TABLE_SECTIONS_AND_CELLS), Suggestion::UseCss)],
    ),
    (
        "charset",
        &[
            (
                Elements::Only(token_set![T![a], T![link]]),
                Suggestion::CharsetOnLink,
            ),
            (
                Elements::Only(token_set![T![script]]),
                Suggestion::CharsetOnScript,
            ),
        ],
    ),
    (
        "classid",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectPlugin,
        )],
    ),
    (
        "clear",
        &[(Elements::Only(token_set![T![br]]), Suggestion::UseCss)],
    ),
    (
        "code",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectPlugin,
        )],
    ),
    (
        "codebase",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectPlugin,
        )],
    ),
    (
        "codetype",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectPlugin,
        )],
    ),
    (
        "color",
        &[(Elements::Only(token_set![T![hr]]), Suggestion::UseCss)],
    ),
    (
        "compact",
        &[(
            Elements::Only(token_set![T![dl], T![menu], T![ol], T![ul]]),
            Suggestion::UseCss,
        )],
    ),
    ("contextmenu", &[(Elements::All, Suggestion::ContextMenu)]),
    (
        "coords",
        &[(
            Elements::Only(token_set![T![a]]),
            Suggestion::ImageMapOnLink,
        )],
    ),
    (
        "datafld",
        &[(
            Elements::Only(token_set![
                T![a],
                T![button],
                T![div],
                T![fieldset],
                T![frame],
                T![iframe],
                T![img],
                T![input],
                T![label],
                T![legend],
                T![marquee],
                T![object],
                T![select],
                T![span],
                T![textarea]
            ]),
            Suggestion::DataBinding,
        )],
    ),
    (
        "dataformatas",
        &[(
            Elements::Only(token_set![
                T![button],
                T![div],
                T![input],
                T![label],
                T![legend],
                T![marquee],
                T![object],
                T![option],
                T![select],
                T![span],
                T![table]
            ]),
            Suggestion::DataBinding,
        )],
    ),
    (
        "datapagesize",
        &[(Elements::Only(token_set![T![table]]), Suggestion::NoEffect)],
    ),
    (
        "datasrc",
        &[(
            Elements::Only(token_set![
                T![a],
                T![button],
                T![div],
                T![frame],
                T![iframe],
                T![img],
                T![input],
                T![label],
                T![legend],
                T![marquee],
                T![object],
                T![option],
                T![select],
                T![span],
                T![table],
                T![textarea]
            ]),
            Suggestion::DataBinding,
        )],
    ),
    (
        "declare",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectDeclare,
        )],
    ),
    ("dropzone", &[(Elements::All, Suggestion::DropZone)]),
    (
        "event",
        &[(
            Elements::Only(token_set![T![script]]),
            Suggestion::ScriptEvent,
        )],
    ),
    (
        "for",
        &[(
            Elements::Only(token_set![T![script]]),
            Suggestion::ScriptEvent,
        )],
    ),
    (
        "frame",
        &[(Elements::Only(token_set![T![table]]), Suggestion::UseCss)],
    ),
    (
        "frameborder",
        &[(Elements::Only(token_set![T![iframe]]), Suggestion::UseCss)],
    ),
    (
        "framespacing",
        &[(Elements::Only(token_set![T![iframe]]), Suggestion::UseCss)],
    ),
    (
        "height",
        &[(
            Elements::Only(token_set![
                T![table],
                T![thead],
                T![tbody],
                T![tfoot],
                T![td],
                T![th],
                T![tr]
            ]),
            Suggestion::UseCss,
        )],
    ),
    (
        "hreflang",
        &[(Elements::Only(token_set![T![area]]), Suggestion::NoEffect)],
    ),
    (
        "hspace",
        &[(Elements::Only(REPLACED_ELEMENTS), Suggestion::UseCss)],
    ),
    (
        "ismap",
        &[(Elements::Only(token_set![T![input]]), Suggestion::NoEffect)],
    ),
    (
        "label",
        &[(
            Elements::Only(token_set![T![menu]]),
            Suggestion::ContextMenu,
        )],
    ),
    (
        "language",
        &[(
            Elements::Only(token_set![T![script]]),
            Suggestion::ScriptLanguage,
        )],
    ),
    (
        "leftmargin",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "link",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "longdesc",
        &[(
            Elements::Only(token_set![T![iframe], T![img]]),
            Suggestion::LongDescription,
        )],
    ),
    (
        "lowsrc",
        &[(
            Elements::Only(token_set![T![img]]),
            Suggestion::LowQualitySource,
        )],
    ),
    (
        "manifest",
        &[(Elements::Only(token_set![T![html]]), Suggestion::Manifest)],
    ),
    (
        "marginheight",
        &[(
            Elements::Only(token_set![T![body], T![iframe]]),
            Suggestion::UseCss,
        )],
    ),
    (
        "marginwidth",
        &[(
            Elements::Only(token_set![T![body], T![iframe]]),
            Suggestion::UseCss,
        )],
    ),
    (
        "methods",
        &[(
            Elements::Only(token_set![T![a], T![link]]),
            Suggestion::Methods,
        )],
    ),
    (
        "name",
        &[(
            Elements::Only(token_set![T![a], T![embed], T![img], T![option]]),
            Suggestion::UseId,
        )],
    ),
    (
        "nohref",
        &[(Elements::Only(token_set![T![area]]), Suggestion::NoEffect)],
    ),
    (
        "noshade",
        &[(Elements::Only(token_set![T![hr]]), Suggestion::UseCss)],
    ),
    (
        "nowrap",
        &[(
            Elements::Only(token_set![T![td], T![th]]),
            Suggestion::UseCss,
        )],
    ),
    ("onshow", &[(Elements::All, Suggestion::ContextMenu)]),
    (
        "profile",
        &[(Elements::Only(token_set![T![head]]), Suggestion::NoEffect)],
    ),
    (
        "rev",
        &[(Elements::Only(token_set![T![a], T![link]]), Suggestion::Rev)],
    ),
    (
        "rightmargin",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "rules",
        &[(Elements::Only(token_set![T![table]]), Suggestion::UseCss)],
    ),
    (
        "scheme",
        &[(Elements::Only(token_set![T![meta]]), Suggestion::MetaScheme)],
    ),
    (
        "scope",
        &[(Elements::Only(token_set![T![td]]), Suggestion::CellScope)],
    ),
    (
        "scrolling",
        &[(Elements::Only(token_set![T![iframe]]), Suggestion::UseCss)],
    ),
    (
        "shape",
        &[(
            Elements::Only(token_set![T![a]]),
            Suggestion::ImageMapOnLink,
        )],
    ),
    (
        "size",
        &[(Elements::Only(token_set![T![hr]]), Suggestion::UseCss)],
    ),
    (
        "standby",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectStandby,
        )],
    ),
    (
        "summary",
        &[(
            Elements::Only(token_set![T![table]]),
            Suggestion::TableSummary,
        )],
    ),
    (
        "target",
        &[(Elements::Only(token_set![T![link]]), Suggestion::NoEffect)],
    ),
    (
        "text",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "topmargin",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "type",
        &[
            (Elements::Only(token_set![T![area]]), Suggestion::NoEffect),
            (
                Elements::Only(token_set![T![li], T![ul]]),
                Suggestion::UseCss,
            ),
            (Elements::Only(token_set![T![menu]]), Suggestion::MenuType),
            (Elements::Only(token_set![T![style]]), Suggestion::StyleType),
        ],
    ),
    (
        "typemustmatch",
        &[(
            Elements::Only(token_set![T![object]]),
            Suggestion::ObjectTypeMustMatch,
        )],
    ),
    (
        "urn",
        &[(Elements::Only(token_set![T![a], T![link]]), Suggestion::Urn)],
    ),
    (
        "usemap",
        &[(
            Elements::Only(token_set![T![input], T![object]]),
            Suggestion::UseMap,
        )],
    ),
    (
        "valign",
        &[(Elements::Only(TABLE_SECTIONS_AND_CELLS), Suggestion::UseCss)],
    ),
    (
        "version",
        &[(Elements::Only(token_set![T![html]]), Suggestion::NoEffect)],
    ),
    (
        "vlink",
        &[(Elements::Only(token_set![T![body]]), Suggestion::UseCss)],
    ),
    (
        "vspace",
        &[(Elements::Only(REPLACED_ELEMENTS), Suggestion::UseCss)],
    ),
    (
        "width",
        &[(
            Elements::Only(token_set![
                T![col],
                T![hr],
                T![pre],
                T![table],
                T![td],
                T![th]
            ]),
            Suggestion::UseCss,
        )],
    ),
];

#[cfg(test)]
mod tests {
    use super::OBSOLETE_ATTRIBUTES;
    use biome_string_case::StrLikeExtension;
    use std::cmp::Ordering;

    #[test]
    fn obsolete_attributes_are_sorted() {
        for pair in OBSOLETE_ATTRIBUTES.windows(2) {
            let (left, _) = pair[0];
            let (right, _) = pair[1];
            assert_eq!(
                left.cmp_ignore_ascii_case(right),
                Ordering::Less,
                "`{left}` must come before `{right}`"
            );
        }
    }
}
