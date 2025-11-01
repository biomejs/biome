use crate::assert_lex;

#[test]
fn lex_doc_end() {
    assert_lex!(
        "...",
        DOC_END:3,
    );
}

#[test]
fn lex_doc_end_followed_by_trivia() {
    assert_lex!(
        "... # trivia",
        DOC_END:3,
        WHITESPACE:1,
        COMMENT:8,
    );
}

#[test]
fn lex_doc_end_followed_unexpected_token() {
    assert_lex!(
        "... 10",
        DOC_END:3,
        WHITESPACE:1,
        ERROR_TOKEN:2,
    );
}

#[test]
fn lex_doc_end_close_previous_document() {
    assert_lex!(
        r#"a: b
...
"#,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        NEWLINE:1,
        MAPPING_END:0,
        DOC_END:3,
        NEWLINE:1,
    );
}
