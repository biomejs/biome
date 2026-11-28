use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlContent, AnyHtmlElement, HtmlRoot};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, BatchMutationExt, TextRange, chain_trivia_pieces};
use biome_rule_options::no_vue_content_outside_block::NoVueContentOutsideBlockOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow content outside of the top-level blocks in Vue components.
    ///
    /// A Vue component file is made of top-level blocks, such as `<template>`,
    /// `<script>`, `<style>`, and custom blocks like `<docs>`. Vue only reads
    /// what is inside these blocks and drops everything between them. Text or
    /// code placed outside of a block never renders or runs, so it is usually
    /// a leftover or a mistake.
    ///
    /// HTML comments outside of the blocks are allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div>Hello</div>
    /// </template>
    ///
    /// console.log("hello");
    ///
    /// <script>
    /// export default {};
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <!-- A comment between the blocks -->
    /// <template>
    ///   <div>Hello</div>
    /// </template>
    ///
    /// <script>
    /// console.log("hello");
    /// export default {};
    /// </script>
    ///
    /// <docs>
    ///   Some documentation.
    /// </docs>
    /// ```
    ///
    pub NoVueContentOutsideBlock {
        version: "2.6.10",
        name: "noVueContentOutsideBlock",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-content-outside-block").same()],
        fix_kind: FixKind::Unsafe,
    }
}

/// A run of content between two top-level blocks or comments.
pub struct ContentRun {
    first: AnyHtmlContent,
    last: AnyHtmlContent,
}

impl Rule for NoVueContentOutsideBlock {
    type Query = Ast<HtmlRoot>;
    type State = ContentRun;
    type Signals = Box<[Self::State]>;
    type Options = NoVueContentOutsideBlockOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return Box::default();
        }

        let mut runs = Vec::new();
        let mut current: Option<ContentRun> = None;
        for element in ctx.query().elements() {
            let AnyHtmlElement::AnyHtmlContent(content) = element else {
                runs.extend(current.take());
                continue;
            };
            match &mut current {
                // Comments are trivia attached to the surrounding content, so a
                // comment between two pieces of content splits them into two
                // runs. This keeps the comment out of the reported range and
                // out of the removal.
                Some(run)
                    if !run.last.syntax().has_trailing_comments()
                        && !content.syntax().has_leading_comments() =>
                {
                    run.last = content;
                }
                _ => {
                    runs.extend(current.replace(ContentRun {
                        first: content.clone(),
                        last: content,
                    }));
                }
            }
        }
        runs.extend(current);

        runs.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::new(
                    state.first.syntax().text_trimmed_range().start(),
                    state.last.syntax().text_trimmed_range().end(),
                ),
                markup! {
                    "This content is outside of the top-level blocks."
                },
            )
            .note(markup! {
                "Vue only reads the top-level blocks of a component, such as "<Emphasis>"<template>"</Emphasis>", "<Emphasis>"<script>"</Emphasis>", and "<Emphasis>"<style>"</Emphasis>". Anything outside of them is dropped, so it never renders or runs."
            })
            .note(markup! {
                "Move this content into a block, or remove it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let first = state.first.syntax();
        let last = state.last.syntax();

        // The trivia around the run holds the comments next to it, which have to
        // stay in the file, so it moves onto the next token. Trailing trivia
        // without comments is only spacing, so it is dropped. When nothing else
        // separates the leading trivia from a next token that starts on its own
        // line, the whitespace that ends the leading trivia would only add blank
        // lines, so it is dropped too.
        let leading: Vec<_> = first.first_leading_trivia()?.pieces().collect();
        let trailing: Vec<_> = last.last_trailing_trivia()?.pieces().collect();
        let next_token = last.next_token()?;
        let next_leading = next_token.leading_trivia();
        let kept_trailing = if trailing.iter().any(|piece| piece.is_comments()) {
            &trailing[..]
        } else {
            &[]
        };
        let kept_leading = if kept_trailing.is_empty()
            && next_leading
                .pieces()
                .next()
                .is_some_and(|piece| piece.is_newline())
        {
            let end = leading
                .iter()
                .rposition(|piece| !piece.is_whitespace() && !piece.is_newline())
                .map_or(0, |index| index + 1);
            &leading[..end]
        } else {
            &leading[..]
        };
        let new_next_token = next_token.with_leading_trivia_pieces(chain_trivia_pieces(
            chain_trivia_pieces(kept_leading.iter().cloned(), kept_trailing.iter().cloned()),
            next_leading.pieces(),
        ));

        let mut mutation = ctx.root().begin();
        mutation.replace_token_discard_trivia(next_token, new_next_token);
        for element in first
            .siblings(biome_rowan::Direction::Next)
            .take_while(|element| element != last)
            .chain([last.clone()])
        {
            mutation.remove_element(element.into());
        }

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Remove the content outside of the top-level blocks."
            },
            mutation,
        ))
    }
}
