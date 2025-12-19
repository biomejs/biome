use crate::JsRuleAction;
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
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::TwFullCandidate;
use std::collections::{HashMap, HashSet};

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
        sources: &[RuleSource::EslintBetterTailwindCss("enforce-shorthand-classes").inspired()],
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
        let string_nodes = extract_string_syntax_nodes(&node);
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
                .map(|s| s.as_str())
                .unwrap_or("this utility");
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
            if Some(i) == insert_at {
                if let Some(repl) = &violation.replacement {
                    new_parts.push(repl.clone());
                }
            }
            if !originals.iter().any(|o| o == p) {
                new_parts.push((*p).to_string());
            }
        }

        // If we didn't see any of the originals in the tokenized list (edge-case),
        // but we do have a replacement, append it at the end to avoid losing the fix.
        if insert_at.is_none() {
            if let Some(repl) = &violation.replacement {
                new_parts.push(repl.clone());
            }
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
            AnyTailwindString::JsStringLiteralExpression(lit) => lit.inner_string_text(),
            AnyTailwindString::JsxString(jsx_str) => jsx_str.inner_string_text(),
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
                .flat_map(|node| AnyTailwindString::cast(node))
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
                .flat_map(|node| AnyTailwindString::cast(node))
                .collect()
        }
    }
}

/// Define groups of compressable Tailwind CSS classes.
/// Each group is a slice of tuples where the first element is a slice
/// of base Tailwind class prefixes (as strings) and the second element is a slice
/// of replacement template strings.
static TW_COMPRESSABLES: &[&[(&[&str], &[&str])]] = &[
    // size
    &[(&["w", "h"], &["size"])],
    // margin shorthands
    &[
        (&["ml", "mr", "mt", "mb"], &["m"]),
        (&["mx", "my"], &["m"]),
        (&["ms", "me"], &["mx"]),
        (&["ml", "mr"], &["mx"]),
        (&["mt", "mb"], &["my"]),
    ],
    // padding shorthands
    &[
        (&["pl", "pr", "pt", "pb"], &["p"]),
        (&["px", "py"], &["p"]),
        (&["ps", "pe"], &["px"]),
        (&["pl", "pr"], &["px"]),
        (&["pt", "pb"], &["py"]),
    ],
    // border shorthands
    &[
        (
            &["border-t", "border-b", "border-l", "border-r"],
            &["border"],
        ),
        (&["border-x", "border-y"], &["border"]),
        (&["border-s", "border-e"], &["border-x"]),
        (&["border-l", "border-r"], &["border-x"]),
        (&["border-t", "border-b"], &["border-y"]),
    ],
    // border-spacing
    &[(
        &["border-spacing-x", "border-spacing-y"],
        &["border-spacing"],
    )],
    // rounded corners
    &[
        (
            &["rounded-tl", "rounded-tr", "rounded-bl", "rounded-br"],
            &["rounded"],
        ),
        (&["rounded-t", "rounded-b"], &["rounded"]),
        (&["rounded-l", "rounded-r"], &["rounded"]),
        (&["rounded-tl", "rounded-tr"], &["rounded-t"]),
        (&["rounded-bl", "rounded-br"], &["rounded-b"]),
        (&["rounded-tl", "rounded-bl"], &["rounded-l"]),
        (&["rounded-tr", "rounded-br"], &["rounded-r"]),
    ],
    // scroll margin
    &[
        (
            &["scroll-mt", "scroll-mb", "scroll-ml", "scroll-mr"],
            &["scroll-m"],
        ),
        (&["scroll-mx", "scroll-my"], &["scroll-m"]),
        (&["scroll-ms", "scroll-me"], &["scroll-mx"]),
        (&["scroll-ml", "scroll-mr"], &["scroll-mx"]),
        (&["scroll-mt", "scroll-mb"], &["scroll-my"]),
    ],
    // scroll padding
    &[
        (
            &["scroll-pt", "scroll-pb", "scroll-pl", "scroll-pr"],
            &["scroll-p"],
        ),
        (&["scroll-px", "scroll-py"], &["scroll-p"]),
        (&["scroll-pl", "scroll-pr"], &["scroll-px"]),
        (&["scroll-ps", "scroll-pe"], &["scroll-px"]),
        (&["scroll-pt", "scroll-pb"], &["scroll-py"]),
    ],
    // inset / position
    &[
        (&["top", "right", "bottom", "left"], &["inset"]),
        (&["right", "left"], &["inset-x"]),
        (&["bottom", "top"], &["inset-y"]),
        (&["inset-x", "inset-y"], &["inset"]),
    ],
    // divide
    &[(&["divide-x", "divide-y"], &["divide"])],
    // space
    &[(&["space-x", "space-y"], &["space"])],
    // gap
    &[(&["gap-x", "gap-y"], &["gap"])],
    // translate
    &[(&["translate-x", "translate-y"], &["translate"])],
    // rotate
    &[(&["rotate-x", "rotate-y"], &["rotate"])],
    // skew
    &[(&["skew-x", "skew-y"], &["skew"])],
    // scale (including 3d)
    &[
        (&["scale-x", "scale-y", "scale-z"], &["scale", "scale-3d"]),
        (&["scale-x", "scale-y"], &["scale"]),
    ],
    // place-* helpers
    &[
        (&["content", "justify-content"], &["place-content"]),
        (&["items", "justify-items"], &["place-items"]),
        (&["self", "justify-self"], &["place-self"]),
    ],
    // truncate helpers
    &[(
        &["overflow-hidden", "text-ellipsis", "whitespace-nowrap"],
        &["truncate"],
    )],
];

#[derive(Debug, Clone)]
pub struct TailwindShorthandViolation {
    pub originals: Vec<String>,
    pub original_ranges: Vec<TextRange>,
    pub replacement: Option<String>,
}

/// Analyze a Tailwind class list string, detect opportunities to replace multiple
/// longhand utilities with a single shorthand, and return the list of violations (originals + optional replacement)
pub fn analyze_tailwind_shorthand(source: &str) -> Vec<TailwindShorthandViolation> {
    let parse = parse_tailwind(source);
    let root = parse.tree();
    let candidates = root.candidates();

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct GroupKey<'a> {
        variants: &'a str,
        negative: bool,
        important: bool,
        value: Option<&'a str>,
        modifier: Option<&'a str>,
    }

    #[derive(Debug, Clone)]
    struct TwClass<'a> {
        index: usize,
        range: TextRange,
        base: &'a str,
        key: GroupKey<'a>,
    }

    fn extract_parts<'a>(
        source: &'a str,
        full: &TwFullCandidate,
    ) -> Option<(
        &'a str,
        bool,
        bool,
        &'a str,
        Option<&'a str>,
        Option<&'a str>,
    )> {
        let variants = &source[full.variants().syntax().text_range()];
        let negative = full.negative_token().is_some();
        let important = full.excl_token().is_some();
        let candidate = full.candidate().ok()?;
        if let Some(func) = candidate.as_tw_functional_candidate() {
            let base = &source[func.base_token().ok()?.text_range()];
            let value_node = func.value().ok()?;
            let value_range = match value_node {
                biome_tailwind_syntax::AnyTwValue::TwArbitraryValue(n) => n.syntax().text_range(),
                biome_tailwind_syntax::AnyTwValue::TwBogusValue(n) => n.syntax().text_range(),
                biome_tailwind_syntax::AnyTwValue::TwCssVariableValue(n) => n.syntax().text_range(),
                biome_tailwind_syntax::AnyTwValue::TwNamedValue(n) => n.syntax().text_range(),
            };
            let value = &source[value_range];
            let modifier = func.modifier().map(|m| {
                let range = match m {
                    biome_tailwind_syntax::AnyTwModifier::TwModifier(n) => n.syntax().text_range(),
                    biome_tailwind_syntax::AnyTwModifier::TwBogusModifier(n) => {
                        n.syntax().text_range()
                    }
                };
                &source[range]
            });

            // HACK: Special-case certain helpers that are parsed as functional candidates but should
            // behave like static bases for compression (e.g., truncate helpers).
            // Treat them as static bases (no value) so they can match TW_COMPRESSABLES patterns.
            let combined: Option<&'static str> = match (base, value) {
                ("overflow", "hidden") => Some("overflow-hidden"),
                ("text", "ellipsis") => Some("text-ellipsis"),
                ("whitespace", "nowrap") => Some("whitespace-nowrap"),
                _ => None,
            };

            if let Some(b) = combined {
                Some((variants, negative, important, b, None, modifier))
            } else {
                Some((variants, negative, important, base, Some(value), modifier))
            }
        } else if let Some(st) = candidate.as_tw_static_candidate() {
            let base = &source[st.base_token().ok()?.text_range()];
            Some((variants, negative, important, base, None, None))
        } else {
            None
        }
    }

    fn build_from_key(key: &GroupKey<'_>, replacement_base: &str) -> String {
        let mut out = String::new();
        if !key.variants.is_empty() {
            out.push_str(key.variants);
            if !out.ends_with(':') {
                out.push(':');
            }
        }
        if key.negative {
            out.push('-');
        }
        out.push_str(replacement_base);
        if let Some(v) = key.value {
            out.push('-');
            out.push_str(v);
        }
        if let Some(m) = key.modifier {
            out.push_str(m);
        }
        if key.important {
            out.push('!');
        }
        out
    }

    let mut classes = Vec::new();
    for (idx, item) in candidates.into_iter().enumerate() {
        if let Ok(any) = item {
            if let Some(full) = any.as_tw_full_candidate() {
                if let Some((variants, negative, important, base, value, modifier)) =
                    extract_parts(source, &full)
                {
                    let key = GroupKey {
                        variants,
                        negative,
                        important,
                        value,
                        modifier,
                    };
                    let range = full.syntax().text_range();
                    classes.push(TwClass {
                        index: idx,
                        range,
                        base,
                        key,
                    });
                }
            }
        }
    }

    // Group classes by key, then by base
    let mut groups: HashMap<GroupKey<'_>, HashMap<&str, usize>> = HashMap::new();
    for class in &classes {
        groups
            .entry(class.key.clone())
            .or_default()
            .insert(class.base, class.index);
    }

    let mut used: HashSet<usize> = HashSet::new();
    let mut violations: Vec<TailwindShorthandViolation> = Vec::new();
    // removed unused inserts

    for pattern_group in TW_COMPRESSABLES {
        for (required_bases, replacement_bases) in *pattern_group {
            for (key, base_map) in &groups {
                // Ensure all required bases exist and are unused
                let mut indices: Vec<usize> = Vec::new();
                let mut all_present = true;
                for &b in *required_bases {
                    match base_map.get(b) {
                        Some(&i) if !used.contains(&i) => indices.push(i),
                        _ => {
                            all_present = false;
                            break;
                        }
                    }
                }
                if !all_present {
                    continue;
                }

                // Choose a replacement that doesn't already exist in the same key
                let mut replacement: Option<String> = None;
                for &rb in *replacement_bases {
                    if base_map.get(rb).is_none() {
                        replacement = Some(build_from_key(key, rb));
                        break;
                    }
                }

                // Record violation
                let mut originals = Vec::new();
                let mut ranges = Vec::new();
                for &i in &indices {
                    let c = &classes[i];
                    originals.push(source[c.range].to_string());
                    ranges.push(c.range);
                    used.insert(i);
                }

                violations.push(TailwindShorthandViolation {
                    originals,
                    original_ranges: ranges,
                    replacement: replacement.clone(),
                });

                // no scheduling; insertion handled by fixer elsewhere
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_ml_mr_to_mx() {
        let input = "ml-2 mr-2";
        let violations = analyze_tailwind_shorthand(input);
        // Expect a single replacement to mx-2
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        // originals should contain ml-2 and mr-2 (order may vary)
        assert!(v.originals.iter().any(|s| s == "ml-2"));
        assert!(v.originals.iter().any(|s| s == "mr-2"));
        assert_eq!(v.replacement.as_deref(), Some("mx-2"));
    }

    #[test]
    fn compress_w_h_to_size_with_variants() {
        let input = "hover:w-4 hover:h-4";
        let violations = analyze_tailwind_shorthand(input);
        // The analyzer should compress to the size shorthand preserving the variant.
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s.contains("w-4")));
        assert!(v.originals.iter().any(|s| s.contains("h-4")));
        assert_eq!(v.replacement.as_deref(), Some("hover:size-4"));
    }

    #[test]
    fn compress_w_h_to_size_with_arbitrary_values() {
        let input = "w-[10px] h-[10px]";
        let violations = analyze_tailwind_shorthand(input);
        // The analyzer should compress to the size shorthand preserving the variant.
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s.contains("w-[10px]")));
        assert!(v.originals.iter().any(|s| s.contains("h-[10px]")));
        assert_eq!(v.replacement.as_deref(), Some("size-[10px]"));
    }

    // Variants + negatives + important should be preserved on the replacement.
    #[test]
    fn compress_with_variants_negative_and_important() {
        let input = "md:hover:-mt-1! md:hover:-mb-1!";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "md:hover:-mt-1!"));
        assert!(v.originals.iter().any(|s| s == "md:hover:-mb-1!"));
        assert_eq!(v.replacement.as_deref(), Some("md:hover:-my-1!"));
    }

    // Functional values should be preserved.
    #[test]
    fn compress_functional_values() {
        let input = "w-[10px] h-[10px]";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "w-[10px]"));
        assert!(v.originals.iter().any(|s| s == "h-[10px]"));
        assert_eq!(v.replacement.as_deref(), Some("size-[10px]"));
    }

    // Padding shorthands: pl/pr -> px, pt/pb -> py, and px/py -> p.
    #[test]
    fn compress_pl_pr_to_px() {
        let input = "pl-2 pr-2";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "pl-2"));
        assert!(v.originals.iter().any(|s| s == "pr-2"));
        assert_eq!(v.replacement.as_deref(), Some("px-2"));
    }

    #[test]
    fn compress_full_padding_to_p() {
        let input = "pl-2 pr-2 pt-2 pb-2";
        let violations = analyze_tailwind_shorthand(input);
        // Should directly compress the four sides into p-2
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("p-2"));
        assert!(v.originals.iter().any(|s| s == "pl-2"));
        assert!(v.originals.iter().any(|s| s == "pr-2"));
        assert!(v.originals.iter().any(|s| s == "pt-2"));
        assert!(v.originals.iter().any(|s| s == "pb-2"));
    }

    // Logical padding shorthands: ps/pe -> px.
    #[test]
    fn compress_ps_pe_to_px() {
        let input = "ps-3 pe-3";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("px-3"));
        assert!(v.originals.iter().any(|s| s == "ps-3"));
        assert!(v.originals.iter().any(|s| s == "pe-3"));
    }

    // Border shorthands.
    #[test]
    fn compress_border_lr_to_border_x() {
        let input = "border-l border-r";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "border-l"));
        assert!(v.originals.iter().any(|s| s == "border-r"));
        assert_eq!(v.replacement.as_deref(), Some("border-x"));
    }

    #[test]
    fn compress_border_x_y_to_border() {
        let input = "border-x border-y";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "border-x"));
        assert!(v.originals.iter().any(|s| s == "border-y"));
        assert_eq!(v.replacement.as_deref(), Some("border"));
    }

    // Rounded corners shorthands.
    #[test]
    fn compress_rounded_corners_top() {
        let input = "rounded-tl rounded-tr";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "rounded-tl"));
        assert!(v.originals.iter().any(|s| s == "rounded-tr"));
        assert_eq!(v.replacement.as_deref(), Some("rounded-t"));
    }

    #[test]
    fn compress_rounded_corners_top_named() {
        let input = "rounded-tl-md rounded-tr-md";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "rounded-tl-md"));
        assert!(v.originals.iter().any(|s| s == "rounded-tr-md"));
        assert_eq!(v.replacement.as_deref(), Some("rounded-t-md"));
    }

    // Scroll margin and padding shorthands.
    #[test]
    fn compress_scroll_margin_lr_to_mx() {
        let input = "scroll-ml-3 scroll-mr-3";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("scroll-mx-3"));
        assert!(v.originals.iter().any(|s| s == "scroll-ml-3"));
        assert!(v.originals.iter().any(|s| s == "scroll-mr-3"));
    }

    #[test]
    fn compress_scroll_mx_my_to_m() {
        let input = "scroll-mx-3 scroll-my-3";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("scroll-m-3"));
        assert!(v.originals.iter().any(|s| s == "scroll-mx-3"));
        assert!(v.originals.iter().any(|s| s == "scroll-my-3"));
    }

    // Border spacing shorthand.
    #[test]
    fn compress_border_spacing_xy_to_border_spacing() {
        let input = "border-spacing-x-2 border-spacing-y-2";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("border-spacing-2"));
        assert!(v.originals.iter().any(|s| s == "border-spacing-x-2"));
        assert!(v.originals.iter().any(|s| s == "border-spacing-y-2"));
    }

    // Inset shorthand from inset-x and inset-y.
    #[test]
    fn compress_inset_xy_to_inset() {
        let input = "inset-x-4 inset-y-4";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("inset-4"));
        assert!(v.originals.iter().any(|s| s == "inset-x-4"));
        assert!(v.originals.iter().any(|s| s == "inset-y-4"));
    }

    // Gap, Space, Divide shorthands.
    #[test]
    fn compress_gap_xy_to_gap() {
        let input = "gap-x-2 gap-y-2";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("gap-2"));
        assert!(v.originals.iter().any(|s| s == "gap-x-2"));
        assert!(v.originals.iter().any(|s| s == "gap-y-2"));
    }

    #[test]
    fn compress_space_xy_to_space() {
        let input = "space-x-1 space-y-1";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("space-1"));
        assert!(v.originals.iter().any(|s| s == "space-x-1"));
        assert!(v.originals.iter().any(|s| s == "space-y-1"));
    }

    #[test]
    fn compress_divide_xy_to_divide() {
        let input = "divide-x-2 divide-y-2";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("divide-2"));
        assert!(v.originals.iter().any(|s| s == "divide-x-2"));
        assert!(v.originals.iter().any(|s| s == "divide-y-2"));
    }

    // Transform shorthands.
    #[test]
    fn compress_translate_xy_to_translate() {
        let input = "translate-x-3 translate-y-3";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("translate-3"));
        assert!(v.originals.iter().any(|s| s == "translate-x-3"));
        assert!(v.originals.iter().any(|s| s == "translate-y-3"));
    }

    #[test]
    fn compress_skew_xy_to_skew() {
        let input = "skew-x-2 skew-y-2";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("skew-2"));
        assert!(v.originals.iter().any(|s| s == "skew-x-2"));
        assert!(v.originals.iter().any(|s| s == "skew-y-2"));
    }

    #[test]
    fn compress_scale_xy_to_scale() {
        let input = "scale-x-110 scale-y-110";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        // For 2D scale, we expect "scale-110"
        assert_eq!(v.replacement.as_deref(), Some("scale-110"));
        assert!(v.originals.iter().any(|s| s == "scale-x-110"));
        assert!(v.originals.iter().any(|s| s == "scale-y-110"));
    }

    // Truncate helpers compress to truncate.
    #[test]
    fn compress_truncate_helpers() {
        let input = "overflow-hidden text-ellipsis whitespace-nowrap";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert_eq!(v.replacement.as_deref(), Some("truncate"));
        assert!(v.originals.iter().any(|s| s == "overflow-hidden"));
        assert!(v.originals.iter().any(|s| s == "text-ellipsis"));
        assert!(v.originals.iter().any(|s| s == "whitespace-nowrap"));
    }

    // Non-compression cases.
    #[test]
    fn do_not_compress_different_values() {
        let input = "ml-2 mr-3";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn do_not_compress_different_variants() {
        let input = "hover:ml-2 mr-2";
        let violations = analyze_tailwind_shorthand(input);
        assert_eq!(violations.len(), 0);
    }

    // If the shorthand already exists, we should not suggest a replacement
    // (but we still detect the redundant longhands).
    #[test]
    fn no_replacement_when_shorthand_already_present() {
        let input = "mx-2 ml-2 mr-2";
        let violations = analyze_tailwind_shorthand(input);
        // We expect one violation grouping ml-2 and mr-2, but replacement is None because mx-2 already exists.
        assert_eq!(violations.len(), 1);
        let v = &violations[0];
        assert!(v.originals.iter().any(|s| s == "ml-2"));
        assert!(v.originals.iter().any(|s| s == "mr-2"));
        assert_eq!(v.replacement, None);
    }
}
