use crate::kind_src::KindsSrc;

pub const JSON_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (":", "COLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
    ],
    keywords: &["null", "true", "false"],
    literals: &["JSON_STRING_LITERAL", "JSON_NUMBER_LITERAL"],
    // keep comment token for json 5 extension
    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "COMMENT",
        "MULTILINE_COMMENT",
    ],
    nodes: &[
        "JSON_ROOT",
        "JSON_NUMBER_VALUE",
        "JSON_STRING_VALUE",
        "JSON_BOOLEAN_VALUE",
        "JSON_NULL_VALUE",
        "JSON_ARRAY_VALUE",
        "JSON_OBJECT_VALUE",
        "JSON_MEMBER_LIST",
        "JSON_MEMBER",
        "JSON_MEMBER_NAME",
        "JSON_ARRAY_ELEMENT_LIST",
        // Bogus nodes
        "JSON_BOGUS",
        "JSON_BOGUS_MEMBER_NAME",
        "JSON_BOGUS_VALUE",
    ],
};
