use super::EmbedContent;
use biome_languages::DocumentFileSource;
use biome_languages::HtmlFileSource;
use biome_languages::YamlFileSource;
use biome_rowan::Text;
use biome_string_case::StrLikeExtension;

/// Describes embedded content extracted from a Markdown syntax node.
pub(crate) enum EmbedCandidate {
    Frontmatter {
        content: EmbedContent,
    },
    CodeBlock {
        content: EmbedContent,
        info_string: Text,
    },
    HtmlBlock {
        content: EmbedContent,
    },
}

impl EmbedCandidate {
    pub(crate) fn content(&self) -> EmbedContent {
        match self {
            Self::Frontmatter { content }
            | Self::CodeBlock { content, .. }
            | Self::HtmlBlock { content } => content.clone(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct EmbedMatch {
    pub(crate) file_source: DocumentFileSource,
}

pub(crate) struct EmbedDetectorsRegistry;

impl EmbedDetectorsRegistry {
    pub(crate) fn detect_match(candidate: &EmbedCandidate) -> Option<EmbedMatch> {
        let file_source = match candidate {
            EmbedCandidate::Frontmatter { .. } => YamlFileSource::yaml().into(),
            EmbedCandidate::CodeBlock { info_string, .. } => {
                resolve_code_block_language(info_string.text())?
            }
            EmbedCandidate::HtmlBlock { .. } => HtmlFileSource::html().into(),
        };
        Some(EmbedMatch { file_source })
    }
}

fn resolve_code_block_language(info_string: &str) -> Option<DocumentFileSource> {
    let language = info_string
        .split_whitespace()
        .next()?
        .to_ascii_lowercase_cow();

    let file_source = DocumentFileSource::from_extension(&language, true);
    if file_source != DocumentFileSource::Unknown {
        return Some(file_source);
    }

    let file_source = DocumentFileSource::from_language_id(&language, None);
    (file_source != DocumentFileSource::Unknown).then_some(file_source)
}
