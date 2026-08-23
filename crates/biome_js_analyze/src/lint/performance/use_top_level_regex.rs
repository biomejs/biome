use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsPropertyModifier, JsPropertyClassMember,
    JsRegexLiteralExpression,
};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_top_level_regex::UseTopLevelRegexOptions;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    JsRuleAction,
    services::{control_flow::AnyJsControlFlowRoot, semantic::Semantic},
    utils::module_constant::{
        collision_free_module_constant_name_with_facts, extract_module_constant,
        extract_module_constant_with_reserved_names, is_module_constant_extractable_with_facts,
        module_constant_facts, module_constant_insertion_slot, module_constant_regex_name,
    },
};

declare_lint_rule! {
    /// Require regex literals to be declared at the top level.
    ///
    /// This rule is useful to avoid performance issues when using regex literals inside functions called many times (hot paths). Regex literals create a new RegExp object when they are evaluated. (See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp) By declaring them at the top level, this overhead can be avoided.
    ///
    /// It's important to note that this rule is not recommended for all cases. Placing regex literals at the top level can hurt startup times. In browser contexts, this can result in longer page loads.
    ///
    /// The unsafe fix intentionally changes evaluation timing and may change object identity or the
    /// visibility of mutable properties. Apply it only when those runtime semantics are acceptable.
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

    /// Builds an extraction action, or returns `None` when coordinated extraction cannot proceed.
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let regex = ctx.query().clone();
        let root = ctx.root();
        let (candidate_name, reserved_names, transfer_header) =
            coordinated_extraction(&root, ctx.model(), &regex)?;
        let value = AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsRegexLiteralExpression(regex.clone()),
        );
        let (mutation, name) = if reserved_names.is_empty() && transfer_header {
            extract_module_constant(&root, ctx.model(), regex.syntax(), value, &candidate_name)?
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

    !regex
        .syntax()
        .ancestors()
        .skip(1)
        .all(|node| match AnyJsControlFlowRoot::try_cast(node) {
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
        })
}

/// Finds the current regex's name and extraction details while scanning regexes in source order.
/// Identical literals reuse a name; earlier distinct literals reserve the names they select.
/// Returns `None` when the current regex is not eligible for module-constant extraction.
fn coordinated_extraction(
    root: &biome_js_syntax::AnyJsRoot,
    model: &SemanticModel,
    current: &JsRegexLiteralExpression,
) -> Option<(String, FxHashSet<String>, bool)> {
    // Scan regex literals in source order. Reuse a name for identical literals and reserve names
    // chosen for earlier different literals when fixing multiple diagnostics together.
    let facts = module_constant_facts(root, model);
    let mut reserved_names = FxHashSet::default();
    let mut names_by_value = FxHashMap::default();

    for descendant in root.syntax().descendants() {
        let Some(regex) = JsRegexLiteralExpression::cast(descendant) else {
            continue;
        };
        if !is_actionable_regex(&regex)
            || !is_module_constant_extractable_with_facts(root, regex.syntax(), &facts)
        {
            continue;
        }

        let value_key = regex.syntax().text_trimmed().to_string();
        if let Some(name) = names_by_value.get(&value_key).cloned() {
            if regex.syntax() == current.syntax() {
                reserved_names.remove(&name);
                return Some((
                    name,
                    reserved_names,
                    module_constant_insertion_slot(root, regex.syntax()) == Some(0),
                ));
            }
            continue;
        }

        let pattern = regex
            .decompose()
            .ok()
            .map(|(pattern, _)| pattern.text().to_string())
            .unwrap_or_default();
        let candidate_name = module_constant_regex_name(regex.syntax(), &pattern);
        let name = collision_free_module_constant_name_with_facts(
            model,
            regex.syntax(),
            &candidate_name,
            &reserved_names,
            &facts,
        );
        let transfer_header = module_constant_insertion_slot(root, regex.syntax()) == Some(0);
        names_by_value.insert(value_key, name.clone());

        if regex.syntax() == current.syntax() {
            return Some((name, reserved_names, transfer_header));
        }

        reserved_names.insert(name);
    }

    None
}
