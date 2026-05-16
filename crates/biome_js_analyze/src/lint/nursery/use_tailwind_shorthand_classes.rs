use crate::JsRuleAction;
use crate::tailwind::{AnyTailwindClassString, apply_fixed_class_string, host_range};
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_shorthand_classes::{
    TailwindShorthandViolation, analyze_tailwind_shorthand, auto_fix,
};
use biome_tailwind_syntax::TwRoot;

pub enum TailwindShorthandState {
    Violation {
        root: TwRoot,
        violation: TailwindShorthandViolation,
    },
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
    /// ```jsx,expect_diagnostic
    /// <div className="w-4 h-4" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="size-4" />;
    /// ```
    ///
    /// The rule checks string arguments and tagged template literals passed to known helper functions.
    /// This is useful for libraries like [`clsx`](https://github.com/lukeed/clsx),
    /// [`cva`](https://cva.style/), or CSS-in-JS helpers such as `tw`.
    ///
    /// Tagged template members like `tw.div` are also checked when their base function name is enabled.
    ///
    /// ## Known limitations
    ///
    /// This rule currently doesn't check bare strings inside framework-specific class collections,
    /// such as array or object entries in Vue, Svelte, or Astro class bindings:
    ///
    /// ```svelte
    /// <div class={["w-4 h-4", selected && "px-2 py-2"]}></div>
    /// <div class={{ "mr-3 ml-3": active }}></div>
    /// ```
    ///
    /// It also doesn't check untagged template chunks inside framework class attributes:
    ///
    /// ```svelte
    /// <div class={`border-x border-y ${extra}`}></div>
    /// ```
    ///
    /// In Astro, bare strings inside `class:list` arrays are not checked unless they are passed
    /// to a configured helper function such as `clsx`.
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic
    /// clsx("w-4 h-4");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// tw.div`w-4 h-4`;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js
    /// clsx("size-4");
    /// ```
    ///
    /// ```js
    /// tw.div`size-4`;
    /// ```
    ///
    pub UseTailwindShorthandClasses {
        version: "next",
        name: "useTailwindShorthandClasses",
        language: "jsx",
        domains: &[RuleDomain::Tailwind],
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-shorthand-classes").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseTailwindShorthandClasses {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = TailwindShorthandState;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return Vec::new().into_boxed_slice();
        }

        let root = ctx.query().tailwind_root();
        analyze_tailwind_shorthand(&root.candidates())
            .into_iter()
            .map(|violation| TailwindShorthandState::Violation {
                root: root.clone(),
                violation,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let TailwindShorthandState::Violation { violation, .. } = state;

        let first_range = host_range(
            ctx.query().node(),
            violation.uncompressed_nodes.first()?.range(),
        )?;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first_range,
            markup! {
                "These Tailwind classes can be replaced with a shorthand class."
            },
        );

        for candidate in violation.uncompressed_nodes.iter().skip(1) {
                if let Some(range) = host_range(ctx.query().node(), candidate.range()) {
                diagnostic = diagnostic.detail(
                    range,
                    markup! {
                        "Compressible utility used here."
                    },
                );
            }
        }

        diagnostic = diagnostic.note(markup! {
            "Using fewer classes reduces duplication and improves readability."
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let TailwindShorthandState::Violation { root, violation } = state;

        let fixed = auto_fix(root, violation)?.commit().to_string();
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(
            &mut mutation,
            ctx.query().node(),
            &fixed,
            ctx.preferred_quote(),
            ctx.preferred_jsx_quote(),
        );

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
