use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    binding_ext::{AnyJsBindingDeclaration, JsAnyParameterParentFunction},
    JsIdentifierBinding,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::{services::semantic::Semantic, utils::rename::RenameSymbolExtensions, JsRuleAction};

declare_rule! {
    /// Disallow unused function parameters.
    ///
    /// There is an exception to this rule:
    /// parameters that starts with underscore, e.g. `function foo(_a, _b) {}`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo(myVar) {
    ///     console.log('foo');
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Promise((accept, reject) => {
    ///     window.setTimeout(accept, 1000);
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const squares = [[1, 1], [2, 4], [3, 9], [4, 16]];
    /// squares.filter(([k, v]) => v > 5);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo(myVar) {
    ///     console.log(myVar);
    /// }
    /// ```
    ///
    pub NoUnusedFunctionParameters {
        version: "1.8.0",
        name: "noUnusedFunctionParameters",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

/// Suggestion if the binding is unused
#[derive(Debug)]
pub enum SuggestedFix {
    /// No suggestion will be given
    NoSuggestion,
    /// Suggest to prefix the name of the binding with underscore
    PrefixUnderscore,
}

fn is_function_that_is_ok_parameter_not_be_used(
    parent_function: &Option<JsAnyParameterParentFunction>,
) -> bool {
    matches!(
        parent_function,
        Some(
            // bindings in signatures are ok to not be used
            JsAnyParameterParentFunction::TsMethodSignatureClassMember(_)
            | JsAnyParameterParentFunction::TsCallSignatureTypeMember(_)
            | JsAnyParameterParentFunction::TsConstructSignatureTypeMember(_)
            | JsAnyParameterParentFunction::TsConstructorSignatureClassMember(_)
            | JsAnyParameterParentFunction::TsMethodSignatureTypeMember(_)
            | JsAnyParameterParentFunction::TsSetterSignatureClassMember(_)
            | JsAnyParameterParentFunction::TsSetterSignatureTypeMember(_)
            // bindings in function types are ok to not be used
            | JsAnyParameterParentFunction::TsFunctionType(_)
            // binding in declare are ok to not be used
            | JsAnyParameterParentFunction::TsDeclareFunctionDeclaration(_)
        )
    )
}

impl Rule for NoUnusedFunctionParameters {
    type Query = Semantic<JsIdentifierBinding>;
    type State = SuggestedFix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let declaration = binding.declaration()?;

        let name = binding.name_token().ok()?;
        let name = name.text_trimmed();

        if name.starts_with('_') {
            return None;
        }

        let parent_function = match declaration
            .parent_binding_pattern_declaration()
            .unwrap_or(declaration)
        {
            AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter.parent_function(),
            AnyJsBindingDeclaration::JsRestParameter(parameter) => parameter.parent_function(),
            AnyJsBindingDeclaration::JsBogusParameter(_) => {
                return Some(SuggestedFix::NoSuggestion)
            }
            _ => return None,
        };
        if is_function_that_is_ok_parameter_not_be_used(&parent_function) {
            return None;
        }
        let model = ctx.model();
        if binding.all_references(model).next().is_some() {
            return None;
        }
        Some(if binding.is_under_object_pattern_binding()? {
            SuggestedFix::NoSuggestion
        } else {
            SuggestedFix::PrefixUnderscore
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                binding.range(),
                markup! {
                    "This "<Emphasis>"parameter"</Emphasis>" is unused."
                },
            )
            .note(markup! {
                "Unused parameters might be the result of an incomplete refactoring."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, suggestion: &Self::State) -> Option<JsRuleAction> {
        match suggestion {
            SuggestedFix::NoSuggestion => None,
            SuggestedFix::PrefixUnderscore => {
                let binding = ctx.query();
                let mut mutation = ctx.root().begin();

                let name = binding.name_token().ok()?;
                let name_trimmed = name.text_trimmed();
                let new_name = format!("_{}", name_trimmed);

                let model = ctx.model();
                mutation.rename_node_declaration(model, binding, &new_name);

                Some(JsRuleAction::new(
                    ActionCategory::QuickFix,
                    ctx.metadata().applicability(),
                    markup! { "If this is intentional, prepend "<Emphasis>{name_trimmed}</Emphasis>" with an underscore." }
                    .to_owned(),
                    mutation,
                ))
            }
        }
    }
}
