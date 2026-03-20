use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute};
use biome_rowan::{AstNode, BatchMutationExt};
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
        version: "2.4.0",
        name: "noRedundantRoles",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-redundant-roles").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    redundant_attribute: HtmlAttribute,
    role_value: String,
    element_name: String,
}

impl Rule for NoRedundantRoles {
    type Query = Ast<AnyHtmlElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoRedundantRolesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let element_name = node.name()?;
        let element_name_str = element_name.text().to_lowercase();

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let role_value = role_attribute_value.trim().to_lowercase();

        let explicit_role = AriaRole::from_roles(&role_value)?;
        let implicit_role = get_implicit_role_for_element(&element_name_str, node)?;

        if explicit_role == implicit_role {
            return Some(RuleState {
                redundant_attribute: role_attribute,
                role_value: role_value.to_string(),
                element_name: element_name_str.to_string(),
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.redundant_attribute.range(),
            markup! {
                "Using the role attribute '"{&state.role_value}"' on the '"{&state.element_name}"' element is redundant, because it is implied by its semantic."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.redundant_attribute.clone());
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
/// This is a simplified version of the implicit role mappings from the
/// WAI-ARIA spec (https://www.w3.org/TR/html-aria/) that handles the
/// common cases where the implicit role depends only on the tag name.
///
/// For elements whose implicit role depends on attributes (e.g. `<input type="...">`),
/// we also inspect the element's attributes.
fn get_implicit_role_for_element(
    element_name: &str,
    node: &AnyHtmlElement,
) -> Option<AriaRole> {
    Some(match element_name {
        "article" => AriaRole::Article,
        "aside" => AriaRole::Complementary,
        "blockquote" => AriaRole::Blockquote,
        "button" => AriaRole::Button,
        "caption" | "figcaption" | "legend" => AriaRole::Caption,
        "code" => AriaRole::Code,
        "datalist" => AriaRole::Listbox,
        "del" | "s" => AriaRole::Deletion,
        "dd" => AriaRole::Definition,
        "dt" | "dfn" => AriaRole::Term,
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
        "menu" | "ul" | "ol" => AriaRole::List,
        "meter" => AriaRole::Meter,
        "nav" => AriaRole::Navigation,
        "li" => AriaRole::Listitem,
        "option" => AriaRole::Option,
        "hgroup" | "optgroup" | "address" | "details" | "fieldset" => AriaRole::Group,
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
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => AriaRole::Heading,
        "tbody" | "tfoot" | "thead" => AriaRole::Rowgroup,
        "th" => {
            let scope_value = get_attribute_value(node, "scope");
            match scope_value.as_deref() {
                Some("col") => AriaRole::Columnheader,
                _ => AriaRole::Rowheader,
            }
        }
        "input" => {
            let type_value = get_attribute_value(node, "type");
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
        }
        "a" | "area" => {
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
        "b" | "bdi" | "bdo" | "body" | "data" | "div" | "i" | "q" | "samp" | "small"
        | "span" | "u" | "pre" | "header" | "footer" => AriaRole::Generic,
        _ => return None,
    })
}

/// Helper to extract a string attribute value from an HTML element.
fn get_attribute_value(node: &AnyHtmlElement, name: &str) -> Option<String> {
    let attr = node.find_attribute_by_name(name)?;
    let value = attr.initializer()?.value().ok()?.string_value()?;
    Some(value)
}
