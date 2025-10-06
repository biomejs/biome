use crate::{JsRuleAction, services::semantic::Semantic, utils::rename::RenameSymbolExtensions};
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    JsIdentifierBinding, JsSyntaxKind,
    binding_ext::{AnyJsBindingDeclaration, AnyJsParameterParentFunction},
};
use biome_rowan::{AstNode, BatchMutationExt, Direction};
use biome_rule_options::no_unused_function_parameters::NoUnusedFunctionParametersOptions;

declare_lint_rule! {
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
    /// ```js
    /// function withObjectSpread({ a, ...rest }) {
    ///	    return rest;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule has the following options
    ///
    /// ### `ignoreRestSiblings`
    /// **Since `v2.1.0`**
    ///
    /// Whether to ignore unused variables from an object destructuring with a spread.
    /// Example: `a` and `b` in `function({ a, b, ...rest }) { return rest;}` should be ignored by this rule when set to false.
    ///
    /// Defaults to `true`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreRestSiblings": false
    ///   }
    /// }
    /// ```
    ///
    /// ```js,use_options,expect_diagnostic
    /// function withObjectSpread({ b, ...rest }) {
    ///	    return rest;
    /// }
    /// ```
    ///
    ///
    pub NoUnusedFunctionParameters {
        version: "1.8.0",
        name: "noUnusedFunctionParameters",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
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
    parent_function: &Option<AnyJsParameterParentFunction>,
) -> bool {
    matches!(
        parent_function,
        Some(
            // bindings in signatures are ok to not be used
            AnyJsParameterParentFunction::TsMethodSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsCallSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsConstructSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsConstructorSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsMethodSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsSetterSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsSetterSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsIndexSignatureClassMember(_)
            // bindings in function types are ok to not be used
            | AnyJsParameterParentFunction::TsFunctionType(_)
            | AnyJsParameterParentFunction::TsConstructorType(_)
            // binding in declare are ok to not be used
            | AnyJsParameterParentFunction::TsDeclareFunctionDeclaration(_)
            | AnyJsParameterParentFunction::TsDeclareFunctionExportDefaultDeclaration(_)
        )
    )
}

impl Rule for NoUnusedFunctionParameters {
    type Query = Semantic<JsIdentifierBinding>;
    type State = SuggestedFix;
    type Signals = Option<Self::State>;
    type Options = NoUnusedFunctionParametersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let declaration = binding.declaration()?;

        let ignore_rest_siblings = ctx.options().ignore_rest_siblings;

        let name = binding.name_token().ok()?;
        let name = name.text_trimmed();

        if name.starts_with('_') {
            return None;
        }

        if ignore_rest_siblings {
            // Ignore object patterns with a rest spread.
            // e.g. `{ a, ...rest }`
            if let AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_) = &declaration
                && declaration
                    .syntax()
                    .siblings(Direction::Next)
                    .last()
                    .is_some_and(|last_sibling| {
                        matches!(
                            last_sibling.kind(),
                            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST
                        )
                    })
            {
                return None;
            }
        }

        let parent_function = match declaration
            .parent_binding_pattern_declaration()
            .unwrap_or(declaration)
        {
            AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter.parent_function(),
            AnyJsBindingDeclaration::JsRestParameter(parameter) => parameter.parent_function(),
            AnyJsBindingDeclaration::JsBogusParameter(_) => {
                return Some(SuggestedFix::NoSuggestion);
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
                let new_name = format!("_{name_trimmed}");

                let model = ctx.model();
                if !mutation.rename_node_declaration(model, binding, &new_name) {
                    return None;
                }

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "If this is intentional, prepend "<Emphasis>{name_trimmed}</Emphasis>" with an underscore." }
                    .to_owned(),
                    mutation,
                ))
            }
        }
    }
}
