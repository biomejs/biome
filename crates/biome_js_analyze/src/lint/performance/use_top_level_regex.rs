use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsLiteralExpression, AnyJsPropertyModifier,
    JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember, JsMethodObjectMember,
    JsPropertyClassMember, JsPropertyObjectMember, JsRegexLiteralExpression,
    JsSetterClassMember, JsSetterObjectMember, JsSyntaxNode, JsVariableDeclarator,
};
use biome_js_semantic::SemanticModel;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_top_level_regex::UseTopLevelRegexOptions;
use rustc_hash::FxHashSet;

use crate::{
    JsRuleAction,
    services::{control_flow::AnyJsControlFlowRoot, semantic::Semantic},
    utils::module_constant::{
        collision_free_module_constant_name, extract_module_constant,
        extract_module_constant_with_reserved_names, is_module_constant_extractable,
        module_constant_insertion_slot,
    },
};

declare_lint_rule! {
    /// Require regex literals to be declared at the top level.
    ///
    /// This rule is useful to avoid performance issues when using regex literals inside functions called many times (hot paths). Regex literals create a new RegExp object when they are evaluated. (See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp) By declaring them at the top level, this overhead can be avoided.
    ///
    /// It's important to note that this rule is not recommended for all cases. Placing regex literals at the top level can hurt startup times. In browser contexts, this can result in longer page loads.
    ///
    /// Additionally, this rule ignores regular expressions with the `g` and/or `y` flags, as they maintain internal state and can cause
    /// [side effects](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex#avoiding_side_effects) when calling `test` and `exec` with them.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo(someString) {
    ///     return /[a-Z]*/.test(someString)
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const REGEX = /[a-Z]*/;
    ///
    /// function foo(someString) {
    ///     return REGEX.test(someString)
    /// }
    /// ```
    ///
    /// ```js
    /// function foo(str) {
    ///     return /[a-Z]*/g.exec(str)
    /// }
    /// ```
    ///
    pub UseTopLevelRegex {
        version: "1.8.0",
        name: "useTopLevelRegex",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseTopLevelRegex {
    type Query = Semantic<JsRegexLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseTopLevelRegexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex = ctx.query();
        is_actionable_regex(regex).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This regex literal is not defined in the top level scope. This can lead to performance issues if this function is called frequently."
                },
            )
            .note(markup! {
                "Move the regex literal outside of this scope, and place it at the top level of this module, as a constant."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let regex = ctx.query().clone();
        if !is_actionable_regex(&regex) {
            return None;
        }

        let root = ctx.root();
        let (candidate_name, reserved_names, transfer_header) =
            coordinated_extraction(&root, ctx.model(), &regex)?;
        let value = AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsRegexLiteralExpression(regex.clone()),
        );
        let (mutation, name) = if reserved_names.is_empty() && transfer_header {
            extract_module_constant(
                &root,
                ctx.model(),
                regex.syntax(),
                value,
                &candidate_name,
            )?
        } else {
            extract_module_constant_with_reserved_names(
                &root,
                ctx.model(),
                regex.syntax(),
                value,
                &candidate_name,
                &reserved_names,
                transfer_header,
            )?
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Extract the regex literal into "<Emphasis>{name}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

fn is_actionable_regex(regex: &JsRegexLiteralExpression) -> bool {
    // Ignore regular expressions with the g and/or y flags, as calling test/exec has side effects.
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex#avoiding_side_effects
    let Ok((_, flags)) = regex.decompose() else {
        return false;
    };
    if flags.text().contains('g') || flags.text().contains('y') {
        return false;
    }

    !regex.syntax().ancestors().skip(1).all(|node| {
        match AnyJsControlFlowRoot::try_cast(node) {
            Ok(node) => {
                matches!(
                    node,
                    AnyJsControlFlowRoot::JsStaticInitializationBlockClassMember(_)
                        | AnyJsControlFlowRoot::TsModuleDeclaration(_)
                        | AnyJsControlFlowRoot::JsModule(_)
                        | AnyJsControlFlowRoot::JsScript(_)
                )
            }
            Err(node) => {
                if let Some(node) = JsPropertyClassMember::cast(node) {
                    node.modifiers().iter().any(|modifier| {
                        matches!(modifier, AnyJsPropertyModifier::JsStaticModifier(_))
                    })
                } else {
                    true
                }
            }
        }
    })
}

fn coordinated_extraction(
    root: &biome_js_syntax::AnyJsRoot,
    model: &SemanticModel,
    current: &JsRegexLiteralExpression,
) -> Option<(String, FxHashSet<String>, bool)> {
    // Actions are merged in source order, so reserve names from earlier
    // extractable literals to keep repeated candidates unique in a fix-all.
    let mut reserved_names = FxHashSet::default();

    for descendant in root.syntax().descendants() {
        let Some(regex) = JsRegexLiteralExpression::cast(descendant) else {
            continue;
        };
        if !is_actionable_regex(&regex)
            || !is_module_constant_extractable(root, model, regex.syntax())
        {
            continue;
        }

        let candidate_name = regex_constant_name(&regex);
        let name = collision_free_module_constant_name(
            model,
            regex.syntax(),
            &candidate_name,
            &reserved_names,
        );
        let transfer_header = module_constant_insertion_slot(root, regex.syntax()) == Some(0);

        if regex.syntax() == current.syntax() {
            return Some((name, reserved_names, transfer_header));
        }

        reserved_names.insert(name);
    }

    None
}

fn regex_constant_name(regex: &JsRegexLiteralExpression) -> String {
    for ancestor in regex.syntax().ancestors().skip(1) {
        if let Some(declarator) = JsVariableDeclarator::cast(ancestor.clone())
            && let Some(name) = declarator
                .id()
                .ok()
                .and_then(|pattern| pattern.as_any_js_binding().cloned())
                .and_then(|binding| binding_name(&binding))
        {
            if let Some(name) = normalize_name_component(&name, true) {
                return format!("{name}_REGEX");
            }
        }

        if let Some(function) = AnyJsFunction::cast(ancestor.clone())
            && let Some(name) = function_name(&function)
        {
            if let Some(name) = normalize_name_component(&name, true) {
                return format!("{name}_REGEX");
            }
        }

        if let Some(name) = method_name(&ancestor) {
            if let Some(name) = normalize_name_component(&name, true) {
                return format!("{name}_REGEX");
            }
        }

        if let Some(name) = property_name(&ancestor) {
            if let Some(name) = normalize_name_component(&name, true) {
                return format!("{name}_REGEX");
            }
        }
    }

    // A context-free literal uses its pattern as a stable fallback name.
    let pattern = regex
        .decompose()
        .ok()
        .map(|(pattern, _)| pattern.text().to_string())
        .and_then(|pattern| normalize_name_component(&pattern, false));
    pattern.map_or_else(
        || "REGEX".to_string(),
        |pattern| format!("REGEX_{pattern}"),
    )
}

fn binding_name(binding: &AnyJsBinding) -> Option<String> {
    binding
        .as_js_identifier_binding()?
        .name_token()
        .ok()
        .map(|token| token.text_trimmed().to_string())
}

fn function_name(function: &AnyJsFunction) -> Option<String> {
    let binding = match function {
        AnyJsFunction::JsFunctionDeclaration(function) => function.id().ok(),
        AnyJsFunction::JsFunctionExportDefaultDeclaration(function) => function.id(),
        AnyJsFunction::JsFunctionExpression(function) => function.id(),
        AnyJsFunction::JsArrowFunctionExpression(_) => None,
    }?;
    binding_name(&binding)
}

fn method_name(node: &JsSyntaxNode) -> Option<String> {
    if let Some(method) = JsMethodObjectMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(method) = JsGetterObjectMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(method) = JsSetterObjectMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(method) = JsMethodClassMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.text().to_string());
    }
    if let Some(method) = JsGetterClassMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.text().to_string());
    }
    if let Some(method) = JsSetterClassMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.text().to_string());
    }
    None
}

fn property_name(node: &JsSyntaxNode) -> Option<String> {
    if let Some(property) = JsPropertyObjectMember::cast(node.clone()) {
        return property.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(property) = JsPropertyClassMember::cast(node.clone()) {
        return property.name().ok()?.name().map(|name| name.text().to_string());
    }
    None
}

fn normalize_name_component(text: &str, ensure_identifier_start: bool) -> Option<String> {
    let mut normalized = String::new();
    let mut previous_is_lowercase = false;

    for character in text.chars() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && previous_is_lowercase {
                normalized.push('_');
            }
            normalized.push(character.to_ascii_uppercase());
            previous_is_lowercase = character.is_ascii_lowercase();
        } else {
            if !normalized.ends_with('_') {
                normalized.push('_');
            }
            previous_is_lowercase = false;
        }
    }

    let normalized = normalized.trim_matches('_');
    if normalized.is_empty() {
        return None;
    }

    if ensure_identifier_start
        && normalized
            .as_bytes()
            .first()
            .is_some_and(|character| character.is_ascii_digit())
    {
        Some(format!("NUMBER_{normalized}"))
    } else {
        Some(normalized.to_string())
    }
}
