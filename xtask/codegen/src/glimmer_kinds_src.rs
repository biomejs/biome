use crate::kind_src::KindsSrc;

pub const GLIMMER_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("{{", "L_CURLY2"),
        ("}}", "R_CURLY2"),
        ("/>", "SELF_CLOSING"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("/", "SLASH"),
        ("#", "HASH"),
        ("@", "AT"),
        (".", "DOT"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("|", "PIPE"),
        (",", "COMMA"),
    ],
    keywords: &["null", "true", "false", "undefined", "this", "else"],
    literals: &[
        "GLIMMER_STRING_LITERAL",
        "GLIMMER_NUMBER_LITERAL",
        "GLIMMER_TEXT",
    ],
    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "GLIMMER_COMMENT",
    ],
    nodes: &[
        // Root
        "GLIMMER_ROOT",
        "GLIMMER_STATEMENT_LIST",
        // Statements
        "GLIMMER_ELEMENT_NODE",
        "GLIMMER_TEXT_NODE",
        "GLIMMER_MUSTACHE_STATEMENT",
        "GLIMMER_BLOCK_STATEMENT",
        "GLIMMER_COMMENT_STATEMENT",
        // Element parts
        "GLIMMER_OPENING_ELEMENT",
        "GLIMMER_CLOSING_ELEMENT",
        "GLIMMER_ATTRIBUTE",
        "GLIMMER_NAMED_ARGUMENT",
        "GLIMMER_ATTRIBUTE_LIST",
        "GLIMMER_ELEMENT_MODIFIER_STATEMENT",
        "GLIMMER_MODIFIER_LIST",
        "GLIMMER_CONCAT_STATEMENT",
        "GLIMMER_CONCAT_PART_LIST",
        // Block parts
        "GLIMMER_BLOCK_OPENING",
        "GLIMMER_BLOCK",
        "GLIMMER_BLOCK_PARAM_LIST",
        "GLIMMER_BLOCK_INVERSE",
        "GLIMMER_BLOCK_CLOSING",
        // Expressions
        "GLIMMER_PATH_EXPRESSION",
        "GLIMMER_SUB_EXPRESSION",
        "GLIMMER_EXPRESSION_LIST",
        // Path parts
        "GLIMMER_THIS_HEAD",
        "GLIMMER_AT_HEAD",
        "GLIMMER_VAR_HEAD",
        "GLIMMER_PATH_TAIL",
        // Hash
        "GLIMMER_HASH",
        "GLIMMER_HASH_PAIR",
        "GLIMMER_HASH_PAIR_LIST",
        // Literals
        "GLIMMER_STRING_LITERAL",
        "GLIMMER_NUMBER_LITERAL",
        "GLIMMER_BOOLEAN_LITERAL",
        "GLIMMER_NULL_LITERAL",
        "GLIMMER_UNDEFINED_LITERAL",
        // Identifier
        "GLIMMER_IDENTIFIER",
        // Bogus nodes
        "GLIMMER_BOGUS",
        "GLIMMER_BOGUS_STATEMENT",
        "GLIMMER_BOGUS_EXPRESSION",
        "GLIMMER_BOGUS_ATTRIBUTE",
    ],
};
