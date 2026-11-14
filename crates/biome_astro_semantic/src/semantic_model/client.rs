use super::builder::{ModelBuilder, NewSymbol, synthetic_name, to_host};
use super::javascript::{
    collect_free_references, collect_module_bindings, identifier_semantic_name,
};
use super::model::*;
use super::snippets::AstroSnippet;
use biome_html_syntax::{AstroDefineDirective, HtmlElement, HtmlRoot};
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsRoot, JsSyntaxKind,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxKind, Text, TextRange, TokenText, WalkEvent,
};

pub(crate) fn collect_client_semantics(
    builder: &mut ModelBuilder,
    host: &HtmlRoot,
    snippets: &[AstroSnippet],
    ids: &[SnippetId],
    client_globals: &[Box<str>],
) {
    mark_unreadable_define_vars(builder, host, ids);

    let scripts: Vec<_> = ids
        .iter()
        .enumerate()
        .filter_map(|(index, id)| match builder.data.snippets[*id].host {
            SnippetHost::Script(block) if !builder.data.blocks[block].external => {
                Some((index, *id, block))
            }
            _ => None,
        })
        .collect();
    let define_vars: Vec<_> = ids
        .iter()
        .enumerate()
        .filter_map(|(index, id)| match builder.data.snippets[*id].host {
            SnippetHost::ScriptDefineVars(block) if !builder.data.blocks[block].external => {
                Some((index, *id, block))
            }
            _ => None,
        })
        .collect();
    let mut declarations: Vec<_> = scripts
        .iter()
        .map(|(index, id, block)| (*index, *id, *block, true))
        .chain(
            define_vars
                .iter()
                .map(|(index, id, block)| (*index, *id, *block, false)),
        )
        .collect();
    declarations.sort_by_key(|(_, id, _, _)| builder.data.snippets[*id].range.start());
    for (index, id, block, is_script) in declarations {
        let Some(scope) = builder.data.blocks[block].scope else {
            continue;
        };
        if is_script {
            collect_module_bindings(builder, &snippets[index], id, scope);
        } else {
            collect_define_vars(builder, &snippets[index], id, scope);
        }
    }

    let external_scopes: Vec<_> = builder
        .data
        .blocks
        .iter()
        .filter(|block| block.kind == BlockKind::Script && block.external)
        .filter_map(|block| block.scope)
        .collect();
    for scope in external_scopes {
        for namespace in [Namespace::Value, Namespace::Type, Namespace::Namespace] {
            builder.open(scope, namespace, OpenReason::ExternalSource);
        }
    }

    for (index, id, block) in scripts {
        let Some(scope) = builder.data.blocks[block].scope else {
            continue;
        };
        collect_free_references(builder, &snippets[index], id, scope, client_globals);
    }
}

fn mark_unreadable_define_vars(builder: &mut ModelBuilder, host: &HtmlRoot, ids: &[SnippetId]) {
    for event in host
        .syntax()
        .preorder_of_kinds(AstroDefineDirective::KIND_SET)
    {
        let (value, block) = if let WalkEvent::Enter(node) = event
            && let Some(directive) = AstroDefineDirective::cast_ref(&node)
            && let Ok(value) = directive.value()
            && let Some(name) = value.name().ok().and_then(|name| name.value_token().ok())
            && name.text_trimmed() == "vars"
            && let Some(element_range) = directive.syntax().ancestors().find_map(|node| {
                if let Some(element) = HtmlElement::cast_ref(&node) {
                    Some(element.range())
                } else {
                    biome_html_syntax::HtmlSelfClosingElement::cast_ref(&node)
                        .map(|element| element.range())
                }
            })
            && let Some(block) = builder
                .data
                .blocks
                .iter()
                .position(|block| {
                    block.kind == BlockKind::Script
                        && block.scope.is_some()
                        && block.range == element_range
                })
                .map(BlockId::new)
            && !builder.data.blocks[block].external
        {
            (value, block)
        } else {
            continue;
        };
        let initializer = value
            .initializer()
            .and_then(|initializer| initializer.value().ok());
        let expression_range = initializer
            .as_ref()
            .and_then(|value| value.as_html_attribute_single_text_expression())
            .and_then(|expression| expression.expression().ok())
            .and_then(|expression| expression.html_literal_token().ok())
            .map(|token| token.text_range());
        if expression_range.is_some_and(|range| {
            ids.iter().any(|id| {
                builder.data.snippets[*id].range == range
                    && matches!(
                        builder.data.snippets[*id].host,
                        SnippetHost::ScriptDefineVars(candidate) if candidate == block
                    )
            })
        }) {
            continue;
        }
        let Some(scope) = builder.data.blocks[block].scope else {
            continue;
        };
        let reason = match initializer {
            None => OpenReason::ParseError,
            Some(value) if value.as_html_attribute_single_text_expression().is_some() => {
                OpenReason::ParseError
            }
            Some(_) => OpenReason::NonLiteral,
        };
        builder.open(scope, Namespace::Value, reason);
    }
}

fn collect_define_vars(
    builder: &mut ModelBuilder,
    snippet: &AstroSnippet,
    snippet_id: SnippetId,
    scope: ScopeId,
) {
    let AnyJsRoot::JsExpressionTemplateRoot(root) = snippet.root else {
        builder.open(scope, Namespace::Value, OpenReason::ParseError);
        return;
    };
    let Some(expression) = root.expression() else {
        builder.open(scope, Namespace::Value, OpenReason::ParseError);
        return;
    };
    let expression = expression.omit_parentheses();
    let object = match expression {
        AnyJsExpression::JsObjectExpression(object) => object,
        AnyJsExpression::JsBogusExpression(_) => {
            builder.open(scope, Namespace::Value, OpenReason::ParseError);
            return;
        }
        _ => {
            builder.open(scope, Namespace::Value, OpenReason::NonLiteral);
            return;
        }
    };

    for member in object.members().iter() {
        let Ok(member) = member else {
            builder.open(scope, Namespace::Value, OpenReason::ParseError);
            continue;
        };
        match &member {
            AnyJsObjectMember::JsSpread(_) => {
                builder.open(scope, Namespace::Value, OpenReason::Spread);
                continue;
            }
            AnyJsObjectMember::JsBogusMember(_) => {
                builder.open(scope, Namespace::Value, OpenReason::ParseError);
                continue;
            }
            AnyJsObjectMember::JsMetavariable(_) => {
                builder.open(scope, Namespace::Value, OpenReason::NonLiteral);
                continue;
            }
            _ => {}
        }
        if member_has_parse_error(&member) {
            builder.open(scope, Namespace::Value, OpenReason::ParseError);
            continue;
        }
        let (name, semantic_name, range, declaration) = match static_member(&member) {
            Ok(Some(member)) => member,
            Ok(None) => {
                builder.open(scope, Namespace::Value, OpenReason::Computed);
                continue;
            }
            Err(reason) => {
                builder.open(scope, Namespace::Value, reason);
                continue;
            }
        };
        let range = to_host(range, snippet.content_offset);
        let declaration = to_host(declaration, snippet.content_offset);
        let existing = builder.data.scopes[scope]
            .symbols
            .iter()
            .copied()
            .find(|id| {
                let symbol = &builder.data.symbols[*id];
                symbol.origin == Origin::DefineVars
                    && symbol.semantic_name.text() == semantic_name.text()
            });
        if let Some(existing) = existing {
            let symbol = &mut builder.data.symbols[existing];
            symbol.name = name;
            symbol.semantic_name = semantic_name;
            symbol.range = range;
            symbol.declaration = declaration;
            symbol.snippet = Some(snippet_id);
        } else {
            builder.add_symbol(NewSymbol {
                name,
                semantic_name,
                range,
                declaration,
                snippet: Some(snippet_id),
                scope,
                namespace: Namespace::Value,
                kind: SymbolKind::Synthetic,
                origin: Origin::DefineVars,
                declares_value: true,
                declares_type: false,
                declares_namespace: false,
            });
        }
    }
}

fn member_has_parse_error(member: &AnyJsObjectMember) -> bool {
    member.syntax().kind().is_bogus()
        || member_required_field_is_missing(member)
        || member.syntax().descendants().any(|node| {
            node.kind().is_bogus()
                || AnyJsObjectMember::cast_ref(&node)
                    .is_some_and(|member| member_required_field_is_missing(&member))
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

fn static_member(
    member: &AnyJsObjectMember,
) -> Result<Option<(TokenText, Text, TextRange, TextRange)>, OpenReason> {
    if let AnyJsObjectMember::JsShorthandPropertyObjectMember(member) = member {
        let token = member
            .name()
            .and_then(|name| name.value_token())
            .map_err(|_| OpenReason::ParseError)?;
        let name = token.token_text_trimmed();
        let semantic_name = identifier_semantic_name(name.clone());
        return Ok(Some((
            name,
            semantic_name,
            token.text_trimmed_range(),
            member.syntax().text_trimmed_range(),
        )));
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
) -> Result<Option<(TokenText, Text, TextRange, TextRange)>, OpenReason> {
    let (text, token) = match &name {
        AnyJsObjectMemberName::JsComputedMemberName(computed) => {
            if let Ok(expression) = computed.expression()
                && let AnyJsExpression::JsTemplateExpression(template) =
                    expression.omit_parentheses()
                && template.is_constant()
                && template.elements().is_empty()
            {
                let start = template
                    .l_tick_token()
                    .map_err(|_| OpenReason::ParseError)?
                    .text_trimmed_range()
                    .end();
                return Ok(Some((
                    synthetic_name(""),
                    Text::new_static(""),
                    TextRange::empty(start),
                    declaration,
                )));
            }
            let Some(text) = name.name() else {
                return Ok(None);
            };
            let Some(token) = name.name_token() else {
                return Ok(None);
            };
            (text, token)
        }
        AnyJsObjectMemberName::JsLiteralMemberName(_) => (
            name.name().ok_or(OpenReason::ParseError)?,
            name.name_token().ok_or(OpenReason::ParseError)?,
        ),
        AnyJsObjectMemberName::JsMetavariable(_) => return Err(OpenReason::NonLiteral),
    };
    let range = text.source_range(token.text_range());
    let is_string = token.kind() == JsSyntaxKind::JS_STRING_LITERAL;
    let is_template = token.kind() == JsSyntaxKind::TEMPLATE_CHUNK;
    let (text, semantic_name) = if is_template {
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
    Ok(Some((text, semantic_name, range, declaration)))
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
