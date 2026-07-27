use super::EmbedContent;
use biome_languages::{CssFileSource, DocumentFileSource, GraphqlFileSource, HtmlFileSource};
use biome_rowan::{TextRange, TextSize, TokenText};

/// Language that can be embedded inside JavaScript template literals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GuestLanguage {
    Css,
    GraphQL,
    Html,
}

impl From<GuestLanguage> for DocumentFileSource {
    fn from(value: GuestLanguage) -> Self {
        match value {
            GuestLanguage::Css => CssFileSource::css().into(),
            GuestLanguage::GraphQL => GraphqlFileSource::graphql().into(),
            GuestLanguage::Html => HtmlFileSource::html().into(),
        }
    }
}

/// Describes where a potential embedded language can be found in JavaScript.
pub(crate) enum EmbedCandidate {
    TaggedTemplate {
        tag: TemplateTagKind,
        content: EmbedContent,
        /// For templates with interpolations, the combined text with placeholders
        /// and per-chunk slice info. `None` for single-chunk templates.
        combined_chunks: Option<CombinedEmbedContent>,
    },
}

impl EmbedCandidate {
    pub fn content(&self) -> EmbedContent {
        match self {
            Self::TaggedTemplate { content, .. } => content.clone(),
        }
    }

    /// Returns the text to parse for this candidate.
    /// For single-chunk templates, returns the chunk's text.
    /// For multi-chunk templates, returns the combined text with placeholders.
    pub fn combined_text(&self) -> String {
        match self {
            Self::TaggedTemplate {
                combined_chunks: Some(combined),
                ..
            } => combined.combined_text.clone(),
            Self::TaggedTemplate { content, .. } => content.text.text().to_string(),
        }
    }

    /// Returns the combined content info, if this is a multi-chunk template.
    pub fn combined_chunks(&self) -> Option<&CombinedEmbedContent> {
        match self {
            Self::TaggedTemplate {
                combined_chunks, ..
            } => combined_chunks.as_ref(),
        }
    }
}

/// Information about a placeholder in the combined embedded text.
#[derive(Debug, Clone)]
pub(crate) struct PlaceholderSlice {
    /// The original chunk's text range in the source document.
    pub chunk_range: TextRange,
    /// The byte offset where this chunk's text starts in the combined text.
    pub combined_start: TextSize,
    /// The byte offset where this chunk's text ends in the combined text.
    pub combined_end: TextSize,
}

/// Combined embedded content for templates with interpolations.
/// Contains the full concatenated text with placeholders and slice positions.
#[derive(Debug, Clone)]
pub(crate) struct CombinedEmbedContent {
    pub combined_text: String,
    pub slices: Vec<PlaceholderSlice>,
    pub base_offset: TextSize,
}

/// Describes how a JavaScript template tag was classified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TemplateTagKind {
    Identifier(TokenText),
    MemberExpression {
        object: TokenText,
        property: TokenText,
    },
    CallExpression {
        callee: TokenText,
    },
}

#[derive(Debug)]
pub(crate) struct EmbedMatch {
    pub guest: GuestLanguage,
}

pub(crate) struct EmbedDetectorsRegistry;

impl EmbedDetectorsRegistry {
    pub fn detect_match(
        candidate: &EmbedCandidate,
        file_source: &DocumentFileSource,
    ) -> Option<EmbedMatch> {
        for detector in JS_DETECTORS.iter() {
            if let Some(guest) = detector.try_match(candidate, file_source) {
                return Some(EmbedMatch { guest });
            }
        }
        None
    }
}

enum EmbedDetector {
    TemplateTag {
        tag: &'static str,
        target: EmbedTarget,
    },
    TemplateExpression {
        object: &'static str,
        target: EmbedTarget,
    },
}

impl EmbedDetector {
    fn try_match(
        &self,
        candidate: &EmbedCandidate,
        file_source: &DocumentFileSource,
    ) -> Option<GuestLanguage> {
        match (self, candidate) {
            (
                Self::TemplateTag { tag, target },
                EmbedCandidate::TaggedTemplate {
                    tag: TemplateTagKind::Identifier(name),
                    ..
                },
            ) => {
                if name.text() == *tag {
                    target.resolve(candidate, file_source)
                } else {
                    None
                }
            }
            (
                Self::TemplateExpression { object, target },
                EmbedCandidate::TaggedTemplate { tag, .. },
            ) => match tag {
                TemplateTagKind::MemberExpression { object: obj, .. } => {
                    if obj.text() == *object {
                        target.resolve(candidate, file_source)
                    } else {
                        None
                    }
                }
                TemplateTagKind::CallExpression { callee } => {
                    if callee.text() == *object {
                        target.resolve(candidate, file_source)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

enum EmbedTarget {
    Static(GuestLanguage),
}

impl EmbedTarget {
    fn resolve(
        &self,
        _candidate: &EmbedCandidate,
        _file_source: &DocumentFileSource,
    ) -> Option<GuestLanguage> {
        match self {
            Self::Static(guest) => Some(*guest),
        }
    }
}

static JS_DETECTORS: [EmbedDetector; 6] = [
    EmbedDetector::TemplateTag {
        tag: "css",
        target: EmbedTarget::Static(GuestLanguage::Css),
    },
    EmbedDetector::TemplateExpression {
        object: "styled",
        target: EmbedTarget::Static(GuestLanguage::Css),
    },
    EmbedDetector::TemplateTag {
        tag: "gql",
        target: EmbedTarget::Static(GuestLanguage::GraphQL),
    },
    EmbedDetector::TemplateTag {
        tag: "graphql",
        target: EmbedTarget::Static(GuestLanguage::GraphQL),
    },
    EmbedDetector::TemplateExpression {
        object: "graphql",
        target: EmbedTarget::Static(GuestLanguage::GraphQL),
    },
    EmbedDetector::TemplateTag {
        tag: "html",
        target: EmbedTarget::Static(GuestLanguage::Html),
    },
];
