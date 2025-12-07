use biome_analyze::RuleSource;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::AstNode;
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

        if !is_button_element(element) {
            return None;
        }

        let type_attribute = element.find_attribute_by_name("type");

        let Some(attribute) = type_attribute else {
            return Some(UseButtonTypeState { missing_prop: true });
        };

        let Some(initializer) = attribute.initializer() else {
            return Some(UseButtonTypeState {
                missing_prop: false,
            });
        };

        let value = initializer.value().ok()?;

        if ALLOWED_BUTTON_TYPES.contains(&&*value.string_value()?) {
            return None;
        }

        Some(UseButtonTypeState {
            missing_prop: false,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();

        let message = if state.missing_prop {
            (markup! {
                "Provide an explicit "<Emphasis>"type"</Emphasis>" attribute for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        } else {
            (markup!{
                "Provide a valid "<Emphasis>"type"</Emphasis>" attribute for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        };

        Some(RuleDiagnostic::new(rule_category!(),
            span,
            message
        )
            .note(markup! {
                "The default "<Emphasis>"type"</Emphasis>" of a button is "<Emphasis>"submit"</Emphasis>", which causes the submission of a form when placed inside a `form` element. "
            })
            .note(
            markup! {

                "Allowed button types are: "<Emphasis>"submit"</Emphasis>", "<Emphasis>"button"</Emphasis>" or "<Emphasis>"reset"</Emphasis>""
            }
        ))
    }
}

fn is_button_element(element: &AnyHtmlElement) -> bool {
    element
        .name()
        .is_some_and(|name| name.text().eq_ignore_ascii_case("button"))
}
