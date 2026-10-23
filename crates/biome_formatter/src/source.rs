//! Describes the source code that the formatter formats.
//!
//! The formatter remembers where each piece of its output comes from in the
//! source code. The printer uses these locations to build a source map, and
//! the caller uses them to replace embedded code, such as the CSS inside an
//! HTML `<style>` tag. The formatter never reads the locations itself: a
//! [Source] chooses their types, and the formatter stores them and hands them
//! back.

use crate::Printed;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, Range};
use std::rc::Rc;

/// Describes the source code that a formatter formats.
///
/// Two documents with the same [Source] can be combined. For example, the
/// document of the CSS inside an HTML `<style>` tag can replace the content of
/// that tag in the HTML document.
pub trait Source: Copy + Debug + Default + Eq + Hash + 'static {
    /// A location in the source code, such as a byte offset.
    ///
    /// The default value is the location of the first character.
    type Position: Copy + Debug + Default + Eq + Hash;

    /// A range of the source code, such as a pair of byte offsets.
    type Span: Clone + Debug + Eq;

    /// Text taken from the source code, which a document can store without
    /// copying it.
    type Text: Clone + Debug + Eq + Deref<Target = str>;

    /// Identifies a piece of code written in another language, such as the
    /// JavaScript inside an HTML `<script>` tag.
    type EmbeddedKey: Clone + Debug + Eq + Hash;

    /// Maps positions back to the original source code when the source code
    /// was changed before formatting.
    type SourceMap: SourceMap<Self>;

    /// Returns the position right after `text`, given that `text` starts at
    /// `position`.
    ///
    /// `text` is the text the formatter writes for that position. Its line
    /// endings are already normalized, so it can differ from the source code.
    fn advance(position: Self::Position, text: &str) -> Self::Position;

    /// Returns the range from `start` to `end`.
    fn span(start: Self::Position, end: Self::Position) -> Self::Span;
}

/// Changes the source positions of printed output, for example to point at
/// the source code as it was before it was changed for formatting.
pub trait SourceMap<S: Source> {
    /// Returns `printed` with its source positions changed.
    fn map_printed(&self, printed: Printed<S>) -> Printed<S>;
}

/// A plain-text source, with positions as byte offsets.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PlainSource;

impl Source for PlainSource {
    type Position = u32;
    type Span = Range<u32>;
    type Text = Rc<str>;
    type EmbeddedKey = u32;
    type SourceMap = NoSourceMap;

    fn advance(position: Self::Position, text: &str) -> Self::Position {
        position + u32::try_from(text.len()).expect("text length to fit in a u32")
    }

    fn span(start: Self::Position, end: Self::Position) -> Self::Span {
        start..end
    }
}

/// The source map of source code that is never changed before formatting.
#[derive(Debug)]
pub enum NoSourceMap {}

impl<S: Source> SourceMap<S> for NoSourceMap {
    fn map_printed(&self, _printed: Printed<S>) -> Printed<S> {
        match *self {}
    }
}
