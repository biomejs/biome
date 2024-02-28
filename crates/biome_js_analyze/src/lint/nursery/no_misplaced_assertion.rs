use crate::semantic_services::Semantic;
use biome_analyze::{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_js_syntax::{
    AnyJsNamedImportSpecifier, JsCallExpression, JsIdentifierBinding, JsImport, JsModule,
};
use biome_rowan::{AstNode, WalkEvent};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
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

impl Rule for NoMisplacedAssertion {
    type Query = Semantic<JsModule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
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
                            if callee.is_assertion_call() {
                                if inside_describe_call && !inside_it_call {
                                    assertion_call =
                                        Some(node.callee().ok()?.as_js_reference_identifier()?);
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
            if let Some(binding) = binding {
                let ident = JsIdentifierBinding::cast_ref(binding.syntax())?;
                let import_specifier = ident.parent::<AnyJsNamedImportSpecifier>()?;
                let import = import_specifier.import_clause()?.parent::<JsImport>()?;
                let source_text = import.source_text().ok()?;
                if (matches!(source_text.text(), "chai") && call_text.text_trimmed() == "expect")
                    || (matches!(call_text.text_trimmed(), "assert")
                        && matches!(source_text.text(), "node:assert" | "node:assert/strict"))
                {
                    return Some(assertion_call.range());
                }
            } else {
                return Some(assertion_call.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "The assertion isn't inside an it call."
            },
        ))
    }
}
