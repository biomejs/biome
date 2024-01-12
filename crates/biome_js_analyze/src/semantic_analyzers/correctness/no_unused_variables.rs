use crate::JsRuleAction;
use crate::{semantic_services::Semantic, utils::rename::RenameSymbolExtensions};
use biome_analyze::RuleSource;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::binding_ext::{
    AnyJsBindingDeclaration, AnyJsIdentifierBinding, JsAnyParameterParentFunction,
};
use biome_js_syntax::declaration_ext::is_in_ambient_context;
use biome_js_syntax::{
    AnyJsExpression, JsClassExpression, JsFileSource, JsForStatement, JsFunctionExpression,
    JsIdentifierExpression, JsSequenceExpression, JsSyntaxKind, JsSyntaxNode, TsConditionalType,
    TsInferType,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult};

declare_rule! {
    /// Disallow unused variables.
    ///
    /// There are two exceptions to this rule:
    /// 1. variables that starts with underscore, ex: `let _something;`
    /// 2. the `React` variable;
    ///
    /// The pattern of having an underscore as prefix of a name of variable is a very diffuse
    /// pattern among programmers, and Biome decided to follow it.
    ///
    /// Importing the `React` variable was a mandatory pattern until some time ago:
    ///
    /// For the time being this rule will ignore it, but this **might change in the future releases**.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let a = 4;
    /// a++;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export function foo(myVar) {
    ///     console.log('foo');
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     foo();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = () => {
    ///     foo();
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// export function f<T>() {}
    /// ```
    ///
    /// # Valid
    ///
    /// ```js
    /// function foo(b) {
    ///     console.log(b)
    /// };
    /// foo();
    /// ```
    ///
    /// ```js
    /// export function foo(_unused) {}
    /// ```
    ///
    /// ```jsx
    /// import React from 'react';
    /// function foo() {
    ///     return <div />;
    /// };
    /// foo();
    /// ```
    ///
    /// ```ts
    /// function used_overloaded(): number;
    /// function used_overloaded(s: string): string;
    /// function used_overloaded(s?: string) {
    ///     return s;
    /// }
    /// used_overloaded();
    /// ```
    pub(crate) NoUnusedVariables {
        version: "1.0.0",
        name: "noUnusedVariables",
        source: RuleSource::Eslint("no-unused-vars"),
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

/// Suggestion if the bindnig is unused
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

fn suggestion_for_binding(binding: &AnyJsIdentifierBinding) -> Option<SuggestedFix> {
    if binding.is_under_object_pattern_binding()? {
        Some(SuggestedFix::NoSuggestion)
    } else {
        Some(SuggestedFix::PrefixUnderscore)
    }
}

// It is ok in some Typescripts constructs for a parameter to be unused.
// Returning None means is ok to be unused
fn suggested_fix_if_unused(binding: &AnyJsIdentifierBinding) -> Option<SuggestedFix> {
    match binding.declaration()? {
        // ok to not be used
        AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExpression(_)
        | AnyJsBindingDeclaration::JsFunctionExpression(_)
        | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
        | AnyJsBindingDeclaration::TsMappedType(_) => None,

        // Some parameters are ok to not be used
        AnyJsBindingDeclaration::JsArrowFunctionExpression(_) => {
            suggestion_for_binding(binding)
        }
        AnyJsBindingDeclaration::TsPropertyParameter(_) => None,
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
            if is_function_that_is_ok_parameter_not_be_used(&parameter.parent_function()) {
                None
            } else {
                suggestion_for_binding(binding)
            }
        }
        AnyJsBindingDeclaration::JsRestParameter(parameter) => {
            if is_function_that_is_ok_parameter_not_be_used(&parameter.parent_function()) {
                None
            } else {
                suggestion_for_binding(binding)
            }
        }

        // declarations need to be check if they are under `declare`
        node @ AnyJsBindingDeclaration::JsVariableDeclarator(_) => {
            if is_in_ambient_context(node.syntax()) {
                None
            } else {
                suggestion_for_binding(binding)
            }
        }
        node @ (AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)
        | AnyJsBindingDeclaration::JsClassDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionDeclaration(_)
        | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
        | AnyJsBindingDeclaration::TsEnumDeclaration(_)
        | AnyJsBindingDeclaration::TsModuleDeclaration(_)
        | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)) => {
            if is_in_ambient_context(node.syntax()) {
                None
            } else {
                Some(SuggestedFix::NoSuggestion)
            }
        }

        // Bindings under catch are never ok to be unused
        AnyJsBindingDeclaration::JsCatchDeclaration(_)
        // Type parameters are never ok to be unused
        | AnyJsBindingDeclaration::TsTypeParameter(_) => Some(SuggestedFix::PrefixUnderscore),

        AnyJsBindingDeclaration::TsInferType(_) => {
            let binding_name_token = binding.name_token().ok()?;
            let binding_name = binding_name_token.text_trimmed();
            let conditional_type = binding.syntax().ancestors().find_map(TsConditionalType::cast)?;
            let last_binding_name_token = conditional_type.extends_type().ok()?.syntax()
                .descendants()
                .filter_map(TsInferType::cast)
                .filter_map(|infer_type| infer_type.name().ok()?.ident_token().ok())
                .filter(|infer_type_name| infer_type_name.text_trimmed() == binding_name)
                .last()?;
            // We ignore `infer T` that precedes another `infer T`.
            // Thus, only the last `infer T` is considered.
            // See https://github.com/biomejs/biome/issues/565
            if binding_name_token.text_range() == last_binding_name_token.text_range() {
                Some(SuggestedFix::NoSuggestion)
            } else {
                None
            }
        }

        // Bindings under unknown parameter are never ok to be unused
        AnyJsBindingDeclaration::JsBogusParameter(_)
        // Imports are never ok to be unused
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
        // exports with binding are ok to be unused
        | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
            Some(SuggestedFix::NoSuggestion)
        }
    }
}

impl Rule for NoUnusedVariables {
    type Query = Semantic<AnyJsIdentifierBinding>;
    type State = SuggestedFix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        if ctx
            .source_type::<JsFileSource>()
            .language()
            .is_definition_file()
        {
            // Ignore TypeScript declaration files
            // This allows ignoring declaration files without any `export`
            // that implicitly export their members.
            return None;
        }

        let binding = ctx.query();
        let name = binding.name_token().ok()?;
        let name = name.text_trimmed();

        // Legacy React framework requires to import `React`, even if it is not used.
        // This is required for old versions of the Babel compiler.
        if name.starts_with('_') || name == "React" {
            return None;
        }

        // Ignore expressions
        if binding.parent::<JsFunctionExpression>().is_some()
            || binding.parent::<JsClassExpression>().is_some()
        {
            return None;
        }

        let Some(suggestion) = suggested_fix_if_unused(binding) else {
            return None;
        };

        let model = ctx.model();
        if model.is_exported(binding) {
            return None;
        }

        // We need to check if all uses of this binding are somehow recursive or unused
        let declaration = binding.declaration()?;
        let declaration = declaration.syntax();
        binding
            .all_references(model)
            .filter_map(|reference| {
                let ref_parent = reference.syntax().parent()?;
                if reference.is_write() {
                    // Skip self assignment such as `a += 1` and `a++`.
                    // Ensure that the assignment is not used in an used expression.
                    let is_statement_like = ref_parent
                        .ancestors()
                        .find(|x| {
                            matches!(
                                x.kind(),
                                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                                    | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
                                    | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                            )
                        })
                        .and_then(|x| is_unused_expression(&x).ok())
                        .unwrap_or(false);
                    if is_statement_like {
                        return None;
                    }
                } else if JsIdentifierExpression::can_cast(ref_parent.kind())
                    && is_unused_expression(&ref_parent).ok()?
                {
                    // The reference is in an unused expression
                    return None;
                }
                Some(ref_parent)
            })
            .all(|ref_parent| {
                if matches!(
                    declaration.kind(),
                    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION | JsSyntaxKind::TS_MAPPED_TYPE
                ) {
                    // Expression in an return position inside an arrow function expression are used.
                    // Type parameters declared in mapped types are only used in the mapped type.
                    return false;
                }
                let mut is_unused = true;
                for ancestor in ref_parent.ancestors() {
                    if &ancestor == declaration {
                        // inside the declaration
                        return is_unused;
                    }
                    match ancestor.kind() {
                            JsSyntaxKind::JS_FUNCTION_BODY => {
                                // reset because we are inside a function
                                is_unused = true;
                            }
                            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                            | JsSyntaxKind::JS_CALL_EXPRESSION
                            | JsSyntaxKind::JS_NEW_EXPRESSION
                            // These can call a getter
                            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                                // The ref can be leaked or code can be executed
                                is_unused = false;
                            }
                            _ => {}
                        }
                }
                // Always false when the ref is outside the declaration
                false
            })
            .then_some(suggestion)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();

        let symbol_type = match binding.syntax().parent().unwrap().kind() {
            JsSyntaxKind::JS_FORMAL_PARAMETER => "parameter",
            JsSyntaxKind::JS_FUNCTION_DECLARATION => "function",
            JsSyntaxKind::JS_CLASS_DECLARATION => "class",
            JsSyntaxKind::TS_INTERFACE_DECLARATION => "interface",
            JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => "type alias",
            JsSyntaxKind::TS_TYPE_PARAMETER => "type parameter",
            _ => "variable",
        };

        let diag = RuleDiagnostic::new(
            rule_category!(),
            binding.syntax().text_trimmed_range(),
            markup! {
                "This " {symbol_type} " is unused."
            },
        );

        let diag = diag.note(
            markup! {"Unused variables usually are result of incomplete refactoring, typos and other source of bugs."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, suggestion: &Self::State) -> Option<JsRuleAction> {
        match suggestion {
            SuggestedFix::NoSuggestion => None,
            SuggestedFix::PrefixUnderscore => {
                let binding = ctx.query();
                let mut mutation = ctx.root().begin();

                let name = match binding {
                    AnyJsIdentifierBinding::JsIdentifierBinding(binding) => {
                        binding.name_token().ok()?
                    }
                    AnyJsIdentifierBinding::TsIdentifierBinding(binding) => {
                        binding.name_token().ok()?
                    }
                    AnyJsIdentifierBinding::TsTypeParameterName(binding) => {
                        binding.ident_token().ok()?
                    }
                };
                let name_trimmed = name.text_trimmed();
                let new_name = format!("_{}", name_trimmed);

                let model = ctx.model();
                mutation.rename_node_declaration(model, binding.clone(), &new_name);

                Some(JsRuleAction {
                    mutation,
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "If this is intentional, prepend "<Emphasis>{name_trimmed}</Emphasis>" with an underscore." }
                        .to_owned(),
                })
            }
        }
    }
}

/// Returns `true` if `expr` is unused.
fn is_unused_expression(expr: &JsSyntaxNode) -> SyntaxResult<bool> {
    debug_assert!(AnyJsExpression::can_cast(expr.kind()));
    // We use range as a way to identify nodes without owning them.
    let mut previous = expr.text_trimmed_range();
    for parent in expr.ancestors().skip(1) {
        match parent.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => return Ok(true),
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                previous = parent.text_trimmed_range();
                continue;
            }
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let seq_expr = JsSequenceExpression::unwrap_cast(parent);
                // If the expression is not the rightmost node in a comma sequence
                if seq_expr.left()?.range() == previous {
                    return Ok(true);
                }
                previous = seq_expr.range();
                continue;
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                let for_stmt = JsForStatement::unwrap_cast(parent);
                if let Some(for_test) = for_stmt.test() {
                    return Ok(for_test.range() != previous);
                }
                return Ok(true);
            }
            _ => break,
        }
    }
    Ok(false)
}
