use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationOrRule, CssDeclarationOrRuleList, CssKeyframesAtRule, ScssNestingDeclaration,
    TwUtilityAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, Text, TextRange};
use biome_rule_options::no_duplicate_properties::NoDuplicatePropertiesOptions;
use rustc_hash::FxHashMap;

use crate::utils::{AnyPropertyStatement, is_tailwind_utility_fallback};

declare_lint_rule! {
    /// Disallow duplicate properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate properties. It ignores custom properties and declarations inside `@keyframes` blocks.
    ///
    /// ## SCSS limitations
    ///
    /// Declarations are compared within each authored declaration block. The rule does not expand
    /// mixins or includes, so duplicates introduced only after SCSS expansion are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   color: pink;
    ///   color: orange;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: pink;
    ///   background: orange;
    /// }
    /// ```
    ///
    pub NoDuplicateProperties {
        version: "1.9.4",
        name: "noDuplicateProperties",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties").same()],
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Ast<CssDeclarationOrRuleList>;
    type State = (TextRange, (TextRange, Text));
    type Signals = Option<Self::State>;
    type Options = NoDuplicatePropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        // `find_duplicate` checks a group of SCSS nested properties with the block around it when
        // the group's name is plain and that block is a list this rule queries.
        if node
            .syntax()
            .grand_parent()
            .and_then(ScssNestingDeclaration::cast)
            .is_some_and(|group| {
                group
                    .syntax()
                    .parent()
                    .is_some_and(|list| CssDeclarationOrRuleList::can_cast(list.kind()))
                    && AnyPropertyStatement::from(group).name().is_some()
            })
        {
            return None;
        }

        let mut in_utility = false;
        for ancestor in node.syntax().ancestors() {
            if CssKeyframesAtRule::can_cast(ancestor.kind()) {
                return None;
            }
            in_utility |= TwUtilityAtRule::can_cast(ancestor.kind());
        }

        find_duplicate(node, None, in_utility, &mut FxHashMap::default())
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_occurrence_range, (duplicate_range, duplicate_property_name)) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                duplicate_range,
                markup! {
                    "Duplicate properties can lead to unexpected behavior and may override previous declarations unintentionally."
                },
            )
            .detail(first_occurrence_range, markup! {
                <Emphasis>{duplicate_property_name.text()}</Emphasis> " is already defined here."
            })
            .note(markup! {
                "Remove or rename the duplicate property to ensure consistent styling."
            }),
        )
    }
}

/// Finds the first declaration in `list`, or in a group of nested properties within it, that sets
/// a property already in `seen`. Returns the range of the earlier declaration, paired with the
/// range and full name of the duplicate.
///
/// SCSS groups of nested properties are searched with their names joined to `prefix`, the way
/// Sass emits them, so `border: { radius: 1px; }` sets `border-radius`. A group sets its own
/// name only when it has a value, as in `font: bold { family: serif; }`.
///
/// `in_utility` skips Tailwind fallback declarations, see [is_tailwind_utility_fallback].
fn find_duplicate(
    list: &CssDeclarationOrRuleList,
    prefix: Option<&str>,
    in_utility: bool,
    seen: &mut FxHashMap<Text, TextRange>,
) -> Option<(TextRange, (TextRange, Text))> {
    for item in list {
        let statement = match item {
            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(declaration) => {
                if in_utility && is_tailwind_utility_fallback(&declaration) {
                    continue;
                }
                AnyPropertyStatement::from(declaration)
            }
            AnyCssDeclarationOrRule::ScssNestingDeclaration(group) => {
                AnyPropertyStatement::from(group)
            }
            _ => continue,
        };
        let Some(name) = statement.name() else {
            continue;
        };
        let prop_name: Text = match prefix {
            Some(prefix) => {
                let name = name.text_trimmed();
                let mut joined = String::with_capacity(prefix.len() + 1 + name.len());
                joined.push_str(prefix);
                joined.push('-');
                joined.push_str(name);
                joined.into()
            }
            None => name.token_text_trimmed().into(),
        };

        if prop_name.starts_with("--") {
            continue;
        }

        let (sets_own_name, nested) = match &statement {
            AnyPropertyStatement::CssDeclarationWithSemicolon(_) => (true, None),
            AnyPropertyStatement::ScssNestingDeclaration(group) => (
                group.value().is_ok_and(|value| !value.items().is_empty()),
                group
                    .block()
                    .ok()
                    .and_then(|block| block.as_css_declaration_or_rule_block().cloned())
                    .map(|block| block.items()),
            ),
        };

        if sets_own_name && let Some(&first) = seen.get(&prop_name) {
            return Some((first, (name.text_trimmed_range(), prop_name)));
        }

        if let Some(nested) = nested
            && let Some(duplicate) = find_duplicate(&nested, Some(&prop_name), in_utility, seen)
        {
            return Some(duplicate);
        }

        // Nested names are this name plus a hyphenated suffix, so none can match it, and it can be
        // added after them by move instead of clone.
        if sets_own_name {
            seen.insert(prop_name, name.text_trimmed_range());
        }
    }

    None
}
