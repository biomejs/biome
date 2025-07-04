use crate::lexer::AstroLexer;
use biome_astro_syntax::{AstroFileSource, AstroSyntaxKind};
use biome_parser::prelude::*;
use biome_rowan::{TextRange, TextSize};

#[derive(Debug)]
pub struct TokenSource<'src> {
    lexer: AstroLexer<'src>,
    trivia: Vec<SyntaxTrivia>,
    current: Option<LexerCheckpoint<AstroSyntaxKind>>,
}

impl<'src> TokenSource<'src> {
    pub fn new(source: &'src str, file_source: AstroFileSource) -> Self {
        let mut lexer = AstroLexer::new(source, file_source);
        let current = Some(lexer.next_token());

        Self {
            lexer,
            trivia: Vec::new(),
            current,
        }
    }

    pub fn trivia(self) -> Vec<SyntaxTrivia> {
        self.trivia
    }

    fn advance(&mut self) {
        if let Some(current) = &self.current {
            if current.current_kind != AstroSyntaxKind::EOF {
                self.current = Some(self.lexer.next_token());
            }
        }
    }
}

impl<'src> BumpWithContext for TokenSource<'src> {
    type Kind = AstroSyntaxKind;
    type Context = ();

    fn peek2(&mut self, _context: Self::Context) -> Option<Self::Kind> {
        // For simplicity, we don't implement lookahead
        None
    }

    fn bump(&mut self, _context: Self::Context) {
        self.advance();
    }

    fn bump_remap(&mut self, kind: Self::Kind, _context: Self::Context) {
        if let Some(ref mut current) = self.current {
            current.current_kind = kind;
        }
        self.advance();
    }
}

impl<'src> TokenSource<AstroSyntaxKind> for TokenSource<'src> {
    fn current(&self) -> Self::Kind {
        self.current
            .as_ref()
            .map(|c| c.current_kind)
            .unwrap_or(AstroSyntaxKind::EOF)
    }

    fn current_range(&self) -> TextRange {
        if let Some(current) = &self.current {
            TextRange::new(current.current_start, current.current_start)
        } else {
            TextRange::new(TextSize::from(0), TextSize::from(0))
        }
    }

    fn text(&self) -> &str {
        ""
    }

    fn has_preceding_line_break(&self) -> bool {
        false
    }

    fn bump(&mut self) {
        self.advance();
    }

    fn bump_remap(&mut self, kind: Self::Kind) {
        if let Some(ref mut current) = self.current {
            current.current_kind = kind;
        }
        self.advance();
    }

    fn has_nth_preceding_line_break(&self, _n: usize) -> bool {
        false
    }

    fn nth(&self, _n: usize) -> Self::Kind {
        self.current()
    }

    fn push_trivia(&mut self, trivia: SyntaxTrivia) {
        self.trivia.push(trivia);
    }
}