//! JavaScript supports parenthesizing expressions, assignments, and TypeScript types.
//! Parenthesizing an expression can be desired to change the operator precedence or to ease readability.
//!
//! Biome is opinionated about which parentheses to keep or where to insert parentheses.
//! It removes parentheses that aren't necessary to keep the original semantic and to improve readability.
//! Biome also inserts parentheses around nodes where we think this improves readability.
//!
//! The [NeedsParentheses] trait forms the foundation of Biome's parentheses formatting.
//! Most of the nodes supporting parentheses (expressions, assignments, and types) implements this trait.
//! [NeedsParentheses::needs_parentheses] is the trait's main method that implements
//! the rules when a node requires parentheses.
//! Tests are available in the `biome_js_formatter` crate.
//!
//! A node requires parentheses to:
//! - improve readability: `a << b << 3` is harder to read than `(a << b) << 3`
//! - form valid syntax: `class A extends 3 + 3 {}` isn't valid, but `class A extends (3 + 3) {}` is
//! - preserve operator precedence: `(a + 3) * 4` has a different meaning than `a + 3 * 4`
//!
//! The challenge of formatting parenthesized nodes is that a tree with parentheses and a tree without
//! parentheses (that have the same semantics) must result in the same output. For example,
//! formatting `(a + 3) + 5` must yield the same formatted output as `a + 3 + 5` or `a + (3 + 5)` or even
//! `(((a + 3) + 5))` even though all these trees differ by the number of parenthesized expressions.
//!
//! There are two measures taken by Biome to ensure formatting is stable regardless of the number of parenthesized nodes in a tree:
//!
//! ## Removing parenthesized nodes
//!
//! The JavaScript formatter pre-processes (See `biome_js_formatter::JsFormatSyntaxRewriter`)
//! the input CST and removes all parenthesized expressions, assignments, and types except if:
//!
//! - The parenthesized node has a syntax error (skipped token trivia, missing inner expression)
//! - The node has a directly preceding closure type cast comment
//! - The inner expression is a bogus node
//!
//! Removing the parenthesized nodes has the benefit that a input tree with parentheses and an input tree
//! without parentheses have the same structure for as far as the formatter is concerned and thus,
//! the formatter makes the same decisions for both trees.
//!
//! ## Parentheses insertion
//!
//! The parentheses that get removed by the pre-processing step are re-inserted by the nodes' formatter.
//! The rule inserts parentheses for each node where `FormatNodeRule::needs_parentheses` returns true.

mod assignment;
mod expression;
mod tstype;

/// Node that may be parenthesized to ensure it forms valid syntax or to improve readability
pub trait NeedsParentheses: biome_rowan::AstNode<Language = crate::JsLanguage> {
    /// Returns `true` if this node requires parentheses to form valid syntax or improve readability.
    ///
    /// Returns `false` if the parentheses can be omitted safely without changing the semantic.
    fn needs_parentheses(&self) -> bool;
}
