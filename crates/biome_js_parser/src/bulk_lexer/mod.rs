//! A lexer that tokenizes a whole source text ahead of parsing, into flat arrays of token starts
//! and kinds.
//!
//! [JsLexer] lexes one token at a time in the context the parser asks for: inside a template
//! literal, among JSX children, after `=` in a JSX tag. It never decides between a regex and a
//! `/`, which the parser asks it to re-lex. This lexer decides all of that up front, from the
//! source alone:
//!
//! 1. [classify] computes bitmaps of character classes with SIMD, and from them the token
//!    starts, assuming every token is an identifier, a number, consecutive whitespace, a line
//!    break or a single punctuator.
//! 2. `carve` walks the bytes that open strings, comments, template literals, regexes and JSX,
//!    and records those tokens. It tracks the nesting of template substitutions and JSX elements,
//!    and decides from the tokens before a `/` or `<` whether it starts a regex or a JSX element.
//! 3. `coalesce` merges the bytes of numbers and of operators longer than one byte.
//! 4. `compress` writes the start and the kind of every token, and then finds the keywords.
//!
//! Each token records the lexing contexts in which [JsLexer] would return it as is, so that
//! [BulkTokenSource] can check its prediction against the context the parser asks for. A regex
//! the parser reads as a division, a JSX element the parser reads as type parameters, and the
//! contexts this lexer doesn't predict, such as Astro templates, leave the parser asking for a
//! token no bulk token matches. The token source lexes those with [JsLexer] until it reaches a
//! bulk token that fits again.
//!
//! Tokens that need escapes, Unicode identifier tables or diagnostics are lexed with [JsLexer]
//! too, one at a time, so that they and their diagnostics match it exactly.

mod carve;
mod classify;
mod scan;
mod token_source;

#[cfg(test)]
mod tests;

#[cfg(feature = "bulk_lexer")]
pub(crate) use token_source::{BulkTokenSource, BulkTokenSourceCheckpoint};

use crate::lexer::{JsLexContext, JsLexer, TextSize};
use biome_js_syntax::JsSyntaxKind::{self, *};
use biome_languages::JsFileSource;
use biome_languages::javascript::LanguageVariant;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::Lexer;
use classify::{Block, classify};
use scan::{BYTE_TOKENS, KEYWORD_KINDS, keyword, operator, scan_number};

/// Mask of the [JsSyntaxKind] in a token's kind and flags.
pub(crate) const KIND_MASK: u16 = 0x03FF;
/// The token is an identifier or keyword written with a unicode escape.
pub(crate) const UNICODE_ESCAPE: u16 = 1 << 10;
/// [BulkTokens::diagnostics] has diagnostics for the token.
pub(crate) const DIAGNOSTICS: u16 = 1 << 11;
/// [JsLexer] returns the token in [JsLexContext::Regular], except that a regex here is a `/` or
/// `/=` there, which the parser re-lexes into the regex.
pub(crate) const REGULAR: u16 = 1 << 12;
/// [JsLexer] returns the token in [JsLexContext::TemplateElement].
pub(crate) const TEMPLATE: u16 = 1 << 13;
/// [JsLexer] returns the token in [JsLexContext::JsxChild] outside of Astro templates.
pub(crate) const JSX_CHILD: u16 = 1 << 14;
/// [JsLexer] returns the token in [JsLexContext::JsxAttributeValue] outside of Astro templates.
pub(crate) const JSX_ATTRIBUTE_VALUE: u16 = 1 << 15;

const _: () = assert!(JsSyntaxKind::__LAST as u16 <= KIND_MASK);

/// What the source allows, which decides how [BulkTokens] reads a `<`.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BulkLexOptions {
    /// The source may contain JSX, outside of Astro templates.
    pub(crate) jsx: bool,
    /// The source is TypeScript.
    pub(crate) typescript: bool,
}

impl BulkLexOptions {
    pub(crate) fn new(source_type: &JsFileSource) -> Self {
        let embedding = source_type.as_embedding_kind();
        Self {
            jsx: source_type.variant() == LanguageVariant::Jsx
                && !embedding.is_astro_frontmatter()
                && !embedding.is_astro_template(),
            typescript: source_type.language().is_typescript(),
        }
    }
}

/// A diagnostic of a token in [BulkTokens].
#[derive(Debug)]
pub(crate) struct TokenDiagnostic {
    /// The start of the token.
    pub(crate) start: u32,
    pub(crate) diagnostic: ParseDiagnostic,
    /// Whether [JsLexer] reports it for a template element of a tagged template too, which
    /// drops the diagnostics of escape sequences.
    pub(crate) in_tagged_template: bool,
}

/// The tokens of a source text, lexed ahead of parsing.
pub(crate) struct BulkTokens {
    /// The start of each token, followed by the source length.
    starts: Vec<u32>,
    /// The kind and flags of each token. The last two tokens are `EOF`s.
    kinds: Vec<u16>,
    /// The diagnostics of the tokens with [DIAGNOSTICS], ordered by their start.
    diagnostics: Vec<TokenDiagnostic>,
}

impl BulkTokens {
    /// Lexes `lexer`'s source. Leaves the position of `lexer` unspecified, and its diagnostics
    /// as they were.
    pub(crate) fn new(lexer: &mut JsLexer, options: BulkLexOptions) -> Self {
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2")
            && std::arch::is_x86_feature_detected!("bmi1")
            && std::arch::is_x86_feature_detected!("bmi2")
            && std::arch::is_x86_feature_detected!("lzcnt")
            && std::arch::is_x86_feature_detected!("popcnt")
        {
            // SAFETY: the CPU supports the features.
            return unsafe { Self::new_with_bit_instructions(lexer, options) };
        }
        Self::new_portable(lexer, options)
    }

    /// [Self::new], compiled with the bit manipulation instructions of x86_64 CPUs since 2013,
    /// which the passes over the bitmaps use for every token.
    ///
    /// # Safety
    /// The CPU must support AVX2, BMI1, BMI2, LZCNT and POPCNT.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2,bmi1,bmi2,lzcnt,popcnt")]
    unsafe fn new_with_bit_instructions(lexer: &mut JsLexer, options: BulkLexOptions) -> Self {
        Self::new_portable(lexer, options)
    }

    #[inline(always)]
    fn new_portable(lexer: &mut JsLexer, options: BulkLexOptions) -> Self {
        let source = lexer.source().as_bytes();
        let blocks = classify(source);
        let mut pass = Pass::new(source, blocks, lexer, options);
        pass.carve();
        pass.coalesce();
        pass.compress()
    }

    /// Tokens that hold only the end of the file, so that every token comes from [JsLexer].
    pub(crate) fn new_empty(len: TextSize) -> Self {
        let len = u32::from(len);
        Self {
            starts: vec![len, len, len],
            kinds: vec![EOF as u16 | REGULAR, EOF as u16 | REGULAR],
            diagnostics: Vec::new(),
        }
    }

    /// Returns the kind and flags of token `index`.
    #[inline(always)]
    pub(crate) fn get(&self, index: u32) -> Option<u16> {
        self.kinds.get(index as usize).copied()
    }

    /// Returns the start of token `index`, or the source length after the last token.
    #[inline(always)]
    pub(crate) fn start(&self, index: u32) -> u32 {
        self.starts[index as usize]
    }

    /// Returns the index of the token starting at `offset`, searching forward from token `hint`
    /// first, as the token source moves forward through the source.
    pub(crate) fn find_from(&self, offset: TextSize, hint: u32) -> Option<u32> {
        let offset = u32::from(offset);
        let starts = &self.starts[..self.kinds.len()];
        let mut low = 0;
        let mut high = starts.len();
        let hint = hint as usize;
        if starts.get(hint).is_some_and(|&start| start <= offset) {
            // Gallop forward from the hint to bound the binary search.
            low = hint;
            let mut step = 1;
            while low + step < high && starts[low + step] <= offset {
                low += step;
                step *= 2;
            }
            high = high.min(low + step + 1);
        }
        let index = low + starts[low..high].partition_point(|&start| start < offset);
        (starts.get(index) == Some(&offset)).then_some(index as u32)
    }

    /// Returns the diagnostics of the token starting at `start`.
    pub(crate) fn diagnostics_at(&self, start: TextSize) -> &[TokenDiagnostic] {
        let start = u32::from(start);
        let first = self
            .diagnostics
            .partition_point(|diagnostic| diagnostic.start < start);
        let len = self.diagnostics[first..]
            .iter()
            .take_while(|diagnostic| diagnostic.start == start)
            .count();
        &self.diagnostics[first..first + len]
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (u32, u32, u16)> + '_ {
        self.kinds
            .iter()
            .enumerate()
            .map(|(index, &kind)| (self.starts[index], self.starts[index + 1], kind))
    }
}

/// Returns the [JsSyntaxKind] of a token's kind and flags.
#[inline(always)]
pub(crate) fn kind(raw: u16) -> JsSyntaxKind {
    let kind = raw & KIND_MASK;
    assert!(kind <= JsSyntaxKind::__LAST as u16);
    // SAFETY: `kind` is a value of `JsSyntaxKind`, a `repr(u16)` enum, as checked above.
    // `JsSyntaxKind::from` does the same, but doesn't inline across crates.
    unsafe { std::mem::transmute::<u16, JsSyntaxKind>(kind) }
}

/// Returns the flag of the tokens that [JsLexer] returns as is in `context`, if [BulkTokens]
/// predicts the tokens of that context.
#[inline(always)]
pub(crate) fn context_flag(context: JsLexContext) -> Option<u16> {
    match context {
        JsLexContext::Regular => Some(REGULAR),
        JsLexContext::TemplateElement { .. } => Some(TEMPLATE),
        JsLexContext::JsxChild { astro: false } => Some(JSX_CHILD),
        JsLexContext::JsxAttributeValue { astro: false } => Some(JSX_ATTRIBUTE_VALUE),
        JsLexContext::JsxChild { astro: true }
        | JsLexContext::JsxAttributeValue { astro: true }
        | JsLexContext::JsxRawText(_) => None,
    }
}

/// The state of the passes over one source text.
struct Pass<'a, 'src> {
    source: &'a [u8],
    blocks: Vec<Block>,
    options: BulkLexOptions,
    /// The tokens found by `carve`, in source order.
    carved: Vec<Override>,
    /// The end of the last token found by `carve`.
    carved_end: usize,
    /// The tokens found by `coalesce`, in source order.
    coalesced: Vec<Override>,
    diagnostics: Vec<TokenDiagnostic>,
    /// The offsets of the `>` that end the JSX elements in JavaScript, in source order.
    jsx_ends: Vec<u32>,
    lexer: &'a mut JsLexer<'src>,
    bom_length: usize,
    /// Built when the first block comment needs it, as it costs more than lexing a short source.
    block_comment_end: Option<memchr::memmem::Finder<'static>>,
}

/// The kind and flags of a token that a pass after [classify] found, which
/// [Block::overridden] marks.
#[derive(Clone, Copy, Debug)]
struct Override {
    start: u32,
    end: u32,
    kind: u16,
}

impl<'a, 'src> Pass<'a, 'src> {
    fn new(
        source: &'a [u8],
        blocks: Vec<Block>,
        lexer: &'a mut JsLexer<'src>,
        options: BulkLexOptions,
    ) -> Self {
        Self {
            source,
            blocks,
            options,
            carved: Vec::new(),
            carved_end: 0,
            coalesced: Vec::new(),
            diagnostics: Vec::new(),
            jsx_ends: Vec::new(),
            lexer,
            bom_length: 0,
            block_comment_end: None,
        }
    }

    /// Returns the byte at `offset`, or zero past the end of the source.
    #[inline(always)]
    fn byte(&self, offset: usize) -> u8 {
        self.source.get(offset).copied().unwrap_or(0)
    }

    #[inline(always)]
    fn set_start(&mut self, offset: usize) {
        if offset < self.source.len() {
            self.blocks[offset / 64].starts |= 1 << (offset % 64);
        }
    }

    /// Removes the starts in `from..to`.
    fn clear_starts(&mut self, from: usize, to: usize) {
        if from >= to {
            return;
        }
        let (first, last) = (from / 64, (to - 1) / 64);
        let low = u64::MAX << (from % 64);
        let high = u64::MAX >> (63 - (to - 1) % 64);
        if first == last {
            self.blocks[first].starts &= !(low & high);
        } else {
            self.blocks[first].starts &= !low;
            for block in &mut self.blocks[first + 1..last] {
                block.starts = 0;
            }
            self.blocks[last].starts &= !high;
        }
    }

    /// Records a token of `kind` spanning `start..end`, after all the tokens `carve` recorded so
    /// far.
    fn set_token(&mut self, start: usize, end: usize, kind: u16) {
        debug_assert!(self.carved_end <= start);
        self.carved.push(Override {
            start: start as u32,
            end: end as u32,
            kind,
        });
        self.carved_end = end;
        self.set_start(start);
        self.set_bounds(start, end);
    }

    /// Marks `start` as overridden and the bytes in `start + 1..end` as inside its token.
    fn set_bounds(&mut self, start: usize, end: usize) {
        self.blocks[start / 64].overridden |= 1 << (start % 64);
        self.clear_starts(start + 1, end);
        self.set_start(end);
    }

    #[inline(always)]
    fn is_start(&self, offset: usize) -> bool {
        self.blocks[offset / 64].starts & (1 << (offset % 64)) != 0
    }

    /// Returns the first offset at or after `from` with a bit in `field`, or the source length.
    #[inline(always)]
    fn next_bit(&self, from: usize, field: impl Fn(&Block) -> u64) -> usize {
        self.next_bit_before(from, self.source.len(), field)
    }

    /// Returns the first offset in `from..to` with a bit in `field`, or `to`.
    #[inline(always)]
    fn next_bit_before(&self, from: usize, to: usize, field: impl Fn(&Block) -> u64) -> usize {
        if from >= to {
            return to;
        }
        let mut index = from / 64;
        let mut bits = field(&self.blocks[index]) & (u64::MAX << (from % 64));
        while bits == 0 {
            index += 1;
            if index * 64 >= to {
                return to;
            }
            bits = field(&self.blocks[index]);
        }
        (index * 64 + bits.trailing_zeros() as usize).min(to)
    }

    /// Returns the last token start before `offset`.
    fn previous_start(&self, offset: usize) -> Option<usize> {
        if offset == 0 {
            return None;
        }
        let mut index = (offset - 1) / 64;
        let mut bits = self.blocks[index].starts & (u64::MAX >> (63 - (offset - 1) % 64));
        loop {
            if bits != 0 {
                return Some(index * 64 + 63 - bits.leading_zeros() as usize);
            }
            if index == 0 {
                return None;
            }
            index -= 1;
            bits = self.blocks[index].starts;
        }
    }

    /// Returns the end of the token starting at `offset`, which must be one whose end the
    /// starts already give.
    #[inline(always)]
    fn token_end(&self, offset: usize) -> usize {
        self.next_bit(offset + 1, |block| block.starts)
    }

    /// Returns the kind and flags of the token that `carve` found at `start`.
    fn carved_kind(&self, start: usize) -> Option<u16> {
        let index = self
            .carved
            .partition_point(|token| (token.start as usize) < start);
        self.carved
            .get(index)
            .filter(|token| token.start as usize == start)
            .map(|token| token.kind)
    }

    /// Lexes the token at `offset` with [JsLexer] and records it. Returns its end.
    fn delegate(&mut self, offset: usize, context: JsLexContext) -> usize {
        let (raw, end) = self.lex_one(offset, context);
        if kind(raw) == UNICODE_BOM {
            self.bom_length = end;
        }
        self.set_token(offset, end, raw);
        end
    }

    /// Lexes the token at `offset` with [JsLexer], and records its diagnostics. Returns its kind
    /// and flags, and its end.
    fn lex_one(&mut self, offset: usize, context: JsLexContext) -> (u16, usize) {
        let diagnostics = self.lexer.diagnostics_len();
        let kind = self.lexer.lex_at(offset, false, context);
        let end = self.lexer.position();
        let mut raw = kind as u16 | context_flag(context).unwrap_or(0);
        if self.lexer.has_unicode_escape() {
            raw |= UNICODE_ESCAPE;
        }
        let reported = self.lexer.take_diagnostics_after(diagnostics);
        if !reported.is_empty() {
            raw |= DIAGNOSTICS;
            // A tagged template drops the diagnostics of escape sequences, which come before
            // the one of an unterminated template.
            let in_tagged_template = if let JsLexContext::TemplateElement { .. } = context {
                self.lexer.lex_at(
                    offset,
                    false,
                    JsLexContext::TemplateElement { tagged: true },
                );
                self.lexer.take_diagnostics_after(diagnostics).len()
            } else {
                reported.len()
            };
            let first_in_tagged_template = reported.len() - in_tagged_template;
            self.diagnostics
                .extend(reported.into_iter().enumerate().map(|(index, diagnostic)| {
                    TokenDiagnostic {
                        start: offset as u32,
                        diagnostic,
                        in_tagged_template: index >= first_in_tagged_template,
                    }
                }));
        }
        (raw, end)
    }

    /// Merges the bytes of numbers and of operators longer than one byte.
    #[inline(always)]
    fn coalesce(&mut self) {
        for index in 0..self.blocks.len() - 1 {
            let candidates = self.blocks[index].merge & !self.blocks[index].overridden;
            let mut done = 0;
            loop {
                let bits = self.blocks[index].starts & candidates & !done;
                if bits == 0 {
                    break;
                }
                let bit = bits.trailing_zeros() as usize;
                done |= (2u64 << bit).wrapping_sub(1);
                let start = index * 64 + bit;
                let byte = self.source[start];
                let next = self.byte(start + 1);
                if byte.is_ascii_digit() || (byte == b'.' && next.is_ascii_digit()) {
                    match scan_number(self.source, start) {
                        Some(end) => {
                            self.set_coalesced(start, end, JS_NUMBER_LITERAL as u16 | REGULAR);
                        }
                        None => self.delegate_number(start),
                    }
                } else if let Some((kind, len)) = operator(byte, next, self.byte(start + 2)) {
                    self.set_coalesced(start, start + len, kind as u16 | REGULAR);
                }
            }
        }
    }

    /// Records a token spanning `start..end`, after all the tokens `coalesce` recorded so far.
    #[inline(always)]
    fn set_coalesced(&mut self, start: usize, end: usize, kind: u16) {
        self.coalesced.push(Override {
            start: start as u32,
            end: end as u32,
            kind,
        });
        self.set_bounds(start, end);
    }

    /// Lexes the number at `start` with [JsLexer], which may continue it with an identifier
    /// that `carve` lexed already. While a token ends inside a token of `carve`, whose starts
    /// are gone, lexes the token after it too.
    #[cold]
    fn delegate_number(&mut self, start: usize) {
        let mut start = start;
        loop {
            let (kind, end) = self.lex_one(start, JsLexContext::Regular);
            self.set_coalesced(start, end, kind);
            let after = self
                .carved
                .partition_point(|token| (token.start as usize) < end);
            let inside_carved = after > 0 && self.carved[after - 1].end as usize > end;
            if !inside_carved {
                return;
            }
            start = end;
        }
    }

    /// Writes the start and kind of every token.
    #[inline(always)]
    fn compress(mut self) -> BulkTokens {
        let overrides = self.merge_overrides();
        let count: usize = self
            .blocks
            .iter()
            .map(|block| block.starts.count_ones() as usize)
            .sum();
        let mut starts: Vec<u32> = Vec::with_capacity(count + 3);
        let mut kinds: Vec<u16> = Vec::with_capacity(count + 2);
        let starts_out = starts.as_mut_ptr();
        let kinds_out = kinds.as_mut_ptr();
        let source = self.source;
        let mut written = 0;
        let mut next_override = 0;
        for (index, block) in self.blocks.iter().enumerate() {
            let mut bits = block.starts;
            while bits != 0 {
                let bit = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let start = index * 64 + bit;
                // Pick the override or the kind of the first byte without a branch, as either is
                // common.
                let overridden = ((block.overridden >> bit) & 1) as usize;
                // SAFETY: starts are offsets in the source.
                let by_byte = BYTE_TOKENS[unsafe { *source.get_unchecked(start) } as usize];
                // SAFETY: every overridden start has an override, and one more follows them.
                let by_override = unsafe { *overrides.get_unchecked(next_override) };
                let kind = if overridden != 0 {
                    by_override
                } else {
                    by_byte
                };
                next_override += overridden;
                // SAFETY: there are `count` starts, which both vectors have room for.
                unsafe {
                    starts_out.add(written).write(start as u32);
                    kinds_out.add(written).write(kind);
                }
                written += 1;
            }
        }
        assert_eq!(written, count);
        // SAFETY: the loop initialized `count` elements.
        unsafe {
            starts.set_len(count);
            kinds.set_len(count);
        }
        self.find_keywords(&mut kinds);

        let len = self.source.len() as u32;
        starts.extend([len, len, len]);
        kinds.extend([EOF as u16 | REGULAR, EOF as u16 | REGULAR]);
        // `carve` and `coalesce` each report in source order.
        self.diagnostics.sort_by_key(|diagnostic| diagnostic.start);
        BulkTokens {
            starts,
            kinds,
            diagnostics: self.diagnostics,
        }
    }

    /// Returns the kinds of the carved and coalesced tokens that are still tokens, in source
    /// order, followed by one extra kind that `compress` may read but never uses.
    fn merge_overrides(&mut self) -> Vec<u16> {
        let mut merged = Vec::with_capacity(self.carved.len() + self.coalesced.len() + 1);
        let (mut carved, mut coalesced) = (0, 0);
        while carved < self.carved.len() || coalesced < self.coalesced.len() {
            let take_carved = match (self.carved.get(carved), self.coalesced.get(coalesced)) {
                (Some(left), Some(right)) => left.start < right.start,
                (left, _) => left.is_some(),
            };
            let token = if take_carved {
                carved += 1;
                self.carved[carved - 1]
            } else {
                coalesced += 1;
                self.coalesced[coalesced - 1]
            };
            if self.is_start(token.start as usize) {
                merged.push(token.kind);
            }
        }
        merged.push(0);
        merged
    }

    /// Replaces the kind of the identifiers in `kinds` that are keywords.
    #[inline(always)]
    fn find_keywords(&self, kinds: &mut [u16]) {
        // The index in `kinds` of the first token of the block.
        let mut first = 0;
        for (index, block) in self.blocks.iter().enumerate() {
            let mut bits = block.starts & block.lower_pair & !block.overridden;
            while bits != 0 {
                let bit = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let start = index * 64 + bit;
                let token = first + (block.starts & ((1 << bit) - 1)).count_ones() as usize;
                let rest = block.starts >> bit >> 1;
                let end = if rest != 0 {
                    start + 1 + rest.trailing_zeros() as usize
                } else {
                    self.token_end(start)
                };
                kinds[token] = KEYWORD_KINDS[keyword(self.source, start, end) as usize];
            }
            first += block.starts.count_ones() as usize;
        }
    }
}
