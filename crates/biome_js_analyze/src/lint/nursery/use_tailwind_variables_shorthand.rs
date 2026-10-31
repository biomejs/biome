use crate::JsRuleAction;
use crate::tailwind::{AnyTailwindClassString, apply_fixed_class_string, host_range};
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::BatchMutationExt;
use biome_rule_options::use_tailwind_variables_shorthand::UseTailwindVariablesShorthandOptions;
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_variables_shorthand::{
    ArbitraryVariableValue, arbitrary_variable_values,
};

declare_lint_rule! {
    /// Enforce the shorthand syntax for CSS variables in Tailwind CSS classes.
    ///
    /// Tailwind CSS has a shorthand for a class whose value is a single CSS variable.
    /// You write the variable in parentheses, and Tailwind CSS adds `var()` for you.
    /// For example, `bg-(--brand)` generates the same CSS as `bg-[var(--brand)]`.
    /// This rule reports the longer form, so that every class in a project
    /// reads CSS variables the same way.
    ///
    /// The rule checks the value of a class and the part after `/`,
    /// such as `[var(--opacity)]` in `bg-red-500/[var(--opacity)]`.
    /// A fallback value is kept, so `bg-[var(--brand,#000)]` becomes `bg-(--brand,#000)`.
    ///
    /// The rule only reports a value that is a single `var()` with nothing around it.
    /// It doesn't report `w-[calc(var(--width)+1rem)]` or `text-[length:var(--size)]`.
    /// It also doesn't report classes that set a CSS property directly,
    /// such as `[color:var(--brand)]`.
    ///
    /// The shorthand requires Tailwind CSS v4.0 or later.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="bg-[var(--brand)]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="hover:bg-red-500/[var(--opacity)]" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="bg-(--brand) hover:bg-red-500/(--opacity)" />;
    /// <div className="w-[calc(var(--width)+1rem)] [color:var(--brand)]" />;
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    pub UseTailwindVariablesShorthand {
        version: "2.6.5",
        name: "useTailwindVariablesShorthand",
        language: "jsx",
        domains: &[RuleDomain::Tailwind],
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-consistent-variable-syntax").same()],
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseTailwindVariablesShorthand {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = ArbitraryVariableValue;
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindVariablesShorthandOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        arbitrary_variable_values(&ctx.query().tailwind_root().candidates())
            .into_iter()
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, value: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), value.range)?,
                markup! {
                    "This Tailwind CSS value reads a CSS variable with "<Emphasis>"[var()]"</Emphasis>" instead of the shorthand."
                },
            )
            .note(markup! {
                "Tailwind CSS adds "<Emphasis>"var()"</Emphasis>" to a variable written in parentheses, so the shorthand is shorter and generates the same CSS."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, value: &Self::State) -> Option<JsRuleAction> {
        let fixed = value.fixed_class_string(&ctx.query().tailwind_root())?;
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
            markup! { "Use the CSS variable shorthand." }.to_owned(),
            mutation,
        ))
    }
}
