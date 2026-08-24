use biome_analyze::{
    FixKind, QueryMatch, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsPropertyModifier, AnyJsRoot,
    JsPropertyClassMember, JsRegexLiteralExpression, JsSyntaxNode,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_top_level_regex::UseTopLevelRegexOptions;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;

use crate::{
    JsRuleAction,
    services::{control_flow::AnyJsControlFlowRoot, semantic::Semantic},
    utils::module_constant::{
        collision_free_module_constant_name_with_facts,
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
        let (mutation, name) = extract_module_constant_with_reserved_names(
            &root,
            ctx.model(),
            regex.syntax(),
            value,
            &candidate_name,
            &reserved_names,
            transfer_header,
        )?;

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
    root: &AnyJsRoot,
    model: &SemanticModel,
    current: &JsRegexLiteralExpression,
) -> Option<(String, FxHashSet<String>, bool)> {
    let current_range = current.syntax().text_range();

    REGEX_COORDINATION.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache
            .as_ref()
            .is_none_or(|cached| !cached.root.eq(root.syntax()))
        {
            *cache = Some(CachedRegexCoordination {
                root: root.syntax().clone(),
                extractions: build_coordinated_extractions(root, model),
            });
        }

        let extraction = cache.as_ref()?.extractions.get(&current_range)?.clone();
        Some((
            extraction.name,
            extraction.reserved_names,
            extraction.transfer_header,
        ))
    })
}

#[derive(Clone)]
struct CoordinatedExtraction {
    name: String,
    reserved_names: FxHashSet<String>,
    transfer_header: bool,
}

struct CachedRegexCoordination {
    root: JsSyntaxNode,
    extractions: FxHashMap<TextRange, CoordinatedExtraction>,
}

thread_local! {
    static REGEX_COORDINATION: RefCell<Option<CachedRegexCoordination>> = const { RefCell::new(None) };
}

fn build_coordinated_extractions(
    root: &AnyJsRoot,
    model: &SemanticModel,
) -> FxHashMap<TextRange, CoordinatedExtraction> {
    // Scan regex literals in source order. Reuse a name for identical literals and reserve names
    // chosen for earlier different literals when fixing multiple diagnostics together.
    let facts = module_constant_facts(root, model);
    let mut extractions = FxHashMap::default();
    let mut reserved_names = FxHashSet::default();
    let mut names_by_value: FxHashMap<String, Vec<String>> = FxHashMap::default();

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
        let pattern = regex
            .decompose()
            .ok()
            .map(|(pattern, _)| pattern.text().to_string())
            .unwrap_or_default();
        let candidate_name = module_constant_regex_name(regex.syntax(), &pattern);
        let (name, reused_name) = if let Some(names) = names_by_value.get(&value_key) {
            let reusable_name = names.iter().find(|name| {
                let name = name.as_str();
                let mut names_available_for_target = reserved_names.clone();
                names_available_for_target.remove(name);
                collision_free_module_constant_name_with_facts(
                    model,
                    regex.syntax(),
                    name,
                    &names_available_for_target,
                    &facts,
                ) == name
            });

            if let Some(name) = reusable_name {
                (name.clone(), true)
            } else {
                (
                    collision_free_module_constant_name_with_facts(
                        model,
                        regex.syntax(),
                        &candidate_name,
                        &reserved_names,
                        &facts,
                    ),
                    false,
                )
            }
        } else {
            (
                collision_free_module_constant_name_with_facts(
                    model,
                    regex.syntax(),
                    &candidate_name,
                    &reserved_names,
                    &facts,
                ),
                false,
            )
        };

        let mut names_for_current = reserved_names.clone();
        names_for_current.remove(&name);
        extractions.insert(
            regex.syntax().text_range(),
            CoordinatedExtraction {
                name: name.clone(),
                reserved_names: names_for_current,
                transfer_header: module_constant_insertion_slot(root, regex.syntax()) == Some(0),
            },
        );

        if !reused_name {
            names_by_value
                .entry(value_key)
                .or_default()
                .push(name.clone());
            reserved_names.insert(name);
        }
    }

    extractions
}

#[cfg(test)]
mod tests {
    use super::{coordinated_extraction, JsRegexLiteralExpression};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_languages::JsFileSource;
    use biome_rowan::AstNode;

    #[test]
    fn does_not_reuse_regex_name_when_shadowed() {
        let parsed = parse(
            r#"function read(value) {
    /x/.test(value);
    ((READ_REGEX) => /x/.test(value))(value);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let regexes = parsed
            .syntax()
            .descendants()
            .filter_map(JsRegexLiteralExpression::cast)
            .collect::<Vec<_>>();
        let second = regexes.get(1).expect("expected a second regex literal");

        let (name, _, _) = coordinated_extraction(&parsed.tree(), &model, second)
            .expect("expected a coordinated extraction");

        assert_eq!(name, "READ_REGEX_2");
    }
}
