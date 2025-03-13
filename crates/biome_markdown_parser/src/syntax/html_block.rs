use crate::parser::MarkdownParser;
use crate::syntax::SyntaxError;
use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::Parser;
use biome_parser::prelude::ParserProgress;
use biome_rowan::TextRange;

/// Parse an HTML block
///
/// An HTML block is a block-level HTML element that starts with an opening tag
/// and ends with a corresponding closing tag.
///
/// Example:
/// ```markdown
/// <div>
///   This is HTML content
/// </div>
/// ```
pub(crate) fn parse_html_block(p: &mut MarkdownParser) -> ParsedSyntax {
    // Check if we're at the start of an HTML tag
    if !p.at(MarkdownSyntaxKind::L_ANGLE) {
        return ParsedSyntax::None;
    }

    let m = p.start();

    // Consume the opening '<'
    p.bump();

    // Parse the tag name
    let tag_name = parse_tag_name(p);

    // Parse attributes
    parse_attributes(p);

    // Check for self-closing tag
    let is_self_closing = p.at(MarkdownSyntaxKind::SLASH);
    if is_self_closing {
        p.bump(); // Consume '/'
    }

    // Consume the closing '>'
    if p.at(MarkdownSyntaxKind::R_ANGLE) {
        p.bump();
    } else {
        // Error: missing closing angle bracket
        let error = SyntaxError::new("Expected '>' to close HTML tag", p.current_range());
        p.error(error);
    }

    // If not self-closing, parse content and closing tag
    if !is_self_closing && !tag_name.is_empty() {
        // Parse content until closing tag
        parse_html_content(p);

        // Parse closing tag
        if p.at(MarkdownSyntaxKind::L_ANGLE) {
            p.bump(); // Consume '<'

            if p.at(MarkdownSyntaxKind::SLASH) {
                p.bump(); // Consume '/'

                // Parse closing tag name
                let closing_tag_name = parse_tag_name(p);

                // Verify tag names match
                if closing_tag_name != tag_name {
                    let error = SyntaxError::new(
                        &format!(
                            "Expected closing tag '</{}>', found '</{}>'",
                            tag_name, closing_tag_name
                        ),
                        p.current_range(),
                    );
                    p.error(error);
                }

                // Consume the closing '>'
                if p.at(MarkdownSyntaxKind::R_ANGLE) {
                    p.bump();
                } else {
                    // Error: missing closing angle bracket
                    let error =
                        SyntaxError::new("Expected '>' to close HTML tag", p.current_range());
                    p.error(error);
                }
            }
        }
    }

    m.complete(p, MarkdownSyntaxKind::MD_HTML_BLOCK);
    ParsedSyntax::Parsed
}

/// Parse an HTML tag name
fn parse_tag_name(p: &mut MarkdownParser) -> String {
    let mut tag_name = String::new();

    // Tag names consist of alphanumeric characters, hyphens, and underscores
    while p.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
        let text = p.current_text();
        if text
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            tag_name.push_str(text);
            p.bump();
        } else {
            break;
        }
    }

    tag_name
}

/// Parse HTML attributes
fn parse_attributes(p: &mut MarkdownParser) {
    // Continue parsing attributes until we reach '>' or '/>'
    while !p.at(MarkdownSyntaxKind::R_ANGLE)
        && !p.at(MarkdownSyntaxKind::SLASH)
        && !p.at(MarkdownSyntaxKind::EOF)
    {
        // Skip whitespace
        if p.at(MarkdownSyntaxKind::WHITESPACE) {
            p.bump();
            continue;
        }

        // Parse attribute name
        if p.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
            let attr_m = p.start();

            // Consume attribute name
            while p.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
                let text = p.current_text();
                if text
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                {
                    p.bump();
                } else {
                    break;
                }
            }

            // Check for attribute value
            if p.at(MarkdownSyntaxKind::EQ) {
                p.bump(); // Consume '='

                // Parse attribute value
                if p.at(MarkdownSyntaxKind::MD_STRING_LITERAL) {
                    p.bump(); // Consume string literal
                } else if p.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
                    // Unquoted attribute value
                    while p.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
                        let text = p.current_text();
                        if text
                            .chars()
                            .all(|c| !c.is_whitespace() && c != '>' && c != '/')
                        {
                            p.bump();
                        } else {
                            break;
                        }
                    }
                }
            }

            attr_m.complete(p, MarkdownSyntaxKind::MD_STRING);
        } else {
            // If we don't recognize the token, just bump it
            p.bump();
        }
    }
}

/// Parse HTML content
fn parse_html_content(p: &mut MarkdownParser) {
    // Continue parsing until we reach the closing tag or EOF
    while !p.at(MarkdownSyntaxKind::EOF) {
        if p.at(MarkdownSyntaxKind::L_ANGLE) && p.nth(1) == MarkdownSyntaxKind::SLASH {
            // Found closing tag
            break;
        }

        // Parse nested HTML blocks
        if p.at(MarkdownSyntaxKind::L_ANGLE) && p.nth(1) != MarkdownSyntaxKind::SLASH {
            parse_html_block(p);
            continue;
        }

        // Parse any markdown content inside the HTML block
        if p.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
            let m = p.start();
            p.bump();
            m.complete(p, MarkdownSyntaxKind::MD_TEXTUAL);
            continue;
        }

        // If we don't recognize the token, just bump it
        p.bump();
    }
}
