// Shared Tailwind shorthand analysis logic.
//
// This module contains the compressible groups, the analyzer that finds groups
// of longhand Tailwind utilities that can be replaced by a shorthand, and the
// violation struct returned by the analyzer.
//
// This code is intentionally standalone and does not depend on any JavaScript
// syntax nodes so it can be reused by multiple language-specific lint crates.

use biome_analyze::QueryMatch;
use biome_rowan::{AstNode, TextRange};
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::TwFullCandidate;
use std::collections::{HashMap, HashSet};

/// Define groups of compressable Tailwind CSS classes.
/// Each group is a slice of tuples where the first element is a slice
/// of base Tailwind class prefixes (as strings) and the second element is a slice
/// of replacement template strings.
pub static TW_COMPRESSABLES: &[&[(&[&str], &[&str])]] = &[
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
    ) -> Option<(&'a str, GroupKey<'a>)> {
        let variants = &source[full.variants().syntax().text_range()];
        let negative = full.negative_token().is_some();
        let important = full.excl_token().is_some();
        let candidate = full.candidate().ok()?;
        if let Some(func) = candidate.as_tw_functional_candidate() {
            let base = &source[func.base_token().ok()?.text_trimmed_range()];
            let value_node = func.value().ok()?;
            let value_range = value_node.syntax().text_range();
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
                Some((
                    b,
                    GroupKey {
                        variants,
                        negative,
                        important,
                        value: None,
                        modifier,
                    },
                ))
            } else {
                Some((
                    base,
                    GroupKey {
                        variants,
                        negative,
                        important,
                        value: Some(value),
                        modifier,
                    },
                ))
            }
        } else if let Some(st) = candidate.as_tw_static_candidate() {
            let base = &source[st.base_token().ok()?.text_trimmed_range()];
            Some((
                base,
                GroupKey {
                    variants,
                    negative,
                    important,
                    value: None,
                    modifier: None,
                },
            ))
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
    for (index, item) in candidates.into_iter().enumerate() {
        if let Some(full) = item.as_tw_full_candidate()
            && let Some((base, key)) = extract_parts(source, full)
        {
            let range = full.syntax().text_range();
            classes.push(TwClass {
                index,
                range,
                base,
                key,
            });
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
        // The analyzer should compress to the size shorthand preserving the value.
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
