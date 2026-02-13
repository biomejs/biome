//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum HtmlSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    L_ANGLE,
    R_ANGLE,
    SLASH,
    EQ,
    BANG,
    MINUS,
    CDATA_START,
    CDATA_END,
    FENCE,
    L_CURLY,
    R_CURLY,
    L_DOUBLE_CURLY,
    R_DOUBLE_CURLY,
    SV_CURLY_AT,
    SV_CURLY_HASH,
    SV_CURLY_SLASH,
    SV_CURLY_COLON,
    COMMA,
    COLON,
    AT,
    DOT,
    L_BRACKET,
    R_BRACKET,
    HASH,
    L_PAREN,
    R_PAREN,
    DOT3,
    PIPE,
    NULL_KW,
    TRUE_KW,
    FALSE_KW,
    DOCTYPE_KW,
    HTML_KW,
    DEBUG_KW,
    KEY_KW,
    RENDER_KW,
    CONST_KW,
    ATTACH_KW,
    ELSE_KW,
    IF_KW,
    AS_KW,
    EACH_KW,
    THEN_KW,
    AWAIT_KW,
    CATCH_KW,
    SNIPPET_KW,
    BIND_KW,
    TRANSITION_KW,
    USE_KW,
    ANIMATE_KW,
    IN_KW,
    OUT_KW,
    STYLE_KW,
    CLASS_KW,
    HTML_STRING_LITERAL,
    HTML_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    HTML_ROOT,
    HTML_DIRECTIVE,
    HTML_SELF_CLOSING_TAG,
    HTML_ELEMENT,
    HTML_OPENING_ELEMENT,
    HTML_CLOSING_ELEMENT,
    HTML_SELF_CLOSING_ELEMENT,
    HTML_ATTRIBUTE,
    HTML_ATTRIBUTE_INITIALIZER_CLAUSE,
    HTML_STRING,
    HTML_TAG_NAME,
    HTML_COMPONENT_NAME,
    HTML_MEMBER_NAME,
    HTML_ATTRIBUTE_NAME,
    HTML_ELEMENT_LIST,
    HTML_ATTRIBUTE_LIST,
    HTML_CONTENT,
    HTML_EMBEDDED_CONTENT,
    HTML_CDATA_SECTION,
    COMMENT,
    HTML_DOUBLE_TEXT_EXPRESSION,
    HTML_SINGLE_TEXT_EXPRESSION,
    HTML_TEXT_EXPRESSION,
    HTML_SPREAD_ATTRIBUTE,
    HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION,
    HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION,
    ASTRO_FRONTMATTER_ELEMENT,
    ASTRO_EMBEDDED_CONTENT,
    SVELTE_DEBUG_BLOCK,
    SVELTE_KEY_BLOCK,
    SVELTE_KEY_OPENING_BLOCK,
    SVELTE_KEY_CLOSING_BLOCK,
    SVELTE_BINDING_LIST,
    SVELTE_NAME,
    SVELTE_RENDER_BLOCK,
    SVELTE_ATTACH_ATTRIBUTE,
    SVELTE_HTML_BLOCK,
    SVELTE_CONST_BLOCK,
    SVELTE_IF_BLOCK,
    SVELTE_IF_OPENING_BLOCK,
    SVELTE_ELSE_IF_CLAUSE_LIST,
    SVELTE_ELSE_CLAUSE,
    SVELTE_IF_CLOSING_BLOCK,
    SVELTE_ELSE_IF_CLAUSE,
    SVELTE_EACH_BLOCK,
    SVELTE_EACH_OPENING_BLOCK,
    SVELTE_EACH_AS_KEYED_ITEM,
    SVELTE_EACH_KEYED_ITEM,
    SVELTE_EACH_INDEX,
    SVELTE_EACH_KEY,
    SVELTE_EACH_CLOSING_BLOCK,
    SVELTE_AWAIT_BLOCK,
    SVELTE_AWAIT_OPENING_BLOCK,
    SVELTE_AWAIT_THEN_BLOCK,
    SVELTE_AWAIT_CLAUSES_LIST,
    SVELTE_AWAIT_CATCH_BLOCK,
    SVELTE_AWAIT_CLOSING_BLOCK,
    SVELTE_AWAIT_THEN_CLAUSE,
    SVELTE_AWAIT_CATCH_CLAUSE,
    SVELTE_SNIPPET_BLOCK,
    SVELTE_SNIPPET_OPENING_BLOCK,
    SVELTE_SNIPPET_CLOSING_BLOCK,
    SVELTE_CURLY_DESTRUCTURED_NAME,
    SVELTE_SQUARE_DESTRUCTURED_NAME,
    SVELTE_BINDING_ASSIGNMENT_BINDING_LIST,
    SVELTE_REST_BINDING,
    SVELTE_BIND_DIRECTIVE,
    SVELTE_TRANSITION_DIRECTIVE,
    SVELTE_IN_DIRECTIVE,
    SVELTE_OUT_DIRECTIVE,
    SVELTE_USE_DIRECTIVE,
    SVELTE_ANIMATE_DIRECTIVE,
    SVELTE_STYLE_DIRECTIVE,
    SVELTE_CLASS_DIRECTIVE,
    SVELTE_DIRECTIVE_VALUE,
    SVELTE_DIRECTIVE_MODIFIER,
    SVELTE_DIRECTIVE_MODIFIER_LIST,
    SVELTE_LITERAL,
    VUE_DIRECTIVE,
    VUE_DIRECTIVE_ARGUMENT,
    VUE_V_BIND_SHORTHAND_DIRECTIVE,
    VUE_V_ON_SHORTHAND_DIRECTIVE,
    VUE_V_SLOT_SHORTHAND_DIRECTIVE,
    VUE_STATIC_ARGUMENT,
    VUE_DYNAMIC_ARGUMENT,
    VUE_MODIFIER_LIST,
    VUE_MODIFIER,
    HTML_BOGUS,
    HTML_BOGUS_ELEMENT,
    HTML_BOGUS_ATTRIBUTE,
    HTML_BOGUS_TEXT_EXPRESSION,
    ASTRO_BOGUS_FRONTMATTER,
    SVELTE_BOGUS_BLOCK,
    VUE_BOGUS_DIRECTIVE,
    VUE_BOGUS_DIRECTIVE_ARGUMENT,
    #[doc(hidden)]
    __LAST,
}
use self::HtmlSyntaxKind::*;
impl HtmlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            L_ANGLE
                | R_ANGLE
                | SLASH
                | EQ
                | BANG
                | MINUS
                | CDATA_START
                | CDATA_END
                | FENCE
                | L_CURLY
                | R_CURLY
                | L_DOUBLE_CURLY
                | R_DOUBLE_CURLY
                | SV_CURLY_AT
                | SV_CURLY_HASH
                | SV_CURLY_SLASH
                | SV_CURLY_COLON
                | COMMA
                | COLON
                | AT
                | DOT
                | L_BRACKET
                | R_BRACKET
                | HASH
                | L_PAREN
                | R_PAREN
                | DOT3
                | PIPE
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(self, HTML_STRING_LITERAL | HTML_LITERAL)
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            HTML_ELEMENT_LIST
                | HTML_ATTRIBUTE_LIST
                | SVELTE_BINDING_LIST
                | SVELTE_ELSE_IF_CLAUSE_LIST
                | SVELTE_AWAIT_CLAUSES_LIST
                | SVELTE_BINDING_ASSIGNMENT_BINDING_LIST
                | SVELTE_DIRECTIVE_MODIFIER_LIST
                | VUE_MODIFIER_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<Self> {
        let kw = match ident {
            "null" => NULL_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "doctype" => DOCTYPE_KW,
            "html" => HTML_KW,
            "debug" => DEBUG_KW,
            "key" => KEY_KW,
            "render" => RENDER_KW,
            "const" => CONST_KW,
            "attach" => ATTACH_KW,
            "else" => ELSE_KW,
            "if" => IF_KW,
            "as" => AS_KW,
            "each" => EACH_KW,
            "then" => THEN_KW,
            "await" => AWAIT_KW,
            "catch" => CATCH_KW,
            "snippet" => SNIPPET_KW,
            "bind" => BIND_KW,
            "transition" => TRANSITION_KW,
            "use" => USE_KW,
            "animate" => ANIMATE_KW,
            "in" => IN_KW,
            "out" => OUT_KW,
            "style" => STYLE_KW,
            "class" => CLASS_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_ANGLE => "<",
            R_ANGLE => ">",
            SLASH => "/",
            EQ => "=",
            BANG => "!",
            MINUS => "-",
            CDATA_START => "<![CDATA[",
            CDATA_END => "]]>",
            FENCE => "---",
            L_CURLY => "{",
            R_CURLY => "}",
            L_DOUBLE_CURLY => "{{",
            R_DOUBLE_CURLY => "}}",
            SV_CURLY_AT => "{@",
            SV_CURLY_HASH => "{#",
            SV_CURLY_SLASH => "{/",
            SV_CURLY_COLON => "{:",
            COMMA => ",",
            COLON => ":",
            AT => "@",
            DOT => ".",
            L_BRACKET => "[",
            R_BRACKET => "]",
            HASH => "#",
            L_PAREN => "(",
            R_PAREN => ")",
            DOT3 => "...",
            PIPE => "|",
            NULL_KW => "null",
            TRUE_KW => "true",
            FALSE_KW => "false",
            DOCTYPE_KW => "doctype",
            HTML_KW => "html",
            DEBUG_KW => "debug",
            KEY_KW => "key",
            RENDER_KW => "render",
            CONST_KW => "const",
            ATTACH_KW => "attach",
            ELSE_KW => "else",
            IF_KW => "if",
            AS_KW => "as",
            EACH_KW => "each",
            THEN_KW => "then",
            AWAIT_KW => "await",
            CATCH_KW => "catch",
            SNIPPET_KW => "snippet",
            BIND_KW => "bind",
            TRANSITION_KW => "transition",
            USE_KW => "use",
            ANIMATE_KW => "animate",
            IN_KW => "in",
            OUT_KW => "out",
            STYLE_KW => "style",
            CLASS_KW => "class",
            EOF => "",
            HTML_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [<] => { $ crate :: HtmlSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: HtmlSyntaxKind :: R_ANGLE } ; [/] => { $ crate :: HtmlSyntaxKind :: SLASH } ; [=] => { $ crate :: HtmlSyntaxKind :: EQ } ; [!] => { $ crate :: HtmlSyntaxKind :: BANG } ; [-] => { $ crate :: HtmlSyntaxKind :: MINUS } ; ["<![CDATA["] => { $ crate :: HtmlSyntaxKind :: CDATA_START } ; ["]]>"] => { $ crate :: HtmlSyntaxKind :: CDATA_END } ; [---] => { $ crate :: HtmlSyntaxKind :: FENCE } ; ['{'] => { $ crate :: HtmlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: HtmlSyntaxKind :: R_CURLY } ; ["{{"] => { $ crate :: HtmlSyntaxKind :: L_DOUBLE_CURLY } ; ["}}"] => { $ crate :: HtmlSyntaxKind :: R_DOUBLE_CURLY } ; ["{@"] => { $ crate :: HtmlSyntaxKind :: SV_CURLY_AT } ; ["{#"] => { $ crate :: HtmlSyntaxKind :: SV_CURLY_HASH } ; ["{/"] => { $ crate :: HtmlSyntaxKind :: SV_CURLY_SLASH } ; ["{:"] => { $ crate :: HtmlSyntaxKind :: SV_CURLY_COLON } ; [,] => { $ crate :: HtmlSyntaxKind :: COMMA } ; [:] => { $ crate :: HtmlSyntaxKind :: COLON } ; [@] => { $ crate :: HtmlSyntaxKind :: AT } ; [.] => { $ crate :: HtmlSyntaxKind :: DOT } ; ['['] => { $ crate :: HtmlSyntaxKind :: L_BRACKET } ; [']'] => { $ crate :: HtmlSyntaxKind :: R_BRACKET } ; [#] => { $ crate :: HtmlSyntaxKind :: HASH } ; ['('] => { $ crate :: HtmlSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: HtmlSyntaxKind :: R_PAREN } ; [...] => { $ crate :: HtmlSyntaxKind :: DOT3 } ; [|] => { $ crate :: HtmlSyntaxKind :: PIPE } ; [null] => { $ crate :: HtmlSyntaxKind :: NULL_KW } ; [true] => { $ crate :: HtmlSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: HtmlSyntaxKind :: FALSE_KW } ; [doctype] => { $ crate :: HtmlSyntaxKind :: DOCTYPE_KW } ; [html] => { $ crate :: HtmlSyntaxKind :: HTML_KW } ; [debug] => { $ crate :: HtmlSyntaxKind :: DEBUG_KW } ; [key] => { $ crate :: HtmlSyntaxKind :: KEY_KW } ; [render] => { $ crate :: HtmlSyntaxKind :: RENDER_KW } ; [const] => { $ crate :: HtmlSyntaxKind :: CONST_KW } ; [attach] => { $ crate :: HtmlSyntaxKind :: ATTACH_KW } ; [else] => { $ crate :: HtmlSyntaxKind :: ELSE_KW } ; [if] => { $ crate :: HtmlSyntaxKind :: IF_KW } ; [as] => { $ crate :: HtmlSyntaxKind :: AS_KW } ; [each] => { $ crate :: HtmlSyntaxKind :: EACH_KW } ; [then] => { $ crate :: HtmlSyntaxKind :: THEN_KW } ; [await] => { $ crate :: HtmlSyntaxKind :: AWAIT_KW } ; [catch] => { $ crate :: HtmlSyntaxKind :: CATCH_KW } ; [snippet] => { $ crate :: HtmlSyntaxKind :: SNIPPET_KW } ; [bind] => { $ crate :: HtmlSyntaxKind :: BIND_KW } ; [transition] => { $ crate :: HtmlSyntaxKind :: TRANSITION_KW } ; [use] => { $ crate :: HtmlSyntaxKind :: USE_KW } ; [animate] => { $ crate :: HtmlSyntaxKind :: ANIMATE_KW } ; [in] => { $ crate :: HtmlSyntaxKind :: IN_KW } ; [out] => { $ crate :: HtmlSyntaxKind :: OUT_KW } ; [style] => { $ crate :: HtmlSyntaxKind :: STYLE_KW } ; [class] => { $ crate :: HtmlSyntaxKind :: CLASS_KW } ; [ident] => { $ crate :: HtmlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: HtmlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: HtmlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: HtmlSyntaxKind :: HASH } ; }
