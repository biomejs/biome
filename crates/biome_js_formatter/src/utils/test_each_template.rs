use crate::js::auxiliary::template_element::TemplateElementOptions;
use crate::js::lists::template_element_list::TemplateElementIndention;
use crate::prelude::*;
use biome_formatter::printer::Printer;
use biome_formatter::{
    format_args, write, CstFormatContext, FormatOptions, RemoveSoftLinesBuffer, VecBuffer,
};
use biome_js_syntax::{AnyJsTemplateElement, JsTemplateElementList};
use biome_text_size::{TextRange, TextSize};
use std::cmp;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
enum EachTemplateElement {
    /// A significant value in the test each table. It's a row element.
    Column(EachTemplateColumn),
    /// Indicates the end of the current row.
    LineBreak,
}

/// Row element containing the column information.
#[derive(Debug)]
struct EachTemplateColumn {
    /// Formatted text of the column.
    text: String,
    /// Formatted text width.
    width: TextSize,
    /// Corresponding range for the text to replace it.
    range: TextRange,
    /// Indicates the line break in the text.
    will_break: bool,
}

impl EachTemplateColumn {
    fn new(text: String, range: TextRange, will_break: bool) -> Self {
        let width = TextSize::try_from(text.width())
            .expect("integer overflow while converting a text width to `TextSize`");

        EachTemplateColumn {
            text,
            width,
            range,
            will_break,
        }
    }
}

struct EachTemplateTableBuilder {
    /// Holds information about the current row.
    current_row: EachTemplateCurrentRow,
    /// Information about all rows.
    rows: Vec<EachTemplateRow>,
    /// Contains the maximum length of each column of all rows.
    columns_width: Vec<TextSize>,
    /// Elements for formatting.
    elements: Vec<EachTemplateElement>,
}

impl EachTemplateTableBuilder {
    fn new() -> Self {
        EachTemplateTableBuilder {
            current_row: EachTemplateCurrentRow::new(),
            rows: Vec::new(),
            columns_width: Vec::new(),
            elements: Vec::new(),
        }
    }

    /// Adds a new item to the buffer.
    fn entry(&mut self, element: EachTemplateElement) {
        match &element {
            EachTemplateElement::Column(column) => {
                if column.will_break {
                    self.current_row.has_line_break_column = true;
                }

                // if there was no column with a line break, then add width of the current column to the buffer
                if !self.current_row.has_line_break_column {
                    self.current_row.column_widths.push(column.width);
                }
            }
            EachTemplateElement::LineBreak => {
                self.next_row();
            }
        }
        self.elements.push(element);
    }

    /// Advance the table state to a new row.
    /// Merge the current row columns width with the table ones if row doesn't contain a line break column.
    fn next_row(&mut self) {
        if !self.current_row.has_line_break_column {
            let table_column_width_iter = self.columns_width.iter_mut();
            let mut row_column_width_iter = self.current_row.column_widths.iter();

            // find the maximum length between the table and the current row
            for table_column_width in table_column_width_iter {
                let row_column_width = match row_column_width_iter.next() {
                    Some(width) => width,
                    _ => break,
                };
                *table_column_width = cmp::max(*table_column_width, *row_column_width);
            }

            // add the remaining items to the buffer
            self.columns_width.extend(row_column_width_iter);
        }

        // save information about the row to buffer
        self.rows.push(EachTemplateRow {
            has_line_break_column: self.current_row.has_line_break_column,
        });

        self.current_row.reset();
    }

    fn finish(mut self) -> EachTemplateTable {
        self.next_row();

        EachTemplateTable {
            rows: self.rows,
            columns_width: self.columns_width,
            elements: self.elements,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EachTemplateTable {
    /// Information about all rows.
    rows: Vec<EachTemplateRow>,
    /// Contains the maximum length of each column of all rows.
    columns_width: Vec<TextSize>,
    /// Elements for formatting.
    elements: Vec<EachTemplateElement>,
}

#[derive(Debug)]
struct EachTemplateCurrentRow {
    /// Contains the maximum length of the current column.
    column_widths: Vec<TextSize>,
    /// Whether the current row contains a column with a line break.
    has_line_break_column: bool,
}

impl EachTemplateCurrentRow {
    fn new() -> Self {
        EachTemplateCurrentRow {
            column_widths: Vec::new(),
            has_line_break_column: false,
        }
    }

    /// Reset the state of the current row when moving to the next line.
    fn reset(&mut self) {
        self.column_widths.clear();
        self.has_line_break_column = false;
    }
}

#[derive(Debug)]
struct EachTemplateRow {
    /// Whether the current row contains a column with a line break.
    has_line_break_column: bool,
}

/// Separator between columns in a row.
struct EachTemplateSeparator;

impl Format<JsFormatContext> for EachTemplateSeparator {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [text("|")])
    }
}

impl EachTemplateTable {
    pub(crate) fn from(list: &JsTemplateElementList, f: &mut JsFormatter) -> FormatResult<Self> {
        let mut iter = list.into_iter().peekable();

        let mut builder = EachTemplateTableBuilder::new();

        // the table must have a header
        // e.g. a | b | expected
        let header = match iter.next() {
            Some(AnyJsTemplateElement::JsTemplateChunkElement(header)) => header,
            // we check this case in `is_test_each_pattern` and `is_test_each_pattern_elements` functions
            _ => return Err(FormatError::SyntaxError),
        };

        // It's safe to mark the header as checked here because we check that node doesn't have any trivia
        // when we call `is_test_each_pattern`
        f.context()
            .comments()
            .mark_suppression_checked(header.syntax());

        write!(f, [format_removed(&header.template_chunk_token()?)])?;

        let header = header.template_chunk_token()?;

        // split the header to get columns
        for column in header.text_trimmed().split_terminator('|') {
            let text = column.trim().to_string();
            let range = header.text_range();

            let column = EachTemplateColumn::new(text, range, false);

            builder.entry(EachTemplateElement::Column(column));
        }

        builder.entry(EachTemplateElement::LineBreak);

        while let Some(element) = iter.next() {
            match element {
                AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                    // It's safe to mark the element as checked here because we check that node doesn't have any trivia
                    // when we call `is_test_each_pattern`
                    f.context()
                        .comments()
                        .mark_suppression_checked(element.syntax());

                    write!(f, [format_removed(&element.template_chunk_token()?)])?;

                    let has_line_break = element
                        .template_chunk_token()?
                        .text_trimmed()
                        .contains('\n');
                    let is_last = iter.peek().is_none();

                    // go to the next line if the current element contains a line break
                    if has_line_break && !is_last {
                        builder.entry(EachTemplateElement::LineBreak);
                    }
                }
                AnyJsTemplateElement::JsTemplateElement(element) => {
                    let mut vec_buffer = VecBuffer::new(f.state_mut());

                    // The softline buffer replaces all softline breaks with a space or removes it entirely
                    // to "mimic" an infinite print width
                    let mut buffer = RemoveSoftLinesBuffer::new(&mut vec_buffer);

                    let mut recording = buffer.start_recording();

                    let options = TemplateElementOptions {
                        after_new_line: false,
                        indention: TemplateElementIndention::default(),
                    };

                    // print the current column with infinite print width
                    write!(recording, [element.format().with_options(options)])?;
                    let recorded = recording.stop();

                    // whether there was a line break when formatting the column
                    let will_break = recorded.will_break();

                    let root = Document::from(vec_buffer.into_vec());

                    let range = element.range();
                    let print_options = f.options().as_print_options();
                    let printed = Printer::new(print_options).print(&root)?;
                    let text = printed.into_code();

                    let column = EachTemplateColumn::new(text, range, will_break);

                    builder.entry(EachTemplateElement::Column(column));
                }
            }
        }

        let each_table = builder.finish();

        Ok(each_table)
    }
}

impl Format<JsFormatContext> for EachTemplateTable {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let table_content = format_with(|f| {
            let mut current_column: usize = 0;
            let mut current_row: usize = 0;

            let mut iter = self.elements.iter().peekable();

            write!(f, [hard_line_break()])?;

            while let Some(element) = iter.next() {
                let next_item = iter.peek();
                let is_last = next_item.is_none();
                let is_last_in_row =
                    matches!(next_item, Some(EachTemplateElement::LineBreak)) || is_last;

                match element {
                    EachTemplateElement::Column(column) => {
                        let mut text = column.text.to_string();

                        if current_column != 0 && (!is_last_in_row || !text.is_empty()) {
                            text = std::format!(" {text}");
                        }

                        // align the column based on the maximum column width in the table
                        if !is_last_in_row {
                            if !self.rows[current_row].has_line_break_column {
                                let column_width = self
                                    .columns_width
                                    .get(current_column)
                                    .copied()
                                    .unwrap_or_default();

                                let padding = " ".repeat(
                                    column_width
                                        .checked_sub(column.width)
                                        .unwrap_or_default()
                                        .into(),
                                );

                                text.push_str(&padding);
                            }

                            text.push(' ');
                        }

                        write!(f, [dynamic_text(&text, column.range.start())])?;

                        if !is_last_in_row {
                            write!(f, [EachTemplateSeparator])?;
                        }

                        current_column += 1;
                    }
                    EachTemplateElement::LineBreak => {
                        current_column = 0;
                        current_row += 1;

                        if !is_last {
                            write!(f, [hard_line_break()])?;
                        }
                    }
                }
            }
            Ok(())
        });

        write!(f, [indent(&format_args!(table_content)), hard_line_break()])
    }
}
