use crate::parser::HtmlParser;
use crate::syntax::ParsedSyntax;
use crate::syntax::ParsedSyntax::{Absent, Present};
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::Parser;

/// Parses a Glimmer path like `this.foo`, `@arg`, or `helper`
///
/// GlimmerPath =
///     segments: GlimmerPathSegmentList
///
/// GlimmerPathSegmentList = (GlimmerPathSegment ('.' GlimmerPathSegment)*)
///
/// GlimmerPathSegment =
///     value_token: 'ident'
pub(crate) fn parse_glimmer_path(p: &mut HtmlParser, context: HtmlLexContext) -> ParsedSyntax {
    if !p.at(IDENT) && !p.at(AT) && !p.at(HTML_LITERAL) {
        return Absent;
    }

    let m = p.start();

    // Parse optional @ prefix (for argument references like @arg)
    let has_at = p.at(AT);
    if has_at {
        p.bump_with_context(AT, context);
    }

    // Start the path segments list
    let list_m = p.start();

    // Parse first segment
    if p.at(IDENT) || p.at(HTML_LITERAL) {
        let segment_m = p.start();
        p.bump_any_with_context(context); // IDENT or HTML_LITERAL
        segment_m.complete(p, GLIMMER_PATH_SEGMENT);
    } else if has_at {
        // @ was present but no identifier follows
        p.error(p.err_builder("Expected identifier after '@'", p.cur_range()));
    }

    // Parse remaining segments separated by dots
    while p.at(DOT) {
        // Bump DOT - it will be a separator in the list
        p.bump_with_context(DOT, context);

        if p.at(IDENT) || p.at(HTML_LITERAL) {
            let segment_m = p.start();
            p.bump_any_with_context(context);
            segment_m.complete(p, GLIMMER_PATH_SEGMENT);
        }
    }

    list_m.complete(p, GLIMMER_PATH_SEGMENT_LIST);

    Present(m.complete(p, GLIMMER_PATH))
}

/// Parses a Glimmer argument list
///
/// GlimmerArgumentList = AnyGlimmerArgument*
///
/// Parses both:
/// - GlimmerPositionalArgument (values like `"string"` or `path`)
/// - GlimmerNamedArgument (name=value like `key=value`)
pub(crate) fn parse_glimmer_argument_list(
    p: &mut HtmlParser,
    context: HtmlLexContext,
) -> ParsedSyntax {
    let m = p.start();

    while !p.at(R_TRIPLE_CURLY) && !p.at(R_DOUBLE_CURLY) && !p.at(EOF) {
        if p.at(HTML_STRING_LITERAL) {
            // Positional argument with string literal
            let arg_m = p.start();
            let value_m = p.start();
            p.bump_with_context(HTML_STRING_LITERAL, context);
            value_m.complete(p, GLIMMER_STRING_LITERAL);
            arg_m.complete(p, GLIMMER_POSITIONAL_ARGUMENT);
        } else if p.at(IDENT) || p.at(HTML_LITERAL) {
            // Check if this is the 'as' keyword followed by block params
            // in patterns like "as |param|"
            let checkpoint = p.checkpoint();
            let is_as_keyword = p.cur_text() == "as";
            p.bump_with_context(p.cur(), context);

            if is_as_keyword && p.at(T![|]) {
                // This is block params, not an argument - rewind and stop
                p.rewind(checkpoint);
                break;
            }

            // Not block params, check if it's a named argument or positional
            // Check if next token is '='
            if p.at(T![=]) {
                // This is a named argument: rewind and parse properly
                p.rewind(checkpoint);

                let arg_m = p.start();
                // Parse name_token (just bump the identifier)
                p.bump_any_with_context(context);

                // Parse eq_token '='
                p.bump_with_context(T![=], context);

                // Parse value (can be string literal or path)
                if p.at(HTML_STRING_LITERAL) {
                    let value_m = p.start();
                    p.bump_with_context(HTML_STRING_LITERAL, context);
                    value_m.complete(p, GLIMMER_STRING_LITERAL);
                } else {
                    let _ = parse_glimmer_path(p, context);
                }

                arg_m.complete(p, GLIMMER_NAMED_ARGUMENT);
            } else {
                // This is a positional argument: rewind and parse as path
                p.rewind(checkpoint);

                let arg_m = p.start();
                let _ = parse_glimmer_path(p, context);
                arg_m.complete(p, GLIMMER_POSITIONAL_ARGUMENT);
            }
        } else {
            // Unknown token, stop parsing arguments
            break;
        }
    }

    Present(m.complete(p, GLIMMER_ARGUMENT_LIST))
}

/// Parse a Glimmer mustache expression (escaped output).
/// Grammar: GlimmerMustacheExpression =
///   l_curly2_token: '{{'
///   path: GlimmerPath
///   arguments: GlimmerArgumentList
///   r_curly2_token: '}}'
pub(crate) fn parse_glimmer_mustache_expression(p: &mut HtmlParser) -> ParsedSyntax {
    use crate::token_source::TextExpressionKind;

    if !p.at(T!["{{"]) {
        return Absent;
    }

    // Only parse as Glimmer if DoubleGlimmer context is enabled (i.e., in .gjs/.gts files)
    // For other files (Vue, etc.), return Absent to fall back to generic parsing
    if p.options().text_expression != Some(TextExpressionKind::DoubleGlimmer) {
        return Absent;
    }

    let m = p.start();

    // Use DoubleGlimmer context for proper Glimmer tokenization
    let context = HtmlLexContext::TextExpression(TextExpressionKind::DoubleGlimmer);

    // Bump opening {{ with Glimmer context
    p.bump_with_context(T!["{{"], context);

    // Parse the path (e.g., this.foo, helper, @arg)
    let _ = parse_glimmer_path(p, context);

    // Parse the argument list (can be empty)
    let _ = parse_glimmer_argument_list(p, context);

    // Bump closing }} and switch back to regular context
    if p.at(T!["}}"]) {
        p.bump_with_context(T!["}}"], HtmlLexContext::Regular);
    } else {
        p.error(p.err_builder("Expected closing }}", p.cur_range()));
    }

    Present(m.complete(p, GLIMMER_MUSTACHE_EXPRESSION))
}

/// Parse a Glimmer block helper.
/// Grammar: GlimmerBlockHelper =
///   opening: GlimmerBlockHelperOpening
///   children: HtmlElementList
///   closing: GlimmerBlockHelperClosing
pub(crate) fn parse_glimmer_block_helper(p: &mut HtmlParser) -> ParsedSyntax {
    use crate::token_source::TextExpressionKind;

    if !p.at(T!["{{"]) {
        return Absent;
    }

    // Only parse as Glimmer if DoubleGlimmer context is enabled
    if p.options().text_expression != Some(TextExpressionKind::DoubleGlimmer) {
        return Absent;
    }

    // Check if this is a block helper by looking ahead for '#'
    let checkpoint = p.checkpoint();
    let context = HtmlLexContext::TextExpression(TextExpressionKind::DoubleGlimmer);

    p.bump_with_context(T!["{{"], context);

    if !p.at(T![#]) {
        // Not a block helper, rewind
        p.rewind(checkpoint);
        return Absent;
    }

    // Rewind and start properly
    p.rewind(checkpoint);

    let m = p.start();

    // Parse opening tag
    let _ = parse_glimmer_block_helper_opening(p);

    // Parse children (HTML elements/text between opening and closing)
    // We need to manually control the parsing to stop at the closing tag
    let list_m = p.start();
    loop {
        // Check if we're at the closing tag pattern {{/
        if p.at(T!["{{"]) {
            let checkpoint = p.checkpoint();
            let ctx = HtmlLexContext::TextExpression(TextExpressionKind::DoubleGlimmer);
            p.bump_with_context(T!["{{"], ctx);

            if p.at(T![/]) {
                // This is the closing tag, rewind and stop
                p.rewind(checkpoint);
                break;
            } else {
                // Not a closing tag, rewind and let ElementList handle it
                p.rewind(checkpoint);
            }
        }

        // Parse one element
        let mut element_list = crate::syntax::ElementList::default();
        if element_list.parse_element(p).is_absent() {
            // Can't parse anything, stop
            break;
        }
    }
    list_m.complete(p, HTML_ELEMENT_LIST);

    // Parse closing tag
    let _ = parse_glimmer_block_helper_closing(p);

    Present(m.complete(p, GLIMMER_BLOCK_HELPER))
}

/// Parse the opening tag of a block helper.
/// Grammar: GlimmerBlockHelperOpening =
///   l_curly2_token: '{{'
///   hash_token: '#'
///   helper: GlimmerPath
///   arguments: GlimmerArgumentList
///   block_params: GlimmerBlockParams?
///   r_curly2_token: '}}'
fn parse_glimmer_block_helper_opening(p: &mut HtmlParser) -> ParsedSyntax {
    use crate::token_source::TextExpressionKind;

    if !p.at(T!["{{"]) {
        return Absent;
    }

    let m = p.start();
    let context = HtmlLexContext::TextExpression(TextExpressionKind::DoubleGlimmer);

    // Bump opening {{
    p.bump_with_context(T!["{{"], context);

    // Bump # token
    if p.at(T![#]) {
        p.bump_with_context(T![#], context);
    } else {
        p.error(p.err_builder("Expected '#' for block helper", p.cur_range()));
    }

    // Parse helper path (e.g., 'if', 'each', 'let')
    let _ = parse_glimmer_path(p, context);

    // Parse arguments
    let _ = parse_glimmer_argument_list(p, context);

    // Parse optional block params (as |param1 param2|)
    if p.at(IDENT) {
        // Check if this is 'as' keyword
        let checkpoint = p.checkpoint();
        p.bump_with_context(IDENT, context);

        if p.at(T![|]) {
            // This looks like block params, rewind and parse properly
            p.rewind(checkpoint);
            let _ = parse_glimmer_block_params(p, context);
        } else {
            // Not block params, rewind
            p.rewind(checkpoint);
        }
    }

    // Bump closing }}
    if p.at(T!["}}"]) {
        p.bump_with_context(T!["}}"], HtmlLexContext::Regular);
    } else {
        p.error(p.err_builder("Expected closing }}", p.cur_range()));
    }

    Present(m.complete(p, GLIMMER_BLOCK_HELPER_OPENING))
}

/// Parse the closing tag of a block helper.
/// Grammar: GlimmerBlockHelperClosing =
///   l_curly2_token: '{{'
///   slash_token: '/'
///   helper: GlimmerPath
///   r_curly2_token: '}}'
fn parse_glimmer_block_helper_closing(p: &mut HtmlParser) -> ParsedSyntax {
    use crate::token_source::TextExpressionKind;

    if !p.at(T!["{{"]) {
        return Absent;
    }

    let m = p.start();
    let context = HtmlLexContext::TextExpression(TextExpressionKind::DoubleGlimmer);

    // Bump opening {{
    p.bump_with_context(T!["{{"], context);

    // Bump / token
    if p.at(T![/]) {
        p.bump_with_context(T![/], context);
    } else {
        p.error(p.err_builder("Expected '/' for block helper closing", p.cur_range()));
    }

    // Parse helper path
    let _ = parse_glimmer_path(p, context);

    // Bump closing }}
    if p.at(T!["}}"]) {
        p.bump_with_context(T!["}}"], HtmlLexContext::Regular);
    } else {
        p.error(p.err_builder("Expected closing }}", p.cur_range()));
    }

    Present(m.complete(p, GLIMMER_BLOCK_HELPER_CLOSING))
}

/// Parse block parameters (as |param1 param2|).
/// Grammar: GlimmerBlockParams =
///   as_token: 'ident'
///   l_pipe_token: '|'
///   params: GlimmerBlockParamList
///   r_pipe_token: '|'
fn parse_glimmer_block_params(p: &mut HtmlParser, context: HtmlLexContext) -> ParsedSyntax {
    if !p.at(IDENT) || p.cur_text() != "as" {
        return Absent;
    }

    let m = p.start();

    // Bump 'as' keyword
    p.bump_with_context(IDENT, context);

    // Bump opening |
    if p.at(T![|]) {
        p.bump_with_context(T![|], context);
    } else {
        p.error(p.err_builder("Expected '|' after 'as' keyword", p.cur_range()));
    }

    // Parse parameter list
    let list_m = p.start();
    while p.at(IDENT) {
        let param_m = p.start();
        p.bump_with_context(IDENT, context);
        param_m.complete(p, GLIMMER_BLOCK_PARAM);
    }
    list_m.complete(p, GLIMMER_BLOCK_PARAM_LIST);

    // Bump closing |
    if p.at(T![|]) {
        p.bump_with_context(T![|], context);
    } else {
        p.error(p.err_builder("Expected closing '|'", p.cur_range()));
    }

    Present(m.complete(p, GLIMMER_BLOCK_PARAMS))
}
