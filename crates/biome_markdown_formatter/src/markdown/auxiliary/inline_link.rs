use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::write;
use biome_markdown_syntax::{AnyMdInline, MdInlineItemList, MdInlineLink, MdInlineLinkFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineLink;
impl FormatNodeRule<MdInlineLink> for FormatMdInlineLink {
    fn fmt_fields(&self, node: &MdInlineLink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineLinkFields {
            title,
            text,
            destination,
            r_brack_token,
            r_paren_token,
            l_brack_token,
            l_paren_token,
        } = node.as_fields();

        let mut text_items = text.iter();
        let single_textual = match text_items.next() {
            Some(AnyMdInline::MdTextual(textual)) if text_items.next().is_none() => Some(textual),
            _ => None,
        };

        let should_escape_text = if let Some(textual) = &single_textual {
            textual.value_token()?.text() == "*"
        } else {
            false
        };

        let text = format_with(|f| {
            if should_escape_text && let Some(textual) = &single_textual {
                textual
                    .format()
                    .with_options(FormatMdTextualOptions {
                        should_escape: true,
                        ..FormatMdTextualOptions::default()
                    })
                    .fmt(f)
            } else {
                text.format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::trim_all(),
                        keep_fences_in_italics: false,
                        text_context: TextContext::Neutral,
                    })
                    .fmt(f)
            }
        });

        write!(
            f,
            [
                l_brack_token.format(),
                text,
                r_brack_token.format(),
                l_paren_token.format(),
                format_inline_destination(&destination, TextPrintMode::trim_all())
            ]
        )?;
        if let Some(title) = title {
            write!(f, [title.format()])?;
        }

        write!(f, [r_paren_token.format()])
    }
}

/// Formats the address part of an inline link or image.
///
/// Most addresses can use the normal formatter. A narrow case needs special
/// handling: an address with parentheses and a raw `>` is printed as
/// `<...%3E...>`. The angle brackets keep the address together, and `%3E`
/// keeps the inner `>` from being mistaken for the closing bracket.
///
/// The fallback mode keeps the existing behavior for all other addresses. Links
/// and images use slightly different fallback trimming rules, so callers pass
/// the rule they already used before this special case existed.
pub(crate) fn format_inline_destination(
    destination: &MdInlineItemList,
    fallback_print_mode: TextPrintMode,
) -> impl Format<MarkdownFormatContext> + '_ {
    format_with(
        move |f: &mut MarkdownFormatter| match inline_destination_format(destination) {
            InlineDestinationFormat::WrapAndEncode => {
                write!(f, [token("<")])?;
                for item in destination.iter() {
                    match item {
                        AnyMdInline::MdTextual(textual) => {
                            let value_token = textual.value_token()?;
                            if value_token.text() == ">" {
                                f.context()
                                    .comments()
                                    .mark_suppression_checked(textual.syntax());
                                write!(f, [format_replaced(&value_token, &token("%3E"))])?;
                            } else {
                                write!(f, [textual.format()])?;
                            }
                        }
                        item => write!(f, [item.format()])?,
                    }
                }
                write!(f, [token(">")])
            }
            InlineDestinationFormat::PreserveWrapped => destination
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    print_mode: TextPrintMode::trim_all(),
                    keep_fences_in_italics: false,
                    text_context: TextContext::Neutral,
                })
                .fmt(f),
            InlineDestinationFormat::Fallback => destination
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    print_mode: fallback_print_mode,
                    keep_fences_in_italics: false,
                    text_context: TextContext::Neutral,
                })
                .fmt(f),
        },
    )
}

enum InlineDestinationFormat {
    /// The address has parentheses and a raw `>`. To keep it valid, wrap the
    /// whole address in `<...>` and write the inner `>` as `%3E`.
    WrapAndEncode,
    /// The address is already in the form we want. Keep the `<...>` wrapper
    /// instead of letting the normal image formatting remove it again.
    PreserveWrapped,
    /// The address does not need special handling. Format it the normal way.
    Fallback,
}

/// Chooses whether an address needs the special `<...%3E...>` form.
///
/// The special form is only used for plain text addresses that contain both a
/// parenthesis and a raw `>`. Other addresses are left alone because changing
/// them could alter user input that does not need fixing.
fn inline_destination_format(destination: &MdInlineItemList) -> InlineDestinationFormat {
    let items: Vec<_> = destination.iter().collect();
    if should_preserve_wrapped_encoded_destination(&items) {
        return InlineDestinationFormat::PreserveWrapped;
    }

    // Only handle plain address text. If the address contains spaces, `<`, or
    // another kind of inline content, the usual formatter is safer.
    let mut has_parenthesis = false;
    let mut has_greater_than = false;

    for item in items {
        let AnyMdInline::MdTextual(textual) = item else {
            return InlineDestinationFormat::Fallback;
        };
        let Ok(value_token) = textual.value_token() else {
            return InlineDestinationFormat::Fallback;
        };
        let text = value_token.text();
        if text.contains(['\n', '\r', ' ', '\t']) || text.contains('<') {
            return InlineDestinationFormat::Fallback;
        }
        if text.contains(['(', ')']) {
            has_parenthesis = true;
        }
        if text == ">" {
            has_greater_than = true;
        }
    }

    if has_parenthesis && has_greater_than {
        InlineDestinationFormat::WrapAndEncode
    } else {
        InlineDestinationFormat::Fallback
    }
}

/// Detects an address that is already printed as `<...%3E...>`.
///
/// After formatting once, the parser reads the outer `<` and `>` as part of the
/// address. This function recognizes that already-finished shape so a second
/// formatting pass keeps it unchanged instead of removing the wrapper.
fn should_preserve_wrapped_encoded_destination(items: &[AnyMdInline]) -> bool {
    let starts_with_lt = matches!(items.first(), Some(AnyMdInline::MdTextual(textual)) if textual.value_token().is_ok_and(|token| token.text() == "<"));
    let ends_with_gt = matches!(items.last(), Some(AnyMdInline::MdTextual(textual)) if textual.value_token().is_ok_and(|token| token.text() == ">"));
    if !starts_with_lt || !ends_with_gt || items.len() <= 2 {
        return false;
    }

    let mut has_parenthesis = false;
    let mut has_encoded_greater_than = false;

    for item in &items[1..items.len() - 1] {
        let AnyMdInline::MdTextual(textual) = item else {
            return false;
        };
        let Ok(value_token) = textual.value_token() else {
            return false;
        };
        let text = value_token.text();
        if text.contains(['\n', '\r', ' ', '\t', '<', '>']) {
            return false;
        }
        if text.contains(['(', ')']) {
            has_parenthesis = true;
        }
        if text.contains("%3E") {
            has_encoded_greater_than = true;
        }
    }

    has_parenthesis && has_encoded_greater_than
}
