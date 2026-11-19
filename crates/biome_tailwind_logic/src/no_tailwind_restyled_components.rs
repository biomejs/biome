use crate::class_category::candidate_category;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_tailwind_restyled_components::TailwindComponentAllowance;
use biome_rule_options::tailwind_utility_category::TailwindUtilityCategory;
use biome_tailwind_syntax::TwCandidateList;

/// Returns whether the allowance component `name` matches a component whose name is made of
/// `segments`, such as `Card` and `Root` for `Card.Root`.
///
/// `name` matches if it equals any segment, so both `Card` and `Root` match `Card.Root`.
/// A dotted `name` matches if its segments appear consecutively, so `Card.Root` matches
/// `UI.Card.Root` but not `Card.Header.Root`.
pub fn matches_component_name<S: AsRef<str>>(segments: &[S], name: &str) -> bool {
    segments
        .windows(name.split('.').count())
        .any(|window| window.iter().map(AsRef::as_ref).eq(name.split('.')))
}

/// A utility that restyles a component.
pub struct RestyledUtility {
    /// The range of the utility in the class string.
    pub range: TextRange,
    /// The index in the configured `allow` list of the entry whose `advice` is shown.
    pub advice_index: Option<usize>,
}

/// Returns the entries of `allow` whose components match a component, using `matches_name`
/// to test each configured name.
///
/// Also returns the index in `allow` of the first matching entry that has `advice`.
pub fn component_allowances(
    allow: &[TailwindComponentAllowance],
    matches_name: impl Fn(&str) -> bool,
) -> (Vec<&TailwindComponentAllowance>, Option<usize>) {
    let mut allowances = Vec::new();
    let mut advice = None;
    for (index, allowance) in allow.iter().enumerate() {
        if allowance.components.matches(&matches_name) {
            if advice.is_none() && allowance.advice.is_some() {
                advice = Some(index);
            }
            allowances.push(allowance);
        }
    }
    (allowances, advice)
}

/// Returns ranges of appearance utilities not covered by the supplied allowances.
/// Layout utilities are ignored.
pub fn restyled_component_ranges(
    candidates: &TwCandidateList,
    allowances: &[&TailwindComponentAllowance],
) -> Vec<TextRange> {
    candidates
        .iter()
        .flatten()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
        .filter_map(|full| {
            let category = candidate_category(&full.candidate().ok()?);
            if category == TailwindUtilityCategory::Layout {
                return None;
            }
            if allowances.iter().any(|allow| {
                allow.categories.contains(&category)
                    || allow
                        .classes
                        .iter()
                        .any(|class| full.syntax().text_trimmed() == class.as_ref())
            }) {
                return None;
            }
            Some(full.range())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_tailwind_parser::parse_tailwind;

    #[test]
    fn component_name_matches_any_segment() {
        let segments = ["UI", "Card", "Root"];
        for name in ["UI", "Card", "Root", "UI.Card", "Card.Root", "UI.Card.Root"] {
            assert!(matches_component_name(&segments, name), "{name}");
        }
        for name in [
            "",
            "Car",
            "UI.Root",
            "Root.Card",
            "UI.Card.Root.Item",
            "Card.Root.",
        ] {
            assert!(!matches_component_name(&segments, name), "{name}");
        }
        assert!(matches_component_name(&["my-button"], "my-button"));
        assert!(!matches_component_name(&["my-button"], "my"));
    }

    #[test]
    fn category_allowances() {
        use TailwindUtilityCategory::{Color, Effects, Motion, Shape, Spacing, Typography};
        use biome_rule_options::no_tailwind_restyled_components::{
            TailwindAllowedComponents, TailwindComponentAllowance,
        };

        for (input, expected) in [
            ("text-sm", Typography),
            ("text-red-500", Color),
            ("bg-cover", Effects),
            ("bg-red-500", Color),
            ("bg-gradient-to-r", Effects),
            ("stroke-2", Shape),
            ("stroke-red-500", Color),
            ("border-spacing-2", Spacing),
            ("border-2", Shape),
            ("border-red-500", Color),
            ("decoration-2", Typography),
            ("decoration-red-500", Color),
            ("shadow-lg", Effects),
            ("shadow-red-500", Color),
            ("from-50%", Effects),
            ("from-red-500", Color),
            ("caret-red-500", Color),
            ("not-italic", Typography),
            ("[font-size:14px]", Typography),
            ("[line-height:2]", Typography),
            ("[letter-spacing:2px]", Typography),
            ("[border-width:2px]", Shape),
            ("[padding:0]", Spacing),
            ("[color:red]", Color),
            ("[transition:none]", Motion),
            ("[opacity:0.5]", Effects),
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            for category in [Color, Effects, Motion, Shape, Spacing, Typography] {
                let allow = TailwindComponentAllowance {
                    components: TailwindAllowedComponents::Name("*".into()),
                    categories: vec![category],
                    classes: vec![],
                    advice: None,
                };
                let ranges = restyled_component_ranges(&parse.tree().candidates(), &[&allow]);
                assert_eq!(
                    ranges.is_empty(),
                    category == expected,
                    "{input}: {category:?}"
                );
            }
        }
    }

    #[test]
    fn appearance_utilities() {
        for input in [
            "leading-6",
            "tracking-wide",
            "ring-offset-2",
            "outline-offset-4",
            "text-wrap",
            "rounded-none",
            "hover:!rounded-none",
            "dark:rounded-none!",
            "text-sm/6",
            "border-t-2",
            "font-bold",
            "shadow",
            "ring",
            "italic",
            "not-italic",
            "no-underline",
            "line-through",
            "normal-case",
            "subpixel-antialiased",
            "[&>span]:text-sm",
            "bg-[url('a:b')]",
            "[font-size:14px]",
            "hover:[border-radius:0]",
            "backdrop-blur-sm",
            "inset-shadow-sm",
            "text-[14px]",
            "text-[length:var(--size)]",
            "border-[3px]",
            "border-dashed",
            "shadow-lg",
            "shadow-[0_1px_2px_black]",
            "shadow-[inset_0_1px_2px_rgb(0_0_0/0.25)]",
            "[border:solid_1px_red]",
            "[border:none]",
            "[outline:dashed]",
            "[text-decoration:underline_red]",
            "[border:thin_red]",
            "shadow-[rgb(0_0_0)_0_1px]",
            "bg-linear-to-r",
            "bg-gradient-to-r",
            "bg-gradient-to-tl",
            "hover:!bg-gradient-to-b",
            "[border:1px_solid_red]",
            "[background:url(image.png)]",
            "[border-top-width:2px]",
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            let ranges = restyled_component_ranges(&parse.tree().candidates(), &[]);
            assert_eq!(ranges.len(), 1, "{input}");
            assert_eq!(&input[ranges[0]], input);
        }
    }

    #[test]
    fn explicit_spacing_and_color_allowances() {
        use biome_rule_options::no_tailwind_restyled_components::{
            TailwindAllowedComponents, TailwindComponentAllowance,
        };
        let allow = TailwindComponentAllowance {
            components: TailwindAllowedComponents::Name("*".into()),
            categories: vec![
                TailwindUtilityCategory::Spacing,
                TailwindUtilityCategory::Color,
            ],
            classes: vec![],
            advice: None,
        };
        for input in [
            "",
            "mt-4 w-full",
            "flex grid gap-4",
            "absolute inset-0 z-10",
            "-mx-2",
            "sm:w-[400px]",
            "[margin-top:1rem]",
            "[width:100%]",
            "brand-button",
            "text bg p from",
            "background-card",
            "group peer",
            "[--brand:red]",
            "p-4 px-0 space-x-2",
            "bg-red-500 text-white border-blue-500",
            "hover:!bg-red-500/50 dark:text-white!",
            "text-[#fff] bg-[rgb(0,0,0)]",
            "text-[color:var(--brand)] text-brand",
            "border-[color:var(--brand)]",
            "ring-red-500 outline-blue-500 decoration-red-500 drop-shadow-red-500",
            "shadow-red-500 shadow-[#fff]",
            "shadow-[rgb(0_0_0/0.25)] border-[rgb(0_0_0/0.25)]",
            "fill-current stroke-red-500 accent-blue-500 caret-black",
            "stroke-none",
            "bg-opacity-50",
            "[color:red] [background-color:red] [border-color:red] [padding:0]",
            "[border:red] [outline:rgb(0_0_0)] [text-decoration:red]",
            "border-spacing-2",
            "text-left",
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            assert!(
                restyled_component_ranges(&parse.tree().candidates(), &[&allow]).is_empty(),
                "{input}"
            );
        }
    }
    #[test]
    fn no_categories_allowed_by_default() {
        for input in [
            "p-4",
            "gap-2",
            "bg-red-500",
            "text-white",
            "shadow-red-500",
            "caret-black",
            "from-red-500",
            "[color:red]",
            "[padding:0]",
            "[border:red]",
            "[background:red]",
            "rounded-none",
            "font-bold",
            "opacity-50",
            "animate-spin",
        ] {
            let parse = parse_tailwind(input);
            assert!(!parse.has_errors(), "{input}");
            let ranges = restyled_component_ranges(&parse.tree().candidates(), &[]);
            assert_eq!(ranges.len(), 1, "{input}");
        }
        let parse = parse_tailwind("mt-4 w-full flex text-left appearance-none border-collapse");
        assert!(!parse.has_errors());
        assert!(restyled_component_ranges(&parse.tree().candidates(), &[]).is_empty());
    }
}
