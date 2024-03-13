use crate::semantic_services::Semantic;
use biome_analyze::{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
     JsCallExpression, JsIdentifierBinding, JsImport, JsModule,
};
use biome_rowan::{AstNode, WalkEvent};
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Checks that the assertion function, for example `expect`, is placed inside an `it()` function call.
    ///
    /// Placing (and using) the `expect` assertion function can result in unexpected behavoiurs when executing your testing suite.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe(() => {
    ///     expect()
    /// })
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    ///
    /// ```
    ///
    pub NoMisplacedAssertion {
        version: "next",
        name: "noMisplacedAssertion",
        recommended: false,
        source: RuleSource::EslintJest("no-standalone-expect"),
        source_kind: RuleSourceKind::Inspired,
    }
}

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoMisplacedOptions {
    /// The name of the function that will run the assertion. By default, its name is `expect`.
    assertion_function_names: Vec<String>,
    /// A list of specifiers that export the `assertionFunctionName` function.
    ///
    /// If your assertion function name is a global, provide an empty array.
    ///
    /// Defaults to: `"chai"`, `"node:assert"` and `node:assert/strict`.
    specifiers: Vec<String>,
}

impl Default for NoMisplacedOptions {
    fn default() -> Self {
        Self {
            assertion_function_names: vec!["expect".to_string(), "assert".to_string()],
            specifiers: vec![
                "chai".to_string(),
                "node:assert".to_string(),
                "node:assert/strict".to_string(),
            ],
        }
    }
}

impl Rule for NoMisplacedAssertion {
    type Query = Semantic<JsModule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoMisplacedOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let options = ctx.options();
        let preorder = node.syntax().preorder();
        let mut inside_describe_call = false;
        let mut inside_it_call = false;
        let mut assertion_call = None;
        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(node) = JsCallExpression::cast(node) {
                        if let Ok(callee) = node.callee() {
                            if callee.is_test_describe_call() {
                                inside_describe_call = true
                            }
                            if callee.is_test_it_call() {
                                inside_it_call = true
                            }
                        }
                    }
                }
                WalkEvent::Leave(node) => {

                    if let Some(node) = JsCallExpression::cast(node) {
                        if let Ok(callee) = node.callee() {
                            if callee.is_test_describe_call() {
                                inside_describe_call = false
                            }
                            if callee.is_test_it_call() {
                                inside_it_call = false
                            }
                            if let Some(identifier) = callee.get_callee_object_identifier() {
                                if inside_describe_call && !inside_it_call {
                                    assertion_call =
                                        Some(identifier);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(assertion_call) = assertion_call {
            let call_text = assertion_call.value_token().ok()?;
            let binding = model.binding(&assertion_call);
            dbg!(&binding);
            if !options.specifiers.is_empty() {
                if let Some(binding) = binding {
                    let ident = JsIdentifierBinding::cast_ref(binding.syntax())?;
                    let import = ident.syntax().ancestors().find_map(JsImport::cast)?;
                    let source_text = import.source_text().ok()?;
                    if options.assertion_function_names.iter()
                        .find(|function_name| function_name.as_str() == call_text.text_trimmed())
                        .is_some()

                        && options
                        .specifiers
                        .iter()
                        .find(|specifier| specifier.as_str() == source_text.text())
                        .is_some()

                    {
                        return Some(assertion_call.range());
                    }
                }
            } else {
                if options.assertion_function_names.iter()
                    .find(|function_name| function_name.as_str() == call_text.text_trimmed())
                    .is_some()

                {
                    return Some(assertion_call.range());
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "The assertion isn't inside a "<Emphasis>"it()"</Emphasis>" function call."
                },
            )
            .note(markup! {
                "This will result in unexpected behaviours from your test suite."
            })
            .note(markup! {
                "Move the assertion inside a "<Emphasis>"it()"</Emphasis>" function call."
            }),
        )
    }
}
