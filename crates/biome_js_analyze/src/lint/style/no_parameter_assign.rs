use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Severity;
use biome_js_semantic::{Reference, ReferencesExtensions};
use biome_js_syntax::AnyJsAssignment;
use biome_js_syntax::{
    AnyJsStatement, JsExpressionStatement, JsIdentifierBinding,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_js_syntax::{JsAssignmentExpression, JsPostUpdateExpression, JsPreUpdateExpression};
use biome_rowan::AstNode;
use biome_rowan::declare_node_union;
use biome_rule_options::no_parameter_assign::{NoParameterAssignOptions, PropertyAssignmentMode};

declare_lint_rule! {
    /// Disallow reassigning `function` parameters.
    ///
    /// Assignment to `function` parameters can be misleading and confusing,
    /// as modifying parameters will also mutate the `arguments` object.
    /// It is often unintended and indicative of a programmer error.
    ///
    /// In contrast to the _ESLint_ rule, this rule cannot be configured to report
    /// assignments to a property of a parameter.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f(param) {
    ///     param = 13;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(param) {
    ///     param++;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(param) {
    ///     for (param of arr) {}
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class C {
    ///     constructor(readonly prop: number) {
    ///         prop++;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(param) {
    ///     let local = param;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### propertyAssignment
    ///
    /// The `noParameterAssign` rule can be configured using the `propertyAssignment` option, which determines whether property assignments on function parameters are allowed or denied. By default, `propertyAssignment` is set to `allow`.
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "propertyAssignment": "allow"
    ///     }
    /// }
    /// ```
    ///
    /// - **allow**: Allows property assignments on function parameters. This is the default behavior.
    ///   - Example:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "propertyAssignment": "allow"
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// function update(obj) {
    ///     obj.key = "value"; // No diagnostic
    /// }
    /// ```
    ///
    /// - **deny**: Disallows property assignments on function parameters, enforcing stricter immutability.
    ///   - Example:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "propertyAssignment": "deny"
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options,expect_diagnostic
    /// function update(obj) {
    ///     obj.key = "value"; // Diagnostic: Assignment to a property of function parameter is not allowed.
    /// }
    /// ```
    pub NoParameterAssign {
        version: "1.0.0",
        name: "noParameterAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-param-reassign").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoParameterAssign {
    type Query = Semantic<JsIdentifierBinding>;
    type State = ProblemType;
    type Signals = Vec<Self::State>;
    type Options = NoParameterAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let mut signals = Vec::new();
        let model = ctx.model();

        if let Some(declaration) = binding.declaration() {
            let options = ctx.options();
            if options.property_assignment == PropertyAssignmentMode::Deny
                && matches!(declaration, AnyJsBindingDeclaration::JsFormalParameter(_))
            {
                let expressions: Vec<_> = binding
                    .all_reads(model)
                    .filter_map(|reference| extract_statement_from_reference(&reference))
                    .filter_map(|statement| {
                        let left = statement
                            .expression()
                            .ok()?
                            .as_js_assignment_expression()?
                            .left()
                            .ok()?;

                        match left.as_any_js_assignment()? {
                            AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                                assignment.object().ok()
                            }
                            AnyJsAssignment::JsStaticMemberAssignment(assignment) => {
                                assignment.object().ok()
                            }
                            _ => None,
                        }
                    })
                    .map(|expression| {
                        ProblemType::PropertyAssignment(expression.syntax().text_trimmed_range())
                    })
                    .collect();

                signals.extend(expressions);
            }

            if matches!(
                declaration,
                AnyJsBindingDeclaration::JsFormalParameter(_)
                    | AnyJsBindingDeclaration::JsRestParameter(_)
                    | AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
                    | AnyJsBindingDeclaration::TsPropertyParameter(_)
            ) {
                let param_reassignments: Vec<_> = binding
                    .all_writes(model)
                    .map(|expression| {
                        ProblemType::ParameterAssignment(expression.syntax().text_trimmed_range())
                    })
                    .collect();

                signals.extend(param_reassignments);
            }
        }

        signals
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            ProblemType::ParameterAssignment(text_range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    text_range,
                    markup! {
                        "Assigning a "<Emphasis>"function parameter"</Emphasis>" is confusing."
                    },
                )
                    .detail(
                        ctx.query().syntax().text_trimmed_range(),
                        markup! {
                        "The "<Emphasis>"parameter"</Emphasis>" is declared here:"
                    },
                    )
                    .note(markup! {
                    "Developers usually expect function parameters to be readonly. To align with this expectation, use a local variable instead."
                }),
            ),

            ProblemType::PropertyAssignment(text_range) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        text_range,
                        markup! {
                        "Assigning to a "<Emphasis>"property of a function parameter"</Emphasis>" is confusing."
                    })
                        .note(markup! {"Function callers usually don't expect the parameters they pass in to be modified. To avoid mutation, create a new instance and return it to the caller."}),
                )
            }
        }
    }
}

fn extract_statement_from_reference(reference: &Reference) -> Option<JsExpressionStatement> {
    reference
        .syntax()
        .ancestors()
        .skip(2) // skip the reference identifier and its expression
        .skip_while(|node| AnyJsAssignmentLike::can_cast(node.kind()))
        .find_map(AnyJsStatement::cast)
        .and_then(|stmt| match stmt {
            AnyJsStatement::JsExpressionStatement(statement) => Some(statement),
            _ => None,
        })
}

declare_node_union! {
    pub AnyJsAssignmentLike =
        JsPostUpdateExpression
        | JsPreUpdateExpression
        | JsAssignmentExpression
}

#[derive(Debug)]
pub enum ProblemType {
    ParameterAssignment(TextRange),
    PropertyAssignment(TextRange),
}
