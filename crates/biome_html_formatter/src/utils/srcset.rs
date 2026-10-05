//! Parsing for the `srcset` attribute of `<img>` and `<source>`.
//!
//! The formatter lays the candidates out one per line when they do not fit,
//! with the descriptors aligned, which means it has to know where each URL and
//! descriptor begins and ends.

use crate::{HtmlFormatContext, HtmlFormatter};
use biome_formatter::{
    Buffer as _,
    prelude::{
        Format, FormatResult, format_with, if_group_breaks, if_group_fits_on_line,
        located_token_text, soft_line_break_or_space, token,
    },
    write,
};
use biome_html_syntax::HtmlSyntaxToken;
use biome_rowan::{TextRange, TextSize, TokenText};
use biome_unicode_table::{
    Dispatch::{COM, DIG, WHS, ZER},
    lookup_byte,
};

/// One candidate of a `srcset` attribute: where an image lives, and the
/// descriptor saying when a browser should pick it.
///
/// Both fields are byte ranges into the attribute value that was parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SrcsetCandidate {
    pub(crate) url: TextRange,
    pub(crate) descriptor: Option<TextRange>,
}

/// Which quantity a candidate's descriptor measures.
///
/// A `srcset` may only use one of them; a browser has no way to compare a
/// width against a pixel density.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DescriptorKind {
    /// `100w`, the intrinsic width in pixels.
    Width,
    /// `2x`, the pixel density.
    Density,
    /// `100h`, the intrinsic height. Long removed from the spec, but still
    /// parsed, and still mutually exclusive with the other two.
    Height,
}

/// Splits the value of a `srcset` attribute into its candidates.
///
/// Returns `None` when the value should be left exactly as the author wrote
/// it: when it holds no candidate at all, when a descriptor is malformed, or
/// when the descriptors are not all of one kind. Reformatting any of those
/// would be guessing at what was meant.
///
/// A URL runs to the next whitespace rather than to the next comma, as the
/// HTML spec says it does, so a candidate may point at a path that itself
/// contains commas.
///
/// See <https://html.spec.whatwg.org/multipage/images.html#parsing-a-srcset-attribute>.
pub(crate) fn parse_srcset(value: &str) -> Option<Vec<SrcsetCandidate>> {
    let bytes = value.as_bytes();
    let mut candidates = Vec::new();
    let mut kind = None;
    let mut position = 0;

    while position < bytes.len() {
        // Whitespace and commas between candidates carry no meaning. Skipping
        // commas here is also what drops an empty candidate such as `a.png,,`.
        if matches!(lookup_byte(bytes[position]), WHS | COM) {
            position += 1;
            continue;
        }

        let url_start = position;
        while position < bytes.len() && !matches!(lookup_byte(bytes[position]), WHS) {
            position += 1;
        }
        let mut url_end = position;

        // A URL that ends in commas ends its candidate too, and that candidate
        // has no descriptor.
        if matches!(lookup_byte(bytes[url_end - 1]), COM) {
            while url_end > url_start && matches!(lookup_byte(bytes[url_end - 1]), COM) {
                url_end -= 1;
            }
            candidates.push(SrcsetCandidate {
                url: TextRange::new(
                    TextSize::from(url_start as u32),
                    TextSize::from(url_end as u32),
                ),
                descriptor: None,
            });
            continue;
        }

        while position < bytes.len() && matches!(lookup_byte(bytes[position]), WHS) {
            position += 1;
        }
        let descriptor_start = position;
        while position < bytes.len() && !matches!(lookup_byte(bytes[position]), COM) {
            position += 1;
        }
        let mut descriptor_end = position;
        while descriptor_end > descriptor_start
            && matches!(lookup_byte(bytes[descriptor_end - 1]), WHS)
        {
            descriptor_end -= 1;
        }

        let descriptor = if descriptor_end > descriptor_start {
            let text = &value[descriptor_start..descriptor_end];
            let descriptor_kind = parse_descriptor_kind(text)?;
            if *kind.get_or_insert(descriptor_kind) != descriptor_kind {
                return None;
            }
            Some(TextRange::new(
                TextSize::from(descriptor_start as u32),
                TextSize::from(descriptor_end as u32),
            ))
        } else {
            None
        };

        candidates.push(SrcsetCandidate {
            url: TextRange::new(
                TextSize::from(url_start as u32),
                TextSize::from(url_end as u32),
            ),
            descriptor,
        });
    }

    (!candidates.is_empty()).then_some(candidates)
}

/// The number of bytes a descriptor spends before its decimal point.
///
/// Descriptors are aligned on that point rather than on their first character,
/// so that `0.5x` and `1111x` line up the way a reader expects.
pub(crate) fn descriptor_integer_len(descriptor: &str) -> usize {
    descriptor
        .find('.')
        .unwrap_or_else(|| descriptor.len().saturating_sub(1))
}

/// Classifies a descriptor, rejecting anything a browser would not accept.
fn parse_descriptor_kind(descriptor: &str) -> Option<DescriptorKind> {
    let (number, kind) = match descriptor.as_bytes().last()? {
        b'w' => (&descriptor[..descriptor.len() - 1], DescriptorKind::Width),
        b'x' => (&descriptor[..descriptor.len() - 1], DescriptorKind::Density),
        b'h' => (&descriptor[..descriptor.len() - 1], DescriptorKind::Height),
        _ => return None,
    };

    match kind {
        // A width or a height is a positive integer; zero would select an
        // image that can never be drawn. Requiring a digit other than zero
        // also rejects a descriptor that is nothing but its unit, since an
        // empty number holds no such digit.
        DescriptorKind::Width | DescriptorKind::Height => {
            let is_positive_integer = number
                .bytes()
                .all(|byte| matches!(lookup_byte(byte), ZER | DIG))
                && number.bytes().any(|byte| matches!(lookup_byte(byte), DIG));
            is_positive_integer.then_some(kind)
        }
        // A density is any non-negative number, and `1.5x` is ordinary.
        DescriptorKind::Density => is_valid_non_negative_float(number).then_some(kind),
    }
}

/// Check if `number` is a valid floating-point number according to the HTML spec, and is also non-negative.
///
/// See: <https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#floating-point-numbers>
fn is_valid_non_negative_float(number: &str) -> bool {
    let bytes = number.as_bytes();
    let mut position = 0;

    let integer_start = position;
    while position < bytes.len() && matches!(lookup_byte(bytes[position]), ZER | DIG) {
        position += 1;
    }
    let has_integer = position > integer_start;

    if matches!(bytes.get(position), Some(b'.')) {
        position += 1;
        let fraction_start = position;
        while position < bytes.len() && matches!(lookup_byte(bytes[position]), ZER | DIG) {
            position += 1;
        }
        if position == fraction_start {
            return false;
        }
    } else if !has_integer {
        return false;
    }

    if matches!(bytes.get(position), Some(b'e' | b'E')) {
        position += 1;
        if matches!(bytes.get(position), Some(b'+' | b'-')) {
            position += 1;
        }
        let exponent_start = position;
        while position < bytes.len() && matches!(lookup_byte(bytes[position]), ZER | DIG) {
            position += 1;
        }
        if position == exponent_start {
            return false;
        }
    }

    position == bytes.len()
}

/// Prints the candidates of a `srcset`, separated by `, ` while they fit on
/// one line and one per line once they do not.
///
/// When broken, each descriptor is pushed right so that every decimal point
/// lands in the same column, which is what turns a wall of URLs into a table
/// of sizes. The padding only applies to the broken form; laid out flat, a
/// single space separates a URL from its descriptor.
pub(crate) struct FormatSrcsetCandidates<'a> {
    pub(crate) candidates: &'a [SrcsetCandidate],
    pub(crate) content: &'a TokenText,
    pub(crate) value_token: &'a HtmlSyntaxToken,
}

impl Format<HtmlFormatContext> for FormatSrcsetCandidates<'_> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        let text = self.content.text();
        let widest_url = self
            .candidates
            .iter()
            .map(|candidate| text[candidate.url].len())
            .max()
            .unwrap_or(0);
        let widest_descriptor_integer = self
            .candidates
            .iter()
            .filter_map(|candidate| candidate.descriptor)
            .map(|descriptor| descriptor_integer_len(&text[descriptor]))
            .max()
            .unwrap_or(0);

        let separator = format_with(|f| write!(f, [token(","), soft_line_break_or_space()]));
        let mut candidates = f.join_with(separator);
        for candidate in self.candidates {
            candidates.entry(&format_with(|f| {
                let url = self.content.clone().slice(candidate.url);
                write!(
                    f,
                    [located_token_text(
                        self.value_token,
                        url.source_range(self.value_token.text_range())
                    )]
                )?;

                if let Some(descriptor) = candidate.descriptor {
                    // One space of separation, plus whatever it takes to line the
                    // decimal points up with the widest candidate.
                    let padding =
                        widest_url - text[candidate.url].len() + 1 + widest_descriptor_integer
                            - descriptor_integer_len(&text[descriptor]);
                    let descriptor = self.content.clone().slice(descriptor);
                    write!(
                        f,
                        [
                            if_group_breaks(&format_with(|f| {
                                for _ in 0..padding {
                                    write!(f, [token(" ")])?;
                                }
                                Ok(())
                            })),
                            if_group_fits_on_line(&token(" ")),
                            located_token_text(
                                self.value_token,
                                descriptor.source_range(self.value_token.text_range())
                            )
                        ]
                    )?;
                }

                Ok(())
            }));
        }

        candidates.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Renders the parse result as `url|descriptor` pairs, so the tests read
    /// as the candidates they describe.
    fn candidates(value: &str) -> Option<Vec<String>> {
        Some(
            parse_srcset(value)?
                .into_iter()
                .map(|candidate| {
                    let url = &value[candidate.url];
                    match candidate.descriptor {
                        Some(descriptor) => format!("{url}|{}", &value[descriptor]),
                        None => format!("{url}|"),
                    }
                })
                .collect(),
        )
    }

    #[test]
    fn parses_urls_with_and_without_descriptors() {
        assert_eq!(candidates("a.png"), Some(vec!["a.png|".into()]));
        assert_eq!(
            candidates("a.png 1x, b.png 2x"),
            Some(vec!["a.png|1x".into(), "b.png|2x".into()])
        );
        assert_eq!(
            candidates("a.png 400w,\tb.png   805w"),
            Some(vec!["a.png|400w".into(), "b.png|805w".into()])
        );
        assert_eq!(
            candidates("\n  a.png,\n  b.png\n"),
            Some(vec!["a.png|".into(), "b.png|".into()])
        );
    }

    #[test]
    fn parses_density_descriptors_with_exponents() {
        assert_eq!(
            candidates("a.png 1e2x, b.png 1E+2x, c.png .5e-1x"),
            Some(vec![
                "a.png|1e2x".into(),
                "b.png|1E+2x".into(),
                "c.png|.5e-1x".into()
            ])
        );
    }

    #[test]
    fn a_url_may_contain_commas() {
        // Regression shape from prettier's #8150: the comma belongs to the
        // path, and only whitespace ends the URL.
        assert_eq!(
            candidates("img_c_scale,w_200.jpg 200w, img_c_scale,w_379.jpg 379w"),
            Some(vec![
                "img_c_scale,w_200.jpg|200w".into(),
                "img_c_scale,w_379.jpg|379w".into()
            ])
        );
    }

    #[test]
    fn any_whitespace_byte_ends_a_url() {
        // HTML counts a form feed as whitespace but not a vertical tab. The
        // byte dispatch groups the two, so both end the URL, which leaves
        // `b.png` to be rejected as a descriptor and the value left alone.
        assert_eq!(candidates("a\x0Cb.png"), None);
        assert_eq!(candidates("a\x0Bb.png"), None);
    }

    #[test]
    fn drops_empty_candidates() {
        assert_eq!(
            candidates("a.png 1x,,  b.png 2x"),
            Some(vec!["a.png|1x".into(), "b.png|2x".into()])
        );
        assert_eq!(candidates("a.png,,"), Some(vec!["a.png|".into()]));
    }

    #[test]
    fn rejects_values_that_must_be_left_alone() {
        // Nothing to lay out.
        assert_eq!(candidates(""), None);
        assert_eq!(candidates("   "), None);
        // A browser cannot compare a density against a width.
        assert_eq!(candidates("a.png 1x, b.png 200w"), None);
        // Malformed descriptors.
        assert_eq!(candidates("a.png 1y"), None);
        assert_eq!(candidates("a.png 0w"), None);
        assert_eq!(candidates("a.png w"), None);
        assert_eq!(candidates("a.png 1.5w"), None);
        assert_eq!(candidates("a.png -1x"), None);
        assert_eq!(candidates("a.png 1x 2x"), None);
        assert_eq!(candidates("a.png 1.x"), None);
        assert_eq!(candidates("a.png +1e2x"), None);
        assert_eq!(candidates("a.png 1ex"), None);
        assert_eq!(candidates("a.png 1e+x"), None);
        assert_eq!(candidates("a.png 1e2e3x"), None);
    }

    #[test]
    fn a_missing_descriptor_does_not_conflict_with_the_others() {
        assert_eq!(
            candidates("a.png, b.png 2x"),
            Some(vec!["a.png|".into(), "b.png|2x".into()])
        );
    }

    #[test]
    fn descriptor_integer_len_measures_up_to_the_decimal_point() {
        assert_eq!(descriptor_integer_len("400w"), 3);
        assert_eq!(descriptor_integer_len("1610w"), 4);
        assert_eq!(descriptor_integer_len("2x"), 1);
        assert_eq!(descriptor_integer_len("0.5x"), 1);
        assert_eq!(descriptor_integer_len("3.3333x"), 1);
        assert_eq!(descriptor_integer_len("1111x"), 4);
    }
}
