use crate::{
    frameworks::is_framework_api_reference,
    globals::{is_js_node_global, is_js_web_global},
    services::database::ResolvedImports,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, AnyJsIdentifierReference,
    AnyJsMemberExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectBindingPatternMember,
    AnyJsObjectMemberName, AnyTsType, JsAssignmentExpression, JsBinaryOperator,
    JsConditionalExpression, JsFunctionBody, JsIfStatement, JsLogicalExpression,
    JsLogicalOperator, JsObjectAssignmentPattern, JsPropertyClassMember, JsSyntaxKind,
    JsSyntaxNode, JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator, TsDeclareStatement,
    TsExportDeclareClause, is_transparent_expression_wrapper, static_value::StaticValue,
};
use biome_languages::JsFileSource;
use biome_module_graph::server_rendered_component_importer;
use biome_package::PackageJson;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_top_level_browser_globals::NoTopLevelBrowserGlobalsOptions;
use camino::Utf8PathBuf;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow unguarded browser globals at the top level of code that runs during server-side rendering.
    ///
    /// Browser globals such as `window`, `document`, and `localStorage` may be unavailable
    /// during server-side rendering. Move their use into `onMounted` in Vue or `onMount`
    /// in Svelte, or check that the browser environment is available first.
    ///
    /// This rule checks Vue scripts, Svelte scripts, Astro frontmatter, and `.svelte.js` and
    /// `.svelte.ts` modules. It also checks JavaScript and TypeScript modules that these
    /// components import statically, directly or through other modules, because the server
    /// evaluates those modules when it renders the component. Dynamic imports, type-only
    /// imports, and imports from Astro `<script>` tags aren't followed.
    ///
    /// It excludes template expressions, function bodies, parameter default values,
    /// instance field initializers, and type annotations.
    /// It does not follow function calls, including immediately invoked functions.
    /// Globals shared with Node.js, such as `console` and `fetch`, are allowed.
    ///
    /// Supported guards include `typeof` checks, SvelteKit's `browser`, `BROWSER` from
    /// `esm-env`, and Vite's `import.meta.env.SSR`. In projects that depend on Nuxt,
    /// `import.meta.client` and `import.meta.server` are supported as well.
    /// Guards apply to `if` branches, conditional expressions, and short-circuit expressions,
    /// and may be compared with `true` or `false`.
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
        domains: &[RuleDomain::Vue, RuleDomain::Svelte, RuleDomain::Astro, RuleDomain::Project],
        sources: &[RuleSource::EslintSvelte("no-top-level-browser-globals").inspired()],
    }
}

pub struct BrowserGlobal {
    name: Box<str>,
    range: TextRange,
    /// The component that imports the module, when the module isn't a component itself.
    component: Option<Utf8PathBuf>,
}

impl Rule for NoTopLevelBrowserGlobals {
    type Query = ResolvedImports<AnyJsIdentifierReference>;
    type State = BrowserGlobal;
    type Signals = Vec<Self::State>;
    type Options = NoTopLevelBrowserGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source = ctx.source_type::<JsFileSource>();
        let embedding = source.as_embedding_kind();
        let is_component = embedding.is_vue() || embedding.is_svelte() || embedding.is_astro();
        if is_component && !(source.is_embedded_source() || source.is_svelte_source_module()) {
            return Vec::new();
        }
        let Some(model) = ctx.get_service::<SemanticModel>() else {
            return Vec::new();
        };

        let reference = ctx.query();
        if model.binding(reference).is_some() || reference.is_only_type() {
            return Vec::new();
        }
        let Some((node, names)) = accessed_globals(reference) else {
            return Vec::new();
        };
        let nuxt = is_nuxt_project(ctx);
        let mut globals = names
            .into_iter()
            .filter(|(name, _)| {
                is_js_web_global(name.text())
                    && !is_js_node_global(name.text())
                    && runs_on_server(&node, name.text(), model, nuxt)
            })
            .map(|(name, range)| BrowserGlobal {
                name: name.text().into(),
                range,
                component: None,
            })
            .collect::<Vec<_>>();
        if globals.is_empty() || is_component {
            return globals;
        }

        // Other modules only run on the server when a server-rendered component imports them.
        let db = ctx.db();
        let component = db
            .module_for_path(ctx.file_path())
            .filter(|module| module.kind(db).as_js_module_info().is_some())
            .and_then(|module| server_rendered_component_importer(db, module));
        let Some(component) = component else {
            return Vec::new();
        };
        for global in &mut globals {
            global.component = Some(component.clone());
        }
        globals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! { "Unexpected top-level browser global "<Emphasis>{state.name.as_ref()}</Emphasis>"." },
        )
        .note("Browser globals may be unavailable on the server. Accessing them can throw an error and prevent the page from rendering.");
        if let Some(component) = &state.component {
            let cwd = std::env::current_dir()
                .ok()
                .and_then(|cwd| Utf8PathBuf::from_path_buf(cwd).ok())
                .unwrap_or_default();
            let component = component.strip_prefix(&cwd).unwrap_or(component);
            diagnostic = diagnostic.note(markup! {
                "This module runs during server-side rendering because "<Info>{component.as_str()}</Info>" imports it."
            });
        }
        Some(diagnostic.note("Move this access into onMounted in Vue or onMount in Svelte, or guard it with a browser availability check."))
    }
}

declare_node_union! {
    AnyExcludedContext = AnyJsFunction
        | AnyTsType
        | JsFunctionBody
        | TsDeclareStatement
        | TsExportDeclareClause
}

/// Returns the node whose evaluation accesses globals through `reference`, along with the
/// names and ranges of the accessed globals.
///
/// A reference to `globalThis` accesses the globals named by a member expression or by an
/// object pattern that destructures it.
fn accessed_globals(
    reference: &AnyJsIdentifierReference,
) -> Option<(JsSyntaxNode, Vec<(StaticValue, TextRange)>)> {
    let token = reference.value_token().ok()?;
    let mut node = reference.syntax().clone();
    if let Some(parent) = node.parent()
        && parent.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
    {
        node = parent;
    }
    if token.text_trimmed() != "globalThis" {
        let range = token.text_trimmed_range();
        return Some((node, vec![(StaticValue::String(token), range)]));
    }

    let node = AnyJsExpression::cast(node)?.outer_expression()?.into_syntax();
    let parent = node.parent()?;
    if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
        if member.object().ok()?.syntax() != &node {
            return None;
        }
        let name = member.member_name()?;
        let range = name.range();
        return Some((parent, vec![(name, range)]));
    }
    // `const { document } = globalThis` and `({ document } = globalThis)`
    let names = if let Some(declarator) = parent.parent().and_then(JsVariableDeclarator::cast) {
        let AnyJsBindingPattern::JsObjectBindingPattern(pattern) = declarator.id().ok()? else {
            return None;
        };
        pattern
            .properties()
            .into_iter()
            .filter_map(|property| match property.ok()? {
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                    member_name(&property.member().ok()?)
                }
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                    property,
                ) => {
                    let token = property
                        .identifier()
                        .ok()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok()?;
                    let range = token.text_trimmed_range();
                    Some((StaticValue::String(token), range))
                }
                _ => None,
            })
            .collect()
    } else if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent)
        && assignment.right().ok()?.syntax() == &node
    {
        let pattern = JsObjectAssignmentPattern::cast(assignment.left().ok()?.into_syntax())?;
        pattern
            .properties()
            .into_iter()
            .filter_map(|property| match property.ok()? {
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
                    member_name(&property.member().ok()?)
                }
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                    property,
                ) => {
                    let token = property.identifier().ok()?.name_token().ok()?;
                    let range = token.text_trimmed_range();
                    Some((StaticValue::String(token), range))
                }
                _ => None,
            })
            .collect()
    } else {
        return None;
    };
    Some((node, names))
}

fn member_name(member: &AnyJsObjectMemberName) -> Option<(StaticValue, TextRange)> {
    let name = match member {
        AnyJsObjectMemberName::JsLiteralMemberName(name) => StaticValue::String(name.value().ok()?),
        AnyJsObjectMemberName::JsComputedMemberName(name) => {
            name.expression().ok()?.omit_parentheses().as_static_value()?
        }
        AnyJsObjectMemberName::JsMetavariable(_) => return None,
    };
    let range = name.range();
    Some((name, range))
}

/// Checks whether evaluating `node` accesses `global` when the module loads, outside of
/// functions and browser environment guards.
fn runs_on_server(node: &JsSyntaxNode, global: &str, model: &SemanticModel, nuxt: bool) -> bool {
    let mut child = node.clone();
    let mut is_typeof_operand = true;
    let mut is_parameter_decorator = false;
    for parent in node.ancestors().skip(1) {
        match parent.kind() {
            JsSyntaxKind::JS_FORMAL_PARAMETER
            | JsSyntaxKind::JS_REST_PARAMETER
            | JsSyntaxKind::TS_PROPERTY_PARAMETER => {
                is_parameter_decorator = child.kind() == JsSyntaxKind::JS_DECORATOR_LIST;
            }
            // Parameter decorators run when the class is defined, but default values only run
            // when the function is called.
            JsSyntaxKind::JS_PARAMETERS | JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS
                if !is_parameter_decorator =>
            {
                return false;
            }
            _ => {}
        }
        if AnyExcludedContext::can_cast(parent.kind()) {
            return false;
        }
        if let Some(field) = JsPropertyClassMember::cast_ref(&parent)
            && field.value().is_some_and(|value| value.syntax() == &child)
            && !field
                .modifiers()
                .into_iter()
                .any(|modifier| modifier.as_js_static_modifier().is_some())
        {
            return false;
        }
        if is_typeof_operand
            && let Some(unary) = JsUnaryExpression::cast_ref(&parent)
            && unary
                .operator()
                .is_ok_and(|operator| operator == JsUnaryOperator::Typeof)
        {
            return false;
        }
        // A sequence evaluates its operands before `typeof`, so an undeclared name still throws.
        is_typeof_operand &= is_transparent_expression_wrapper(&parent)
            && parent.kind() != JsSyntaxKind::JS_SEQUENCE_EXPRESSION;
        if let Some((test, truthy)) = guard(&parent, &child)
            && guarantees_browser(&test, truthy, global, model, nuxt, 32)
        {
            return false;
        }
        child = parent;
    }
    true
}

/// Returns the condition that must hold for `parent` to evaluate `child`, and whether it
/// must be truthy or falsy.
fn guard(parent: &JsSyntaxNode, child: &JsSyntaxNode) -> Option<(AnyJsExpression, bool)> {
    if let Some(statement) = JsIfStatement::cast_ref(parent) {
        let test = statement.test().ok()?;
        if test.syntax() == child {
            return None;
        }
        let truthy = statement.consequent().ok()?.syntax() == child;
        Some((test, truthy))
    } else if let Some(conditional) = JsConditionalExpression::cast_ref(parent) {
        let test = conditional.test().ok()?;
        if test.syntax() == child {
            return None;
        }
        let truthy = conditional.consequent().ok()?.syntax() == child;
        Some((test, truthy))
    } else if let Some(logical) = JsLogicalExpression::cast_ref(parent) {
        if logical.right().ok()?.syntax() != child {
            return None;
        }
        let truthy = match logical.operator().ok()? {
            JsLogicalOperator::LogicalAnd => true,
            JsLogicalOperator::LogicalOr => false,
            JsLogicalOperator::NullishCoalescing => return None,
        };
        Some((logical.left().ok()?, truthy))
    } else {
        None
    }
}

fn is_nuxt_project(ctx: &RuleContext<NoTopLevelBrowserGlobals>) -> bool {
    ctx.get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()
        .and_then(Option::as_ref)
        .is_some_and(|(_, manifest)| manifest.contains_dependency("nuxt"))
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
    nuxt: bool,
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
            guarantees_browser(&argument, !truthy, global, model, nuxt, depth - 1)
        });
    }
    if let AnyJsExpression::JsLogicalExpression(logical) = &expression {
        let (Ok(left), Ok(right), Ok(operator)) =
            (logical.left(), logical.right(), logical.operator())
        else {
            return false;
        };
        let left = guarantees_browser(&left, truthy, global, model, nuxt, depth - 1);
        let right = guarantees_browser(&right, truthy, global, model, nuxt, depth - 1);
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
        let (equal, strict) = match operator {
            JsBinaryOperator::Equality => (truthy, false),
            JsBinaryOperator::StrictEquality => (truthy, true),
            JsBinaryOperator::Inequality => (!truthy, false),
            JsBinaryOperator::StrictInequality => (!truthy, true),
            _ => return false,
        };
        if typeof_guard(&left, &right, equal, global, model)
            || typeof_guard(&right, &left, equal, global, model)
        {
            return true;
        }
        return strict
            && (boolean_guard(&left, &right, equal, global, model, nuxt, depth)
                || boolean_guard(&right, &left, equal, global, model, nuxt, depth));
    }
    // SvelteKit's `browser` export identifies client-side execution.
    // `esm-env` provides the equivalent `BROWSER` flag through conditional exports.
    if is_framework_api_reference(&expression, model, "browser", &["$app/environment"], None)
        || is_framework_api_reference(&expression, model, "BROWSER", &["esm-env"], None)
    {
        return truthy;
    }
    meta_browser_guard(&expression, nuxt).is_some_and(|browser| browser == truthy)
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

/// Checks a strict comparison of `operand` with a boolean literal, such as
/// `import.meta.env.SSR === false`.
fn boolean_guard(
    operand: &AnyJsExpression,
    value: &AnyJsExpression,
    equal: bool,
    global: &str,
    model: &SemanticModel,
    nuxt: bool,
    depth: u8,
) -> bool {
    let Some(StaticValue::Boolean(value)) = value.clone().omit_parentheses().as_static_value()
    else {
        return false;
    };
    // `a && b` may produce a non-boolean falsy value, so `(a && b) !== false` doesn't show that
    // it's truthy. The other supported conditions produce booleans.
    if !equal
        && matches!(
            operand.clone().omit_parentheses(),
            AnyJsExpression::JsLogicalExpression(_)
        )
    {
        return false;
    }
    let truthy = (value.text_trimmed() == "true") == equal;
    guarantees_browser(operand, truthy, global, model, nuxt, depth - 1)
}

fn meta_browser_guard(expression: &AnyJsExpression, nuxt: bool) -> Option<bool> {
    let member = AnyJsMemberExpression::cast_ref(expression.syntax())?;
    let name = member.member_name()?;
    let object = member.object().ok()?.omit_parentheses();
    // Nuxt injects `import.meta.client` and `import.meta.server`. Without Nuxt, they're
    // `undefined`, so `!import.meta.server` would also hold on the server.
    if matches!(object, AnyJsExpression::JsImportMetaExpression(_)) {
        return match name.text() {
            "client" if nuxt => Some(true),
            "server" if nuxt => Some(false),
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
