use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_tailwind_raw_colors::NoTailwindRawColorsOptions;
use biome_tailwind_syntax::{AnyTwCandidate, AnyTwFullCandidate, AnyTwValue, TwCandidateList};
use smallvec::SmallVec;

/// Returns parse-relative ranges of default palette utilities not explicitly allowed.
pub fn raw_color_ranges(
    candidates: &TwCandidateList,
    options: &NoTailwindRawColorsOptions,
) -> SmallVec<[TextRange; 4]> {
    // capacity 4 is a rough estimate based on my personal experience

    candidates
        .iter()
        .flatten()
        .filter_map(|candidate| {
            let AnyTwFullCandidate::TwFullCandidate(full) = candidate else {
                return None;
            };
            if full.negative_token().is_some() {
                return None;
            }
            let AnyTwCandidate::TwFunctionalCandidate(candidate) = full.candidate().ok()? else {
                return None;
            };
            let base = candidate.base_token().ok()?;
            if !is_color_utility(base.text_trimmed()) {
                return None;
            }
            let AnyTwValue::TwNamedValue(value) = candidate.value().ok()? else {
                return None;
            };
            let value = value.value_token().ok()?;
            let color = value.text_trimmed();
            (is_palette_color(color) && !options.is_allowed_color(color))
                .then(|| candidate.syntax().text_trimmed_range())
        })
        .collect()
}

const COLOR_UTILITIES: &[&str] = &[
    "accent",
    "bg",
    "border",
    "border-b",
    "border-e",
    "border-l",
    "border-r",
    "border-s",
    "border-t",
    "border-x",
    "border-y",
    "caret",
    "decoration",
    "divide",
    "drop-shadow",
    "fill",
    "from",
    "inset-ring",
    "inset-shadow",
    "mask-b-from",
    "mask-b-to",
    "mask-conic-from",
    "mask-conic-to",
    "mask-l-from",
    "mask-l-to",
    "mask-linear-from",
    "mask-linear-to",
    "mask-r-from",
    "mask-r-to",
    "mask-radial-from",
    "mask-radial-to",
    "mask-t-from",
    "mask-t-to",
    "mask-x-from",
    "mask-x-to",
    "mask-y-from",
    "mask-y-to",
    "outline",
    "placeholder",
    "ring",
    "ring-offset",
    "shadow",
    "stroke",
    "text",
    "text-shadow",
    "to",
    "via",
];

const PALETTE_COLORS: &[&str] = &[
    "amber", "blue", "cyan", "emerald", "fuchsia", "gray", "green", "indigo", "lime", "mauve",
    "mist", "neutral", "olive", "orange", "pink", "purple", "red", "rose", "sky", "slate", "stone",
    "taupe", "teal", "violet", "yellow", "zinc",
];

const PALETTE_SHADES: &[&str] = &[
    "100", "200", "300", "400", "50", "500", "600", "700", "800", "900", "950",
];

fn is_color_utility(base: &str) -> bool {
    COLOR_UTILITIES.binary_search(&base).is_ok()
}

fn is_palette_color(value: &str) -> bool {
    let Some((color, shade)) = value.rsplit_once('-') else {
        return false;
    };
    PALETTE_SHADES.binary_search(&shade).is_ok() && PALETTE_COLORS.binary_search(&color).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{COLOR_UTILITIES, PALETTE_COLORS, PALETTE_SHADES, raw_color_ranges};
    use biome_rule_options::no_tailwind_raw_colors::NoTailwindRawColorsOptions;
    use biome_tailwind_parser::parse_tailwind;

    #[test]
    fn allows_builtin_colors_without_exceptions() {
        for color in ["black", "white", "transparent", "current", "inherit"] {
            for utility in COLOR_UTILITIES {
                for class in [
                    format!("{utility}-{color}"),
                    format!("hover:{utility}-{color}/50!"),
                ] {
                    let parse = parse_tailwind(&class);
                    assert!(!parse.has_errors(), "{class}");
                    assert!(
                        raw_color_ranges(
                            &parse.tree().candidates(),
                            &NoTailwindRawColorsOptions::default()
                        )
                        .is_empty(),
                        "{class}"
                    );
                }
            }
        }
    }

    #[test]
    fn lookup_tables_are_sorted() {
        for table in [COLOR_UTILITIES, PALETTE_COLORS, PALETTE_SHADES] {
            assert!(table.is_sorted_by(|left, right| left.cmp(right).is_lt()));
        }
    }

    #[test]
    fn detects_palette_across_utilities_and_variants() {
        for class in [
            "bg-pink-500",
            "hover:text-red-50/50",
            "dark:!border-t-slate-950",
            "[&:nth-child(2)]:ring-offset-gray-100",
            "from-blue-100!",
            "via-cyan-200",
            "to-teal-300",
            "divide-zinc-400",
            "placeholder-gray-500",
            "fill-zinc-950",
            "stroke-emerald-600",
            "decoration-orange-700",
            "outline-amber-800",
            "shadow-yellow-900",
            "inset-shadow-lime-500",
            "inset-ring-green-500",
            "text-shadow-violet-500",
            "drop-shadow-purple-500",
            "accent-fuchsia-500",
            "caret-rose-500",
            "bg-neutral-500",
            "bg-stone-500",
            "bg-sky-500",
            "bg-indigo-500",
            "bg-taupe-500",
            "bg-mauve-500",
            "bg-mist-500",
            "bg-olive-500",
        ] {
            let parse = parse_tailwind(class);
            assert!(!parse.has_errors(), "{class}");
            assert_eq!(
                raw_color_ranges(
                    &parse.tree().candidates(),
                    &NoTailwindRawColorsOptions::default()
                )
                .len(),
                1,
                "{class}"
            );
        }
    }

    #[test]
    fn detects_mask_palette_colors() {
        for direction in ["linear", "radial", "conic", "x", "y", "t", "r", "b", "l"] {
            for stop in ["from", "to"] {
                for color in ["red-500", "slate-950", "gray-50"] {
                    let class = format!("mask-{direction}-{stop}-{color}");
                    let parse = parse_tailwind(&class);
                    assert!(!parse.has_errors(), "{class}");
                    assert_eq!(
                        raw_color_ranges(
                            &parse.tree().candidates(),
                            &NoTailwindRawColorsOptions::default()
                        )
                        .len(),
                        1,
                        "{class}"
                    );
                }
            }
        }
    }

    #[test]
    fn allows_semantic_colors_and_unrelated_utilities() {
        for class in [
            "bg-primary",
            "text-muted-foreground",
            "border-brand-500",
            "bg-transparent",
            "text-current",
            "border-inherit",
            "fill-none",
            "font-black",
            "w-red-500",
            "bg-red-550",
            "bg-red-500-extra",
            "custom-red-500",
            "bg-[#ff00aa]",
            "bg-(--brand)",
            "[color:red]",
            "[&.bg-red-500]:p-4",
            "-bg-red-500",
            "mask-linear-from-primary",
            "mask-radial-to-transparent",
            "mask-conic-from-50",
            "mask-t-to-50%",
            "mask-x-from-[#ff00aa]",
        ] {
            let parse = parse_tailwind(class);
            assert!(!parse.has_errors(), "{class}");
            assert!(
                raw_color_ranges(
                    &parse.tree().candidates(),
                    &NoTailwindRawColorsOptions::default()
                )
                .is_empty(),
                "{class}"
            );
        }
    }

    #[test]
    fn ranges_exclude_variants_and_preserve_byte_offsets() {
        let source = "p-4 hover:bg-pink-500/50 text-primary\n!border-gray-50";
        let parse = parse_tailwind(source);
        let ranges = raw_color_ranges(
            &parse.tree().candidates(),
            &NoTailwindRawColorsOptions::default(),
        );
        let classes: Vec<_> = ranges.into_iter().map(|range| &source[range]).collect();
        assert_eq!(classes, ["bg-pink-500/50", "border-gray-50"]);
    }
}
