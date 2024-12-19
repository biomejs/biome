use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, JsUnaryOperator,
    TsEnumDeclaration,
};
use biome_rowan::{AstNode, TextRange};
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Require all enum members to be literal values.
    ///
    /// Usually, an enum member is initialized with a literal number or a literal string.
    /// However, _TypeScript_ allows the value of an enum member to be many different kinds of expressions.
    /// Using a computed enum member is often error-prone and confusing.
    /// This rule requires the initialization of enum members with constant expressions.
    /// It allows numeric and bitwise expressions for supporting [enum flags](https://stackoverflow.com/questions/39359740/what-are-enum-flags-in-typescript/39359953#39359953).
    /// It also allows referencing previous enum members.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// const x = 2;
    /// enum Computed {
    ///     A,
    ///     B = x,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Direction {
    ///     Left,
    ///     Right,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum Order {
    ///     Less = -1,
    ///     Equal = 0,
    ///     Greater = 1,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum State {
    ///     Open = "Open",
    ///     Close = "Close",
    /// }
    /// ```
    ///
    /// ```ts
    /// enum FileAccess {
    ///     None = 0,
    ///     Read = 1,
    ///     Write = 1 << 1,
    ///     All = Read | Write
    /// }
    /// ```
    pub UseLiteralEnumMembers {
        version: "1.0.0",
        name: "useLiteralEnumMembers",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-literal-enum-member")],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for UseLiteralEnumMembers {
    type Query = Ast<TsEnumDeclaration>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_declaration = ctx.query();
        let mut result = Vec::new();
        let mut enum_member_names = FxHashSet::default();
        let Ok(enum_name) = enum_declaration.id() else {
            return result.into_boxed_slice();
        };
        let Some(enum_name) = enum_name
            .as_js_identifier_binding()
            .and_then(|x| x.name_token().ok())
        else {
            return result.into_boxed_slice();
        };
        let enum_name = enum_name.text_trimmed();
        for enum_member in enum_declaration.members() {
            let Ok(enum_member) = enum_member else {
                continue;
            };
            // no initializer => sequentially assigned literal integer
            if let Some(initializer) = enum_member.initializer() {
                if let Ok(initializer) = initializer.expression() {
                    let range = initializer.range();
                    if !is_constant_enum_expression(initializer, enum_name, &enum_member_names) {
                        result.push(range);
                    }
                }
            };
            if let Ok(name) = enum_member.name() {
                if let Some(name) = name.name() {
                    enum_member_names.insert(name.to_string());
                }
            }
        }
        result.into_boxed_slice()
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        initializer_range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            initializer_range,
            markup! {
                "The enum member should be initialized with a literal value such as a number or a string."
            },
        ))
    }
}

/// Returns true if `expr` is a constant enum expression.
/// A constant enum expression can contain numbers, string literals, and reference to
/// one of the enum member of `enum_member_names` of the enum name `enum_name`.
/// These values can be combined thanks to numeric, bitwise, and concatenation operations.
fn is_constant_enum_expression(
    expr: AnyJsExpression,
    enum_name: &str,
    enum_member_names: &FxHashSet<String>,
) -> bool {
    (move || {
        // stack that holds expressions to validate.
        let mut stack = Vec::new();
        stack.push(expr);
        while let Some(expr) = stack.pop() {
            match expr.omit_parentheses() {
                AnyJsExpression::AnyJsLiteralExpression(expr) => {
                    if !matches!(
                        expr,
                        AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                            | AnyJsLiteralExpression::JsStringLiteralExpression(_)
                    ) {
                        return Some(false);
                    }
                }
                AnyJsExpression::JsTemplateExpression(expr) => {
                    if !expr.is_constant() {
                        return Some(false);
                    }
                }
                AnyJsExpression::JsUnaryExpression(expr) => {
                    if !matches!(
                        expr.operator(),
                        Ok(JsUnaryOperator::BitwiseNot
                            | JsUnaryOperator::Minus
                            | JsUnaryOperator::Plus)
                    ) {
                        return Some(false);
                    }
                    stack.push(expr.argument().ok()?)
                }
                AnyJsExpression::JsBinaryExpression(expr) => {
                    if !expr.is_binary_operation() && !expr.is_numeric_operation() {
                        return Some(false);
                    }
                    stack.push(expr.left().ok()?);
                    stack.push(expr.right().ok()?);
                }
                AnyJsExpression::JsIdentifierExpression(expr) => {
                    // Allow reference to previous member name
                    let name = expr.name().ok()?;
                    if !enum_member_names.contains(name.value_token().ok()?.text_trimmed()) {
                        return Some(false);
                    }
                }
                AnyJsExpression::JsStaticMemberExpression(expr) => {
                    if !is_enum_member_reference(expr.into(), enum_name, enum_member_names) {
                        return Some(false);
                    }
                }
                AnyJsExpression::JsComputedMemberExpression(expr) => {
                    if !is_enum_member_reference(expr.into(), enum_name, enum_member_names) {
                        return Some(false);
                    }
                }
                _ => {
                    return Some(false);
                }
            }
        }
        Some(true)
    })()
    .unwrap_or_default()
}

// Return true if `expr` is a reference to one of the enum member `enum_member_names`
// of the enum named `enum_name`.
fn is_enum_member_reference(
    expr: AnyJsMemberExpression,
    enum_name: &str,
    enum_member_names: &FxHashSet<String>,
) -> bool {
    (move || {
        // Allow reference to previous member name namespaced by the enum name
        let object = expr.object().ok()?.omit_parentheses();
        let object = object.as_js_reference_identifier()?;
        Some(object.has_name(enum_name) && enum_member_names.contains(expr.member_name()?.text()))
    })()
    .unwrap_or_default()
}
