use std::cmp::Ordering;

use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeText, TextRange, TextSize};
use biome_string_case::Collator;
use biome_tailwind_syntax::{AnyTwFullCandidate, TailwindSyntaxNode, TailwindSyntaxToken, TwRoot};

use super::sort_v4_variants::{VariantGroups, VariantWeight};
use crate::tailwind_v4::{CandidatePlacement, Signature, TailwindDesignSystem, VariantKey};

/// Sort the candidates of a parsed Tailwind class list and return the joined,
/// space-separated result. `design` adds the user's theme values, utilities,
/// and variants to the default configuration.
pub fn sort_class_list(root: &TwRoot, design: &TailwindDesignSystem) -> String {
    let candidates = root.candidates();

    // A variant's weight depends on the whole list, so classify first and
    // weight in a second pass.
    let mut pending: Vec<(PendingSortKey, SyntaxNodeText)> = Vec::with_capacity(candidates.len());
    for candidate in candidates.iter().flatten() {
        let text = candidate.syntax().text_trimmed();
        let key = PendingSortKey::classify(&candidate, design);
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
#[derive(Clone, Debug, Eq, PartialEq)]
enum PendingSortKey {
    Unknown,
    Known {
        signature: Signature,
        count: u8,
        text: CandidateText,
        variants: Vec<VariantKey>,
    },
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

impl PendingSortKey {
    /// Classify a candidate by where Tailwind places its rule, or `Unknown`
    /// when Tailwind generates nothing for it.
    fn classify(candidate: &AnyTwFullCandidate, design: &TailwindDesignSystem) -> Self {
        match CandidatePlacement::of(candidate, design) {
            Some(CandidatePlacement {
                signature,
                count,
                variants,
                ..
            }) => Self::Known {
                signature,
                count,
                text: CandidateText(candidate.syntax().clone()),
                variants,
            },
            None => Self::Unknown,
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

#[cfg(test)]
mod tests {
    use super::*;
    use biome_module_graph::{
        TailwindFunctionalUtility, TailwindStylesheet, TailwindThemeEntry,
        TailwindUtilityDeclaration, TailwindValueArgument, TailwindValueFunction,
    };
    use biome_rowan::NodeCache;
    use biome_tailwind_parser::{parse_tailwind, parse_tailwind_with_options};

    /// A known key with the given placement whose text is the placeholder
    /// candidate `x`, for tests that only exercise the placement.
    fn known(property_idx: u16, property_count: u8) -> SortKey {
        let parsed = parse_tailwind("x");
        let candidate = parsed.tree().candidates().iter().next().unwrap().unwrap();
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
        let parsed = parse_tailwind(input);
        let full = parsed.tree().candidates().iter().next().unwrap().unwrap();
        let pending = PendingSortKey::classify(&full, &TailwindDesignSystem::default());
        // Groups from this one candidate; a plain utility gets empty
        // `variant_weight`.
        let variants: &[VariantKey] = match &pending {
            PendingSortKey::Known { variants, .. } => variants,
            PendingSortKey::Unknown => &[],
        };
        let groups = VariantGroups::new(variants);
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
            .flatten()
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
        sort_class_list(
            &parse_tailwind(input).tree(),
            &TailwindDesignSystem::default(),
        )
    }

    #[test]
    fn css_variable_value_sorts_like_arbitrary_var() {
        assert_eq!(
            sort("w-(--sidebar-width) flex h-full flex-col"),
            "flex h-full w-(--sidebar-width) flex-col"
        );
        assert_eq!(
            sort("max-h-(--console-max-h) flex shrink-0 flex-col"),
            "flex max-h-(--console-max-h) shrink-0 flex-col"
        );
        assert_eq!(
            sort("min-w-(--anchor-width) w-max max-w-sm p-0"),
            "w-max max-w-sm min-w-(--anchor-width) p-0"
        );
        assert_eq!(
            sort("w-(--x) w-[var(--x)] flex"),
            sort("w-[var(--x)] w-(--x) flex")
        );
        assert_eq!(sort("bg-(--brand)/50 flex"), "flex bg-(--brand)/50");
    }

    #[test]
    fn dashed_bare_variant_value_is_known() {
        assert_eq!(
            sort("supports-backdrop-filter:backdrop-blur-xs fixed inset-0"),
            "fixed inset-0 supports-backdrop-filter:backdrop-blur-xs"
        );
        assert_eq!(
            sort("supports-backdrop-filter:flex supports-grid:flex"),
            "supports-backdrop-filter:flex supports-grid:flex"
        );
    }

    #[test]
    fn stylesheet_grid_template_values_make_class_known() {
        let mut design = TailwindDesignSystem::default();
        design.add_theme_variable("--grid-template-columns-master-aside", "1fr 20rem");
        design.add_theme_variable("--grid-template-rows-layout", "auto 1fr");
        assert_eq!(
            sort("lg:grid-cols-master-aside grid gap-6"),
            "lg:grid-cols-master-aside grid gap-6"
        );
        assert_eq!(
            sort_with("lg:grid-cols-master-aside grid gap-6", &design),
            "grid gap-6 lg:grid-cols-master-aside"
        );
        assert_eq!(
            sort_with("grid-rows-layout flex", &design),
            "flex grid-rows-layout"
        );
    }

    fn sort_with(input: &str, design: &TailwindDesignSystem) -> String {
        sort_class_list(&parse_tailwind(input).tree(), design)
    }

    #[test]
    fn stylesheet_theme_value_makes_class_known() {
        let mut design = TailwindDesignSystem::default();
        design.add_theme_variable("--color-brand", "#00f");
        // Unknown classes come first, so `bg-brand` only moves once the theme
        // defines it.
        assert_eq!(sort("bg-brand flex"), "bg-brand flex");
        assert_eq!(sort_with("bg-brand flex", &design), "flex bg-brand");
    }

    #[test]
    fn stylesheet_theme_reset_removes_defaults() {
        let mut design = TailwindDesignSystem::default();
        design.reset_theme("--color");
        assert_eq!(sort_with("flex bg-red-500", &design), "bg-red-500 flex");
    }

    #[test]
    fn stylesheet_utility_sorts_by_its_properties() {
        let mut design = TailwindDesignSystem::default();
        design.add_utility("btn-primary", ["color", "background-color"], 2, None);
        assert_eq!(sort("btn-primary flex"), "btn-primary flex");
        assert_eq!(sort_with("btn-primary flex", &design), "flex btn-primary");
        assert_eq!(
            sort_with("text-red-500 btn-primary bg-blue-500", &design),
            "btn-primary bg-blue-500 text-red-500"
        );
    }

    #[test]
    fn stylesheet_functional_roots_keep_built_in_splits() {
        let functional_utilities = ["bg-red", "bg-white"]
            .map(|name| TailwindFunctionalUtility {
                name: name.into(),
                declarations: Box::default(),
                has_only_declarations: true,
            })
            .into();
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            functional_utilities,
            ..TailwindStylesheet::default()
        });
        let sort_with_roots = |input: &str| {
            let parse = parse_tailwind_with_options(
                input,
                &mut NodeCache::default(),
                design.parser_options(),
            );
            sort_class_list(&parse.tree(), &design)
        };
        assert_eq!(sort_with_roots("bg-red-500 flex"), "flex bg-red-500");
        assert_eq!(sort_with_roots("bg-white flex"), "flex bg-white");
    }

    /// A declaration of `property`, with a `--value(…)` function, or a
    /// `--modifier(…)` one when `is_modifier`, taking `arguments`.
    fn declaration(
        property: &str,
        function: Option<(bool, &[TailwindValueArgument])>,
    ) -> TailwindUtilityDeclaration {
        TailwindUtilityDeclaration {
            property: property.to_string().into(),
            functions: function
                .map(|(is_modifier, arguments)| TailwindValueFunction {
                    is_modifier,
                    arguments: arguments.into(),
                })
                .into_iter()
                .collect(),
        }
    }

    fn value(property: &str, arguments: &[TailwindValueArgument]) -> TailwindUtilityDeclaration {
        declaration(property, Some((false, arguments)))
    }

    fn functional(
        name: &str,
        declarations: impl Into<Box<[TailwindUtilityDeclaration]>>,
    ) -> TailwindFunctionalUtility {
        TailwindFunctionalUtility {
            name: name.to_string().into(),
            declarations: declarations.into(),
            has_only_declarations: true,
        }
    }

    /// Sorts `input` with a design system that declares `utilities` and the
    /// theme variables `theme`, parsing it the way the analyzer does.
    fn sort_with_functional(
        input: &str,
        utilities: impl Into<Vec<TailwindFunctionalUtility>>,
        theme: &[(&str, &str)],
    ) -> String {
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            theme: theme
                .iter()
                .map(|(name, value)| TailwindThemeEntry::Variable {
                    name: name.to_string().into(),
                    value: value.to_string().into(),
                })
                .collect(),
            functional_utilities: utilities.into(),
            ..TailwindStylesheet::default()
        });
        let parse =
            parse_tailwind_with_options(input, &mut NodeCache::default(), design.parser_options());
        sort_class_list(&parse.tree(), &design)
    }

    // The expected orders below come from Tailwind CSS 4.3.3's `getClassOrder`.

    #[test]
    fn functional_utility_takes_values_its_arguments_accept() {
        use TailwindValueArgument::*;
        let tab = functional(
            "tab",
            [value(
                "tab-size",
                &[
                    Type("integer".into()),
                    Theme("--tab-size".into()),
                    Literal("inherit".into()),
                    ArbitraryType("integer".into()),
                ],
            )],
        );
        assert_eq!(
            sort_with_functional(
                "tab-4 tab-github tab-inherit tab-[3] tab-foo flex p-2",
                [tab],
                &[("--tab-size-github", "8")],
            ),
            "tab-foo flex p-2 tab-4 tab-[3] tab-github tab-inherit"
        );
    }

    #[test]
    fn functional_utility_takes_bare_numbers_tailwind_accepts() {
        let num = functional(
            "num",
            [value(
                "margin-top",
                &[TailwindValueArgument::Type("number".into())],
            )],
        );
        assert_eq!(
            sort_with_functional("num-1.5 num-1.3 num-2 num-02 flex mt-2", [num], &[]),
            "num-1.3 num-02 mt-2 num-1.5 num-2 flex"
        );
    }

    #[test]
    fn functional_utility_ratio_drops_other_values() {
        let ratio = functional(
            "ratio",
            [
                value(
                    "aspect-ratio",
                    &[TailwindValueArgument::Type("ratio".into())],
                ),
                value("width", &[TailwindValueArgument::Type("integer".into())]),
            ],
        );
        assert_eq!(
            sort_with_functional(
                "ratio-16/9 ratio-4 ratio-[16/9] flex aspect-video",
                [ratio],
                &[]
            ),
            "ratio-[16/9] flex aspect-video ratio-16/9 ratio-4"
        );
    }

    #[test]
    fn functional_utility_ratio_parts_are_read_as_numbers() {
        let ratio = functional(
            "rt",
            [value(
                "aspect-ratio",
                &[TailwindValueArgument::Type("ratio".into())],
            )],
        );
        assert_eq!(
            sort_with_functional("rt-07/9 rt-1.0/2 rt-1.5/2 flex", [ratio], &[]),
            "rt-1.5/2 flex rt-1.0/2 rt-07/9"
        );
    }

    #[test]
    fn functional_utility_ratio_rejects_an_unresolved_modifier() {
        use TailwindValueArgument::*;
        let utility = functional(
            "ar",
            [
                value("aspect-ratio", &[Type("ratio".into())]),
                declaration("opacity", Some((true, &[ArbitraryType("*".into())]))),
            ],
        );
        assert_eq!(
            sort_with_functional("ar-16/9 flex", [utility], &[]),
            "ar-16/9 flex"
        );
    }

    #[test]
    fn functional_utility_takes_integer_percentages() {
        let utility = functional(
            "pc",
            [value(
                "opacity",
                &[TailwindValueArgument::Type("percentage".into())],
            )],
        );
        assert_eq!(
            sort_with_functional("pc-50% pc-1.5% pc-05% flex", [utility], &[]),
            "pc-1.5% pc-05% flex pc-50%"
        );
    }

    #[test]
    fn functional_utility_infers_font_family_types() {
        use TailwindValueArgument::*;
        let family = functional(
            "ff",
            [value("font-family", &[ArbitraryType("family-name".into())])],
        );
        let generic = functional(
            "gf",
            [value(
                "font-family",
                &[ArbitraryType("generic-name".into())],
            )],
        );
        assert_eq!(
            sort_with_functional(
                "ff-[Inter] ff-['Open_Sans'] ff-[serif] ff-[10px] flex",
                [family],
                &[]
            ),
            "ff-[10px] flex ff-['Open_Sans'] ff-[Inter] ff-[serif]"
        );
        assert_eq!(
            sort_with_functional("gf-[serif] gf-[sans-serif] gf-[Inter] flex", [generic], &[]),
            "gf-[Inter] flex gf-[sans-serif] gf-[serif]"
        );
    }

    #[test]
    fn functional_utility_modifier_must_resolve() {
        use TailwindValueArgument::*;
        let utility = functional(
            "mod",
            [
                value(
                    "color",
                    &[ArbitraryType("color".into()), Theme("--color".into())],
                ),
                declaration("opacity", Some((true, &[Type("integer".into())]))),
            ],
        );
        assert_eq!(
            sort_with_functional(
                "mod-red-500/50 mod-red-500 mod-[#fff]/20 mod-red-500/foo flex text-red-500",
                [utility],
                &[]
            ),
            "mod-red-500/foo flex mod-[#fff]/20 mod-red-500/50 mod-red-500 text-red-500"
        );
    }

    #[test]
    fn functional_utility_definitions_sort_by_the_first() {
        use TailwindValueArgument::*;
        let numeric = functional("two", [value("padding", &[Type("integer".into())])]);
        let keyword = functional(
            "two",
            [
                value("display", &[Literal("grid".into()), Literal("flex".into())]),
                declaration("gap", None),
            ],
        );
        assert_eq!(
            sort_with_functional(
                "two-4 two-grid two-block flex p-4 grid",
                [numeric, keyword],
                &[]
            ),
            "two-block two-grid flex grid p-4 two-4"
        );
    }

    #[test]
    fn functional_utility_needs_a_value_function() {
        let utility = functional("novalue", [declaration("color", None)]);
        assert_eq!(
            sort_with_functional("novalue-4 novalue-red flex", [utility], &[]),
            "novalue-4 novalue-red flex"
        );
    }

    #[test]
    fn negative_functional_utility_needs_the_sign() {
        let utility = functional(
            "-neg",
            [value(
                "translate",
                &[TailwindValueArgument::Type("integer".into())],
            )],
        );
        assert_eq!(
            sort_with_functional("-neg-4 neg-4 flex translate-x-2", [utility], &[]),
            "neg-4 flex translate-x-2 -neg-4"
        );
    }

    #[test]
    fn negative_functional_root_keeps_built_in_splits() {
        let utility = functional(
            "-bg-red",
            [value(
                "outline-width",
                &[TailwindValueArgument::Type("integer".into())],
            )],
        );
        assert_eq!(
            sort_with_functional(
                "bg-red-500 -bg-red-4 bg-red-4 flex outline-2",
                [utility],
                &[]
            ),
            "bg-red-4 flex bg-red-500 outline-2 -bg-red-4"
        );
    }

    #[test]
    fn functional_utility_and_built_in_root_both_apply() {
        let utility = functional(
            "bg-red",
            [value(
                "outline-width",
                &[TailwindValueArgument::Type("integer".into())],
            )],
        );
        assert_eq!(
            sort_with_functional(
                "bg-red-500 bg-red-4 bg-red bg-white flex outline-2",
                [utility],
                &[]
            ),
            "bg-red flex bg-red-500 bg-white outline-2 bg-red-4"
        );
    }

    #[test]
    fn stylesheet_variant_sorts_after_built_in_variants() {
        let mut design = TailwindDesignSystem::default();
        design.add_variant("theme-midnight");
        assert_eq!(
            sort_with("theme-midnight:flex hover:flex flex", &design),
            "flex hover:flex theme-midnight:flex"
        );
    }

    #[test]
    fn stylesheet_breakpoints_sort_by_length() {
        let mut design = TailwindDesignSystem::default();
        design.add_theme_variable("--breakpoint-3xl", "120rem");
        design.add_theme_variable("--breakpoint-sm", "100rem");
        assert_eq!(
            sort_with("3xl:flex sm:flex 2xl:flex md:flex", &design),
            "md:flex 2xl:flex sm:flex 3xl:flex"
        );
        assert_eq!(
            sort_with("max-md:flex max-3xl:flex", &design),
            "max-3xl:flex max-md:flex"
        );
    }

    #[test]
    fn stylesheet_breakpoint_reset_removes_defaults() {
        let mut design = TailwindDesignSystem::default();
        design.reset_theme("--breakpoint");
        assert_eq!(sort_with("flex sm:flex", &design), "sm:flex flex");
    }

    #[test]
    fn stylesheet_container_sizes_sort_by_length() {
        let mut design = TailwindDesignSystem::default();
        design.add_theme_variable("--container-8xl", "100rem");
        assert_eq!(
            sort_with("@8xl:flex @sm:flex", &design),
            "@sm:flex @8xl:flex"
        );
    }

    fn classify_all(input: &str) -> Vec<SortKey> {
        let parsed = parse_tailwind(input);
        let pending: Vec<PendingSortKey> = parsed
            .tree()
            .candidates()
            .iter()
            .flatten()
            .map(|candidate| PendingSortKey::classify(&candidate, &TailwindDesignSystem::default()))
            .collect();
        let groups = VariantGroups::new(
            pending
                .iter()
                .filter_map(|key| match key {
                    PendingSortKey::Known { variants, .. } => Some(variants.as_slice()),
                    PendingSortKey::Unknown => None,
                })
                .flatten(),
        );
        pending
            .into_iter()
            .map(|key| key.into_sort_key(&groups))
            .collect()
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

    // region: sort key classification

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
