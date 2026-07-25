//! Parsing for the `srcset` attribute of `<img>` and `<source>`.
//!
//! The formatter lays the candidates out one per line when they do not fit,
//! with the descriptors aligned, which means it has to know where each URL and
//! descriptor begins and ends.

use biome_rowan::{TextRange, TextSize};

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
    let mut candidates: Vec<SrcsetCandidate> = Vec::new();
    let mut kind = None;
    let mut position = 0;

    while position < bytes.len() {
        // Whitespace and commas between candidates carry no meaning. Skipping
        // commas here is also what drops an empty candidate such as `a.png,,`.
        if is_srcset_whitespace(bytes[position]) || bytes[position] == b',' {
            position += 1;
            continue;
        }

        let url_start = position;
        while position < bytes.len() && !is_srcset_whitespace(bytes[position]) {
            position += 1;
        }
        let mut url_end = position;

        // A URL that ends in commas ends its candidate too, and that candidate
        // has no descriptor.
        if bytes[url_end - 1] == b',' {
            while url_end > url_start && bytes[url_end - 1] == b',' {
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

        while position < bytes.len() && is_srcset_whitespace(bytes[position]) {
            position += 1;
        }
        let descriptor_start = position;
        while position < bytes.len() && bytes[position] != b',' {
            position += 1;
        }
        let mut descriptor_end = position;
        while descriptor_end > descriptor_start && is_srcset_whitespace(bytes[descriptor_end - 1]) {
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
        // image that can never be drawn.
        DescriptorKind::Width | DescriptorKind::Height => {
            let is_positive_integer = !number.is_empty()
                && number.bytes().all(|byte| byte.is_ascii_digit())
                && number.bytes().any(|byte| byte != b'0');
            is_positive_integer.then_some(kind)
        }
        // A density is any non-negative number, and `1.5x` is ordinary.
        DescriptorKind::Density => {
            let is_non_negative = number
                .parse::<f64>()
                .is_ok_and(|density| density.is_finite() && density >= 0.0);
            // `parse::<f64>` also accepts `inf`, `NaN` and a leading `+`,
            // none of which are numbers as far as HTML is concerned.
            let is_numeric = number
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'.');
            (is_non_negative && is_numeric).then_some(kind)
        }
    }
}

/// The whitespace that separates the parts of a `srcset`, per the HTML spec.
fn is_srcset_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0C')
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
