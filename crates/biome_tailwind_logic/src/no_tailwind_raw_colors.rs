use std::io;

use biome_console::fmt::{Display, Formatter};
use biome_rowan::{AstNode, AstSeparatedList, Direction, TextRange};
use biome_rule_options::no_tailwind_raw_colors::NoTailwindRawColorsOptions;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwValue, TailwindSyntaxToken, TwCandidateList,
    TwFunctionalCandidate,
};
use color::{DynamicColor, Oklab, parse_color};
use smallvec::SmallVec;

use crate::use_tailwind_sorted_classes::{THEME_COLOR_VALUES, TailwindDesignSystem};

/// A class that sets a default palette color, such as `bg-pink-500`, or
/// `black` or `white` when the options forbid them.
pub struct RawColor {
    /// The class without its variants and `!`.
    candidate: TwFunctionalCandidate,
    /// The palette color, such as `pink-500` or `white`.
    color: TailwindSyntaxToken,
}

impl RawColor {
    /// The range of the class without its variants and `!`, relative to the
    /// start of the class string.
    pub fn range(&self) -> TextRange {
        self.candidate.syntax().text_trimmed_range()
    }

    /// Returns the classes that set a similar color of the stylesheet
    /// instead, closest first, such as `bg-brand/50` for `bg-pink-500/50`.
    /// Each one replaces the text at [Self::range].
    ///
    /// Only colors whose value is written out, such as `#00f` or
    /// `oklch(0.5 0.2 240)`, can be compared. Colors that refer to another
    /// value, as in `var(--primary)`, are left out, and so are the stylesheet's
    /// own palette colors, such as `--color-pink-400`, even when
    /// `allowedColors` lists them. `--color-black` and `--color-white` are left
    /// out too when the options forbid `black` and `white`. The result is empty
    /// when the stylesheet has no similar color.
    pub fn closest_design_classes<'a>(
        &'a self,
        design: &'a TailwindDesignSystem,
        options: &NoTailwindRawColorsOptions,
    ) -> SmallVec<[SuggestedClass<'a>; MAX_SUGGESTIONS]> {
        self.closest_design_colors(design, options)
            .into_iter()
            .map(|color| SuggestedClass {
                raw_color: self,
                color,
            })
            .collect()
    }

    fn closest_design_colors<'a>(
        &self,
        design: &'a TailwindDesignSystem,
        options: &NoTailwindRawColorsOptions,
    ) -> SmallVec<[&'a str; MAX_SUGGESTIONS]> {
        let color = self.color.text_trimmed();
        // The stylesheet can give a palette color another value.
        let target = match design.stylesheet_color(color) {
            Some(value) => parse_color(value).ok(),
            None => THEME_COLOR_VALUES.get(color).copied(),
        };
        let Some(target) = target else {
            return SmallVec::new();
        };
        let mut closest: SmallVec<[(f32, &str); MAX_SUGGESTIONS]> = SmallVec::new();
        for (name, value) in design.stylesheet_colors() {
            if is_raw_color(name, options) {
                continue;
            }
            let Ok(candidate) = parse_color(value) else {
                continue;
            };
            let difference = color_difference(target, candidate);
            // A value that overflows, such as `oklch(0.5 1e39 0)`, has a
            // difference that isn't a number.
            if difference.is_nan() || difference > MAX_DIFFERENCE {
                continue;
            }
            // Names break ties, so the result doesn't depend on the order of
            // the stylesheet's colors.
            let index = closest.partition_point(|&(other_difference, other_name)| {
                other_difference
                    .total_cmp(&difference)
                    .then_with(|| other_name.cmp(name))
                    .is_lt()
            });
            if index == MAX_SUGGESTIONS {
                continue;
            }
            closest.truncate(MAX_SUGGESTIONS - 1);
            closest.insert(index, (difference, name));
        }
        closest.into_iter().map(|(_, name)| name).collect()
    }
}

/// The class of a [RawColor] with a color of the stylesheet in place of the
/// palette color.
pub struct SuggestedClass<'a> {
    raw_color: &'a RawColor,
    color: &'a str,
}

impl Display for SuggestedClass<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        let tokens = self
            .raw_color
            .candidate
            .syntax()
            .descendants_tokens(Direction::Next);
        for token in tokens {
            fmt.write_str(if token == self.raw_color.color {
                self.color
            } else {
                token.text_trimmed()
            })?;
        }
        Ok(())
    }
}

/// Returns the color utilities that set a default palette color, or `black` or
/// `white` when the options forbid them, and that aren't explicitly allowed.
pub fn raw_colors(
    candidates: &TwCandidateList,
    options: &NoTailwindRawColorsOptions,
) -> SmallVec<[RawColor; 4]> {
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
            let color = value.value_token().ok()?;
            let name = color.text_trimmed();
            if !is_raw_color(name, options) || options.is_allowed_color(name) {
                return None;
            }
            Some(RawColor { candidate, color })
        })
        .collect()
}

/// The number of classes [RawColor::closest_design_classes] returns at most.
pub const MAX_SUGGESTIONS: usize = 3;

/// The largest [color_difference] between colors that still look similar: a
/// distance of 0.2. It covers a color a few shades lighter or darker, such as
/// `red-500` and `red-700`, and the same shade of a neighboring hue, such as
/// `red-500` and `orange-500`.
const MAX_DIFFERENCE: f32 = 0.2 * 0.2;

/// Returns the squared distance between two colors in the Oklab color space,
/// where equal distances look about equally different. Alpha counts as a
/// fourth axis, so a transparent color isn't close to its opaque form.
fn color_difference(left: DynamicColor, right: DynamicColor) -> f32 {
    let left = left.to_alpha_color::<Oklab>().components;
    let right = right.to_alpha_color::<Oklab>().components;
    left.iter()
        .zip(right)
        .map(|(left, right)| (left - right) * (left - right))
        .sum()
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

fn is_raw_color(value: &str, options: &NoTailwindRawColorsOptions) -> bool {
    is_palette_color(value)
        || (!options.allow_black_and_white() && matches!(value, "black" | "white"))
}

fn is_palette_color(value: &str) -> bool {
    let Some((color, shade)) = value.rsplit_once('-') else {
        return false;
    };
    PALETTE_SHADES.binary_search(&shade).is_ok() && PALETTE_COLORS.binary_search(&color).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{COLOR_UTILITIES, PALETTE_COLORS, PALETTE_SHADES, raw_colors};
    use crate::use_tailwind_sorted_classes::{THEME_COLOR_VALUES, TailwindDesignSystem};
    use biome_console::markup;
    use biome_rule_options::no_tailwind_raw_colors::NoTailwindRawColorsOptions;
    use biome_tailwind_parser::parse_tailwind;

    /// Returns the classes suggested for the single raw color in `class`,
    /// given the variables of `theme`.
    fn closest_design_classes(class: &str, theme: &[(&str, &str)]) -> Vec<String> {
        closest_design_classes_with_options(class, theme, &NoTailwindRawColorsOptions::default())
    }

    fn closest_design_classes_with_options(
        class: &str,
        theme: &[(&str, &str)],
        options: &NoTailwindRawColorsOptions,
    ) -> Vec<String> {
        let mut design = TailwindDesignSystem::default();
        for (name, value) in theme {
            design.add_theme_variable(name, value);
        }
        let parse = parse_tailwind(class);
        let raw_colors = raw_colors(&parse.tree().candidates(), options);
        assert_eq!(raw_colors.len(), 1, "{class}");
        raw_colors[0]
            .closest_design_classes(&design, options)
            .into_iter()
            .map(|class| {
                let markup = markup! { {class} }.to_owned();
                markup.0.iter().map(|node| node.content.as_str()).collect()
            })
            .collect()
    }

    #[test]
    fn every_palette_color_has_a_default_value() {
        for color in PALETTE_COLORS {
            for shade in PALETTE_SHADES {
                let name = format!("{color}-{shade}");
                assert!(THEME_COLOR_VALUES.contains_key(name.as_str()), "{name}");
            }
        }
    }

    #[test]
    fn suggests_nothing_without_stylesheet_colors() {
        assert!(closest_design_classes("bg-pink-500", &[]).is_empty());
    }

    #[test]
    fn suggests_classes_with_similar_colors_closest_first() {
        let theme = [
            ("--color-surface", "#fff"),
            ("--color-danger", "oklch(0.58 0.24 27)"),
            ("--color-brand", "#ec4899"),
            ("--color-ink", "black"),
        ];
        assert_eq!(
            closest_design_classes("hover:bg-pink-500/50!", &theme),
            ["bg-brand/50", "bg-danger/50"]
        );
        assert_eq!(
            closest_design_classes("border-t-red-600/[.5]", &theme),
            ["border-t-danger/[.5]", "border-t-brand/[.5]"]
        );
        assert_eq!(
            closest_design_classes("text-slate-50", &theme),
            ["text-surface"]
        );
        assert!(closest_design_classes("bg-blue-500", &theme).is_empty());
    }

    #[test]
    fn suggests_three_classes_at_most_ordered_by_name_on_ties() {
        let theme = [
            ("--color-e", "#ef4444"),
            ("--color-c", "#ef4444"),
            ("--color-a", "#ef4444"),
            ("--color-d", "#ef4444"),
            ("--color-b", "#ef4444"),
            ("--color-exact", "oklch(63.7% 0.237 25.331)"),
        ];
        assert_eq!(
            closest_design_classes("bg-red-500", &theme),
            ["bg-exact", "bg-a", "bg-b"]
        );
    }

    #[test]
    fn skips_colors_that_cannot_be_compared() {
        let theme = [
            ("--color-primary", "var(--primary)"),
            ("--color-mixed", "color-mix(in oklab, red, white)"),
            ("--color-red-600", "oklch(63.7% 0.237 25.331)"),
            ("--color-faded", "oklch(63.7% 0.237 25.331 / 10%)"),
            ("--spacing-red", "oklch(63.7% 0.237 25.331)"),
            ("--color-overflow", "oklch(0.5 1e39 0)"),
        ];
        assert!(closest_design_classes("bg-red-500", &theme).is_empty());
    }

    #[test]
    fn compares_against_the_stylesheet_value_of_a_palette_color() {
        let theme = [
            ("--color-pink-500", "#00f"),
            ("--color-brand", "#ec4899"),
            ("--color-link", "#00e"),
        ];
        assert_eq!(closest_design_classes("bg-pink-500", &theme), ["bg-link"]);
        let theme = [
            ("--color-pink-500", "var(--pink)"),
            ("--color-brand", "#ec4899"),
        ];
        assert!(closest_design_classes("bg-pink-500", &theme).is_empty());
        let theme = [
            ("--color-pink-500", "oklch(0.5 1e39 0)"),
            ("--color-brand", "#ec4899"),
        ];
        assert!(closest_design_classes("bg-pink-500", &theme).is_empty());
    }

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
                        raw_colors(
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
    fn detects_black_and_white_when_not_allowed() {
        let options = NoTailwindRawColorsOptions {
            allow_black_and_white: Some(false),
            ..Default::default()
        };
        for color in ["black", "white"] {
            for utility in COLOR_UTILITIES {
                for class in [
                    format!("{utility}-{color}"),
                    format!("hover:{utility}-{color}/50!"),
                ] {
                    let parse = parse_tailwind(&class);
                    assert!(!parse.has_errors(), "{class}");
                    assert_eq!(
                        raw_colors(&parse.tree().candidates(), &options).len(),
                        1,
                        "{class}"
                    );
                }
            }
        }
        for class in [
            "bg-transparent",
            "text-current",
            "border-inherit",
            "font-black",
            "bg-white-500",
        ] {
            let parse = parse_tailwind(class);
            assert!(!parse.has_errors(), "{class}");
            assert!(
                raw_colors(&parse.tree().candidates(), &options).is_empty(),
                "{class}"
            );
        }
    }

    #[test]
    fn allowed_colors_take_precedence_over_black_and_white() {
        let options = NoTailwindRawColorsOptions {
            allow_black_and_white: Some(false),
            allowed_colors: Some(["white".into()].into_iter().collect()),
        };
        let parse = parse_tailwind("bg-white text-black");
        let raw_colors = raw_colors(&parse.tree().candidates(), &options);
        let classes: Vec<_> = raw_colors
            .iter()
            .map(|raw_color| &"bg-white text-black"[raw_color.range()])
            .collect();
        assert_eq!(classes, ["text-black"]);
    }

    #[test]
    fn suggests_theme_colors_for_black_and_white() {
        let options = NoTailwindRawColorsOptions {
            allow_black_and_white: Some(false),
            ..Default::default()
        };
        let theme = [
            ("--color-white", "#fff"),
            ("--color-surface", "#fff"),
            ("--color-ink", "oklch(0.1 0 0)"),
        ];
        assert_eq!(
            closest_design_classes_with_options("bg-white/50", &theme, &options),
            ["bg-surface/50"]
        );
        assert_eq!(
            closest_design_classes_with_options("text-black", &theme, &options),
            ["text-ink"]
        );
        assert_eq!(
            closest_design_classes_with_options("text-slate-50", &theme, &options),
            ["text-surface"]
        );
        assert_eq!(
            closest_design_classes("text-slate-50", &theme),
            ["text-surface", "text-white"]
        );
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
                raw_colors(
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
                        raw_colors(
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
                raw_colors(
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
        let raw_colors = raw_colors(
            &parse.tree().candidates(),
            &NoTailwindRawColorsOptions::default(),
        );
        let classes: Vec<_> = raw_colors
            .iter()
            .map(|raw_color| &source[raw_color.range()])
            .collect();
        assert_eq!(classes, ["bg-pink-500/50", "border-gray-50"]);
    }
}
