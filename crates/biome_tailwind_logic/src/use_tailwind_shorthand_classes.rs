use std::hash::Hash;

use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, Direction, TextRange, TokenText,
    WalkEvent,
};
use biome_tailwind_factory::make;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwModifier, AnyTwValue, TailwindLanguage, TailwindSyntaxKind,
    TailwindSyntaxNode, TailwindSyntaxToken, TwCandidateList, TwFullCandidate, TwRoot,
    TwVariantList, is_node_equal,
};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct TailwindShorthandViolation {
    pub uncompressed_nodes: Vec<TwFullCandidate>,
    pub replacement_bases: &'static [&'static str],
    /// Replacement bases that represent static classes and must discard the
    /// functional value from the candidate they replace.
    pub static_replacement_bases: &'static [&'static str],
    existing_replacement: Option<TwFullCandidate>,
}

#[derive(Debug, Clone, Eq)]
struct GroupKey {
    variants: TwVariantList,
    negative: bool,
    important: bool,
    value: Option<AnyTwValue>,
    modifier: Option<AnyTwModifier>,
}

#[derive(Debug, Default)]
struct GroupCandidates {
    candidates: Vec<TwFullCandidate>,
    by_base: FxHashMap<TokenText, Vec<usize>>,
}

impl PartialEq for GroupKey {
    fn eq(&self, other: &Self) -> bool {
        let value_equal = match (&self.value, &other.value) {
            (Some(a), Some(b)) => is_node_equal(a.syntax(), b.syntax()),
            (None, None) => true,
            _ => return false,
        };
        let modifier_equal = match (&self.modifier, &other.modifier) {
            (Some(a), Some(b)) => is_node_equal(a.syntax(), b.syntax()),
            (None, None) => true,
            _ => return false,
        };

        self.negative == other.negative
            && self.important == other.important
            && value_equal
            && modifier_equal
            && is_node_equal(self.variants.syntax(), other.variants.syntax())
    }
}

impl std::hash::Hash for GroupKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_node(self.variants.syntax(), state);
        self.negative.hash(state);
        self.important.hash(state);
        if let Some(ref value) = self.value {
            hash_node(value.syntax(), state);
        }
        if let Some(ref modifier) = self.modifier {
            hash_node(modifier.syntax(), state);
        }
    }
}

fn hash_node<H: std::hash::Hasher>(node: &TailwindSyntaxNode, state: &mut H) {
    for event in node.preorder_with_tokens(Direction::Next) {
        match event {
            WalkEvent::Enter(element) => {
                element.kind().hash(state);
                if let Some(token) = element.as_token() {
                    token.text_trimmed().hash(state);
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }
}

/// Returns `true` if `full` is a functional candidate whose base token matches
/// `base` and whose value text matches `value`, without allocating any strings.
fn candidate_matches(full: &TwFullCandidate, base: &str, value: &str) -> bool {
    match full.candidate().ok() {
        Some(AnyTwCandidate::TwFunctionalCandidate(func)) => {
            func.base_token()
                .ok()
                .is_some_and(|t| t.text_trimmed() == base)
                && func
                    .value()
                    .ok()
                    .is_some_and(|v| v.syntax().text_trimmed() == value)
        }
        Some(AnyTwCandidate::TwStaticCandidate(st)) => {
            // Static candidates have no value part; match when `value` is empty.
            value.is_empty()
                && st
                    .base_token()
                    .ok()
                    .is_some_and(|t| t.text_trimmed() == base)
        }
        _ => false,
    }
}

fn find_static_class_candidate(
    candidates: &TwCandidateList,
    base: &str,
    variants: &TwVariantList,
    important: bool,
) -> Option<TwFullCandidate> {
    candidates
        .iter()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
        .find(|candidate| {
            (candidate_matches(candidate, base, "")
                || base
                    .rsplit_once('-')
                    .is_some_and(|(base, value)| candidate_matches(candidate, base, value)))
                && candidate.negative_token().is_none()
                && candidate.excl_token().is_some() == important
                && is_node_equal(candidate.variants().syntax(), variants.syntax())
        })
}

/// Special-case detection for `overflow-hidden text-ellipsis whitespace-nowrap` → `truncate`.
///
/// These three classes cannot be handled by the general value-grouping logic
/// because each has a different value (`hidden`, `ellipsis`, `nowrap`), so
/// they never end up in the same group.
fn check_truncate_shorthand(candidates: &TwCandidateList) -> Vec<TailwindShorthandViolation> {
    /// Each tuple is `(base, value)` for a functional candidate that forms part
    /// of the `truncate` shorthand.  Using `(base, value)` pairs avoids any
    /// string allocation during matching.
    const TRUNCATE_PARTS: &[(&str, &str)] = &[
        ("overflow", "hidden"),
        ("text", "ellipsis"),
        ("whitespace", "nowrap"),
    ];

    let mut violations = Vec::new();
    let mut used_candidates = FxHashSet::default();
    let (first_base, first_value) = TRUNCATE_PARTS[0];
    for first in candidates
        .iter()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
    {
        if !candidate_matches(&first, first_base, first_value)
            || first.negative_token().is_some()
            || used_candidates.contains(&first.range())
        {
            continue;
        }
        let first_variants = first.variants();
        let important = first.excl_token().is_some();
        let mut matched = Vec::with_capacity(TRUNCATE_PARTS.len());
        for &(base, value) in TRUNCATE_PARTS {
            let Some(candidate) = candidates
                .iter()
                .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
                .find(|candidate| {
                    candidate_matches(candidate, base, value)
                        && candidate.negative_token().is_none()
                        && candidate.excl_token().is_some() == important
                        && !used_candidates.contains(&candidate.range())
                        && is_node_equal(candidate.variants().syntax(), first_variants.syntax())
                })
            else {
                matched.clear();
                break;
            };
            matched.push(candidate);
        }
        if matched.len() != TRUNCATE_PARTS.len() {
            continue;
        }

        used_candidates.extend(matched.iter().map(AstNode::range));
        violations.push(TailwindShorthandViolation {
            uncompressed_nodes: matched,
            replacement_bases: &["truncate"],
            static_replacement_bases: &["truncate"],
            existing_replacement: find_static_class_candidate(
                candidates,
                "truncate",
                &first_variants,
                important,
            ),
        });
    }

    violations
}

fn find_candidates_for_base(
    required_base: &'static str,
    group: &GroupCandidates,
    patterns: &'static [(&'static [&'static str], &'static [&'static str])],
    used_candidates: &FxHashSet<TextRange>,
    visiting: &mut Vec<&'static str>,
) -> Option<Vec<TwFullCandidate>> {
    if let Some(candidate) = group.by_base.get(required_base).and_then(|indexes| {
        indexes.iter().find_map(|&index| {
            let candidate = &group.candidates[index];
            (!used_candidates.contains(&candidate.range())).then(|| candidate.clone())
        })
    }) {
        return Some(vec![candidate]);
    }

    if visiting.contains(&required_base) {
        return None;
    }
    visiting.push(required_base);
    let result = patterns
        .iter()
        .find_map(|(required_bases, replacement_bases)| {
            if *replacement_bases != [required_base] {
                return None;
            }
            let mut matched = Vec::new();
            let mut unavailable = used_candidates.clone();
            for &base in *required_bases {
                let candidates =
                    find_candidates_for_base(base, group, patterns, &unavailable, visiting)?;
                unavailable.extend(candidates.iter().map(AstNode::range));
                matched.extend(candidates);
            }
            Some(matched)
        });
    visiting.pop();
    result
}

pub fn analyze_tailwind_shorthand(candidates: &TwCandidateList) -> Vec<TailwindShorthandViolation> {
    fn extract_key_and_base(full: &TwFullCandidate) -> Option<(GroupKey, TokenText)> {
        let variants = full.variants();
        let negative = full.negative_token().is_some();
        let important = full.excl_token().is_some();
        let candidate = full.candidate().ok()?;
        match candidate {
            AnyTwCandidate::TwFunctionalCandidate(func) => {
                let base = func.base_token().ok()?.token_text_trimmed();
                let value = func.value().ok()?;
                let modifier = func.modifier();
                Some((
                    GroupKey {
                        variants,
                        negative,
                        important,
                        value: Some(value),
                        modifier,
                    },
                    base,
                ))
            }
            AnyTwCandidate::TwStaticCandidate(st) => {
                let base = st.base_token().ok()?.token_text_trimmed();
                Some((
                    GroupKey {
                        variants,
                        negative,
                        important,
                        value: None,
                        modifier: None,
                    },
                    base,
                ))
            }
            _ => None,
        }
    }

    if candidates.len() < 2 {
        // can't possibly compress anything if there isn't 2 candidates
        return vec![];
    }

    // build a map to make candidate lookups faster
    let mut groups: FxHashMap<GroupKey, GroupCandidates> = FxHashMap::default();
    for candidate in candidates
        .iter()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
    {
        let Some((key, base)) = extract_key_and_base(&candidate) else {
            continue;
        };
        let group = groups.entry(key).or_default();
        let candidate_index = group.candidates.len();
        group.candidates.push(candidate);
        group.by_base.entry(base).or_default().push(candidate_index);
    }

    let mut violations: Vec<TailwindShorthandViolation> = Vec::new();

    for pattern_group in TW_COMPRESSABLES {
        for (key, group) in &groups {
            let group_candidates = &group.candidates;
            let candidates_by_base = &group.by_base;
            let mut used_candidates = FxHashSet::default();

            for (required_bases, replacement_bases) in *pattern_group {
                // Special case: `w`/`h` → `size` must skip values that `size` doesn't support
                // (e.g. `screen`, container sizes like `xs`/`sm`/`md`/...).
                let is_size_pattern = *required_bases == ["w", "h"];

                if group_candidates.len() < required_bases.len() {
                    // Not enough candidates to match the required bases
                    continue;
                }

                // Skip if the value is a keyword that `size` doesn't support
                if is_size_pattern
                    && let Some(value) = &key.value
                    && let Some(value) = value.as_tw_named_value()
                    && let Some(value) = value.value_token().ok()
                    && SIZE_BLOCKED_VALUES.contains(value.text_trimmed())
                {
                    continue;
                }

                // Check if all required bases are present
                let mut found_all = true;
                let mut flagged_candidates = Vec::with_capacity(required_bases.len());
                for &rb in *required_bases {
                    let Some(candidates) = find_candidates_for_base(
                        rb,
                        group,
                        pattern_group,
                        &used_candidates,
                        &mut Vec::new(),
                    ) else {
                        found_all = false;
                        break;
                    };
                    used_candidates.extend(candidates.iter().map(AstNode::range));
                    flagged_candidates.extend(candidates);
                }
                if !found_all {
                    for candidate in &flagged_candidates {
                        used_candidates.remove(&candidate.range());
                    }
                    continue;
                }
                debug_assert!(
                    flagged_candidates.len() >= required_bases.len(),
                    "should have found a candidate for each required base -- \n Group Key: {:#?}\n{} candidates: {:?}, required_bases: {:?}, replacement_bases: {:?}\noriginal strings: {:?}",
                    key,
                    flagged_candidates.len(),
                    flagged_candidates
                        .iter()
                        .map(|c| c.candidate().ok().and_then(|candidate| {
                            if let Some(func) = candidate.as_tw_functional_candidate() {
                                func.base_token().ok().map(|t| t.text().to_string())
                            } else if let Some(st) = candidate.as_tw_static_candidate() {
                                st.base_token().ok().map(|t| t.text().to_string())
                            } else {
                                None
                            }
                        }))
                        .collect::<Vec<_>>(),
                    required_bases,
                    replacement_bases,
                    flagged_candidates
                        .iter()
                        .map(|c| c.syntax().to_string())
                        .collect::<Vec<_>>()
                );
                let is_three_axis_scale_pattern =
                    *required_bases == ["scale-x", "scale-y", "scale-z"];
                let has_static_replacement = is_three_axis_scale_pattern
                    && find_static_class_candidate(
                        candidates,
                        "scale-3d",
                        &key.variants,
                        key.important,
                    )
                    .is_some();
                let replacement_bases: &[&str] = if has_static_replacement {
                    &["scale"]
                } else {
                    replacement_bases
                };
                let existing_replacement = replacement_bases.iter().find_map(|replacement_base| {
                    candidates_by_base
                        .get(*replacement_base)
                        .and_then(|candidate_indexes| candidate_indexes.first())
                        .map(|&index| group_candidates[index].clone())
                });

                violations.push(TailwindShorthandViolation {
                    uncompressed_nodes: flagged_candidates,
                    replacement_bases,
                    static_replacement_bases: if is_three_axis_scale_pattern
                        && !has_static_replacement
                    {
                        &["scale-3d"]
                    } else {
                        &[]
                    },
                    existing_replacement,
                });
            }
        }
    }

    // Special case: `overflow-hidden text-ellipsis whitespace-nowrap` → `truncate`.
    // These classes cannot be handled by the general value-grouping logic because
    // each has a different value, so they never end up in the same group.
    violations.extend(check_truncate_shorthand(candidates));

    violations
}

/// Create a batch mutation that fixes a single shorthand violation.
pub fn auto_fix(
    root: &TwRoot,
    state: &TailwindShorthandViolation,
) -> Option<BatchMutation<TailwindLanguage>> {
    let mut mutation = root.clone().begin();
    apply_auto_fix(&mut mutation, state)?;

    Some(mutation)
}

pub fn auto_fix_all(
    root: &TwRoot,
    states: &[TailwindShorthandViolation],
) -> Option<BatchMutation<TailwindLanguage>> {
    let mut mutation = root.clone().begin();
    for state in states {
        apply_auto_fix(&mut mutation, state)?;
    }

    Some(mutation)
}

fn apply_auto_fix(
    mutation: &mut BatchMutation<TailwindLanguage>,
    state: &TailwindShorthandViolation,
) -> Option<()> {
    let mut old_candidates = state.uncompressed_nodes.clone();
    old_candidates.extend(state.existing_replacement.iter().cloned());
    old_candidates.sort_unstable_by_key(AstNode::range);
    let replacement_start = old_candidates
        .len()
        .checked_sub(state.replacement_bases.len())?;
    let replacements = old_candidates.split_off(replacement_start);
    for (to_modify, replacement_base) in replacements
        .into_iter()
        .zip(state.replacement_bases.iter().copied())
    {
        if state.static_replacement_bases.contains(&replacement_base) {
            // The replacement is a static class (e.g. `truncate`), but the candidate
            // being replaced is a functional candidate (e.g. `whitespace-nowrap`).
            // We must replace the entire TwFullCandidate node, not just the base token,
            // because changing the node kind from functional to static requires
            // rebuilding the whole subtree.
            let new_base_token = TailwindSyntaxToken::new_detached(
                TailwindSyntaxKind::TW_BASE,
                replacement_base,
                [],
                [],
            );
            let new_static = make::tw_static_candidate(new_base_token).build();
            let mut new_full = make::tw_full_candidate(
                to_modify.variants(),
                AnyTwCandidate::TwStaticCandidate(new_static),
            );
            if let Some(legacy_important) = to_modify.legacy_important_token() {
                new_full = new_full.with_legacy_important_token(legacy_important);
            }
            if let Some(excl_token) = to_modify.excl_token() {
                new_full = new_full.with_excl_token(excl_token);
            }
            mutation.replace_node(to_modify, new_full.build());
        } else {
            match to_modify.candidate().ok()? {
                AnyTwCandidate::TwFunctionalCandidate(func) => {
                    // Functional candidate: replace just the base token.
                    // The `-value` part (e.g. `-4` in `border-x-4`) stays intact.
                    let base_token = func.base_token().ok()?;
                    mutation.replace_token(
                        base_token,
                        TailwindSyntaxToken::new_detached(
                            TailwindSyntaxKind::TW_BASE,
                            replacement_base,
                            [],
                            [],
                        ),
                    );
                }
                AnyTwCandidate::TwStaticCandidate(old_static) => {
                    // Static candidate (e.g. bare `border-x` with no value):
                    // replace the whole node since there is no value slot to keep.
                    // Any `/modifier` on the old candidate is carried over.
                    let new_base = TailwindSyntaxToken::new_detached(
                        TailwindSyntaxKind::TW_BASE,
                        replacement_base,
                        [],
                        [],
                    );
                    let mut new_static = make::tw_static_candidate(new_base);
                    if let Some(modifier) = old_static.modifier() {
                        new_static = new_static.with_modifier(modifier);
                    }
                    let new_static = new_static.build();
                    let mut new_full = make::tw_full_candidate(
                        to_modify.variants(),
                        AnyTwCandidate::TwStaticCandidate(new_static),
                    );
                    if let Some(legacy_important) = to_modify.legacy_important_token() {
                        new_full = new_full.with_legacy_important_token(legacy_important);
                    }
                    if let Some(excl_token) = to_modify.excl_token() {
                        new_full = new_full.with_excl_token(excl_token);
                    }
                    mutation.replace_node(to_modify, new_full.build());
                }
                _ => return None,
            }
        }
    }

    for to_remove in old_candidates {
        mutation.remove_node(to_remove);
    }

    Some(())
}

pub static TW_COMPRESSABLES: &[&[(&[&str], &[&str])]] = &[
    &[(&["w", "h"], &["size"])],
    &[
        (&["ml", "mr", "mt", "mb"], &["m"]),
        (&["mx", "my"], &["m"]),
        (&["ms", "me"], &["mx"]),
        (&["ml", "mr"], &["mx"]),
        (&["mt", "mb"], &["my"]),
    ],
    &[
        (&["pl", "pr", "pt", "pb"], &["p"]),
        (&["px", "py"], &["p"]),
        (&["ps", "pe"], &["px"]),
        (&["pl", "pr"], &["px"]),
        (&["pt", "pb"], &["py"]),
    ],
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
    &[(
        &["border-spacing-x", "border-spacing-y"],
        &["border-spacing"],
    )],
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
    &[
        (&["top", "right", "bottom", "left"], &["inset"]),
        (&["right", "left"], &["inset-x"]),
        (&["bottom", "top"], &["inset-y"]),
        (&["inset-x", "inset-y"], &["inset"]),
    ],
    &[(&["divide-x", "divide-y"], &["divide"])],
    &[(&["gap-x", "gap-y"], &["gap"])],
    &[(&["translate-x", "translate-y"], &["translate"])],
    &[(&["rotate-x", "rotate-y"], &["rotate"])],
    &[(&["skew-x", "skew-y"], &["skew"])],
    &[
        (&["scale-x", "scale-y", "scale-z"], &["scale", "scale-3d"]),
        (&["scale-x", "scale-y"], &["scale"]),
    ],
    &[
        (&["content", "justify"], &["place-content"]),
        (&["items", "justify-items"], &["place-items"]),
        (&["self", "justify-self"], &["place-self"]),
    ],
];

/// Values supported by `w`/`h` but NOT by `size`.
/// These keyword values exist as width/height utilities but have no `size-*` equivalent,
/// so `w-screen h-screen` should NOT be compressed to `size-screen`.
static SIZE_BLOCKED_VALUES: phf::Set<&'static str> = phf::phf_set! {
    "screen", "3xs", "2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl",
};

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use biome_rowan::AstNode;
    use biome_tailwind_parser::parse_tailwind;
    use insta::assert_snapshot;

    use super::{analyze_tailwind_shorthand, auto_fix_all};

    const INVALID_CASES: &[&str] = &[
        "pl-2 pr-2 pt-2 pb-2",
        "hover:w-4 hover:h-4",
        "ml-2 mr-2",
        "ml-2 mr-2 text-blue-500 font-bold",
        "mr-3 ml-3",
        "mt-2 mb-2",
        "-mt-2 -mb-2",
        "pl-2 pr-2",
        "pt-2 pb-2",
        "pt-4 pr-2 pb-4 pl-4",
        "border-x border-y",
        "border-l-2 border-r-2",
        "border-t-2 border-b-2",
        "scroll-ml-2 scroll-mr-2",
        "scroll-mt-2 scroll-mb-2",
        "scroll-pl-2 scroll-pr-2",
        "scroll-pt-2 scroll-pb-2",
        "divide-x-4 divide-y-4",
        "rotate-x-45 rotate-y-45",
        "right-4 left-4",
        "bottom-4 top-4",
        "overflow-hidden text-ellipsis whitespace-nowrap",
        "pt-1 pr-1 pb-1 pl-1",
        "border-x border-y overflow-hidden text-ellipsis whitespace-nowrap w-8 h-8",
        "hover:w-10 hover:h-10 bg-gray-100 p-4",
        "ps-6 pe-6 py-6",
        "w-4 h-4",
        "w-[10px] h-[10px]",
        "sm:hover:w-4 sm:hover:h-4",
        "size-4 w-4 h-4",
        "w-4 h-4 size-4",
        "scale-x-20% scale-y-20% scale-z-20%",
        "-scale-x-20% -scale-y-20% -scale-z-20%",
        "scale-x-20%! scale-y-20%! scale-z-20%!",
        "truncate overflow-hidden text-ellipsis whitespace-nowrap",
        "scale-3d scale-x-20% scale-y-20% scale-z-20%",
        "content-center justify-center",
        "hover:overflow-hidden focus:overflow-hidden focus:text-ellipsis focus:whitespace-nowrap",
        "overflow-hidden! text-ellipsis! whitespace-nowrap!",
        "hover:overflow-hidden hover:text-ellipsis hover:whitespace-nowrap focus:overflow-hidden focus:text-ellipsis focus:whitespace-nowrap",
    ];

    const VALID_CASES: &[&str] = &[
        "mx-2 -my-2",
        "p-2 pl-4",
        "hover:size-4",
        "border",
        "truncate",
        "ml-2 mr-3",
        "hover:ml-2 focus:mr-2",
        "mx-2 mr-2",
        "pt-2 mr-3",
        "focus:w-4 hover:h-4",
        "w-[10px] h-[20px]",
        "w-screen h-screen",
    ];

    #[test]
    fn valid_cases() {
        let snapshot = render_cases(VALID_CASES, true);
        assert_snapshot!("valid_cases", snapshot);
    }

    #[test]
    fn invalid_cases() {
        let snapshot = render_cases(INVALID_CASES, false);
        assert_snapshot!("invalid_cases", snapshot);
    }

    fn render_cases(cases: &[&str], expect_no_violations: bool) -> String {
        let mut snapshot = String::new();

        for input in cases {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "failed to parse test input: {input}");

            let root = parse.tree();
            let violations = analyze_tailwind_shorthand(&root.candidates());
            if expect_no_violations {
                assert!(violations.is_empty(), "expected no violations for: {input}");
            } else {
                assert!(!violations.is_empty(), "expected violations for: {input}");
            }

            writeln!(snapshot, "## Input\n\n```text\n{input}\n```\n").unwrap();
            writeln!(snapshot, "Violations: {}", violations.len()).unwrap();

            for (index, violation) in violations.iter().enumerate() {
                writeln!(snapshot, "- Violation {}", index + 1).unwrap();
                writeln!(
                    snapshot,
                    "  nodes: {:?}",
                    violation
                        .uncompressed_nodes
                        .iter()
                        .map(|node| node.syntax().text_trimmed().to_string())
                        .collect::<Vec<_>>()
                )
                .unwrap();
                writeln!(
                    snapshot,
                    "  replacements: {:?}",
                    violation.replacement_bases
                )
                .unwrap();
                writeln!(
                    snapshot,
                    "  static replacements: {:?}",
                    violation.static_replacement_bases
                )
                .unwrap();
            }

            let fixed = auto_fix_all(&root, &violations).map_or_else(
                || input.to_string(),
                |mutation| mutation.commit().to_string(),
            );
            let fixed_parse = parse_tailwind(&fixed);
            assert!(
                !fixed_parse.has_errors(),
                "failed to parse fixed output: {fixed}"
            );
            assert!(
                analyze_tailwind_shorthand(&fixed_parse.tree().candidates()).is_empty(),
                "fixed output still contains shorthand violations: {fixed}"
            );
            writeln!(snapshot, "Fixed: `{fixed}`\n").unwrap();
        }

        snapshot
    }
}
