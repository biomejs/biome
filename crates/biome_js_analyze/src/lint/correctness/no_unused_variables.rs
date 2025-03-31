use crate::JsRuleAction;
use crate::{services::semantic::Semantic, utils::rename::RenameSymbolExtensions};
use biome_analyze::RuleSource;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::{ReferencesExtensions, SemanticModel};
use biome_js_syntax::binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding};
use biome_js_syntax::declaration_ext::is_in_ambient_context;
use biome_js_syntax::{
    AnyJsExpression, JsClassExpression, JsFileSource, JsForStatement, JsFunctionExpression,
    JsIdentifierExpression, JsModuleItemList, JsSequenceExpression, JsSyntaxKind, JsSyntaxNode,
    TsConditionalType, TsDeclarationModule, TsInferType,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction, SyntaxResult};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    Deserialize,
    biome_deserialize_macros::Deserializable,
    Eq,
    PartialEq,
    Serialize,
    Default,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoUnusedVariablesOptions {
    /// Whether to ignore unused variables from an object destructuring with a spread
    /// (i.e.: whether `a` and `b` in `const { a, b, ...rest } = obj` should be ignored by this rule).
    #[serde(default)]
    ignore_rest_siblings: bool,
}

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
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreRestSiblings": true
    ///   }
    /// }
    /// ```
    ///
    /// - `ignoreRestSiblings`: Whether to ignore unused variables from an object destructuring with a spread (i.e.: whether `a` and `b` in `const { a, b, ...rest } = obj` should be ignored by this rule). Defaults to `false`.
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
    /// ```js,expect_diagnostic
    /// // With `ignoreRestSiblings: false`
    /// const car = { brand: "Tesla", year: 2019, countryCode: "US" };
    /// const { brand, ...other } = car;
    /// console.log(other);
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
    ///
    /// ```js
    /// // With `ignoreRestSiblings: false`
    /// const car = { brand: "Tesla", year: 2019, countryCode: "US" };
    /// const { brand: _brand, ...other } = car;
    /// console.log(other);
    /// ```
    ///
    /// ```js,use_options
    /// // With `ignoreRestSiblings: true`
    /// const car = { brand: "Tesla", year: 2019, countryCode: "US" };
    /// const { brand, ...other } = car;
    /// console.log(other);
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

/// Returns `true` if the binding is part of an object pattern with a rest element as a sibling
fn is_rest_spread_sibling(decl: &AnyJsBindingDeclaration) -> bool {
    if let node @ (AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
    | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)) = decl
    {
        node.syntax()
            .siblings(Direction::Next)
            .last()
            .is_some_and(|last_sibling| {
                matches!(
                    last_sibling.kind(),
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST
                )
            })
    } else {
        false
    }
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
fn suggested_fix_if_unused(
    binding: &AnyJsIdentifierBinding,
    options: &NoUnusedVariablesOptions,
) -> Option<SuggestedFix> {
    let decl = binding.declaration()?;
    // It is fine to ignore unused rest spread siblings if the option is enabled
    if options.ignore_rest_siblings && is_rest_spread_sibling(&decl) {
        return None;
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
        | AnyJsBindingDeclaration::JsFormalParameter(_)
        | AnyJsBindingDeclaration::JsRestParameter(_)
        | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => {
            None
        }
    }
}

impl Rule for NoUnusedVariables {
    type Query = Semantic<AnyJsIdentifierBinding>;
    type State = SuggestedFix;
    type Signals = Option<Self::State>;
    type Options = NoUnusedVariablesOptions;

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
                // All top-level types and variables are available in every files of the project.
                // Thus, it is ok if top-level types are not used locally.
                let is_top_level = items.parent::<TsDeclarationModule>().is_some();
                if is_top_level && items.into_iter().all(|x| x.as_any_js_statement().is_some()) {
                    return None;
                }
            }
        }

        // Ignore name prefixed with `_`
        let is_underscore_prefixed = binding.name_token().ok()?.text_trimmed().starts_with('_');
        if !is_underscore_prefixed && is_unused(model, binding) {
            suggested_fix_if_unused(binding, ctx.options())
        } else {
            None
        }
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

        let mut diag = diag.note(
            markup! {"Unused variables usually are result of incomplete refactoring, typos and other source of bugs."},
        );

        // Check if this binding is part of an object pattern with a rest element
        if let Some(decl) = binding.declaration() {
            if is_rest_spread_sibling(&decl) {
                diag = diag.note(
                    markup! {"You can use the 'ignoreRestSiblings' option to ignore unused variables in an object destructuring with a spread."},
                );
            }
        }

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

/// Returns `true` if `binding` is considered as unused.
pub fn is_unused(model: &SemanticModel, binding: &AnyJsIdentifierBinding) -> bool {
    if matches!(binding, AnyJsIdentifierBinding::TsLiteralEnumMemberName(_)) {
        // Enum members can be unused.
        return false;
    }

    // Ignore expressions
    if binding.parent::<JsFunctionExpression>().is_some()
        || binding.parent::<JsClassExpression>().is_some()
    {
        return false;
    }

    if model.is_exported(binding) {
        return false;
    }

    // We need to check if all uses of this binding are somehow recursive or unused
    let Some(declaration) = binding.declaration() else {
        return false;
    };
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
