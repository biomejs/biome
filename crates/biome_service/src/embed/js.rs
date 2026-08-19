use super::EmbedContent;
use biome_languages::{CssFileSource, DocumentFileSource, GraphqlFileSource};
use biome_rowan::TokenText;

/// Language that can be embedded inside JavaScript template literals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GuestLanguage {
    Css,
    GraphQL,
}

impl From<GuestLanguage> for DocumentFileSource {
    fn from(value: GuestLanguage) -> Self {
        match value {
            GuestLanguage::Css => CssFileSource::css().into(),
            GuestLanguage::GraphQL => GraphqlFileSource::graphql().into(),
        }
    }
}

/// Describes where a potential embedded language can be found in JavaScript.
pub(crate) enum EmbedCandidate {
    TaggedTemplate {
        tag: TemplateTagKind,
        content: EmbedContent,
    },
}

impl EmbedCandidate {
    pub fn content(&self) -> EmbedContent {
        match self {
            Self::TaggedTemplate { content, .. } => content.clone(),
        }
    }
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

static JS_DETECTORS: [EmbedDetector; 5] = [
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
];
