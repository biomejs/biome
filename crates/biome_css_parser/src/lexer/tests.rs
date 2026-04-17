#![cfg(test)]
#![expect(unused_mut, unused_variables)]

use crate::lexer::CssLexContext;
use crate::CssParserOptions;
use biome_css_syntax::CssFileSource;
use biome_css_syntax::{
    CssSyntaxKind::{self, EOF},
    T, TextRange,
};
use biome_parser::lexer::{Lexer, LexerWithCheckpoint};
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use super::{
    source_cursor::SourceCursor,
    scan_cursor::{CssScanCursor, StringBodyScanStop, UrlBodyStartScan},
    CssLexer, TextSize,
};

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let options = CssParserOptions::default().allow_wrong_line_comments().allow_css_modules();
        let mut lexer = CssLexer::from_str($src).with_options(options);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(CssLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                biome_css_syntax::CssSyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].0,
            );

            assert_eq!(
                tokens[idx].1.len(),
                TextSize::from($len),
                "expected token length of {}, but found {:?} for token {:?}",
                $len,
                tokens[idx].1.len(),
                tokens[idx].0,
            );

            new_str.push_str(&$src[tokens[idx].1]);
            tok_idx += tokens[idx].1.len();

            idx += 1;
        )*

        if idx < tokens.len() {
            panic!(
                "expected {} tokens but lexer returned {}, first unexpected token is '{:?}'",
                idx,
                tokens.len(),
                tokens[idx].0
            );
        } else {
            assert_eq!(idx, tokens.len());
        }

        assert_eq!($src, new_str, "Failed to reconstruct input");
    }};
}

// This is for testing if the lexer is truly lossless
// It parses random strings and puts them back together with the produced tokens and compares
#[quickcheck]
fn losslessness(string: String) -> bool {
    // using an mpsc channel allows us to spawn a thread and spawn the lexer there, then if
    // it takes more than 2 seconds we panic because it is 100% infinite recursion
    let cloned = string.clone();
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut lexer = CssLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(CssLexContext::default()) != EOF {
            tokens.push(lexer.current_range());
        }

        sender
            .send(tokens)
            .expect("Could not send tokens to receiver");
    });
    let token_ranges = receiver
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or_else(|_| panic!("Lexer is infinitely recursing with this code: ->{string}<-"));

    let mut new_str = String::with_capacity(string.len());
    let mut idx = TextSize::from(0);

    for range in token_ranges {
        new_str.push_str(&string[range]);
        idx += range.len();
    }

    string == new_str
}

#[test]
fn empty() {
    assert_lex! {
        "",
    }
}

#[test]
fn rewind_restores_lexer_source_cursor_position() {
    let mut lexer = CssLexer::from_str("url(foo)");

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.position(), 3);
    assert_eq!(lexer.current_byte(), Some(b'('));

    let checkpoint = lexer.checkpoint();

    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);
    assert_eq!(lexer.position(), 4);
    assert_eq!(lexer.current_byte(), Some(b'f'));

    lexer.rewind(checkpoint);

    assert_eq!(lexer.position(), 3);
    assert_eq!(lexer.current_byte(), Some(b'('));
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(3), TextSize::from(4))
    );
}

#[test]
fn url_body_context_emits_raw_url_value() {
    let mut lexer = CssLexer::from_str("url(foo#{$x}.css)");

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        CssSyntaxKind::CSS_URL_VALUE_RAW_LITERAL
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(4), TextSize::from(16))
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T![')']);
}

#[test]
fn url_body_context_falls_back_for_interpolated_function() {
    let mut lexer = CssLexer::from_str("url(#{name}(bar))");

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        T![#]
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(4), TextSize::from(5))
    );

    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['{']);
    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::IDENT
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['}']);
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);
    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::IDENT
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T![')']);
    assert_eq!(lexer.next_token(CssLexContext::Regular), T![')']);
}

#[test]
fn url_body_context_reuses_pending_raw_scan_after_leading_trivia() {
    let mut lexer = CssLexer::from_str("url( foo)");

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: false,
        }),
        CssSyntaxKind::WHITESPACE
    );

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: false,
        }),
        CssSyntaxKind::CSS_URL_VALUE_RAW_LITERAL
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(5), TextSize::from(8))
    );

    assert_eq!(lexer.next_token(CssLexContext::Regular), T![')']);
}

#[test]
fn url_body_context_drops_stale_pending_raw_scan_after_regular_fallback() {
    let mut lexer = CssLexer::from_str("url( foo)");

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: false,
        }),
        CssSyntaxKind::WHITESPACE
    );

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::IDENT
    );

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: false,
        }),
        T![')']
    );
}

#[test]
fn url_body_context_skips_scss_line_comment_trivia_before_interpolated_function() {
    let mut lexer =
        CssLexer::from_str("url(// comment\n#{name}(bar))").with_source_type(CssFileSource::scss());

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        CssSyntaxKind::COMMENT
    );
    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        CssSyntaxKind::NEWLINE
    );
    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        T![#]
    );
}

#[test]
fn url_body_context_preserves_protocol_relative_url_in_scss() {
    let mut lexer =
        CssLexer::from_str("url(//cdn.example.com/app.css)").with_source_type(CssFileSource::scss());

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        CssSyntaxKind::CSS_URL_VALUE_RAW_LITERAL
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(4), TextSize::from(29))
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T![')']);
}

#[test]
fn url_body_context_handles_escaped_non_ascii_in_raw_url() {
    let mut lexer = CssLexer::from_str("url(foo\\ébar)").with_source_type(CssFileSource::scss());

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::URL_KW
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['(']);

    assert_eq!(
        lexer.next_token(CssLexContext::UrlBody {
            scss_exclusive_syntax_allowed: true,
        }),
        CssSyntaxKind::CSS_URL_VALUE_RAW_LITERAL
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(4), TextSize::from(13))
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T![')']);
}

#[test]
fn css_scan_cursor_reports_interpolated_function_shape() {
    let cursor = CssScanCursor::new(SourceCursor::new("foo#{1 + 1}(bar)", 0), true, true);

    assert!(cursor.is_at_scss_interpolated_function());
}

#[test]
fn css_scan_cursor_respects_line_comment_config_in_url_body_scanning() {
    let source = "// comment\nfoo)";

    let scss_cursor = CssScanCursor::new(SourceCursor::new(source, 0), true, true);
    let css_cursor = CssScanCursor::new(SourceCursor::new(source, 0), false, false);

    let scss_scan = scss_cursor.scan_url_body_start(false);
    let css_scan = css_cursor.scan_url_body_start(false);

    assert!(matches!(
        scss_scan,
        UrlBodyStartScan::RawValue(scan) if scan.start == 11
    ));
    assert!(matches!(
        css_scan,
        UrlBodyStartScan::RawValue(scan) if scan.start == 0
    ));
}

#[test]
fn css_scan_cursor_scans_plain_string_without_interpolation_mode() {
    let cursor = CssScanCursor::new(SourceCursor::new("\"a#{b}\"", 1), true, true);
    let scan = cursor.scan_plain_string_body(super::CssStringQuote::Double);

    assert!(matches!(
        scan.stop,
        StringBodyScanStop::ClosingQuote { .. }
    ));
}

#[test]
fn css_scan_cursor_treats_form_feed_as_string_newline() {
    let cursor = CssScanCursor::new(SourceCursor::new("\"a\u{000C}b\"", 1), true, true);
    let scan = cursor.scan_plain_string_body(super::CssStringQuote::Double);

    assert_eq!(
        scan.stop,
        StringBodyScanStop::Newline {
            position: 2,
            len: 1,
        }
    );
    assert!(scan.invalid_escape_ranges.is_empty());
}

#[test]
fn css_scan_cursor_treats_backslash_form_feed_as_line_continuation() {
    let cursor = CssScanCursor::new(SourceCursor::new("\"a\\\u{000C}b\"", 1), true, true);
    let scan = cursor.scan_plain_string_body(super::CssStringQuote::Double);

    assert!(matches!(
        scan.stop,
        StringBodyScanStop::ClosingQuote { position: 5 }
    ));
    assert!(scan.invalid_escape_ranges.is_empty());
}

#[test]
fn css_scan_cursor_identifier_mode_controls_slash_consumption() {
    let mut cursor = CssScanCursor::new(SourceCursor::new("/foo", 0), true, true);
    let mut buf = [0u8; 8];

    let scan = cursor.consume_ident_sequence(&mut buf);
    assert_eq!(scan.count, 0);
    assert_eq!(scan.stop_byte, Some(b'/'));
    assert_eq!(cursor.position(), 0);

    let mut cursor = CssScanCursor::new(SourceCursor::new("/foo", 0), true, true);
    let mut buf = [0u8; 8];

    let scan = cursor.consume_ident_sequence_with_slash(&mut buf);
    assert_eq!(scan.count, 4);
    assert_eq!(&buf[..scan.count], b"/foo");
    assert_eq!(cursor.position(), 4);
}

#[test]
fn css_scan_cursor_does_not_treat_backslash_form_feed_as_identifier_escape() {
    let mut cursor = CssScanCursor::new(SourceCursor::new("\\\u{000C}foo", 0), true, true);
    let mut buf = [0u8; 8];

    assert!(!cursor.is_ident_start());
    let scan = cursor.consume_ident_sequence(&mut buf);
    assert_eq!(scan.count, 0);
    assert_eq!(scan.stop_byte, Some(b'\\'));
    assert_eq!(cursor.position(), 0);
}

#[test]
fn css_scan_cursor_consumes_identifier_sequence_with_escape() {
    let mut cursor = CssScanCursor::new(SourceCursor::new(r#"\66 oo-bar"#, 0), true, true);
    let mut buf = [0u8; 16];

    let scan = cursor.consume_ident_sequence(&mut buf);

    assert!(scan.count > 0);
    assert!(scan.only_ascii_used);
    assert_eq!(&buf[..scan.count], b"foo-bar");
    assert_eq!(scan.position, 10);
    assert_eq!(scan.stop_byte, None);
    assert_eq!(cursor.position(), 10);
}

#[test]
fn css_scan_cursor_consumes_identifier_sequence_with_slash_mode() {
    let mut cursor = CssScanCursor::new(SourceCursor::new("w/2", 0), true, true);
    let mut buf = [0u8; 16];

    let scan = cursor.consume_ident_sequence_with_slash(&mut buf);

    assert_eq!(&buf[..scan.count], b"w/2");
    assert!(scan.only_ascii_used);
    assert_eq!(scan.position, 3);
    assert_eq!(scan.stop_byte, None);
    assert_eq!(cursor.position(), 3);
}

#[test]
fn css_scan_cursor_consumes_identifier_sequence_tracks_non_ascii_transition() {
    let mut cursor = CssScanCursor::new(SourceCursor::new("abécd ", 0), true, true);
    let mut buf = [0u8; 16];

    let scan = cursor.consume_ident_sequence(&mut buf);

    assert_eq!(&buf[..scan.count], b"ab");
    assert!(!scan.only_ascii_used);
    assert_eq!(scan.position, 6);
    assert_eq!(scan.stop_byte, Some(b' '));
    assert_eq!(cursor.position(), 6);
}

#[test]
fn css_lexer_consumes_identifier_sequence_with_escape() {
    let mut lexer = CssLexer::from_str(r#"\66 oo-bar"#);
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) =
        lexer.consume_ident_sequence(&mut buf, false);

    assert!(count > 0);
    assert!(only_ascii_used);
    assert_eq!(&buf[..count], b"foo-bar");
    assert_eq!(lexer.position(), 10);
    assert_eq!(lexer.current_byte(), None);
}

#[test]
fn css_lexer_consumes_identifier_sequence_with_slash_mode() {
    let mut lexer = CssLexer::from_str("w/2");
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) =
        lexer.consume_ident_sequence(&mut buf, true);

    assert_eq!(&buf[..count], b"w/2");
    assert!(only_ascii_used);
    assert_eq!(lexer.position(), 3);
    assert_eq!(lexer.current_byte(), None);
}

#[test]
fn css_lexer_consumes_identifier_sequence_stops_before_slash_when_disabled() {
    let mut lexer = CssLexer::from_str("w/2");
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) =
        lexer.consume_ident_sequence(&mut buf, false);

    assert_eq!(&buf[..count], b"w");
    assert!(only_ascii_used);
    assert_eq!(lexer.position(), 1);
    assert_eq!(lexer.current_byte(), Some(b'/'));
}

#[test]
fn css_lexer_consumes_identifier_sequence_tracks_non_ascii_transition() {
    let mut lexer = CssLexer::from_str("abécd ");
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) =
        lexer.consume_ident_sequence(&mut buf, false);

    assert_eq!(&buf[..count], b"ab");
    assert!(!only_ascii_used);
    assert_eq!(lexer.position(), 6);
    assert_eq!(lexer.current_byte(), Some(b' '));
}

#[test]
fn css_lexer_tailwind_identifier_sequence_stops_before_hyphen_star_suffix() {
    let mut lexer = CssLexer::from_str("--color-*")
        .with_options(CssParserOptions::default().allow_tailwind_directives());
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) = lexer.consume_ident_sequence(&mut buf, false);

    assert_eq!(&buf[..count], b"--color");
    assert!(only_ascii_used);
    assert_eq!(lexer.position(), 7);
    assert_eq!(lexer.current_byte(), Some(b'-'));
}

#[test]
fn css_lexer_tailwind_identifier_sequence_keeps_double_hyphen_before_star() {
    let mut lexer = CssLexer::from_str("--*")
        .with_options(CssParserOptions::default().allow_tailwind_directives());
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) = lexer.consume_ident_sequence(&mut buf, false);

    assert_eq!(&buf[..count], b"--");
    assert!(only_ascii_used);
    assert_eq!(lexer.position(), 2);
    assert_eq!(lexer.current_byte(), Some(b'*'));
}

#[test]
fn css_lexer_tailwind_identifier_sequence_does_not_rewind_into_escaped_hyphen_before_star() {
    let mut lexer = CssLexer::from_str(r"--foo\2d*")
        .with_options(CssParserOptions::default().allow_tailwind_directives());
    let mut buf = [0u8; 16];

    let (count, only_ascii_used) = lexer.consume_ident_sequence(&mut buf, false);

    assert_eq!(&buf[..count], b"--foo-");
    assert!(only_ascii_used);
    assert_eq!(lexer.position(), 8);
    assert_eq!(lexer.current_byte(), Some(b'*'));
}

#[test]
fn css_lexer_tailwind_identifier_sequence_keeps_count_when_buffer_fills_before_suffix_hyphen() {
    let mut lexer = CssLexer::from_str("ab-*")
        .with_options(CssParserOptions::default().allow_tailwind_directives());
    let mut buf = [0u8; 2];

    let (count, only_ascii_used) = lexer.consume_ident_sequence(&mut buf, false);

    assert_eq!(&buf[..count], b"ab");
    assert!(only_ascii_used);
    assert_eq!(count, 2);
    assert_eq!(lexer.position(), 2);
    assert_eq!(lexer.current_byte(), Some(b'-'));
}

#[test]
fn lexer_scan_cursor_at_detects_interpolated_function_from_offset() {
    let lexer =
        CssLexer::from_str("xxfoo#{1 + 1}(bar)").with_source_type(CssFileSource::scss());

    assert!(lexer.scan_cursor_at(2).is_at_scss_interpolated_function());
}

#[test]
fn lexer_helper_handles_escaped_quote_inside_string() {
    let lexer = CssLexer::from_str(r#"foo#{"a\"b"}(bar)"#).with_source_type(CssFileSource::scss());

    assert!(lexer.is_at_scss_interpolated_function(0));
}

#[test]
fn lexer_helper_handles_non_ascii_escape_inside_string() {
    let lexer = CssLexer::from_str(r#"foo#{"a\é"}(bar)"#).with_source_type(CssFileSource::scss());

    assert!(lexer.is_at_scss_interpolated_function(0));
}

#[test]
fn lexer_helper_ignores_line_comment_inside_interpolation_body() {
    let lexer = CssLexer::from_str("foo#{// comment with } and \"quote\"\nname}(bar)")
        .with_source_type(CssFileSource::scss());

    assert!(lexer.is_at_scss_interpolated_function(0));
}

#[test]
fn same_quote_nested_string_in_interpolation_reuses_cached_first_chunk_scan() {
    let mut lexer =
        CssLexer::from_str("\"a#{\"b#{c}d\"}e\"").with_source_type(CssFileSource::scss());

    assert_eq!(
        lexer.next_token(CssLexContext::Regular),
        CssSyntaxKind::SCSS_STRING_QUOTE
    );
    assert!(lexer.has_pending_scss_string_start());

    assert_eq!(
        lexer.next_token(CssLexContext::ScssString(super::CssStringQuote::Double)),
        CssSyntaxKind::SCSS_STRING_CONTENT_LITERAL
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(1), TextSize::from(2))
    );
    assert!(!lexer.has_pending_scss_string_start());

    assert_eq!(
        lexer.next_token(CssLexContext::ScssString(super::CssStringQuote::Double)),
        T![#]
    );
    assert_eq!(lexer.next_token(CssLexContext::Regular), T!['{']);

    assert_eq!(
        lexer.next_token(CssLexContext::ScssStringInterpolation(
            super::CssStringQuote::Double,
        )),
        CssSyntaxKind::SCSS_STRING_QUOTE
    );
    assert!(lexer.has_pending_scss_string_start());

    assert_eq!(
        lexer.next_token(CssLexContext::ScssString(super::CssStringQuote::Double)),
        CssSyntaxKind::SCSS_STRING_CONTENT_LITERAL
    );
    assert_eq!(
        lexer.current_range(),
        TextRange::new(TextSize::from(5), TextSize::from(6))
    );
    assert!(!lexer.has_pending_scss_string_start());

    assert_eq!(
        lexer.next_token(CssLexContext::ScssString(super::CssStringQuote::Double)),
        T![#]
    );
}

#[test]
fn css_scan_cursor_consumes_identifier_escape_as_a_single_part() {
    let mut cursor = CssScanCursor::new(SourceCursor::new(r"\61 bc", 0), false, false);
    let mut buf = [0u8; 8];
    let scan = cursor.consume_ident_sequence(&mut buf);

    assert_eq!(scan.count, 3);
    assert_eq!(&buf[..scan.count], b"abc");
    assert_eq!(cursor.position(), 6);
}

#[test]
fn css_scan_cursor_scans_raw_url_value_until_closing_paren() {
    let cursor = CssScanCursor::new(SourceCursor::new(r"foo\)bar)", 0), false, false);
    let scan = cursor
        .scan_url_raw_value()
        .expect("expected raw url scan");

    assert_eq!(scan.start, 0);
    assert_eq!(scan.end, 8);
    assert!(scan.terminated);
}

#[test]
fn string() {
    assert_lex! {
        "'5098382'",
        CSS_STRING_LITERAL:9
    }

    // double quote
    assert_lex! {
        r#"'hel"lo"'"#,
        CSS_STRING_LITERAL:9
    }

    // escaped quote
    assert_lex! {
        r"'hel\'lo\''",
        CSS_STRING_LITERAL:11
    }

    // escaped quote
    assert_lex! {
        r#""hel\"lo\"""#,
        CSS_STRING_LITERAL:11
    }

    // unicode
    assert_lex! {
        "'юникод'",
        CSS_STRING_LITERAL:14
    }

    // missing single closing quote
    assert_lex! {
        "'he",
        ERROR_TOKEN:3
    }

    // missing double closing quote
    assert_lex! {
        r#""he"#,
        ERROR_TOKEN:3
    }

    // line break
    assert_lex! {
        r#"'he
    "#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4
    }

    // line break
    assert_lex! {
        r#"'he
    '"#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4,
        ERROR_TOKEN:1
    }

    assert_lex! {
        r#""Escaped \n""#,
        CSS_STRING_LITERAL:12
    }

    assert_lex! {
        r#""Escaped \r""#,
        CSS_STRING_LITERAL:12
    }

    assert_lex! {
        "\"a\\\r\nb\"",
        CSS_STRING_LITERAL:7
    }

    assert_lex! {
        "\"\\41\nb\"",
        CSS_STRING_LITERAL:7
    }

    // invalid escape sequence
    assert_lex! {
        r"'\0'",
        ERROR_TOKEN:4
    }
}

#[test]
fn number() {
    assert_lex! {
        "5098382",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "509.382",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        ".382",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "+123",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "-123",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "+123",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "123e10",
        CSS_NUMBER_LITERAL:6
    }

    assert_lex! {
        "123e+10",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "123e-10",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "123E10",
        CSS_NUMBER_LITERAL:6
    }

    assert_lex! {
        "123E+10",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "123E-10",
        CSS_NUMBER_LITERAL:7
    }
}

#[test]
fn dimension() {
    assert_lex! {
        "5098382s",
        CSS_DIMENSION_VALUE:7,
        S_KW:1
    }
    assert_lex! {
        "10.0px",
        CSS_DIMENSION_VALUE:4,
        PX_KW:2
    }
    assert_lex! {
        "10.0e+7fr",
        CSS_DIMENSION_VALUE:7,
        FR_KW:2
    }
    assert_lex! {
        "0\\0",
        CSS_DIMENSION_VALUE:1,
        IDENT:2
    }

    // A space breaks the dimension token and leaves it as a number literal
    assert_lex! {
        "1 px",
        CSS_NUMBER_LITERAL:1,
        WHITESPACE:1,
        PX_KW:2
    }

    // Percentages aren't technically dimensions, but we treat them similarly.
    assert_lex! {
        "100%",
        CSS_PERCENTAGE_VALUE:3,
        PERCENT:1
    }

    assert_lex! {
        "100 %",
        CSS_NUMBER_LITERAL:3,
        WHITESPACE:1
        PERCENT:1
    }
}

#[test]
fn cdo_and_cdc() {
    assert_lex! {
        "<!-- -->",
        CDO:4,
        WHITESPACE:1,
        CDC:3
    }
}

#[test]
fn keywords() {
    assert_lex! {
        "media keyframes important from through sass each debug warn error for function include mixin return",
        MEDIA_KW:5,
        WHITESPACE:1,
        KEYFRAMES_KW:9,
        WHITESPACE:1,
        IMPORTANT_KW:9,
        WHITESPACE:1,
        FROM_KW:4,
        WHITESPACE:1,
        THROUGH_KW:7,
        WHITESPACE:1,
        SASS_KW:4,
        WHITESPACE:1,
        EACH_KW:4,
        WHITESPACE:1,
        DEBUG_KW:5,
        WHITESPACE:1,
        WARN_KW:4,
        WHITESPACE:1,
        ERROR_KW:5,
        WHITESPACE:1,
        FOR_KW:3,
        WHITESPACE:1,
        FUNCTION_KW:8,
        WHITESPACE:1,
        INCLUDE_KW:7,
        WHITESPACE:1,
        MIXIN_KW:5,
        WHITESPACE:1,
        RETURN_KW:6
    }
}

#[test]
fn attribute() {
    assert_lex! {
        "$=",
        DOLLAR_EQ:2
    }
}

#[test]
fn identifier() {
    assert_lex! {
        "--",
        MINUS:1,
        MINUS:1
    }

    assert_lex! {
        "i4f5g7",
        IDENT:6
    }

    assert_lex! {
        "class",
        IDENT:5
    }

    assert_lex! {
        r"cl\aass",
        IDENT:7
    }

    assert_lex! {
        r"\ccl\aass",
        IDENT:9
    }

    assert_lex! {
        "-class",
        IDENT:6
    }

    assert_lex! {
        r"-cl\aass",
        IDENT:8
    }

    assert_lex! {
        r"-\acl\aass",
        IDENT:10
    }

    assert_lex! {
        "--property",
        IDENT:10
    }

    assert_lex! {
        r"--prop\eerty",
        IDENT:12
    }

    assert_lex! {
        r"--\pprop\eerty",
        IDENT:14
    }
}

#[test]
fn wrong_line_comments() {
    assert_lex! {
        "//abc
    ",
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:4
    }

    assert_lex! {
        "//a",
        COMMENT:3
    }
}

#[test]
fn block_comment() {
    assert_lex! {
        "/*
        */",
        MULTILINE_COMMENT:13
    }

    assert_lex! {
        "/* */",
        COMMENT:5
    }

    assert_lex! {
        "/* *",
        COMMENT:4
    }
}

#[test]
fn char() {
    assert_lex! {
        "!",
        BANG:1
    }
    assert_lex! {
        "%",
        PERCENT:1
    }
    assert_lex! {
        "/",
        SLASH:1
    }
}
