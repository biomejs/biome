use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlElement, HtmlRoot};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_vue_block_order::{BlockOrderEntry, UseVueBlockOrderOptions};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce the order of the top-level blocks in Vue components.
    ///
    /// A Vue component file is made of top-level blocks, such as `<script>`,
    /// `<template>`, and `<style>`. Putting these blocks in the same order in
    /// every component makes components easier to read, because you always
    /// know where to look for each part.
    ///
    /// By default, `<script>` and `<template>` blocks come first, in any order,
    /// followed by `<style>` blocks. Blocks whose tag name isn't part of the
    /// configured order, such as a custom `<docs>` block, can appear anywhere.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <style></style>
    /// <script></script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template></template>
    /// <script></script>
    /// <docs></docs>
    /// <style></style>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `order`
    ///
    /// Default: `[["script", "template"], "style"]`
    ///
    /// The order of the top-level blocks. Each entry is either a tag name, or a
    /// list of tag names whose blocks may appear in any order relative to each
    /// other. If a tag name is listed more than once, its first position is used.
    ///
    /// The following configuration puts `<template>` first, then `<script>`,
    /// then `<style>`:
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "order": ["template", "script", "style"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script></script>
    /// <template></template>
    /// <style></style>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template></template>
    /// <script></script>
    /// <style></style>
    /// ```
    ///
    pub UseVueBlockOrder {
        version: "next",
        name: "useVueBlockOrder",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("block-order").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

pub struct BlockOrderViolation {
    /// The top-level block that is out of place.
    block: AnyHtmlElement,
    /// The index of `block` in the list of top-level nodes.
    block_index: usize,
    /// A block above `block` that should come after it. When several blocks
    /// above it should come after it, this is the first of those that belong
    /// to the earliest position in the configured order.
    first_unordered: AnyHtmlElement,
    /// The index of `first_unordered` in the list of top-level nodes.
    first_unordered_index: usize,
}

impl Rule for UseVueBlockOrder {
    type Query = Ast<HtmlRoot>;
    type State = BlockOrderViolation;
    type Signals = Box<[Self::State]>;
    type Options = UseVueBlockOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return Box::default();
        }

        let order = ctx.options().order();
        // The first block found at each position of the configured order.
        let mut first_at_position: Vec<Option<(usize, AnyHtmlElement)>> = vec![None; order.len()];
        let mut signals = Vec::new();

        for (block_index, block) in ctx.query().html().iter().enumerate() {
            let Some(position) = block.name().and_then(|name| position_of(order, name.text()))
            else {
                continue;
            };

            if let Some((first_unordered_index, first_unordered)) =
                first_at_position[position + 1..].iter().flatten().next()
            {
                signals.push(BlockOrderViolation {
                    block: block.clone(),
                    block_index,
                    first_unordered: first_unordered.clone(),
                    first_unordered_index: *first_unordered_index,
                });
            }

            first_at_position[position].get_or_insert((block_index, block));
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.block.name()?;
        let unordered_name = state.first_unordered.name()?;
        let order = BlockOrder(ctx.options().order());

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.block.range(),
                markup! {
                    "The "<Emphasis>"<"{name.text()}">"</Emphasis>" block should be above the "<Emphasis>"<"{unordered_name.text()}">"</Emphasis>" block."
                },
            )
            .detail(
                state.first_unordered.range(),
                markup! {
                    "This "<Emphasis>"<"{unordered_name.text()}">"</Emphasis>" block should come after it."
                },
            )
            .note(markup! {
                "Putting the blocks of every component in the same order makes components easier to read."
            })
            .note(markup! {
                "This project puts top-level blocks in this order: "{order}"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let name = state.block.name()?;
        let unordered_name = state.first_unordered.name()?;
        let elements: Vec<_> = ctx.query().html().iter().collect();
        let start = state.first_unordered_index;
        let end = state.block_index;
        if start >= end || end >= elements.len() {
            return None;
        }

        // Blocks with the same tag name keep their relative order, because the
        // order of blocks such as `<style>` affects the compiled component.
        // So every block between the two that shares the tag name of the
        // reported block moves along with it.
        let is_moved = |element: &AnyHtmlElement| {
            element.name().is_some_and(|other| other.text() == name.text())
        };
        let moved = elements[start + 1..end]
            .iter()
            .filter(|element| is_moved(element))
            .chain([&state.block]);
        let reordered: Vec<_> = elements[..start]
            .iter()
            .chain(moved)
            .chain(
                elements[start..end]
                    .iter()
                    .filter(|element| !is_moved(element)),
            )
            .chain(&elements[end + 1..])
            .collect();

        // Replacing a block keeps the whitespace and comments of the replaced
        // one, so whitespace and comments stay where they are and only the
        // blocks move.
        let mut mutation = ctx.root().begin();
        for (element, replacement) in elements.iter().zip(reordered) {
            if element != replacement {
                mutation.replace_node(element.clone(), replacement.clone());
            }
        }

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Move the "<Emphasis>"<"{name.text()}">"</Emphasis>" block above the "<Emphasis>"<"{unordered_name.text()}">"</Emphasis>" block."
            },
            mutation,
        ))
    }
}

/// Returns the position of `tag` in the configured order, or `None` when the
/// order doesn't list it.
fn position_of(order: &[BlockOrderEntry], tag: &str) -> Option<usize> {
    order
        .iter()
        .position(|entry| entry.tags().iter().any(|candidate| candidate.as_ref() == tag))
}

/// Formats the configured order, such as "`<script>` and `<template>` in any
/// order, then `<style>`".
struct BlockOrder<'a>(&'a [BlockOrderEntry]);

impl biome_console::fmt::Display for BlockOrder<'_> {
    fn fmt(&self, f: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for (index, entry) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_str(", then ")?;
            }
            let tags = entry.tags();
            for (tag_index, tag) in tags.iter().enumerate() {
                if tag_index > 0 {
                    f.write_str(if tag_index + 1 == tags.len() {
                        " and "
                    } else {
                        ", "
                    })?;
                }
                f.write_markup(markup! { <Emphasis>"<"{tag.as_ref()}">"</Emphasis> })?;
            }
            if tags.len() > 1 {
                f.write_str(" in any order")?;
            }
        }
        Ok(())
    }
}
