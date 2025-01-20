use biome_diagnostics::{Category, Diagnostic};
use biome_rowan::{TextLen, TextRange, TextSize};
use std::ops::Add;

/// Single instance of a suppression comment, with the following syntax:
///
/// `// biome-ignore { <category> { (<value>) }? }+: <reason>`
///
/// The category broadly describes what feature is being suppressed (formatting,
/// linting, ...) with the value being and optional, category-specific name of
/// a specific element to disable (for instance a specific lint name). A single
/// suppression may specify one or more categories + values, for instance to
/// disable multiple lints at once
///
/// A suppression must specify a reason: this part has no semantic meaning but
/// is required to document why a particular feature is being disabled for this
/// line (lint false-positive, specific formatting requirements, ...)
#[derive(Debug, PartialEq, Eq)]
pub struct Suppression<'a> {
    /// List of categories for this suppression
    ///
    /// Categories are a pair of the category name +
    /// an optional category value
    pub categories: Vec<(&'a Category, Option<&'a str>)>,
    /// Reason for this suppression comment to exist
    pub reason: &'a str,

    /// What suppression is
    pub kind: SuppressionKind,

    range: TextRange,
}

impl Suppression<'_> {
    pub fn range(&self) -> TextRange {
        self.range
    }
    // pub fn reason_range(&self) -> TextRange {}
    // pub fn categories(&self)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuppressionKind {
    /// Suppressions that start with `// biome-ignore`
    Classic,
    /// Suppressions that start with `// biome-ignore-all`
    All,
    /// Suppressions that start with `// biome-ignore-start`
    RangeStart,
    /// Suppressions that start with `// biome-ignore-end`
    RangeEnd,
}

impl SuppressionKind {
    pub fn as_str(&self) -> &str {
        match self {
            SuppressionKind::Classic => "biome-ignore",
            SuppressionKind::All => "biome-ignore-all",
            SuppressionKind::RangeStart => "biome-ignore-start",
            SuppressionKind::RangeEnd => "biome-ignore-end",
        }
    }

    fn text_len(&self) -> TextSize {
        self.as_str().text_len()
    }
}

const ALL_PATTERNS: [&str; 2] = ["-ALL", "-all"];
const RANGE_START_PATTERNS: [&str; 2] = ["-START", "-start"];
const RANGE_END_PATTERNS: [&str; 2] = ["-END", "-end"];

pub fn parse_suppression_comment(
    base: &str,
) -> impl Iterator<Item = Result<Suppression, SuppressionDiagnostic>> {
    let (head, mut comment) = if base.starts_with('#') {
        base.split_at(1)
    } else if base.starts_with("<!--") {
        base.split_at(4)
    } else {
        base.split_at(2)
    };

    let is_block_comment = match head {
        "//" => false,
        "/*" => {
            comment = comment
                .strip_suffix("*/")
                .or_else(|| comment.strip_suffix(['*', '/']))
                .unwrap_or(comment);
            true
        }
        "#" => false,
        "<!--" => {
            comment = comment.strip_suffix("-->").unwrap_or(comment);
            true
        }
        token => panic!("comment with unknown opening token {token:?}, from {comment}"),
    };

    comment.lines().filter_map(move |line| {
        let mut kind = SuppressionKind::Classic;
        // Eat start of line whitespace
        let mut line = line.trim_start();

        // If we're in a block comment eat stars, then whitespace again
        if is_block_comment {
            line = line.trim_start_matches('*').trim_start()
        }

        line = line.trim_start();

        const PATTERN: [[char; 2]; 12] = [
            ['b', 'B'],
            ['i', 'I'],
            ['o', 'O'],
            ['m', 'M'],
            ['e', 'E'],
            ['-', '_'],
            ['i', 'I'],
            ['g', 'G'],
            ['n', 'N'],
            ['o', 'O'],
            ['r', 'R'],
            ['e', 'E'],
        ];

        if line.is_empty() {
            return None;
        }

        // it's a biome-ignore comment
        // Checks for `/biome[-_]ignore/i` without a regex, or skip the line
        // entirely if it doesn't match
        for pattern in PATTERN {
            line = line.strip_prefix(pattern)?;
        }

        for all_pattern in ALL_PATTERNS {
            if let Some(result) = line.strip_prefix(all_pattern) {
                kind = SuppressionKind::All;
                line = result;
            }
        }

        for start_pattern in RANGE_START_PATTERNS {
            if let Some(result) = line.strip_prefix(start_pattern) {
                kind = SuppressionKind::RangeStart;
                line = result;
            }
        }

        for end_patter in RANGE_END_PATTERNS {
            if let Some(result) = line.strip_prefix(end_patter) {
                kind = SuppressionKind::RangeEnd;
                line = result;
            }
        }

        let line = line.trim_start();
        let range = base
            .find(kind.as_str())
            .map(|start| {
                let start = TextSize::from(start as u32);
                let end = start.add(kind.text_len());
                TextRange::new(start, end)
            })
            .expect("To find the suppression prefix");
        Some(
            parse_suppression_line(line, kind, range).map_err(|err| SuppressionDiagnostic {
                message: err.message,
                // Adjust the position of the diagnostic in the whole comment
                span: err.span + offset_from(base, line),
            }),
        )
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Diagnostic)]
#[diagnostic(category = "suppressions/parse")]
pub struct SuppressionDiagnostic {
    #[message]
    #[description]
    message: SuppressionDiagnosticKind,
    #[location(span)]
    span: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SuppressionDiagnosticKind {
    MissingColon,
    ParseCategory(String),
    MissingCategory,
    MissingParen,
}

impl std::fmt::Display for SuppressionDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuppressionDiagnosticKind::MissingColon => write!(
                f,
                "unexpected token, expected one of ':', '(' or whitespace"
            ),
            SuppressionDiagnosticKind::ParseCategory(category) => {
                write!(f, "failed to parse category {category:?}")
            }
            SuppressionDiagnosticKind::MissingCategory => {
                write!(f, "unexpected token, expected one of ':' or whitespace")
            }
            SuppressionDiagnosticKind::MissingParen => write!(f, "unexpected token, expected ')'"),
        }
    }
}

impl biome_console::fmt::Display for SuppressionDiagnosticKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            SuppressionDiagnosticKind::MissingColon => write!(
                fmt,
                "unexpected token, expected one of ':', '(' or whitespace"
            ),
            SuppressionDiagnosticKind::ParseCategory(category) => {
                write!(fmt, "failed to parse category {category:?}")
            }
            SuppressionDiagnosticKind::MissingCategory => {
                write!(fmt, "unexpected token, expected one of ':' or whitespace")
            }
            SuppressionDiagnosticKind::MissingParen => {
                write!(fmt, "unexpected token, expected ')'")
            }
        }
    }
}

/// Parse the `{ <category> { (<value>) }? }+: <reason>` section of a suppression line
fn parse_suppression_line(
    base: &str,
    kind: SuppressionKind,
    range: TextRange,
) -> Result<Suppression, SuppressionDiagnostic> {
    let mut line = base;
    let mut categories = Vec::new();

    'outer: loop {
        // Find either a colon opening parenthesis or space
        let separator = line
            .find(|c: char| c == ':' || c == '(' || c.is_whitespace())
            .ok_or_else(|| SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingColon,
                span: TextRange::at(offset_from(base, line), TextSize::of(line)),
            })?;

        let (category, rest) = line.split_at(separator);
        let category = category.trim_end();
        let category: Option<&'static Category> = if !category.is_empty() {
            let category = category.parse().map_err(|()| SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::ParseCategory(category.into()),
                span: TextRange::at(offset_from(base, category), TextSize::of(category)),
            })?;
            Some(category)
        } else {
            None
        };

        // Skip over and match the separator
        let (separator, rest) = rest.split_at(1);

        match separator {
            // Colon token: stop parsing categories
            ":" => {
                if let Some(category) = category {
                    categories.push((category, None));
                }

                line = rest.trim_start();
                break 'outer;
            }
            // Paren token: parse a category + value
            "(" => {
                let category = category.ok_or_else(|| SuppressionDiagnostic {
                    message: SuppressionDiagnosticKind::MissingCategory,
                    span: TextRange::at(
                        offset_from(base, line),
                        offset_from(line, separator) + TextSize::of(separator),
                    ),
                })?;
                let paren = rest.find(')').ok_or_else(|| SuppressionDiagnostic {
                    message: SuppressionDiagnosticKind::MissingParen,
                    span: TextRange::at(offset_from(base, rest), TextSize::of(rest)),
                })?;

                let (value, rest) = rest.split_at(paren);
                let value = value.trim();

                categories.push((category, Some(value)));

                line = rest.strip_prefix(')').unwrap().trim_start();
            }
            // Whitespace: push a category without value
            _ => {
                if let Some(category) = category {
                    categories.push((category, None));
                }

                line = rest.trim_start();
            }
        }
    }

    let reason = line.trim_end();
    Ok(Suppression {
        categories,
        reason,
        kind,
        range,
    })
}

/// Returns the byte offset of `substr` within `base`
///
/// # Safety
///
/// `substr` must be a substring of `base`, or calling this method will result
/// in undefined behavior.
fn offset_from(base: &str, substr: &str) -> TextSize {
    let base_len = base.len();
    debug_assert!(substr.len() <= base_len);

    let base = base.as_ptr();
    let substr = substr.as_ptr();
    let offset = unsafe { substr.offset_from(base) };

    // SAFETY: converting from `isize` to `usize` can only fail if `offset` is
    // negative, meaning `base` is either a substring of `substr` or the two
    // string slices are unrelated
    let offset = usize::try_from(offset).expect("usize underflow");
    debug_assert!(offset <= base_len);

    // SAFETY: the conversion from `usize` to `TextSize` can fail if `offset`
    // is larger than 2^32
    TextSize::try_from(offset).expect("TextSize overflow")
}

#[cfg(test)]
mod tests_suppression_kinds {
    use crate::{parse_suppression_comment, Suppression, SuppressionKind};
    use biome_diagnostics::category;
    use biome_rowan::{TextRange, TextSize};

    #[test]
    fn classic() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );
    }

    #[test]
    fn all() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );
    }

    #[test]
    fn range_start() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-start format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation",
                kind: SuppressionKind::RangeStart,
                range: TextRange::new(TextSize::from(3), TextSize::from(21))
            })],
        );
    }

    #[test]
    fn range_end() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-end format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation",
                kind: SuppressionKind::RangeEnd,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );
    }
}

#[cfg(test)]
mod tests_biome_ignore_inline {
    use biome_diagnostics::category;
    use biome_rowan::{TextRange, TextSize};

    use crate::{offset_from, SuppressionDiagnostic, SuppressionDiagnosticKind, SuppressionKind};

    use super::{parse_suppression_comment, Suppression};

    #[test]
    fn parse_simple_suppression() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore parse: explanation1").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation1",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/** biome-ignore parse: explanation2 */")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation2",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(4), TextSize::from(16))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * biome-ignore parse: explanation3
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation3",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(24), TextSize::from(36))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * biome-ignore parse: explanation4
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation4",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(50), TextSize::from(62))
            })],
        );
    }
    #[test]
    fn parse_unclosed_block_comment_suppressions() {
        assert_eq!(
            parse_suppression_comment("/* biome-ignore format: explanation").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/* biome-ignore format: explanation *").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/* biome-ignore format: explanation /").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );
    }

    #[test]
    fn parse_multiple_suppression() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore parse(foo) parse(dog): explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("foo")),
                    (category!("parse"), Some("dog"))
                ],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/** biome-ignore parse(bar) parse(cat): explanation */")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("bar")),
                    (category!("parse"), Some("cat"))
                ],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(4), TextSize::from(16))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * biome-ignore parse(yes) parse(frog): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("yes")),
                    (category!("parse"), Some("frog"))
                ],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(24), TextSize::from(36))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * biome-ignore parse(wow) parse(fish): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("wow")),
                    (category!("parse"), Some("fish"))
                ],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(50), TextSize::from(62))
            })],
        );
    }

    #[test]
    fn parse_multiple_suppression_categories() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation",
                kind: SuppressionKind::Classic,
                range: TextRange::new(TextSize::from(3), TextSize::from(15))
            })],
        );
    }

    #[test]
    fn check_offset_from() {
        const BASE: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";

        assert_eq!(offset_from(BASE, BASE), TextSize::from(0));

        let (_, substr) = BASE.split_at(55);
        assert_eq!(offset_from(BASE, substr), TextSize::from(55));

        let (_, substr) = BASE.split_at(BASE.len());
        assert_eq!(offset_from(BASE, substr), TextSize::of(BASE));
    }

    #[test]
    fn diagnostic_missing_colon() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore format explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingColon,
                span: TextRange::new(TextSize::from(23), TextSize::from(34))
            })],
        );
    }

    #[test]
    fn diagnostic_missing_paren() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore format(:").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingParen,
                span: TextRange::new(TextSize::from(23), TextSize::from(24))
            })],
        );
    }

    #[test]
    fn diagnostic_missing_category() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore (value): explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingCategory,
                span: TextRange::new(TextSize::from(16), TextSize::from(17))
            })],
        );
    }

    #[test]
    fn diagnostic_unknown_category() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore unknown: explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::ParseCategory(String::from("unknown")),
                span: TextRange::new(TextSize::from(16), TextSize::from(23))
            })],
        );
    }
}

#[cfg(test)]
mod tests_biome_ignore_toplevel {
    use biome_diagnostics::category;
    use biome_rowan::{TextRange, TextSize};

    use crate::{offset_from, SuppressionDiagnostic, SuppressionDiagnosticKind, SuppressionKind};

    use super::{parse_suppression_comment, Suppression};

    #[test]
    fn parse_simple_suppression() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all parse: explanation1")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation1",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/** biome-ignore-all parse: explanation2 */")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation2",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(4), TextSize::from(20))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * biome-ignore-all parse: explanation3
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation3",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(24), TextSize::from(40))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * biome-ignore-all parse: explanation4
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation4",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(50), TextSize::from(66))
            })],
        );
    }
    #[test]
    fn parse_unclosed_block_comment_suppressions() {
        assert_eq!(
            parse_suppression_comment("/* biome-ignore-all format: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/* biome-ignore-all format: explanation *")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/* biome-ignore-all format: explanation /")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );
    }

    #[test]
    fn parse_multiple_suppression() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all parse(foo) parse(dog): explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("foo")),
                    (category!("parse"), Some("dog"))
                ],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );

        assert_eq!(
            parse_suppression_comment("/** biome-ignore-all parse(bar) parse(cat): explanation */")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("bar")),
                    (category!("parse"), Some("cat"))
                ],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(4), TextSize::from(20))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * biome-ignore-all parse(yes) parse(frog): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("yes")),
                    (category!("parse"), Some("frog"))
                ],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(24), TextSize::from(40))
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * biome-ignore-all parse(wow) parse(fish): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("wow")),
                    (category!("parse"), Some("fish"))
                ],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(50), TextSize::from(66))
            })],
        );
    }

    #[test]
    fn parse_multiple_suppression_categories() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation",
                kind: SuppressionKind::All,
                range: TextRange::new(TextSize::from(3), TextSize::from(19))
            })],
        );
    }

    #[test]
    fn check_offset_from() {
        const BASE: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";

        assert_eq!(offset_from(BASE, BASE), TextSize::from(0));

        let (_, substr) = BASE.split_at(55);
        assert_eq!(offset_from(BASE, substr), TextSize::from(55));

        let (_, substr) = BASE.split_at(BASE.len());
        assert_eq!(offset_from(BASE, substr), TextSize::of(BASE));
    }

    #[test]
    fn diagnostic_missing_colon() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all format explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingColon,
                span: TextRange::new(TextSize::from(27), TextSize::from(38))
            })],
        );
    }

    #[test]
    fn diagnostic_missing_paren() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all format(:").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingParen,
                span: TextRange::new(TextSize::from(27), TextSize::from(28))
            })],
        );
    }

    #[test]
    fn diagnostic_missing_category() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all (value): explanation")
                .collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingCategory,
                span: TextRange::new(TextSize::from(20), TextSize::from(21))
            })],
        );
    }

    #[test]
    fn diagnostic_unknown_category() {
        assert_eq!(
            parse_suppression_comment("// biome-ignore-all unknown: explanation")
                .collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::ParseCategory(String::from("unknown")),
                span: TextRange::new(TextSize::from(20), TextSize::from(27))
            })],
        );
    }
}
