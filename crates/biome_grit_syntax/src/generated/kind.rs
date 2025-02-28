//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum GritSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    DOT3,
    DOLLAR_UNDERSCORE,
    MATCH,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    PLUS,
    STAR,
    SLASH,
    PERCENT,
    DOT,
    COLON,
    EQ,
    EQ2,
    FAT_ARROW,
    BANG,
    NEQ,
    MINUS,
    LTEQ,
    GTEQ,
    PLUSEQ,
    BACKTICK,
    SEQUENTIAL_KW,
    MULTIFILE_KW,
    ENGINE_KW,
    LANGUAGE_KW,
    BIOME_KW,
    MARZANO_KW,
    JS_KW,
    CSS_KW,
    JSON_KW,
    GRIT_KW,
    HTML_KW,
    TYPESCRIPT_KW,
    JSX_KW,
    JS_DO_NOT_USE_KW,
    AS_KW,
    LIMIT_KW,
    WHERE_KW,
    ORELSE_KW,
    MAYBE_KW,
    AFTER_KW,
    BEFORE_KW,
    CONTAINS_KW,
    UNTIL_KW,
    INCLUDES_KW,
    IF_KW,
    ELSE_KW,
    WITHIN_KW,
    BUBBLE_KW,
    NOT_KW,
    OR_KW,
    AND_KW,
    ANY_KW,
    SOME_KW,
    EVERY_KW,
    PRIVATE_KW,
    PATTERN_KW,
    PREDICATE_KW,
    FUNCTION_KW,
    TRUE_KW,
    FALSE_KW,
    UNDEFINED_KW,
    LIKE_KW,
    RETURN_KW,
    GRIT_INT,
    GRIT_NEGATIVE_INT,
    GRIT_DOUBLE,
    GRIT_STRING,
    GRIT_REGEX,
    GRIT_SNIPPET_REGEX,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    MULTILINE_COMMENT,
    ERROR_TOKEN,
    GRIT_ANNOTATION,
    GRIT_BACKTICK_SNIPPET,
    GRIT_RAW_BACKTICK_SNIPPET,
    GRIT_NAME,
    GRIT_VARIABLE,
    GRIT_JAVASCRIPT_BODY,
    GRIT_BRACKETED_PATTERN,
    GRIT_BRACKETED_PREDICATE,
    GRIT_CURLY_PATTERN,
    GRIT_ROOT,
    GRIT_SEQUENTIAL,
    GRIT_FILES,
    GRIT_DEFINITION_LIST,
    GRIT_VERSION,
    GRIT_ENGINE_NAME,
    GRIT_LANGUAGE_DECLARATION,
    GRIT_LANGUAGE_FLAVOR,
    GRIT_LANGUAGE_FLAVOR_LIST,
    GRIT_LANGUAGE_FLAVOR_KIND,
    GRIT_PATTERN_LIST,
    GRIT_MUL_OPERATION,
    GRIT_DIV_OPERATION,
    GRIT_MOD_OPERATION,
    GRIT_ADD_OPERATION,
    GRIT_SUB_OPERATION,
    GRIT_PATTERN_AS,
    GRIT_PATTERN_LIMIT,
    GRIT_ASSIGNMENT_AS_PATTERN,
    GRIT_PATTERN_ACCUMULATE,
    GRIT_PATTERN_WHERE,
    GRIT_PATTERN_NOT,
    GRIT_PATTERN_OR,
    GRIT_PATTERN_OR_ELSE,
    GRIT_PATTERN_ANY,
    GRIT_PATTERN_AND,
    GRIT_PATTERN_MAYBE,
    GRIT_PATTERN_AFTER,
    GRIT_PATTERN_BEFORE,
    GRIT_PATTERN_CONTAINS,
    GRIT_PATTERN_UNTIL_CLAUSE,
    GRIT_PATTERN_INCLUDES,
    GRIT_REWRITE,
    GRIT_PATTERN_IF_ELSE,
    GRIT_PATTERN_ELSE_CLAUSE,
    GRIT_WITHIN,
    GRIT_BUBBLE_SCOPE,
    GRIT_BUBBLE,
    GRIT_NAMED_ARG,
    GRIT_NAMED_ARG_LIST,
    GRIT_NODE_LIKE,
    GRIT_LIKE,
    GRIT_LIKE_THRESHOLD,
    GRIT_MAP,
    GRIT_MAP_ELEMENT_LIST,
    GRIT_MAP_ELEMENT,
    GRIT_MAP_ACCESSOR,
    GRIT_LIST,
    GRIT_LIST_PATTERN_LIST,
    GRIT_LIST_ACCESSOR,
    GRIT_DOT,
    GRIT_DOTDOTDOT,
    GRIT_SOME,
    GRIT_EVERY,
    GRIT_REGEX_PATTERN,
    GRIT_REGEX_PATTERN_VARIABLES,
    GRIT_PATTERN_DEFINITION_BODY,
    GRIT_PATTERN_DEFINITION,
    GRIT_PATTERN_ARG_LIST,
    GRIT_PREDICATE_LIST,
    GRIT_PREDICATE_CURLY,
    GRIT_PREDICATE_DEFINITION,
    GRIT_FUNCTION_DEFINITION,
    GRIT_JAVASCRIPT_FUNCTION_DEFINITION,
    GRIT_JAVASCRIPT_BODY_WRAPPER,
    GRIT_PREDICATE_NOT,
    GRIT_PREDICATE_MAYBE,
    GRIT_PREDICATE_AND,
    GRIT_PREDICATE_OR,
    GRIT_PREDICATE_ANY,
    GRIT_PREDICATE_IF_ELSE,
    GRIT_PREDICATE_ELSE_CLAUSE,
    GRIT_PREDICATE_REWRITE,
    GRIT_PREDICATE_ASSIGNMENT,
    GRIT_PREDICATE_ACCUMULATE,
    GRIT_PREDICATE_GREATER,
    GRIT_PREDICATE_LESS,
    GRIT_PREDICATE_GREATER_EQUAL,
    GRIT_PREDICATE_LESS_EQUAL,
    GRIT_PREDICATE_NOT_EQUAL,
    GRIT_PREDICATE_EQUAL,
    GRIT_PREDICATE_MATCH,
    GRIT_PREDICATE_CALL,
    GRIT_PREDICATE_RETURN,
    GRIT_VARIABLE_LIST,
    GRIT_LANGUAGE_NAME,
    GRIT_LANGUAGE_SPECIFIC_SNIPPET,
    GRIT_CODE_SNIPPET,
    GRIT_NOT,
    GRIT_UNDERSCORE,
    GRIT_BACKTICK_SNIPPET_LITERAL,
    GRIT_BOOLEAN_LITERAL,
    GRIT_UNDEFINED_LITERAL,
    GRIT_INT_LITERAL,
    GRIT_NEGATIVE_INT_LITERAL,
    GRIT_DOUBLE_LITERAL,
    GRIT_STRING_LITERAL,
    GRIT_RAW_BACKTICK_SNIPPET_LITERAL,
    GRIT_REGEX_LITERAL,
    GRIT_SNIPPET_REGEX_LITERAL,
    GRIT_BOGUS,
    GRIT_BOGUS_CONTAINER,
    GRIT_BOGUS_DEFINITION,
    GRIT_BOGUS_MAP_ELEMENT,
    GRIT_BOGUS_LANGUAGE_DECLARATION,
    GRIT_BOGUS_LANGUAGE_FLAVOR_KIND,
    GRIT_BOGUS_LANGUAGE_NAME,
    GRIT_BOGUS_LITERAL,
    GRIT_BOGUS_NAMED_ARG,
    GRIT_BOGUS_PATTERN,
    GRIT_BOGUS_PREDICATE,
    GRIT_BOGUS_VERSION,
    #[doc(hidden)]
    __LAST,
}
use self::GritSyntaxKind::*;
impl GritSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            DOT3 | DOLLAR_UNDERSCORE
                | MATCH
                | SEMICOLON
                | COMMA
                | L_PAREN
                | R_PAREN
                | L_CURLY
                | R_CURLY
                | L_BRACK
                | R_BRACK
                | L_ANGLE
                | R_ANGLE
                | PLUS
                | STAR
                | SLASH
                | PERCENT
                | DOT
                | COLON
                | EQ
                | EQ2
                | FAT_ARROW
                | BANG
                | NEQ
                | MINUS
                | LTEQ
                | GTEQ
                | PLUSEQ
                | BACKTICK
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            GRIT_INT
                | GRIT_NEGATIVE_INT
                | GRIT_DOUBLE
                | GRIT_STRING
                | GRIT_REGEX
                | GRIT_SNIPPET_REGEX
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            GRIT_DEFINITION_LIST
                | GRIT_LANGUAGE_FLAVOR_LIST
                | GRIT_PATTERN_LIST
                | GRIT_NAMED_ARG_LIST
                | GRIT_MAP_ELEMENT_LIST
                | GRIT_LIST
                | GRIT_LIST_PATTERN_LIST
                | GRIT_PATTERN_ARG_LIST
                | GRIT_PREDICATE_LIST
                | GRIT_VARIABLE_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<GritSyntaxKind> {
        let kw = match ident {
            "sequential" => SEQUENTIAL_KW,
            "multifile" => MULTIFILE_KW,
            "engine" => ENGINE_KW,
            "language" => LANGUAGE_KW,
            "biome" => BIOME_KW,
            "marzano" => MARZANO_KW,
            "js" => JS_KW,
            "css" => CSS_KW,
            "json" => JSON_KW,
            "grit" => GRIT_KW,
            "html" => HTML_KW,
            "typescript" => TYPESCRIPT_KW,
            "jsx" => JSX_KW,
            "js_do_not_use" => JS_DO_NOT_USE_KW,
            "as" => AS_KW,
            "limit" => LIMIT_KW,
            "where" => WHERE_KW,
            "orelse" => ORELSE_KW,
            "maybe" => MAYBE_KW,
            "after" => AFTER_KW,
            "before" => BEFORE_KW,
            "contains" => CONTAINS_KW,
            "until" => UNTIL_KW,
            "includes" => INCLUDES_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            "within" => WITHIN_KW,
            "bubble" => BUBBLE_KW,
            "not" => NOT_KW,
            "or" => OR_KW,
            "and" => AND_KW,
            "any" => ANY_KW,
            "some" => SOME_KW,
            "every" => EVERY_KW,
            "private" => PRIVATE_KW,
            "pattern" => PATTERN_KW,
            "predicate" => PREDICATE_KW,
            "function" => FUNCTION_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "undefined" => UNDEFINED_KW,
            "like" => LIKE_KW,
            "return" => RETURN_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            DOT3 => "...",
            DOLLAR_UNDERSCORE => "$_",
            MATCH => "<:",
            SEMICOLON => ";",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            PLUS => "+",
            STAR => "*",
            SLASH => "/",
            PERCENT => "%",
            DOT => ".",
            COLON => ":",
            EQ => "=",
            EQ2 => "==",
            FAT_ARROW => "=>",
            BANG => "!",
            NEQ => "!=",
            MINUS => "-",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            BACKTICK => "`",
            SEQUENTIAL_KW => "sequential",
            MULTIFILE_KW => "multifile",
            ENGINE_KW => "engine",
            LANGUAGE_KW => "language",
            BIOME_KW => "biome",
            MARZANO_KW => "marzano",
            JS_KW => "js",
            CSS_KW => "css",
            JSON_KW => "json",
            GRIT_KW => "grit",
            HTML_KW => "html",
            TYPESCRIPT_KW => "typescript",
            JSX_KW => "jsx",
            JS_DO_NOT_USE_KW => "js_do_not_use",
            AS_KW => "as",
            LIMIT_KW => "limit",
            WHERE_KW => "where",
            ORELSE_KW => "orelse",
            MAYBE_KW => "maybe",
            AFTER_KW => "after",
            BEFORE_KW => "before",
            CONTAINS_KW => "contains",
            UNTIL_KW => "until",
            INCLUDES_KW => "includes",
            IF_KW => "if",
            ELSE_KW => "else",
            WITHIN_KW => "within",
            BUBBLE_KW => "bubble",
            NOT_KW => "not",
            OR_KW => "or",
            AND_KW => "and",
            ANY_KW => "any",
            SOME_KW => "some",
            EVERY_KW => "every",
            PRIVATE_KW => "private",
            PATTERN_KW => "pattern",
            PREDICATE_KW => "predicate",
            FUNCTION_KW => "function",
            TRUE_KW => "true",
            FALSE_KW => "false",
            UNDEFINED_KW => "undefined",
            LIKE_KW => "like",
            RETURN_KW => "return",
            GRIT_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [...] => { $ crate :: GritSyntaxKind :: DOT3 } ; ["$_"] => { $ crate :: GritSyntaxKind :: DOLLAR_UNDERSCORE } ; [<:] => { $ crate :: GritSyntaxKind :: MATCH } ; [;] => { $ crate :: GritSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: GritSyntaxKind :: COMMA } ; ['('] => { $ crate :: GritSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: GritSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: GritSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: GritSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: GritSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: GritSyntaxKind :: R_BRACK } ; [<] => { $ crate :: GritSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: GritSyntaxKind :: R_ANGLE } ; [+] => { $ crate :: GritSyntaxKind :: PLUS } ; [*] => { $ crate :: GritSyntaxKind :: STAR } ; [/] => { $ crate :: GritSyntaxKind :: SLASH } ; [%] => { $ crate :: GritSyntaxKind :: PERCENT } ; [.] => { $ crate :: GritSyntaxKind :: DOT } ; [:] => { $ crate :: GritSyntaxKind :: COLON } ; [=] => { $ crate :: GritSyntaxKind :: EQ } ; [==] => { $ crate :: GritSyntaxKind :: EQ2 } ; [=>] => { $ crate :: GritSyntaxKind :: FAT_ARROW } ; [!] => { $ crate :: GritSyntaxKind :: BANG } ; [!=] => { $ crate :: GritSyntaxKind :: NEQ } ; [-] => { $ crate :: GritSyntaxKind :: MINUS } ; [<=] => { $ crate :: GritSyntaxKind :: LTEQ } ; [>=] => { $ crate :: GritSyntaxKind :: GTEQ } ; [+=] => { $ crate :: GritSyntaxKind :: PLUSEQ } ; ['`'] => { $ crate :: GritSyntaxKind :: BACKTICK } ; [sequential] => { $ crate :: GritSyntaxKind :: SEQUENTIAL_KW } ; [multifile] => { $ crate :: GritSyntaxKind :: MULTIFILE_KW } ; [engine] => { $ crate :: GritSyntaxKind :: ENGINE_KW } ; [language] => { $ crate :: GritSyntaxKind :: LANGUAGE_KW } ; [biome] => { $ crate :: GritSyntaxKind :: BIOME_KW } ; [marzano] => { $ crate :: GritSyntaxKind :: MARZANO_KW } ; [js] => { $ crate :: GritSyntaxKind :: JS_KW } ; [css] => { $ crate :: GritSyntaxKind :: CSS_KW } ; [json] => { $ crate :: GritSyntaxKind :: JSON_KW } ; [grit] => { $ crate :: GritSyntaxKind :: GRIT_KW } ; [html] => { $ crate :: GritSyntaxKind :: HTML_KW } ; [typescript] => { $ crate :: GritSyntaxKind :: TYPESCRIPT_KW } ; [jsx] => { $ crate :: GritSyntaxKind :: JSX_KW } ; [js_do_not_use] => { $ crate :: GritSyntaxKind :: JS_DO_NOT_USE_KW } ; [as] => { $ crate :: GritSyntaxKind :: AS_KW } ; [limit] => { $ crate :: GritSyntaxKind :: LIMIT_KW } ; [where] => { $ crate :: GritSyntaxKind :: WHERE_KW } ; [orelse] => { $ crate :: GritSyntaxKind :: ORELSE_KW } ; [maybe] => { $ crate :: GritSyntaxKind :: MAYBE_KW } ; [after] => { $ crate :: GritSyntaxKind :: AFTER_KW } ; [before] => { $ crate :: GritSyntaxKind :: BEFORE_KW } ; [contains] => { $ crate :: GritSyntaxKind :: CONTAINS_KW } ; [until] => { $ crate :: GritSyntaxKind :: UNTIL_KW } ; [includes] => { $ crate :: GritSyntaxKind :: INCLUDES_KW } ; [if] => { $ crate :: GritSyntaxKind :: IF_KW } ; [else] => { $ crate :: GritSyntaxKind :: ELSE_KW } ; [within] => { $ crate :: GritSyntaxKind :: WITHIN_KW } ; [bubble] => { $ crate :: GritSyntaxKind :: BUBBLE_KW } ; [not] => { $ crate :: GritSyntaxKind :: NOT_KW } ; [or] => { $ crate :: GritSyntaxKind :: OR_KW } ; [and] => { $ crate :: GritSyntaxKind :: AND_KW } ; [any] => { $ crate :: GritSyntaxKind :: ANY_KW } ; [some] => { $ crate :: GritSyntaxKind :: SOME_KW } ; [every] => { $ crate :: GritSyntaxKind :: EVERY_KW } ; [private] => { $ crate :: GritSyntaxKind :: PRIVATE_KW } ; [pattern] => { $ crate :: GritSyntaxKind :: PATTERN_KW } ; [predicate] => { $ crate :: GritSyntaxKind :: PREDICATE_KW } ; [function] => { $ crate :: GritSyntaxKind :: FUNCTION_KW } ; [true] => { $ crate :: GritSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: GritSyntaxKind :: FALSE_KW } ; [undefined] => { $ crate :: GritSyntaxKind :: UNDEFINED_KW } ; [like] => { $ crate :: GritSyntaxKind :: LIKE_KW } ; [return] => { $ crate :: GritSyntaxKind :: RETURN_KW } ; [ident] => { $ crate :: GritSyntaxKind :: IDENT } ; [EOF] => { $ crate :: GritSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: GritSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: GritSyntaxKind :: HASH } ; }
