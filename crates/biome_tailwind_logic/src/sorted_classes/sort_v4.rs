use std::cmp::Ordering;
use std::sync::Arc;

use smallvec::SmallVec;

use biome_rowan::{AstNode, AstNodeList, SyntaxNodeText, TextRange, TextSize, TokenText};
use biome_string_case::Collator;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, CssGenericComponentValueList,
    TailwindSyntaxNode, TailwindSyntaxToken, TwFullCandidate, TwRoot,
};

use super::arbitrary_value_match::value_matches_type;
use super::sort_v4_variants::{
    VariantGroups, VariantKey, VariantWeight, variant_keys_from_candidate,
};
use super::tailwind_preset_v4::{
    FUNCTIONAL_UTILITIES, KEYWORD_POOL, PROPERTY_INDEX, SIGNATURE_POOL, STATIC_UTILITIES,
};
use super::tailwind_preset_v4_types::{
    ArbitraryBranch, FunctionalEntry, ModifierKind, NamedBranch, NamedValueType, Negative,
    ThemeNamespace,
};
pub use super::tailwind_registry::TailwindRegistry;
use super::tailwind_registry::{
    BareValueType, FunctionalDecl, RegisteredFunctional, ValueShape, join_into,
};

#[cfg(test)]
use super::tailwind_preset_v4_types::CssDataType;

/// Sort the candidates of a parsed Tailwind class list and return the joined,
/// space-separated result. The `registry` overlays custom utilities,
/// variants, and theme keys from the project's Tailwind stylesheet on
/// top of the builtin preset; pass `&TailwindRegistry::default()` when
/// no project stylesheet is available.
pub fn sort_class_list(root: &TwRoot, registry: &TailwindRegistry) -> String {
    let candidates = root.candidates();

    // A variant's weight depends on the whole list, so classify first and
    // weight in a second pass.
    let mut pending = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let text = candidate.syntax().text_trimmed();
        let key = PendingSortKey::from_candidate(&candidate, registry);
        pending.push((key, text));
    }

    // Group the variants across the list, then weight each pending key.
    let variant_groups = VariantGroups::new(
        pending
            .iter()
            .filter_map(|(key, _)| match key {
                PendingSortKey::Known { variants, .. } => Some(variants.as_slice()),
                PendingSortKey::Unknown => None,
            })
            .flatten(),
        registry,
    );
    let mut keyed: Vec<(SortKey, SyntaxNodeText)> = pending
        .into_iter()
        .map(|(key, text)| (key.into_sort_key(&variant_groups), text))
        .collect();

    // `Vec::sort_by` is stable, so Unknown-vs-Unknown comparisons returning
    // `Equal` keep input order.
    keyed.sort_by(|a, b| compare(&a.0, &b.0));

    // Sort is in-place; total text length is unchanged. Pre-size the output
    // so chunked emission never re-allocates.
    let text_bytes: usize = keyed.iter().map(|(_, t)| usize::from(t.len())).sum();
    let separators = keyed.len().saturating_sub(1);
    let mut result = String::with_capacity(text_bytes + separators);
    for (_, text) in &keyed {
        if !result.is_empty() {
            result.push(' ');
        }
        text.for_each_chunk(|chunk| result.push_str(chunk));
    }
    result
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SortKey {
    Unknown,
    Known {
        /// Variant weight (`hover:`, `sm:`, …), empty for a plain
        /// utility. The outermost sort key, so variantless utilities
        /// sort first.
        variant_weight: VariantWeight,
        signature: Signature,
        /// Total declaration count — Tailwind's tie-break after the
        /// signature (wider utilities sort first).
        count: u8,
        /// The whole candidate text (variants, sign, name, `!`).
        /// Tailwind's last tiebreak compares this text with
        /// [TwNameCollator].
        text: CandidateText,
    },
}

/// A candidate's whole text as an allocation-free view into the syntax
/// tree, ordered the way Tailwind's `compare()` orders raw candidates.
// TODO: equality compares text chunk-wise and cannot short-circuit on
// syntax kind mismatches; switch to a structural node comparison once a
// generic `is_node_equal` is available.
#[derive(Clone, Debug)]
struct CandidateText(TailwindSyntaxNode);

impl PartialEq for CandidateText {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl Eq for CandidateText {}

impl CandidateText {
    fn text(&self) -> SyntaxNodeText {
        self.0.text_trimmed()
    }

    /// [TwNameCollator] order over the whole text. Two candidates in one
    /// bucket usually share their entire variant prefix (`hover:p-2` /
    /// `hover:p-4`), so the shared prefix is skipped bytewise and only the
    /// tail from the first difference goes through the collator — backed
    /// up to the start of a digit run (`p-12` / `p-1a` compare `12` with
    /// `1`) or of a multi-byte character, so the result is exactly the
    /// collator's.
    fn compare(&self, other: &Self) -> Ordering {
        let mut a = TextChunks::new(&self.0);
        let mut b = TextChunks::new(&other.0);
        // Bytes shared so far, and how many of the last shared bytes are
        // ASCII digits, or belong to a multi-byte character the two texts
        // may diverge inside of (only one of the two can be non-zero).
        let mut shared = 0usize;
        let mut trailing_digits = 0usize;
        let mut trailing_char_bytes = 0usize;
        loop {
            let (Some(x), Some(y)) = (a.rest(), b.rest()) else {
                return match (a.rest().is_some(), b.rest().is_some()) {
                    (false, false) => Ordering::Equal,
                    (false, true) => Ordering::Less,
                    (true, false) => Ordering::Greater,
                    (true, true) => unreachable!(),
                };
            };
            let n = x.len().min(y.len());
            let diverged = x[..n].iter().zip(&y[..n]).position(|(p, q)| p != q);
            let matched = diverged.unwrap_or(n);
            for &byte in &x[..matched] {
                if byte.is_ascii_digit() {
                    trailing_digits += 1;
                    trailing_char_bytes = 0;
                } else if byte.is_ascii() {
                    trailing_digits = 0;
                    trailing_char_bytes = 0;
                } else if (byte & 0xC0) == 0x80 {
                    // Continuation byte: still inside the character.
                    trailing_char_bytes += 1;
                } else {
                    // Lead byte: a new multi-byte character starts here.
                    trailing_digits = 0;
                    trailing_char_bytes = 1;
                }
            }
            shared += matched;
            if diverged.is_some() {
                break;
            }
            a.advance(n);
            b.advance(n);
        }
        // Both texts share every byte before `restart`, so it is a
        // character boundary in both.
        let restart = TextSize::from((shared - trailing_digits - trailing_char_bytes) as u32);
        let tail = |text: SyntaxNodeText| text.slice(TextRange::new(restart, text.len()));
        TwNameCollator.cmp(tail(self.text()).chars(), tail(other.text()).chars())
    }
}

/// Cursor over the byte chunks of a node's trimmed text, one token at a
/// time, without allocating.
struct TextChunks {
    range: TextRange,
    /// The current token and the part of its text inside `range`, in
    /// token-local offsets.
    current: Option<(TailwindSyntaxToken, TextRange)>,
    offset: usize,
}

impl TextChunks {
    fn new(node: &TailwindSyntaxNode) -> Self {
        let mut chunks = Self {
            range: node.text_trimmed_range(),
            current: None,
            offset: 0,
        };
        chunks.enter(node.first_token());
        chunks
    }

    /// Make `token` current, skipping ahead over tokens with no text inside
    /// the trimmed range; a token past the range ends the walk.
    fn enter(&mut self, mut token: Option<TailwindSyntaxToken>) {
        self.offset = 0;
        self.current = None;
        while let Some(candidate) = token {
            if candidate.text_range().start() >= self.range.end() {
                return;
            }
            if let Some(inside) = candidate.text_range().intersect(self.range)
                && !inside.is_empty()
            {
                let local = inside - candidate.text_range().start();
                self.current = Some((candidate, local));
                return;
            }
            token = candidate.next_token();
        }
    }

    /// The unread bytes of the current chunk, or `None` at the end.
    fn rest(&self) -> Option<&[u8]> {
        let (token, local) = self.current.as_ref()?;
        Some(
            &token.text().as_bytes()
                [usize::from(local.start()) + self.offset..usize::from(local.end())],
        )
    }

    fn advance(&mut self, bytes: usize) {
        self.offset += bytes;
        let done = self
            .current
            .as_ref()
            .is_some_and(|(_, local)| self.offset >= usize::from(local.len()));
        if done {
            let next = self
                .current
                .as_ref()
                .and_then(|(token, _)| token.next_token());
            self.enter(next);
        }
    }
}

/// A classified candidate whose variants are resolved but not yet
/// weighted — weighting needs the whole list.
/// [PendingSortKey::into_sort_key] finishes the [SortKey] once the
/// [VariantGroups] are built.
#[derive(Clone, Debug)]
enum PendingSortKey<'r> {
    Unknown,
    Known {
        signature: Signature,
        count: u8,
        text: CandidateText,
        variants: Vec<VariantKey<'r>>,
    },
}

/// The set of CSS properties a candidate's declarations set, encoded as
/// ascending indices into Tailwind's canonical property order — the
/// order in which properties first appear in Tailwind's generated
/// stylesheet. The config analyzer captures these lists verbatim from
/// the `propertySort` that Tailwind's `compileAstNodes` computes per
/// candidate.
///
/// This is the primary Tailwind sort key; `count` and the candidate text
/// only break signature ties.
#[derive(Clone, Debug)]
pub(super) enum Signature {
    /// A generated `SIGNATURE_POOL` entry.
    Pool(&'static [u16]),
    /// A single property, for arbitrary-property candidates
    /// (`[display:block]`).
    Property(u16),
    /// An arbitrary-property candidate Tailwind can't order: a custom
    /// property (`[--my-var:1]`) or an unknown property name. Tailwind
    /// emits an empty `propertySort`, which sorts after every real
    /// property; `as_slice` returns `&[]` so the prefix comparison in
    /// `Ord` places it last, and equal empty signatures let the name
    /// break the tie (`[--my-var:1]` before `[--my-var:2]`).
    CustomProperty,
    /// The signature of a custom `@utility` from the project's
    /// stylesheet, owned by the registry (static utilities) or computed
    /// for the candidate (functional utilities, whose surviving
    /// declarations depend on the value).
    Custom(Arc<[u16]>),
}

impl Signature {
    pub(super) fn as_slice(&self) -> &[u16] {
        match self {
            Self::Pool(indices) => indices,
            Self::Property(index) => std::slice::from_ref(index),
            Self::CustomProperty => &[],
            Self::Custom(indices) => indices,
        }
    }
}

/// Equality goes through `as_slice` so that, per the `Ord` contract, a
/// `Pool` and a `Property` holding the same indices are equal.
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Signature {}

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares the index lists: the first differing index after the shared
/// prefix decides, and when one list is a prefix of the other, the longer
/// list sorts first. `size-4` (height + width) shares its height index
/// with `h-4` (height alone) and wins by length, while `w-4` (width
/// alone) sorts after both on the later width index.
///
/// This walks the shared prefix rather than delegating to slice's own
/// lexicographic `Ord` because the tie-break is inverted: slice order
/// treats a shorter prefix as lesser, but a candidate whose signature is
/// a prefix of another's must sort first here. The walk itself is cheap
/// regardless — `SIGNATURE_POOL` entries top out at 8 indices.
impl Ord for Signature {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b) = (self.as_slice(), other.as_slice());
        let mut i = 0;
        while i < a.len() && i < b.len() && a[i] == b[i] {
            i += 1;
        }
        match (a.get(i), b.get(i)) {
            (Some(a), Some(b)) => a.cmp(b),
            // The longer list wins a shared prefix.
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

/// Orders candidates inside one (variant weight, signature, count)
/// bucket the way Tailwind's own `compare()` does on the whole
/// candidate text: plain code-point order, except that digit sequences
/// compare as integers (`p-75` < `p-700`, `red-50` < `red-100`). Code-point
/// order places `!` and `-` before digits, digits before `[`, and `[`
/// before letters (`-m-4` < `m-4`, `w-4` < `w-[1px]` < `w-auto`,
/// `flex` < `flex!`), matching the order Tailwind emits candidates in —
/// which is why this does not reuse
/// [biome_string_case::CldrAsciiCollator]: CLDR collation places
/// punctuation before digits and interleaves letter case.
///
/// Because the text starts with the variants, two candidates whose
/// variants weigh the same but read differently (`hover:sm:flex` and
/// `sm:hover:block`) order by their variant spelling before their name.
struct TwNameCollator;

impl Collator for TwNameCollator {
    type Char = char;

    fn weight(&self, c: &char) -> impl Ord {
        *c
    }

    fn as_digit(&self, c: &char) -> Option<impl Ord> {
        c.is_ascii_digit().then_some(*c)
    }
}

impl<'r> PendingSortKey<'r> {
    /// Classify a candidate into its utility placement and variants, or
    /// `Unknown` for a shape we can't place.
    fn from_candidate(candidate: &AnyTwFullCandidate, registry: &'r TailwindRegistry) -> Self {
        let AnyTwFullCandidate::TwFullCandidate(node) = candidate else {
            return Self::Unknown;
        };

        // An unrecognized variant leaves the candidate unplaced.
        let Some(variants) = variant_keys_from_candidate(node, registry) else {
            return Self::Unknown;
        };

        match candidate_placement(node, registry) {
            None => Self::Unknown,
            Some((signature, count)) => Self::Known {
                signature,
                count,
                text: CandidateText(node.syntax().clone()),
                variants,
            },
        }
    }

    /// Finish a [SortKey] by weighting the variants against
    /// `variant_groups`.
    fn into_sort_key(self, variant_groups: &VariantGroups) -> SortKey {
        match self {
            Self::Unknown => SortKey::Unknown,
            Self::Known {
                signature,
                count,
                text,
                variants,
            } => {
                let Some(variant_weight) = variant_groups.weight_for(&variants) else {
                    return SortKey::Unknown;
                };
                SortKey::Known {
                    variant_weight,
                    signature,
                    count,
                    text,
                }
            }
        }
    }
}

fn candidate_placement(
    node: &TwFullCandidate,
    registry: &TailwindRegistry,
) -> Option<(Signature, u8)> {
    // The legacy leading `!` and the trailing `!` are each fine on
    // their own; Tailwind rejects a candidate spelling both.
    if node.legacy_important_token().is_some() && node.excl_token().is_some() {
        return None;
    }

    let is_negative = node.negative_token().is_some();

    match node.candidate().ok()? {
        AnyTwCandidate::TwArbitraryCandidate(a) => {
            let property_token = a.property_token().ok()?;
            // An arbitrary property accepts a numeric opacity-style
            // modifier (`[color:red]/50`, `[padding:1px]/2`,
            // `[--my-var:1]/(--x)`) but not a bare word or percentage
            // (`[color:red]/foo`), regardless of the property.
            if let Some(modifier) = a.modifier()
                && !modifier_accepted(ModifierKind::Opacity, &modifier, registry)
            {
                return None;
            }
            // Tailwind places every arbitrary-property candidate. A
            // property in the known order sorts by its index; a custom
            // property (`[--my-var:1]`) or an unknown property name has
            // an empty Tailwind order and sorts after every real
            // property, ordered among themselves by candidate text.
            let signature = PROPERTY_INDEX
                .get(property_token.text_trimmed())
                .map_or(Signature::CustomProperty, |&property_idx| {
                    Signature::Property(property_idx)
                });
            Some((signature, 1))
        }
        AnyTwCandidate::TwBogusCandidate(_) => None,

        AnyTwCandidate::TwStaticCandidate(s) => {
            let name = s.base_token().ok()?;
            let name = name.text_trimmed();
            if let Some(modifier) = s.modifier() {
                // Static registrations take no modifier; the few
                // valid bare-with-modifier forms (`@container/main`
                // names the container, `shadow/50` sets the shadow
                // color opacity) compile through a functional root's
                // bare placement, picked by modifier shape. A functional
                // `@utility` needs a value, so only the preset applies.
                if is_negative {
                    return None;
                }
                let entry = FUNCTIONAL_UTILITIES.get(name)?;
                let placement = if modifier_accepted(ModifierKind::Opacity, &modifier, registry) {
                    entry.bare_opacity
                } else {
                    entry.bare_name
                };
                placement.map(|(sig, count)| (pool_signature(sig), count))
            } else if let Some(placement) = static_placement(name, is_negative, registry) {
                Some(placement)
            } else if !is_negative
                && let Some(entry) = FUNCTIONAL_UTILITIES.get(name)
                && let Some((sig, count)) = entry.bare
            {
                // Functional utilities with a default compile bare
                // (`border`, `ring`, `shadow`) without a static
                // registration.
                Some((pool_signature(sig), count))
            } else {
                None
            }
        }

        AnyTwCandidate::TwFunctionalCandidate(f) => {
            let base = f.base_token().ok()?;
            let base = base.text_trimmed();
            let value = f.value().ok()?;
            let modifier = f.modifier();

            // Tailwind resolves a candidate's full name as a static
            // utility before trying functional roots: `w-full`,
            // `m-auto`, and `justify-center` are static
            // registrations even though the grammar splits them
            // into base and value. Statics take no modifier, so a
            // modifier skips the lookup.
            if modifier.is_none()
                && let Some(text) = named_text(&value)
                && let Some(placement) =
                    joined_static_placement(base, text.text(), is_negative, registry)
            {
                return Some(placement);
            }

            let builtin = FUNCTIONAL_UTILITIES.get(base).and_then(|entry| {
                builtin_functional_placement(
                    entry,
                    &value,
                    modifier.as_ref(),
                    is_negative,
                    registry,
                )
            });
            // `@utility` never registers a negative form.
            let custom = if is_negative {
                None
            } else {
                registry.custom_functional(base).and_then(|entry| {
                    resolve_custom_functional(
                        entry,
                        CandidateValue::Node(&value),
                        modifier.as_ref(),
                        registry,
                    )
                })
            };
            match merge_placements(builtin, custom) {
                Some(placement) => Some(placement),
                // The Tailwind lexer splits a dashed custom root it has
                // never heard of at the first `-` (`my-thing-4` → `my` +
                // `thing-4`), so rejoin the base with the value's leading
                // segments and probe the custom roots.
                None if !is_negative && registry.has_custom_functional() => {
                    let text = named_or_number_text(&value)?;
                    dashed_custom_functional(base, text.text(), modifier.as_ref(), registry)
                }
                None => None,
            }
        }
    }
}

fn pool_signature(idx: u16) -> Signature {
    Signature::Pool(SIGNATURE_POOL[usize::from(idx)])
}

/// A static utility's placement, merging the preset's registration with
/// a custom `@utility` of the same name: Tailwind compiles both and
/// positions the candidate by whichever sorts first.
#[inline]
fn static_placement(
    name: &str,
    is_negative: bool,
    registry: &TailwindRegistry,
) -> Option<(Signature, u8)> {
    let builtin = STATIC_UTILITIES
        .get(name)
        // Tailwind registers negative statics individually
        // (`-m-px` exists, `-flex` does not); `@utility` never does.
        .filter(|entry| !is_negative || entry.has_negative)
        .map(|entry| (pool_signature(entry.sig), entry.count));
    let custom = if is_negative {
        None
    } else {
        registry
            .custom_static(name)
            .map(|entry| (Signature::Custom(entry.sig.clone()), entry.count))
    };
    merge_placements(builtin, custom)
}

/// Of two placements the same candidate compiles to, the one Tailwind
/// positions the candidate by: the earlier signature, then the wider
/// utility. Inlined so the empty-registry case (`right` always `None`)
/// collapses to the builtin lookup.
#[inline]
fn merge_placements(
    left: Option<(Signature, u8)>,
    right: Option<(Signature, u8)>,
) -> Option<(Signature, u8)> {
    match (left, right) {
        (Some(left), Some(right)) => {
            let earlier = left
                .0
                .cmp(&right.0)
                .then_with(|| right.1.cmp(&left.1))
                .is_le();
            Some(if earlier { left } else { right })
        }
        (left, right) => left.or(right),
    }
}

/// The preset's placement of a functional candidate, through the named
/// or arbitrary branches of its root.
fn builtin_functional_placement(
    entry: &FunctionalEntry,
    value: &AnyTwValue,
    modifier: Option<&AnyTwModifier>,
    is_negative: bool,
    registry: &TailwindRegistry,
) -> Option<(Signature, u8)> {
    let (named_branches, arbitrary_branches) = if is_negative {
        match entry.negative? {
            Negative::SameBranches => (entry.named_branches, entry.arbitrary_branches),
            Negative::Distinct {
                named_branches,
                arbitrary_branches,
            } => (named_branches, arbitrary_branches),
        }
    } else {
        (entry.named_branches, entry.arbitrary_branches)
    };
    let resolved = match value {
        AnyTwValue::TwArbitraryValue(arb) => {
            resolve_arbitrary_branch(arbitrary_branches, Some(&arb.value()), modifier, registry)
        }
        // `w-(--x)` is Tailwind's shorthand for `w-[var(--x)]`: an
        // arbitrary value no data-type predicate matches, so it takes
        // the root's type-blind branch.
        AnyTwValue::TwCssVariableValue(_) => {
            resolve_arbitrary_branch(arbitrary_branches, None, modifier, registry)
        }
        _ => resolve_named_branch(named_branches, value, modifier, registry),
    };
    resolved.map(|(sig, count)| (pool_signature(sig), count))
}

/// The longest static utility name in the preset
/// (`font-stretch-ultra-condensed`, 28 bytes), rounded up; the joined
/// `base-value` probe stops there unless the stylesheet registered a
/// longer name.
const LONGEST_STATIC_NAME: usize = 32;

/// Longest name the joined-name probes reassemble on the stack; a
/// custom name past this is still found through the heap fallback.
const JOIN_BUFFER: usize = 64;

/// Look up `base-value` as a static utility, reassembling the name in a
/// stack buffer rather than allocating.
fn joined_static_placement(
    base: &str,
    value: &str,
    is_negative: bool,
    registry: &TailwindRegistry,
) -> Option<(Signature, u8)> {
    let len = base.len() + 1 + value.len();
    if len > LONGEST_STATIC_NAME.max(registry.longest_static_name) {
        return None;
    }
    let mut buf = [0u8; JOIN_BUFFER];
    match join_dashed(&mut buf, base, value) {
        Some(name) => static_placement(name, is_negative, registry),
        None => static_placement(&[base, "-", value].concat(), is_negative, registry),
    }
}

/// `base-value` in `buf`, or `None` when it does not fit.
#[inline]
fn join_dashed<'b>(buf: &'b mut [u8], base: &str, value: &str) -> Option<&'b str> {
    let mid = base.len() + 1;
    if mid > buf.len() {
        return None;
    }
    buf[..base.len()].copy_from_slice(base.as_bytes());
    buf[base.len()] = b'-';
    let (_, rest) = buf.split_at_mut(mid);
    let tail_len = join_into(rest, value, "")?.len();
    str::from_utf8(&buf[..mid + tail_len]).ok()
}

/// A candidate value for custom-functional resolution: the parsed node,
/// or the text tail left after rejoining a dashed custom root.
#[derive(Copy, Clone)]
enum CandidateValue<'a> {
    Node(&'a AnyTwValue),
    Text(&'a str),
}

impl CandidateValue<'_> {
    /// The value's text when it is a named, number, or percentage value
    /// (`4`, `github`, `50%`); `None` for arbitrary and variable values.
    fn named_text(self) -> Option<TokenText> {
        match self {
            Self::Node(value) => match value {
                AnyTwValue::TwNamedValue(_) | AnyTwValue::TwNumberValue(_) => {
                    named_or_number_text(value)
                }
                AnyTwValue::TwPercentageValue(percentage) => percentage
                    .value_token()
                    .ok()
                    .map(|token| token.token_text_trimmed()),
                _ => None,
            },
            Self::Text(_) => None,
        }
    }

    fn arbitrary(self) -> Option<CssGenericComponentValueList> {
        match self {
            Self::Node(AnyTwValue::TwArbitraryValue(arb)) => Some(arb.value()),
            _ => None,
        }
    }

    /// Whether the value is a bare number (`4`, `1.5`), and whether it is
    /// an integer.
    fn number_kind(self) -> Option<bool> {
        match self {
            Self::Node(AnyTwValue::TwNumberValue(number)) => {
                let token = number.value_token().ok()?;
                Some(!token.text_trimmed().contains('.'))
            }
            Self::Text(text) => {
                let mut chars = text.chars();
                let first = chars.next()?;
                if !first.is_ascii_digit() {
                    return None;
                }
                let mut seen_dot = false;
                for c in chars {
                    match c {
                        '0'..='9' => {}
                        '.' if !seen_dot => seen_dot = true,
                        _ => return None,
                    }
                }
                Some(!seen_dot)
            }
            _ => None,
        }
    }

    fn is_percentage(self) -> bool {
        match self {
            Self::Node(value) => matches!(value, AnyTwValue::TwPercentageValue(_)),
            Self::Text(text) => text
                .strip_suffix('%')
                .is_some_and(|number| CandidateValue::Text(number).number_kind().is_some()),
        }
    }
}

/// Resolve a candidate against a custom `@utility name-* { … }` the way
/// Tailwind's functional `compileFn` does: every declaration whose
/// `--value(…)` / `--modifier(…)` resolves survives, the rest are
/// dropped, and the candidate is invalid when a used `--value(…)` or
/// `--modifier(…)` resolved nowhere, when a modifier was given that
/// nothing consumed, or when a fraction value meets a modifier. The
/// surviving declarations give the signature and count.
fn resolve_custom_functional(
    entry: &RegisteredFunctional,
    value: CandidateValue<'_>,
    modifier: Option<&AnyTwModifier>,
    registry: &TailwindRegistry,
) -> Option<(Signature, u8)> {
    let modifier_value = match modifier {
        None => None,
        Some(AnyTwModifier::TwModifier(modifier)) => Some(modifier.value().ok()?),
        Some(_) => return None,
    };
    let modifier_is_number = modifier_value
        .as_ref()
        .is_some_and(|value| matches!(value, AnyTwValue::TwNumberValue(_)));

    let mut used_value = false;
    let mut resolved_value = false;
    let mut used_modifier = false;
    let mut resolved_modifier = false;
    let mut is_ratio = false;
    // (declaration, resolved a non-ratio value) for the survivors.
    let mut survivors: SmallVec<[(&FunctionalDecl, bool); 8]> = SmallVec::new();

    for decl in &entry.decls {
        let mut non_ratio_value = false;
        if let Some(shapes) = &decl.value {
            used_value = true;
            match resolve_value_shapes(shapes, value, modifier_is_number, registry) {
                Some(ratio) => {
                    resolved_value = true;
                    is_ratio |= ratio;
                    non_ratio_value = !ratio;
                }
                None => continue,
            }
        }
        if let Some(shapes) = &decl.modifier {
            let Some(modifier_value) = &modifier_value else {
                continue;
            };
            used_modifier = true;
            if resolve_value_shapes(
                shapes,
                CandidateValue::Node(modifier_value),
                false,
                registry,
            )
            .is_none()
            {
                continue;
            }
            resolved_modifier = true;
        }
        survivors.push((decl, non_ratio_value));
    }

    if (used_value && !resolved_value)
        || (used_modifier && !resolved_modifier)
        || (is_ratio && resolved_modifier)
        || (modifier.is_some() && !is_ratio && !resolved_modifier)
    {
        return None;
    }
    // No emptiness guard here on purpose: a modifier-only utility on a
    // candidate with no modifier drops every declaration, and Tailwind
    // still compiles it — to an empty node set that `getClassOrder`
    // sorts last, exactly the empty signature this returns.
    // A fraction (`aspect-1/2`) keeps only the declarations that
    // resolved it as a ratio.
    if is_ratio {
        survivors.retain(|(_, non_ratio_value)| !*non_ratio_value);
    }

    let mut indices: SmallVec<[u16; 8]> = SmallVec::new();
    for (decl, _) in &survivors {
        if let Some(hint) = decl.sort_hint {
            indices.push(hint);
            break;
        }
        indices.extend(decl.property);
    }
    indices.sort_unstable();
    indices.dedup();
    let count = u8::try_from(survivors.len()).unwrap_or(u8::MAX);
    Some((Signature::Custom(Arc::from(indices.as_slice())), count))
}

/// Match `value` against `--value(…)` argument shapes in order;
/// `Some(is_ratio)` for the first shape that accepts it.
fn resolve_value_shapes(
    shapes: &[ValueShape],
    value: CandidateValue<'_>,
    modifier_is_number: bool,
    registry: &TailwindRegistry,
) -> Option<bool> {
    for shape in shapes {
        match shape {
            ValueShape::Literal(literal) => {
                if value
                    .named_text()
                    .is_some_and(|text| text.text() == literal.as_ref())
                    || matches!(value, CandidateValue::Text(text) if text == literal.as_ref())
                {
                    return Some(false);
                }
            }
            ValueShape::Theme(prefix) => {
                let matched = match value {
                    CandidateValue::Text(text) => registry.theme_var_contains(prefix, text),
                    CandidateValue::Node(_) => value
                        .named_text()
                        .is_some_and(|text| registry.theme_var_contains(prefix, text.text())),
                };
                if matched {
                    return Some(false);
                }
            }
            ValueShape::Bare(BareValueType::Ratio) => {
                if modifier_is_number && value.number_kind().is_some() {
                    return Some(true);
                }
            }
            ValueShape::Bare(BareValueType::Integer) => {
                if value.number_kind() == Some(true) {
                    return Some(false);
                }
            }
            ValueShape::Bare(BareValueType::Number) => {
                if value.number_kind().is_some() {
                    return Some(false);
                }
            }
            ValueShape::Bare(BareValueType::Percentage) => {
                if value.is_percentage() {
                    return Some(false);
                }
            }
            ValueShape::Arbitrary(data_type) => {
                if let Some(list) = value.arbitrary()
                    && data_type.is_none_or(|data_type| value_matches_type(&list, data_type))
                {
                    return Some(false);
                }
            }
        }
    }
    None
}

/// `my-thing-4` lexed as base `my`, value `thing-4`: probe `my-thing`
/// (then `my-thing-…`) as a custom functional root and resolve the
/// remaining text as the value.
fn dashed_custom_functional(
    base: &str,
    value: &str,
    modifier: Option<&AnyTwModifier>,
    registry: &TailwindRegistry,
) -> Option<(Signature, u8)> {
    let max_len = registry.longest_functional_name;
    let mut buf = [0u8; JOIN_BUFFER];
    let mut best: Option<(&RegisteredFunctional, usize)> = None;
    for (index, _) in value.match_indices('-') {
        let root_len = base.len() + 1 + index;
        if root_len > max_len {
            break;
        }
        let Some(root) = join_dashed(&mut buf, base, &value[..index]) else {
            break;
        };
        if let Some(entry) = registry.custom_functional(root) {
            best = Some((entry, index + 1));
        }
    }
    let (entry, rest) = best?;
    resolve_custom_functional(
        entry,
        CandidateValue::Text(&value[rest..]),
        modifier,
        registry,
    )
}

fn compare(a: &SortKey, b: &SortKey) -> Ordering {
    match (a, b) {
        // Unknowns float to the front; relative order between unknowns is
        // preserved by the stable sort.
        (SortKey::Unknown, SortKey::Unknown) => Ordering::Equal,
        (SortKey::Unknown, SortKey::Known { .. }) => Ordering::Less,
        (SortKey::Known { .. }, SortKey::Unknown) => Ordering::Greater,
        (
            SortKey::Known {
                variant_weight: v1,
                signature: s1,
                count: c1,
                text: t1,
            },
            SortKey::Known {
                variant_weight: v2,
                signature: s2,
                count: c2,
                text: t2,
            },
            // Variants sort outermost — a plain utility before any
            // variant (`flex hover:flex sm:flex`).
        ) => v1
            .cmp(v2)
            .then_with(|| s1.cmp(s2))
            // Wider utilities (e.g. `sr-only` setting 9 properties) win
            // a signature tie so they sort before narrower utilities.
            .then_with(|| c2.cmp(c1))
            // Bucket ties order by the whole candidate text, the way
            // Tailwind emits them (`m-2 m-4 m-auto m-px`,
            // `collapse invisible visible`, `flex flex!`).
            .then_with(|| t1.compare(t2)),
    }
}

/// Does this utility's branch list declare a `NamedBranch::Typed(Ratio)` slot?
/// Used as the gate for fraction-aliased modifiers (`w-1/2` ↔ `w-[50%]`) —
/// mirrors Tailwind's `supportsFractions` flag
fn entry_has_ratio_branch(branches: &[NamedBranch]) -> bool {
    branches
        .iter()
        .any(|b| matches!(b, NamedBranch::Typed(NamedValueType::Ratio, ..)))
}

/// `n/m` Tailwind fraction shorthand: the value is a bare number, the
/// modifier is a bare number, and the utility actually accepts fractions.
fn is_fraction_modifier(
    value: &AnyTwValue,
    modifier: &AnyTwModifier,
    branches: &[NamedBranch],
) -> bool {
    let AnyTwModifier::TwModifier(m) = modifier else {
        return false;
    };
    matches!(value, AnyTwValue::TwNumberValue(_))
        && matches!(m.value(), Ok(AnyTwValue::TwNumberValue(_)))
        && entry_has_ratio_branch(branches)
}

fn named_text(value: &AnyTwValue) -> Option<TokenText> {
    let AnyTwValue::TwNamedValue(named) = value else {
        return None;
    };
    named
        .value_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

fn named_or_number_text(value: &AnyTwValue) -> Option<TokenText> {
    match value {
        AnyTwValue::TwNamedValue(named) => named
            .value_token()
            .ok()
            .map(|token| token.token_text_trimmed()),
        AnyTwValue::TwNumberValue(number) => number
            .value_token()
            .ok()
            .map(|token| token.token_text_trimmed()),
        _ => None,
    }
}

fn named_value_type_matches(
    value_type: NamedValueType,
    value: &AnyTwValue,
    has_fraction_modifier: bool,
) -> bool {
    matches!(
        (value_type, value, has_fraction_modifier),
        (NamedValueType::Number, AnyTwValue::TwNumberValue(_), false)
            | (
                NamedValueType::Percentage,
                AnyTwValue::TwPercentageValue(_),
                false
            )
            | (NamedValueType::Ratio, AnyTwValue::TwNumberValue(_), true)
    )
}

/// Walk a basename's named branch list and return the first matching
/// branch's `(property_idx, property_count)` placement. Branch order in
/// the preset already reflects the resolution precedence we want
/// (Keyword → Theme → Typed).
fn resolve_named_branch(
    branches: &[NamedBranch],
    value: &AnyTwValue,
    modifier: Option<&AnyTwModifier>,
    registry: &TailwindRegistry,
) -> Option<(u16, u8)> {
    let has_fraction_modifier = match modifier {
        None => false,
        Some(m) if is_fraction_modifier(value, m, branches) => true,
        Some(AnyTwModifier::TwModifier(_)) => false,
        Some(AnyTwModifier::TwBogusModifier(_)) => return None,
    };

    for &branch in branches {
        let (modifier_kind, property_idx, property_count) = match branch {
            NamedBranch::Theme(namespace, m, p, c) => {
                if has_fraction_modifier {
                    continue;
                }
                let Some(text) = named_or_number_text(value) else {
                    continue;
                };
                if !registry.theme_contains(namespace, text.text()) {
                    continue;
                }
                (m, p, c)
            }
            NamedBranch::Keyword(pool_idx, m, p, c) => {
                let Some(text) = named_text(value) else {
                    continue;
                };
                if !KEYWORD_POOL[usize::from(pool_idx)].contains(&text.text()) {
                    continue;
                }
                (m, p, c)
            }
            NamedBranch::Typed(value_type, m, p, c) => {
                if !named_value_type_matches(value_type, value, has_fraction_modifier) {
                    continue;
                }
                (m, p, c)
            }
        };
        if !has_fraction_modifier
            && let Some(modifier) = modifier
            && !modifier_accepted(modifier_kind, modifier, registry)
        {
            return None;
        }
        return Some((property_idx, property_count));
    }
    None
}

/// Whether `modifier` is a well-formed `/modifier` for a branch that accepts
/// `kind`. Opacity takes a number, arbitrary value, or CSS variable;
/// line-height takes those plus a `--leading-*` theme keyword. A percentage
/// or a bare word is never a valid modifier.
fn modifier_accepted(
    kind: ModifierKind,
    modifier: &AnyTwModifier,
    registry: &TailwindRegistry,
) -> bool {
    let AnyTwModifier::TwModifier(modifier) = modifier else {
        return false;
    };
    let Ok(value) = modifier.value() else {
        return false;
    };
    match kind {
        ModifierKind::None => false,
        ModifierKind::Opacity => is_numeric_modifier(&value),
        ModifierKind::LineHeight => {
            is_numeric_modifier(&value)
                || matches!(&value, AnyTwValue::TwNamedValue(_))
                    && named_text(&value).is_some_and(|text| {
                        registry.theme_contains(ThemeNamespace::Leading, text.text())
                    })
        }
    }
}

/// A number (`/50`), an arbitrary value (`/[0.5]`), or a CSS variable
/// (`/(--x)`) — the modifier value kinds every modifier-accepting branch
/// allows.
fn is_numeric_modifier(value: &AnyTwValue) -> bool {
    matches!(
        value,
        AnyTwValue::TwNumberValue(_)
            | AnyTwValue::TwArbitraryValue(_)
            | AnyTwValue::TwCssVariableValue(_)
    )
}

/// Walk a basename's arbitrary branch list and return the first matching
/// branch's `(property_idx, property_count)` placement. Typed branches
/// precede the type-blind fallback in generated preset order. A modifier is
/// gated against the matched branch just as in [resolve_named_branch]: an
/// arbitrary value on a branch that takes no modifier (`p-[4px]/2`,
/// `w-[1px]/foo`) is not a real candidate.
fn resolve_arbitrary_branch(
    branches: &[ArbitraryBranch],
    list: Option<&CssGenericComponentValueList>,
    modifier: Option<&AnyTwModifier>,
    registry: &TailwindRegistry,
) -> Option<(u16, u8)> {
    for &branch in branches {
        let (modifier_kind, property_idx, property_count) = match branch {
            ArbitraryBranch::Typed(value_type, m, p, c) => {
                if !list.is_some_and(|list| value_matches_type(list, value_type)) {
                    continue;
                }
                (m, p, c)
            }
            ArbitraryBranch::Fallback(m, p, c) => (m, p, c),
        };
        if let Some(modifier) = modifier
            && !modifier_accepted(modifier_kind, modifier, registry)
        {
            return None;
        }
        return Some((property_idx, property_count));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_tailwind_parser::parse_tailwind;

    /// A known key with the given placement whose text is the placeholder
    /// candidate `x`, for tests that only exercise the placement.
    fn known(property_idx: u16, property_count: u8) -> SortKey {
        let parsed = parse_tailwind("x");
        let candidate = parsed.tree().candidates().iter().next().unwrap();
        SortKey::Known {
            variant_weight: VariantWeight::default(),
            signature: Signature::Property(property_idx),
            count: property_count,
            text: CandidateText(candidate.syntax().clone()),
        }
    }

    /// The candidate text of a known key, materialized for assertion
    /// messages.
    fn name_text(key: &SortKey) -> String {
        let SortKey::Known { text, .. } = key else {
            panic!("expected a known key");
        };
        text.text().to_string()
    }

    fn nat_cmp(a: &str, b: &str) -> Ordering {
        TwNameCollator.cmp(a.chars(), b.chars())
    }

    fn classify(input: &str) -> SortKey {
        let reg = TailwindRegistry::default();
        let parsed = parse_tailwind(input);
        let full = parsed.tree().candidates().iter().next().unwrap();
        let pending = PendingSortKey::from_candidate(&full, &reg);
        let variants: &[VariantKey<'_>] = match &pending {
            PendingSortKey::Known { variants, .. } => variants,
            PendingSortKey::Unknown => &[],
        };
        let groups = VariantGroups::new(variants, &reg);
        pending.into_sort_key(&groups)
    }

    /// Classify a whole class list against one shared [VariantGroups], the
    /// way `sort_class_list` does. Needed to exercise variant-against-variant
    /// ordering: [classify] builds groups per candidate, so every lone
    /// variant lands in group 0 and compares equal to any other.
    #[test]
    fn candidate_text_compare_matches_the_collator() {
        // Every pair from a corpus that hits the fast path's edges: shared
        // variant prefixes, digit runs cut at the divergence (`p-12` /
        // `p-1a`), prefixes of one another, divergence inside a multi-byte
        // character (`w-é` / `w-è` share the lead byte), and both `!`
        // spellings.
        let corpus = "p-1 p-12 p-1a p-2 p-4 p-40 p-400 p-4! !p-4 hover:p-4 hover:p-2 \
            hover:sm:flex sm:hover:flex sm:hover:block hover:sm:block \
            @sm:block @sm/main:flex @sm:flex bg-red-500/50 bg-red-500/[.35] \
            bg-red-500/[.3] w-1/2 w-1/3 w-[1px] w-[10px] w-auto -m-4 m-4 -m-px \
            [color:red] [color:red]! w-é w-è w-éa w-éb w-ü w-ǖ flex \
            flex! !flex text-red-500 text-red-50 text-red-5000 p-4/50 hover:!p-4";
        let parsed = parse_tailwind(corpus);
        let texts: Vec<CandidateText> = parsed
            .tree()
            .candidates()
            .iter()
            .map(|c| CandidateText(c.syntax().clone()))
            .collect();
        for a in &texts {
            for b in &texts {
                let expected = TwNameCollator.cmp(a.text().chars(), b.text().chars());
                assert_eq!(a.compare(b), expected, "{} vs {}", a.text(), b.text());
            }
        }
    }

    /// Sort a class string end to end.
    fn sort(input: &str) -> String {
        let reg = TailwindRegistry::default();
        sort_class_list(&parse_tailwind(input).tree(), &reg)
    }

    fn classify_all(input: &str) -> Vec<SortKey> {
        let reg = TailwindRegistry::default();
        let parsed = parse_tailwind(input);
        let pending: Vec<PendingSortKey> = parsed
            .tree()
            .candidates()
            .iter()
            .map(|candidate| PendingSortKey::from_candidate(&candidate, &reg))
            .collect();
        let groups = VariantGroups::new(
            pending
                .iter()
                .filter_map(|key| match key {
                    PendingSortKey::Known { variants, .. } => Some(variants.as_slice()),
                    PendingSortKey::Unknown => None,
                })
                .flatten(),
            &reg,
        );
        pending
            .into_iter()
            .map(|key| key.into_sort_key(&groups))
            .collect()
    }

    fn functional_parts(input: &str) -> (AnyTwValue, Option<AnyTwModifier>) {
        let parsed = parse_tailwind(input);
        let full = parsed.tree().candidates().iter().next().unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        (functional.value().unwrap(), functional.modifier())
    }

    // region: compare

    #[test]
    fn compare_unknown_is_less_than_known() {
        assert_eq!(compare(&SortKey::Unknown, &known(5, 1)), Ordering::Less);
        assert_eq!(compare(&known(5, 1), &SortKey::Unknown), Ordering::Greater);
    }

    #[test]
    fn compare_unknown_pair_is_equal_so_stable_sort_keeps_input_order() {
        assert_eq!(
            compare(&SortKey::Unknown, &SortKey::Unknown),
            Ordering::Equal
        );
    }

    #[test]
    fn compare_orders_by_property_idx_ascending() {
        assert_eq!(compare(&known(3, 1), &known(5, 1)), Ordering::Less);
    }

    #[test]
    fn compare_breaks_property_idx_tie_by_property_count_descending() {
        // sr-only-shape utility (count=9) wins over a single-property one.
        let wider = known(5, 9);
        let narrow = known(5, 1);
        assert_eq!(compare(&wider, &narrow), Ordering::Less);
    }

    #[test]
    fn compare_breaks_bucket_tie_by_name() {
        // Distinct static utilities in one visibility bucket order by
        // name, the way Tailwind emits them — not by registration order,
        // which would put `visible` first.
        let collapse = classify("collapse");
        let visible = classify("visible");
        assert_eq!(compare(&collapse, &visible), Ordering::Less);
        assert_eq!(compare(&visible, &collapse), Ordering::Greater);
    }

    #[test]
    fn compare_returns_equal_for_identical_known_keys() {
        assert_eq!(compare(&known(5, 1), &known(5, 1)), Ordering::Equal);
    }

    #[test]
    fn compare_breaks_exact_key_tie_plain_before_important() {
        let plain = classify("flex");
        let important = classify("flex!");
        assert_eq!(compare(&plain, &important), Ordering::Less);
        assert_eq!(compare(&important, &plain), Ordering::Greater);
    }

    #[test]
    fn compare_breaks_bucket_tie_by_variant_spelling_before_name() {
        // `hover:sm:` and `sm:hover:` weigh the same, so Tailwind falls
        // through to the whole candidate text, where the variant spelling
        // comes before the utility name.
        assert_eq!(
            sort("sm:hover:block hover:sm:flex"),
            "hover:sm:flex sm:hover:block"
        );
        assert_eq!(sort("@sm:block @sm/main:flex"), "@sm/main:flex @sm:block");
    }

    #[test]
    fn compare_orders_by_first_differing_signature_index() {
        // `container` touches `--tw-container-component`, which precedes
        // `display` in the property order.
        let container = classify("container");
        let flex = classify("flex");
        assert_eq!(compare(&container, &flex), Ordering::Less);
    }

    #[test]
    fn compare_longer_signature_wins_a_shared_prefix() {
        // `size-4` = height + width; `h-4` = height alone. The longer
        // list sorts first, and `w-4` = width alone sorts after both.
        let size = classify("size-4");
        let height = classify("h-4");
        let width = classify("w-4");
        assert_eq!(compare(&size, &height), Ordering::Less);
        assert_eq!(compare(&height, &width), Ordering::Less);
    }

    #[test]
    fn compare_breaks_bucket_tie_by_name_before_importance() {
        // `p-2! p-4`: the name decides; the important suffix does not
        // pull `p-4` ahead of `p-2!`.
        let important_two = classify("p-2!");
        let plain_four = classify("p-4");
        assert_eq!(compare(&important_two, &plain_four), Ordering::Less);
    }

    #[test]
    fn compare_breaks_value_tie_by_modifier() {
        let twenty_five = classify("bg-red-500/25");
        let fifty = classify("bg-red-500/50");
        assert_eq!(compare(&twenty_five, &fifty), Ordering::Less);
        assert_eq!(compare(&fifty, &fifty), Ordering::Equal);
    }

    #[test]
    fn compare_puts_negatives_before_positives_in_one_bucket() {
        // `-m-4 m-2`: the sign participates in the name, and `-`
        // precedes every letter in code-point order.
        let negative = classify("-m-4");
        let positive = classify("m-2");
        assert_eq!(compare(&negative, &positive), Ordering::Less);
    }

    // endregion: compare

    // region: natural comparison

    #[test]
    fn collator_compares_digit_sequences_numerically() {
        assert_eq!(nat_cmp("2", "10"), Ordering::Less);
        assert_eq!(nat_cmp("red-50", "red-100"), Ordering::Less);
        // The digit sequence is compared as a whole number, not from the
        // point of divergence: `75` < `700` even though `5` > `0`.
        assert_eq!(nat_cmp("75", "700"), Ordering::Less);
        assert_eq!(nat_cmp("[2px]", "[10rem]"), Ordering::Less);
    }

    #[test]
    fn collator_uses_code_point_order_outside_digit_sequences() {
        // Digits precede `[`, and `[` precedes letters.
        assert_eq!(nat_cmp("4", "[1px]"), Ordering::Less);
        assert_eq!(nat_cmp("[13px]", "auto"), Ordering::Less);
        assert_eq!(nat_cmp("2xl", "base"), Ordering::Less);
        assert_eq!(nat_cmp("bold", "light"), Ordering::Less);
    }

    #[test]
    fn collator_puts_prefixes_first() {
        assert_eq!(nat_cmp("", "sm"), Ordering::Less);
        assert_eq!(nat_cmp("in", "in-out"), Ordering::Less);
        assert_eq!(nat_cmp("1", "1.5"), Ordering::Less);
        assert_eq!(nat_cmp("sm", "sm"), Ordering::Equal);
    }

    // endregion: natural comparison

    // region: branch resolution

    #[test]
    fn resolve_named_branch_returns_first_matching_branch() {
        // Two NamedBranch::Typed(Number) branches with different property_idx;
        // first one to match wins.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("p-5");
        let branches = &[
            NamedBranch::Typed(NamedValueType::Number, ModifierKind::None, 10, 1),
            NamedBranch::Typed(NamedValueType::Number, ModifierKind::None, 20, 1),
        ];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_classifies_value_internally() {
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("p-5");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Number,
            ModifierKind::None,
            10,
            1,
        )];

        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_skips_typed_when_matcher_returns_false_then_falls_back() {
        let reg = TailwindRegistry::default();
        let branches = &[
            ArbitraryBranch::Typed(CssDataType::Number, ModifierKind::None, 10, 1),
            ArbitraryBranch::Fallback(ModifierKind::None, 20, 1),
        ];
        let full = parse_tailwind("p-[10px]")
            .tree()
            .candidates()
            .iter()
            .next()
            .unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };
        assert_eq!(
            resolve_arbitrary_branch(branches, Some(&arbitrary.value()), None, &reg),
            Some((20, 1))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_skips_named_branches_by_construction() {
        let reg = TailwindRegistry::default();
        let full = parse_tailwind("p-[10px]")
            .tree()
            .candidates()
            .iter()
            .next()
            .unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Fallback(ModifierKind::None, 20, 1)];
        assert_eq!(
            resolve_arbitrary_branch(branches, Some(&arbitrary.value()), None, &reg),
            Some((20, 1))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_rejects_a_modifier_on_a_branch_that_takes_none() {
        // `p-[4px]/2`: the padding arbitrary branch accepts no modifier, so
        // the `/2` makes the candidate invalid.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("p-[4px]/2");
        let AnyTwValue::TwArbitraryValue(arb) = &value else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Fallback(ModifierKind::None, 20, 1)];
        assert_eq!(
            resolve_arbitrary_branch(branches, Some(&arb.value()), modifier.as_ref(), &reg),
            None
        );
    }

    #[test]
    fn resolve_arbitrary_branch_accepts_a_numeric_modifier_on_an_opacity_branch() {
        // `bg-[#fff]/50`: an arbitrary color takes a numeric opacity modifier.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("bg-[#fff]/50");
        let AnyTwValue::TwArbitraryValue(arb) = &value else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Typed(
            CssDataType::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_arbitrary_branch(branches, Some(&arb.value()), modifier.as_ref(), &reg),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_returns_none_when_kind_does_not_match_value_type() {
        // A named value like "abc" never satisfies NamedBranch::Typed(Number)
        // because dispatch is by parser node kind, not text scanning.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("p-abc");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Number,
            ModifierKind::None,
            1,
            1,
        )];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            None
        );
    }

    #[test]
    fn resolve_named_branch_ratio_matches_ratio_typed_branch() {
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("w-1/2");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Ratio,
            ModifierKind::None,
            7,
            1,
        )];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            Some((7, 1))
        );
    }

    #[test]
    fn resolve_named_branch_percentage_only_matches_percentage_typed_branch() {
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("from-25%");
        let branches = &[
            NamedBranch::Typed(NamedValueType::Number, ModifierKind::None, 1, 1),
            NamedBranch::Typed(NamedValueType::Percentage, ModifierKind::None, 2, 1),
        ];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            Some((2, 1))
        );
    }

    #[test]
    fn resolve_named_branch_accepts_valid_opacity_modifier() {
        // A numeric opacity modifier is valid on a color branch.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("bg-red-500/50");
        let branches = &[NamedBranch::Theme(
            ThemeNamespace::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_rejects_a_modifier_on_a_branch_that_takes_none() {
        // `w-1/foo`: the number branch takes no modifier, so the `/foo`
        // modifier makes the candidate invalid rather than sorting as `w-1`.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("w-1/foo");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Number,
            ModifierKind::None,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            None
        );
    }

    #[test]
    fn resolve_named_branch_rejects_a_bare_word_opacity_modifier() {
        // `bg-red-500/foo`: a color branch takes a numeric opacity modifier,
        // not a bare word.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("bg-red-500/foo");
        let branches = &[NamedBranch::Theme(
            ThemeNamespace::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), &reg),
            None
        );
    }

    #[test]
    fn resolve_named_branch_accepts_a_leading_keyword_for_line_height() {
        // `text-lg/loose` is a valid line-height modifier (`loose` is a
        // `--leading-*` keyword).
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("text-lg/loose");
        let line_height = &[NamedBranch::Theme(
            ThemeNamespace::Text,
            ModifierKind::LineHeight,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(line_height, &value, modifier.as_ref(), &reg),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_rejects_a_leading_keyword_on_an_opacity_branch() {
        // `bg-red-500/loose`: a leading keyword is not a valid opacity modifier.
        let reg = TailwindRegistry::default();
        let (value, modifier) = functional_parts("bg-red-500/loose");
        let opacity = &[NamedBranch::Theme(
            ThemeNamespace::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(opacity, &value, modifier.as_ref(), &reg),
            None
        );
    }

    // endregion: branch resolution

    // region: sort key classification

    #[test]
    fn arbitrary_candidate_takes_signature_from_property_index() {
        let display_idx = *PROPERTY_INDEX.get("display").unwrap();
        let key = classify("[display:block]");
        let SortKey::Known {
            signature, count, ..
        } = &key
        else {
            panic!("expected a plain known key");
        };
        assert_eq!(*signature, Signature::Property(display_idx));
        assert_eq!(*count, 1);
        assert_eq!(name_text(&key), "[display:block]");
    }

    #[test]
    fn custom_and_unknown_arbitrary_properties_sort_after_real_ones() {
        // Tailwind places every arbitrary-property candidate, but gives a
        // custom property (`--my-var`) or an unknown property name an empty
        // order, so it sorts after any candidate whose property is in the
        // known order.
        let display = classify("[display:block]");
        let custom = classify("[--my-var:1]");
        let unknown = classify("[foobar:1]");
        assert!(matches!(custom, SortKey::Known { .. }));
        assert!(matches!(unknown, SortKey::Known { .. }));
        assert_eq!(compare(&display, &custom), Ordering::Less);
        assert_eq!(compare(&display, &unknown), Ordering::Less);
        // Empty-order candidates tie on signature and order by name text.
        assert_eq!(
            compare(&classify("[--my-var:1]"), &classify("[--my-var:2]")),
            Ordering::Less
        );
    }

    #[test]
    fn important_suffix_is_position_neutral_in_the_key() {
        let SortKey::Known {
            variant_weight,
            signature,
            count,
            ..
        } = classify("flex")
        else {
            panic!("expected `flex` to classify as a plain known key");
        };
        let SortKey::Known {
            variant_weight: important_weight,
            signature: important_signature,
            count: important_count,
            text,
        } = classify("flex!")
        else {
            panic!("expected `flex!` to classify as a known key");
        };
        assert_eq!(important_weight, variant_weight);
        assert_eq!(important_signature, signature);
        assert_eq!(important_count, count);
        assert_eq!(text.text(), "flex!");
    }

    #[test]
    fn important_suffix_classifies_functional_and_arbitrary_candidates() {
        assert!(matches!(classify("p-4!"), SortKey::Known { .. }));
        assert!(matches!(
            classify("[display:block]!"),
            SortKey::Known { .. }
        ));
        assert_eq!(
            sort("p-4! p-4 [display:block]! [display:block]"),
            "[display:block] [display:block]! p-4 p-4!"
        );
    }

    #[test]
    fn variants_classify_and_keep_importance() {
        // A recognized variant places the candidate; `!` still rides
        // through as the final tiebreak.
        assert!(matches!(classify("hover:flex!"), SortKey::Known { .. }));
        assert_eq!(sort("hover:flex! hover:flex"), "hover:flex hover:flex!");
        // A variant sorts after its variantless twin.
        assert_eq!(
            compare(&classify("flex"), &classify("hover:flex")),
            Ordering::Less
        );
    }

    #[test]
    fn static_variant_names_resolve_through_the_root_probe() {
        // Statics whose registered name spans several named segments
        // (`motion-safe`, and `any-pointer-coarse` at the probe-buffer
        // bound) reassemble through the same longest-prefix probe as
        // functional roots.
        assert!(matches!(
            classify("motion-safe:flex"),
            SortKey::Known { .. }
        ));
        assert!(matches!(
            classify("any-pointer-coarse:flex"),
            SortKey::Known { .. }
        ));
    }

    #[test]
    fn variants_order_against_each_other_by_breakpoint() {
        // Shared groups (via `classify_all`) are what make one variant
        // comparable to another. Ascending breakpoints: `sm` before `md`.
        let keys = classify_all("sm:flex md:flex");
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
        // `max-*` is descending, so the larger breakpoint sorts first.
        let keys = classify_all("max-lg:flex max-sm:flex");
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
    }

    #[test]
    fn group_and_peer_scopes_sort_bare_first_then_by_modifier_text() {
        let keys = classify_all("group-hover:flex group-hover/a:flex group-hover/b:flex");
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
        assert_eq!(compare(&keys[1], &keys[2]), Ordering::Less);
        // A bracketed modifier compares by its inner value (`.5` < `menu`).
        let keys = classify_all("group-hover/[.5]:flex group-hover/menu:flex");
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
    }

    #[test]
    fn a_modifier_outside_group_peer_and_container_variants_is_unknown() {
        assert_eq!(classify("hover/foo:flex"), SortKey::Unknown);
        assert_eq!(classify("has-hover/5:flex"), SortKey::Unknown);
        assert_eq!(classify("min-[600px]/5:flex"), SortKey::Unknown);
    }

    #[test]
    fn an_unresolvable_breakpoint_or_container_size_is_unknown() {
        assert_eq!(classify("min-abc:flex"), SortKey::Unknown);
        assert_eq!(classify("@max-abc:flex"), SortKey::Unknown);
        assert_eq!(classify("min-[var(--w)]:flex"), SortKey::Unknown);
    }

    #[test]
    fn variants_resolving_equal_lengths_share_a_rank() {
        // `min-[40rem]` and `sm` both resolve 40rem, so their variant
        // weights tie and the candidate text decides.
        let keys = classify_all("min-[40rem]:flex sm:flex");
        assert_eq!(weight_of(&keys[0]), weight_of(&keys[1]));
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
        // A container modifier does not participate in the weight.
        let keys = classify_all("@sm/main:flex @sm:flex");
        assert_eq!(weight_of(&keys[0]), weight_of(&keys[1]));
    }

    fn weight_of(key: &SortKey) -> &VariantWeight {
        let SortKey::Known { variant_weight, .. } = key else {
            panic!("expected a known key");
        };
        variant_weight
    }

    #[test]
    fn container_sizes_compare_by_unit_text_before_magnitude() {
        // 400px > 384px (= 24rem), but `px` < `rem` textually, so the
        // arbitrary pixel size groups before every named (rem) size.
        let keys = classify_all("@min-[400px]:flex @sm:flex");
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
    }

    #[test]
    fn arbitrary_selectors_compare_with_underscores_decoded() {
        // `_` decodes to a space, which sorts below `>`.
        let keys = classify_all("[&_p]:flex [&>p]:flex");
        assert_eq!(compare(&keys[0], &keys[1]), Ordering::Less);
    }

    #[test]
    fn unparseable_arbitrary_breakpoints_keep_the_comparator_total() {
        // Parseable and unparseable arbitrary breakpoint values sharing one
        // order bucket used to compare non-transitively (unparseable values
        // compared equal to everything), which can panic the sort. The
        // unparseable value now ranks last, so grouping the list completes.
        let keys = classify_all("min-[2rem]:flex min-[10rem]:flex min-[15xyz]:flex sm:flex flex");
        assert_eq!(keys.len(), 5);
    }

    #[test]
    fn functional_shaped_static_names_resolve_through_the_static_table() {
        // `w-full` parses as functional `w` + `full`; the whole name is
        // a static registration, and the join must land on the same
        // width signature as functional `w` candidates.
        let SortKey::Known { signature, .. } = classify("w-full") else {
            panic!("expected `w-full` to classify as known");
        };
        let SortKey::Known {
            signature: functional_signature,
            ..
        } = classify("w-10")
        else {
            panic!("expected `w-10` to classify as known");
        };
        assert_eq!(signature, functional_signature);
        // Static names whose parsed base is no functional utility at
        // all resolve too.
        assert!(matches!(classify("justify-center"), SortKey::Known { .. }));
        assert!(matches!(classify("inline-block"), SortKey::Known { .. }));
    }

    #[test]
    fn bare_functional_defaults_resolve_through_the_bare_placement() {
        // `border` and `ring` compile bare through a default value and
        // have no static registration.
        assert!(matches!(classify("border"), SortKey::Known { .. }));
        assert!(matches!(classify("ring"), SortKey::Known { .. }));
        // A valueless utility without a default stays unknown, and bare
        // negatives are never registered.
        assert_eq!(classify("w"), SortKey::Unknown);
        assert_eq!(classify("-border"), SortKey::Unknown);
    }

    #[test]
    fn joined_static_negatives_require_a_registered_negative_form() {
        // Tailwind registers `-m-px` but no `-w-full`.
        assert!(matches!(classify("-m-px"), SortKey::Known { .. }));
        assert_eq!(classify("-w-full"), SortKey::Unknown);
    }

    #[test]
    fn joined_static_lookup_skips_modified_candidates() {
        // Statics take no modifier; `w-full/50` is not a valid
        // candidate and resolves through no branch either.
        assert_eq!(classify("w-full/50"), SortKey::Unknown);
    }

    #[test]
    fn classification_captures_name_text() {
        let text_of = |input: &str| name_text(&classify(input));
        assert_eq!(text_of("p-4"), "p-4");
        // Fractions, modifiers, and arbitrary values all ride along in
        // the candidate text.
        assert_eq!(text_of("w-1/2"), "w-1/2");
        assert_eq!(text_of("bg-red-500/50"), "bg-red-500/50");
        assert_eq!(text_of("w-[13px]"), "w-[13px]");
        assert_eq!(text_of("[color:red]/50"), "[color:red]/50");
        assert_eq!(text_of("flex"), "flex");
        // The sign comes from the full candidate, outside the inner
        // candidate node.
        assert_eq!(text_of("-mt-2"), "-mt-2");
        // The important suffix and the variants ride along too.
        assert_eq!(text_of("p-4!"), "p-4!");
        assert_eq!(text_of("hover:p-4"), "hover:p-4");
    }

    // endregion: sort key classification
}
