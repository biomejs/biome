use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_analyze::{FixKind, QueryMatch, RuleSource};
use biome_console::markup;
use biome_js_syntax::{
    JsCallExpression, JsStringLiteralExpression, JsSyntaxNode, JsxAttribute, JsxString,
};
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rowan::{SyntaxResult, TextRange, TokenText};
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;
use biome_tailwind_logic::lint::use_tailwind_shorthand_classes::{
    TailwindShorthandViolation, analyze_tailwind_shorthand,
};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce using less Tailwind utilities instead of multiple utilities that are functionally the same.
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
    /// ```jsx,expect_diagnostic
    /// <div className="ml-2 mr-2" />
    /// ```
    /// ```jsx,expect_diagnostic
    /// <div className="pl-2 pr-2 pt-2 pb-2" />
    /// ```
    /// ```jsx,expect_diagnostic
    /// <div className="hover:w-4 hover:h-4" />
    /// ```
    /// ```jsx,expect_diagnostic
    /// <div className="border-x border-y" />
    /// ```
    /// ```jsx,expect_diagnostic
    /// <div className="overflow-hidden text-ellipsis whitespace-nowrap" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="mx-2 -my-2" />
    /// <div className="p-2 pl-4" />
    /// <div className="hover:size-4" />
    /// <div className="border" />
    /// <div className="truncate" />
    /// ```
    ///
    pub UseTailwindShorthandClasses {
        version: "next",
        name: "useTailwindShorthandClasses",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::Tailwind],
        // Inspired because this rule is actually a little more intelligent than the original ESLint version.
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-shorthand-classes").inspired()],
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
  pub AnyTailwindContainer = JsxAttribute | JsCallExpression
}

impl Rule for UseTailwindShorthandClasses {
    type Query = Ast<AnyTailwindContainer>;
    type State = (JsSyntaxNode, TailwindShorthandViolation);
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindShorthandClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut signals: Vec<Self::State> = Vec::new();
        let string_nodes = extract_string_syntax_nodes(node);
        for string_node in string_nodes {
            let Ok(value) = string_node.inner_string_text() else {
                continue;
            };
            let violations = analyze_tailwind_shorthand(&value);
            for mut violation in violations {
                // fix the text ranges to be relative to the original string node
                for range in &mut violation.original_ranges {
                    let offset = value.source_range(string_node.range()).start();
                    *range = TextRange::new(range.start() + offset, range.end() + offset);
                }
                signals.push((string_node.syntax().clone(), violation));
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (node, state) = state;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.text_range(),
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

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let (node, violation) = state;

        // Locate the string token to edit (either a normal JS string literal or a JSX string)
        let old_token = if let Some(lit) = JsStringLiteralExpression::cast(node.clone()) {
            lit.value_token().ok()?
        } else if let Some(jsx) = JsxString::cast(node.clone()) {
            jsx.value_token().ok()?
        } else {
            return None;
        };

        // Original token text (usually quoted, e.g. "ml-2 mr-2")
        let original = old_token.text_trimmed();
        let original_str = original;

        // Extract inner content and quotes (if any)
        let (content_start, content_end, quote_prefix, quote_suffix) =
            if original_str.starts_with('"') || original_str.starts_with('\'') {
                (
                    1usize,
                    original_str.len().saturating_sub(1),
                    original_str.chars().next().unwrap_or('"'),
                    original_str.chars().last().unwrap_or('"'),
                )
            } else {
                (0usize, original_str.len(), '\0', '\0')
            };

        let inner = &original_str[content_start..content_end];

        // Tokenize on ASCII whitespace to operate on class tokens
        let parts: Vec<&str> = inner.split_whitespace().collect();
        let originals: Vec<&str> = violation.originals.iter().map(|s| s.as_str()).collect();

        // Find insertion point for the replacement: the first occurrence among the originals
        let mut insert_at: Option<usize> = None;
        for (i, p) in parts.iter().enumerate() {
            if originals.iter().any(|o| o == p) {
                insert_at = Some(i);
                break;
            }
        }

        // Build the new list of class tokens:
        // - skip all originals (longhands)
        // - insert the replacement (if any) at the first original's position
        let mut new_parts: Vec<String> = Vec::with_capacity(parts.len().saturating_add(1));

        for (i, p) in parts.iter().enumerate() {
            if Some(i) == insert_at
                && let Some(repl) = &violation.replacement {
                    new_parts.push(repl.clone());
                }
            if !originals.iter().any(|o| o == p) {
                new_parts.push((*p).to_string());
            }
        }

        // If we didn't see any of the originals in the tokenized list (edge-case),
        // but we do have a replacement, append it at the end to avoid losing the fix.
        if insert_at.is_none()
            && let Some(repl) = &violation.replacement {
                new_parts.push(repl.clone());
            }

        let new_inner = new_parts.join(" ");

        // Recompose full token text with the original quotes (if present)
        let new_text = if content_start == 0 {
            new_inner.clone()
        } else {
            let mut s = String::with_capacity(new_inner.len() + 2);
            s.push(quote_prefix);
            s.push_str(&new_inner);
            s.push(quote_suffix);
            s
        };

        // If nothing changed, don't emit an action
        if new_text == original_str {
            return None;
        }

        // Create the new token and apply the mutation
        let new_token =
            biome_js_syntax::JsSyntaxToken::new_detached(old_token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token(old_token, new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace longhand Tailwind utilities with the shorthand." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
  pub AnyTailwindString = JsStringLiteralExpression | JsxString
}

impl AnyTailwindString {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        match self {
            Self::JsStringLiteralExpression(lit) => lit.inner_string_text(),
            Self::JsxString(jsx_str) => jsx_str.inner_string_text(),
        }
    }
}

fn extract_string_syntax_nodes(input: &AnyTailwindContainer) -> Vec<AnyTailwindString> {
    match input {
        AnyTailwindContainer::JsxAttribute(attr) => {
            // check if the attribute name is "className" or "class"
            let Ok(name) = attr.name_value_token() else {
                return Vec::new();
            };
            // TODO: configurable attribute names, good default set of names
            if name.text() != "className" && name.text() != "class" {
                return Vec::new();
            }
            // Extract string syntax nodes from JsxAttribute
            let Some(init) = attr.initializer() else {
                return Vec::new();
            };
            init.syntax()
                .descendants()
                .filter_map(AnyTailwindString::cast)
                .collect()
        }
        AnyTailwindContainer::JsCallExpression(call_expr) => {
            // only process if the call expression is a call to a function named "tw"
            let Some(func_name) = call_expr
                .callee()
                .ok()
                .and_then(|callee| callee.as_js_identifier_expression().cloned())
                .and_then(|ident| ident.name().ok())
                .and_then(|name| name.value_token().ok())
            else {
                return Vec::new();
            };
            // TODO: configurable function names, good default set of names
            if func_name.text() != "tw" && func_name.text() != "cn" && func_name.text() != "clsx" {
                return Vec::new();
            }

            // Extract string syntax nodes from JsCallExpression
            call_expr
                .syntax()
                .descendants()
                .filter_map(AnyTailwindString::cast)
                .collect()
        }
    }
}
