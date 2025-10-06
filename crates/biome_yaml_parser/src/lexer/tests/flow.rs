use crate::assert_lex;

#[test]
fn lex_double_quoted_literal() {
    assert_lex!(
        "\"hello world\"",
        FLOW_START:0,
        DOUBLE_QUOTED_LITERAL:13,
        FLOW_END:0,
    );
}

#[test]
fn lex_multiline_double_quoted_literal() {
    assert_lex!(
        r#"
"hello
  world"
"#,
        NEWLINE:1,
        FLOW_START:0,
        DOUBLE_QUOTED_LITERAL:15,
        FLOW_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_missing_closing_quote_double_quote_literal() {
    assert_lex!(
        "\"hello world",
        FLOW_START:0,
        DOUBLE_QUOTED_LITERAL:12,
        FLOW_END:0,
    );
}

#[test]
fn lex_missing_closing_quote_double_quote_literal_in_mapping() {
    assert_lex!(
        r#"
map:
  "hello world
"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:3,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:2,
        FLOW_START:0,
        DOUBLE_QUOTED_LITERAL:12,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_double_quote_as_key() {
    assert_lex!(
        r#"
"key": value
"#,
        NEWLINE:1,
        MAPPING_START:0,
        DOUBLE_QUOTED_LITERAL:5,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:5,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_single_quoted_literal() {
    assert_lex!(
        "'hello world'",
        FLOW_START:0,
        SINGLE_QUOTED_LITERAL:13,
        FLOW_END:0,
    );
}

#[test]
fn lex_multiline_single_quoted_literal() {
    assert_lex!(
        r#"
'hello
 world'
"#,
        NEWLINE:1,
        FLOW_START:0,
        SINGLE_QUOTED_LITERAL:14,
        FLOW_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_missing_closing_quote_single_quoted_literal() {
    assert_lex!(
        "'hello world",
        FLOW_START:0,
        SINGLE_QUOTED_LITERAL:12,
        FLOW_END:0,
    );
}

#[test]
fn lex_missing_closing_quote_single_quoted_literal_in_sequence() {
    assert_lex!(
        r#"
- 'hello
 world"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        SINGLE_QUOTED_LITERAL:13,
        FLOW_END:0,
        SEQUENCE_END:0,
    );
}

#[test]
fn lex_single_quote_as_key() {
    assert_lex!(
        "'key': value",
        MAPPING_START:0,
        SINGLE_QUOTED_LITERAL:5,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:5,
        FLOW_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_simple_plain() {
    assert_lex!(
        "simple_plain\n",
        FLOW_START:0,
        PLAIN_LITERAL:13,
        FLOW_END:0,
    );
}

#[test]
fn lex_flow_plain_start_with_colon() {
    assert_lex!(
        ":abc",
        FLOW_START:0,
        PLAIN_LITERAL:4,
        FLOW_END:0,
    );
}

#[test]
fn lex_flow_plain_start_with_dash() {
    assert_lex!(
        "-abc",
        FLOW_START:0,
        PLAIN_LITERAL:4,
        FLOW_END:0,
    );
}

#[test]
fn lex_flow_plain_start_with_question() {
    assert_lex!(
        "?abc",
        FLOW_START:0,
        PLAIN_LITERAL:4,
        FLOW_END:0,
    );
}

#[test]
fn lex_plain_followed_by_colon_in_flow_collection() {
    assert_lex!(
        "[abc:]",
        FLOW_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:3,
        COLON:1,
        R_BRACK:1,
        FLOW_END:0,
    );
}

// The last colon violates c-mapping-value [ lookahead = ns-plain-safe(c) ]
// equivalent to `[{':' : null}]`
#[test]
fn lex_colon_as_plain() {
    assert_lex!(
        "[::]",
        FLOW_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:1,
        COLON:1,
        R_BRACK:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_plain_with_colon_in_flow_sequence() {
    assert_lex!(
        "[abc:abc]",
        FLOW_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:7,
        R_BRACK:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_plain_with_colon_in_flow_mapping() {
    assert_lex!(
        "{abc:abc}",
        FLOW_START:0,
        L_CURLY:1,
        PLAIN_LITERAL:7,
        R_CURLY:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_multiline_plain() {
    assert_lex!(
        r#"
Just a
multiline
plain token
"#,
        NEWLINE:1,
        FLOW_START:0,
        PLAIN_LITERAL:29,
        FLOW_END:0,
    );
}

#[test]
fn lex_mapping_with_multiline_plain() {
    assert_lex!(
        r#"
this: is
 a multiline
 plain
"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:4,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:22,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

// This is actually not permitted by the standard, but we lex it anyway. We can catch this in the
// syntax analyzer for better error message
#[test]
fn lex_multiline_plain_as_key() {
    assert_lex!(
        r#"
multiline
key: oh no
"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:13,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:5,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_multiline_plain_as_key_in_nested_mapping() {
    assert_lex!(
        r#"
normal:
  multiline
 key: oh no
"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:6,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:2,
        MAPPING_START:0,
        PLAIN_LITERAL:14,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:5,
        FLOW_END:0,
        MAPPING_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_incorrectly_nested_sequence_as_plain() {
    assert_lex!(
        r#"
- a
 - b
 - c
 - d
"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:16,
        FLOW_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_incorrectly_nested_mapping_as_multiline_plain_key() {
    assert_lex!(
        r#"
a: b
  c:"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:5,
        COLON:1,
        MAPPING_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_single_flow_collection() {
    assert_lex!(
        r#"
[1, {4: 5, 6:
[7, 8]}, 3]
"#,
        NEWLINE:1,
        FLOW_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        L_CURLY:1,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        L_BRACK:1,
        PLAIN_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        R_BRACK:1,
        R_CURLY:1,
        COMMA:1,
        WHITESPACE:1
        PLAIN_LITERAL:1,
        R_BRACK:1,
        FLOW_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_empty_flow_collection() {
    assert_lex!(
        "[]",
        FLOW_START:0,
        L_BRACK:1,
        R_BRACK:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_multi_line_flow_in_block_sequence() {
    assert_lex!(
        r#"
- [
 abc]
"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        L_BRACK:1,
        NEWLINE:1,
        WHITESPACE:1,
        PLAIN_LITERAL:3,
        R_BRACK:1,
        FLOW_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_flow_map_adjacent_key_value() {
    assert_lex!(
        "{'abc':abc}",
        FLOW_START:0,
        L_CURLY:1,
        SINGLE_QUOTED_LITERAL:5,
        COLON:1,
        PLAIN_LITERAL:3,
        R_CURLY:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_block_map_cant_have_adjacent_value() {
    assert_lex!(
        r#"
"key":value
"#,
        NEWLINE:1,
        FLOW_START:0,
        DOUBLE_QUOTED_LITERAL:5,
        FLOW_END:0,
        FLOW_START:0,
        // Since this plain token is not in any block scope, it consumes the succeed newline
        PLAIN_LITERAL:7,
        FLOW_END:0,
    );
}

#[test]
fn lex_flow_map_short_hand_empty_key_value() {
    assert_lex!(
        "[:]",
        FLOW_START:0,
        L_BRACK:1,
        COLON:1,
        R_BRACK:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_flow_get_out_of_block_sequence_scope() {
    assert_lex!(
        r#"
- [
abc]"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        L_BRACK:1,
        FLOW_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
        FLOW_START:0,
        PLAIN_LITERAL:4,
        FLOW_END:0,
    );
}

#[test]
fn lex_flow_collection_closing_bracket_on_sequence_start() {
    assert_lex!(
        r#"
- [abc
]"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
        FLOW_START:0,
        R_BRACK:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_comment_dont_break_out_of_flow_collection_scope() {
    assert_lex!(
        r#"
a:
  b: [
  # abc
   ]"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:2,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0
        L_BRACK:1,
        NEWLINE:1,
        WHITESPACE:2,
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:3,
        R_BRACK:1,
        FLOW_END:0
        MAPPING_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_unclosed_flow_collection() {
    assert_lex!(
        r#"
[:
abc:"#,
        NEWLINE:1,
        FLOW_START:0,
        L_BRACK:1,
        COLON:1,
        NEWLINE:1,
        PLAIN_LITERAL:3,
        COLON:1,
        FLOW_END:0,
    );
}

#[test]
fn lex_unclosed_flow_collection_inside_block_map() {
    assert_lex!(
        r#"
a:
  b:
    [abc"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:2,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:4,
        FLOW_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:3,
        FLOW_END:0
        MAPPING_END:0,
        MAPPING_END:0,
    );
}
