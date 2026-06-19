use std::{borrow::Cow, cmp::Ordering, iter::zip};

use biome_analyze::shared::sort_attributes::{AttributeGroup, SortableAttribute};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    AnyJsxAttribute, JsLanguage, JsxAttribute, JsxAttributeList, JsxOpeningElement,
    JsxSelfClosingElement,
};
use biome_rowan::{AstNode, AstNodeExt, BatchMutationExt, SyntaxToken, TriviaPieceKind};
use biome_rule_options::use_sorted_attributes::{
    AttributeGroups, SortOrder, SortScope, UseSortedAttributesOptions, default_attribute_groups,
};
use crate::JsRuleAction;

declare_source_rule! {
    /// Enforce attribute sorting in JSX elements.
    ///
    /// This rule checks if the JSX props are sorted in a consistent way.
    /// Props are sorted alphabetically using a [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order).
    /// This rule will not consider spread props as sortable.
    /// Instead, whenever it encounters a spread prop, it will sort all the
    /// previous non spread props up until the nearest spread prop, if one
    /// exist.
    /// This prevents breaking the override of certain props using spread
    /// props.
    ///
    /// ## Examples
    ///
    /// ```jsx,expect_diff
    /// <Hello lastName="Smith" firstName="John" />;
    /// ```
    ///
    /// ```jsx,expect_diff
    /// <Hello lastName="Smith" firstName="John" {...this.props} tel="0000" address="111 Main Street"  {...another.props} lastName="Smith" />;
    /// ```
    ///
    /// ## Options
    /// This actions accepts following options
    ///
    /// ### `sortOrder`
    /// This options supports `natural` and `lexicographic` values. Where as `natural` is the default.
    ///
    /// Following will apply the natural sort order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "natural"
    ///     }
    /// }
    /// ```
    /// ```jsx,use_options,expect_diagnostic
    /// <Hello tel={5555555} {...this.props} opt1="John" opt2="" opt12="" opt11="" />;
    /// ```
    ///
    /// Following will apply the lexicographic sort order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    /// ```jsx,use_options,expect_diagnostic
    /// <Hello tel={5555555} {...this.props} opt1="John" opt2="" opt12="" opt11="" />;
    /// ```
    ///
    /// ### `groups`
    ///
    /// Controls the ordering of special prop categories. Only active when
    /// `sortScope` is `"group"`. Accepts an ordered array of predefined group
    /// tokens. Props not matching any group are placed after all named groups.
    ///
    /// Available group tokens:
    /// - `":CALLBACK:"` — Callback props: names starting with `on` + uppercase (e.g. `onClick`).
    /// - `":IMPLICIT:"` — Implicit (boolean shorthand) props: no value (e.g. `<Foo disabled />`).
    /// - `":RESERVED:"` — React reserved props: `key` and `ref`.
    /// - `":DOM_RESERVED:"` — DOM-only reserved props: `children` and `dangerouslySetInnerHTML`.
    /// - `":REST:"` — Catch-all for props that don't match any other configured group.
    ///   Sorted like a regular group, using `sortOrder`. If omitted from `groups`, unmatched
    ///   props are instead placed after all named groups, in their original relative order (unsorted).
    ///
    /// When not configured, the default ordering is:
    /// `[":IMPLICIT:", ":RESERVED:", ":DOM_RESERVED:", ":REST:", ":CALLBACK:"]`.
    ///
    /// ### `sortScope`
    ///
    /// Controls how `sortOrder` interacts with `groups`.
    ///
    /// - `"global"` (default): flat sort across all props, groups are ignored.
    ///   Preserves existing behavior.
    /// - `"group"`: props are first partitioned into groups (defined by `groups`),
    ///   sorted within each group using `sortOrder`, then concatenated in group order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortScope": "group",
    ///         "groups": [":RESERVED:", ":IMPLICIT:", ":CALLBACK:"]
    ///     }
    /// }
    /// ```
    /// ```jsx,use_options,expect_diagnostic
    /// <Hello onClick={fn} disabled key="1" name="John" />;
    /// ```
    ///
    pub UseSortedAttributes {
        version: "2.0.0",
        name: "useSortedAttributes",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-sort-props").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedAttributes {
    type Query = Ast<JsxAttributeList>;
    type State = AttributeGroup<SortableJsxAttribute>;
    type Signals = Box<[Self::State]>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let props = ctx.query();
        let options = ctx.options();
        let comparator = select_comparator(options);
        let sort_scope = options.sort_scope.unwrap_or_default();
        let groups = resolve_groups(options, sort_scope);

        let mut current_bucket: Vec<SortableJsxAttribute> = Vec::new();
        let mut prop_groups: Vec<AttributeGroup<SortableJsxAttribute>> = Vec::new();

        for prop in props {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    current_bucket.push(SortableJsxAttribute(attr));
                }
                // Spread or shorthand attribute resets sort order: it carries
                // an opaque expression that may have side effects on the
                // resulting prop set, so attributes on either side cannot be
                // freely reordered across it.
                AnyJsxAttribute::JsxSpreadAttribute(_)
                | AnyJsxAttribute::JsxShorthandAttribute(_) => {
                    flush_bucket(
                        &mut current_bucket,
                        &mut prop_groups,
                        groups.as_ref(),
                        sort_scope,
                        comparator,
                    );
                }
                AnyJsxAttribute::JsMetavariable(_) => {}
            }
        }
        flush_bucket(
            &mut current_bucket,
            &mut prop_groups,
            groups.as_ref(),
            sort_scope,
            comparator,
        );

        prop_groups.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            Self::text_range(ctx, state)?,
            markup! {
                "The attributes are not sorted. "
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query().syntax().ancestors().skip(1).find_map(|node| {
            JsxOpeningElement::cast_ref(&node)
                .map(|element| element.range())
                .or_else(|| JsxSelfClosingElement::cast_ref(&node).map(|element| element.range()))
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let options = ctx.options();
        let comparator = select_comparator(options);
        let sort_scope = options.sort_scope.unwrap_or_default();
        let groups = resolve_groups(options, sort_scope);

        let sorted = match sort_scope {
            SortScope::Global => state.get_sorted_attributes(comparator)?,
            SortScope::Group => {
                get_sorted_by_groups(&state.attrs, groups.as_ref(), comparator)?
            }
        };

        for (SortableJsxAttribute(attr), SortableJsxAttribute(sorted_attr)) in
            zip(state.attrs.iter(), sorted)
        {
            mutation.replace_node_discard_trivia(attr.clone(), sorted_attr);
        }

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the JSX props." },
            mutation,
        ))
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns the effective `AttributeGroups` when `sort_scope` is `Group`.
fn resolve_groups(
    options: &UseSortedAttributesOptions,
    sort_scope: SortScope,
) -> Option<AttributeGroups> {
    if sort_scope == SortScope::Global {
        return None;
    }
    Some(
        options
            .groups
            .clone()
            .unwrap_or_else(default_attribute_groups),
    )
}

type ComparatorFn = fn(&SortableJsxAttribute, &SortableJsxAttribute) -> Ordering;

/// Selects the comparator based on `sortOrder`.
fn select_comparator(options: &UseSortedAttributesOptions) -> ComparatorFn {
    match options.sort_order.unwrap_or_default() {
        SortOrder::Natural => SortableJsxAttribute::ascii_nat_cmp,
        SortOrder::Lexicographic => SortableJsxAttribute::lexicographic_cmp,
    }
}

/// Checks a bucket of props and, if unsorted, pushes an `AttributeGroup` to `out`.
fn flush_bucket(
    bucket: &mut Vec<SortableJsxAttribute>,
    out: &mut Vec<AttributeGroup<SortableJsxAttribute>>,
    groups: Option<&AttributeGroups>,
    sort_scope: SortScope,
    comparator: ComparatorFn,
) {
    if bucket.is_empty() {
        return;
    }

    let is_already_sorted = match sort_scope {
        SortScope::Global => {
            let bc = |a: &SortableJsxAttribute, b: &SortableJsxAttribute| {
                comparator(a, b) != Ordering::Greater
            };
            bucket.is_sorted_by(bc)
        }
        SortScope::Group => {
            let sorted = get_sorted_by_groups(bucket, groups, comparator);
            sorted.is_some_and(|sorted| bucket.iter().zip(sorted.iter()).all(|(a, b)| a.0 == b.0))
        }
    };

    if is_already_sorted {
        bucket.clear();
    } else {
        out.push(AttributeGroup {
            attrs: std::mem::take(bucket),
        });
    }
}

/// Returns props sorted with group-aware logic.
///
/// Sorts independently-bucketed groups and concatenates them. The
/// separating-whitespace fixup is applied **once** over the fully concatenated
/// result (see [`ensure_separating_whitespace`]); doing it per-bucket would
/// miss the seams between buckets and could emit mashed-together, invalid JSX.
fn get_sorted_by_groups(
    attrs: &[SortableJsxAttribute],
    groups: Option<&AttributeGroups>,
    comparator: ComparatorFn,
) -> Option<Vec<SortableJsxAttribute>> {
    let mut result = sort_into_buckets(attrs, groups, comparator)?;
    ensure_separating_whitespace(&mut result)?;
    Some(result)
}

/// Ensures every attribute except the last carries separating trailing
/// whitespace.
///
/// Group sorting concatenates independently-sorted buckets. A bucket's last
/// attribute may be the source's final prop (e.g. the one right before `/>` or
/// `>`), which carries no trailing whitespace. Once reordered next to another
/// bucket, the two attributes would render mashed together
/// (`onClick={fn}key="1"`), producing invalid JSX. This pass runs once over the
/// fully concatenated result, mirroring what
/// `AttributeGroup::get_sorted_attributes` does for the global (flat) path.
fn ensure_separating_whitespace(attrs: &mut [SortableJsxAttribute]) -> Option<()> {
    let mut iter = attrs.iter_mut().peekable();
    while let Some(attr) = iter.next() {
        if iter.peek().is_some() {
            let ends_in_whitespace = attr
                .node()
                .syntax()
                .last_trailing_trivia()
                .and_then(|last_trivia| last_trivia.last())
                .is_some_and(|last| last.is_whitespace() || last.is_newline());

            let next_starts_with_whitespace = iter
                .peek()
                .and_then(|next_attr| next_attr.node().syntax().first_leading_trivia())
                .and_then(|first_trivia| first_trivia.first())
                .is_some_and(|first| first.is_whitespace() || first.is_newline());

            if !ends_in_whitespace && !next_starts_with_whitespace {
                let old_last_token = attr.node().syntax().last_token()?;
                let new_trailing_trivia: Vec<(TriviaPieceKind, String)> = old_last_token
                    .trailing_trivia()
                    .pieces()
                    .map(|p| (p.kind(), p.text().to_string()))
                    .chain(std::iter::once((TriviaPieceKind::Whitespace, " ".to_string())))
                    .collect();
                // Convert owned strings back to &str for with_trailing_trivia
                let trivia_refs: Vec<(TriviaPieceKind, &str)> = new_trailing_trivia
                    .iter()
                    .map(|(k, s)| (*k, s.as_str()))
                    .collect();
                let new_last_token = old_last_token.with_trailing_trivia(trivia_refs);
                *attr = attr.clone().replace_token(old_last_token, new_last_token)?;
            }
        }
    }
    Some(())
}

/// Assigns each prop to a group bucket, sorts named buckets, and concatenates.
/// The implicit rest bucket (when `:REST:` is not configured) is left unsorted.
fn sort_into_buckets(
    attrs: &[SortableJsxAttribute],
    groups: Option<&AttributeGroups>,
    comparator: ComparatorFn,
) -> Option<Vec<SortableJsxAttribute>> {
    if attrs.is_empty() {
        return Some(Vec::new());
    }

    // The implicit-fallback-bucket index is `groups.len()` (one past the
    // last named group). `AttributeGroups::group_index` only ever returns
    // this index for props that match neither a named group nor a
    // configured `:REST:` group, so this bucket is only ever non-empty when
    // `:REST:` isn't configured. Such props are NOT sorted — their original
    // relative order is preserved. An explicit `:REST:` group, by contrast,
    // occupies its own (sorted) bucket at its configured position.
    let rest_index = groups.map_or(0, |g| g.len());

    let annotated: Vec<(usize, SortableJsxAttribute)> = attrs
        .iter()
        .map(|attr| {
            let idx = match groups {
                Some(g) => g.group_index(&attr.0),
                None => 0,
            };
            (idx, attr.clone())
        })
        .collect();

    let max_bucket = annotated.iter().map(|(idx, _)| *idx).max().unwrap_or(0);
    let mut buckets: Vec<Vec<SortableJsxAttribute>> = vec![Vec::new(); max_bucket + 1];
    for (idx, attr) in annotated {
        buckets[idx].push(attr);
    }

    let mut result: Vec<SortableJsxAttribute> = Vec::with_capacity(attrs.len());
    for (idx, mut bucket) in buckets.into_iter().enumerate() {
        // The rest bucket preserves source order; named buckets are sorted.
        // Separating whitespace is fixed up once by the caller.
        if idx != rest_index {
            bucket.sort_by(comparator);
        }
        result.extend(bucket);
    }

    Some(result)
}

// ── SortableJsxAttribute ──────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Clone)]
pub struct SortableJsxAttribute(JsxAttribute);

impl SortableAttribute for SortableJsxAttribute {
    type Language = JsLanguage;

    fn name(&self) -> Option<SyntaxToken<Self::Language>> {
        self.0.name().ok()?.name_token().ok()
    }

    fn node(&self) -> &impl AstNode<Language = Self::Language> {
        &self.0
    }

    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(
            self.0
                .replace_token_discard_trivia(prev_token, next_token)?,
        ))
    }
}

