use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_tailwind_syntax::{AnyTwFullCandidate, TwCandidateList, TwFullCandidate, is_node_equal};

use crate::tailwind_v4::{CandidateProperties, TailwindDesignSystem};

/// A class that sets the same CSS properties as an earlier class of its list.
#[derive(Clone, Debug)]
pub struct ConflictingClass {
    pub class: TwFullCandidate,
    /// The first class of the list that sets the same properties as `class`
    /// under the same variants.
    pub earlier: TwFullCandidate,
}

/// Returns the classes of `candidates` that set the same CSS properties as an
/// earlier class with the same variants, in source order. `hover:flex` and
/// `hover:grid` conflict, while `flex` and `hover:grid` don't.
///
/// Classes are compared with the first class of the list that sets their
/// properties. A repeated class isn't reported, so `flex flex grid` only
/// reports `grid`.
///
/// Variants must be written the same way to match: `sm:hover:flex` isn't
/// compared with `hover:sm:grid`. Classes whose properties can't be told, as
/// described at [CandidateProperties::of], are skipped.
///
/// `is_partial` tells whether the class at a range continues outside
/// `candidates`, as `border` does in `` `border${suffix}` ``. Those classes are
/// skipped too.
pub fn find_conflicting_classes(
    candidates: &TwCandidateList,
    design: &TailwindDesignSystem,
    is_partial: impl Fn(TextRange) -> bool,
) -> Box<[ConflictingClass]> {
    // The first class found for each set of properties and variants.
    let mut first_classes: Vec<(CandidateProperties, TwFullCandidate)> = Vec::new();
    let mut conflicts = Vec::new();
    for candidate in candidates.iter().flatten() {
        if is_partial(candidate.range()) {
            continue;
        }
        let Some(properties) = CandidateProperties::of(&candidate, design) else {
            continue;
        };
        let AnyTwFullCandidate::TwFullCandidate(class) = candidate else {
            continue;
        };
        let variants = class.variants();
        let earlier = first_classes.iter().find_map(|(other, earlier)| {
            (*other == properties && is_node_equal(earlier.variants().syntax(), variants.syntax()))
                .then_some(earlier)
        });
        match earlier {
            Some(earlier) => {
                if earlier.syntax().text_trimmed() != class.syntax().text_trimmed() {
                    conflicts.push(ConflictingClass {
                        class,
                        earlier: earlier.clone(),
                    });
                }
            }
            None => first_classes.push((properties, class)),
        }
    }
    conflicts.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use biome_module_graph::{
        TailwindFunctionalUtility, TailwindStylesheet, TailwindThemeEntry, TailwindUtility,
        TailwindUtilityDeclaration, TailwindValueArgument, TailwindValueFunction,
    };
    use biome_rowan::{AstNode, AstSeparatedList, NodeCache};
    use biome_tailwind_parser::parse_tailwind_with_options;

    use super::find_conflicting_classes;
    use crate::tailwind_v4::TailwindDesignSystem;

    /// Returns each conflict of `input` as `class > earlier`.
    fn conflicts_with(input: &str, design: &TailwindDesignSystem) -> Vec<String> {
        let parsed =
            parse_tailwind_with_options(input, &mut NodeCache::default(), design.parser_options());
        assert!(!parsed.has_errors(), "failed to parse: {input}");
        find_conflicting_classes(&parsed.tree().candidates(), design, |_| false)
            .iter()
            .map(|conflict| {
                format!(
                    "{} > {}",
                    conflict.class.syntax().text_trimmed(),
                    conflict.earlier.syntax().text_trimmed()
                )
            })
            .collect()
    }

    fn conflicts(input: &str) -> Vec<String> {
        conflicts_with(input, TailwindDesignSystem::default_ref())
    }

    #[test]
    fn reports_classes_that_set_the_same_properties() {
        for (input, expected) in [
            ("flex grid", &["grid > flex"][..]),
            ("flex w-full grid", &["grid > flex"]),
            ("p-2 p-4", &["p-4 > p-2"]),
            ("p-2 p-[3px]", &["p-[3px] > p-2"]),
            ("p-2 p-(--gap)", &["p-(--gap) > p-2"]),
            ("m-2 -m-2", &["-m-2 > m-2"]),
            ("w-1/2 w-full", &["w-full > w-1/2"]),
            (
                "text-red-500 text-blue-500/50",
                &["text-blue-500/50 > text-red-500"],
            ),
            ("text-sm text-lg/8", &["text-lg/8 > text-sm"]),
            ("text-sm text-lg", &["text-lg > text-sm"]),
            ("text-[14px] text-[1rem]", &["text-[1rem] > text-[14px]"]),
            ("text-sm text-[14px]/6", &["text-[14px]/6 > text-sm"]),
            ("shadow-md shadow-lg", &["shadow-lg > shadow-md"]),
            ("bg-red-500 bg-[#fff]", &["bg-[#fff] > bg-red-500"]),
            (
                "border-2 border-[calc(1px+2px)]",
                &["border-[calc(1px+2px)] > border-2"],
            ),
            (
                "text-red-500 text-[hsl(210_50%_50%)]",
                &["text-[hsl(210_50%_50%)] > text-red-500"],
            ),
            ("bg-red-500 bg-(--brand)", &["bg-(--brand) > bg-red-500"]),
            (
                "font-bold font-(--weight)",
                &["font-(--weight) > font-bold"],
            ),
            ("font-sans font-[Arial]", &["font-[Arial] > font-sans"]),
            (
                "shadow-md/50 shadow-lg/20",
                &["shadow-lg/20 > shadow-md/50"],
            ),
            (
                "[font-size:14px] [font-size:1rem]",
                &["[font-size:1rem] > [font-size:14px]"],
            ),
            ("flex [display:grid]", &["[display:grid] > flex"]),
            ("[display:grid] flex", &["flex > [display:grid]"]),
            (
                "hover:flex hover:[display:grid]",
                &["hover:[display:grid] > hover:flex"],
            ),
            (
                "text-[14px] [font-size:1rem]",
                &["[font-size:1rem] > text-[14px]"],
            ),
            (
                "duration-initial [--tw-duration:1s]",
                &["[--tw-duration:1s] > duration-initial"],
            ),
            ("flex flex!", &["flex! > flex"]),
            ("flex !grid", &["!grid > flex"]),
            ("hover:flex hover:grid", &["hover:grid > hover:flex"]),
            (
                "sm:hover:flex sm:hover:grid",
                &["sm:hover:grid > sm:hover:flex"],
            ),
            ("block inline flex", &["inline > block", "flex > block"]),
            ("flex p-2 grid p-4", &["grid > flex", "p-4 > p-2"]),
            ("flex flex grid", &["grid > flex"]),
            ("grid flex flex", &["flex > grid", "flex > grid"]),
            (
                "[--gap:1rem] [--gap:2rem]",
                &["[--gap:2rem] > [--gap:1rem]"],
            ),
            (
                "bg-linear-to-r bg-linear-to-b",
                &["bg-linear-to-b > bg-linear-to-r"],
            ),
            ("space-x-2 space-x-4", &["space-x-4 > space-x-2"]),
            ("select-none select-text", &["select-text > select-none"]),
            ("block-4 block-8", &["block-8 > block-4"]),
            (
                "outline-dashed outline-dotted",
                &["outline-dotted > outline-dashed"],
            ),
            (
                "lining-nums oldstyle-nums",
                &["oldstyle-nums > lining-nums"],
            ),
            (
                "text-shadow-sm text-shadow-lg",
                &["text-shadow-lg > text-shadow-sm"],
            ),
        ] {
            assert_eq!(conflicts(input), expected, "{input}");
        }
    }

    #[test]
    fn accepts_classes_that_set_different_properties() {
        for input in [
            "flex",
            "flex w-full",
            "flex flex",
            "p-2 px-4",
            "px-2 py-2",
            "mt-2 mb-2",
            "w-4 h-4 size-4",
            "text-lg text-red-500",
            "text-[1rem] text-[red]",
            "border border-red-500 border-dashed",
            "shadow-md ring-2",
            "flex hover:grid",
            "sm:flex md:grid",
            "hover:flex focus:grid",
            "sm:hover:flex hover:sm:grid",
            "group-hover:flex group-hover/item:grid",
            "flex-1 flex-col",
            "[color:red] [background:red]",
            "[--gap:1rem] [--size:1rem]",
            "transform-3d transform-content",
            "contain-size contain-layout",
            "lining-nums tabular-nums",
            "select-none outline-dashed",
            "block-4 inline-4",
            "perspective-near perspective-origin-top",
            "text-shadow-sm text-shadow-red-500",
            "scrollbar-thumb-red-500 scrollbar-track-red-500",
            "border-spacing-x-2 border-spacing-y-2",
            "duration-initial ease-initial",
            "divide-y-reverse [--tw-divide-y-reverse:1]",
            "flex hover:[display:grid]",
            "text-sm [font-size:14px]",
            "font-bold [font-weight:700]",
            "space-x-2 [margin-inline-start:1px]",
            "text-sm text-[14px]",
            "text-[14px] text-[1rem]/6",
            "shadow-md shadow-lg/50",
            "text-red-500 text-(length:--size)",
            "text-red-500 text-[length:var(--size)]",
            "bg-red-500 bg-(image:--hero)",
            "bg-cover bg-[10px]",
            "border-2 border-[hsl(calc(180+30)_50%_50%)]",
            "text-[14px] text-[oklch(clamp(0.2,0.5,0.9)_0.1_250)]",
            "font-sans font-(--weight)",
            "font-sans font-[var(--weight)]",
            "foo bar",
            "unknown:flex unknown:grid",
        ] {
            assert_eq!(conflicts(input), Vec::<String>::new(), "{input}");
        }
    }

    #[test]
    fn stylesheet_theme_values_and_variants_are_known() {
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            theme: vec![TailwindThemeEntry::Variable {
                name: "--color-brand".into(),
                value: "#00f".into(),
            }],
            custom_variants: vec!["theme-midnight".into()],
            ..TailwindStylesheet::default()
        });
        assert_eq!(
            conflicts_with("bg-brand bg-red-500", &design),
            ["bg-red-500 > bg-brand"]
        );
        assert_eq!(
            conflicts_with("theme-midnight:flex theme-midnight:grid", &design),
            ["theme-midnight:grid > theme-midnight:flex"]
        );
    }

    #[test]
    fn stylesheet_font_sizes_set_what_the_theme_pairs_them_with() {
        let variable = |name: &'static str, value: &'static str| TailwindThemeEntry::Variable {
            name: name.into(),
            value: value.into(),
        };
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            theme: vec![
                variable("--text-huge", "4rem"),
                variable("--text-giant", "6rem"),
                variable("--text-giant--line-height", "1"),
            ],
            ..TailwindStylesheet::default()
        });
        assert_eq!(
            conflicts_with("text-huge text-[4rem]", &design),
            ["text-[4rem] > text-huge"]
        );
        assert_eq!(
            conflicts_with("text-giant text-lg", &design),
            ["text-lg > text-giant"]
        );
        assert_eq!(
            conflicts_with("text-huge text-giant", &design),
            Vec::<String>::new()
        );
    }

    #[test]
    fn stylesheet_font_families_set_what_the_theme_pairs_them_with() {
        let variable = |name: &'static str, value: &'static str| TailwindThemeEntry::Variable {
            name: name.into(),
            value: value.into(),
        };
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            theme: vec![
                variable("--font-display", "Inter"),
                variable("--font-display--font-feature-settings", "\"cv02\""),
                variable("--font-body", "Georgia"),
            ],
            ..TailwindStylesheet::default()
        });
        assert_eq!(
            conflicts_with("font-body font-sans", &design),
            ["font-sans > font-body"]
        );
        assert_eq!(
            conflicts_with("font-display font-sans", &design),
            Vec::<String>::new()
        );
    }

    /// A static `@utility` whose declarations set `properties`, in a rule that
    /// holds only those declarations when `has_only_declarations` is set.
    fn utility(
        name: &'static str,
        properties: &[&'static str],
        has_only_declarations: bool,
    ) -> TailwindUtility {
        let properties: Box<[_]> = properties.iter().map(|&name| name.into()).collect();
        TailwindUtility {
            name: name.into(),
            declaration_count: properties.len(),
            declared_properties: has_only_declarations.then(|| properties.clone()),
            properties,
        }
    }

    /// A functional `@utility` whose declarations set `properties` to an
    /// integer value.
    fn functional(
        name: &'static str,
        properties: &[&'static str],
        has_only_declarations: bool,
    ) -> TailwindFunctionalUtility {
        TailwindFunctionalUtility {
            name: name.into(),
            declarations: properties
                .iter()
                .map(|&property| TailwindUtilityDeclaration {
                    property: property.into(),
                    functions: Box::new([TailwindValueFunction {
                        is_modifier: false,
                        arguments: Box::new([TailwindValueArgument::Type("integer".into())]),
                    }]),
                })
                .collect(),
            has_only_declarations,
        }
    }

    #[test]
    fn stylesheet_utilities_with_only_declarations_are_compared() {
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            utilities: vec![
                utility("tab-wide", &["tab-size"], true),
                utility("tab-narrow", &["tab-size"], true),
                utility("stack", &["display", "flex-direction"], true),
                utility("column", &["flex-direction", "display"], true),
                utility("gap-var", &["--gap"], true),
                utility("square", &["width", "height"], true),
                utility("link", &["color", "color"], false),
                utility("twice", &["color", "color"], true),
            ],
            functional_utilities: vec![
                functional("tabs", &["tab-size"], true),
                functional("nested-tabs", &["tab-size"], false),
            ],
            ..TailwindStylesheet::default()
        });
        for (input, expected) in [
            ("tab-2 tab-wide", &["tab-wide > tab-2"][..]),
            ("tab-wide tab-narrow", &["tab-narrow > tab-wide"]),
            ("[tab-size:4] tab-wide", &["tab-wide > [tab-size:4]"]),
            (
                "hover:tab-wide hover:tab-2",
                &["hover:tab-2 > hover:tab-wide"],
            ),
            ("stack column", &["column > stack"]),
            ("[--gap:1rem] gap-var", &["gap-var > [--gap:1rem]"]),
            ("tabs-3 tab-2", &["tab-2 > tabs-3"]),
            ("tabs-3 tabs-5", &["tabs-5 > tabs-3"]),
        ] {
            assert_eq!(conflicts_with(input, &design), expected, "{input}");
        }
        for input in [
            "tab-wide hover:tab-2",
            "stack flex",
            "size-4 square",
            "text-red-500 link",
            "text-red-500 twice",
            "nested-tabs-3 tab-2",
        ] {
            assert_eq!(
                conflicts_with(input, &design),
                Vec::<String>::new(),
                "{input}"
            );
        }
    }

    #[test]
    fn skips_classes_that_several_utilities_match() {
        let design = TailwindDesignSystem::from(&TailwindStylesheet {
            utilities: vec![
                utility("twin", &["color"], true),
                utility("twin", &["tab-size"], true),
                utility("flex", &["display"], true),
                utility("tab-4", &["tab-size"], true),
            ],
            functional_utilities: vec![
                functional("tabs", &["tab-size"], true),
                functional("tabs", &["tab-size"], true),
                functional("bg-red", &["color"], true),
            ],
            ..TailwindStylesheet::default()
        });
        for input in [
            "tab-2 twin",
            "text-red-500 twin",
            "flex grid",
            "tab-2 tab-4",
            "tab-2 tabs-3",
            "text-blue-500 bg-red-500",
        ] {
            assert_eq!(
                conflicts_with(input, &design),
                Vec::<String>::new(),
                "{input}"
            );
        }
    }

    #[test]
    fn skips_classes_that_continue_outside_the_string() {
        let parsed = parse_tailwind_with_options(
            "border-2 border",
            &mut NodeCache::default(),
            TailwindDesignSystem::default_ref().parser_options(),
        );
        let candidates = parsed.tree().candidates();
        let partial = candidates.iter().flatten().last().unwrap().range();
        let conflicts =
            find_conflicting_classes(&candidates, TailwindDesignSystem::default_ref(), |range| {
                range == partial
            });
        assert!(conflicts.is_empty());
    }
}
