use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::SvelteIfBlock;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_dupe_else_if_blocks::NoDupeElseIfBlocksOptions;

declare_lint_rule! {
    /// Disallow duplicate conditions in Svelte `{#if}` / `{:else if}` chains.
    ///
    /// If an `{:else if}` condition is textually identical to a previous condition in the same
    /// chain, it can never execute, making it dead code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// {#if a}
    ///   <div>a</div>
    /// {:else if b}
    ///   <div>b</div>
    /// {:else if a}
    ///   <div>a again</div>
    /// {/if}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// {#if a}
    ///   <div>a</div>
    /// {:else if b}
    ///   <div>b</div>
    /// {:else if c}
    ///   <div>c</div>
    /// {/if}
    /// ```
    ///
    pub NoDupeElseIfBlocks {
        version: "next",
        name: "noDupeElseIfBlocks",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("no-dupe-else-if-blocks").same()],
    }
}

pub struct State {
    /// Range of the duplicate condition.
    duplicate_range: TextRange,
    /// Range of the first occurrence.
    original_range: TextRange,
    /// The condition text.
    condition: String,
}

impl Rule for NoDupeElseIfBlocks {
    type Query = Ast<SvelteIfBlock>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = NoDupeElseIfBlocksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Collect (text, range) for each condition in the chain.
        let mut conditions: Vec<(String, TextRange)> = Vec::new();
        let mut violations: Vec<State> = Vec::new();

        // Opening block condition.
        if let Ok(opening) = node.opening_block()
            && let Ok(expr) = opening.expression()
            && let Ok(token) = expr.html_literal_token()
        {
            let text = token.text_trimmed().to_string();
            let range = token.text_trimmed_range();
            conditions.push((text, range));
        }

        // Else-if clause conditions.
        for clause in node.else_if_clauses().iter() {
            let Ok(expr) = clause.expression() else {
                continue;
            };
            let Ok(token) = expr.html_literal_token() else {
                continue;
            };
            let text = token.text_trimmed().to_string();
            let range = token.text_trimmed_range();

            if let Some((_, original_range)) =
                conditions.iter().find(|(prev_text, _)| prev_text == &text)
            {
                violations.push(State {
                    duplicate_range: clause.range(),
                    original_range: *original_range,
                    condition: text.clone(),
                });
            }

            conditions.push((text, range));
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let condition = state.condition.as_str();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.duplicate_range,
                markup! {
                    "This branch can never execute. Its condition "<Emphasis>{condition}</Emphasis>" is a duplicate of a previous condition."
                },
            )
            .detail(
                state.original_range,
                "This is the first occurrence of the condition.",
            ),
        )
    }
}
