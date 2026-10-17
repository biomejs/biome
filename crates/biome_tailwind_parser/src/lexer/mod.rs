//! The lexers of Tailwind class strings.
//!
//! [simd::SimdLexer] classifies the whole class string with SIMD up front, lexes the
//! regular-context tokens into flat arrays, and lexes any other token from the same bitmaps.
//!
//! The tests check it against the lexer of the reference implementation in `crate::legacy`.

mod base_name_store;
pub(crate) mod simd;

#[cfg(test)]
pub(crate) use base_name_store::is_delimiter;
pub use base_name_store::{BASENAME_STORE, BaseNameStore};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::TextLen;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_tailwind_syntax::TextSize;

/// The contexts in which [simd::SimdLexer::lex] lexes a token, which the token source's groupings
/// haven't replaced yet.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum TailwindLexContext {
    /// The default state.
    #[default]
    Regular,
    /// CSS value lexing inside arbitrary values and arbitrary candidate values.
    CssValue,
}

/// The diagnostic for `char` at `start`, which no token can start with.
pub(crate) fn unexpected_character(char: char, start: TextSize) -> ParseDiagnostic {
    ParseDiagnostic::new(
        format!("Unexpected character `{char}`"),
        start..start + char.text_len(),
    )
}

/// The diagnostic for the whitespace byte at `start` that CSS doesn't allow: `\x0B` or `\x0C`.
pub(crate) fn invalid_whitespace(start: TextSize) -> ParseDiagnostic {
    ParseDiagnostic::new(
        "The CSS standard only allows tabs, whitespace, carriage return and line feed whitespace.",
        start..start + TextSize::from(1),
    )
    .with_hint("Use a regular whitespace character instead.")
}

/// Returns the kind of the CSS identifier `text`: a keyword such as a unit, or `IDENT`.
pub(crate) fn css_identifier_kind(text: &[u8]) -> TailwindSyntaxKind {
    match text {
        b"url" => URL_KW,
        b"var" => VAR_KW,
        b"em" => EM_KW,
        b"rem" => REM_KW,
        b"ex" => EX_KW,
        b"rex" => REX_KW,
        b"cap" => CAP_KW,
        b"rcap" => RCAP_KW,
        b"ch" => CH_KW,
        b"rch" => RCH_KW,
        b"ic" => IC_KW,
        b"ric" => RIC_KW,
        b"lh" => LH_KW,
        b"rlh" => RLH_KW,
        b"vw" => VW_KW,
        b"svw" => SVW_KW,
        b"lvw" => LVW_KW,
        b"dvw" => DVW_KW,
        b"vh" => VH_KW,
        b"svh" => SVH_KW,
        b"lvh" => LVH_KW,
        b"dvh" => DVH_KW,
        b"vi" => VI_KW,
        b"svi" => SVI_KW,
        b"lvi" => LVI_KW,
        b"dvi" => DVI_KW,
        b"vb" => VB_KW,
        b"svb" => SVB_KW,
        b"lvb" => LVB_KW,
        b"dvb" => DVB_KW,
        b"vmin" => VMIN_KW,
        b"svmin" => SVMIN_KW,
        b"lvmin" => LVMIN_KW,
        b"dvmin" => DVMIN_KW,
        b"vmax" => VMAX_KW,
        b"svmax" => SVMAX_KW,
        b"lvmax" => LVMAX_KW,
        b"dvmax" => DVMAX_KW,
        b"cm" => CM_KW,
        b"mm" => MM_KW,
        b"q" => Q_KW,
        b"in" => IN_KW,
        b"pc" => PC_KW,
        b"pt" => PT_KW,
        b"px" => PX_KW,
        b"mozmm" => MOZMM_KW,
        b"rpx" => RPX_KW,
        b"cqw" => CQW_KW,
        b"cqh" => CQH_KW,
        b"cqi" => CQI_KW,
        b"cqb" => CQB_KW,
        b"cqmin" => CQMIN_KW,
        b"cqmax" => CQMAX_KW,
        b"deg" => DEG_KW,
        b"grad" => GRAD_KW,
        b"rad" => RAD_KW,
        b"turn" => TURN_KW,
        b"s" => S_KW,
        b"ms" => MS_KW,
        b"hz" => HZ_KW,
        b"khz" => KHZ_KW,
        b"dpi" => DPI_KW,
        b"dpcm" => DPCM_KW,
        b"dppx" => DPPX_KW,
        b"x" => X_KW,
        b"fr" => FR_KW,
        _ => IDENT,
    }
}
