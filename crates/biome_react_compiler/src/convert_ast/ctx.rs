use biome_js_syntax::{AnyJsRoot, TextRange, TextSize};
use biome_line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use react_compiler_ast::common::{BaseNode, Position, SourceLocation};

pub(crate) struct ConvertCtx<'a> {
    pub(crate) root: &'a AnyJsRoot,
    line_index: LineIndex,
    /// UTF-16 code unit offset of the start of each line, used to compute the
    /// absolute UTF-16 `index` that Babel positions carry alongside
    /// line/column.
    utf16_line_starts: Vec<u32>,
}

impl<'a> ConvertCtx<'a> {
    pub(super) fn new(root: &'a AnyJsRoot, source: &'a str) -> Self {
        let line_index = LineIndex::new(source);
        let utf16_line_starts = utf16_line_starts(&line_index, TextSize::of(source));

        Self {
            root,
            line_index,
            utf16_line_starts,
        }
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
            node_id: Some(node_id_from_range(range)),
        }
    }

    pub(crate) fn source_location(&self, range: TextRange) -> SourceLocation {
        SourceLocation {
            start: self.position(range.start()),
            end: self.position(range.end()),
            filename: None,
            identifier_name: None,
        }
    }

    fn position(&self, offset: TextSize) -> Position {
        let line_col = self
            .line_index
            .line_col(offset)
            .unwrap_or(LineCol { line: 0, col: 0 });
        let wide = self
            .line_index
            .to_wide(WideEncoding::Utf16, line_col)
            .unwrap_or(WideLineCol {
                line: line_col.line,
                col: line_col.col,
            });
        let line_start = self
            .utf16_line_starts
            .get(wide.line as usize)
            .copied()
            .unwrap_or_default();
        Position {
            line: wide.line + 1,
            column: wide.col,
            index: Some(line_start + wide.col),
        }
    }
}

/// Computes the absolute UTF-16 code unit offset of each line start by
/// accumulating the UTF-16 length of every line (including its terminator).
fn utf16_line_starts(line_index: &LineIndex, source_len: TextSize) -> Vec<u32> {
    let mut starts = Vec::with_capacity(line_index.newlines.len());
    let mut acc: u32 = 0;
    for (line, start) in line_index.newlines.iter().enumerate() {
        starts.push(acc);
        let end = line_index
            .newlines
            .get(line + 1)
            .copied()
            .unwrap_or(source_len);
        let line_col = LineCol {
            line: line as u32,
            col: (end - *start).into(),
        };
        if let Some(wide) = line_index.to_wide(WideEncoding::Utf16, line_col) {
            acc += wide.col;
        }
    }
    starts
}

fn node_id_from_range(range: TextRange) -> u32 {
    let start: u32 = range.start().into();
    start.saturating_add(1)
}
