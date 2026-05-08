use crate::HtmlRuleAction;
use crate::tailwind::{apply_fixed_class_string, class_string, host_range};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;
use biome_tailwind_logic::use_tailwind_shorthand_classes::{
    TailwindShorthandViolation, analyze_tailwind_shorthand, auto_fix,
};
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::TwRoot;

pub struct TailwindShorthandState {
    root: TwRoot,
    violation: TailwindShorthandViolation,
}

declare_lint_rule! {
    /// Enforce using fewer Tailwind utilities instead of multiple utilities that are functionally the same.
    ///
    /// This rule detects sequences of Tailwind CSS utility classes that can be replaced by a single
    /// shorter utility. Using shorthands reduces duplication, keeps class lists readable, and helps
    /// prevent drift where one side gets updated but the matching side does not.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="w-4 h-4"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="size-4"></div>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `attributes`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["data-classes"]
    ///     }
    /// }
    /// ```
    ///
    /// Default: `[]`
    ///
    /// The `class` attribute is always checked.
    /// Use this option to add more HTML attribute names that should be treated as Tailwind class lists.
    ///
    /// #### Invalid
    ///
    /// ```html,expect_diagnostic,use_options
    /// <div data-classes="w-4 h-4"></div>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```html,use_options
    /// <div data-classes="size-4"></div>
    /// ```
    ///
    pub UseTailwindShorthandClasses {
        version: "next",
        name: "useTailwindShorthandClasses",
        language: "html",
        domains: &[RuleDomain::Tailwind],
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-shorthand-classes").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseTailwindShorthandClasses {
    type Query = Ast<HtmlAttribute>;
    type State = TailwindShorthandState;
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindShorthandClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(class_list) = class_string(ctx.query(), ctx.options()) else {
            return Vec::new().into_boxed_slice();
        };

        let parse = parse_tailwind(class_list.text());
        if parse.has_errors() {
            return Vec::new().into_boxed_slice();
        }

        let root = parse.tree();
        analyze_tailwind_shorthand(&root.candidates())
            .into_iter()
            .map(|violation| TailwindShorthandState {
                root: root.clone(),
                violation,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let first_range = host_range(
            ctx.query(),
            state.violation.uncompressed_nodes.first()?.range(),
        )?;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first_range,
            markup! {
                "These Tailwind classes can be replaced with a shorthand class."
            },
        );

        for candidate in state.violation.uncompressed_nodes.iter().skip(1) {
            if let Some(range) = host_range(ctx.query(), candidate.range()) {
                diagnostic = diagnostic.detail(
                    range,
                    markup! {
                        "Compressible utility used here."
                    },
                );
            }
        }

        Some(diagnostic.note(markup! {
            "Using fewer classes reduces duplication and improves readability."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let fixed = auto_fix(&state.root, &state.violation)?.commit().to_string();

        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(&mut mutation, ctx.query(), &fixed)?;

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use the Tailwind shorthand classes."
            }
            .to_owned(),
            mutation,
        ))
    }
}
