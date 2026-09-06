use crate::{
    frameworks::is_framework_api_reference,
    globals::{is_js_node_global, is_js_web_global},
    services::semantic::Semantic,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsIdentifierReference, AnyJsMemberExpression, AnyTsType,
    JsBinaryOperator, JsConditionalExpression, JsFunctionBody, JsIfStatement, JsLogicalExpression,
    JsLogicalOperator, JsParameters, JsPropertyClassMember, JsSyntaxKind, JsUnaryExpression,
    JsUnaryOperator, TsDeclareStatement, TsExportDeclareClause, is_transparent_expression_wrapper,
    static_value::StaticValue,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_top_level_browser_globals::NoTopLevelBrowserGlobalsOptions;

declare_lint_rule! {
    /// Disallow unguarded browser globals at the top level of Vue and Svelte scripts.
    ///
    /// Browser globals such as `window`, `document`, and `localStorage` may be unavailable
    /// during server-side rendering. Move their use into `onMounted` in Vue or `onMount`
    /// in Svelte, or check that the browser environment is available first.
    ///
    /// This rule checks Vue scripts, Svelte scripts, and `.svelte.js` and `.svelte.ts` modules.
    /// It excludes template expressions, function bodies, instance field initializers, and type annotations.
    /// It does not follow function calls, including immediately invoked functions.
    /// Globals shared with Node.js, such as `console` and `fetch`, are allowed.
    ///
    /// Supported guards include `typeof` checks, SvelteKit's `browser`, `BROWSER` from
    /// `esm-env`, Vite's `import.meta.env.SSR`, and Nuxt's `import.meta.client` and
    /// `import.meta.server`. Guards apply to `if` branches, conditional expressions,
    /// and short-circuit expressions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const width = window.innerWidth;
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// const theme = localStorage.getItem("theme");
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { onMounted } from "vue";
    /// onMounted(() => console.log(window.innerWidth));
    /// </script>
    /// ```
    ///
    /// ```svelte
    /// <script>
    /// import { browser } from "$app/environment";
    /// if (browser) {
    ///     console.log(localStorage.getItem("theme"));
    /// }
    /// </script>
    /// ```
    ///
    pub NoTopLevelBrowserGlobals {
        version: "next",
        name: "noTopLevelBrowserGlobals",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue, RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-top-level-browser-globals").inspired()],
    }
}

impl Rule for NoTopLevelBrowserGlobals {
    type Query = Semantic<AnyJsIdentifierReference>;
    type State = StaticValue;
    type Signals = Option<Self::State>;
    type Options = NoTopLevelBrowserGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source = ctx.source_type::<JsFileSource>();
        let embedding = source.as_embedding_kind();
        if !(embedding.is_vue() || embedding.is_svelte())
            || !(source.is_embedded_source() || source.is_svelte_source_module())
        {
            return None;
        }

        let reference = ctx.query();
        if ctx.model().binding(reference).is_some() || reference.is_only_type() {
            return None;
        }
        let token = reference.value_token().ok()?;
        let mut node = reference.syntax().clone();
        if let Some(parent) = node.parent()
            && parent.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
        {
            node = parent;
        }
        let name = if token.text_trimmed() == "globalThis" {
            node = AnyJsExpression::cast(node)?
                .outer_expression()?
                .into_syntax();
            let member = AnyJsMemberExpression::cast(node.parent()?)?;
            if member.object().ok()?.syntax() != &node {
                return None;
            }
            node = member.syntax().clone();
            member.member_name()?
        } else {
            StaticValue::String(token)
        };
        if !is_js_web_global(name.text()) || is_js_node_global(name.text()) {
            return None;
        }
        let mut child = node.clone();
        let mut is_typeof_operand = true;
        for parent in node.ancestors().skip(1) {
            if AnyExcludedContext::can_cast(parent.kind()) {
                return None;
            }
            if let Some(field) = JsPropertyClassMember::cast_ref(&parent)
                && field.value().is_some_and(|value| value.syntax() == &child)
                && !field
                    .modifiers()
                    .into_iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some())
            {
                return None;
            }
            if is_typeof_operand
                && let Some(unary) = JsUnaryExpression::cast_ref(&parent)
                && unary.operator().ok()? == JsUnaryOperator::Typeof
            {
                return None;
            }
            // A sequence evaluates its operands before `typeof`, so an undeclared name still throws.
            is_typeof_operand &= is_transparent_expression_wrapper(&parent)
                && parent.kind() != JsSyntaxKind::JS_SEQUENCE_EXPRESSION;
            let guard = if let Some(statement) = JsIfStatement::cast_ref(&parent) {
                if statement.test().ok()?.syntax() == &child {
                    child = parent;
                    continue;
                }
                let truthy = statement.consequent().ok()?.syntax() == &child;
                (statement.test().ok()?, truthy)
            } else if let Some(conditional) = JsConditionalExpression::cast_ref(&parent) {
                if conditional.test().ok()?.syntax() == &child {
                    child = parent;
                    continue;
                }
                (
                    conditional.test().ok()?,
                    conditional.consequent().ok()?.syntax() == &child,
                )
            } else if let Some(logical) = JsLogicalExpression::cast_ref(&parent) {
                if logical.right().ok()?.syntax() != &child {
                    child = parent;
                    continue;
                }
                let truthy = match logical.operator().ok()? {
                    JsLogicalOperator::LogicalAnd => true,
                    JsLogicalOperator::LogicalOr => false,
                    JsLogicalOperator::NullishCoalescing => {
                        child = parent;
                        continue;
                    }
                };
                (logical.left().ok()?, truthy)
            } else {
                child = parent;
                continue;
            };
            if guarantees_browser(
                &guard.0,
                guard.1,
                name.text(),
                ctx.model(),
                embedding.is_vue(),
                32,
            ) {
                return None;
            }
            child = parent;
        }
        Some(name)
    }

    fn diagnostic(_: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            name.range(),
            markup! { "Unexpected top-level browser global "<Emphasis>{name.text()}</Emphasis>"." },
        )
        .note("Browser globals may be unavailable on the server. Accessing them can throw an error and prevent the page from rendering.")
        .note("Move this access into onMounted in Vue or onMount in Svelte, or guard it with a browser availability check."))
    }
}

declare_node_union! {
    AnyExcludedContext = AnyJsFunction
        | AnyTsType
        | JsFunctionBody
        | JsParameters
        | TsDeclareStatement
        | TsExportDeclareClause
}

fn browser_global(expression: &AnyJsExpression, model: &SemanticModel) -> Option<StaticValue> {
    let expression = expression.inner_expression()?;
    let name = if let Some(reference) = expression.as_js_reference_identifier() {
        if model.binding(&reference).is_some() {
            return None;
        }
        StaticValue::String(reference.value_token().ok()?)
    } else {
        let member = AnyJsMemberExpression::cast_ref(expression.syntax())?;
        let object = member.object().ok()?.inner_expression()?;
        let reference = object.as_js_reference_identifier()?;
        if !reference.has_name("globalThis") || model.binding(&reference).is_some() {
            return None;
        }
        member.member_name()?
    };
    (is_js_web_global(name.text()) && !is_js_node_global(name.text())).then_some(name)
}

/// Checks whether a condition being truthy or falsy, as selected by `truthy`, establishes
/// a browser environment or the availability of `global`. Unsupported conditions and
/// exhausted recursion budgets return `false`.
fn guarantees_browser(
    expression: &AnyJsExpression,
    truthy: bool,
    global: &str,
    model: &SemanticModel,
    vue: bool,
    depth: u8,
) -> bool {
    if depth == 0 {
        return false;
    }
    let expression = expression.clone().omit_parentheses();
    if let AnyJsExpression::JsUnaryExpression(unary) = &expression
        && unary
            .operator()
            .is_ok_and(|op| op == JsUnaryOperator::LogicalNot)
    {
        return unary.argument().is_ok_and(|argument| {
            guarantees_browser(&argument, !truthy, global, model, vue, depth - 1)
        });
    }
    if let AnyJsExpression::JsLogicalExpression(logical) = &expression {
        let (Ok(left), Ok(right), Ok(operator)) =
            (logical.left(), logical.right(), logical.operator())
        else {
            return false;
        };
        let left = guarantees_browser(&left, truthy, global, model, vue, depth - 1);
        let right = guarantees_browser(&right, truthy, global, model, vue, depth - 1);
        return match (operator, truthy) {
            (JsLogicalOperator::LogicalAnd, true) | (JsLogicalOperator::LogicalOr, false) => {
                left || right
            }
            (JsLogicalOperator::LogicalOr, true) | (JsLogicalOperator::LogicalAnd, false) => {
                left && right
            }
            _ => false,
        };
    }
    if let AnyJsExpression::JsBinaryExpression(binary) = &expression {
        let (Ok(left), Ok(right), Ok(operator)) =
            (binary.left(), binary.right(), binary.operator())
        else {
            return false;
        };
        let equal = match operator {
            JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality => truthy,
            JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality => !truthy,
            _ => return false,
        };
        return typeof_guard(&left, &right, equal, global, model)
            || typeof_guard(&right, &left, equal, global, model);
    }
    // SvelteKit's `browser` export identifies client-side execution.
    // `esm-env` provides the equivalent `BROWSER` flag through conditional exports.
    if is_framework_api_reference(&expression, model, "browser", &["$app/environment"], None)
        || is_framework_api_reference(&expression, model, "BROWSER", &["esm-env"], None)
    {
        return truthy;
    }
    meta_browser_guard(&expression, vue).is_some_and(|browser| browser == truthy)
}

fn typeof_guard(
    operand: &AnyJsExpression,
    value: &AnyJsExpression,
    equal: bool,
    global: &str,
    model: &SemanticModel,
) -> bool {
    let operand = operand.clone().omit_parentheses();
    let AnyJsExpression::JsUnaryExpression(unary) = operand else {
        return false;
    };
    if !unary
        .operator()
        .is_ok_and(|operator| operator == JsUnaryOperator::Typeof)
    {
        return false;
    }
    let Some(name) = unary
        .argument()
        .ok()
        .and_then(|argument| browser_global(&argument, model))
    else {
        return false;
    };
    if name.text() != global && !matches!(name.text(), "window" | "document") {
        return false;
    }
    let Some(value) = value.clone().omit_parentheses().as_static_value() else {
        return false;
    };
    match value.as_string_constant() {
        Some("undefined") => !equal,
        Some("object" | "function" | "string" | "number" | "boolean" | "symbol" | "bigint") => {
            equal
        }
        _ => false,
    }
}

fn meta_browser_guard(expression: &AnyJsExpression, vue: bool) -> Option<bool> {
    let member = AnyJsMemberExpression::cast_ref(expression.syntax())?;
    let name = member.member_name()?;
    let object = member.object().ok()?.omit_parentheses();
    if vue && matches!(object, AnyJsExpression::JsImportMetaExpression(_)) {
        return match name.text() {
            "client" => Some(true),
            "server" => Some(false),
            _ => None,
        };
    }
    let object = AnyJsMemberExpression::cast_ref(object.syntax())?;
    (name.text() == "SSR"
        && object.member_name()?.text() == "env"
        && matches!(
            object.object().ok()?.omit_parentheses(),
            AnyJsExpression::JsImportMetaExpression(_)
        ))
    .then_some(false)
}
