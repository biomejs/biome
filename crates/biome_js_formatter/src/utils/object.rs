use crate::context::{JsFormatOptions, QuoteProperties};
use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::utils::StringLiteralParentKind;
use crate::utils::string_utils::{
    SourceFileKind, can_remove_number_quotes_by_file_type, is_string_member_safe_to_unquote,
};
use biome_formatter::token::number::{NumberFormatOptions, format_trimmed_number};
use biome_formatter::{QuoteStyle, write};
use biome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsObjectMember, AnyJsObjectMemberName,
    AnyTsTypeMember, JsClassMemberList, JsLiteralMemberName, JsObjectMemberList, JsSyntaxKind,
    JsSyntaxToken, TsTypeMemberList, inner_string_text,
};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};
use biome_unicode_table::is_js_ident;
use std::borrow::Cow;
use unicode_width::UnicodeWidthStr;

declare_node_union! {
    pub(crate) AnyJsMemberName = AnyJsObjectMemberName | AnyJsClassMemberName
}

impl Format<JsFormatContext> for AnyJsMemberName {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            Self::AnyJsObjectMemberName(name) => name.format().fmt(f),
            Self::AnyJsClassMemberName(name) => name.format().fmt(f),
        }
    }
}

/// Returns whether any key of `list` cannot be written without quotes, which
/// forces every key of the object to be quoted under
/// `quoteProperties: consistent`.
pub(crate) fn object_member_list_requires_quotes(
    list: &JsObjectMemberList,
    file_source: SourceFileKind,
) -> bool {
    list.iter()
        .filter_map(|member| member.ok())
        .any(|member| object_member_requires_quotes(&member, file_source))
}

fn object_member_requires_quotes(member: &AnyJsObjectMember, file_source: SourceFileKind) -> bool {
    let name = match member {
        AnyJsObjectMember::JsGetterObjectMember(member) => member.name(),
        AnyJsObjectMember::JsMethodObjectMember(member) => member.name(),
        AnyJsObjectMember::JsPropertyObjectMember(member) => member.name(),
        AnyJsObjectMember::JsSetterObjectMember(member) => member.name(),
        _ => return false,
    };
    let Ok(AnyJsObjectMemberName::JsLiteralMemberName(literal)) = name else {
        return false;
    };
    literal_name_requires_quotes(&literal, file_source)
}

/// Returns whether any key of a class member list cannot be written without
/// quotes, which forces every key of the class to be quoted under
/// `quoteProperties: consistent`.
pub(crate) fn class_member_list_requires_quotes(
    list: &JsClassMemberList,
    file_source: SourceFileKind,
) -> bool {
    list.iter()
        .any(|member| class_member_requires_quotes(&member, file_source))
}

fn class_member_requires_quotes(member: &AnyJsClassMember, file_source: SourceFileKind) -> bool {
    let name = match member {
        AnyJsClassMember::JsGetterClassMember(member) => member.name(),
        AnyJsClassMember::JsMethodClassMember(member) => member.name(),
        AnyJsClassMember::JsPropertyClassMember(member) => member.name(),
        AnyJsClassMember::JsSetterClassMember(member) => member.name(),
        AnyJsClassMember::TsGetterSignatureClassMember(member) => member.name(),
        AnyJsClassMember::TsMethodSignatureClassMember(member) => member.name(),
        AnyJsClassMember::TsPropertySignatureClassMember(member) => member.name(),
        AnyJsClassMember::TsSetterSignatureClassMember(member) => member.name(),
        _ => return false,
    };
    let Ok(AnyJsClassMemberName::JsLiteralMemberName(literal)) = name else {
        return false;
    };
    literal_name_requires_quotes(&literal, file_source)
}

/// Returns whether any key of a TypeScript type member list cannot be written
/// without quotes, which forces every key to be quoted under
/// `quoteProperties: consistent`.
pub(crate) fn type_member_list_requires_quotes(
    list: &TsTypeMemberList,
    file_source: SourceFileKind,
) -> bool {
    list.iter()
        .any(|member| type_member_requires_quotes(&member, file_source))
}

fn type_member_requires_quotes(member: &AnyTsTypeMember, file_source: SourceFileKind) -> bool {
    let AnyTsTypeMember::TsPropertySignatureTypeMember(member) = member else {
        return false;
    };
    let Ok(AnyJsObjectMemberName::JsLiteralMemberName(literal)) = member.name() else {
        return false;
    };
    literal_name_requires_quotes(&literal, file_source)
}

/// Returns whether a literal member name is a string that cannot be written
/// without quotes.
fn literal_name_requires_quotes(
    literal: &JsLiteralMemberName,
    file_source: SourceFileKind,
) -> bool {
    let Ok(value) = literal.value() else {
        return false;
    };
    if value.kind() != JS_STRING_LITERAL {
        return false;
    }
    !is_string_member_safe_to_unquote(inner_string_text(&value).text(), file_source)
}

/// Returns whether `literal` is a member name of a member list whose keys all
/// have to be quoted because of `quoteProperties: consistent`.
pub(crate) fn literal_member_name_needs_quotes(
    literal: &JsLiteralMemberName,
    f: &JsFormatter,
) -> bool {
    if f.options().quote_properties() != QuoteProperties::Consistent {
        return false;
    }
    let Some(member) = literal.syntax().parent() else {
        return false;
    };
    // Prettier quotes TypeScript property signatures but not method, getter, or
    // setter signatures.
    if matches!(
        member.kind(),
        JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER
            | JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER
            | JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER
    ) {
        return false;
    }
    let Some(list) = member.parent() else {
        return false;
    };
    f.context().member_list_needs_quotes(&list)
}

/// Returns the number as Biome prints it for a member name, which may differ
/// from the source, such as `.1` becoming `0.1`.
fn printed_member_number(text: &str) -> Cow<'_, str> {
    format_trimmed_number(
        text,
        NumberFormatOptions::default().keep_one_trailing_decimal_zero(),
    )
}

/// Returns whether a non-string member name token should be quoted to keep all
/// keys of a member list quoted under `quoteProperties: consistent`.
fn token_needs_quotes(value: &JsSyntaxToken, options: &JsFormatOptions) -> bool {
    match value.kind() {
        JS_STRING_LITERAL => false,
        JsSyntaxKind::JS_NUMBER_LITERAL => can_remove_number_quotes_by_file_type(
            &printed_member_number(value.text_trimmed()),
            options.source_type().into(),
        ),
        _ => is_js_ident(value.text_trimmed()),
    }
}

fn write_quoted_text(
    text: &str,
    value: &JsSyntaxToken,
    quote_style: QuoteStyle,
    f: &mut JsFormatter,
) -> FormatResult<()> {
    let quote = quote_style.as_char();
    let quoted = std::format!("{quote}{text}{quote}");
    write!(
        f,
        [format_replaced(
            value,
            &syntax_token_cow_slice(
                Cow::Owned(quoted),
                value,
                value.text_trimmed_range().start(),
            ),
        )]
    )
}

/// Writes a literal member name token without its surrounding comments,
/// applying `quoteProperties`, and returns the printed width.
pub(crate) fn write_literal_member_name_value(
    value: &JsSyntaxToken,
    needs_quotes: bool,
    f: &mut JsFormatter,
) -> FormatResult<usize> {
    let options = f.options();

    match value.kind() {
        JS_STRING_LITERAL => {
            let quote_properties = if needs_quotes {
                QuoteProperties::Preserve
            } else {
                options.quote_properties()
            };
            let format = FormatLiteralStringToken::new(value, StringLiteralParentKind::Member);
            let cleaned = format.clean_text_with(options, quote_properties);
            let width = cleaned.width();
            cleaned.fmt(f)?;
            Ok(width)
        }
        JsSyntaxKind::JS_NUMBER_LITERAL if needs_quotes && token_needs_quotes(value, options) => {
            let printed = printed_member_number(value.text_trimmed());
            write_quoted_text(&printed, value, options.quote_style(), f)?;
            Ok(printed.width() + 2)
        }
        JsSyntaxKind::JS_NUMBER_LITERAL => {
            format_number_token(
                value,
                NumberFormatOptions::default().keep_one_trailing_decimal_zero(),
            )
            .fmt(f)?;
            Ok(printed_member_number(value.text_trimmed()).width())
        }
        _ if needs_quotes && token_needs_quotes(value, options) => {
            write_quoted_text(value.text_trimmed(), value, options.quote_style(), f)?;
            Ok(value.text_trimmed().width() + 2)
        }
        _ => {
            write!(f, [value.format()])?;
            Ok(value.text_trimmed().width())
        }
    }
}

pub(crate) fn write_member_name(
    name: &AnyJsMemberName,
    f: &mut JsFormatter,
) -> FormatResult<usize> {
    match name {
        name @ (AnyJsMemberName::AnyJsClassMemberName(
            AnyJsClassMemberName::JsLiteralMemberName(literal),
        )
        | AnyJsMemberName::AnyJsObjectMemberName(
            AnyJsObjectMemberName::JsLiteralMemberName(literal),
        )) => {
            let value = literal.value()?;
            let needs_quotes = literal_member_name_needs_quotes(literal, f);

            if value.kind() == JS_STRING_LITERAL {
                write!(f, [format_leading_comments(name.syntax())])?;
                let width = write_literal_member_name_value(&value, needs_quotes, f)?;
                write!(f, [format_trailing_comments(name.syntax())])?;

                Ok(width)
            } else {
                write!(f, [name])?;

                let printed = if value.kind() == JsSyntaxKind::JS_NUMBER_LITERAL {
                    printed_member_number(value.text_trimmed())
                } else {
                    Cow::Borrowed(value.text_trimmed())
                };

                if needs_quotes && token_needs_quotes(&value, f.options()) {
                    Ok(printed.width() + 2)
                } else {
                    Ok(printed.width())
                }
            }
        }
        name => {
            write!(f, [&name])?;
            Ok(name.to_trimmed_string().width())
        }
    }
}
