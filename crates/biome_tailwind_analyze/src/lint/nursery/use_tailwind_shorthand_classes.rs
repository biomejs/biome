use std::collections::HashMap;
use std::hash::Hash;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, Direction, WalkEvent};
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwModifier, AnyTwValue, TailwindSyntaxKind, TailwindSyntaxNode,
    TailwindSyntaxToken, TwCandidateList, TwFullCandidate, TwRoot, TwVariantList,
};

use crate::TailwindRuleAction;

declare_lint_rule! {
    /// Enforce using fewer Tailwind utilities instead of multiple utilities that are functionally the same.
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
    /// ```tailwind,expect_diagnostic
    /// ml-2 mr-2
    /// ```
    /// ```tailwind,expect_diagnostic
    /// pl-2 pr-2 pt-2 pb-2
    /// ```
    /// ```tailwind,expect_diagnostic
    /// hover:w-4 hover:h-4
    /// ```
    ///
    /// ### Valid
    ///
    /// ```tailwind
    /// mx-2 -my-2
    /// p-2 pl-4
    /// hover:size-4
    /// ```
    ///
    pub UseTailwindShorthandClasses {
        version: "next",
        name: "useTailwindShorthandClasses",
        language: "tailwind",
        recommended: false,
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-shorthand-classes").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseTailwindShorthandClasses {
    type Query = Ast<TwRoot>;
    type State = TailwindShorthandViolation;
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindShorthandClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let violations = analyze_tailwind_shorthand(root.candidates());
        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let first_range = state.uncompressed_nodes.first()?.range();

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first_range,
            markup! {
                "Prefer Tailwind shorthand utilities over multiple longhand utilities."
            },
        );

        for candidate in state.uncompressed_nodes.iter().skip(1) {
            diagnostic = diagnostic.detail(
                candidate.range(),
                markup! {
                    "Compressable utility used here."
                },
            );
        }
        diagnostic = diagnostic.note(markup! {
            "You can use less classes to reduce duplication and improve readability."
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<TailwindRuleAction> {
        let mut mutation = ctx.root().begin();
        let mut old_candidates = state.uncompressed_nodes.iter().rev();
        for replacement_base in state.replacement_bases {
            let Some(to_modify) = old_candidates.next() else {
                break;
            };
            let base_token = to_modify
                .candidate()
                .ok()?
                .as_tw_functional_candidate()?
                .base_token()
                .ok()?;
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

        for to_remove in old_candidates {
            mutation.remove_node(to_remove.clone());
        }

        Some(TailwindRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use this simplified class instead." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct TailwindShorthandViolation {
    pub uncompressed_nodes: Vec<TwFullCandidate>,
    pub replacement_bases: &'static [&'static str],
}

#[derive(Debug, Clone, Eq)]
struct GroupKey {
    variants: TwVariantList,
    negative: bool,
    important: bool,
    value: Option<AnyTwValue>,
    modifier: Option<AnyTwModifier>,
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
        is_node_equal(self.variants.syntax(), other.variants.syntax())
            && self.negative == other.negative
            && self.important == other.important
            && value_equal
            && modifier_equal
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

fn analyze_tailwind_shorthand(candidates: TwCandidateList) -> Vec<TailwindShorthandViolation> {
    fn extract_key(full: &TwFullCandidate) -> Option<GroupKey> {
        let variants = full.variants();
        let negative = full.negative_token().is_some();
        let important = full.excl_token().is_some();
        let candidate = full.candidate().ok()?;
        if let Some(func) = candidate.as_tw_functional_candidate() {
            let value = func.value().ok()?;
            let modifier = func.modifier();

            // let combined: Option<&'static str> = match (base, value) {
            //     ("overflow", "hidden") => Some("overflow-hidden"),
            //     ("text", "ellipsis") => Some("text-ellipsis"),
            //     ("whitespace", "nowrap") => Some("whitespace-nowrap"),
            //     _ => None,
            // };

            // if let Some(b) = combined {
            //     Some((
            //         b,
            //         GroupKey {
            //             variants,
            //             negative,
            //             important,
            //             value: None,
            //             modifier,
            //         },
            //     ))
            // } else {
            Some(GroupKey {
                variants,
                negative,
                important,
                value: Some(value),
                modifier,
            })
            // }
        } else if let Some(_) = candidate.as_tw_static_candidate() {
            Some(GroupKey {
                variants,
                negative,
                important,
                value: None,
                modifier: None,
            })
        } else {
            None
        }
    }

    let mut groups: HashMap<GroupKey, Vec<TwFullCandidate>> = HashMap::new();
    for candidate in candidates.iter() {
        let Some(candidate) = candidate.as_tw_full_candidate().cloned() else {
            continue;
        };
        let Some(key) = extract_key(&candidate) else {
            continue;
        };
        groups.entry(key).or_default().push(candidate);
    }

    let mut violations: Vec<TailwindShorthandViolation> = Vec::new();

    for pattern_group in TW_COMPRESSABLES {
        for (required_bases, replacement_bases) in *pattern_group {
            for (key, candidates) in &groups {
                if candidates.len() < required_bases.len() {
                    // Not enough candidates to match the required bases
                    continue;
                }

                // Check if all required bases are present
                let mut found_all = true;
                let mut flagged_candidates = Vec::with_capacity(replacement_bases.len());
                for &rb in *required_bases {
                    let Some(candidate) =
                        candidates
                            .iter()
                            .find(|candidate| match candidate.candidate().ok() {
                                Some(AnyTwCandidate::TwFunctionalCandidate(func)) => func
                                    .base_token()
                                    .ok()
                                    .is_some_and(|t| t.text_trimmed() == rb),
                                Some(AnyTwCandidate::TwStaticCandidate(st)) => {
                                    st.base_token().ok().is_some_and(|t| t.text_trimmed() == rb)
                                }
                                _ => false,
                            })
                    else {
                        found_all = false;
                        break;
                    };
                    flagged_candidates.push((*candidate).clone());
                }
                if !found_all {
                    continue;
                }
                debug_assert!(
                    flagged_candidates.len() == required_bases.len(),
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
                violations.push(TailwindShorthandViolation {
                    uncompressed_nodes: flagged_candidates,
                    replacement_bases,
                });
            }
        }
    }

    violations
}

/// Verifies that both nodes are equal by checking their descendants (nodes included) kinds
/// and tokens (same kind and inner token text).
pub(crate) fn is_node_equal(a_node: &TailwindSyntaxNode, b_node: &TailwindSyntaxNode) -> bool {
    let a_tree = a_node.preorder_with_tokens(Direction::Next);
    let b_tree = b_node.preorder_with_tokens(Direction::Next);
    for (a_event, b_event) in std::iter::zip(a_tree, b_tree) {
        let (a_child, b_child) = match (a_event, b_event) {
            (WalkEvent::Enter(a), WalkEvent::Enter(b)) => (a, b),
            (WalkEvent::Leave(_), WalkEvent::Leave(_)) => continue,
            _ => return false,
        };
        if a_child.kind() != b_child.kind() {
            return false;
        }
        let a_token = a_child.as_token();
        let b_token = b_child.as_token();
        match (a_token, b_token) {
            // both are nodes
            (None, None) => {}
            // one of them is a node
            (None, Some(_)) | (Some(_), None) => return false,
            // both are tokens
            (Some(a), Some(b)) => {
                if a.text_trimmed() != b.text_trimmed() {
                    return false;
                }
            }
        }
    }
    true
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
    &[(&["space-x", "space-y"], &["space"])],
    &[(&["gap-x", "gap-y"], &["gap"])],
    &[(&["translate-x", "translate-y"], &["translate"])],
    &[(&["rotate-x", "rotate-y"], &["rotate"])],
    &[(&["skew-x", "skew-y"], &["skew"])],
    &[
        (&["scale-x", "scale-y", "scale-z"], &["scale", "scale-3d"]),
        (&["scale-x", "scale-y"], &["scale"]),
    ],
    &[
        (&["content", "justify-content"], &["place-content"]),
        (&["items", "justify-items"], &["place-items"]),
        (&["self", "justify-self"], &["place-self"]),
    ],
    &[(
        &["overflow-hidden", "text-ellipsis", "whitespace-nowrap"],
        &["truncate"],
    )],
];
