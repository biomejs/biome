use biome_astro_syntax::{AstroFileSource, AstroSyntaxKind::*, T};
use biome_parser::prelude::*;
use biome_rowan::{TextRange, TextSize};

#[derive(Debug, Clone)]
pub struct AstroLexer<'src> {
    source: &'src str,
    current: usize,
    file_source: AstroFileSource,
}

impl<'src> AstroLexer<'src> {
    pub fn new(source: &'src str, file_source: AstroFileSource) -> Self {
        Self {
            source,
            current: 0,
            file_source,
        }
    }

    pub fn next_token(&mut self) -> LexerCheckpoint<AstroSyntaxKind> {
        self.skip_whitespace();

        if self.current >= self.source.len() {
            return LexerCheckpoint {
                current_start: TextSize::from(self.current as u32),
                current_flags: TokenFlags::empty(),
                current_kind: EOF,
            };
        }

        let start = self.current;
        let kind = self.read_token();

        LexerCheckpoint {
            current_start: TextSize::from(start as u32),
            current_flags: TokenFlags::empty(),
            current_kind: kind,
        }
    }

    fn read_token(&mut self) -> AstroSyntaxKind {
        let start_byte = self.current_byte();
        
        match start_byte {
            Some(b'{') => {
                self.advance();
                T!['{']
            }
            Some(b'}') => {
                self.advance();
                T!['}']
            }
            Some(b'<') => {
                self.advance();
                if self.starts_with("!--") {
                    self.read_comment()
                } else if self.starts_with("![CDATA[") {
                    self.read_cdata()
                } else if self.starts_with("!") {
                    self.read_doctype()
                } else {
                    T!['<']
                }
            }
            Some(b'>') => {
                self.advance();
                T!['>']
            }
            Some(b'/') => {
                self.advance();
                T!['/']
            }
            Some(b'=') => {
                self.advance();
                T!['=']
            }
            Some(b'`') => {
                self.advance();
                T!['`']
            }
            Some(b'.') if self.starts_with("...") => {
                self.advance_by(3);
                T!['...']
            }
            Some(b'-') if self.starts_with("---") => {
                self.advance_by(3);
                T!['---']
            }
            Some(b'"') | Some(b'\'') => self.read_string_literal(),
            _ => self.read_text_or_name(),
        }
    }

    fn read_comment(&mut self) -> AstroSyntaxKind {
        // Skip "!--"
        self.advance_by(3);
        
        // Read until "-->"
        while !self.starts_with("-->") && !self.is_at_end() {
            self.advance();
        }
        
        if self.starts_with("-->") {
            self.advance_by(3);
        }
        
        ASTRO_COMMENT
    }

    fn read_cdata(&mut self) -> AstroSyntaxKind {
        // Skip "![CDATA["
        self.advance_by(8);
        
        // Read until "]]>"
        while !self.starts_with("]]>") && !self.is_at_end() {
            self.advance();
        }
        
        if self.starts_with("]]>") {
            self.advance_by(3);
        }
        
        ASTRO_COMMENT
    }

    fn read_doctype(&mut self) -> AstroSyntaxKind {
        // This is a simplified DOCTYPE lexer
        while self.current_byte() != Some(b'>') && !self.is_at_end() {
            self.advance();
        }
        
        if self.current_byte() == Some(b'>') {
            self.advance();
        }
        
        ASTRO_DOCTYPE
    }

    fn read_string_literal(&mut self) -> AstroSyntaxKind {
        let quote = self.current_byte().unwrap();
        self.advance(); // Skip opening quote
        
        while let Some(current) = self.current_byte() {
            if current == quote {
                self.advance(); // Skip closing quote
                break;
            }
            if current == b'\\' {
                self.advance(); // Skip escape character
                if !self.is_at_end() {
                    self.advance(); // Skip escaped character
                }
            } else {
                self.advance();
            }
        }
        
        ASTRO_STRING_LITERAL
    }

    fn read_text_or_name(&mut self) -> AstroSyntaxKind {
        // This is a simplified implementation
        // In a real parser, we'd need more sophisticated logic
        // to distinguish between different types of content
        
        while let Some(current) = self.current_byte() {
            match current {
                b'<' | b'>' | b'{' | b'}' | b'=' | b'/' | b'`' => break,
                b' ' | b'\t' | b'\n' | b'\r' => break,
                _ => self.advance(),
            }
        }
        
        // For now, treat everything as text
        // In a real implementation, we'd distinguish between:
        // - ASTRO_ELEMENT_NAME
        // - ASTRO_COMPONENT_NAME  
        // - ASTRO_ATTRIBUTE_NAME
        // - ASTRO_TEXT
        // etc.
        ASTRO_TEXT
    }

    fn skip_whitespace(&mut self) {
        while let Some(current) = self.current_byte() {
            match current {
                b' ' | b'\t' | b'\n' | b'\r' => self.advance(),
                _ => break,
            }
        }
    }

    fn current_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.current).copied()
    }

    fn advance(&mut self) {
        if self.current < self.source.len() {
            self.current += 1;
        }
    }

    fn advance_by(&mut self, count: usize) {
        for _ in 0..count {
            self.advance();
        }
    }

    fn starts_with(&self, pattern: &str) -> bool {
        self.source[self.current..].starts_with(pattern)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}