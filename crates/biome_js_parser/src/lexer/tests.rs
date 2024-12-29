#![cfg(test)]
#![expect(unused_mut, unused_variables)]

use super::{JsLexContext, JsLexer, TextRange, TextSize};
use crate::span::Span;
use biome_js_syntax::JsSyntaxKind::{self, EOF};
use biome_js_syntax::JsSyntaxKind::{JS_NUMBER_LITERAL, NEWLINE, WHITESPACE};
use biome_js_syntax::T;
use biome_parser::lexer::{BufferedLexer, Lexer};
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = JsLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(JsLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                biome_js_syntax::JsSyntaxKind::$kind,
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

            new_str.push_str(&$src[tokens[idx].1.as_range()]);
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
        let mut lexer = JsLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(JsLexContext::default()) != EOF {
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
        new_str.push_str(&string[range.as_range()]);
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
fn identifier() {
    assert_lex! {
        "Abcdefg",
        IDENT:7
    }
}

#[test]
fn unicode_identifier() {
    assert_lex! {
        r#"\uD83D\uDCA9"#,
        ERROR_TOKEN:5,
        IDENT: 1,
        ERROR_TOKEN:5,
        JS_NUMBER_LITERAL: 1,
    }

    assert_lex! {
        r#"a\uD83D\uDCA9"#,
        IDENT:1,
        ERROR_TOKEN:5,
        IDENT: 1,
        ERROR_TOKEN:5,
        JS_NUMBER_LITERAL: 1,
    }

    assert_lex! {
        r#"a\uD83D"#,
        IDENT:1,
        ERROR_TOKEN:5,
        IDENT: 1,
    }
}

#[test]
fn punctuators() {
    assert_lex! {
        "!%%&()*+,-.:;<=>?[]^{}|~",
        BANG:1,
        PERCENT:1,
        PERCENT:1,
        AMP:1,
        L_PAREN:1,
        R_PAREN:1,
        STAR:1,
        PLUS:1,
        COMMA:1,
        MINUS:1,
        DOT:1,
        COLON:1,
        SEMICOLON:1,
        LTEQ:2,
        R_ANGLE:1,
        QUESTION:1,
        L_BRACK:1,
        R_BRACK:1,
        CARET:1,
        L_CURLY:1,
        R_CURLY:1,
        PIPE:1,
        TILDE:1,
    }
}

#[test]
fn bang() {
    assert_lex!(
        r#"!/a/"#,
        BANG:1,
        SLASH:1,
        IDENT:1,
        SLASH:1
    );

    assert_lex!(
        r#"a!/a/"#,
        IDENT:1,
        BANG:1,
        SLASH:1,
        IDENT:1,
        SLASH:1
    );

    assert_lex!(
        "1 && !2",
        JS_NUMBER_LITERAL:1,
        WHITESPACE:1,
        AMP2:2,
        WHITESPACE:1,
        BANG:1
        JS_NUMBER_LITERAL:1
    );
}

#[test]
fn consecutive_punctuators() {
    assert_lex! {
        "&&&&^^^||",
        AMP2:2,
        AMP2:2,
        CARET:1,
        CARET:1,
        CARET:1,
        PIPE2:2,
    }
}

#[test]
fn unicode_whitespace() {
    assert_lex! {
        " \u{00a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}",
        WHITESPACE:48
    }
}

#[test]
fn unicode_whitespace_ident_part() {
    assert_lex! {
        "Abcd\u{2006}",
        IDENT:4,
        WHITESPACE:3 // length is in bytes
    }
}

#[test]
fn all_whitespace() {
    assert_lex! {
        "\n\t\t",
        NEWLINE:1
        WHITESPACE:2
    }
    assert_lex! {
        "\r\n\t\t",
        NEWLINE:2
        WHITESPACE:2
    }
    assert_lex! {
        "\n\n",
        NEWLINE:1
        NEWLINE:1
    }
    assert_lex! {
        "\r\n\r\n",
        NEWLINE:2
        NEWLINE:2
    }
    assert_lex! {
        "\r\r\r\r",
        NEWLINE:1
        NEWLINE:1
        NEWLINE:1
        NEWLINE:1
    }
    assert_lex! {
        "\r\r\n\n\u{2028}\u{2029}",
        NEWLINE:1
        NEWLINE:2
        NEWLINE:1
        NEWLINE:3
        NEWLINE:3
    }
}

#[test]
fn empty_string() {
    assert_lex! {
        r#""""#,
        JS_STRING_LITERAL:2
    }

    assert_lex! {
        "''",
        JS_STRING_LITERAL:2
    }
}

#[test]
fn simple_string() {
    assert_lex! {
        r"'abcdefghijklmnopqrstuvwxyz123456789\'10🦀'",
        JS_STRING_LITERAL:45
    }

    assert_lex! {
        r#""abcdefghijklmnopqrstuvwxyz123456789\"10🦀""#,
        JS_STRING_LITERAL:45
    }
}

#[test]
fn string_unicode_escape_invalid() {
    assert_lex! {
        r#""abcd\u21""#,
        ERROR_TOKEN:10
    }

    assert_lex! {
        r"'abcd\u21'",
        ERROR_TOKEN:10
    }
}

#[test]
fn string_unicode_escape_valid() {
    assert_lex! {
        r#""abcd\u2000a""#,
        JS_STRING_LITERAL:13
    }

    assert_lex! {
        r"'abcd\u2000a'",
        JS_STRING_LITERAL:13
    }
}

#[test]
fn string_unicode_escape_surrogates() {
    assert_lex! {
        r#""\uD83D\uDCA9""#,
        JS_STRING_LITERAL:14
    }

    assert_lex! {
        r#""\uD83D""#,
        JS_STRING_LITERAL:8
    }
}

#[test]
fn string_unicode_escape_valid_resolving_to_endquote() {
    assert_lex! {
        r#""abcd\u0022a""#,
        JS_STRING_LITERAL:13
    }

    assert_lex! {
        r"'abcd\u0027a'",
        JS_STRING_LITERAL:13
    }
}

#[test]
fn string_hex_escape_invalid() {
    assert_lex! {
        r#""abcd \xZ0 \xGH""#,
        ERROR_TOKEN:16
    }

    assert_lex! {
        r"'abcd \xZ0 \xGH'",
        ERROR_TOKEN:16
    }
}

#[test]
fn string_hex_escape_valid() {
    assert_lex! {
        r#""abcd \x00 \xAB""#,
        JS_STRING_LITERAL:16
    }

    assert_lex! {
        r"'abcd \x00 \xAB'",
        JS_STRING_LITERAL:16
    }
}

#[test]
fn unterminated_string() {
    assert_lex! {
        r#""abcd"#,
        ERROR_TOKEN:5
    }

    assert_lex! {
        r#"'abcd"#,
        ERROR_TOKEN:5
    }
}

#[test]
fn string_all_escapes() {
    assert_lex! {
        r#""\x\u2004\u20\ux\xNN""#,
        ERROR_TOKEN:21
    }

    assert_lex! {
        r"'\x\u2004\u20\ux\xNN'",
        ERROR_TOKEN:21
    }
}

#[test]
fn complex_string_1() {
    assert_lex! {
        r#" _this += "str'n\u200bg";"#,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        PLUSEQ:2,
        WHITESPACE:1,
        JS_STRING_LITERAL:14,
        SEMICOLON:1
    }

    assert_lex! {
        r#" _this += 'str"n\u200bg';"#,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        PLUSEQ:2,
        WHITESPACE:1,
        JS_STRING_LITERAL:14,
        SEMICOLON:1
    }
}

#[test]
fn unterminated_string_length() {
    assert_lex! {
        "'abc",
        ERROR_TOKEN:4
    }
}

#[test]
fn unterminated_string_with_escape_len() {
    assert_lex! {
        "'abc\\",
        ERROR_TOKEN:5
    }

    assert_lex! {
        r"'abc\x",
        ERROR_TOKEN:6
    }

    assert_lex! {
        r"'abc\x4",
        ERROR_TOKEN:7
    }

    assert_lex! {
        r"'abc\x45",
        ERROR_TOKEN:8
    }

    assert_lex! {
        r"'abc\u",
        ERROR_TOKEN:6
    }

    assert_lex! {
        r"'abc\u20",
        ERROR_TOKEN:8
    }
}

#[test]
fn dollarsign_underscore_idents() {
    assert_lex! {
        "$a",
        IDENT:2
    }
}

#[test]
fn labels_a() {
    assert_lex! {
        "await",
        AWAIT_KW:5
    }

    assert_lex! {
        "awaited",
        IDENT:7
    }
}

#[test]
fn labels_b() {
    assert_lex! {
        "break",
        BREAK_KW:5
    }

    assert_lex! {
        "breaking speed records",
        IDENT:8,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        IDENT:7
    }
}

#[test]
fn labels_c() {
    assert_lex! {
        "continue, const, class, catch, case",
        CONTINUE_KW:8,
        COMMA:1,
        WHITESPACE:1,
        CONST_KW:5,
        COMMA:1,
        WHITESPACE:1,
        CLASS_KW:5,
        COMMA:1,
        WHITESPACE:1,
        CATCH_KW:5,
        COMMA:1,
        WHITESPACE:1,
        CASE_KW:4
    }

    assert_lex! {
        "classy crabs",
        IDENT:6,
        WHITESPACE:1,
        IDENT:5
    }
}

#[test]
fn labels_d() {
    assert_lex! {
        "debugger default delete do",
        DEBUGGER_KW:8,
        WHITESPACE:1,
        DEFAULT_KW:7,
        WHITESPACE:1,
        DELETE_KW:6,
        WHITESPACE:1,
        DO_KW:2
    }

    assert_lex! {
        "derive doot d",
        IDENT:6,
        WHITESPACE:1,
        IDENT:4,
        WHITESPACE:1,
        IDENT:1
    }
}

#[test]
fn labels_e() {
    assert_lex! {
        "else enum export extends",
        ELSE_KW:4,
        WHITESPACE:1,
        ENUM_KW:4,
        WHITESPACE:1,
        EXPORT_KW:6,
        WHITESPACE:1,
        EXTENDS_KW:7
    }

    assert_lex! {
        "e exports elsey",
        IDENT:1,
        WHITESPACE:1,
        IDENT:7,
        WHITESPACE:1,
        IDENT:5
    }
}

#[test]
fn labels_f() {
    assert_lex! {
        "finally for function",
        FINALLY_KW:7,
        WHITESPACE:1,
        FOR_KW:3,
        WHITESPACE:1,
        FUNCTION_KW:8
    }

    assert_lex! {
        "finally, foreign food!",
        FINALLY_KW:7,
        COMMA:1,
        WHITESPACE:1,
        IDENT:7,
        WHITESPACE:1,
        IDENT:4,
        BANG:1
    }
}

#[test]
fn labels_i() {
    assert_lex! {
        "i in instanceof if import",
        IDENT:1,
        WHITESPACE:1,
        IN_KW: 2,
        WHITESPACE:1,
        INSTANCEOF_KW:10,
        WHITESPACE:1,
        IF_KW:2,
        WHITESPACE:1,
        IMPORT_KW:6
    }

    assert_lex! {
        "icecream is interesting, innit?",
        IDENT:8,
        WHITESPACE:1,
        IS_KW:2,
        WHITESPACE:1,
        IDENT:11,
        COMMA:1,
        WHITESPACE:1,
        IDENT:5,
        QUESTION:1
    }
}

#[test]
fn labels_n() {
    assert_lex! {
        "new",
        NEW_KW:3
    }

    assert_lex! {
        "newly n",
        IDENT:5,
        WHITESPACE:1,
        IDENT:1
    }
}

#[test]
fn labels_r() {
    assert_lex! {
        "return",
        RETURN_KW:6
    }

    assert_lex! {
        "returning",
        IDENT:9
    }
}

#[test]
fn labels_s() {
    assert_lex! {
        "switch super",
        SWITCH_KW:6,
        WHITESPACE:1,
        SUPER_KW:5
    }

    assert_lex! {
        "superb switching",
        IDENT:6,
        WHITESPACE:1,
        IDENT:9
    }
}

#[test]
fn labels_t() {
    assert_lex! {
        "this try throw typeof t",
        THIS_KW:4,
        WHITESPACE:1,
        TRY_KW:3,
        WHITESPACE:1,
        THROW_KW:5,
        WHITESPACE:1,
        TYPEOF_KW:6,
        WHITESPACE:1,
        IDENT:1
    }

    assert_lex! {
        "thistle throwing tea",
        IDENT:7,
        WHITESPACE:1,
        IDENT:8,
        WHITESPACE:1,
        IDENT:3
    }
}

#[test]
fn labels_v() {
    assert_lex! {
        "var void v",
        VAR_KW:3,
        WHITESPACE:1,
        VOID_KW:4,
        WHITESPACE:1,
        IDENT:1
    }

    assert_lex! {
        "variable voiding is bad",
        IDENT:8,
        WHITESPACE:1,
        IDENT:7,
        WHITESPACE:1,
        IS_KW:2,
        WHITESPACE:1,
        IDENT:3
    }
}

#[test]
fn labels_w() {
    assert_lex! {
        "with while w",
        WITH_KW:4,
        WHITESPACE:1,
        WHILE_KW:5,
        WHITESPACE:1,
        IDENT:1
    }

    assert_lex! {
        "whiley withow",
        IDENT:6,
        WHITESPACE:1,
        IDENT:6
    }
}

#[test]
fn labels_y() {
    assert_lex! {
        "yield",
        YIELD_KW:5
    }

    assert_lex! {
        "yielding",
        IDENT:8
    }
}

#[test]
fn number_basic() {
    assert_lex! {
        "1",
        JS_NUMBER_LITERAL:1
    }

    assert_lex! {
        "123456 ",
        JS_NUMBER_LITERAL:6,
        WHITESPACE:1
    }

    assert_lex! {
        "90",
        JS_NUMBER_LITERAL:2
    }

    assert_lex! {
        ".13",
        JS_NUMBER_LITERAL:3
    }
}

#[test]
fn number_basic_err() {
    assert_lex! {
        "2_?",
        JS_NUMBER_LITERAL:2, // numeric separator error
        QUESTION:1
    }

    assert_lex! {
        r"25\u0046abcdef",
        ERROR_TOKEN:14
    }

    assert_lex! {
        r"25\uFEFFb",
        JS_NUMBER_LITERAL:2,
        ERROR_TOKEN:6,
        IDENT:1
    }

    assert_lex! {
        r".32\u0046abde",
        ERROR_TOKEN:13
    }

    assert_lex! {
        r#".0_e1"#, // numeric separator error
        JS_NUMBER_LITERAL:5,
    }

    assert_lex! {
        r#"10e_1"#, // numeric separator error
        ERROR_TOKEN:5
    }
}

#[test]
fn number_leading_zero_err() {
    assert_lex! {
        r#"0_0"#,
        JS_NUMBER_LITERAL:3 // error: numeric separator can not be used after leading 0
    }

    assert_lex! {
        r#"01.1"#,
        JS_NUMBER_LITERAL:4, // error: unexpected number
    }

    assert_lex! {
        r#"01n"#,
        JS_NUMBER_LITERAL:3 // error: Octal literals are not allowed for BigInts.
    }
}

#[test]
fn number_complex() {
    assert_lex! {
        "3e-5 123e+56",
        JS_NUMBER_LITERAL:4,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "3.14159e+1",
        JS_NUMBER_LITERAL:10
    }

    assert_lex! {
        ".0e34",
        JS_NUMBER_LITERAL:5
    }

    assert_lex! {
        "0e00",
        JS_NUMBER_LITERAL:4
    }
}

#[test]
fn dot_number_disambiguation() {
    assert_lex! {
        ".e+5",
        DOT:1,
        IDENT:1,
        PLUS:1,
        JS_NUMBER_LITERAL:1
    }

    assert_lex! {
        ".0e+5",
        JS_NUMBER_LITERAL:5
    }
}

#[test]
fn binary_literals() {
    assert_lex! {
        "0b10101010, 0B10101010, 0b10101010n",
        JS_NUMBER_LITERAL:10,
        COMMA:1,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:10,
        COMMA:1,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:11
    }
}

#[test]
fn octal_literals() {
    assert_lex! {
        "0o01742242, 0B10101010, 0b10101010n",
        JS_NUMBER_LITERAL:10,
        COMMA:1,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:10,
        COMMA:1,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:11
    }
}

#[test]
fn bigint_literals() {
    assert_lex! {
        "0n 1743642n 1n",
        JS_NUMBER_LITERAL:2,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:8,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:2
    }
}

#[test]
fn shebang() {
    assert_lex! {
        "#! /bin/node",
        JS_SHEBANG:12
    }

    assert_lex! {
        "#!/bin/node\n",
        JS_SHEBANG:11,
        NEWLINE:1
    }

    assert_lex! {
        "#!/bin/node\r\n",
        JS_SHEBANG:11,
        NEWLINE:2
    }

    assert_lex! {
        "#!/usr/bin/env deno\u{2028}",
        JS_SHEBANG:19,
        NEWLINE:3
    }

    assert_lex! {
        "#0",
        ERROR_TOKEN:1,
        JS_NUMBER_LITERAL:1
    }

    assert_lex! {
        "0#!/bin/deno",
        JS_NUMBER_LITERAL:1,
        HASH:1,
        BANG:1,
        SLASH:1,
        IDENT:3,
        SLASH:1,
        IDENT:4,
    }
}

#[test]
fn single_line_comments() {
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
fn division() {
    assert_lex! {
        "var a = 5 / 6",
        VAR_KW:3,
        WHITESPACE:1,
        IDENT:1,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:1,
        WHITESPACE:1,
        SLASH:1,
        WHITESPACE:1,
        JS_NUMBER_LITERAL:1
    }
}

#[test]
fn fuzz_fail_1() {
    assert_lex! {
        "$\\u",
        IDENT:1,
        ERROR_TOKEN:2
    }
}

#[test]
fn fuzz_fail_2() {
    assert_lex! {
        "..",
        DOT:1,
        DOT:1
    }
}

#[test]
fn fuzz_fail_3() {
    assert_lex! {
        "0e",
        ERROR_TOKEN:2
    }
}

#[test]
fn fuzz_fail_4() {
    assert_lex! {
        "0o 0b 0x",
        ERROR_TOKEN:2,
        WHITESPACE:1,
        ERROR_TOKEN:2,
        WHITESPACE:1,
        ERROR_TOKEN:2
    }
}

#[test]
fn fuzz_fail_5() {
    assert_lex! {
        "//\u{2028}",
        COMMENT:2,
        NEWLINE:3
    }
}

#[test]
fn fuzz_fail_6() {
    assert_lex! {
        "//\u{200a}",
        COMMENT:5
    }
}

#[test]
fn unicode_ident_start_handling() {
    assert_lex! {
        "αβeta_tester",
        IDENT:14
    }
}

#[test]
fn unicode_ident_separated_by_unicode_whitespace() {
    assert_lex! {
        "β\u{FEFF}α",
        IDENT:2,
        WHITESPACE:3,
        IDENT:2
    }
}

#[test]
fn err_on_unterminated_unicode() {
    assert_lex! {
        "+\\u{A",
        PLUS:1
        ERROR_TOKEN:4
    }
}

#[test]
fn issue_30() {
    assert_lex! {
        "let foo = { α: true }",
        LET_KW:3,
        WHITESPACE:1,
        IDENT:3,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        L_CURLY:1,
        WHITESPACE:1,
        IDENT:2,
        COLON:1,
        WHITESPACE:1,
        TRUE_KW:4,
        WHITESPACE:1,
        R_CURLY:1
    }
}
#[test]
fn at_token() {
    assert_lex! {
        "@",
        AT:1
    }

    assert_lex! {
        "@foo",
        AT:1,
        IDENT:3
    }
}

#[test]
fn object_expr_getter() {
    assert_lex! {
        "({ get [foo]() {} })",
        L_PAREN:1
        L_CURLY:1
        WHITESPACE:1
        GET_KW:3
        WHITESPACE:1
        L_BRACK:1
        IDENT:3
        R_BRACK:1
        L_PAREN:1
        R_PAREN:1
        WHITESPACE:1
        L_CURLY:1
        R_CURLY:1
        WHITESPACE:1
        R_CURLY:1
        R_PAREN:1
    }
}

#[test]
fn newline_space_must_be_two_tokens() {
    assert_lex! {
        "\n ",
        NEWLINE:1
        WHITESPACE:1
    }
    assert_lex! {
        " \n",
        WHITESPACE:1
        NEWLINE:1
    }
    assert_lex! {
        " \n ",
        WHITESPACE:1
        NEWLINE:1
        WHITESPACE:1
    }

    assert_lex! {
        " a\n b \n ",
        WHITESPACE:1
        IDENT:1
        NEWLINE:1
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        NEWLINE:1
        WHITESPACE:1
    }
    assert_lex! {
        "a //COMMENT \n /*COMMENT*/ b /*COM\nMENT*/",
        IDENT:1
        WHITESPACE:1
        COMMENT:10
        NEWLINE:1
        WHITESPACE:1
        COMMENT:11
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        MULTILINE_COMMENT:12
    }
    assert_lex! {
        "a //COMMENT \n /*COMMENT*/ b /*COM\nMENT*/",
        IDENT:1
        WHITESPACE:1
        COMMENT:10
        NEWLINE:1
        WHITESPACE:1
        COMMENT:11
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        MULTILINE_COMMENT:12
    }

    //Now with CR
    assert_lex! {
        "\r\n ",
        NEWLINE:2
        WHITESPACE:1
    }

    assert_lex! {
        " \r\n",
        WHITESPACE:1
        NEWLINE:2
    }
    assert_lex! {
        " \r\n ",
        WHITESPACE:1
        NEWLINE:2
        WHITESPACE:1
    }

    assert_lex! {
        " a\r\n b \r\n ",
        WHITESPACE:1
        IDENT:1
        NEWLINE:2
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        NEWLINE:2
        WHITESPACE:1
    }
    assert_lex! {
        "a //COMMENT \r\n /*COMMENT*/ b /*COM\r\nMENT*/",
        IDENT:1
        WHITESPACE:1
        COMMENT:10
        NEWLINE:2
        WHITESPACE:1
        COMMENT:11
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        MULTILINE_COMMENT:13
    }
    assert_lex! {
        "a //COMMENT \r\n /*COMMENT*/ b /*COM\r\nMENT*/",
        IDENT:1
        WHITESPACE:1
        COMMENT:10
        NEWLINE:2
        WHITESPACE:1
        COMMENT:11
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        MULTILINE_COMMENT:13
    }
}

#[test]
fn are_we_jsx() {
    assert_lex! {
        r#"<some-div>{"Hey" + 1 + fn()}</some-div>"#,
        L_ANGLE:1
        IDENT:4
        MINUS:1
        IDENT:3
        R_ANGLE:1
        L_CURLY:1
        JS_STRING_LITERAL:5
        WHITESPACE:1
        PLUS:1
        WHITESPACE:1
        JS_NUMBER_LITERAL:1
        WHITESPACE:1
        PLUS:1
        WHITESPACE:1
        IDENT:2
        L_PAREN:1
        R_PAREN:1
        R_CURLY:1
        L_ANGLE:1
        SLASH:1
        IDENT:4
        MINUS:1
        IDENT:3
        R_ANGLE:1
    }
}

#[test]
fn keywords() {
    let keywords = vec![
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "false",
        "finally",
        "for",
        "function",
        "if",
        "in",
        "instanceof",
        "import",
        "new",
        "null",
        "return",
        "super",
        "switch",
        "this",
        "throw",
        "try",
        "true",
        "typeof",
        "var",
        "void",
        "while",
        "with",
        // Strict mode contextual keywords
        "implements",
        "interface",
        "let",
        "package",
        "private",
        "protected",
        "public",
        "static",
        "yield",
        // contextual keywords
        "abstract",
        "as",
        "asserts",
        "assert",
        "any",
        "async",
        "await",
        "boolean",
        "constructor",
        "declare",
        "get",
        "infer",
        "is",
        "keyof",
        "module",
        "namespace",
        "never",
        "readonly",
        "require",
        "number",
        "object",
        "set",
        "string",
        "symbol",
        "type",
        "undefined",
        "unique",
        "unknown",
        "from",
        "global",
        "bigint",
        "override",
        "of",
    ];

    for keyword in keywords {
        let kind = JsSyntaxKind::from_keyword(keyword).expect(
            "Expected `JsSyntaxKind::from_keyword` to return a kind for keyword {keyword}.",
        );

        let mut lexer = JsLexer::from_str(keyword);
        lexer.next_token(JsLexContext::default());

        let lexed_kind = lexer.current();
        assert_eq!(
            lexed_kind, kind,
            "Expected token '{keyword}' to be of kind {kind:?} but is {lexed_kind:?}."
        );

        let lexed_range = lexer.current_range();
        assert_eq!(
            lexed_range.len(),
            TextSize::from(keyword.len() as u32),
            "Expected lexed keyword to be of len {} but has length {:?}",
            keyword.len(),
            lexed_range.len()
        );

        assert_eq!(lexer.next_token(JsLexContext::default()), EOF);
    }
}

#[test]
fn without_lookahead() {
    let lexer = JsLexer::from_str("let a\n = 5");
    let mut buffered = BufferedLexer::new(lexer);

    buffered.next_token(JsLexContext::default());
    assert_eq!(buffered.current(), T![let]);
    assert!(!buffered.has_preceding_line_break());
    assert_eq!(
        buffered.current_range(),
        TextRange::at(TextSize::from(0), TextSize::from(3))
    );

    assert_eq!(buffered.next_token(JsLexContext::default()), WHITESPACE);
    assert_eq!(buffered.next_token(JsLexContext::default()), T![ident]);
    assert_eq!(buffered.next_token(JsLexContext::default()), NEWLINE);
    assert_eq!(buffered.next_token(JsLexContext::default()), WHITESPACE);
    assert_eq!(buffered.next_token(JsLexContext::default()), T![=]);
    assert!(buffered.has_preceding_line_break());
    assert_eq!(buffered.next_token(JsLexContext::default()), WHITESPACE);
    assert_eq!(
        buffered.next_token(JsLexContext::default()),
        JS_NUMBER_LITERAL
    );
    assert_eq!(buffered.next_token(JsLexContext::default()), T![EOF]);
}

#[test]
fn lookahead() {
    let lexer = JsLexer::from_str("let a\n = 5");
    let mut buffered = BufferedLexer::new(lexer);

    buffered.next_token(JsLexContext::default());
    assert_eq!(buffered.current(), T![let]);
    assert!(!buffered.has_preceding_line_break());
    assert_eq!(
        buffered.current_range(),
        TextRange::at(TextSize::from(0), TextSize::from(3))
    );

    {
        let lookahead = buffered
            .lookahead_iter()
            .map(|l| l.kind())
            .collect::<Vec<_>>();

        assert_eq!(
            lookahead,
            vec![
                WHITESPACE,
                T![ident],
                NEWLINE,
                WHITESPACE,
                T![=],
                WHITESPACE,
                JS_NUMBER_LITERAL,
                T![EOF]
            ]
        );
    }

    assert_eq!(buffered.current(), T![let]);
    assert_eq!(buffered.next_token(JsLexContext::default()), WHITESPACE);

    {
        let mut lookahead = buffered.lookahead_iter();
        let nth1 = lookahead.next().unwrap();
        let nth2 = lookahead.next().unwrap();
        let nth3 = lookahead.next().unwrap();
        let nth4 = lookahead.next().unwrap();

        assert_eq!(nth1.kind(), T![ident]);
        assert_eq!(nth2.kind(), NEWLINE);
        assert_eq!(nth3.kind(), WHITESPACE);
        assert_eq!(nth4.kind(), T![=]);
        assert!(nth4.has_preceding_line_break());
    }

    assert_eq!(buffered.next_token(JsLexContext::default()), T![ident]);
    assert_eq!(buffered.next_token(JsLexContext::default()), NEWLINE);
    assert_eq!(buffered.next_token(JsLexContext::default()), WHITESPACE);
    assert_eq!(buffered.next_token(JsLexContext::default()), T![=]);
    assert!(buffered.has_preceding_line_break());
    assert_eq!(buffered.next_token(JsLexContext::default()), WHITESPACE);
    assert_eq!(
        buffered.next_token(JsLexContext::default()),
        JS_NUMBER_LITERAL
    );
    assert_eq!(buffered.next_token(JsLexContext::default()), T![EOF]);
}
