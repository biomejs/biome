use rome_diagnostics::{Category, Diagnostic};
use rome_rowan::{TextRange, TextSize};

/// Single instance of a suppression comment, with the following syntax:
///
/// `// rome-ignore { <category> { (<value>) }? }+: <reason>`
///
/// The category broadly describes what feature is being suppressed (formatting,
/// linting, ...) with the value being and optional, category-specific name of
/// a specific element to disable (for instance a specific lint name). A single
/// suppression may specify one or more categories + values, for instance to
/// disable multiple lints at once
///
/// A suppression must specify a reason: this part has no semantic meaning but
/// is required to document why a particular feature is being disable for this
/// line (lint false-positive, specific formatting requirements, ...)
#[derive(Debug, PartialEq, Eq)]
pub struct Suppression<'a> {
    /// List of categories for this suppression
    ///
    /// Categories are pair of the category name +
    /// an optional category value
    pub categories: Vec<(&'a Category, Option<&'a str>)>,
    /// Reason for this suppression comment to exist
    pub reason: &'a str,
}

pub fn parse_suppression_comment(
    base: &str,
) -> impl Iterator<Item = Result<Suppression, SuppressionDiagnostic>> {
    let (head, mut comment) = base.split_at(2);
    let is_block_comment = match head {
        "//" => false,
        "/*" => {
            comment = comment
                .strip_suffix("*/")
                .or_else(|| comment.strip_suffix(&['*', '/']))
                .unwrap_or(comment);
            true
        }
        token => panic!("comment with unknown opening token {token:?}, from {comment}"),
    };

    comment.lines().filter_map(move |line| {
        // Eat start of line whitespace
        let mut line = line.trim_start();

        // If we're in a block comment eat stars, then whitespace again
        if is_block_comment {
            line = line.trim_start_matches('*').trim_start()
        }

        const PATTERNS: [[char; 2]; 11] = [
            ['r', 'R'],
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

        // Checks for `/rome[-_]ignore/i` without a regex, or skip the line
        // entirely if it doesn't match
        for pattern in PATTERNS {
            line = line.strip_prefix(pattern)?;
        }

        let line = line.trim_start();
        Some(
            parse_suppression_line(line).map_err(|err| SuppressionDiagnostic {
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

impl rome_console::fmt::Display for SuppressionDiagnosticKind {
    fn fmt(&self, fmt: &mut rome_console::fmt::Formatter) -> std::io::Result<()> {
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
fn parse_suppression_line(base: &str) -> Result<Suppression, SuppressionDiagnostic> {
    let mut line = base;
    let mut categories = Vec::new();

    loop {
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
                break;
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
    Ok(Suppression { categories, reason })
}

/// Returns the byte offset of `substr` within `base`
///
/// # Safety
///
/// `substr` must be a substring of `base`, or calling this method will result
/// in undefined behavior.
fn offset_from(base: &str, substr: &str) -> TextSize {
    let base_len = base.len();
    assert!(substr.len() <= base_len);

    let base = base.as_ptr();
    let substr = substr.as_ptr();
    let offset = unsafe { substr.offset_from(base) };

    // SAFETY: converting from `isize` to `usize` can only fail if `offset` is
    // negative, meaning `base` is either a substring of `substr` or the two
    // string slices are unrelated
    let offset = usize::try_from(offset).expect("usize underflow");
    assert!(offset <= base_len);

    // SAFETY: the conversion from `usize` to `TextSize` can fail if `offset`
    // is larger than 2^32
    TextSize::try_from(offset).expect("TextSize overflow")
}

#[cfg(test)]
mod tests {
    use rome_diagnostics::category;
    use rome_rowan::{TextRange, TextSize};

    use crate::{offset_from, SuppressionDiagnostic, SuppressionDiagnosticKind};

    use super::{parse_suppression_comment, Suppression};

    #[test]
    fn parse_simple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse: explanation1").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation1"
            })],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse: explanation2 */").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation2"
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * rome-ignore parse: explanation3
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation3"
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * rome-ignore parse: explanation4
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation4"
            })],
        );
    }
    #[test]
    fn parse_unclosed_block_comment_suppressions() {
        assert_eq!(
            parse_suppression_comment("/* rome-ignore format: explanation").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation"
            })],
        );

        assert_eq!(
            parse_suppression_comment("/* rome-ignore format: explanation *").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation"
            })],
        );

        assert_eq!(
            parse_suppression_comment("/* rome-ignore format: explanation /").collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation"
            })],
        );
    }

    #[test]
    fn parse_multiple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse(foo) parse(dog): explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("foo")),
                    (category!("parse"), Some("dog"))
                ],
                reason: "explanation"
            })],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse(bar) parse(cat): explanation */")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("bar")),
                    (category!("parse"), Some("cat"))
                ],
                reason: "explanation"
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * rome-ignore parse(yes) parse(frog): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("yes")),
                    (category!("parse"), Some("frog"))
                ],
                reason: "explanation"
            })],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * rome-ignore parse(wow) parse(fish): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![
                    (category!("parse"), Some("wow")),
                    (category!("parse"), Some("fish"))
                ],
                reason: "explanation"
            })],
        );
    }

    #[test]
    fn parse_multiple_suppression_categories() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Ok(Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation"
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
            parse_suppression_comment("// rome-ignore format explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingColon,
                span: TextRange::new(TextSize::from(22), TextSize::from(33))
            })],
        );
    }

    #[test]
    fn diagnostic_missing_paren() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore format(:").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingParen,
                span: TextRange::new(TextSize::from(22), TextSize::from(23))
            })],
        );
    }

    #[test]
    fn diagnostic_missing_category() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore (value): explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::MissingCategory,
                span: TextRange::new(TextSize::from(15), TextSize::from(16))
            })],
        );
    }

    #[test]
    fn diagnostic_unknown_category() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore unknown: explanation").collect::<Vec<_>>(),
            vec![Err(SuppressionDiagnostic {
                message: SuppressionDiagnosticKind::ParseCategory(String::from("unknown")),
                span: TextRange::new(TextSize::from(15), TextSize::from(22))
            })],
        );
    }
}
