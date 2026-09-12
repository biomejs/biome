use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsSwitchClause, JsArrowFunctionExpression,
    JsComputedMemberName, JsConstructorClassMember, JsExport, JsFunctionDeclaration,
    JsGetterClassMember, JsGetterObjectMember, JsInitializerClause, JsMethodClassMember,
    JsMethodModifierList, JsMethodObjectMember, JsPropertyClassMember, JsSetterClassMember,
    JsSetterObjectMember, JsStatementList, JsStaticInitializationBlockClassMember,
    JsSuperExpression, JsThisExpression, JsVariableDeclarator, TsDeclareFunctionDeclaration,
    unescape_js_string,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::use_consistent_function_style::{
    FunctionStyle, UseConsistentFunctionStyleOptions,
};

declare_lint_rule! {
    /// Enforce consistent use of function declarations or expressions assigned to variables.
    ///
    /// A consistent function style makes function definitions easier to recognize.
    /// Use the `style` option to choose the style that matches your project's conventions.
    ///
    /// In both styles, callbacks, methods, and default exports are ignored.
    /// Named exports follow the configured style.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function greet() {
    ///     return "Hello";
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const greet = function() {
    ///     return "Hello";
    /// };
    /// const farewell = () => "Goodbye";
    /// ```
    ///
    /// ```js
    /// export default function greet() {
    ///     return "Hello";
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Type: `"expression" | "declaration"`
    ///
    /// Default: `"expression"`
    ///
    /// With `"expression"`, the rule requires function expressions or arrow functions
    /// assigned to variables.
    ///
    /// These functions can only be called after the variable is initialized. TypeScript
    /// overloads are allowed because they require declarations.
    ///
    /// ```ts
    /// function identity(value: string): string;
    /// function identity(value: number): number;
    /// function identity(value: string | number) {
    ///     return value;
    /// }
    /// ```
    ///
    /// With `"declaration"`, the rule requires function declarations.
    ///
    /// These are hoisted and can be called before their definition. Arrow functions that directly use
    /// `this` or `super` are allowed to preserve their lexical binding.
    /// Variables with TypeScript type annotations are also allowed because a function
    /// declaration cannot be annotated with an existing function type.
    /// A return type annotation on the function does not qualify for this exception.
    ///
    /// The following examples use `"declaration"`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "declaration"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// const greet = function() {};
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// const greet = (): string => "Hello";
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// function greet() {}
    /// const getContext = () => this;
    /// ```
    ///
    /// ```ts,use_options
    /// type Greeting = () => string;
    /// const greet: Greeting = () => "Hello";
    /// const farewell: Greeting = function() { return "Goodbye"; };
    /// ```
    ///
    /// ### `allowArrowFunctions`
    ///
    /// Type: `boolean`
    ///
    /// Default: `false`
    ///
    /// Allow arrow functions regardless of style setting.
    /// Arrow functions are always allowed when expressions are required.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "declaration",
    ///         "allowArrowFunctions": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// const greet = function() { return "Hello"; };
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// function greet() { return "Hello"; }
    /// const farewell = () => "Goodbye";
    /// ```
    ///
    /// ## See Also
    ///
    /// - [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/)
    ///
    pub UseConsistentFunctionStyle {
        version: "2.5.14",
        name: "useConsistentFunctionStyle",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("func-style").inspired()],
    }
}

declare_node_union! {
    pub AnyFunctionStyle = JsFunctionDeclaration | JsVariableDeclarator
}

impl Rule for UseConsistentFunctionStyle {
    type Query = Ast<AnyFunctionStyle>;
    type State = (FunctionStyle, TextRange);
    type Signals = Option<Self::State>;
    type Options = UseConsistentFunctionStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let style = options.style();
        match ctx.query() {
            AnyFunctionStyle::JsFunctionDeclaration(function) => {
                if style == FunctionStyle::Declaration
                    || (ctx.source_type::<JsFileSource>().is_typescript()
                        && is_overloaded(function).unwrap_or(false))
                {
                    return None;
                }
                Some((FunctionStyle::Expression, function.id().ok()?.range()))
            }
            AnyFunctionStyle::JsVariableDeclarator(variable) => {
                if style == FunctionStyle::Expression || variable.variable_annotation().is_some() {
                    return None;
                }
                let mut expression = variable.initializer()?.expression().ok()?;
                loop {
                    expression = match expression {
                        AnyJsExpression::JsParenthesizedExpression(wrapper) => {
                            wrapper.expression().ok()?
                        }
                        AnyJsExpression::TsAsExpression(wrapper) => wrapper.expression().ok()?,
                        AnyJsExpression::TsSatisfiesExpression(wrapper) => {
                            wrapper.expression().ok()?
                        }
                        AnyJsExpression::TsNonNullAssertionExpression(wrapper) => {
                            wrapper.expression().ok()?
                        }
                        AnyJsExpression::TsTypeAssertionExpression(wrapper) => {
                            wrapper.expression().ok()?
                        }
                        AnyJsExpression::TsInstantiationExpression(wrapper) => {
                            wrapper.expression().ok()?
                        }
                        _ => break,
                    };
                }
                match expression {
                    AnyJsExpression::JsFunctionExpression(_) => {}
                    AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                        if options.allow_arrow_functions() || has_this_or_super(&arrow) {
                            return None;
                        }
                    }
                    _ => return None,
                }
                Some((FunctionStyle::Declaration, variable.id().ok()?.range()))
            }
        }
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (style, range): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let (message, advice) = match style {
            FunctionStyle::Expression => (
                "Function expressions are required here, but this is a declaration.",
                "Use a function expression assigned to a variable.",
            ),
            FunctionStyle::Declaration => (
                "Function declarations are required here, but this is an expression.",
                "Use a function declaration.",
            ),
        };
        Some(
            RuleDiagnostic::new(rule_category!(), range, message)
                .note("Using a consistent function style makes function definitions easier to recognize.")
                .note(advice),
        )
    }
}

/// Checks for a TypeScript overload signature with the same name in the surrounding scope.
/// Returns `None` if the function's name or surrounding syntax is unavailable.
fn is_overloaded(function: &JsFunctionDeclaration) -> Option<bool> {
    let name = function
        .id()
        .ok()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()?;
    let name = unescape_js_string(name.token_text_trimmed());
    let parent = function.syntax().parent()?;
    // Exported signatures and implementations each have a JsExport wrapper.
    // Search the enclosing module item list and unwrap exports before comparing signatures.
    let is_export = JsExport::can_cast(parent.kind());
    let list = if is_export { parent.parent()? } else { parent };
    let matches_signature = |sibling| {
        let declaration = TsDeclareFunctionDeclaration::cast(sibling)?;
        let id = declaration.id().ok()?;
        let token = id.as_js_identifier_binding()?.name_token().ok()?;
        Some(unescape_js_string(token.token_text_trimmed()) == name)
    };
    // All switch clauses share a scope, so a signature can occur in another clause.
    if let Some(clause) = list.parent().and_then(AnyJsSwitchClause::cast) {
        return Some(
            clause
                .syntax()
                .parent()?
                .children()
                .filter_map(AnyJsSwitchClause::cast)
                .any(|clause| {
                    clause.consequent().iter().any(|statement| {
                        matches_signature(statement.into_syntax()).unwrap_or(false)
                    })
                }),
        );
    }
    Some(list.children().any(|sibling| {
        let sibling = if is_export {
            let Some(export) = JsExport::cast(sibling) else {
                return false;
            };
            let Ok(clause) = export.export_clause() else {
                return false;
            };
            clause.into_syntax()
        } else {
            sibling
        };
        matches_signature(sibling).unwrap_or(false)
    }))
}

declare_node_union! {
    AnyJsMethod = JsMethodObjectMember | JsMethodClassMember | JsConstructorClassMember
        | JsGetterObjectMember | JsSetterObjectMember | JsGetterClassMember | JsSetterClassMember
}

declare_node_union! {
    AnyJsThisOrSuperExpression = JsThisExpression | JsSuperExpression
}

fn has_this_or_super(arrow: &JsArrowFunctionExpression) -> bool {
    let root = arrow.syntax();
    let mut preorder = root.preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        // Computed names and decorators are evaluated outside the method's function scope.
        let is_method_child = !JsComputedMemberName::can_cast(node.kind())
            && !JsMethodModifierList::can_cast(node.kind())
            && node
                .parent()
                .is_some_and(|parent| AnyJsMethod::can_cast(parent.kind()));
        // Field initializers and static blocks bind `this` and `super` to the class scope.
        let is_class_initializer = node.parent().is_some_and(|parent| {
            (JsInitializerClause::can_cast(node.kind())
                && JsPropertyClassMember::can_cast(parent.kind()))
                || (JsStatementList::can_cast(node.kind())
                    && JsStaticInitializationBlockClassMember::can_cast(parent.kind()))
        });
        if (node != *root && AnyJsFunction::can_cast(node.kind()))
            || is_method_child
            || is_class_initializer
        {
            preorder.skip_subtree();
        } else if AnyJsThisOrSuperExpression::can_cast(node.kind()) {
            return true;
        }
    }
    false
}
