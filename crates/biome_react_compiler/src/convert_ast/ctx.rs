use biome_js_syntax::{AnyJsRoot, TextRange};
use react_compiler_ast::common::{BaseNode, Position, SourceLocation};

pub(crate) struct ConvertCtx<'a> {
    pub(crate) root: &'a AnyJsRoot,
    line_offsets: Vec<u32>,
}

impl<'a> ConvertCtx<'a> {
    pub(super) fn new(root: &'a AnyJsRoot, source: &'a str) -> Self {
        let mut line_offsets = vec![0];
        for (index, char) in source.char_indices() {
            if char == '\n' {
                line_offsets.push((index + 1) as u32);
            }
        }

        Self { root, line_offsets }
    }

    pub(crate) fn base(&self, range: TextRange) -> BaseNode {
        BaseNode {
            node_type: None,
            start: Some(range.start().into()),
            end: Some(range.end().into()),
            loc: Some(self.source_location(range)),
            range: None,
            extra: None,
            leading_comments: None,
            inner_comments: None,
            trailing_comments: None,
        }
    }

    pub(crate) fn source_location(&self, range: TextRange) -> SourceLocation {
        SourceLocation {
            start: self.position(range.start().into()),
            end: self.position(range.end().into()),
            filename: None,
            identifier_name: None,
        }
    }

    fn position(&self, offset: u32) -> Position {
        let line_index = match self.line_offsets.binary_search(&offset) {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };
        let line_start = self.line_offsets[line_index];
        Position {
            line: (line_index + 1) as u32,
            column: offset - line_start,
            index: Some(offset),
        }
    }
}
