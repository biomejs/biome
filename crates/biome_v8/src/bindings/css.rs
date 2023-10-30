//! Generated file, do not edit by hand, see `xtask/codegen`

use super::TemplateRegistry;
use crate::convert::{FromV8, ToV8};
use biome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};
pub(super) fn register_interfaces(
    scope: &mut v8::HandleScope<'_, ()>,
    global: v8::Local<'_, v8::ObjectTemplate>,
    registry: &mut TemplateRegistry,
) {
    registry
        .build_enum::<biome_css_syntax::CssSyntaxKind>(scope, global, "CssSyntaxKind")
        .variant("EOF", biome_css_syntax::CssSyntaxKind::EOF)
        .variant("SEMICOLON", biome_css_syntax::CssSyntaxKind::SEMICOLON)
        .variant("COMMA", biome_css_syntax::CssSyntaxKind::COMMA)
        .variant("L_PAREN", biome_css_syntax::CssSyntaxKind::L_PAREN)
        .variant("R_PAREN", biome_css_syntax::CssSyntaxKind::R_PAREN)
        .variant("L_CURLY", biome_css_syntax::CssSyntaxKind::L_CURLY)
        .variant("R_CURLY", biome_css_syntax::CssSyntaxKind::R_CURLY)
        .variant("L_BRACK", biome_css_syntax::CssSyntaxKind::L_BRACK)
        .variant("R_BRACK", biome_css_syntax::CssSyntaxKind::R_BRACK)
        .variant("L_ANGLE", biome_css_syntax::CssSyntaxKind::L_ANGLE)
        .variant("R_ANGLE", biome_css_syntax::CssSyntaxKind::R_ANGLE)
        .variant("TILDE", biome_css_syntax::CssSyntaxKind::TILDE)
        .variant("HASH", biome_css_syntax::CssSyntaxKind::HASH)
        .variant("AMP", biome_css_syntax::CssSyntaxKind::AMP)
        .variant("PIPE", biome_css_syntax::CssSyntaxKind::PIPE)
        .variant("PLUS", biome_css_syntax::CssSyntaxKind::PLUS)
        .variant("STAR", biome_css_syntax::CssSyntaxKind::STAR)
        .variant("SLASH", biome_css_syntax::CssSyntaxKind::SLASH)
        .variant("CARET", biome_css_syntax::CssSyntaxKind::CARET)
        .variant("PERCENT", biome_css_syntax::CssSyntaxKind::PERCENT)
        .variant("DOT", biome_css_syntax::CssSyntaxKind::DOT)
        .variant("COLON", biome_css_syntax::CssSyntaxKind::COLON)
        .variant("EQ", biome_css_syntax::CssSyntaxKind::EQ)
        .variant("BANG", biome_css_syntax::CssSyntaxKind::BANG)
        .variant("NEQ", biome_css_syntax::CssSyntaxKind::NEQ)
        .variant("MINUS", biome_css_syntax::CssSyntaxKind::MINUS)
        .variant("LTEQ", biome_css_syntax::CssSyntaxKind::LTEQ)
        .variant("GTEQ", biome_css_syntax::CssSyntaxKind::GTEQ)
        .variant("PLUSEQ", biome_css_syntax::CssSyntaxKind::PLUSEQ)
        .variant("PIPEEQ", biome_css_syntax::CssSyntaxKind::PIPEEQ)
        .variant("AMPEQ", biome_css_syntax::CssSyntaxKind::AMPEQ)
        .variant("CARETEQ", biome_css_syntax::CssSyntaxKind::CARETEQ)
        .variant("SLASHEQ", biome_css_syntax::CssSyntaxKind::SLASHEQ)
        .variant("STAREQ", biome_css_syntax::CssSyntaxKind::STAREQ)
        .variant("PERCENTEQ", biome_css_syntax::CssSyntaxKind::PERCENTEQ)
        .variant("AT", biome_css_syntax::CssSyntaxKind::AT)
        .variant("DOLLAR_EQ", biome_css_syntax::CssSyntaxKind::DOLLAR_EQ)
        .variant("TILDE_EQ", biome_css_syntax::CssSyntaxKind::TILDE_EQ)
        .variant("CDC", biome_css_syntax::CssSyntaxKind::CDC)
        .variant("CDO", biome_css_syntax::CssSyntaxKind::CDO)
        .variant(
            "ALICEBLUE_KW",
            biome_css_syntax::CssSyntaxKind::ALICEBLUE_KW,
        )
        .variant(
            "ANTIQUEWHITE_KW",
            biome_css_syntax::CssSyntaxKind::ANTIQUEWHITE_KW,
        )
        .variant("AQUA_KW", biome_css_syntax::CssSyntaxKind::AQUA_KW)
        .variant(
            "AQUAMARINE_KW",
            biome_css_syntax::CssSyntaxKind::AQUAMARINE_KW,
        )
        .variant("AZURE_KW", biome_css_syntax::CssSyntaxKind::AZURE_KW)
        .variant("BEIGE_KW", biome_css_syntax::CssSyntaxKind::BEIGE_KW)
        .variant("BISQUE_KW", biome_css_syntax::CssSyntaxKind::BISQUE_KW)
        .variant("BLACK_KW", biome_css_syntax::CssSyntaxKind::BLACK_KW)
        .variant(
            "BLANCHEDALMOND_KW",
            biome_css_syntax::CssSyntaxKind::BLANCHEDALMOND_KW,
        )
        .variant("BLUE_KW", biome_css_syntax::CssSyntaxKind::BLUE_KW)
        .variant(
            "BLUEVIOLET_KW",
            biome_css_syntax::CssSyntaxKind::BLUEVIOLET_KW,
        )
        .variant("BROWN_KW", biome_css_syntax::CssSyntaxKind::BROWN_KW)
        .variant(
            "BURLYWOOD_KW",
            biome_css_syntax::CssSyntaxKind::BURLYWOOD_KW,
        )
        .variant(
            "CADETBLUE_KW",
            biome_css_syntax::CssSyntaxKind::CADETBLUE_KW,
        )
        .variant(
            "CHARTREUSE_KW",
            biome_css_syntax::CssSyntaxKind::CHARTREUSE_KW,
        )
        .variant(
            "CHOCOLATE_KW",
            biome_css_syntax::CssSyntaxKind::CHOCOLATE_KW,
        )
        .variant("CORAL_KW", biome_css_syntax::CssSyntaxKind::CORAL_KW)
        .variant(
            "CORNFLOWERBLUE_KW",
            biome_css_syntax::CssSyntaxKind::CORNFLOWERBLUE_KW,
        )
        .variant("CORNSILK_KW", biome_css_syntax::CssSyntaxKind::CORNSILK_KW)
        .variant("CRIMSON_KW", biome_css_syntax::CssSyntaxKind::CRIMSON_KW)
        .variant("CYAN_KW", biome_css_syntax::CssSyntaxKind::CYAN_KW)
        .variant("DARKBLUE_KW", biome_css_syntax::CssSyntaxKind::DARKBLUE_KW)
        .variant("DARKCYAN_KW", biome_css_syntax::CssSyntaxKind::DARKCYAN_KW)
        .variant(
            "DARKGOLDENROD_KW",
            biome_css_syntax::CssSyntaxKind::DARKGOLDENROD_KW,
        )
        .variant("DARKGRAY_KW", biome_css_syntax::CssSyntaxKind::DARKGRAY_KW)
        .variant(
            "DARKGREEN_KW",
            biome_css_syntax::CssSyntaxKind::DARKGREEN_KW,
        )
        .variant(
            "DARKKHAKI_KW",
            biome_css_syntax::CssSyntaxKind::DARKKHAKI_KW,
        )
        .variant(
            "DARKMAGENTA_KW",
            biome_css_syntax::CssSyntaxKind::DARKMAGENTA_KW,
        )
        .variant(
            "DARKOLIVEGREEN_KW",
            biome_css_syntax::CssSyntaxKind::DARKOLIVEGREEN_KW,
        )
        .variant(
            "DARKORANGE_KW",
            biome_css_syntax::CssSyntaxKind::DARKORANGE_KW,
        )
        .variant(
            "DARKORCHID_KW",
            biome_css_syntax::CssSyntaxKind::DARKORCHID_KW,
        )
        .variant("DARKRED_KW", biome_css_syntax::CssSyntaxKind::DARKRED_KW)
        .variant(
            "DARKSALMON_KW",
            biome_css_syntax::CssSyntaxKind::DARKSALMON_KW,
        )
        .variant(
            "DARKSEAGREEN_KW",
            biome_css_syntax::CssSyntaxKind::DARKSEAGREEN_KW,
        )
        .variant(
            "DARKSLATEBLUE_KW",
            biome_css_syntax::CssSyntaxKind::DARKSLATEBLUE_KW,
        )
        .variant(
            "DARKSLATEGRAY_KW",
            biome_css_syntax::CssSyntaxKind::DARKSLATEGRAY_KW,
        )
        .variant(
            "DARKTURQUOISE_KW",
            biome_css_syntax::CssSyntaxKind::DARKTURQUOISE_KW,
        )
        .variant(
            "DARKVIOLET_KW",
            biome_css_syntax::CssSyntaxKind::DARKVIOLET_KW,
        )
        .variant("DEEPPINK_KW", biome_css_syntax::CssSyntaxKind::DEEPPINK_KW)
        .variant(
            "DEEPSKYBLUE_KW",
            biome_css_syntax::CssSyntaxKind::DEEPSKYBLUE_KW,
        )
        .variant("DIMGRAY_KW", biome_css_syntax::CssSyntaxKind::DIMGRAY_KW)
        .variant(
            "DODGERBLUE_KW",
            biome_css_syntax::CssSyntaxKind::DODGERBLUE_KW,
        )
        .variant(
            "FIREBRICK_KW",
            biome_css_syntax::CssSyntaxKind::FIREBRICK_KW,
        )
        .variant(
            "FLORALWHITE_KW",
            biome_css_syntax::CssSyntaxKind::FLORALWHITE_KW,
        )
        .variant(
            "FORESTGREEN_KW",
            biome_css_syntax::CssSyntaxKind::FORESTGREEN_KW,
        )
        .variant("FUCHSIA_KW", biome_css_syntax::CssSyntaxKind::FUCHSIA_KW)
        .variant(
            "GAINSBORO_KW",
            biome_css_syntax::CssSyntaxKind::GAINSBORO_KW,
        )
        .variant(
            "GHOSTWHITE_KW",
            biome_css_syntax::CssSyntaxKind::GHOSTWHITE_KW,
        )
        .variant("GOLD_KW", biome_css_syntax::CssSyntaxKind::GOLD_KW)
        .variant(
            "GOLDENROD_KW",
            biome_css_syntax::CssSyntaxKind::GOLDENROD_KW,
        )
        .variant("GRAY_KW", biome_css_syntax::CssSyntaxKind::GRAY_KW)
        .variant("GREEN_KW", biome_css_syntax::CssSyntaxKind::GREEN_KW)
        .variant(
            "GREENYELLOW_KW",
            biome_css_syntax::CssSyntaxKind::GREENYELLOW_KW,
        )
        .variant("HONEYDEW_KW", biome_css_syntax::CssSyntaxKind::HONEYDEW_KW)
        .variant("HOTPINK_KW", biome_css_syntax::CssSyntaxKind::HOTPINK_KW)
        .variant(
            "INDIANRED_KW",
            biome_css_syntax::CssSyntaxKind::INDIANRED_KW,
        )
        .variant("INDIGO_KW", biome_css_syntax::CssSyntaxKind::INDIGO_KW)
        .variant("IVORY_KW", biome_css_syntax::CssSyntaxKind::IVORY_KW)
        .variant("KHAKI_KW", biome_css_syntax::CssSyntaxKind::KHAKI_KW)
        .variant("LAVENDER_KW", biome_css_syntax::CssSyntaxKind::LAVENDER_KW)
        .variant(
            "LAVENDERBLUSH_KW",
            biome_css_syntax::CssSyntaxKind::LAVENDERBLUSH_KW,
        )
        .variant(
            "LAWNGREEN_KW",
            biome_css_syntax::CssSyntaxKind::LAWNGREEN_KW,
        )
        .variant(
            "LEMONCHIFFON_KW",
            biome_css_syntax::CssSyntaxKind::LEMONCHIFFON_KW,
        )
        .variant(
            "LIGHTBLUE_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTBLUE_KW,
        )
        .variant(
            "LIGHTCORAL_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTCORAL_KW,
        )
        .variant(
            "LIGHTCYAN_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTCYAN_KW,
        )
        .variant(
            "LIGHTGOLDENRODYELLOW_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTGOLDENRODYELLOW_KW,
        )
        .variant(
            "LIGHTGREEN_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTGREEN_KW,
        )
        .variant(
            "LIGHTGREY_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTGREY_KW,
        )
        .variant(
            "LIGHTPINK_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTPINK_KW,
        )
        .variant(
            "LIGHTSALMON_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTSALMON_KW,
        )
        .variant(
            "LIGHTSEAGREEN_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTSEAGREEN_KW,
        )
        .variant(
            "LIGHTSKYBLUE_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTSKYBLUE_KW,
        )
        .variant(
            "LIGHTSLATEGRAY_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTSLATEGRAY_KW,
        )
        .variant(
            "LIGHTSTEELBLUE_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTSTEELBLUE_KW,
        )
        .variant(
            "LIGHTYELLOW_KW",
            biome_css_syntax::CssSyntaxKind::LIGHTYELLOW_KW,
        )
        .variant("LIME_KW", biome_css_syntax::CssSyntaxKind::LIME_KW)
        .variant(
            "LIMEGREEN_KW",
            biome_css_syntax::CssSyntaxKind::LIMEGREEN_KW,
        )
        .variant("LINEN_KW", biome_css_syntax::CssSyntaxKind::LINEN_KW)
        .variant("MAGENTA_KW", biome_css_syntax::CssSyntaxKind::MAGENTA_KW)
        .variant("MAROON_KW", biome_css_syntax::CssSyntaxKind::MAROON_KW)
        .variant(
            "MEDIUMAQUAMARINE_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMAQUAMARINE_KW,
        )
        .variant(
            "MEDIUMBLUE_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMBLUE_KW,
        )
        .variant(
            "MEDIUMORCHID_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMORCHID_KW,
        )
        .variant(
            "MEDIUMPURPLE_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMPURPLE_KW,
        )
        .variant(
            "MEDIUMSEAGREEN_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMSEAGREEN_KW,
        )
        .variant(
            "MEDIUMSLATEBLUE_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMSLATEBLUE_KW,
        )
        .variant(
            "MEDIUMSPRINGGREEN_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMSPRINGGREEN_KW,
        )
        .variant(
            "MEDIUMTURQUOISE_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMTURQUOISE_KW,
        )
        .variant(
            "MEDIUMVIOLETRED_KW",
            biome_css_syntax::CssSyntaxKind::MEDIUMVIOLETRED_KW,
        )
        .variant(
            "MIDNIGHTBLUE_KW",
            biome_css_syntax::CssSyntaxKind::MIDNIGHTBLUE_KW,
        )
        .variant(
            "MINTCREAM_KW",
            biome_css_syntax::CssSyntaxKind::MINTCREAM_KW,
        )
        .variant(
            "MISTYROSE_KW",
            biome_css_syntax::CssSyntaxKind::MISTYROSE_KW,
        )
        .variant("MOCCASIN_KW", biome_css_syntax::CssSyntaxKind::MOCCASIN_KW)
        .variant(
            "NAVAJOWHITE_KW",
            biome_css_syntax::CssSyntaxKind::NAVAJOWHITE_KW,
        )
        .variant("NAVY_KW", biome_css_syntax::CssSyntaxKind::NAVY_KW)
        .variant("NAVYBLUE_KW", biome_css_syntax::CssSyntaxKind::NAVYBLUE_KW)
        .variant("OLDLACE_KW", biome_css_syntax::CssSyntaxKind::OLDLACE_KW)
        .variant("OLIVE_KW", biome_css_syntax::CssSyntaxKind::OLIVE_KW)
        .variant(
            "OLIVEDRAB_KW",
            biome_css_syntax::CssSyntaxKind::OLIVEDRAB_KW,
        )
        .variant("ORANGE_KW", biome_css_syntax::CssSyntaxKind::ORANGE_KW)
        .variant(
            "ORANGERED_KW",
            biome_css_syntax::CssSyntaxKind::ORANGERED_KW,
        )
        .variant("ORCHID_KW", biome_css_syntax::CssSyntaxKind::ORCHID_KW)
        .variant(
            "PALEGOLDENROD_KW",
            biome_css_syntax::CssSyntaxKind::PALEGOLDENROD_KW,
        )
        .variant(
            "PALEGREEN_KW",
            biome_css_syntax::CssSyntaxKind::PALEGREEN_KW,
        )
        .variant(
            "PALETURQUOISE_KW",
            biome_css_syntax::CssSyntaxKind::PALETURQUOISE_KW,
        )
        .variant(
            "PALEVIOLETRED_KW",
            biome_css_syntax::CssSyntaxKind::PALEVIOLETRED_KW,
        )
        .variant(
            "PAPAYAWHIP_KW",
            biome_css_syntax::CssSyntaxKind::PAPAYAWHIP_KW,
        )
        .variant(
            "PEACHPUFF_KW",
            biome_css_syntax::CssSyntaxKind::PEACHPUFF_KW,
        )
        .variant("PERU_KW", biome_css_syntax::CssSyntaxKind::PERU_KW)
        .variant("PINK_KW", biome_css_syntax::CssSyntaxKind::PINK_KW)
        .variant("PLUM_KW", biome_css_syntax::CssSyntaxKind::PLUM_KW)
        .variant(
            "POWDERBLUE_KW",
            biome_css_syntax::CssSyntaxKind::POWDERBLUE_KW,
        )
        .variant("PURPLE_KW", biome_css_syntax::CssSyntaxKind::PURPLE_KW)
        .variant("RED_KW", biome_css_syntax::CssSyntaxKind::RED_KW)
        .variant(
            "ROSYBROWN_KW",
            biome_css_syntax::CssSyntaxKind::ROSYBROWN_KW,
        )
        .variant(
            "ROYALBLUE_KW",
            biome_css_syntax::CssSyntaxKind::ROYALBLUE_KW,
        )
        .variant(
            "SADDLEBROWN_KW",
            biome_css_syntax::CssSyntaxKind::SADDLEBROWN_KW,
        )
        .variant("SALMON_KW", biome_css_syntax::CssSyntaxKind::SALMON_KW)
        .variant(
            "SANDYBROWN_KW",
            biome_css_syntax::CssSyntaxKind::SANDYBROWN_KW,
        )
        .variant("SEAGREEN_KW", biome_css_syntax::CssSyntaxKind::SEAGREEN_KW)
        .variant("SEASHELL_KW", biome_css_syntax::CssSyntaxKind::SEASHELL_KW)
        .variant("SIENNA_KW", biome_css_syntax::CssSyntaxKind::SIENNA_KW)
        .variant("SILVER_KW", biome_css_syntax::CssSyntaxKind::SILVER_KW)
        .variant("SKYBLUE_KW", biome_css_syntax::CssSyntaxKind::SKYBLUE_KW)
        .variant(
            "SLATEBLUE_KW",
            biome_css_syntax::CssSyntaxKind::SLATEBLUE_KW,
        )
        .variant(
            "SLATEGRAY_KW",
            biome_css_syntax::CssSyntaxKind::SLATEGRAY_KW,
        )
        .variant("SNOW_KW", biome_css_syntax::CssSyntaxKind::SNOW_KW)
        .variant(
            "SPRINGGREEN_KW",
            biome_css_syntax::CssSyntaxKind::SPRINGGREEN_KW,
        )
        .variant(
            "STEELBLUE_KW",
            biome_css_syntax::CssSyntaxKind::STEELBLUE_KW,
        )
        .variant("TAN_KW", biome_css_syntax::CssSyntaxKind::TAN_KW)
        .variant("TEAL_KW", biome_css_syntax::CssSyntaxKind::TEAL_KW)
        .variant("THISTLE_KW", biome_css_syntax::CssSyntaxKind::THISTLE_KW)
        .variant("TOMATO_KW", biome_css_syntax::CssSyntaxKind::TOMATO_KW)
        .variant(
            "TURQUOISE_KW",
            biome_css_syntax::CssSyntaxKind::TURQUOISE_KW,
        )
        .variant("VIOLET_KW", biome_css_syntax::CssSyntaxKind::VIOLET_KW)
        .variant("WHEAT_KW", biome_css_syntax::CssSyntaxKind::WHEAT_KW)
        .variant("WHITE_KW", biome_css_syntax::CssSyntaxKind::WHITE_KW)
        .variant(
            "WHITESMOKE_KW",
            biome_css_syntax::CssSyntaxKind::WHITESMOKE_KW,
        )
        .variant("YELLOW_KW", biome_css_syntax::CssSyntaxKind::YELLOW_KW)
        .variant(
            "YELLOWGREEN_KW",
            biome_css_syntax::CssSyntaxKind::YELLOWGREEN_KW,
        )
        .variant("MEDIA_KW", biome_css_syntax::CssSyntaxKind::MEDIA_KW)
        .variant(
            "KEYFRAMES_KW",
            biome_css_syntax::CssSyntaxKind::KEYFRAMES_KW,
        )
        .variant("NOT_KW", biome_css_syntax::CssSyntaxKind::NOT_KW)
        .variant("AND_KW", biome_css_syntax::CssSyntaxKind::AND_KW)
        .variant("ONLY_KW", biome_css_syntax::CssSyntaxKind::ONLY_KW)
        .variant("OR_KW", biome_css_syntax::CssSyntaxKind::OR_KW)
        .variant("I_KW", biome_css_syntax::CssSyntaxKind::I_KW)
        .variant(
            "IMPORTANT_KW",
            biome_css_syntax::CssSyntaxKind::IMPORTANT_KW,
        )
        .variant("FROM_KW", biome_css_syntax::CssSyntaxKind::FROM_KW)
        .variant("TO_KW", biome_css_syntax::CssSyntaxKind::TO_KW)
        .variant("VAR_KW", biome_css_syntax::CssSyntaxKind::VAR_KW)
        .variant(
            "CSS_STRING_LITERAL",
            biome_css_syntax::CssSyntaxKind::CSS_STRING_LITERAL,
        )
        .variant(
            "CSS_NUMBER_LITERAL",
            biome_css_syntax::CssSyntaxKind::CSS_NUMBER_LITERAL,
        )
        .variant(
            "CSS_CUSTOM_PROPERTY",
            biome_css_syntax::CssSyntaxKind::CSS_CUSTOM_PROPERTY,
        )
        .variant(
            "CSS_SPACE_LITERAL",
            biome_css_syntax::CssSyntaxKind::CSS_SPACE_LITERAL,
        )
        .variant("ERROR_TOKEN", biome_css_syntax::CssSyntaxKind::ERROR_TOKEN)
        .variant("IDENT", biome_css_syntax::CssSyntaxKind::IDENT)
        .variant("NEWLINE", biome_css_syntax::CssSyntaxKind::NEWLINE)
        .variant("WHITESPACE", biome_css_syntax::CssSyntaxKind::WHITESPACE)
        .variant("COMMENT", biome_css_syntax::CssSyntaxKind::COMMENT)
        .variant(
            "MULTILINE_COMMENT",
            biome_css_syntax::CssSyntaxKind::MULTILINE_COMMENT,
        )
        .variant("CSS_ROOT", biome_css_syntax::CssSyntaxKind::CSS_ROOT)
        .variant(
            "CSS_RULE_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_RULE_LIST,
        )
        .variant(
            "CSS_ID_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_ID_SELECTOR_PATTERN,
        )
        .variant("CSS_RULE", biome_css_syntax::CssSyntaxKind::CSS_RULE)
        .variant(
            "CSS_SELECTOR_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_SELECTOR_LIST,
        )
        .variant(
            "CSS_SELECTOR",
            biome_css_syntax::CssSyntaxKind::CSS_SELECTOR,
        )
        .variant(
            "CSS_ANY_FUNCTION",
            biome_css_syntax::CssSyntaxKind::CSS_ANY_FUNCTION,
        )
        .variant(
            "CSS_AT_KEYFRAMES",
            biome_css_syntax::CssSyntaxKind::CSS_AT_KEYFRAMES,
        )
        .variant(
            "CSS_AT_KEYFRAMES_BODY",
            biome_css_syntax::CssSyntaxKind::CSS_AT_KEYFRAMES_BODY,
        )
        .variant(
            "CSS_AT_MEDIA",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_CONSEQUENT",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_CONSEQUENT,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_FEATURE",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_FEATURE_COMPARE",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_COMPARE,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_FEATURE_PLAIN",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_PLAIN,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_FEATURE_RANGE",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_RANGE,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_RANGE",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_RANGE,
        )
        .variant(
            "CSS_ATTRIBUTE",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE,
        )
        .variant(
            "CSS_ATTRIBUTE_MATCHER",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE_MATCHER,
        )
        .variant(
            "CSS_ATTRIBUTE_META",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE_META,
        )
        .variant(
            "CSS_ATTRIBUTE_MODIFIER",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE_MODIFIER,
        )
        .variant(
            "CSS_ATTRIBUTE_NAME",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE_NAME,
        )
        .variant(
            "CSS_ATTRIBUTE_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR_PATTERN,
        )
        .variant("CSS_BLOCK", biome_css_syntax::CssSyntaxKind::CSS_BLOCK)
        .variant(
            "CSS_CLASS_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_CLASS_SELECTOR_PATTERN,
        )
        .variant(
            "CSS_COMBINATOR_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_COMBINATOR_SELECTOR_PATTERN,
        )
        .variant(
            "CSS_DECLARATION",
            biome_css_syntax::CssSyntaxKind::CSS_DECLARATION,
        )
        .variant(
            "CSS_DIMENSION",
            biome_css_syntax::CssSyntaxKind::CSS_DIMENSION,
        )
        .variant(
            "CSS_IDENTIFIER",
            biome_css_syntax::CssSyntaxKind::CSS_IDENTIFIER,
        )
        .variant(
            "CSS_KEYFRAMES_BLOCK",
            biome_css_syntax::CssSyntaxKind::CSS_KEYFRAMES_BLOCK,
        )
        .variant(
            "CSS_KEYFRAMES_SELECTOR",
            biome_css_syntax::CssSyntaxKind::CSS_KEYFRAMES_SELECTOR,
        )
        .variant("CSS_NUMBER", biome_css_syntax::CssSyntaxKind::CSS_NUMBER)
        .variant(
            "CSS_PARAMETER",
            biome_css_syntax::CssSyntaxKind::CSS_PARAMETER,
        )
        .variant(
            "CSS_PERCENTAGE",
            biome_css_syntax::CssSyntaxKind::CSS_PERCENTAGE,
        )
        .variant(
            "CSS_PSEUDO_CLASS_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PATTERN,
        )
        .variant(
            "CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS",
            biome_css_syntax::CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS,
        )
        .variant("CSS_RATIO", biome_css_syntax::CssSyntaxKind::CSS_RATIO)
        .variant(
            "CSS_SIMPLE_FUNCTION",
            biome_css_syntax::CssSyntaxKind::CSS_SIMPLE_FUNCTION,
        )
        .variant("CSS_STRING", biome_css_syntax::CssSyntaxKind::CSS_STRING)
        .variant(
            "CSS_TYPE_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_TYPE_SELECTOR_PATTERN,
        )
        .variant(
            "CSS_UNIVERSAL_SELECTOR_PATTERN",
            biome_css_syntax::CssSyntaxKind::CSS_UNIVERSAL_SELECTOR_PATTERN,
        )
        .variant(
            "CSS_VAR_FUNCTION",
            biome_css_syntax::CssSyntaxKind::CSS_VAR_FUNCTION,
        )
        .variant(
            "CSS_VAR_FUNCTION_VALUE",
            biome_css_syntax::CssSyntaxKind::CSS_VAR_FUNCTION_VALUE,
        )
        .variant(
            "CSS_ANY_SELECTOR_PATTERN_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_ANY_SELECTOR_PATTERN_LIST,
        )
        .variant(
            "CSS_AT_KEYFRAMES_ITEM_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_AT_KEYFRAMES_ITEM_LIST,
        )
        .variant(
            "CSS_AT_MEDIA_QUERY_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_AT_MEDIA_QUERY_LIST,
        )
        .variant(
            "CSS_ATTRIBUTE_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_ATTRIBUTE_LIST,
        )
        .variant(
            "CSS_DECLARATION_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_DECLARATION_LIST,
        )
        .variant(
            "CSS_KEYFRAMES_SELECTOR_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_KEYFRAMES_SELECTOR_LIST,
        )
        .variant(
            "CSS_PARAMETER_LIST",
            biome_css_syntax::CssSyntaxKind::CSS_PARAMETER_LIST,
        )
        .variant(
            "CSS_DECLARATION_IMPORTANT",
            biome_css_syntax::CssSyntaxKind::CSS_DECLARATION_IMPORTANT,
        )
        .variant("CSS_BOGUS", biome_css_syntax::CssSyntaxKind::CSS_BOGUS)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAnyFunction>(scope, global, "CssAnyFunction")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "css_simple_function",
            CssAnyFunction_css_simple_function,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtKeyframes>(scope, global, "CssAtKeyframes")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "at_token", CssAtKeyframes_at_token)
        .method(scope, "keyframes_token", CssAtKeyframes_keyframes_token)
        .method(scope, "name", CssAtKeyframes_name)
        .method(scope, "css_string", CssAtKeyframes_css_string)
        .method(scope, "body", CssAtKeyframes_body)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtKeyframesBody>(scope, global, "CssAtKeyframesBody")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "l_curly_token", CssAtKeyframesBody_l_curly_token)
        .method(scope, "items", CssAtKeyframesBody_items)
        .method(scope, "r_curly_token", CssAtKeyframesBody_r_curly_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMedia>(scope, global, "CssAtMedia")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "at_token", CssAtMedia_at_token)
        .method(scope, "media_token", CssAtMedia_media_token)
        .method(scope, "query_list", CssAtMedia_query_list)
        .method(scope, "l_curly_token", CssAtMedia_l_curly_token)
        .method(scope, "body", CssAtMedia_body)
        .method(scope, "r_curly_token", CssAtMedia_r_curly_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQuery>(scope, global, "CssAtMediaQuery")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "condition_token", CssAtMediaQuery_condition_token)
        .method(scope, "or_token", CssAtMediaQuery_or_token)
        .method(scope, "only_token", CssAtMediaQuery_only_token)
        .method(scope, "ty", CssAtMediaQuery_ty)
        .method(scope, "consequent", CssAtMediaQuery_consequent)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryConsequent>(
            scope,
            global,
            "CssAtMediaQueryConsequent",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "and_token", CssAtMediaQueryConsequent_and_token)
        .method(
            scope,
            "condition_token",
            CssAtMediaQueryConsequent_condition_token,
        )
        .method(scope, "ty", CssAtMediaQueryConsequent_ty)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryFeature>(
            scope,
            global,
            "CssAtMediaQueryFeature",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "l_paren_token", CssAtMediaQueryFeature_l_paren_token)
        .method(scope, "feature", CssAtMediaQueryFeature_feature)
        .method(scope, "r_paren_token", CssAtMediaQueryFeature_r_paren_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryFeatureBoolean>(
            scope,
            global,
            "CssAtMediaQueryFeatureBoolean",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "css_identifier",
            CssAtMediaQueryFeatureBoolean_css_identifier,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryFeatureCompare>(
            scope,
            global,
            "CssAtMediaQueryFeatureCompare",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "name", CssAtMediaQueryFeatureCompare_name)
        .method(scope, "range", CssAtMediaQueryFeatureCompare_range)
        .method(scope, "value", CssAtMediaQueryFeatureCompare_value)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryFeaturePlain>(
            scope,
            global,
            "CssAtMediaQueryFeaturePlain",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "name", CssAtMediaQueryFeaturePlain_name)
        .method(
            scope,
            "colon_token",
            CssAtMediaQueryFeaturePlain_colon_token,
        )
        .method(scope, "value", CssAtMediaQueryFeaturePlain_value)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryFeatureRange>(
            scope,
            global,
            "CssAtMediaQueryFeatureRange",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "first_value",
            CssAtMediaQueryFeatureRange_first_value,
        )
        .method(
            scope,
            "first_range",
            CssAtMediaQueryFeatureRange_first_range,
        )
        .method(scope, "name", CssAtMediaQueryFeatureRange_name)
        .method(
            scope,
            "second_value",
            CssAtMediaQueryFeatureRange_second_value,
        )
        .method(
            scope,
            "second_range",
            CssAtMediaQueryFeatureRange_second_range,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryRange>(
            scope,
            global,
            "CssAtMediaQueryRange",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "r_angle_token", CssAtMediaQueryRange_r_angle_token)
        .method(scope, "l_angle_token", CssAtMediaQueryRange_l_angle_token)
        .method(
            scope,
            "greater_than_equal_token",
            CssAtMediaQueryRange_greater_than_equal_token,
        )
        .method(
            scope,
            "less_than_equal_token",
            CssAtMediaQueryRange_less_than_equal_token,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttribute>(scope, global, "CssAttribute")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "l_brack_token", CssAttribute_l_brack_token)
        .method(scope, "attribute_name", CssAttribute_attribute_name)
        .method(scope, "attribute_meta", CssAttribute_attribute_meta)
        .method(scope, "r_brack_token", CssAttribute_r_brack_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttributeMatcher>(scope, global, "CssAttributeMatcher")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "matcher_type_token",
            CssAttributeMatcher_matcher_type_token,
        )
        .method(
            scope,
            "exactly_or_hyphen_token",
            CssAttributeMatcher_exactly_or_hyphen_token,
        )
        .method(scope, "prefix_token", CssAttributeMatcher_prefix_token)
        .method(scope, "suffix_token", CssAttributeMatcher_suffix_token)
        .method(
            scope,
            "times_assign_token",
            CssAttributeMatcher_times_assign_token,
        )
        .method(scope, "eq_token", CssAttributeMatcher_eq_token)
        .method(scope, "matcher_name", CssAttributeMatcher_matcher_name)
        .method(scope, "css_identifier", CssAttributeMatcher_css_identifier)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttributeMeta>(scope, global, "CssAttributeMeta")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "attribute_matcher",
            CssAttributeMeta_attribute_matcher,
        )
        .method(
            scope,
            "attribute_modifier",
            CssAttributeMeta_attribute_modifier,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttributeModifier>(
            scope,
            global,
            "CssAttributeModifier",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "i_token", CssAttributeModifier_i_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttributeName>(scope, global, "CssAttributeName")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "css_string", CssAttributeName_css_string)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttributeSelectorPattern>(
            scope,
            global,
            "CssAttributeSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "name", CssAttributeSelectorPattern_name)
        .method(
            scope,
            "attribute_list",
            CssAttributeSelectorPattern_attribute_list,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssBlock>(scope, global, "CssBlock")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "l_curly_token", CssBlock_l_curly_token)
        .method(scope, "declaration_list", CssBlock_declaration_list)
        .method(scope, "r_curly_token", CssBlock_r_curly_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssClassSelectorPattern>(
            scope,
            global,
            "CssClassSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "dot_token", CssClassSelectorPattern_dot_token)
        .method(scope, "name", CssClassSelectorPattern_name)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssCombinatorSelectorPattern>(
            scope,
            global,
            "CssCombinatorSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "left", CssCombinatorSelectorPattern_left)
        .method(
            scope,
            "combinator_token",
            CssCombinatorSelectorPattern_combinator_token,
        )
        .method(scope, "plus_token", CssCombinatorSelectorPattern_plus_token)
        .method(
            scope,
            "bitwise_not_token",
            CssCombinatorSelectorPattern_bitwise_not_token,
        )
        .method(
            scope,
            "css_space_literal_token",
            CssCombinatorSelectorPattern_css_space_literal_token,
        )
        .method(scope, "right", CssCombinatorSelectorPattern_right)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssCustomProperty>(scope, global, "CssCustomProperty")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "value_token", CssCustomProperty_value_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssDeclaration>(scope, global, "CssDeclaration")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "name", CssDeclaration_name)
        .method(
            scope,
            "css_custom_property",
            CssDeclaration_css_custom_property,
        )
        .method(scope, "colon_token", CssDeclaration_colon_token)
        .method(scope, "value", CssDeclaration_value)
        .method(scope, "important", CssDeclaration_important)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssDeclarationImportant>(
            scope,
            global,
            "CssDeclarationImportant",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "excl_token", CssDeclarationImportant_excl_token)
        .method(
            scope,
            "important_token",
            CssDeclarationImportant_important_token,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssDimension>(scope, global, "CssDimension")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "value", CssDimension_value)
        .method(scope, "unit", CssDimension_unit)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssIdSelectorPattern>(
            scope,
            global,
            "CssIdSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "hash_token", CssIdSelectorPattern_hash_token)
        .method(scope, "name", CssIdSelectorPattern_name)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssIdentifier>(scope, global, "CssIdentifier")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "value_token", CssIdentifier_value_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssKeyframesBlock>(scope, global, "CssKeyframesBlock")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "selectors", CssKeyframesBlock_selectors)
        .method(scope, "l_curly_token", CssKeyframesBlock_l_curly_token)
        .method(scope, "declarations", CssKeyframesBlock_declarations)
        .method(scope, "r_curly_token", CssKeyframesBlock_r_curly_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssKeyframesSelector>(
            scope,
            global,
            "CssKeyframesSelector",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "from_token", CssKeyframesSelector_from_token)
        .method(scope, "to_token", CssKeyframesSelector_to_token)
        .method(scope, "css_percentage", CssKeyframesSelector_css_percentage)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssNumber>(scope, global, "CssNumber")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "value_token", CssNumber_value_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssParameter>(scope, global, "CssParameter")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "any_css_value", CssParameter_any_css_value)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssPercentage>(scope, global, "CssPercentage")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "value", CssPercentage_value)
        .method(scope, "reminder_token", CssPercentage_reminder_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssPseudoClassSelectorPattern>(
            scope,
            global,
            "CssPseudoClassSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "colon_token",
            CssPseudoClassSelectorPattern_colon_token,
        )
        .method(scope, "name", CssPseudoClassSelectorPattern_name)
        .method(
            scope,
            "parameters",
            CssPseudoClassSelectorPattern_parameters,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssPseudoClassSelectorPatternParameters>(
            scope,
            global,
            "CssPseudoClassSelectorPatternParameters",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(
            scope,
            "l_paren_token",
            CssPseudoClassSelectorPatternParameters_l_paren_token,
        )
        .method(
            scope,
            "parameter",
            CssPseudoClassSelectorPatternParameters_parameter,
        )
        .method(
            scope,
            "r_paren_token",
            CssPseudoClassSelectorPatternParameters_r_paren_token,
        )
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssRatio>(scope, global, "CssRatio")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "numerator", CssRatio_numerator)
        .method(scope, "denominator", CssRatio_denominator)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssRoot>(scope, global, "CssRoot")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "rules", CssRoot_rules)
        .method(scope, "eof_token", CssRoot_eof_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssRule>(scope, global, "CssRule")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "prelude", CssRule_prelude)
        .method(scope, "block", CssRule_block)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssSelector>(scope, global, "CssSelector")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "pattern_list", CssSelector_pattern_list)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssSimpleFunction>(scope, global, "CssSimpleFunction")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "name", CssSimpleFunction_name)
        .method(scope, "l_paren_token", CssSimpleFunction_l_paren_token)
        .method(scope, "items", CssSimpleFunction_items)
        .method(scope, "r_paren_token", CssSimpleFunction_r_paren_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssString>(scope, global, "CssString")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "value_token", CssString_value_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssTypeSelectorPattern>(
            scope,
            global,
            "CssTypeSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "ident", CssTypeSelectorPattern_ident)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssUniversalSelectorPattern>(
            scope,
            global,
            "CssUniversalSelectorPattern",
        )
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "star_token", CssUniversalSelectorPattern_star_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssVarFunction>(scope, global, "CssVarFunction")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "var_token", CssVarFunction_var_token)
        .method(scope, "l_paren_token", CssVarFunction_l_paren_token)
        .method(scope, "property", CssVarFunction_property)
        .method(scope, "value", CssVarFunction_value)
        .method(scope, "r_paren_token", CssVarFunction_r_paren_token)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssVarFunctionValue>(scope, global, "CssVarFunctionValue")
        .extends::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>(scope)
        .method(scope, "comma_token", CssVarFunctionValue_comma_token)
        .method(scope, "value", CssVarFunctionValue_value)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssBogus>(scope, global, "CssBogus")
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAnySelectorPatternList>(
            scope,
            global,
            "CssAnySelectorPatternList",
        )
        .method(scope, "iter", CssAnySelectorPatternList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtKeyframesItemList>(
            scope,
            global,
            "CssAtKeyframesItemList",
        )
        .method(scope, "iter", CssAtKeyframesItemList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAtMediaQueryList>(scope, global, "CssAtMediaQueryList")
        .method(scope, "iter", CssAtMediaQueryList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssAttributeList>(scope, global, "CssAttributeList")
        .method(scope, "iter", CssAttributeList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssDeclarationList>(scope, global, "CssDeclarationList")
        .method(scope, "iter", CssDeclarationList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssKeyframesSelectorList>(
            scope,
            global,
            "CssKeyframesSelectorList",
        )
        .method(scope, "iter", CssKeyframesSelectorList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssParameterList>(scope, global, "CssParameterList")
        .method(scope, "iter", CssParameterList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssRuleList>(scope, global, "CssRuleList")
        .method(scope, "iter", CssRuleList_iter)
        .finish(scope);
    registry
        .build_class::<biome_css_syntax::CssSelectorList>(scope, global, "CssSelectorList")
        .method(scope, "iter", CssSelectorList_iter)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstNodeListIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::AnyCssRule,
        >>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstNodeListIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::AnyCssSelectorPattern,
        >>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstSeparatedListNodesIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssAtMediaQuery,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstNodeListIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssAttribute,
        >>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstNodeListIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssDeclaration,
        >>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstNodeListIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssKeyframesBlock,
        >>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstSeparatedListNodesIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssKeyframesSelector,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstNodeListIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssParameter,
        >>(scope)
        .iterable(scope, ToV8::to_v8)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstSeparatedListNodesIterator<
            biome_css_syntax::CssLanguage,
            biome_css_syntax::CssSelector,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
}
#[allow(non_snake_case)]
fn AstSeparatedListNodesIterator_next<'s, T: ToV8<'s>>(
    item: biome_rowan::SyntaxResult<T>,
    scope: &mut v8::HandleScope<'s>,
) -> anyhow::Result<v8::Local<'s, v8::Value>> {
    ToV8::to_v8(item?, scope)
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAnyFunction {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAnyFunction,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAnyFunction_css_simple_function<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAnyFunction::cast_ref(&*this).unwrap();
    let result = this.css_simple_function();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtKeyframes {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtKeyframes,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframes_at_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframes::cast_ref(&*this).unwrap();
    let result = this.at_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframes_keyframes_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframes::cast_ref(&*this).unwrap();
    let result = this.keyframes_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframes_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframes::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframes_css_string<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframes::cast_ref(&*this).unwrap();
    let result = this.css_string();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframes_body<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframes::cast_ref(&*this).unwrap();
    let result = this.body();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtKeyframesBody {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtKeyframesBody,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframesBody_l_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframesBody::cast_ref(&*this).unwrap();
    let result = this.l_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtKeyframesBody_items<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframesBody::cast_ref(&*this).unwrap();
    let result = this.items();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssAtKeyframesBody_r_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtKeyframesBody::cast_ref(&*this).unwrap();
    let result = this.r_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMedia {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMedia,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMedia_at_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMedia::cast_ref(&*this).unwrap();
    let result = this.at_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMedia_media_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMedia::cast_ref(&*this).unwrap();
    let result = this.media_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMedia_query_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMedia::cast_ref(&*this).unwrap();
    let result = this.query_list();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssAtMedia_l_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMedia::cast_ref(&*this).unwrap();
    let result = this.l_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMedia_body<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMedia::cast_ref(&*this).unwrap();
    let result = this.body();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMedia_r_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMedia::cast_ref(&*this).unwrap();
    let result = this.r_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQuery {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQuery,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQuery_condition_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQuery::cast_ref(&*this).unwrap();
    let result = this.condition_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQuery_or_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQuery::cast_ref(&*this).unwrap();
    let result = this.or_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQuery_only_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQuery::cast_ref(&*this).unwrap();
    let result = this.only_token();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQuery_ty<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQuery::cast_ref(&*this).unwrap();
    let result = this.ty();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQuery_consequent<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQuery::cast_ref(&*this).unwrap();
    let result = this.consequent();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryConsequent {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryConsequent,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryConsequent_and_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryConsequent::cast_ref(&*this).unwrap();
    let result = this.and_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryConsequent_condition_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryConsequent::cast_ref(&*this).unwrap();
    let result = this.condition_token();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryConsequent_ty<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryConsequent::cast_ref(&*this).unwrap();
    let result = this.ty();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryFeature {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryFeature,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeature_l_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeature::cast_ref(&*this).unwrap();
    let result = this.l_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeature_feature<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeature::cast_ref(&*this).unwrap();
    let result = this.feature();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeature_r_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeature::cast_ref(&*this).unwrap();
    let result = this.r_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryFeatureBoolean {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryFeatureBoolean,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureBoolean_css_identifier<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureBoolean::cast_ref(&*this).unwrap();
    let result = this.css_identifier();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryFeatureCompare {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryFeatureCompare,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureCompare_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureCompare::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureCompare_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureCompare::cast_ref(&*this).unwrap();
    let result = this.range();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureCompare_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureCompare::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryFeaturePlain {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryFeaturePlain,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeaturePlain_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeaturePlain::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeaturePlain_colon_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeaturePlain::cast_ref(&*this).unwrap();
    let result = this.colon_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeaturePlain_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeaturePlain::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryFeatureRange {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryFeatureRange,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureRange_first_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureRange::cast_ref(&*this).unwrap();
    let result = this.first_value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureRange_first_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureRange::cast_ref(&*this).unwrap();
    let result = this.first_range();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureRange_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureRange::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureRange_second_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureRange::cast_ref(&*this).unwrap();
    let result = this.second_value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryFeatureRange_second_range<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryFeatureRange::cast_ref(&*this).unwrap();
    let result = this.second_range();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAtMediaQueryRange {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAtMediaQueryRange,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryRange_r_angle_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryRange::cast_ref(&*this).unwrap();
    let result = this.r_angle_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryRange_l_angle_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryRange::cast_ref(&*this).unwrap();
    let result = this.l_angle_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryRange_greater_than_equal_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryRange::cast_ref(&*this).unwrap();
    let result = this.greater_than_equal_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAtMediaQueryRange_less_than_equal_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAtMediaQueryRange::cast_ref(&*this).unwrap();
    let result = this.less_than_equal_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAttribute {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAttribute,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAttribute_l_brack_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttribute::cast_ref(&*this).unwrap();
    let result = this.l_brack_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttribute_attribute_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttribute::cast_ref(&*this).unwrap();
    let result = this.attribute_name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttribute_attribute_meta<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttribute::cast_ref(&*this).unwrap();
    let result = this.attribute_meta();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
#[allow(non_snake_case)]
fn CssAttribute_r_brack_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttribute::cast_ref(&*this).unwrap();
    let result = this.r_brack_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAttributeMatcher {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAttributeMatcher,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_matcher_type_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.matcher_type_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_exactly_or_hyphen_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.exactly_or_hyphen_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_prefix_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.prefix_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_suffix_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.suffix_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_times_assign_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.times_assign_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_eq_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.eq_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_matcher_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.matcher_name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeMatcher_css_identifier<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMatcher::cast_ref(&*this).unwrap();
    let result = this.css_identifier();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAttributeMeta {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAttributeMeta,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAttributeMeta_attribute_matcher<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMeta::cast_ref(&*this).unwrap();
    let result = this.attribute_matcher();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
#[allow(non_snake_case)]
fn CssAttributeMeta_attribute_modifier<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeMeta::cast_ref(&*this).unwrap();
    let result = this.attribute_modifier();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAttributeModifier {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAttributeModifier,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAttributeModifier_i_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeModifier::cast_ref(&*this).unwrap();
    let result = this.i_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAttributeName {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAttributeName,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAttributeName_css_string<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeName::cast_ref(&*this).unwrap();
    let result = this.css_string();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssAttributeSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssAttributeSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssAttributeSelectorPattern_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssAttributeSelectorPattern_attribute_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssAttributeSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.attribute_list();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
impl<'s> ToV8<'s> for biome_css_syntax::CssBlock {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssBlock,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssBlock_l_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssBlock::cast_ref(&*this).unwrap();
    let result = this.l_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssBlock_declaration_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssBlock::cast_ref(&*this).unwrap();
    let result = this.declaration_list();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssBlock_r_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssBlock::cast_ref(&*this).unwrap();
    let result = this.r_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssClassSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssClassSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssClassSelectorPattern_dot_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssClassSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.dot_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssClassSelectorPattern_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssClassSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssCombinatorSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssCombinatorSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssCombinatorSelectorPattern_left<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCombinatorSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.left();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssCombinatorSelectorPattern_combinator_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCombinatorSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.combinator_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssCombinatorSelectorPattern_plus_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCombinatorSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.plus_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssCombinatorSelectorPattern_bitwise_not_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCombinatorSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.bitwise_not_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssCombinatorSelectorPattern_css_space_literal_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCombinatorSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.css_space_literal_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssCombinatorSelectorPattern_right<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCombinatorSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.right();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssCustomProperty {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssCustomProperty,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssCustomProperty_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssCustomProperty::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssDeclaration {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssDeclaration,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssDeclaration_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclaration::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssDeclaration_css_custom_property<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclaration::cast_ref(&*this).unwrap();
    let result = this.css_custom_property();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssDeclaration_colon_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclaration::cast_ref(&*this).unwrap();
    let result = this.colon_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssDeclaration_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclaration::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssDeclaration_important<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclaration::cast_ref(&*this).unwrap();
    let result = this.important();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssDeclarationImportant {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssDeclarationImportant,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssDeclarationImportant_excl_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclarationImportant::cast_ref(&*this).unwrap();
    let result = this.excl_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssDeclarationImportant_important_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDeclarationImportant::cast_ref(&*this).unwrap();
    let result = this.important_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssDimension {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssDimension,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssDimension_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDimension::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssDimension_unit<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssDimension::cast_ref(&*this).unwrap();
    let result = this.unit();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssIdSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssIdSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssIdSelectorPattern_hash_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssIdSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.hash_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssIdSelectorPattern_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssIdSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssIdentifier {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssIdentifier,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssIdentifier_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssIdentifier::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssKeyframesBlock {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssKeyframesBlock,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssKeyframesBlock_selectors<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesBlock::cast_ref(&*this).unwrap();
    let result = this.selectors();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssKeyframesBlock_l_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesBlock::cast_ref(&*this).unwrap();
    let result = this.l_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssKeyframesBlock_declarations<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesBlock::cast_ref(&*this).unwrap();
    let result = this.declarations();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssKeyframesBlock_r_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesBlock::cast_ref(&*this).unwrap();
    let result = this.r_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssKeyframesSelector {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssKeyframesSelector,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssKeyframesSelector_from_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesSelector::cast_ref(&*this).unwrap();
    let result = this.from_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssKeyframesSelector_to_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesSelector::cast_ref(&*this).unwrap();
    let result = this.to_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssKeyframesSelector_css_percentage<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssKeyframesSelector::cast_ref(&*this).unwrap();
    let result = this.css_percentage();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssNumber {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssNumber,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssNumber_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssNumber::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssParameter {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssParameter,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssParameter_any_css_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssParameter::cast_ref(&*this).unwrap();
    let result = this.any_css_value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssPercentage {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssPercentage,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssPercentage_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPercentage::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssPercentage_reminder_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPercentage::cast_ref(&*this).unwrap();
    let result = this.reminder_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssPseudoClassSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssPseudoClassSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssPseudoClassSelectorPattern_colon_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPseudoClassSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.colon_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssPseudoClassSelectorPattern_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPseudoClassSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssPseudoClassSelectorPattern_parameters<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPseudoClassSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.parameters();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssPseudoClassSelectorPatternParameters {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssPseudoClassSelectorPatternParameters,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssPseudoClassSelectorPatternParameters_l_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPseudoClassSelectorPatternParameters::cast_ref(&*this).unwrap();
    let result = this.l_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssPseudoClassSelectorPatternParameters_parameter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPseudoClassSelectorPatternParameters::cast_ref(&*this).unwrap();
    let result = this.parameter();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssPseudoClassSelectorPatternParameters_r_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssPseudoClassSelectorPatternParameters::cast_ref(&*this).unwrap();
    let result = this.r_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssRatio {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssRatio,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssRatio_numerator<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssRatio::cast_ref(&*this).unwrap();
    let result = this.numerator();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssRatio_denominator<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssRatio::cast_ref(&*this).unwrap();
    let result = this.denominator();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssRoot {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssRoot,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssRoot_rules<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssRoot::cast_ref(&*this).unwrap();
    let result = this.rules();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssRoot_eof_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssRoot::cast_ref(&*this).unwrap();
    let result = this.eof_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssRule {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssRule,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssRule_prelude<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssRule::cast_ref(&*this).unwrap();
    let result = this.prelude();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssRule_block<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssRule::cast_ref(&*this).unwrap();
    let result = this.block();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssSelector {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssSelector,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssSelector_pattern_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssSelector::cast_ref(&*this).unwrap();
    let result = this.pattern_list();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
impl<'s> ToV8<'s> for biome_css_syntax::CssSimpleFunction {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssSimpleFunction,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssSimpleFunction_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssSimpleFunction::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssSimpleFunction_l_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssSimpleFunction::cast_ref(&*this).unwrap();
    let result = this.l_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssSimpleFunction_items<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssSimpleFunction::cast_ref(&*this).unwrap();
    let result = this.items();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn CssSimpleFunction_r_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssSimpleFunction::cast_ref(&*this).unwrap();
    let result = this.r_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssString {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssString,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssString_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssString::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssTypeSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssTypeSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssTypeSelectorPattern_ident<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssTypeSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.ident();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssUniversalSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssUniversalSelectorPattern,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssUniversalSelectorPattern_star_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssUniversalSelectorPattern::cast_ref(&*this).unwrap();
    let result = this.star_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssVarFunction {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssVarFunction,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssVarFunction_var_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunction::cast_ref(&*this).unwrap();
    let result = this.var_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssVarFunction_l_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunction::cast_ref(&*this).unwrap();
    let result = this.l_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssVarFunction_property<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunction::cast_ref(&*this).unwrap();
    let result = this.property();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssVarFunction_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunction::cast_ref(&*this).unwrap();
    let result = this.value();
    if let Some(result) = result {
        let result = ToV8::to_v8(result, scope).unwrap();
        res.set(result);
    } else {
        res.set_undefined();
    }
}
#[allow(non_snake_case)]
fn CssVarFunction_r_paren_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunction::cast_ref(&*this).unwrap();
    let result = this.r_paren_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::CssVarFunctionValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_css_syntax::CssVarFunctionValue,
            biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn CssVarFunctionValue_comma_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunctionValue::cast_ref(&*this).unwrap();
    let result = this.comma_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn CssVarFunctionValue_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_css_syntax::CssVarFunctionValue::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::AnyCssAtMediaQueryFeatureType {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::CssAtMediaQueryFeatureBoolean(node) => ToV8::to_v8(node, scope),
            Self::CssAtMediaQueryFeatureCompare(node) => ToV8::to_v8(node, scope),
            Self::CssAtMediaQueryFeaturePlain(node) => ToV8::to_v8(node, scope),
            Self::CssAtMediaQueryFeatureRange(node) => ToV8::to_v8(node, scope),
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::AnyCssAtMediaQueryType {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::CssAtMediaQueryFeature(node) => ToV8::to_v8(node, scope),
            Self::CssIdentifier(node) => ToV8::to_v8(node, scope),
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::AnyCssAtRule {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::CssAtKeyframes(node) => ToV8::to_v8(node, scope),
            Self::CssAtMedia(node) => ToV8::to_v8(node, scope),
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::AnyCssRule {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::AnyCssAtRule(node) => ToV8::to_v8(node, scope),
            Self::CssRule(node) => ToV8::to_v8(node, scope),
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::AnyCssSelectorPattern {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::CssAttributeSelectorPattern(node) => ToV8::to_v8(node, scope),
            Self::CssClassSelectorPattern(node) => ToV8::to_v8(node, scope),
            Self::CssCombinatorSelectorPattern(node) => ToV8::to_v8(node, scope),
            Self::CssIdSelectorPattern(node) => ToV8::to_v8(node, scope),
            Self::CssPseudoClassSelectorPattern(node) => ToV8::to_v8(node, scope),
            Self::CssTypeSelectorPattern(node) => ToV8::to_v8(node, scope),
            Self::CssUniversalSelectorPattern(node) => ToV8::to_v8(node, scope),
        }
    }
}
impl<'s> ToV8<'s> for biome_css_syntax::AnyCssValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::CssAnyFunction(node) => ToV8::to_v8(node, scope),
            Self::CssCustomProperty(node) => ToV8::to_v8(node, scope),
            Self::CssDimension(node) => ToV8::to_v8(node, scope),
            Self::CssIdentifier(node) => ToV8::to_v8(node, scope),
            Self::CssNumber(node) => ToV8::to_v8(node, scope),
            Self::CssRatio(node) => ToV8::to_v8(node, scope),
            Self::CssString(node) => ToV8::to_v8(node, scope),
        }
    }
}
crate::convert::impl_convert_native!(biome_css_syntax::CssBogus);
crate::convert::impl_convert_native!(biome_css_syntax::CssAnySelectorPatternList);
#[allow(non_snake_case)]
fn CssAnySelectorPatternList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_css_syntax::CssAnySelectorPatternList>::from_v8(scope, this)
        .unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssAtKeyframesItemList);
#[allow(non_snake_case)]
fn CssAtKeyframesItemList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this =
        std::cell::Ref::<biome_css_syntax::CssAtKeyframesItemList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssAtMediaQueryList);
#[allow(non_snake_case)]
fn CssAtMediaQueryList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this =
        std::cell::Ref::<biome_css_syntax::CssAtMediaQueryList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssAttributeList);
#[allow(non_snake_case)]
fn CssAttributeList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_css_syntax::CssAttributeList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssDeclarationList);
#[allow(non_snake_case)]
fn CssDeclarationList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this =
        std::cell::Ref::<biome_css_syntax::CssDeclarationList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssKeyframesSelectorList);
#[allow(non_snake_case)]
fn CssKeyframesSelectorList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this =
        std::cell::Ref::<biome_css_syntax::CssKeyframesSelectorList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssParameterList);
#[allow(non_snake_case)]
fn CssParameterList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_css_syntax::CssParameterList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssRuleList);
#[allow(non_snake_case)]
fn CssRuleList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_css_syntax::CssRuleList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_css_syntax::CssSelectorList);
#[allow(non_snake_case)]
fn CssSelectorList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_css_syntax::CssSelectorList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate :: convert :: impl_convert_native ! (biome_rowan :: AstNodeListIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: AnyCssRule >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstNodeListIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: AnyCssSelectorPattern >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstSeparatedListNodesIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssAtMediaQuery >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstNodeListIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssAttribute >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstNodeListIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssDeclaration >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstNodeListIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssKeyframesBlock >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstSeparatedListNodesIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssKeyframesSelector >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstNodeListIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssParameter >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstSeparatedListNodesIterator < biome_css_syntax :: CssLanguage , biome_css_syntax :: CssSelector >);
