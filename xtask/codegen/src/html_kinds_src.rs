use crate::kind_src::KindsSrc;

pub const HTML_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("/", "SLASH"),
        ("=", "EQ"),
        ("!", "BANG"),
    ],
    keywords: &["null", "true", "false"],
    literals: &["HTML_STRING_LITERAL"],
    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "COMMENT",
        "HTML_IDENT",
    ],
    nodes: &[
        "HTML_ROOT",
        "HTML_DIRECTIVE",
        "HTML_SELF_CLOSING_TAG",
        "HTML_ELEMENT",
        "HTML_OPENING_ELEMENT",
        "HTML_CLOSING_ELEMENT",
        "HTML_SELF_CLOSING_ELEMENT",
        "HTML_ATTRIBUTE",
        "HTML_ATTRIBUTE_INITIALIZER_CLAUSE",
        "HTML_STRING",
        "HTML_NAME",
        "HTML_ELEMENT_LIST",
        "HTML_ATTRIBUTE_LIST",
        // Bogus nodes
        "HTML_BOGUS",
    ],
};
