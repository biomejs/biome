//! Rule documentation parsing and lowering.
//!
//! Analyzer rules store their documentation as Markdown. The consumers of that
//! Markdown need more than rendered prose: fenced blocks describe executable
//! examples and configuration, headings delimit multi-file test contexts, and
//! raw HTML is validated before the website renders it as MDX. This crate lowers
//! those constructs into an owned, source-ordered representation while keeping
//! Markdown parsing details out of the consumers.
//!
//! Summary lowering is separate from document lowering because configuration
//! generation needs only the first paragraph, while rule checks and website
//! generation need the document-level constructs.

mod hir;
mod lower;
mod summary;

pub use hir::*;
pub use summary::RuleSummaryError;

use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
use biome_parser::prelude::ParseDiagnostic;

/// The result of parsing and lowering a complete rule document.
///
/// Lowering uses the recovered syntax tree even when the Markdown parser emits
/// diagnostics. Callers decide whether diagnostics should prevent generation or
/// validation.
#[derive(Debug)]
pub struct RuleDocumentParse {
    document: RuleDocument,
    diagnostics: Vec<ParseDiagnostic>,
}

impl RuleDocumentParse {
    /// Returns the source-ordered constructs used by documentation consumers.
    pub fn document(&self) -> &RuleDocument {
        &self.document
    }

    /// Transfers ownership of the lowered document and discards its parser
    /// diagnostics.
    pub fn into_document(self) -> RuleDocument {
        self.document
    }

    /// Returns diagnostics emitted while parsing the Markdown source.
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }
}

/// Parses Markdown and lowers the headings, fenced code blocks, and raw HTML
/// used by rule documentation consumers.
///
/// The parser uses the default CommonMark-compatible options: front matter,
/// GitHub Flavored Markdown extensions, and CJK-friendly emphasis are disabled.
/// Parser diagnostics are returned alongside the recovered document.
pub fn parse_rule_document(source: &str) -> RuleDocumentParse {
    let parse = parse_markdown(source, MarkdownParserOptions::default());
    let document = lower::lower_document(&parse);
    let diagnostics = parse.into_diagnostics();

    RuleDocumentParse {
        document,
        diagnostics,
    }
}

/// The result of parsing and lowering a rule summary paragraph.
///
/// The summary result is independent of parser diagnostics. A recovered tree
/// can therefore contain both parser diagnostics and a successfully lowered
/// summary, or a [`RuleSummaryError`] for a construct that summaries do not
/// support.
#[derive(Debug)]
pub struct RuleSummaryParse {
    summary: Result<Option<RuleSummary>, RuleSummaryError>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl RuleSummaryParse {
    /// Returns the first content paragraph as normalized summary fragments.
    ///
    /// Blank lines and link-reference definitions before the paragraph are
    /// ignored. Returns `Ok(None)` when the document has no rendered content.
    /// Returns an error when the first content block is not a paragraph or the
    /// paragraph contains an unsupported inline construct.
    pub fn summary(&self) -> Result<Option<&RuleSummary>, &RuleSummaryError> {
        self.summary.as_ref().map(Option::as_ref)
    }

    /// Returns diagnostics emitted while parsing the Markdown source.
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }
}

/// Parses Markdown and lowers its first content paragraph for use as a rule
/// summary.
///
/// Blank lines and link-reference definitions before the paragraph are ignored.
/// Summary paragraphs support text, character references, emphasis, strong
/// emphasis, inline code, and soft line breaks. Other block or inline constructs
/// produce a [`RuleSummaryError`]. Parser diagnostics are returned separately
/// from summary-lowering errors.
pub fn parse_rule_summary(source: &str) -> RuleSummaryParse {
    let parse = parse_markdown(source, MarkdownParserOptions::default());
    let summary = summary::lower_summary(&parse.tree());
    let diagnostics = parse.into_diagnostics();

    RuleSummaryParse {
        summary,
        diagnostics,
    }
}

#[cfg(test)]
mod tests;
