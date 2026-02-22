use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_analyze::{FixKind, RuleSource};
use biome_console::markup;
use biome_rowan::AstNode;
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;
use biome_tailwind_logic::lint::use_tailwind_shorthand_classes::{
    TailwindShorthandViolation, analyze_tailwind_shorthand,
};
use biome_tailwind_syntax::TwRoot;

use crate::TailwindRuleAction;

declare_lint_rule! {
    /// Enforce using fewer Tailwind utilities instead of multiple utilities that are functionally the same.
    ///
    /// This rule detects sequences of Tailwind CSS utility classes that can be replaced by a single
    /// shorter utility. Using shorthands reduces duplication, keeps class lists readable, and helps
    /// prevent drift where one side gets updated but the matching side does not.
    ///
    /// Notes:
    /// - Values must match to compress (for example, `ml-2 mr-3` is not compressed).
    /// - Variants must match to compress (for example, `hover:ml-2 mr-2` is not compressed).
    /// - If an equivalent shorthand already exists for the same key and value, the rule highlights the
    ///   redundant longhands without suggesting an additional shorthand.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```tailwind,expect_diagnostic
    /// ml-2 mr-2
    /// ```
    /// ```tailwind,expect_diagnostic
    /// pl-2 pr-2 pt-2 pb-2
    /// ```
    /// ```tailwind,expect_diagnostic
    /// hover:w-4 hover:h-4
    /// ```
    ///
    /// ### Valid
    ///
    /// ```tailwind
    /// mx-2 -my-2
    /// p-2 pl-4
    /// hover:size-4
    /// ```
    ///
    pub UseTailwindShorthandClasses {
        version: "next",
        name: "useTailwindShorthandClasses",
        language: "tailwind",
        recommended: false,
        // Inspired because this rule is actually a little more intelligent than the original ESLint version.
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-shorthand-classes").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseTailwindShorthandClasses {
    type Query = Ast<TwRoot>;
    type State = TailwindShorthandViolation;
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindShorthandClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let source = root.syntax().text_trimmed().to_string();
        let violations = analyze_tailwind_shorthand(&source);
        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let first_range = state.original_ranges.first().copied()?;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first_range,
            markup! {
                "Prefer Tailwind shorthand utilities over multiple longhand utilities."
            },
        );

        // Highlight each longhand class occurrence
        for (idx, range) in state.original_ranges.iter().enumerate() {
            let original = state
                .originals
                .get(idx)
                .map_or("this utility", |s| s.as_str());
            diagnostic = diagnostic.detail(
                *range,
                markup! {
                    "Longhand utility "<Emphasis>{original}</Emphasis>" used here."
                },
            );
        }

        // Suggest the shorthand replacement when available
        if let Some(replacement) = &state.replacement {
            diagnostic = diagnostic.note(markup! {
                "You can replace them with the shorthand "<Emphasis>{replacement}</Emphasis>" to reduce duplication and improve readability."
            });
        }

        Some(diagnostic)
    }

    fn action(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TailwindRuleAction> {
        // Fix actions for Tailwind embedded snippets are applied at the parent-document level
        // (the JS/HTML file handler rewrites the class string). No AST mutation is needed here.
        None
    }
}
