use crate::JsRuleAction;
use crate::{services::semantic::Semantic, utils::rename::RenameSymbolExtensions};
use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::binding_ext::{
    AnyJsBindingDeclaration, AnyJsIdentifierBinding, AnyJsParameterParentFunction,
};
use biome_js_syntax::declaration_ext::is_in_ambient_context;
use biome_js_syntax::{
    AnyJsExpression, JsClassExpression, JsFileSource, JsForStatement, JsFunctionExpression,
    JsIdentifierExpression, JsModuleItemList, JsSequenceExpression, JsSyntaxKind, JsSyntaxNode,
    TsConditionalType, TsDeclarationModule, TsInferType,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction, SyntaxResult};

declare_lint_rule! {
    /// Disallow unused variables.
    ///
    /// There is an exception to this rule:
    /// variables that starts with underscore, e.g. `let _something;`.
    ///
    /// The pattern of having an underscore as prefix of a name of variable is a very diffuse
    /// pattern among programmers, and Biome decided to follow it.
    ///
    /// This rule won't report unused imports.
    /// If you want to report unused imports,
    /// enable [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/).
    ///
    /// :::caution
    /// From `v2.0.0`, the rule won't check unused function parameters any more.
    /// If you want to report unused function parameters,
    /// enable [noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters/).
    /// :::
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
    /// ### Valid
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
    /// ```ts
    /// function used_overloaded(): number;
    /// function used_overloaded(s: string): string;
    /// function used_overloaded(s?: string) {
    ///     return s;
    /// }
    /// used_overloaded();
    /// ```
    pub NoUnusedVariables {
        version: "1.0.0",
        name: "noUnusedVariables",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-unused-vars"),
            RuleSource::EslintTypeScript("no-unused-vars"),
            RuleSource::EslintUnusedImports("no-unused-vars")
        ],
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
    let decl = binding.declaration()?;
    // It is fine to ignore unused rest spread siblings
    if let node @ (AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
    | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)) = &decl
    {
        if node
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

    match decl.parent_binding_pattern_declaration().unwrap_or(decl) {
        // ok to not be used
        AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExpression(_)
        | AnyJsBindingDeclaration::JsFunctionExpression(_)
        | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
        | AnyJsBindingDeclaration::TsMappedType(_)
        | AnyJsBindingDeclaration::TsEnumMember(_) => None,

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
        AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
        | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_) => {
            None
        }
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
        | AnyJsBindingDeclaration::TsModuleDeclaration(_)) => {
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
        // exports with binding are ok to be unused
        | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
            Some(SuggestedFix::NoSuggestion)
        }
        // Imports are handled by `noUnusedImports`
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
        | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => {
            None
        }
    }
}

impl Rule for NoUnusedVariables {
    type Query = Semantic<AnyJsIdentifierBinding>;
    type State = SuggestedFix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let model = ctx.model();
        let is_declaration_file = ctx
            .source_type::<JsFileSource>()
            .language()
            .is_definition_file();
        if is_declaration_file {
            if let Some(items) = binding
                .syntax()
                .ancestors()
                .skip(1)
                .find_map(JsModuleItemList::cast)
            {
                // A declaration file without top-level exports and imports is a global declaration file.
                // All top-level types and variiables are available in every files of the project.
                // Thus, it is ok if top-level types are not used locally.
                let is_top_level = items.parent::<TsDeclarationModule>().is_some();
                if is_top_level && items.into_iter().all(|x| x.as_any_js_statement().is_some()) {
                    return None;
                }
            }
        }

        if matches!(binding, AnyJsIdentifierBinding::TsLiteralEnumMemberName(_)) {
            // Enum members can be unused.
            return None;
        }

        if binding.name_token().ok()?.text_trimmed().starts_with('_') {
            return None;
        }

        // Ignore expressions
        if binding.parent::<JsFunctionExpression>().is_some()
            || binding.parent::<JsClassExpression>().is_some()
        {
            return None;
        }

        let suggestion = suggested_fix_if_unused(binding)?;

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

        let symbol_type = match binding.syntax().parent()?.kind() {
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
                    AnyJsIdentifierBinding::TsLiteralEnumMemberName(_) => {
                        return None;
                    }
                };
                let name_trimmed = name.text_trimmed();
                let new_name = format!("_{name_trimmed}");

                let model = ctx.model();
                mutation.rename_node_declaration(model, binding, &new_name);

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
