use crate::kind_src::KindsSrc;

pub const TAILWIND_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("/", "SLASH"),
        ("!", "BANG"),
        ("-", "DASH"),
        ("+", "PLUS"),
        ("*", "STAR"),
        (",", "COMMA"),
        (".", "DOT"),
        (":", "COLON"),
        ("=", "EQ"),
        ("#", "HASH"),
        ("%", "PERCENT"),
        ("[", "L_BRACKET"),
        ("]", "R_BRACKET"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        (" ", "WHITESPACE"),
    ],
    literals: &[
        "TW_BASE",
        "TW_VALUE",
        "TW_SELECTOR",
        "TW_PROPERTY",
        "CSS_STRING_LITERAL",
        "CSS_NUMBER_LITERAL",
        "CSS_DIMENSION_VALUE",
        "CSS_PERCENTAGE_VALUE",
        "CSS_COLOR_LITERAL",
        "CSS_URL_VALUE_RAW_LITERAL",
    ],
    tokens: &["ERROR_TOKEN", "IDENT", "NEWLINE"],
    keywords: &[
        "data", "url", "var", // length units
        "em", "rem", "ex", "rex", "cap", "rcap", "ch", "rch", "ic", "ric", "lh", "rlh",
        // Viewport-percentage Lengths
        "vw", "svw", "lvw", "dvw", "vh", "svh", "lvh", "dvh", "vi", "svi", "lvi", "dvi", "vb",
        "svb", "lvb", "dvb", "vmin", "svmin", "lvmin", "dvmin", "vmax", "svmax", "lvmax",
        "dvmax", // Absolute lengths
        "cm", "mm", "q", "in", "pc", "pt", "px", "mozmm", // mini app
        "rpx",   // container lengths
        "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax", // angle units
        "deg", "grad", "rad", "turn", // time units
        "s", "ms", // frequency units
        "hz", "khz", // resolution units
        "dpi", "dpcm", "dppx", "x", // flex units
        "fr",
    ],
    nodes: &[
        "TW_ROOT",
        "TW_CANDIDATE_LIST",
        "TW_FULL_CANDIDATE",
        "TW_ARBITRARY_CANDIDATE",
        "TW_STATIC_CANDIDATE",
        "TW_FUNCTIONAL_CANDIDATE",
        "TW_VARIANT_LIST",
        "TW_ARBITRARY_VARIANT",
        "TW_STATIC_VARIANT",
        "TW_FUNCTIONAL_VARIANT",
        "TW_NAMED_VALUE",
        "TW_ARBITRARY_VALUE",
        "TW_CSS_VARIABLE_VALUE",
        "TW_MODIFIER",
        "TW_DATA_ATTRIBUTE",
        // "TW_DATA_ATTRIBUTE_ARBITRARY_VALUE",
        // Bogus nodes
        "TW_BOGUS",
        "TW_BOGUS_CANDIDATE",
        "TW_BOGUS_VARIANT",
        "TW_BOGUS_MODIFIER",
        "TW_BOGUS_VALUE",
        "CSS_BOGUS_PROPERTY_VALUE",
        // CSS value nodes embedded in arbitrary values
        "CSS_IDENTIFIER",
        "CSS_DASHED_IDENTIFIER",
        "CSS_STRING",
        "CSS_NUMBER",
        "CSS_PERCENTAGE",
        "CSS_RATIO",
        "CSS_FUNCTION",
        "CSS_URL_FUNCTION",
        "CSS_URL_VALUE_RAW",
        "CSS_PARAMETER_LIST",
        "CSS_COMPONENT_VALUE_LIST",
        "CSS_GENERIC_COMPONENT_VALUE_LIST",
        "CSS_GENERIC_DELIMITER",
        "CSS_REGULAR_DIMENSION",
        "CSS_UNKNOWN_DIMENSION",
        "CSS_PARENTHESIZED_EXPRESSION",
        "CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION",
        "CSS_BINARY_EXPRESSION",
        "CSS_UNARY_EXPRESSION",
        "CSS_COLOR",
    ],
};
