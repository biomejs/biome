// This file is auto-generated. Do not edit manually.
// It will be regenerated when running codegen.

use crate::*;
use biome_astro_syntax::{AstroSyntaxKind::*, AstroSyntaxNode, AstroSyntaxToken, T};
use biome_rowan::NodeOrToken;

// Common tokens
pub fn l_curly_token() -> AstroSyntaxToken {
    token(T!['{']).build()
}

pub fn r_curly_token() -> AstroSyntaxToken {
    token(T!['}']).build()
}

pub fn l_angle_token() -> AstroSyntaxToken {
    token(T!['<']).build()
}

pub fn r_angle_token() -> AstroSyntaxToken {
    token(T!['>']).build()
}

pub fn slash_token() -> AstroSyntaxToken {
    token(T!['/']).build()
}

pub fn equals_token() -> AstroSyntaxToken {
    token(T!['=']).build()
}

pub fn dot3_token() -> AstroSyntaxToken {
    token(T!['...']).build()
}

pub fn backtick_token() -> AstroSyntaxToken {
    token(T!['`']).build()
}

pub fn minus3_token() -> AstroSyntaxToken {
    token(T!['---']).build()
}

pub fn doctype_token() -> AstroSyntaxToken {
    token(T![doctype]).build()
}

pub fn html_token() -> AstroSyntaxToken {
    token(T![html]).build()
}

pub fn eof_token() -> AstroSyntaxToken {
    token(EOF).build()
}

// Common text tokens
pub fn astro_string_literal_token(text: &str) -> AstroSyntaxToken {
    token(ASTRO_STRING_LITERAL).with_text(text).build()
}

pub fn astro_text_token(text: &str) -> AstroSyntaxToken {
    token(ASTRO_TEXT).with_text(text).build()
}

pub fn astro_js_content_token(text: &str) -> AstroSyntaxToken {
    token(ASTRO_JS_CONTENT).with_text(text).build()
}

pub fn astro_element_name_token(text: &str) -> AstroSyntaxToken {
    token(ASTRO_ELEMENT_NAME).with_text(text).build()
}

pub fn astro_component_name_token(text: &str) -> AstroSyntaxToken {
    token(ASTRO_COMPONENT_NAME).with_text(text).build()
}

pub fn astro_attribute_name_token(text: &str) -> AstroSyntaxToken {
    token(ASTRO_ATTRIBUTE_NAME).with_text(text).build()
}