use crate::lexer::TomlLexContext;
use crate::semantic::{KeyPart, KeyTracker};
use crate::token_source::TomlTokenSource;
use biome_parser::ParserContext;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_rowan::TextRange;
use biome_toml_syntax::TomlSyntaxKind::{self, *};
use biome_toml_syntax::string_ext::unescape_basic_string;
use std::borrow::Cow;

pub(crate) struct TomlParser<'source> {
    context: ParserContext<TomlSyntaxKind>,
    source: TomlTokenSource<'source>,
    text: &'source str,
    /// The decoded parts of the key that is being parsed.
    key_parts: Vec<KeyPart<'source>>,
    /// `false` if a part of the key that is being parsed is missing or invalid.
    key_is_valid: bool,
    keys: KeyTracker<'source>,
}

impl<'source> TomlParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: TomlTokenSource::from_str(source),
            text: source,
            key_parts: Vec::new(),
            key_is_valid: true,
            keys: KeyTracker::default(),
        }
    }

    /// Starts collecting the parts of a new key.
    pub fn start_key(&mut self) {
        self.key_parts.clear();
        self.key_is_valid = true;
    }

    /// Adds the current token, a bare or quoted key, to the parts of the key.
    pub fn push_key_part(&mut self) {
        let range = self.cur_range();
        let text = &self.text[range];
        let name = match self.cur() {
            TOML_BARE_KEY_LITERAL => Some(Cow::Borrowed(text)),
            TOML_BASIC_STRING_LITERAL => strip_quotes(text, '"').map(|inner| {
                if inner.contains('\\') {
                    Cow::Owned(unescape_basic_string(inner))
                } else {
                    Cow::Borrowed(inner)
                }
            }),
            TOML_LITERAL_STRING_LITERAL => strip_quotes(text, '\'').map(Cow::Borrowed),
            _ => None,
        };

        match name {
            Some(name) => self.key_parts.push(KeyPart { name, range }),
            None => self.key_is_valid = false,
        }
    }

    /// Marks the key that is being parsed as invalid, so it isn't checked for
    /// conflicts.
    pub fn invalidate_key(&mut self) {
        self.key_is_valid = false;
    }

    /// Defines the parsed key of a key/value pair.
    pub fn define_key_value(&mut self) {
        if self.key_is_valid && !self.key_parts.is_empty() {
            self.keys.define_key_value(&self.key_parts);
        }
    }

    /// Starts the section of the `[table]` header whose key was just parsed.
    pub fn start_table(&mut self, range: TextRange) {
        let parts =
            (self.key_is_valid && !self.key_parts.is_empty()).then_some(self.key_parts.as_slice());
        self.keys.start_table(parts, range);
    }

    /// Starts the section of the `[[table]]` header whose key was just parsed.
    pub fn start_array_table(&mut self, range: TextRange) {
        let parts =
            (self.key_is_valid && !self.key_parts.is_empty()).then_some(self.key_parts.as_slice());
        self.keys.start_array_table(parts, range);
    }

    pub fn start_inline_table(&mut self, range: TextRange) {
        self.keys.start_inline_table(range);
    }

    pub fn end_inline_table(&mut self) {
        self.keys.end_inline_table();
    }

    /// Lexes the current token again in `context`.
    pub fn re_lex(&mut self, context: TomlLexContext) -> TomlSyntaxKind {
        self.source.re_lex(context)
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<TomlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let parse_diagnostics = merge_diagnostics(parse_diagnostics, self.keys.finish());
        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for TomlParser<'source> {
    type Kind = TomlSyntaxKind;
    type Source = TomlTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}

/// Returns the text between the quotes of a single-line string, or `None` if
/// the string is unterminated.
fn strip_quotes(text: &str, quote: char) -> Option<&str> {
    if text.len() >= 2 && text.ends_with(quote) {
        Some(&text[1..text.len() - 1])
    } else {
        None
    }
}
