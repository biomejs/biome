use crate::assert_lex;

#[test]
fn lex_simple_mapping_key() {
    assert_lex!(
        "abc:",
        MAPPING_START:0,
        PLAIN_LITERAL:3,
        COLON:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_mapping_key_special_char() {
    assert_lex!(
        "ab,c:d e-[f:#gh: ",
        MAPPING_START:0,
        PLAIN_LITERAL:15,
        COLON:1,
        WHITESPACE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_unambigous_mapping_and_comment() {
    assert_lex!(
        "abc: #abc",
        MAPPING_START:0,
        PLAIN_LITERAL:3,
        COLON:1,
        WHITESPACE:1,
        COMMENT:4,
        MAPPING_END:0,
    );
}

#[test]
fn lex_explicit_mapping() {
    assert_lex!(
        r#"
? abc
: bc
? xyz"#,
        NEWLINE:1,
        MAPPING_START:0,
        QUESTION:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        NEWLINE:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:2,
        FLOW_END:0,
        NEWLINE:1,
        QUESTION:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_compact_mapping_in_sequence() {
    assert_lex!(
        r#"
- a: b
  c: d
  d: f
"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        NEWLINE:3,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        NEWLINE:3,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        MAPPING_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_nested_sequence() {
    assert_lex!(
        r#"
- - 10
  - 20
-
"#,
        NEWLINE:1
        SEQUENCE_START:0
        DASH:1
        WHITESPACE:1
        SEQUENCE_START:0
        DASH:1
        WHITESPACE:1
        FLOW_START:0
        PLAIN_LITERAL:2
        FLOW_END:0
        NEWLINE:3
        DASH:1
        WHITESPACE:1
        FLOW_START:0
        PLAIN_LITERAL:2
        FLOW_END:0
        SEQUENCE_END:0
        NEWLINE:1
        DASH:1
        SEQUENCE_END:0
        NEWLINE:1
    );
}

#[test]
fn lex_nested_compact_sequence() {
    assert_lex!(
        r#"
- - - - -
    -
-
"#,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        SEQUENCE_START:0,
        DASH:1,
        SEQUENCE_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
        WHITESPACE:4,
        DASH:1,
        SEQUENCE_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
        DASH:1,
        SEQUENCE_END:0,
        NEWLINE:1,
    );
}

#[test]
fn lex_mapping_empty_value() {
    assert_lex!(
        r#"
a:
b:
"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_flow_collection_as_key() {
    assert_lex!(
        r#"
[1, 2]: 345
{a: 1, b: 2}: 345
"#,
        NEWLINE:1
        MAPPING_START:0,
        L_BRACK:1,
        PLAIN_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        R_BRACK:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        NEWLINE:1
        L_CURLY:1
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        R_CURLY:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_block_map_empty_key() {
    assert_lex!(
        r#"
: 123
[1, 2]: 678
: 345
"#,
        NEWLINE:1,
        MAPPING_START:0,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        NEWLINE:1,
        L_BRACK:1,
        PLAIN_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        R_BRACK:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        NEWLINE:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}

#[test]
fn lex_nested_mapping() {
    assert_lex!(
        r#"
a:
  b:
  c:
    d:"#,
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
        WHITESPACE:2,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:4,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        MAPPING_END:0,
        MAPPING_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_sequence_followed_by_mapping_key2() {
    assert_lex!(
        r#"
xyz:
- abc
abc:
- abc
:
"#,
        NEWLINE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:3,
        COLON:1,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
        PLAIN_LITERAL:3,
        COLON:1,
        NEWLINE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        SEQUENCE_END:0,
        NEWLINE:1,
        COLON:1,
        NEWLINE:1
        MAPPING_END:0,
    );
}

#[test]
fn lex_invalid_mapping_key() {
    assert_lex!(
        r#"
]:
,:"#,
        NEWLINE:1,
        MAPPING_START:0,
        R_BRACK:1,
        COLON:1,
        NEWLINE:1,
        COMMA:1,
        COLON:1,
        MAPPING_END:0,
    );
}

// Not allowed by standard, but it might be easier to detect this in the syntax analyzer than in
// the lexer itself
#[test]
fn lex_same_line_nested_mapping() {
    assert_lex!(
        "a: b: c",
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0
        MAPPING_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_comment() {
    assert_lex!(
        "# this is a comment",
        COMMENT:19,
    );
}

#[test]
fn lex_comment_dont_break_out_of_block_scope() {
    assert_lex!(
        r#"
a:
  b:
# abc
  d:"#,
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
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:2,
        PLAIN_LITERAL:1,
        COLON:1,
        MAPPING_END:0,
        MAPPING_END:0,
    );
}

#[test]
fn lex_mapping_separated_by_trivia() {
    assert_lex!(
        r#"
? a: 10
# this is another trivia
c: 30
"#,
        NEWLINE:1,
        MAPPING_START:0,
        QUESTION:1,
        WHITESPACE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:2,
        FLOW_END:0,
        MAPPING_END:0,
        NEWLINE:1,
        COMMENT:24,
        NEWLINE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:2,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
    );
}
