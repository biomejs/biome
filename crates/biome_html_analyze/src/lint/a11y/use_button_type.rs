use biome_analyze::RuleSource;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;

use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_button_type::UseButtonTypeOptions;

use crate::utils::is_html_tag;

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
        version: "2.4.0",
        name: "useButtonType",
        language: "html",
        sources: &[RuleSource::EslintReact("button-has-type").inspired(), RuleSource::EslintReactDom("no-missing-button-type").inspired(), RuleSource::EslintReactXyz("dom-no-missing-button-type").inspired(), RuleSource::HtmlEslint("require-button-type").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

const ALLOWED_BUTTON_TYPES: [&str; 3] = ["submit", "button", "reset"];

pub struct UseButtonTypeState {
    missing_prop: bool,
}

impl Rule for UseButtonType {
    type Query = Ast<AnyHtmlTagElement>;
    type State = UseButtonTypeState;
    type Signals = Option<Self::State>;
    type Options = UseButtonTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        if !is_html_tag(element, source_type, "button") {
            return None;
        }

        let type_attribute = element.find_attribute_or_vue_binding("type");

        // If no regular attribute found, check for Svelte shorthand syntax
        let Some(attribute) = type_attribute else {
            // No regular attribute and no shorthand - missing type attribute
            return Some(UseButtonTypeState { missing_prop: true });
        };

        // Check static value first (works for both HTML and Vue static bindings)
        if let Some(string_value) = attribute.as_static_value() {
            return if ALLOWED_BUTTON_TYPES.contains(&string_value.text()) {
                None
            } else {
                Some(UseButtonTypeState {
                    missing_prop: false,
                })
            };
        }

        // For HTML attributes with no initializer (bare `type` with no value is invalid)
        if let Some(html_attr) = attribute.as_html_attribute()
            && html_attr.initializer().is_none()
        {
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
