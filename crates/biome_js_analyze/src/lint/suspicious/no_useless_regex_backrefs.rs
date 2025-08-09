use std::ops::Range;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_diagnostics::Severity;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_useless_regex_backrefs::NoUselessRegexBackrefsOptions;

declare_lint_rule! {
    /// Disallow useless backreferences in regular expression literals that always match an empty string.
    ///
    /// A backreference refers to the submatch of a previous capturing group and matches the same text as that group.
    /// JavaScript regular expression support two syntaxes:
    ///
    /// - `\N` where `N` is a 1-based integer that refers to the N-th declared capturing group.
    /// - `\k<name>` that refers to the capturing group named `name`.
    ///   This syntax is only available in Unicode-aware regular expressions,
    ///   i.e. regular expressions using the `u` or `v` flag.
    ///
    /// A backreference always matches an empty string when it refers to:
    ///
    /// - A group that belongs to another alternate branch.
    ///   In `/(a)|b\1b/`, the group `(a)` and its backreference `\1` are in distinct alternate branches.
    ///   `/(a)|b\1/` is equivalent to `(a)|b/`.
    ///
    /// - A group that appears after the backreference.
    ///   In `/\1(a)/`, the group `(a)` is declared after its backreference `\1`.
    ///   `/\1(a)/` is equivalent to `(a)/`.
    ///
    /// - A group in which the backreference is declared.
    ///   In `/(\1)/`, the backrefernce is nested in the group it refers to.
    ///   `/(\1)/` is equivalent to `/()/`.
    ///
    /// - A group that is inside a negative lookaround assertion without the backreference.
    ///   In `/a(?!(b)).\1/`, the backrefernce is in a negative assertion while its backreference is outside.
    ///   `/a(?!(b)).\1/` is equivalent to `/a(?!(b))./`.
    ///
    /// - A group that is declared before the backreference inside a lookbehind assertion.
    ///   In `/(?<=(a)\1)b/`, the backreference appears after the group while they are in a lookbehind assertion.
    ///   `/(?<=(a)\1)b/` is equivalent to `/(?<=(a))b/`.
    ///
    /// A backreference that always matches an empty string is always successfully matched and is therefore useless.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /(a)|b\1/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /\1(a)/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /(\1)/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /a(?!(b)).\1/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /(?<=(a)\1)b/;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /(a)\1/;
    /// ```
    ///
    /// ```js
    /// /(?<foo>a)\k<foo>/u;
    /// ```
    ///
    /// ```js
    /// /a(?!(b|c)\1)./;
    /// ```
    ///
    pub NoUselessRegexBackrefs {
        version: "2.0.0",
        name: "noUselessRegexBackrefs",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-useless-backreference").same(),
            RuleSource::EslintRegexp("no-useless-backreference").same(),
        ],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoUselessRegexBackrefs {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = BackRefIssue;
    type Signals = Option<Self::State>;
    type Options = NoUselessRegexBackrefsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (pattern, flags) = node.decompose().ok()?;
        let mut pattern_iter = pattern.bytes();
        while let Some(byte) = pattern_iter.next() {
            // We only call the heap memory and computation hungry code when we found a backref.
            if byte == b'\\' && matches!(pattern_iter.next(), Some(b'1'..=b'9' | b'k')) {
                return run(pattern.as_bytes(), flags.as_bytes());
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let pattern_start = u32::from(node.range().start()) + 1;
        let backref_range = {
            let backref_range = state.backref_range();
            let backref_start = pattern_start + backref_range.start as u32;
            let backref_end = pattern_start + backref_range.end as u32;
            TextRange::new(backref_start.into(), backref_end.into())
        };
        let group_range = {
            let group_range = state.group_range();
            let group_start = pattern_start + group_range.start as u32;
            let group_end = pattern_start + group_range.end as u32;
            TextRange::new(group_start.into(), group_end.into())
        };
        match state {
            BackRefIssue::NestedInReferredGroup {
                ..
            } => {
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    backref_range,
                    "This backreference is nested within the group to which it refers, making it always match an empty string.",
                ).detail(group_range, "The group starts here.")
                .note("Remove the backreference or place it outside the group to which it refers."))
            }
            BackRefIssue::DistinctAlternateBranch {
                alternate_index, ..
            } => {
                let alternate_index = pattern_start + *alternate_index as u32;
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    backref_range,
                    "This backreference refers to a group placed in another alternate branch, making it always match an empty string.",
                ).detail(group_range, "The backreference refers to this group.")
                .detail(TextRange::new(alternate_index.into(), (alternate_index + 1).into()), "The alternate separator is here.")
                .note("Remove the backreference or place it in the same alternate branch as the group."))
            }
            BackRefIssue::ReferredGroupDefinedAfter { .. } => {
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    backref_range,
                    "This backreference refers to a group that appears after it, making it always match an empty string.",
                ).note("A backreference must refer to a group defined before its occurrence.")
                .note("Remove the backreference."))
            }
            BackRefIssue::ReferredGroupDefinedBeforeInLookbehind { lookbehind_assertion_start, .. } => {
                let assertion_start = pattern_start + *lookbehind_assertion_start as u32;
                let assertion_end = assertion_start + 4;
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    backref_range,
                    "This backreference refers to a group that appears before itself in a lookbehind assertion, making it always match an empty string.",
                ).detail(group_range, "The backreference refers to this group.")
                .detail(TextRange::new(assertion_start.into(), assertion_end.into()), "The lookbehind assertion is here.")
                .note("Remove the backreference or place it after the group it refers to."))
            }
            BackRefIssue::ReferredGroupInNegatedLookaround {
                negated_assertion_start, ..
            } => {
                let assertion_start = pattern_start + *negated_assertion_start as u32;
                let assertion_end = assertion_start + 3;
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    backref_range,
                    "This backreference refers to a group within a negated assertion, making it always match an empty string.",
                ).detail(group_range, "The backreference refers to this group.")
                .detail(TextRange::new(assertion_start.into(), assertion_end.into()), "The negated assertion is here.")
                .note("Remove the backreference or place it in the negated assertion."))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BackRefIssue {
    NestedInReferredGroup {
        backref_range: Range<u16>,
        group_range: Range<u16>,
    },
    DistinctAlternateBranch {
        backref_range: Range<u16>,
        group_range: Range<u16>,
        alternate_index: u16,
    },
    ReferredGroupDefinedAfter {
        backref_range: Range<u16>,
        group_range: Range<u16>,
    },
    ReferredGroupDefinedBeforeInLookbehind {
        backref_range: Range<u16>,
        group_range: Range<u16>,
        lookbehind_assertion_start: u16,
    },
    ReferredGroupInNegatedLookaround {
        backref_range: Range<u16>,
        group_range: Range<u16>,
        negated_assertion_start: u16,
    },
}
impl BackRefIssue {
    fn backref_range(&self) -> &Range<u16> {
        match self {
            Self::NestedInReferredGroup { backref_range, .. } => backref_range,
            Self::DistinctAlternateBranch { backref_range, .. } => backref_range,
            Self::ReferredGroupDefinedAfter { backref_range, .. } => backref_range,
            Self::ReferredGroupDefinedBeforeInLookbehind { backref_range, .. } => backref_range,
            Self::ReferredGroupInNegatedLookaround { backref_range, .. } => backref_range,
        }
    }
    fn group_range(&self) -> &Range<u16> {
        match self {
            Self::NestedInReferredGroup { group_range, .. } => group_range,
            Self::DistinctAlternateBranch { group_range, .. } => group_range,
            Self::ReferredGroupDefinedAfter { group_range, .. } => group_range,
            Self::ReferredGroupDefinedBeforeInLookbehind { group_range, .. } => group_range,
            Self::ReferredGroupInNegatedLookaround { group_range, .. } => group_range,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RegexOpeningElement {
    kind: RegexElementKind,
    start: u16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RegexElementKind {
    Alternate,
    CapturingGroup { number: u16 },
    NonCapturingGroup,
    LookaheadAssertion,
    LookbehindAssertion,
    NegatedLookaheadAssertion,
    NegatedLookbehindAssertion,
}

#[derive(Debug)]
struct CapturingGroup<'a> {
    /// - group without name such as `(ab)`
    /// - group with name such as `(?<name>ab)`
    name: Option<&'a [u8]>,
    range: Range<u16>,
    negated_assertion_start: Option<u16>,
}
impl<'a> CapturingGroup<'a> {
    fn with_name(name: &'a [u8], range: Range<u16>) -> Self {
        Self {
            name: Some(name),
            range,
            negated_assertion_start: None,
        }
    }
    fn without_name(range: Range<u16>) -> Self {
        Self {
            name: None,
            range,
            negated_assertion_start: None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnresolvedBackrefId<'a> {
    Named(&'a [u8]),
    Number(u16),
}

#[derive(Debug)]
struct UnresolvedBackref<'a> {
    id: UnresolvedBackrefId<'a>,
    range: Range<u16>,
}
impl<'a> UnresolvedBackref<'a> {
    const fn new(id: UnresolvedBackrefId<'a>, range: Range<u16>) -> Self {
        Self { id, range }
    }
}

#[cold]
#[inline(never)]
fn run(pattern: &[u8], flags: &[u8]) -> Option<BackRefIssue> {
    // The index of the capturing group corresponds to its number minus 1.
    // Thus, the capturing group referenced by `\1` is at index `0`.
    let mut capturing_groups: Vec<CapturingGroup> = Vec::new();
    let mut unresolved_backrefs: Vec<UnresolvedBackref> = Vec::new();
    let mut opening_stack: Vec<RegexOpeningElement> = Vec::new();
    let mut pattern_iter = pattern.iter().enumerate();
    let has_v_flag = flags.contains(&b'v');
    let has_u_flag = flags.contains(&b'u');
    let is_unicode_aware = has_v_flag || has_u_flag;
    while let Some((index, byte)) = pattern_iter.next() {
        match byte {
            b'\\' => {
                let Some((_, byte)) = pattern_iter.next() else {
                    // The regex is certainly invalid because it ends with `\`.
                    break;
                };
                // Match against backref syntax `\k<name>` and `\N` with `N` an integer.
                let ref_start = index as u16;
                let mut lookahead = pattern_iter.clone();
                match byte {
                    // `\N`
                    b'1'..=b'9' => {
                        let mut backref_number = Some((byte - b'1' + 1) as u16);
                        let mut ref_end = ref_start + 2;
                        for (_, byte) in lookahead {
                            if !byte.is_ascii_digit() {
                                break;
                            }
                            pattern_iter.next();
                            ref_end += 1;
                            backref_number = backref_number
                                .and_then(|n| n.checked_mul(10))
                                .and_then(|n| n.checked_add((byte - b'0') as u16));
                            if backref_number.is_none() {
                                break;
                            }
                        }
                        let Some(backref_number) = backref_number else {
                            // The number overflows. This cannot be a bakcref.
                            // It is an escape sequence (probably an octal escape sequence).
                            continue;
                        };
                        // `backref_number` is 1-based. Subtract `1` to get a 0-based index.
                        let group = capturing_groups.get(backref_number as usize - 1);
                        if let Some(group) = group.filter(|group| !group.range.is_empty()) {
                            if let Some(issue) =
                                get_issue(&opening_stack, group, ref_start..ref_end)
                            {
                                return Some(issue);
                            }
                        } else {
                            unresolved_backrefs.push(UnresolvedBackref::new(
                                UnresolvedBackrefId::Number(backref_number),
                                ref_start..ref_end,
                            ));
                        }
                    }
                    // `\k<name>`
                    b'k' if is_unicode_aware && matches!(lookahead.next(), Some((_, b'<'))) => {
                        // Eat `<`.
                        pattern_iter.next();
                        let name_start = index + 3;
                        let mut name_end = name_start;
                        for (index, &byte) in lookahead {
                            name_end = index;
                            pattern_iter.next();
                            if byte == b'>' {
                                break;
                            }
                        }
                        let name = &pattern[name_start..name_end];
                        let ref_end = (name_end + 1) as u16;
                        let group = capturing_groups
                            .iter()
                            .find(|group| group.name == Some(name));
                        if let Some(group) = group.filter(|group| !group.range.is_empty()) {
                            if let Some(issue) =
                                get_issue(&opening_stack, group, ref_start..ref_end)
                            {
                                return Some(issue);
                            }
                        } else {
                            unresolved_backrefs.push(UnresolvedBackref::new(
                                UnresolvedBackrefId::Named(name),
                                ref_start..ref_end,
                            ));
                        }
                    }
                    _ => {
                        // Skip the escaped character.
                    }
                }
            }
            b'|' => {
                opening_stack.push(RegexOpeningElement {
                    kind: RegexElementKind::Alternate,
                    start: index as u16,
                });
            }
            b'(' => {
                let start = index as u16;
                let mut lookahead = pattern_iter.clone();
                let kind = if matches!(lookahead.next(), Some((_, b'?'))) {
                    // Eat `?` and the next character.
                    pattern_iter.next();
                    pattern_iter.next();
                    match lookahead.next() {
                        Some((_, b':')) => RegexElementKind::NonCapturingGroup,
                        Some((_, b'=')) => RegexElementKind::LookaheadAssertion,
                        Some((_, b'!')) => RegexElementKind::NegatedLookaheadAssertion,
                        Some((_, b'<')) => {
                            pattern_iter.next();
                            match lookahead.next() {
                                Some((_, b'=')) => RegexElementKind::LookbehindAssertion,
                                Some((_, b'!')) => RegexElementKind::NegatedLookbehindAssertion,
                                Some((name_start, _)) if is_unicode_aware => {
                                    let mut name_end = name_start;
                                    for (index, &byte) in lookahead {
                                        name_end = index;
                                        if byte == b'>' {
                                            break;
                                        }
                                        pattern_iter.next();
                                    }
                                    capturing_groups.push(CapturingGroup::with_name(
                                        &pattern[name_start..name_end],
                                        start..start,
                                    ));
                                    RegexElementKind::CapturingGroup {
                                        number: capturing_groups.len() as u16,
                                    }
                                }
                                // Should be unreachable.
                                _ => RegexElementKind::NonCapturingGroup,
                            }
                        }
                        // Should be unreachable.
                        _ => RegexElementKind::NonCapturingGroup,
                    }
                } else {
                    capturing_groups.push(CapturingGroup::without_name(start..start));
                    RegexElementKind::CapturingGroup {
                        number: capturing_groups.len() as u16,
                    }
                };
                opening_stack.push(RegexOpeningElement { kind, start });
            }
            b')' => {
                while let Some(element) = opening_stack.pop() {
                    match element.kind {
                        RegexElementKind::Alternate => {
                            // Remove alternations of the group we are leaving.
                        }
                        RegexElementKind::CapturingGroup { number } => {
                            let Some(group) = capturing_groups.get_mut(number as usize - 1) else {
                                debug_assert!(
                                    number as usize <= capturing_groups.len(),
                                    "The group has been inserted",
                                );
                                break;
                            };
                            group.range.end = index as u16 + 1;
                            // Try to solve unresolved refs when we found a new capturing group
                            let group_number = UnresolvedBackrefId::Number(number);
                            let group_name = group.name.map(UnresolvedBackrefId::Named);
                            for unresolved_backref in &unresolved_backrefs {
                                if unresolved_backref.id == group_number
                                    || Some(unresolved_backref.id) == group_name
                                {
                                    if let Some(issue) = get_issue(
                                        &opening_stack,
                                        group,
                                        unresolved_backref.range.clone(),
                                    ) {
                                        return Some(issue);
                                    }
                                    break;
                                }
                            }
                            break;
                        }
                        RegexElementKind::NonCapturingGroup
                        | RegexElementKind::LookaheadAssertion
                        | RegexElementKind::LookbehindAssertion => {
                            break;
                        }
                        RegexElementKind::NegatedLookaheadAssertion
                        | RegexElementKind::NegatedLookbehindAssertion => {
                            for group in capturing_groups.iter_mut().rev() {
                                if group.range.start < element.start {
                                    break;
                                }
                                group.negated_assertion_start = Some(element.start);
                            }
                            break;
                        }
                    }
                }
            }
            b'[' => {
                // Skip the character class
                let mut inner_class_count = 0;
                while let Some((_, byte)) = pattern_iter.next() {
                    match byte {
                        b'\\' => {
                            // SKip escaped character.
                            pattern_iter.next();
                        }
                        b'[' if has_v_flag => {
                            // The `v` flag allows char class nesting.
                            inner_class_count += 1;
                        }
                        b']' if has_v_flag && inner_class_count > 0 => {
                            inner_class_count -= 1;
                        }
                        b']' => {
                            // End of the char class.
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    // Unresolved Backrefs are useless escape sequences.
    None
}

fn get_issue(
    opening_stack: &[RegexOpeningElement],
    group: &CapturingGroup,
    backref_range: Range<u16>,
) -> Option<BackRefIssue> {
    if group.range.contains(&backref_range.start) {
        // The backref is nested in the group. e.g. `(a\1)`.
        Some(BackRefIssue::NestedInReferredGroup {
            group_range: group.range.clone(),
            backref_range,
        })
    } else if let Some(negated_assertion_start) = group.negated_assertion_start {
        // The backref references a group inside a negated assertion. e.g. `(?!(a))\1`.
        Some(BackRefIssue::ReferredGroupInNegatedLookaround {
            group_range: group.range.clone(),
            backref_range,
            negated_assertion_start,
        })
    } else {
        let mut alternate_index = None;
        let mut lookbehind_assertion_start = None;
        for &RegexOpeningElement { kind, start } in opening_stack.iter().rev() {
            if group.range.start < start {
                alternate_index = (kind == RegexElementKind::Alternate).then_some(start);
            } else if matches!(
                kind,
                RegexElementKind::CapturingGroup { .. } | RegexElementKind::NonCapturingGroup
            ) {
                // Skip capturing and non-capturing groups.
            } else {
                if matches!(
                    kind,
                    RegexElementKind::LookbehindAssertion
                        | RegexElementKind::NegatedLookbehindAssertion
                ) {
                    lookbehind_assertion_start = Some(start);
                }
                break;
            }
        }
        if backref_range.start < group.range.start {
            if lookbehind_assertion_start.is_some_and(|start| start < backref_range.start) {
                // Allow `(?<=\1(a))`.
                None
            } else {
                // Report `\1(?<=(a))`, `\1(a)`
                Some(BackRefIssue::ReferredGroupDefinedAfter {
                    backref_range,
                    group_range: group.range.clone(),
                })
            }
        } else if let Some(alternate_index) = alternate_index {
            // The backref references a group in another alternate branch. e.g. `(a)|\1`.
            Some(BackRefIssue::DistinctAlternateBranch {
                group_range: group.range.clone(),
                backref_range,
                alternate_index,
            })
        } else {
            lookbehind_assertion_start.map(|lookbehind_assertion_start| {
                // Disallow `(?<=(a)\1)`.
                BackRefIssue::ReferredGroupDefinedBeforeInLookbehind {
                    backref_range,
                    group_range: group.range.clone(),
                    lookbehind_assertion_start,
                }
            })
        }
    }
}
