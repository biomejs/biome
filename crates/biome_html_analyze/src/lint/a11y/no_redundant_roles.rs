use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, Text, TextRange, TokenText};
use biome_rule_options::no_redundant_roles::NoRedundantRolesOptions;
use biome_string_case::StrLikeExtension;

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

        // Fast path: elements with no attributes can't have a role attribute.
        if node.attributes().is_none_or(|a| a.is_empty()) {
            return None;
        }

        let element_name = node.name()?;

        // In non-HTML files (Vue, Svelte, Astro), PascalCase elements like
        // <Button> are components, not native HTML elements. Skip them.
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

        // Bail early if this element can't possibly have an implicit role.
        // This avoids a DOM traversal to find the "role" attribute on elements
        // like <head>, <meta>, <script>, <br>, etc.
        //
        // Try the original casing first (almost always lowercase in practice)
        // to avoid the cost of lowercasing for the common case.
        let name_text = element_name.text();
        let name_lower;
        let name_str = if has_implicit_role(name_text) {
            name_text
        } else {
            name_lower = name_text.to_ascii_lowercase_cow();
            if !has_implicit_role(&name_lower) {
                return None;
            }
            &name_lower
        };

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let role_trimmed = role_value.text().trim();

        let explicit_role = AriaRole::from_roles(role_trimmed)?;

        let implicit_role = get_implicit_role_for_element(name_str, node)?;

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

/// Static map from HTML element names to their implicit ARIA roles.
///
/// Only includes elements with a fixed role that does not depend on attributes.
/// Elements like `input`, `img`, `section`, etc. require attribute inspection
/// and are handled separately in [`get_implicit_role_for_element`].
static SIMPLE_IMPLICIT_ROLES: phf::Map<&str, AriaRole> = phf::phf_map! {
    "article" => AriaRole::Article,
    "aside" => AriaRole::Complementary,
    "blockquote" => AriaRole::Blockquote,
    "button" => AriaRole::Button,
    "caption" => AriaRole::Caption,
    "figcaption" => AriaRole::Caption,
    "legend" => AriaRole::Caption,
    "code" => AriaRole::Code,
    "datalist" => AriaRole::Listbox,
    "del" => AriaRole::Deletion,
    "s" => AriaRole::Deletion,
    "dd" => AriaRole::Definition,
    "dt" => AriaRole::Term,
    "dfn" => AriaRole::Term,
    "mark" => AriaRole::Mark,
    "dialog" => AriaRole::Dialog,
    "em" => AriaRole::Emphasis,
    "figure" => AriaRole::Figure,
    "form" => AriaRole::Form,
    "hr" => AriaRole::Separator,
    "html" => AriaRole::Document,
    "ins" => AriaRole::Insertion,
    "main" => AriaRole::Main,
    "marquee" => AriaRole::Marquee,
    "math" => AriaRole::Math,
    "menu" => AriaRole::List,
    "ul" => AriaRole::List,
    "ol" => AriaRole::List,
    "meter" => AriaRole::Meter,
    "nav" => AriaRole::Navigation,
    "li" => AriaRole::Listitem,
    "option" => AriaRole::Option,
    "hgroup" => AriaRole::Group,
    "optgroup" => AriaRole::Group,
    "address" => AriaRole::Group,
    "details" => AriaRole::Group,
    "fieldset" => AriaRole::Group,
    "output" => AriaRole::Status,
    "p" => AriaRole::Paragraph,
    "progress" => AriaRole::Progressbar,
    "search" => AriaRole::Search,
    "strong" => AriaRole::Strong,
    "sub" => AriaRole::Subscript,
    "sup" => AriaRole::Superscript,
    "svg" => AriaRole::GraphicsDocument,
    "table" => AriaRole::Table,
    "textarea" => AriaRole::Textbox,
    "tr" => AriaRole::Row,
    "td" => AriaRole::Cell,
    "time" => AriaRole::Time,
    "h1" => AriaRole::Heading,
    "h2" => AriaRole::Heading,
    "h3" => AriaRole::Heading,
    "h4" => AriaRole::Heading,
    "h5" => AriaRole::Heading,
    "h6" => AriaRole::Heading,
    "tbody" => AriaRole::Rowgroup,
    "tfoot" => AriaRole::Rowgroup,
    "thead" => AriaRole::Rowgroup,
    "b" => AriaRole::Generic,
    "bdi" => AriaRole::Generic,
    "bdo" => AriaRole::Generic,
    "body" => AriaRole::Generic,
    "data" => AriaRole::Generic,
    "div" => AriaRole::Generic,
    "i" => AriaRole::Generic,
    "q" => AriaRole::Generic,
    "samp" => AriaRole::Generic,
    "small" => AriaRole::Generic,
    "span" => AriaRole::Generic,
    "u" => AriaRole::Generic,
    "pre" => AriaRole::Generic,
    "header" => AriaRole::Generic,
    "footer" => AriaRole::Generic,
};

/// Elements whose implicit role depends on attributes (not in the simple map).
static COMPLEX_ROLE_ELEMENTS: phf::Set<&str> = phf::phf_set! {
    "th", "input", "a", "area", "link", "img", "section", "select",
};

/// Returns `true` if the element has any possible implicit ARIA role.
fn has_implicit_role(element_name: &str) -> bool {
    SIMPLE_IMPLICIT_ROLES.contains_key(element_name)
        || COMPLEX_ROLE_ELEMENTS.contains(element_name)
}

/// Returns the implicit ARIA role for a given HTML element name.
///
/// Based on the WAI-ARIA spec: <https://www.w3.org/TR/html-aria/>
///
/// Expects `element_name` to already be lowercased.
fn get_implicit_role_for_element(element_name: &str, node: &AnyHtmlElement) -> Option<AriaRole> {
    // Fast path: elements with a fixed role (no attribute inspection needed).
    if let Some(&role) = SIMPLE_IMPLICIT_ROLES.get(element_name) {
        return Some(role);
    }

    // Slow path: elements whose implicit role depends on attributes.
    Some(match element_name {
        "th" => {
            let scope_lower = get_attribute_lowercase(node, "scope");
            match scope_lower.as_deref() {
                Some("col") => AriaRole::Columnheader,
                _ => AriaRole::Rowheader,
            }
        }
        "input" => {
            let type_lower = get_attribute_lowercase(node, "type");
            match type_lower.as_deref() {
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
        }
        "a" | "area" | "link" => {
            if node.find_attribute_by_name("href").is_some() {
                AriaRole::Link
            } else {
                AriaRole::Generic
            }
        }
        "img" => {
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
        }
        "section" => {
            if node.find_attribute_by_name("aria-labelledby").is_some()
                || node.find_attribute_by_name("aria-label").is_some()
                || node.find_attribute_by_name("title").is_some()
            {
                AriaRole::Region
            } else {
                AriaRole::Generic
            }
        }
        "select" => {
            let size = get_attribute_value(node, "size")
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(0);
            if node.find_attribute_by_name("multiple").is_none() && size <= 1 {
                AriaRole::Combobox
            } else {
                AriaRole::Listbox
            }
        }
        _ => return None,
    })
}

fn get_attribute_value(node: &AnyHtmlElement, name: &str) -> Option<String> {
    let attr = node.find_attribute_by_name(name)?;
    let value = attr.initializer()?.value().ok()?.string_value()?;
    Some(value.text().to_string())
}

/// Get an attribute value, trimmed and lowercased.
/// HTML enumerated attributes (input type, th scope) are ASCII case-insensitive per spec.
fn get_attribute_lowercase(node: &AnyHtmlElement, name: &str) -> Option<String> {
    let attr = node.find_attribute_by_name(name)?;
    let value = attr.initializer()?.value().ok()?.string_value()?;
    let text = value.text().trim();
    Some(text.to_ascii_lowercase_cow().into_owned())
}
