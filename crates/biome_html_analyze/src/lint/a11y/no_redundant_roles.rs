use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::{AstNode, BatchMutationExt, Text, TextRange, TokenText};
use biome_rule_options::no_redundant_roles::NoRedundantRolesOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce explicit `role` property is not the same as implicit/default role property on an element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <article role="article"></article>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <button role="button"></button>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <h1 role="heading">title</h1>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <article role="presentation"></article>
    /// ```
    ///
    /// ```html
    /// <div role="button"></div>
    /// ```
    ///
    /// ```html
    /// <span></span>
    /// ```
    ///
    pub NoRedundantRoles {
        version: "next",
        name: "noRedundantRoles",
        language: "html",
        sources: &[
            RuleSource::EslintJsxA11y("no-redundant-roles").same(),
            RuleSource::HtmlEslint("no-redundant-role").same(),
        ],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute_range: TextRange,
    role_value: Text,
    element_name: TokenText,
}

impl Rule for NoRedundantRoles {
    type Query = Ast<AnyHtmlElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoRedundantRolesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let element_name = node.name()?;

        // In non-HTML files (Vue, Svelte, Astro), PascalCase elements like
        // <Button> are components, not native HTML elements. Skip them.
        // In plain HTML, tags are case-insensitive and handled by
        // eq_ignore_ascii_case in the implicit role lookup.
        let file_source = ctx.source_type::<HtmlFileSource>();
        if !file_source.is_html()
            && element_name
                .text()
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_uppercase)
        {
            return None;
        }

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let role_trimmed = role_value.text().trim();

        let explicit_role = AriaRole::from_roles(role_trimmed)?;
        let implicit_role =
            get_implicit_role_for_element(element_name.text(), node)?;

        if explicit_role == implicit_role {
            return Some(RuleState {
                attribute_range: role_attribute.range(),
                role_value,
                element_name,
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let role = state.role_value.text().trim();
        let element = state.element_name.text();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.attribute_range,
            markup! {
                "Using the role attribute '"{role}"' on the '"{element}"' element is redundant, because it is implied by its semantic."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let role_attribute = node.find_attribute_by_name("role")?;
        let mut mutation = ctx.root().begin();
        mutation.remove_node(role_attribute);
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the implicit ARIA role for a given HTML element name.
///
/// Based on the WAI-ARIA spec: <https://www.w3.org/TR/html-aria/>
///
/// Uses case-insensitive matching since HTML tag names preserve their
/// original casing from the source.
fn get_implicit_role_for_element(element_name: &str, node: &AnyHtmlElement) -> Option<AriaRole> {
    // HTML tag names are case-insensitive, so match against lowercase.
    // Using eq_ignore_ascii_case via the match guard to avoid allocating.
    Some(
        if element_name.eq_ignore_ascii_case("article") {
            AriaRole::Article
        } else if element_name.eq_ignore_ascii_case("aside") {
            AriaRole::Complementary
        } else if element_name.eq_ignore_ascii_case("blockquote") {
            AriaRole::Blockquote
        } else if element_name.eq_ignore_ascii_case("button") {
            AriaRole::Button
        } else if element_name.eq_ignore_ascii_case("caption")
            || element_name.eq_ignore_ascii_case("figcaption")
            || element_name.eq_ignore_ascii_case("legend")
        {
            AriaRole::Caption
        } else if element_name.eq_ignore_ascii_case("code") {
            AriaRole::Code
        } else if element_name.eq_ignore_ascii_case("datalist") {
            AriaRole::Listbox
        } else if element_name.eq_ignore_ascii_case("del")
            || element_name.eq_ignore_ascii_case("s")
        {
            AriaRole::Deletion
        } else if element_name.eq_ignore_ascii_case("dd") {
            AriaRole::Definition
        } else if element_name.eq_ignore_ascii_case("dt")
            || element_name.eq_ignore_ascii_case("dfn")
        {
            AriaRole::Term
        } else if element_name.eq_ignore_ascii_case("mark") {
            AriaRole::Mark
        } else if element_name.eq_ignore_ascii_case("dialog") {
            AriaRole::Dialog
        } else if element_name.eq_ignore_ascii_case("em") {
            AriaRole::Emphasis
        } else if element_name.eq_ignore_ascii_case("figure") {
            AriaRole::Figure
        } else if element_name.eq_ignore_ascii_case("form") {
            AriaRole::Form
        } else if element_name.eq_ignore_ascii_case("hr") {
            AriaRole::Separator
        } else if element_name.eq_ignore_ascii_case("html") {
            AriaRole::Document
        } else if element_name.eq_ignore_ascii_case("ins") {
            AriaRole::Insertion
        } else if element_name.eq_ignore_ascii_case("main") {
            AriaRole::Main
        } else if element_name.eq_ignore_ascii_case("marquee") {
            AriaRole::Marquee
        } else if element_name.eq_ignore_ascii_case("math") {
            AriaRole::Math
        } else if element_name.eq_ignore_ascii_case("menu")
            || element_name.eq_ignore_ascii_case("ul")
            || element_name.eq_ignore_ascii_case("ol")
        {
            AriaRole::List
        } else if element_name.eq_ignore_ascii_case("meter") {
            AriaRole::Meter
        } else if element_name.eq_ignore_ascii_case("nav") {
            AriaRole::Navigation
        } else if element_name.eq_ignore_ascii_case("li") {
            AriaRole::Listitem
        } else if element_name.eq_ignore_ascii_case("option") {
            AriaRole::Option
        } else if element_name.eq_ignore_ascii_case("hgroup")
            || element_name.eq_ignore_ascii_case("optgroup")
            || element_name.eq_ignore_ascii_case("address")
            || element_name.eq_ignore_ascii_case("details")
            || element_name.eq_ignore_ascii_case("fieldset")
        {
            AriaRole::Group
        } else if element_name.eq_ignore_ascii_case("output") {
            AriaRole::Status
        } else if element_name.eq_ignore_ascii_case("p") {
            AriaRole::Paragraph
        } else if element_name.eq_ignore_ascii_case("progress") {
            AriaRole::Progressbar
        } else if element_name.eq_ignore_ascii_case("search") {
            AriaRole::Search
        } else if element_name.eq_ignore_ascii_case("strong") {
            AriaRole::Strong
        } else if element_name.eq_ignore_ascii_case("sub") {
            AriaRole::Subscript
        } else if element_name.eq_ignore_ascii_case("sup") {
            AriaRole::Superscript
        } else if element_name.eq_ignore_ascii_case("svg") {
            AriaRole::GraphicsDocument
        } else if element_name.eq_ignore_ascii_case("table") {
            AriaRole::Table
        } else if element_name.eq_ignore_ascii_case("textarea") {
            AriaRole::Textbox
        } else if element_name.eq_ignore_ascii_case("tr") {
            AriaRole::Row
        } else if element_name.eq_ignore_ascii_case("td") {
            AriaRole::Cell
        } else if element_name.eq_ignore_ascii_case("time") {
            AriaRole::Time
        } else if element_name.eq_ignore_ascii_case("h1")
            || element_name.eq_ignore_ascii_case("h2")
            || element_name.eq_ignore_ascii_case("h3")
            || element_name.eq_ignore_ascii_case("h4")
            || element_name.eq_ignore_ascii_case("h5")
            || element_name.eq_ignore_ascii_case("h6")
        {
            AriaRole::Heading
        } else if element_name.eq_ignore_ascii_case("tbody")
            || element_name.eq_ignore_ascii_case("tfoot")
            || element_name.eq_ignore_ascii_case("thead")
        {
            AriaRole::Rowgroup
        } else if element_name.eq_ignore_ascii_case("th") {
            let scope_value = get_attribute_value_lowercase(node, "scope");
            match scope_value.as_deref() {
                Some("col") => AriaRole::Columnheader,
                _ => AriaRole::Rowheader,
            }
        } else if element_name.eq_ignore_ascii_case("input") {
            let type_value = get_attribute_value_lowercase(node, "type");
            match type_value.as_deref() {
                Some("checkbox") => AriaRole::Checkbox,
                Some("number") => AriaRole::Spinbutton,
                Some("radio") => AriaRole::Radio,
                Some("range") => AriaRole::Slider,
                Some("button" | "image" | "reset" | "submit") => AriaRole::Button,
                Some("search") => {
                    if node.find_attribute_by_name("list").is_some() {
                        AriaRole::Combobox
                    } else {
                        AriaRole::Searchbox
                    }
                }
                Some(
                    "color" | "date" | "datetime-local" | "file" | "hidden" | "month"
                    | "password" | "time" | "week",
                ) => return None,
                _ => {
                    if node.find_attribute_by_name("list").is_some() {
                        AriaRole::Combobox
                    } else {
                        AriaRole::Textbox
                    }
                }
            }
        } else if element_name.eq_ignore_ascii_case("a")
            || element_name.eq_ignore_ascii_case("area")
            || element_name.eq_ignore_ascii_case("link")
        {
            if node.find_attribute_by_name("href").is_some() {
                AriaRole::Link
            } else {
                AriaRole::Generic
            }
        } else if element_name.eq_ignore_ascii_case("img") {
            let alt_value = get_attribute_value(node, "alt");
            match alt_value.as_deref() {
                Some(value) if !value.trim().is_empty() => AriaRole::Img,
                Some(_) => {
                    if node.find_attribute_by_name("aria-labelledby").is_some()
                        || node.find_attribute_by_name("aria-label").is_some()
                        || node.find_attribute_by_name("title").is_some()
                    {
                        AriaRole::Img
                    } else {
                        AriaRole::Presentation
                    }
                }
                None => AriaRole::Img,
            }
        } else if element_name.eq_ignore_ascii_case("section") {
            if node.find_attribute_by_name("aria-labelledby").is_some()
                || node.find_attribute_by_name("aria-label").is_some()
                || node.find_attribute_by_name("title").is_some()
            {
                AriaRole::Region
            } else {
                AriaRole::Generic
            }
        } else if element_name.eq_ignore_ascii_case("select") {
            let size = get_attribute_value(node, "size")
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(0);
            if node.find_attribute_by_name("multiple").is_none() && size <= 1 {
                AriaRole::Combobox
            } else {
                AriaRole::Listbox
            }
        } else if element_name.eq_ignore_ascii_case("b")
            || element_name.eq_ignore_ascii_case("bdi")
            || element_name.eq_ignore_ascii_case("bdo")
            || element_name.eq_ignore_ascii_case("body")
            || element_name.eq_ignore_ascii_case("data")
            || element_name.eq_ignore_ascii_case("div")
            || element_name.eq_ignore_ascii_case("i")
            || element_name.eq_ignore_ascii_case("q")
            || element_name.eq_ignore_ascii_case("samp")
            || element_name.eq_ignore_ascii_case("small")
            || element_name.eq_ignore_ascii_case("span")
            || element_name.eq_ignore_ascii_case("u")
            || element_name.eq_ignore_ascii_case("pre")
            || element_name.eq_ignore_ascii_case("header")
            || element_name.eq_ignore_ascii_case("footer")
        {
            AriaRole::Generic
        } else {
            return None;
        },
    )
}

fn get_attribute_value(node: &AnyHtmlElement, name: &str) -> Option<String> {
    let attr = node.find_attribute_by_name(name)?;
    let value = attr.initializer()?.value().ok()?.string_value()?;
    Some(value.text().to_string())
}

/// Like get_attribute_value but returns the value trimmed and lowercased.
/// HTML enumerated attributes (like input type, th scope) are ASCII
/// case-insensitive per spec.
fn get_attribute_value_lowercase(node: &AnyHtmlElement, name: &str) -> Option<String> {
    let attr = node.find_attribute_by_name(name)?;
    let value = attr.initializer()?.value().ok()?.string_value()?;
    Some(value.text().trim().to_ascii_lowercase())
}
