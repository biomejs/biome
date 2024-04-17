use crate::kind_src::KindsSrc;

pub const YAML_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (":", "COLON"),
        (",", "COMMA"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("-", "DASH"),
        ("%", "PERCENT"),
        ("*", "STAR"),
        ("#", "HASH"),
        ("!", "BANG"),
        ("@", "AT"),
        ("<<", "SHL"),
        ("&", "AMP"),
        ("|", "PIPE"),
        (">", "R_ANGLE"),
        ("~", "TILDE"),
        ("`", "BACKTICK"),
        ("---", "DOC_START"),
        ("...", "DOC_END"),
    ],
    keywords: &["null"],
    literals: &["YAML_STRING_LITERAL", "YAML_SCALAR"],
    tokens: &["NEWLINE", "WHITESPACE", "IDENT", "COMMENT"],
    nodes: &[
        "YAML_ROOT",
        "YAML_DOCUMENT_LIST",
        "YAML_DOCUMENT",
        "YAML_CONTENT_LIST",
        // Bogus nodes
        "YAML_BOGUS",
        "YAML_BOGUS_VALUE",
    ],
};
