use crate::JsRuleAction;
use crate::tailwind::{
    AnyTailwindClassString, apply_fixed_class_string, class_string, host_range,
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::{Diagnostic, MessageAndDescription};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;
use biome_tailwind_logic::use_tailwind_shorthand_classes::{
    TailwindShorthandViolation, analyze_tailwind_shorthand, auto_fix,
};
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::TwRoot;

pub enum TailwindShorthandState {
    ParseError {
        range: TextRange,
        message: MessageAndDescription,
    },
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
    /// ## Options
    ///
    /// ### `attributes`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["classList"]
    ///     }
    /// }
    /// ```
    ///
    /// Default: `[]`
    ///
    /// Class lists in the `class` and `className` JSX attributes are always checked.
    /// Use this option to add more attribute names that should be treated as Tailwind class lists.
    ///
    /// #### Invalid
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div classList="w-4 h-4" />;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```jsx,use_options
    /// <div classList="size-4" />;
    /// ```
    ///
    /// ### `functions`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": ["clsx", "tw"]
    ///     }
    /// }
    /// ```
    ///
    /// Default: `["clsx", "tw", "twMerge", "twJoin", "cva", "tv", "cn", "cc", "cnb", "ctl"]`
    ///
    /// Use this option to check string arguments and tagged template literals passed to helper functions.
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
    /// ```js,expect_diagnostic,use_options
    /// clsx("w-4 h-4");
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// tw.div`w-4 h-4`;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// clsx("size-4");
    /// ```
    ///
    /// ```js,use_options
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
    type Query = Ast<AnyTailwindClassString>;
    type State = TailwindShorthandState;
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindShorthandClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(value) = class_string(ctx.query(), ctx.options()) else {
            return Vec::new().into_boxed_slice();
        };
        let parse = parse_tailwind(value.text());
        if parse.has_errors() {
            return parse
                .diagnostics()
                .iter()
                .filter_map(|diagnostic| {
                    Some(TailwindShorthandState::ParseError {
                        range: host_range(ctx.query(), diagnostic.location().span?)?,
                        message: diagnostic.message.clone(),
                    })
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
        }

        let root = parse.tree();
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
        let violation = match state {
            TailwindShorthandState::ParseError { range, message } => {
                return Some(
                    RuleDiagnostic::new(rule_category!(), *range, markup! {{message}}).note(
                        markup! {
                            "Biome could not analyze this Tailwind class list because it contains invalid syntax."
                        },
                    ),
                );
            }
            TailwindShorthandState::Violation { violation, .. } => violation,
        };

        let first_range = host_range(
            ctx.query(),
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
            if let Some(range) = host_range(ctx.query(), candidate.range()) {
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
        let TailwindShorthandState::Violation { root, violation } = state else {
            return None;
        };

        let fixed = auto_fix(root, violation)?.commit().to_string();
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(
            &mut mutation,
            ctx.query(),
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
