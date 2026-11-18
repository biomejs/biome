//! Finds the classes and variants of a class string that the options of
//! `noTailwindRestrictedClasses` restrict.
//!
//! A class is compared without its variants or `!`, and a variant without its
//! trailing `:`. Both are also compared without a modifier, such as `/50` in
//! `bg-red-500/50` or `/menu` in `group-hover/menu`, unless the key contains a
//! `/`. A fraction such as `1/2` in `w-1/2` is part of the value, not a
//! modifier, so `w-1` doesn't match `w-1/2`. A `-` that makes a value negative
//! is part of the class, so `mt-4` doesn't match `-mt-4`.
//!
//! A `*` in a key stands for a whole part of the class as the parser splits
//! it, not for any characters. In a class key, it's the base, as in `*-md`,
//! the value, as in `bg-*`, or the value of an arbitrary property, as in
//! `[color:*]`. So `bg-*` matches `bg-red-500` but `bg-red-*` doesn't, because
//! the base is `bg` and the value is `red-500`. In a variant key, it's one or
//! more parts between `-`, as in `group-*` or `*-hover`, and `[*]` matches any
//! arbitrary variant.

use biome_rowan::{AstNode, AstSeparatedList, NodeCache, SyntaxNodeText, TextRange, TextSize};
use biome_rule_options::no_tailwind_restricted_classes::{
    NoTailwindRestrictedClassesOptions, TailwindRestriction, TailwindRestrictions,
};
use biome_tailwind_parser::{TailwindParserOptions, parse_tailwind_with_options};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwVariant, TwFullCandidate, TwRoot,
    TwVariantSegmentList,
};

/// A class or variant that the options restrict.
#[derive(Clone, Copy, Debug)]
pub struct RestrictedClass {
    pub kind: RestrictedKind,
    /// The index of the matching entry in the configured `classes` or `variants`.
    index: usize,
    /// The range of the matched text within the class string.
    pub range: TextRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestrictedKind {
    Class,
    Variant,
}

impl RestrictedClass {
    /// Returns the configured key and restriction that matched.
    pub fn restriction<'a>(
        &self,
        options: &'a NoTailwindRestrictedClassesOptions,
    ) -> Option<(&'a str, &'a TailwindRestriction)> {
        let restrictions = match self.kind {
            RestrictedKind::Class => options.classes(),
            RestrictedKind::Variant => options.variants(),
        }?;
        let (key, restriction) = restrictions.get_index(self.index)?;
        Some((key, restriction))
    }

    /// Returns the matched text, as written in `root`.
    pub fn text(&self, root: &TwRoot) -> SyntaxNodeText {
        root.syntax().text_with_trivia().slice(self.range)
    }

    /// Returns the class string of `root` with the matched text replaced by
    /// the configured `use` value.
    ///
    /// Returns `None` when:
    /// - there's no `use` value, or it's empty;
    /// - the key contains a `*`, because the key then doesn't say which part to replace;
    /// - the options also restrict the replaced text, such as `text-sm/6`
    ///   replacing `text-sm`, because applying the fix would never stop reporting.
    ///
    /// `parser_options` must be the options `root` was parsed with, so the
    /// fixed class string is split into bases and values the same way.
    pub fn fixed_class_string(
        &self,
        root: &TwRoot,
        options: &NoTailwindRestrictedClassesOptions,
        parser_options: TailwindParserOptions,
    ) -> Option<String> {
        let (key, restriction) = self.restriction(options)?;
        if key.contains('*') {
            return None;
        }
        let replacement = restriction.use_instead()?;
        let replacement = match self.kind {
            RestrictedKind::Class => replacement,
            RestrictedKind::Variant => replacement.strip_suffix(':').unwrap_or(replacement),
        };
        if replacement.trim().is_empty() {
            return None;
        }
        let text = root.syntax().text_with_trivia().to_string();
        let start = usize::from(self.range.start());
        let end = usize::from(self.range.end());
        let mut fixed = String::with_capacity(text.len() + replacement.len());
        fixed.push_str(text.get(..start)?);
        fixed.push_str(replacement);
        fixed.push_str(text.get(end..)?);
        let parse = parse_tailwind_with_options(&fixed, &mut NodeCache::default(), parser_options);
        let replaced = TextRange::at(self.range.start(), TextSize::of(replacement));
        let is_still_restricted = restricted_classes(&parse.tree(), options)
            .iter()
            .any(|found| {
                found.range.start() < replaced.end() && replaced.start() < found.range.end()
            });
        (!is_still_restricted).then_some(fixed)
    }
}

impl biome_console::fmt::Display for RestrictedKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Class => "class",
            Self::Variant => "variant",
        })
    }
}

/// Writes text from a class string, such as [`RestrictedClass::text`].
pub struct ClassStringText(pub SyntaxNodeText);

impl biome_console::fmt::Display for ClassStringText {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        self.0.try_for_each_chunk(|chunk| fmt.write_str(chunk))
    }
}

/// Returns the classes and variants of `root` that `options` restrict, in the
/// order they're written.
pub fn restricted_classes(
    root: &TwRoot,
    options: &NoTailwindRestrictedClassesOptions,
) -> Box<[RestrictedClass]> {
    let classes = options.classes().filter(|classes| !classes.is_empty());
    let variants = options.variants().filter(|variants| !variants.is_empty());
    if classes.is_none() && variants.is_none() {
        return Box::default();
    }
    let candidates = root.candidates();
    if candidates.is_empty() {
        return Box::default();
    }
    let text = root.syntax().text_with_trivia().to_string();
    let mut found = Vec::new();
    for candidate in candidates.iter().flatten() {
        let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
            continue;
        };
        if let Some(variants) = variants {
            for variant in candidate.variants().iter().flatten() {
                let Some(variant) = VariantParts::new(variant) else {
                    continue;
                };
                found.extend(find_restriction(
                    variants,
                    RestrictedKind::Variant,
                    variant.range,
                    variant.modifier_start,
                    |key, range| variant.matches(key, range, &text),
                ));
            }
        }
        if let Some(classes) = classes
            && let Some(class) = ClassParts::new(&candidate)
        {
            found.extend(find_restriction(
                classes,
                RestrictedKind::Class,
                class.range,
                class.modifier_start,
                |key, range| class.matches(key, range, &text),
            ));
        }
    }
    found.into_boxed_slice()
}

/// Returns the first entry of `restrictions` whose key `matches` the class or
/// variant at `range`.
///
/// Keys without a `/` are compared with the text before `modifier_start`.
fn find_restriction(
    restrictions: &TailwindRestrictions,
    kind: RestrictedKind,
    range: TextRange,
    modifier_start: Option<TextSize>,
    matches: impl Fn(&str, TextRange) -> bool,
) -> Option<RestrictedClass> {
    restrictions.keys().enumerate().find_map(|(index, key)| {
        let key = match kind {
            RestrictedKind::Class => key.as_ref(),
            RestrictedKind::Variant => key.strip_suffix(':').unwrap_or(key),
        };
        let range = if key.contains('/') {
            range
        } else {
            TextRange::new(range.start(), modifier_start.unwrap_or(range.end()))
        };
        matches(key, range).then_some(RestrictedClass { kind, index, range })
    })
}

/// A class without its variants or `!`.
struct ClassParts {
    /// Starts at the `-` of a negative value.
    range: TextRange,
    /// The start of the modifier, such as `/50` in `bg-red-500/50`.
    ///
    /// A fraction such as `1/2` in `w-1/2` is parsed as a number and a
    /// modifier, but it's one value, so it has no modifier start.
    modifier_start: Option<TextSize>,
    is_negative: bool,
    shape: ClassShape,
}

enum ClassShape {
    /// A class without a value, such as `flex`.
    Static,
    /// A class with a base and a value, such as `bg-red-500`.
    Functional {
        base: TextRange,
        value_start: TextSize,
    },
    /// An arbitrary property, such as `[color:red]`.
    Arbitrary { property: TextRange },
}

impl ClassParts {
    fn new(full: &TwFullCandidate) -> Option<Self> {
        let candidate = full.candidate().ok()?;
        let (modifier, shape) = match &candidate {
            AnyTwCandidate::TwArbitraryCandidate(candidate) => (
                candidate.modifier(),
                ClassShape::Arbitrary {
                    property: candidate.property_token().ok()?.text_trimmed_range(),
                },
            ),
            AnyTwCandidate::TwFunctionalCandidate(candidate) => {
                let value = candidate.value().ok()?;
                let modifier = candidate.modifier().filter(|modifier| {
                    !(value.as_tw_number_value().is_some()
                        && modifier.as_tw_modifier().is_some_and(|modifier| {
                            modifier
                                .value()
                                .is_ok_and(|value| value.as_tw_number_value().is_some())
                        }))
                });
                (
                    modifier,
                    ClassShape::Functional {
                        base: candidate.base_token().ok()?.text_trimmed_range(),
                        value_start: value.syntax().text_trimmed_range().start(),
                    },
                )
            }
            AnyTwCandidate::TwStaticCandidate(candidate) => {
                (candidate.modifier(), ClassShape::Static)
            }
            AnyTwCandidate::TwBogusCandidate(_) => return None,
        };
        let negative = full.negative_token();
        let range = candidate.syntax().text_trimmed_range();
        let start = negative.as_ref().map_or(range.start(), |negative| {
            negative.text_trimmed_range().start()
        });
        Some(Self {
            range: TextRange::new(start, range.end()),
            modifier_start: modifier_start(modifier.as_ref()),
            is_negative: negative.is_some(),
            shape,
        })
    }

    /// Returns whether `key` matches this class, compared up to the end of `range`.
    ///
    /// In a key, `*` stands for the whole base, as in `*-md`, the whole value,
    /// as in `bg-*`, or the value of an arbitrary property, as in `[color:*]`.
    fn matches(&self, key: &str, range: TextRange, text: &str) -> bool {
        if !key.contains('*') {
            return slice(text, range) == Some(key);
        }
        let (is_negative, pattern) = key
            .strip_prefix('-')
            .map_or((false, key), |pattern| (true, pattern));
        if is_negative != self.is_negative {
            return false;
        }
        match self.shape {
            ClassShape::Functional { base, value_start } => {
                if let Some(value) = pattern.strip_prefix("*-") {
                    slice(text, TextRange::new(value_start, range.end())) == Some(value)
                } else if let Some(base_pattern) = pattern.strip_suffix("-*") {
                    slice(text, base) == Some(base_pattern)
                } else {
                    false
                }
            }
            ClassShape::Arbitrary { property } => pattern
                .strip_prefix('[')
                .and_then(|pattern| pattern.strip_suffix(":*]"))
                .is_some_and(|pattern| slice(text, property) == Some(pattern)),
            ClassShape::Static => false,
        }
    }
}

/// A variant without its trailing `:`.
struct VariantParts {
    range: TextRange,
    /// The start of the modifier, such as `/menu` in `group-hover/menu`.
    modifier_start: Option<TextSize>,
    /// The parts of the variant between `-`, or `None` for an arbitrary
    /// variant such as `[&_p]`.
    segments: Option<TwVariantSegmentList>,
}

impl VariantParts {
    fn new(variant: AnyTwVariant) -> Option<Self> {
        let range = variant.syntax().text_trimmed_range();
        match variant {
            AnyTwVariant::TwArbitraryVariant(_) => Some(Self {
                range,
                modifier_start: None,
                segments: None,
            }),
            AnyTwVariant::TwVariantExpression(variant) => Some(Self {
                range,
                modifier_start: modifier_start(variant.modifier().as_ref()),
                segments: Some(variant.segments()),
            }),
            AnyTwVariant::TwBogusVariant(_) => None,
        }
    }

    /// Returns whether `key` matches this variant, compared up to the end of `range`.
    ///
    /// In a key, `*` stands for one or more parts at the end, as in `group-*`,
    /// or at the start, as in `*-hover`. The key `[*]` matches any arbitrary
    /// variant.
    fn matches(&self, key: &str, range: TextRange, text: &str) -> bool {
        let Some(segments) = &self.segments else {
            return key == "[*]" || slice(text, range) == Some(key);
        };
        if !key.contains('*') {
            return slice(text, range) == Some(key);
        }
        if let Some(prefix) = key.strip_suffix("-*") {
            segments
                .iter()
                .take(segments.len().saturating_sub(1))
                .flatten()
                .any(|segment| {
                    let end = segment.syntax().text_trimmed_range().end();
                    slice(text, TextRange::new(range.start(), end)) == Some(prefix)
                })
        } else if let Some(suffix) = key.strip_prefix("*-") {
            segments.iter().skip(1).flatten().any(|segment| {
                let start = segment.syntax().text_trimmed_range().start();
                start <= range.end()
                    && slice(text, TextRange::new(start, range.end())) == Some(suffix)
            })
        } else {
            false
        }
    }
}

fn modifier_start(modifier: Option<&AnyTwModifier>) -> Option<TextSize> {
    modifier.map(|modifier| modifier.syntax().text_trimmed_range().start())
}

fn slice(text: &str, range: TextRange) -> Option<&str> {
    text.get(usize::from(range.start())..usize::from(range.end()))
}

#[cfg(test)]
mod tests {
    use super::restricted_classes;
    use biome_rowan::AstNode;
    use biome_rule_options::no_tailwind_restricted_classes::NoTailwindRestrictedClassesOptions;
    use biome_tailwind_parser::TailwindParserOptions;
    use biome_tailwind_parser::parse_tailwind;

    fn options(json: &str) -> NoTailwindRestrictedClassesOptions {
        serde_json::from_str(json).unwrap()
    }

    /// Returns the matched texts and the fixed class strings.
    fn check(
        source: &str,
        options: &NoTailwindRestrictedClassesOptions,
    ) -> Vec<(String, Option<String>)> {
        let parse = parse_tailwind(source);
        assert!(!parse.has_errors(), "{source}: {:?}", parse.diagnostics());
        let root = parse.tree();
        restricted_classes(&root, options)
            .iter()
            .map(|found| {
                assert_eq!(
                    found.text(&root).to_string(),
                    root.syntax().text_with_trivia().to_string()[found.range].to_string()
                );
                (
                    found.text(&root).to_string(),
                    found.fixed_class_string(&root, options, TailwindParserOptions::default()),
                )
            })
            .collect()
    }

    #[test]
    fn matches_classes_without_variants_important_or_modifier() {
        let options = options(r#"{ "classes": { "bg-green-500": { "use": "bg-success" } } }"#);
        assert_eq!(
            check("p-4 md:hover:bg-green-500/50! flex", &options),
            [(
                "bg-green-500".to_string(),
                Some("p-4 md:hover:bg-success/50! flex".to_string())
            )]
        );
        assert_eq!(
            check("!bg-green-500", &options),
            [("bg-green-500".to_string(), Some("!bg-success".to_string()))]
        );
    }

    #[test]
    fn keys_with_a_slash_include_the_modifier() {
        let options = options(r#"{ "classes": { "w-1/2": { "use": "w-half" } } }"#);
        assert_eq!(
            check("w-1 w-1/2 w-1/3", &options),
            [("w-1/2".to_string(), Some("w-1 w-half w-1/3".to_string()))]
        );
    }

    #[test]
    fn fractions_are_part_of_the_value() {
        let options = options(r#"{ "classes": { "w-1": { "use": "w-px" }, "text-sm": "a" } }"#);
        assert_eq!(
            check("w-1/2 aspect-4/3 w-1 text-sm/6", &options),
            [
                (
                    "w-1".to_string(),
                    Some("w-1/2 aspect-4/3 w-px text-sm/6".to_string())
                ),
                ("text-sm".to_string(), None),
            ]
        );
    }

    #[test]
    fn no_fix_when_the_replacement_is_restricted() {
        let options = options(
            r#"{ "classes": { "text-sm": { "use": "text-sm/6" }, "a": { "use": "b" }, "b": { "use": "a" }, "c": { "use": "c" } }, "variants": { "dark": { "use": "dark" } } }"#,
        );
        assert_eq!(
            check("text-sm a b c dark:p-4", &options),
            [
                ("text-sm".to_string(), None),
                ("a".to_string(), None),
                ("b".to_string(), None),
                ("c".to_string(), None),
                ("dark".to_string(), None),
            ]
        );
    }

    #[test]
    fn variant_replacements_may_end_with_a_colon() {
        let options = options(r#"{ "variants": { "group-hover:": { "use": "group-focus:" } } }"#);
        assert_eq!(
            check("group-hover/menu:p-4", &options),
            [(
                "group-hover".to_string(),
                Some("group-focus/menu:p-4".to_string())
            )]
        );
    }

    #[test]
    fn no_fix_for_an_empty_replacement() {
        let options = options(r#"{ "classes": { "bg-green-500": { "use": " " } } }"#);
        assert_eq!(
            check("hover:bg-green-500", &options),
            [("bg-green-500".to_string(), None)]
        );
    }

    #[test]
    fn negative_values_only_match_negative_keys() {
        let options = options(r#"{ "classes": { "-mt-4": "No negative margins." } }"#);
        assert_eq!(
            check("mt-4 md:-mt-4", &options),
            [("-mt-4".to_string(), None)]
        );
    }

    #[test]
    fn wildcard_keys_have_no_fix() {
        let options = options(r#"{ "classes": { "float-*": { "use": "flex" } } }"#);
        assert_eq!(
            check("float-left [float:left]", &options),
            [("float-left".to_string(), None)]
        );
    }

    #[test]
    fn first_matching_key_wins() {
        let options = options(r#"{ "classes": { "text-*": "first", "text-red-500": "second" } }"#);
        let parse = parse_tailwind("text-red-500");
        let root = parse.tree();
        let found = restricted_classes(&root, &options);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].restriction(&options).unwrap().0, "text-*");
    }

    #[test]
    fn matches_variants() {
        let options = options(
            r#"{ "variants": { "dark:": { "use": "theme-dark" }, "group-hover": "No groups.", "[*]": "No arbitrary variants." } }"#,
        );
        assert_eq!(
            check(
                "dark:md:flex group-hover/menu:p-4 [&_p]:mt-2 hover:flex",
                &options
            ),
            [
                (
                    "dark".to_string(),
                    Some(
                        "theme-dark:md:flex group-hover/menu:p-4 [&_p]:mt-2 hover:flex".to_string()
                    )
                ),
                ("group-hover".to_string(), None),
                ("[&_p]".to_string(), None),
            ]
        );
    }

    #[test]
    fn reports_variants_and_classes_of_the_same_candidate() {
        let options = options(r#"{ "classes": { "flex": "a" }, "variants": { "dark": "b" } }"#);
        assert_eq!(
            check("dark:flex", &options),
            [("dark".to_string(), None), ("flex".to_string(), None)]
        );
    }

    #[test]
    fn ignores_unrestricted_classes() {
        let options = options(
            r#"{ "classes": { "flex": "a", "float-*": "b", "*-md": "c", "bg-red-*": "d" }, "variants": { "dark": "e", "group-*": "f", "*-hover": "g" } }"#,
        );
        assert!(
            check(
                "inline-flex flex-1 float text-red-md text-mdx bg-red-500 darker:p-4 hover:p-4 group/item:p-4 [&_p]:p-4",
                &options
            )
            .is_empty()
        );
    }

    #[test]
    fn class_wildcards_replace_the_base_or_the_value() {
        let options = options(
            r#"{ "classes": { "bg-*": "a", "*-md": "b", "[color:*]": "c", "-*-4": "d", "max-w-*": "e" } }"#,
        );
        assert_eq!(
            check(
                "bg-red-500 hover:bg-[#fff]/50 shadow-md max-w-md mt-4 -mt-4 [color:red] [background:red] bg",
                &options
            ),
            [
                ("bg-red-500".to_string(), None),
                ("bg-[#fff]".to_string(), None),
                ("shadow-md".to_string(), None),
                ("max-w-md".to_string(), None),
                ("-mt-4".to_string(), None),
                ("[color:red]".to_string(), None),
            ]
        );
    }

    #[test]
    fn variant_wildcards_replace_leading_or_trailing_parts() {
        let options = options(r#"{ "variants": { "group-*": "a", "*-hover": "b", "[*]": "c" } }"#);
        assert_eq!(
            check(
                "group-hover:p-1 group-focus/item:p-2 peer-hover:p-3 hover:p-4 [&_p]:p-5 data-[open]:p-6",
                &options
            ),
            [
                ("group-hover".to_string(), None),
                ("group-focus".to_string(), None),
                ("peer-hover".to_string(), None),
                ("[&_p]".to_string(), None),
            ]
        );
    }
}
