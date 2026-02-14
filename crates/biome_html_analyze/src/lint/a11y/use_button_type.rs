use biome_analyze::RuleSource;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_button_type::UseButtonTypeOptions;

declare_lint_rule! {
    /// Enforces the usage and validity of the attribute `type` for the element `button`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <button>Do something</button>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <button type="incorrectType">Do something</button>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <button type="button">Do something</button>
    /// ```
    ///
    pub UseButtonType {
        version: "next",
        name: "useButtonType",
        language: "html",
        sources: &[RuleSource::EslintReact("button-has-type").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

const ALLOWED_BUTTON_TYPES: [&str; 3] = ["submit", "button", "reset"];

pub struct UseButtonTypeState {
    missing_prop: bool,
}

impl Rule for UseButtonType {
    type Query = Ast<AnyHtmlElement>;
    type State = UseButtonTypeState;
    type Signals = Option<Self::State>;
    type Options = UseButtonTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if !is_button_element(element, ctx) {
            return None;
        }

        let type_attribute = element.find_attribute_by_name("type");

        // If no regular attribute found, check for Svelte shorthand syntax
        if type_attribute.is_none() {
            // Check if there's a shorthand attribute like {type}
            if has_dynamic_attribute(element, "type") {
                // We can't validate the runtime value, but the attribute will be provided
                // Assume it's valid since we can't determine the value
                return None;
            }
            // No regular attribute and no shorthand - missing type attribute
            return Some(UseButtonTypeState { missing_prop: true });
        }

        let attribute = type_attribute?;

        let Some(initializer) = attribute.initializer() else {
            return Some(UseButtonTypeState {
                missing_prop: false,
            });
        };

        let value = initializer.value().ok()?;

        // If the value is a dynamic expression (e.g., {foo} in Svelte), we can't validate it,
        // so we assume it's valid to avoid false positives.
        // We only validate static string values.
        if value.as_html_string().is_some() {
            // Static string value - validate it
            if let Some(string_value) = value.string_value()
                && ALLOWED_BUTTON_TYPES.contains(&&*string_value)
            {
                return None;
            }
            // Invalid static value
            return Some(UseButtonTypeState {
                missing_prop: false,
            });
        }

        // Dynamic expression - assume valid
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(),
            ctx.query().range(),
            if state.missing_prop {
                markup! {
                    "Provide an explicit "<Emphasis>"type"</Emphasis>" attribute for the "<Emphasis>"button"</Emphasis>" element."
                }
            } else {
                markup!{
                    "Provide a valid "<Emphasis>"type"</Emphasis>" attribute for the "<Emphasis>"button"</Emphasis>" element."
                }
            }
        )
            .note(markup! {
                "The default "<Emphasis>"type"</Emphasis>" of a button is "<Emphasis>"submit"</Emphasis>", which causes the submission of a form when placed inside a `form` element."
            })
            .note(
            markup! {

                "Allowed button types are: "<Emphasis>"submit"</Emphasis>", "<Emphasis>"button"</Emphasis>" or "<Emphasis>"reset"</Emphasis>""
            }
        ))
    }
}

fn is_button_element(element: &AnyHtmlElement, ctx: &RuleContext<UseButtonType>) -> bool {
    let Some(element_name) = element.name() else {
        return false;
    };

    let source_type = ctx.source_type::<HtmlFileSource>();

    // In HTML files: case-insensitive (BUTTON, Button, button all match)
    // In component frameworks (Vue, Svelte, Astro): case-sensitive (only "button" matches)
    // This means <Button> in Vue/Svelte is treated as a component and ignored
    if source_type.is_html() {
        element_name.text().eq_ignore_ascii_case("button")
    } else {
        element_name.text() == "button"
    }
}

/// Checks if a dynamic attribute (shorthand or directive) exists for the given name.
/// For example, `<button {type}>` (Svelte), `<button :type="foo">` (Vue), or `<button v-bind:type="foo">` (Vue).
fn has_dynamic_attribute(element: &AnyHtmlElement, name: &str) -> bool {
    let Some(attributes) = element.attributes() else {
        return false;
    };

    attributes
        .iter()
        .find_map(|attr| {
            // Check if this is a HtmlSingleTextExpression (Svelte shorthand syntax)
            if let Some(single_expr) = attr.as_html_attribute_single_text_expression() {
                // Check if the expression text matches the attribute name we're looking for
                let expression = single_expr.expression().ok()?.html_literal_token().ok()?;
                return if expression.text() == name {
                    Some(())
                } else {
                    None
                };
            } else if let Some(vue_directive) = attr.as_any_vue_directive() {
                // Check for v-bind:type="foo" (longhand)
                let directive_arg = if let Some(dir) = vue_directive.as_vue_directive()
                    && dir.name_token().ok()?.text_trimmed() == "v-bind"
                {
                    dir.arg()
                } else if let Some(dir) = vue_directive.as_vue_v_bind_shorthand_directive() {
                    dir.arg().ok()
                } else {
                    None
                }?;

                let name_token = directive_arg
                    .arg()
                    .ok()?
                    .as_vue_static_argument()?
                    .name_token()
                    .ok()?;
                if name_token.text_trimmed() == name {
                    return Some(());
                }
            }
            None
        })
        .is_some()
}
