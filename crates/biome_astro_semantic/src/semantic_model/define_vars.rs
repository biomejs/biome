use super::builder::synthetic_name;
use super::javascript::identifier_semantic_name;
use super::model::{OpenReason, OpenReasons};
use super::snippets::AstroSnippet;
use biome_js_syntax::numbers::{canonicalize_js_bigint_literal, split_into_radix_and_number};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsRoot,
    JsSyntaxKind, JsUnaryOperator,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, RawSyntaxKind, SyntaxKind, SyntaxKindSet, Text,
    TextRange, TokenText, WalkEvent,
};

pub(crate) struct DefineVarCandidate {
    pub(crate) name: TokenText,
    pub(crate) semantic_name: Text,
    pub(crate) range: TextRange,
    pub(crate) declaration: TextRange,
}

#[derive(Default)]
pub(crate) struct DefineVarsResult {
    pub(crate) variables: Vec<DefineVarCandidate>,
    pub(crate) open: OpenReasons,
}

pub(crate) fn analyze_define_vars(snippet: &AstroSnippet) -> DefineVarsResult {
    let mut result = DefineVarsResult::default();
    let AnyJsRoot::JsExpressionTemplateRoot(root) = snippet.root else {
        result.open.insert(OpenReason::ParseError);
        return result;
    };
    let Some(expression) = root.expression() else {
        result.open.insert(OpenReason::ParseError);
        return result;
    };
    let expression = expression.omit_parentheses();
    let object = match expression {
        AnyJsExpression::JsObjectExpression(object) => object,
        AnyJsExpression::JsBogusExpression(_) => {
            result.open.insert(OpenReason::ParseError);
            return result;
        }
        _ => {
            result.open.insert(OpenReason::NonLiteral);
            return result;
        }
    };

    for member in object.members().iter() {
        let Ok(member) = member else {
            result.open.insert(OpenReason::ParseError);
            continue;
        };
        match &member {
            AnyJsObjectMember::JsSpread(_) => {
                result.open.insert(OpenReason::Spread);
                continue;
            }
            AnyJsObjectMember::JsBogusMember(_) => {
                result.open.insert(OpenReason::ParseError);
                continue;
            }
            AnyJsObjectMember::JsMetavariable(_) => {
                result.open.insert(OpenReason::NonLiteral);
                continue;
            }
            _ => {}
        }
        if member_has_parse_error(&member) {
            result.open.insert(OpenReason::ParseError);
            continue;
        }
        let variable = match static_member(&member) {
            Ok(Some(variable)) => variable,
            Ok(None) => {
                result.open.insert(OpenReason::Computed);
                continue;
            }
            Err(reason) => {
                result.open.insert(reason);
                continue;
            }
        };
        if is_proto_setter(&member) && variable.semantic_name.text() == "__proto__" {
            continue;
        }
        if let Some(existing) = result
            .variables
            .iter_mut()
            .find(|existing| existing.semantic_name == variable.semantic_name)
        {
            *existing = variable;
        } else {
            result.variables.push(variable);
        }
    }
    result
}

fn is_proto_setter(member: &AnyJsObjectMember) -> bool {
    let AnyJsObjectMember::JsPropertyObjectMember(member) = member else {
        return false;
    };
    matches!(
        member.name(),
        Ok(AnyJsObjectMemberName::JsLiteralMemberName(_))
    )
}

fn member_has_parse_error(member: &AnyJsObjectMember) -> bool {
    let bogus_kinds = [
        JsSyntaxKind::JS_BOGUS,
        JsSyntaxKind::JS_BOGUS_EXPRESSION,
        JsSyntaxKind::JS_BOGUS_STATEMENT,
        JsSyntaxKind::JS_BOGUS_MEMBER,
        JsSyntaxKind::JS_BOGUS_BINDING,
        JsSyntaxKind::JS_BOGUS_PARAMETER,
        JsSyntaxKind::JS_BOGUS_IMPORT_ASSERTION_ENTRY,
        JsSyntaxKind::JS_BOGUS_NAMED_IMPORT_SPECIFIER,
        JsSyntaxKind::JS_BOGUS_ASSIGNMENT,
        JsSyntaxKind::JS_BOGUS_VARIABLE_DECLARATION,
        JsSyntaxKind::TS_BOGUS_TYPE,
    ];
    let kinds = bogus_kinds
        .iter()
        .fold(AnyJsObjectMember::KIND_SET, |set, kind| {
            set.union(SyntaxKindSet::from_raw(RawSyntaxKind(*kind as u16)))
        });
    member.syntax().kind().is_bogus()
        || member_required_field_is_missing(member)
        || member
            .syntax()
            .preorder_of_kinds(kinds)
            .any(|event| match event {
                WalkEvent::Enter(node) => {
                    node.kind().is_bogus()
                        || AnyJsObjectMember::cast_ref(&node)
                            .is_some_and(|member| member_required_field_is_missing(&member))
                }
                WalkEvent::Leave(_) => false,
            })
}

fn member_required_field_is_missing(member: &AnyJsObjectMember) -> bool {
    match member {
        AnyJsObjectMember::JsBogusMember(_) => true,
        AnyJsObjectMember::JsGetterObjectMember(member) => {
            member.name().is_err() || member.body().is_err()
        }
        AnyJsObjectMember::JsMethodObjectMember(member) => {
            member.name().is_err() || member.body().is_err()
        }
        AnyJsObjectMember::JsPropertyObjectMember(member) => {
            member.name().is_err()
                || member.colon_token().is_err()
                || member.value().map_or(true, |value| {
                    matches!(
                        value.omit_parentheses(),
                        AnyJsExpression::JsBogusExpression(_)
                    )
                })
        }
        AnyJsObjectMember::JsSetterObjectMember(member) => {
            member.name().is_err() || member.body().is_err()
        }
        AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => member.name().is_err(),
        AnyJsObjectMember::JsMetavariable(_) | AnyJsObjectMember::JsSpread(_) => false,
    }
}

fn static_member(member: &AnyJsObjectMember) -> Result<Option<DefineVarCandidate>, OpenReason> {
    if let AnyJsObjectMember::JsShorthandPropertyObjectMember(member) = member {
        let token = member
            .name()
            .and_then(|name| name.value_token())
            .map_err(|_| OpenReason::ParseError)?;
        let name = token.token_text_trimmed();
        let semantic_name = identifier_semantic_name(name.clone());
        return Ok(Some(DefineVarCandidate {
            name,
            semantic_name,
            range: token.text_trimmed_range(),
            declaration: member.syntax().text_trimmed_range(),
        }));
    }

    let name = match member {
        AnyJsObjectMember::JsGetterObjectMember(member) => member.name(),
        AnyJsObjectMember::JsMethodObjectMember(member) => member.name(),
        AnyJsObjectMember::JsPropertyObjectMember(member) => member.name(),
        AnyJsObjectMember::JsSetterObjectMember(member) => member.name(),
        AnyJsObjectMember::JsBogusMember(_)
        | AnyJsObjectMember::JsMetavariable(_)
        | AnyJsObjectMember::JsShorthandPropertyObjectMember(_)
        | AnyJsObjectMember::JsSpread(_) => return Ok(None),
    }
    .map_err(|_| OpenReason::ParseError)?;
    object_member_name(name, member.syntax().text_trimmed_range())
}

fn object_member_name(
    name: AnyJsObjectMemberName,
    declaration: TextRange,
) -> Result<Option<DefineVarCandidate>, OpenReason> {
    let (text, token, is_computed) = match &name {
        AnyJsObjectMemberName::JsComputedMemberName(computed) => {
            if let Ok(expression) = computed.expression() {
                let expression = expression.omit_parentheses();
                if let Some(variable) = signed_numeric_member(&expression, declaration)? {
                    return Ok(Some(variable));
                }
                if let AnyJsExpression::JsTemplateExpression(template) = expression
                    && template.is_constant()
                    && template.elements().is_empty()
                {
                    let start = template
                        .l_tick_token()
                        .map_err(|_| OpenReason::ParseError)?
                        .text_trimmed_range()
                        .end();
                    return Ok(Some(DefineVarCandidate {
                        name: synthetic_name(""),
                        semantic_name: Text::new_static(""),
                        range: TextRange::empty(start),
                        declaration,
                    }));
                }
            }
            let Some(text) = name.name() else {
                return Ok(None);
            };
            let Some(token) = name.name_token() else {
                return Ok(None);
            };
            (text, token, true)
        }
        AnyJsObjectMemberName::JsLiteralMemberName(_) => (
            name.name().ok_or(OpenReason::ParseError)?,
            name.name_token().ok_or(OpenReason::ParseError)?,
            false,
        ),
        AnyJsObjectMemberName::JsMetavariable(_) => return Err(OpenReason::NonLiteral),
    };
    if is_computed
        && !matches!(
            token.kind(),
            JsSyntaxKind::JS_STRING_LITERAL
                | JsSyntaxKind::TEMPLATE_CHUNK
                | JsSyntaxKind::JS_NUMBER_LITERAL
                | JsSyntaxKind::JS_BIGINT_LITERAL
                | JsSyntaxKind::TRUE_KW
                | JsSyntaxKind::FALSE_KW
                | JsSyntaxKind::NULL_KW
        )
    {
        return Ok(None);
    }
    let range = text.source_range(token.text_range());
    let is_string = token.kind() == JsSyntaxKind::JS_STRING_LITERAL;
    let is_template = token.kind() == JsSyntaxKind::TEMPLATE_CHUNK;
    let (name, semantic_name) = if token.kind() == JsSyntaxKind::JS_NUMBER_LITERAL {
        let value = canonical_number_key(text.text()).ok_or(OpenReason::ParseError)?;
        (text, value.into())
    } else if token.kind() == JsSyntaxKind::JS_BIGINT_LITERAL {
        let value = canonicalize_js_bigint_literal(text.text())
            .and_then(|value| value.strip_suffix('n').map(str::to_string))
            .ok_or(OpenReason::ParseError)?;
        (text, value.into())
    } else if is_template {
        let normalized = normalize_raw_template_line_endings(text.text());
        let decoded = decode_static_key(&normalized).ok_or(OpenReason::ParseError)?;
        (text, decoded.into())
    } else if is_string && text.contains('\\') {
        let decoded = decode_static_key(text.text()).ok_or(OpenReason::ParseError)?;
        (text, decoded.into())
    } else if is_string {
        (text.clone(), text.into())
    } else {
        let semantic_name = identifier_semantic_name(text.clone());
        (text, semantic_name)
    };
    Ok(Some(DefineVarCandidate {
        name,
        semantic_name,
        range,
        declaration,
    }))
}

fn signed_numeric_member(
    expression: &AnyJsExpression,
    declaration: TextRange,
) -> Result<Option<DefineVarCandidate>, OpenReason> {
    let AnyJsExpression::JsUnaryExpression(unary) = expression else {
        return Ok(None);
    };
    let operator = unary.operator().map_err(|_| OpenReason::ParseError)?;
    if !matches!(operator, JsUnaryOperator::Plus | JsUnaryOperator::Minus) {
        return Ok(None);
    }
    let argument = unary.argument().map_err(|_| OpenReason::ParseError)?;
    let source = expression.syntax().text_trimmed().to_string();
    let semantic_name = match argument.omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(_),
        ) => canonical_number_key(&source),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsBigintLiteralExpression(_),
        ) if operator == JsUnaryOperator::Minus => canonicalize_js_bigint_literal(&source)
            .and_then(|value| value.strip_suffix('n').map(str::to_string)),
        _ => return Ok(None),
    }
    .ok_or(OpenReason::ParseError)?;
    Ok(Some(DefineVarCandidate {
        name: synthetic_name(&source),
        semantic_name: semantic_name.into(),
        range: expression.syntax().text_trimmed_range(),
        declaration,
    }))
}

fn canonical_number_key(text: &str) -> Option<String> {
    let (negative, text) = if let Some(text) = text.strip_prefix('-') {
        (true, text)
    } else {
        (false, text.strip_prefix('+').unwrap_or(text))
    };
    let (radix, digits) = split_into_radix_and_number(text);
    let mut value = if radix == 10 {
        digits.parse::<f64>().ok()?
    } else {
        parse_radix_integer(&digits, u32::from(radix))?
    };
    if negative {
        value = -value;
    }
    if value == 0.0 {
        return Some(String::from("0"));
    }
    if value.is_infinite() {
        return Some(if value.is_sign_negative() {
            String::from("-Infinity")
        } else {
            String::from("Infinity")
        });
    }
    let mut buffer = ryu_js::Buffer::new();
    Some(buffer.format(value).to_string())
}

fn parse_radix_integer(digits: &str, radix: u32) -> Option<f64> {
    let bits_per_digit = match radix {
        2 => 1,
        8 => 3,
        16 => 4,
        _ => return None,
    };
    let mut first = None;
    for (index, byte) in digits.bytes().enumerate() {
        let value = digit_value(byte)?;
        if value >= radix {
            return None;
        }
        if first.is_none() && value != 0 {
            first = Some((index, value));
        }
    }
    let Some((first_index, first_value)) = first else {
        return Some(0.0);
    };
    let first_bits = u32::BITS - first_value.leading_zeros();
    let bit_length =
        usize::try_from(first_bits).ok()? + (digits.len() - first_index - 1) * bits_per_digit;
    if bit_length <= 53 {
        let mut value = 0_u64;
        for byte in digits[first_index..].bytes() {
            value = value * u64::from(radix) + u64::from(digit_value(byte)?);
        }
        return Some(value as f64);
    }
    if bit_length > 1024 {
        return Some(f64::INFINITY);
    }

    let mut significand = 0_u64;
    let mut significant_bits = 0;
    let mut guard = false;
    let mut sticky = false;
    for (index, byte) in digits[first_index..].bytes().enumerate() {
        let value = digit_value(byte)?;
        let highest_bit = if index == 0 {
            first_bits - 1
        } else {
            bits_per_digit as u32 - 1
        };
        for bit in (0..=highest_bit).rev() {
            let value = (value >> bit) & 1;
            if significant_bits < 53 {
                significand = (significand << 1) | u64::from(value);
            } else if significant_bits == 53 {
                guard = value != 0;
            } else {
                sticky |= value != 0;
            }
            significant_bits += 1;
        }
    }

    let mut shift = i32::try_from(bit_length - 53).ok()?;
    if guard && (sticky || significand & 1 == 1) {
        significand += 1;
        if significand == 1 << 53 {
            significand >>= 1;
            shift += 1;
        }
    }
    Some((significand as f64) * 2_f64.powi(shift))
}

fn digit_value(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u32::from(byte - b'A') + 10),
        _ => None,
    }
}

fn decode_static_key(text: &str) -> Option<String> {
    let mut output = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    while let Some((_, character)) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        let (_, escaped) = chars.next()?;
        match escaped {
            'b' => output.push('\u{0008}'),
            'f' => output.push('\u{000c}'),
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            'v' => output.push('\u{000b}'),
            '0' => {
                if chars.peek().is_some_and(|(_, next)| next.is_ascii_digit()) {
                    return None;
                }
                output.push('\0');
            }
            '1'..='9' => return None,
            'x' => output.push(char::from_u32(read_hex(&mut chars, 2)?)?),
            'u' => {
                let code_point = read_unicode_escape(&mut chars)?;
                if (0xD800..=0xDBFF).contains(&code_point) {
                    if chars.next().map(|(_, value)| value) != Some('\\')
                        || chars.next().map(|(_, value)| value) != Some('u')
                    {
                        return None;
                    }
                    let low = read_unicode_escape(&mut chars)?;
                    if !(0xDC00..=0xDFFF).contains(&low) {
                        return None;
                    }
                    let scalar = 0x10000 + ((code_point - 0xD800) << 10) + (low - 0xDC00);
                    output.push(char::from_u32(scalar)?);
                } else if (0xDC00..=0xDFFF).contains(&code_point) {
                    return None;
                } else {
                    output.push(char::from_u32(code_point)?);
                }
            }
            '\r' => {
                if chars.peek().is_some_and(|(_, next)| *next == '\n') {
                    chars.next();
                }
            }
            '\n' | '\u{2028}' | '\u{2029}' => {}
            other => output.push(other),
        }
    }
    Some(output)
}

fn normalize_raw_template_line_endings(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            output.push('\n');
        } else {
            output.push(character);
        }
    }
    output
}

fn read_unicode_escape(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>) -> Option<u32> {
    if chars.peek().is_some_and(|(_, next)| *next == '{') {
        chars.next();
        read_braced_code_point(chars)
    } else {
        read_hex(chars, 4)
    }
}

fn read_hex(
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
    digits: usize,
) -> Option<u32> {
    let mut value = 0;
    for _ in 0..digits {
        let (_, digit) = chars.next()?;
        value = value * 16 + digit.to_digit(16)?;
    }
    Some(value)
}

fn read_braced_code_point(
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
) -> Option<u32> {
    let mut value = 0;
    let mut digits = 0;
    loop {
        let (_, character) = chars.next()?;
        if character == '}' {
            return (digits > 0 && value <= 0x10_FFFF).then_some(value);
        }
        value = value
            .checked_mul(16)?
            .checked_add(character.to_digit(16)?)?;
        digits += 1;
    }
}
