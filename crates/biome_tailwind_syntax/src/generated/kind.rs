//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum TailwindSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    SLASH,
    BANG,
    DASH,
    PLUS,
    STAR,
    COMMA,
    DOT,
    COLON,
    EQ,
    HASH,
    PERCENT,
    L_BRACKET,
    R_BRACKET,
    L_PAREN,
    R_PAREN,
    WHITESPACE,
    DATA_KW,
    URL_KW,
    VAR_KW,
    EM_KW,
    REM_KW,
    EX_KW,
    REX_KW,
    CAP_KW,
    RCAP_KW,
    CH_KW,
    RCH_KW,
    IC_KW,
    RIC_KW,
    LH_KW,
    RLH_KW,
    VW_KW,
    SVW_KW,
    LVW_KW,
    DVW_KW,
    VH_KW,
    SVH_KW,
    LVH_KW,
    DVH_KW,
    VI_KW,
    SVI_KW,
    LVI_KW,
    DVI_KW,
    VB_KW,
    SVB_KW,
    LVB_KW,
    DVB_KW,
    VMIN_KW,
    SVMIN_KW,
    LVMIN_KW,
    DVMIN_KW,
    VMAX_KW,
    SVMAX_KW,
    LVMAX_KW,
    DVMAX_KW,
    CM_KW,
    MM_KW,
    Q_KW,
    IN_KW,
    PC_KW,
    PT_KW,
    PX_KW,
    MOZMM_KW,
    RPX_KW,
    CQW_KW,
    CQH_KW,
    CQI_KW,
    CQB_KW,
    CQMIN_KW,
    CQMAX_KW,
    DEG_KW,
    GRAD_KW,
    RAD_KW,
    TURN_KW,
    S_KW,
    MS_KW,
    HZ_KW,
    KHZ_KW,
    DPI_KW,
    DPCM_KW,
    DPPX_KW,
    X_KW,
    FR_KW,
    TW_BASE,
    TW_VALUE,
    TW_SELECTOR,
    TW_PROPERTY,
    CSS_STRING_LITERAL,
    CSS_NUMBER_LITERAL,
    CSS_DIMENSION_VALUE,
    CSS_PERCENTAGE_VALUE,
    CSS_COLOR_LITERAL,
    CSS_URL_VALUE_RAW_LITERAL,
    ERROR_TOKEN,
    IDENT,
    NEWLINE,
    TW_ROOT,
    TW_CANDIDATE_LIST,
    TW_FULL_CANDIDATE,
    TW_ARBITRARY_CANDIDATE,
    TW_STATIC_CANDIDATE,
    TW_FUNCTIONAL_CANDIDATE,
    TW_VARIANT_LIST,
    TW_ARBITRARY_VARIANT,
    TW_STATIC_VARIANT,
    TW_FUNCTIONAL_VARIANT,
    TW_NAMED_VALUE,
    TW_ARBITRARY_VALUE,
    TW_CSS_VARIABLE_VALUE,
    TW_MODIFIER,
    TW_DATA_ATTRIBUTE,
    TW_BOGUS,
    TW_BOGUS_CANDIDATE,
    TW_BOGUS_VARIANT,
    TW_BOGUS_MODIFIER,
    TW_BOGUS_VALUE,
    CSS_BOGUS_PROPERTY_VALUE,
    CSS_IDENTIFIER,
    CSS_DASHED_IDENTIFIER,
    CSS_STRING,
    CSS_NUMBER,
    CSS_PERCENTAGE,
    CSS_RATIO,
    CSS_FUNCTION,
    CSS_URL_FUNCTION,
    CSS_URL_VALUE_RAW,
    CSS_PARAMETER_LIST,
    CSS_COMPONENT_VALUE_LIST,
    CSS_GENERIC_COMPONENT_VALUE_LIST,
    CSS_GENERIC_DELIMITER,
    CSS_REGULAR_DIMENSION,
    CSS_UNKNOWN_DIMENSION,
    CSS_PARENTHESIZED_EXPRESSION,
    CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION,
    CSS_BINARY_EXPRESSION,
    CSS_UNARY_EXPRESSION,
    CSS_COLOR,
    #[doc(hidden)]
    __LAST,
}
use self::TailwindSyntaxKind::*;
impl TailwindSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            SLASH
                | BANG
                | DASH
                | PLUS
                | STAR
                | COMMA
                | DOT
                | COLON
                | EQ
                | HASH
                | PERCENT
                | L_BRACKET
                | R_BRACKET
                | L_PAREN
                | R_PAREN
                | WHITESPACE
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            TW_BASE
                | TW_VALUE
                | TW_SELECTOR
                | TW_PROPERTY
                | CSS_STRING_LITERAL
                | CSS_NUMBER_LITERAL
                | CSS_DIMENSION_VALUE
                | CSS_PERCENTAGE_VALUE
                | CSS_COLOR_LITERAL
                | CSS_URL_VALUE_RAW_LITERAL
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            TW_CANDIDATE_LIST
                | TW_VARIANT_LIST
                | CSS_PARAMETER_LIST
                | CSS_COMPONENT_VALUE_LIST
                | CSS_GENERIC_COMPONENT_VALUE_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<Self> {
        let kw = match ident {
            "data" => DATA_KW,
            "url" => URL_KW,
            "var" => VAR_KW,
            "em" => EM_KW,
            "rem" => REM_KW,
            "ex" => EX_KW,
            "rex" => REX_KW,
            "cap" => CAP_KW,
            "rcap" => RCAP_KW,
            "ch" => CH_KW,
            "rch" => RCH_KW,
            "ic" => IC_KW,
            "ric" => RIC_KW,
            "lh" => LH_KW,
            "rlh" => RLH_KW,
            "vw" => VW_KW,
            "svw" => SVW_KW,
            "lvw" => LVW_KW,
            "dvw" => DVW_KW,
            "vh" => VH_KW,
            "svh" => SVH_KW,
            "lvh" => LVH_KW,
            "dvh" => DVH_KW,
            "vi" => VI_KW,
            "svi" => SVI_KW,
            "lvi" => LVI_KW,
            "dvi" => DVI_KW,
            "vb" => VB_KW,
            "svb" => SVB_KW,
            "lvb" => LVB_KW,
            "dvb" => DVB_KW,
            "vmin" => VMIN_KW,
            "svmin" => SVMIN_KW,
            "lvmin" => LVMIN_KW,
            "dvmin" => DVMIN_KW,
            "vmax" => VMAX_KW,
            "svmax" => SVMAX_KW,
            "lvmax" => LVMAX_KW,
            "dvmax" => DVMAX_KW,
            "cm" => CM_KW,
            "mm" => MM_KW,
            "q" => Q_KW,
            "in" => IN_KW,
            "pc" => PC_KW,
            "pt" => PT_KW,
            "px" => PX_KW,
            "mozmm" => MOZMM_KW,
            "rpx" => RPX_KW,
            "cqw" => CQW_KW,
            "cqh" => CQH_KW,
            "cqi" => CQI_KW,
            "cqb" => CQB_KW,
            "cqmin" => CQMIN_KW,
            "cqmax" => CQMAX_KW,
            "deg" => DEG_KW,
            "grad" => GRAD_KW,
            "rad" => RAD_KW,
            "turn" => TURN_KW,
            "s" => S_KW,
            "ms" => MS_KW,
            "hz" => HZ_KW,
            "khz" => KHZ_KW,
            "dpi" => DPI_KW,
            "dpcm" => DPCM_KW,
            "dppx" => DPPX_KW,
            "x" => X_KW,
            "fr" => FR_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SLASH => "/",
            BANG => "!",
            DASH => "-",
            PLUS => "+",
            STAR => "*",
            COMMA => ",",
            DOT => ".",
            COLON => ":",
            EQ => "=",
            HASH => "#",
            PERCENT => "%",
            L_BRACKET => "[",
            R_BRACKET => "]",
            L_PAREN => "(",
            R_PAREN => ")",
            WHITESPACE => " ",
            TW_BASE => "base",
            TW_VALUE => "value",
            TW_SELECTOR => "selector",
            TW_PROPERTY => "property",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [/] => { $ crate :: TailwindSyntaxKind :: SLASH } ; [!] => { $ crate :: TailwindSyntaxKind :: BANG } ; [-] => { $ crate :: TailwindSyntaxKind :: DASH } ; [+] => { $ crate :: TailwindSyntaxKind :: PLUS } ; [*] => { $ crate :: TailwindSyntaxKind :: STAR } ; [,] => { $ crate :: TailwindSyntaxKind :: COMMA } ; [.] => { $ crate :: TailwindSyntaxKind :: DOT } ; [:] => { $ crate :: TailwindSyntaxKind :: COLON } ; [=] => { $ crate :: TailwindSyntaxKind :: EQ } ; [#] => { $ crate :: TailwindSyntaxKind :: HASH } ; [%] => { $ crate :: TailwindSyntaxKind :: PERCENT } ; ['['] => { $ crate :: TailwindSyntaxKind :: L_BRACKET } ; [']'] => { $ crate :: TailwindSyntaxKind :: R_BRACKET } ; ['('] => { $ crate :: TailwindSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: TailwindSyntaxKind :: R_PAREN } ; [' '] => { $ crate :: TailwindSyntaxKind :: WHITESPACE } ; [data] => { $ crate :: TailwindSyntaxKind :: DATA_KW } ; [url] => { $ crate :: TailwindSyntaxKind :: URL_KW } ; [var] => { $ crate :: TailwindSyntaxKind :: VAR_KW } ; [em] => { $ crate :: TailwindSyntaxKind :: EM_KW } ; [rem] => { $ crate :: TailwindSyntaxKind :: REM_KW } ; [ex] => { $ crate :: TailwindSyntaxKind :: EX_KW } ; [rex] => { $ crate :: TailwindSyntaxKind :: REX_KW } ; [cap] => { $ crate :: TailwindSyntaxKind :: CAP_KW } ; [rcap] => { $ crate :: TailwindSyntaxKind :: RCAP_KW } ; [ch] => { $ crate :: TailwindSyntaxKind :: CH_KW } ; [rch] => { $ crate :: TailwindSyntaxKind :: RCH_KW } ; [ic] => { $ crate :: TailwindSyntaxKind :: IC_KW } ; [ric] => { $ crate :: TailwindSyntaxKind :: RIC_KW } ; [lh] => { $ crate :: TailwindSyntaxKind :: LH_KW } ; [rlh] => { $ crate :: TailwindSyntaxKind :: RLH_KW } ; [vw] => { $ crate :: TailwindSyntaxKind :: VW_KW } ; [svw] => { $ crate :: TailwindSyntaxKind :: SVW_KW } ; [lvw] => { $ crate :: TailwindSyntaxKind :: LVW_KW } ; [dvw] => { $ crate :: TailwindSyntaxKind :: DVW_KW } ; [vh] => { $ crate :: TailwindSyntaxKind :: VH_KW } ; [svh] => { $ crate :: TailwindSyntaxKind :: SVH_KW } ; [lvh] => { $ crate :: TailwindSyntaxKind :: LVH_KW } ; [dvh] => { $ crate :: TailwindSyntaxKind :: DVH_KW } ; [vi] => { $ crate :: TailwindSyntaxKind :: VI_KW } ; [svi] => { $ crate :: TailwindSyntaxKind :: SVI_KW } ; [lvi] => { $ crate :: TailwindSyntaxKind :: LVI_KW } ; [dvi] => { $ crate :: TailwindSyntaxKind :: DVI_KW } ; [vb] => { $ crate :: TailwindSyntaxKind :: VB_KW } ; [svb] => { $ crate :: TailwindSyntaxKind :: SVB_KW } ; [lvb] => { $ crate :: TailwindSyntaxKind :: LVB_KW } ; [dvb] => { $ crate :: TailwindSyntaxKind :: DVB_KW } ; [vmin] => { $ crate :: TailwindSyntaxKind :: VMIN_KW } ; [svmin] => { $ crate :: TailwindSyntaxKind :: SVMIN_KW } ; [lvmin] => { $ crate :: TailwindSyntaxKind :: LVMIN_KW } ; [dvmin] => { $ crate :: TailwindSyntaxKind :: DVMIN_KW } ; [vmax] => { $ crate :: TailwindSyntaxKind :: VMAX_KW } ; [svmax] => { $ crate :: TailwindSyntaxKind :: SVMAX_KW } ; [lvmax] => { $ crate :: TailwindSyntaxKind :: LVMAX_KW } ; [dvmax] => { $ crate :: TailwindSyntaxKind :: DVMAX_KW } ; [cm] => { $ crate :: TailwindSyntaxKind :: CM_KW } ; [mm] => { $ crate :: TailwindSyntaxKind :: MM_KW } ; [q] => { $ crate :: TailwindSyntaxKind :: Q_KW } ; [in] => { $ crate :: TailwindSyntaxKind :: IN_KW } ; [pc] => { $ crate :: TailwindSyntaxKind :: PC_KW } ; [pt] => { $ crate :: TailwindSyntaxKind :: PT_KW } ; [px] => { $ crate :: TailwindSyntaxKind :: PX_KW } ; [mozmm] => { $ crate :: TailwindSyntaxKind :: MOZMM_KW } ; [rpx] => { $ crate :: TailwindSyntaxKind :: RPX_KW } ; [cqw] => { $ crate :: TailwindSyntaxKind :: CQW_KW } ; [cqh] => { $ crate :: TailwindSyntaxKind :: CQH_KW } ; [cqi] => { $ crate :: TailwindSyntaxKind :: CQI_KW } ; [cqb] => { $ crate :: TailwindSyntaxKind :: CQB_KW } ; [cqmin] => { $ crate :: TailwindSyntaxKind :: CQMIN_KW } ; [cqmax] => { $ crate :: TailwindSyntaxKind :: CQMAX_KW } ; [deg] => { $ crate :: TailwindSyntaxKind :: DEG_KW } ; [grad] => { $ crate :: TailwindSyntaxKind :: GRAD_KW } ; [rad] => { $ crate :: TailwindSyntaxKind :: RAD_KW } ; [turn] => { $ crate :: TailwindSyntaxKind :: TURN_KW } ; [s] => { $ crate :: TailwindSyntaxKind :: S_KW } ; [ms] => { $ crate :: TailwindSyntaxKind :: MS_KW } ; [hz] => { $ crate :: TailwindSyntaxKind :: HZ_KW } ; [khz] => { $ crate :: TailwindSyntaxKind :: KHZ_KW } ; [dpi] => { $ crate :: TailwindSyntaxKind :: DPI_KW } ; [dpcm] => { $ crate :: TailwindSyntaxKind :: DPCM_KW } ; [dppx] => { $ crate :: TailwindSyntaxKind :: DPPX_KW } ; [x] => { $ crate :: TailwindSyntaxKind :: X_KW } ; [fr] => { $ crate :: TailwindSyntaxKind :: FR_KW } ; [ident] => { $ crate :: TailwindSyntaxKind :: IDENT } ; [EOF] => { $ crate :: TailwindSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: TailwindSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: TailwindSyntaxKind :: HASH } ; }
