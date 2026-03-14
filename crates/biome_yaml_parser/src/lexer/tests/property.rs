use crate::assert_lex;

#[test]
fn anchor_property() {
    assert_lex!("&anchor",
        FLOW_START:0,
        ANCHOR_PROPERTY_LITERAL:7,
        FLOW_END:0
    );
}

#[test]
fn tag_property() {
    assert_lex!("!tag",
        FLOW_START:0,
        TAG_PROPERTY_LITERAL:4,
        FLOW_END:0
    );
}

#[test]
fn anchor_with_value() {
    assert_lex!("&anchor value",
        FLOW_START:0,
        ANCHOR_PROPERTY_LITERAL:7,
        WHITESPACE:1,
        PLAIN_LITERAL:5,
        FLOW_END:0
    );
}

#[test]
fn multiple_properties() {
    assert_lex!("&anchor !tag abc",
        FLOW_START:0,
        ANCHOR_PROPERTY_LITERAL:7,
        WHITESPACE:1,
        TAG_PROPERTY_LITERAL:4,
        WHITESPACE:1,
        PLAIN_LITERAL:3,
        FLOW_END:0
    );
}

#[test]
fn block_map_property() {
    assert_lex!(r#"a:
   &abc
  !abc
 b: c"#,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:3,
        ANCHOR_PROPERTY_LITERAL:4,
        NEWLINE:1,
        WHITESPACE:2,
        TAG_PROPERTY_LITERAL:4,
        NEWLINE:1,
        WHITESPACE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        MAPPING_END:0,
        MAPPING_END:0
    );
}

#[test]
fn block_sequence_property() {
    assert_lex!(r#"a:
  !abc
 - 123"#,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:2,
        TAG_PROPERTY_LITERAL:4,
        NEWLINE:1,
        WHITESPACE:1,
        SEQUENCE_START:0,
        DASH:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        SEQUENCE_END:0,
        MAPPING_END:0
    );
}

#[test]
fn property_in_map_key() {
    assert_lex!(r#"a:
 !abc b: c
 d: e"#,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:1,
        MAPPING_START:0,
        TAG_PROPERTY_LITERAL:4,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        NEWLINE:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        MAPPING_END:0,
        MAPPING_END:0
    );
}

#[test]
fn property_in_map_key_multiline() {
    assert_lex!(r#"a:
  &abc
 !abc
 b: c
 d: e"#,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:2,
        MAPPING_START:0,
        ANCHOR_PROPERTY_LITERAL:4,
        NEWLINE:1,
        WHITESPACE:1,
        TAG_PROPERTY_LITERAL:4,
        NEWLINE:1,
        WHITESPACE:1,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        MAPPING_END:0,
        NEWLINE:1,
        WHITESPACE:1,
        MAPPING_START:0,
        PLAIN_LITERAL:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:1,
        FLOW_END:0,
        MAPPING_END:0,
        MAPPING_END:0
    );
}

#[test]
fn property_for_empty_map_key() {
    assert_lex!(r#"&prop : val"#,
        MAPPING_START:0,
        ANCHOR_PROPERTY_LITERAL:5,
        WHITESPACE:1,
        COLON:1,
        WHITESPACE:1,
        FLOW_START:0,
        PLAIN_LITERAL:3,
        FLOW_END:0,
        MAPPING_END:0,
    );
}
