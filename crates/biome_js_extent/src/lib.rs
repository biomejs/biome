//! Finds where a region of JavaScript embedded in another language ends,
//! without parsing it.
//!
//! A host lexer, such as the HTML lexer for Astro, Svelte, and Vue files,
//! must emit an embedded expression or script as a single token before any
//! JavaScript parser sees it. The region ends at a host delimiter, such as the
//! `}` of a text expression, but the same delimiter can appear inside the
//! JavaScript as ordinary content: `{"}"}`. [`JsExtent`] measures the region by
//! skipping strings, comments, regular expression literals, template literals,
//! and JSX, so only a delimiter in plain code ends it.

use biome_unicode_table::{Dispatch::*, lookup_byte};
use smallvec::SmallVec;

/// How a scan reads `<` where an expression may start.
#[derive(Debug)]
pub enum Jsx {
    /// `<` is always a comparison, a shift, or a type argument list.
    Disabled,
    /// `<` followed by a name or `>` opens a JSX element or fragment.
    ///
    /// An element that is neither self-closing nor named by `is_void_element`
    /// has children up to its closing tag. `is_void_element` receives the tag
    /// name as written, so it decides whether names match case-sensitively.
    /// With the HTML void elements, `<br>` in `<p>a<br>b</p>` has no children.
    /// Pass `|_| false` when every element must be closed by `/>` or a
    /// closing tag, as JSX requires.
    Enabled { is_void_element: fn(&[u8]) -> bool },
}

/// Scans embedded JavaScript for the delimiter that ends the construct holding
/// it: the `}` of a text expression, the `---` fence of Astro frontmatter, or
/// the `]` of a Vue dynamic directive argument.
///
/// The closing delimiter must be plain code: a `}`, `---`, or `]` that sits
/// inside a string, a comment, a regex literal, a template literal, or JSX is
/// ordinary content and does not end the construct.
pub struct JsExtent<'src> {
    source: &'src [u8],
    /// The offset of the byte being classified.
    position: usize,
    /// The constructs the current position is nested in, innermost last.
    /// Opening a construct pushes a frame and its terminator pops it again.
    /// An empty stack means plain code, the only place the stop delimiter ends
    /// the scan. Eight inline frames cover typical nesting depths without a
    /// heap allocation.
    stack: SmallVec<[JsContext; 8]>,
    jsx: Jsx,
    /// Whether the scan ever entered JSX from code position.
    entered_jsx: bool,
}

const CODE_BYTES: u8 = 1 << 0;
const TEMPLATE_BYTES: u8 = 1 << 1;
const JSX_TAG_BYTES: u8 = 1 << 2;
const TYPE_ARGUMENT_BYTES: u8 = 1 << 3;
const JSX_CHILDREN_BYTES: u8 = 1 << 4;

/// For each byte, the [`JsContext`]s whose [`JsExtent::run`] arm does more
/// than advance past it.
static INTERESTING_BYTES: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut index = 0;
    while index < 256 {
        let byte = index as u8;
        let mut mask = 0u8;
        if matches!(
            byte,
            b'"' | b'\'' | b'`' | b'/' | b'{' | b'[' | b'}' | b']' | b'-' | b'<' | b'\n' | b'\r'
        ) {
            mask |= CODE_BYTES;
        }
        if matches!(byte, b'\\' | b'`' | b'$') {
            mask |= TEMPLATE_BYTES;
        }
        if matches!(byte, b'"' | b'\'' | b'`' | b'{' | b'=' | b'>' | b',' | b'<')
            || byte.is_ascii_whitespace()
        {
            mask |= JSX_TAG_BYTES;
        }
        if matches!(byte, b'"' | b'\'' | b'`' | b'<' | b'=' | b'>') {
            mask |= TYPE_ARGUMENT_BYTES;
        }
        if matches!(byte, b'{' | b'<') {
            mask |= JSX_CHILDREN_BYTES;
        }
        table[index] = mask;
        index += 1;
    }
    table
};

const fn interest_mask(context: Option<JsContext>) -> u8 {
    match context {
        None | Some(JsContext::Code(_)) => CODE_BYTES,
        Some(JsContext::Template) => TEMPLATE_BYTES,
        Some(JsContext::JsxTag { .. }) => JSX_TAG_BYTES,
        Some(JsContext::TypeArguments) => TYPE_ARGUMENT_BYTES,
        Some(JsContext::JsxChildren) => JSX_CHILDREN_BYTES,
    }
}

/// One level of JavaScript nesting tracked by [`JsExtent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsContext {
    /// Code nested inside a delimiter, closed by the given byte.
    Code(u8),
    /// The body of a template literal, between backticks.
    Template,
    /// A JSX tag, between `<` and its `>`.
    JsxTag {
        /// Whether the tag opened with `</`, which never has children.
        closing: bool,
        /// The offset of the tag name, just past the `<` or `</`.
        name_start: usize,
        /// Whether the scan is inside an unquoted attribute value, where a `/`
        /// is part of the value.
        in_unquoted_value: bool,
    },
    /// A type argument list on a JSX tag, between `<` and its `>`.
    TypeArguments,
    /// The children of a JSX element, where quotes and braces are text.
    JsxChildren,
}

impl JsContext {
    /// A tag whose name starts at `name_start`, before any attribute is seen.
    const fn new_jsx_tag(closing: bool, name_start: usize) -> Self {
        Self::JsxTag {
            closing,
            name_start,
            in_unquoted_value: false,
        }
    }
}

/// The delimiter a [`JsExtent`] run stops at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsScanStop {
    /// The `}` closing a text expression.
    ExpressionEnd,
    /// The `---` closing Astro frontmatter.
    FrontmatterFence,
    /// The `]` closing a Vue dynamic directive argument.
    ArgumentEnd,
    /// The blank line after an MDX ESM block.
    BlankLine,
}

impl JsScanStop {
    /// Returns whether `byte` on its own closes the construct for this stop.
    /// [`Self::FrontmatterFence`] never matches here: its delimiter is the
    /// three-byte `---`, which [`JsExtent::run`] matches separately.
    const fn ends_at(self, byte: u8) -> bool {
        matches!(
            (self, byte),
            (Self::ExpressionEnd, b'}') | (Self::ArgumentEnd, b']')
        )
    }
}

impl<'src> JsExtent<'src> {
    /// Returns the offset of the `}` that closes a text expression, or
    /// `source.len()` when the expression is unterminated. `source` starts
    /// just past the opening `{`.
    pub fn expression_length(source: &'src [u8], jsx: Jsx) -> usize {
        Self::scan(source, JsScanStop::ExpressionEnd, jsx)
    }

    /// Returns the offset of the `---` that closes Astro frontmatter, or
    /// `source.len()` when the fence is missing. Astro's component script is
    /// TypeScript, so `<` there opens a type assertion rather than JSX.
    pub fn frontmatter_length(source: &'src [u8]) -> usize {
        Self::scan(source, JsScanStop::FrontmatterFence, Jsx::Disabled)
    }

    /// Returns the offset of the `]` that closes a Vue dynamic directive
    /// argument, or `source.len()` when the bracket is unbalanced.
    pub fn argument_length(source: &'src [u8]) -> usize {
        Self::scan(source, JsScanStop::ArgumentEnd, Jsx::Disabled)
    }

    /// Returns the offset where the content of an MDX ESM block ends: the
    /// line ending before the first blank line, or `source.len()` when no
    /// blank line follows. A blank line inside braces, a string, a comment,
    /// a template literal, or JSX doesn't end the block, so
    /// `export {\n\n  a\n\n} from "b"` is a single block.
    pub fn esm_length(source: &'src [u8]) -> usize {
        let jsx = Jsx::Enabled {
            is_void_element: |_| false,
        };
        Self::scan(source, JsScanStop::BlankLine, jsx)
    }

    fn scan(source: &'src [u8], stop: JsScanStop, jsx: Jsx) -> usize {
        let mut extent = Self {
            source,
            position: 0,
            stack: SmallVec::new(),
            jsx,
            entered_jsx: false,
        };
        let length = extent.run(stop);
        if length < source.len() || !extent.entered_jsx {
            return length;
        }

        // Running off the end inside JSX means a `<` was really a comparison
        // or a cast, misread as an element whose closing tag never comes;
        // rescan reading `<` as an operator.
        extent.position = 0;
        extent.stack.clear();
        extent.jsx = Jsx::Disabled;
        extent.run(stop)
    }

    fn current_byte(&self) -> Option<u8> {
        self.source.get(self.position).copied()
    }

    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source.get(self.position + offset).copied()
    }

    /// Moves `count` bytes forward, stopping at the source end so an escape at
    /// the very end cannot step past it.
    fn advance(&mut self, count: usize) {
        self.position = (self.position + count).min(self.source.len());
    }

    fn scanned(&self) -> &[u8] {
        &self.source[..self.position]
    }

    fn previous_non_whitespace(&self) -> Option<u8> {
        self.scanned()
            .iter()
            .rev()
            .copied()
            .find(|byte| !byte.is_ascii_whitespace())
    }

    fn set_in_unquoted_value(&mut self, value: bool) {
        if let Some(JsContext::JsxTag {
            in_unquoted_value, ..
        }) = self.stack.last_mut()
        {
            *in_unquoted_value = value;
        }
    }

    /// Advances past every byte the current context handles with a plain
    /// `advance(1)`, so the per-byte cost stays one table load and one add.
    fn skip_uninteresting(&mut self, mask: u8) {
        while let Some(&byte) = self.source.get(self.position) {
            if INTERESTING_BYTES[byte as usize] & mask != 0 {
                break;
            }
            self.position += 1;
        }
    }

    fn run(&mut self, stop: JsScanStop) -> usize {
        while let Some(byte) = self.current_byte() {
            let context = self.stack.last().copied();
            let mask = interest_mask(context);
            if INTERESTING_BYTES[byte as usize] & mask == 0 {
                self.skip_uninteresting(mask);
                continue;
            }
            match context {
                None | Some(JsContext::Code(_)) => match lookup_byte(byte) {
                    QOT => self.skip_string(byte),
                    TPL => {
                        self.stack.push(JsContext::Template);
                        self.advance(1);
                    }
                    SLH => self.skip_slash(),
                    BEO => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(1);
                    }
                    BTO if stop == JsScanStop::ArgumentEnd => {
                        self.stack.push(JsContext::Code(b']'));
                        self.advance(1);
                    }
                    BEC | BTC => {
                        if self.stack.last() == Some(&JsContext::Code(byte)) {
                            self.stack.pop();
                        } else if self.stack.is_empty() && stop.ends_at(byte) {
                            return self.position;
                        }
                        self.advance(1);
                    }
                    WHS if stop == JsScanStop::BlankLine
                        && self.stack.is_empty()
                        && matches!(byte, b'\n' | b'\r') =>
                    {
                        if let Some(end) = self.blank_line_end() {
                            return end;
                        }
                        self.advance(1);
                    }
                    MIN if stop == JsScanStop::FrontmatterFence
                        && self.stack.is_empty()
                        && self.source[self.position..].starts_with(b"---") =>
                    {
                        return self.position;
                    }
                    LSS if matches!(self.jsx, Jsx::Enabled { .. }) && self.jsx_element_starts() => {
                        self.entered_jsx = true;
                        self.stack
                            .push(JsContext::new_jsx_tag(false, self.position + 1));
                        self.advance(1);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::Template) => match lookup_byte(byte) {
                    BSL => self.advance(2),
                    TPL => {
                        self.stack.pop();
                        self.advance(1);
                    }
                    DOL if self.byte_at(1) == Some(b'{') => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(2);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::JsxTag {
                    closing,
                    name_start,
                    in_unquoted_value,
                }) => match lookup_byte(byte) {
                    QOT => self.skip_string(byte),
                    TPL => {
                        self.stack.push(JsContext::Template);
                        self.advance(1);
                    }
                    BEO => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(1);
                    }
                    EQL => {
                        self.set_in_unquoted_value(starts_unquoted_value(self.byte_at(1)));
                        self.advance(1);
                    }
                    MOR => {
                        self.stack.pop();
                        let self_closing =
                            !in_unquoted_value && self.previous_non_whitespace() == Some(b'/');
                        if !closing && !self_closing && !self.is_void_tag(name_start) {
                            self.stack.push(JsContext::JsxChildren);
                        }
                        self.advance(1);
                    }
                    // A `,` cannot appear in a JSX tag, so this was a type parameter list.
                    COM => {
                        self.stack.pop();
                        self.advance(1);
                    }
                    LSS => {
                        self.stack.push(JsContext::TypeArguments);
                        self.advance(1);
                    }
                    _ if in_unquoted_value && byte.is_ascii_whitespace() => {
                        self.set_in_unquoted_value(false);
                        self.advance(1);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::TypeArguments) => match lookup_byte(byte) {
                    QOT => self.skip_string(byte),
                    TPL => {
                        self.stack.push(JsContext::Template);
                        self.advance(1);
                    }
                    LSS => {
                        self.stack.push(JsContext::TypeArguments);
                        self.advance(1);
                    }
                    // The `>` of a function type's arrow does not close the list.
                    EQL if self.byte_at(1) == Some(b'>') => self.advance(2),
                    MOR => {
                        self.stack.pop();
                        self.advance(1);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::JsxChildren) => match lookup_byte(byte) {
                    BEO => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(1);
                    }
                    LSS => match self.byte_at(1) {
                        Some(b'/') => {
                            self.stack.pop();
                            self.stack
                                .push(JsContext::new_jsx_tag(true, self.position + 2));
                            self.advance(2);
                        }
                        Some(next) if starts_tag_name(next) => {
                            self.stack
                                .push(JsContext::new_jsx_tag(false, self.position + 1));
                            self.advance(1);
                        }
                        _ => self.advance(1),
                    },
                    _ => self.advance(1),
                },
            }
        }

        self.position
    }

    /// Skips the string literal whose opening `quote` is the current byte,
    /// stopping just past the closing quote or at the end of the source.
    fn skip_string(&mut self, quote: u8) {
        self.advance(1);
        while let Some(byte) = self.current_byte() {
            self.advance(1);
            if byte == b'\\' {
                self.advance(1);
            } else if byte == quote {
                break;
            }
        }
    }

    /// Skips from a `/` in code position past the comment or regex literal it
    /// opens, or past the slash alone.
    fn skip_slash(&mut self) {
        match self.byte_at(1) {
            Some(b'/') => {
                self.advance(2);
                self.skip_line_comment();
            }
            Some(b'*') => {
                self.advance(2);
                self.skip_block_comment();
            }
            // `/>` closes a tag that was not recognized as JSX, unless the slash
            // sits where only an operand can start: `s.replace(/>/g, "")`.
            Some(b'>') if !self.at_operand_start() => self.advance(1),
            _ if self.scanned().last() != Some(&b'<') && slash_starts_regex(self.scanned()) => {
                self.advance(1);
                self.skip_regex();
            }
            _ => self.advance(1),
        }
    }

    /// Returns whether the previous token leaves the scan where an operand
    /// must start (after `(`, `,`, `=`, `=>`, `return`, ...), so a `/` there
    /// can only open a regex literal, never close a tag.
    fn at_operand_start(&self) -> bool {
        let scanned = self.scanned();
        let Some(end) = scanned.iter().rposition(|byte| !byte.is_ascii_whitespace()) else {
            return true;
        };
        match lookup_byte(scanned[end]) {
            MOR => end > 0 && scanned[end - 1] == b'=',
            PNO | COM | EQL | COL | BTO | EXL | AMP | PIP | QST | SEM | BEO | TLD | CRT | PRC
            | MUL => true,
            IDT => ends_with_expression_keyword(&scanned[..=end]),
            _ => false,
        }
    }

    /// Skips the regex literal body, stopping just past the closing `/` or at
    /// the end of the source. A `/` inside a `[…]` character class does not
    /// close the literal.
    fn skip_regex(&mut self) {
        let mut in_character_class = false;

        while let Some(byte) = self.current_byte() {
            self.advance(1);
            match byte {
                b'\\' => self.advance(1),
                b'[' => in_character_class = true,
                b']' => in_character_class = false,
                b'/' if !in_character_class => break,
                _ => {}
            }
        }
    }

    /// Skips just past the terminating newline, or to the end of the source.
    fn skip_line_comment(&mut self) {
        while let Some(byte) = self.current_byte() {
            self.advance(1);
            if byte == b'\n' {
                break;
            }
        }
    }

    /// Skips just past the closing `*/`, or to the end of the source.
    fn skip_block_comment(&mut self) {
        while let Some(byte) = self.current_byte() {
            if byte == b'*' && self.byte_at(1) == Some(b'/') {
                self.advance(2);
                return;
            }
            self.advance(1);
        }
    }

    /// Returns where the content before a blank line ends, when the line
    /// ending at the current position closes a line that holds only spaces
    /// and tabs. The returned offset is the start of the line ending before
    /// that blank line.
    fn blank_line_end(&self) -> Option<usize> {
        let scanned = self.scanned();
        // The `\n` of a `\r\n` pair was already handled at its `\r`.
        if self.current_byte() == Some(b'\n') && scanned.last() == Some(&b'\r') {
            return None;
        }
        let line_start = scanned
            .iter()
            .rposition(|byte| matches!(byte, b'\n' | b'\r'))?
            + 1;
        if !scanned[line_start..]
            .iter()
            .all(|byte| matches!(byte, b' ' | b'\t'))
        {
            return None;
        }
        let previous_line_end = line_start - 1;
        let crlf = scanned[previous_line_end] == b'\n'
            && previous_line_end > 0
            && scanned[previous_line_end - 1] == b'\r';
        Some(if crlf {
            previous_line_end - 1
        } else {
            previous_line_end
        })
    }

    /// Returns whether the `<` at the current position opens a JSX element
    /// rather than a comparison, a shift, or a TypeScript type argument list.
    fn jsx_element_starts(&self) -> bool {
        self.byte_at(1).is_some_and(starts_tag_name) && at_expression_position(self.scanned())
    }

    /// Returns whether the element whose name starts at `name_start` has no
    /// children, according to the caller's [`Jsx::Enabled`] check.
    fn is_void_tag(&self, name_start: usize) -> bool {
        match self.jsx {
            Jsx::Enabled { is_void_element } => is_void_element(self.tag_name(name_start)),
            Jsx::Disabled => false,
        }
    }

    /// Returns the tag name starting at `start`, which is empty for a fragment.
    fn tag_name(&self, start: usize) -> &[u8] {
        let name = self.source.get(start..).unwrap_or_default();
        let length = name
            .iter()
            .position(|byte| !is_tag_name_byte(*byte))
            .unwrap_or(name.len());

        &name[..length]
    }
}

/// Returns whether a byte can follow `<` in a JSX element, counting the `>` of
/// a fragment.
fn starts_tag_name(byte: u8) -> bool {
    byte == b'>' || byte == b'_' || byte.is_ascii_alphabetic()
}

/// Returns whether `byte` continues the tag name passed to the void-element
/// check: an ASCII letter or digit, `-`, `_`, or `:`.
fn is_tag_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':')
}

/// Returns whether `byte`, the one right after an `=` in a tag, opens an
/// unquoted attribute value.
fn starts_unquoted_value(byte: Option<u8>) -> bool {
    byte.is_some_and(|byte| {
        !matches!(byte, b'"' | b'\'' | b'`' | b'{') && !byte.is_ascii_whitespace()
    })
}

/// Returns whether a `/` after the code scanned so far opens a regex literal
/// rather than a division.
fn slash_starts_regex(scanned: &[u8]) -> bool {
    let Some(end) = scanned.iter().rposition(|byte| !byte.is_ascii_whitespace()) else {
        return true;
    };
    match lookup_byte(scanned[end]) {
        // A keyword such as `return` leaves an operand position; other words do not.
        IDT => ends_with_expression_keyword(&scanned[..=end]),
        DOL | DIG | ZER | PNC | BTC | PLS | MIN => false,
        _ => true,
    }
}

/// Returns whether the code scanned so far ends where an expression may start.
/// After an operand (a non-keyword identifier, a literal, or a closing bracket),
/// the next `<` is an operator instead.
fn at_expression_position(scanned: &[u8]) -> bool {
    let Some(index) = scanned.iter().rposition(|byte| !byte.is_ascii_whitespace()) else {
        return true;
    };

    if is_js_word_byte(scanned[index]) {
        return ends_with_expression_keyword(&scanned[..=index]);
    }

    !matches!(scanned[index], b')' | b']' | b'"' | b'\'' | b'`')
}

/// Returns whether `code` ends with a reserved word that an expression can
/// directly follow, such as `return`. Contextual keywords such as `of` are left
/// out because they are also plain names (`of / 2`), and a word after `.` or
/// `#` is a property name, like the operand in `pool.yield / 100`.
fn ends_with_expression_keyword(code: &[u8]) -> bool {
    let start = code
        .iter()
        .rposition(|byte| !is_js_word_byte(*byte))
        .map_or(0, |index| index + 1);
    let before = code[..start]
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace());
    if before.is_some_and(|index| matches!(lookup_byte(code[index]), PRD | HAS)) {
        return false;
    }

    matches!(
        &code[start..],
        b"await"
            | b"case"
            | b"delete"
            | b"do"
            | b"else"
            | b"in"
            | b"instanceof"
            | b"new"
            | b"return"
            | b"throw"
            | b"typeof"
            | b"void"
            | b"yield"
    )
}

fn is_js_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

#[cfg(test)]
mod tests;
