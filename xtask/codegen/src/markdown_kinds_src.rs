use crate::kind_src::KindsSrc;

// 对Token进行分组，生成is_xx 方法
pub const MARKDOWN_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[("+", "PLUS")],
    keywords: &["calc"],
    literals: &["NUMBER_LITERAL"],
    tokens: &["COMMENT", "NEWLINE", "WHITESPACE","ERROR_TOKEN"],
    nodes: &[
        "ANY_VALUE",
        "ROOT",
        // Bogus nodes
        "BOGUS",
        "NUMBER_VALUE",
        "NUMBER_VALUE_LIST",
    ],
};
