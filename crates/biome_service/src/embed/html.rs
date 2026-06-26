use super::EmbedContent;
use biome_html_syntax::{AnySvelteBlock, HtmlSyntaxKind, ScriptType};
use biome_languages::javascript::{Language, SvelteVariableKind};
use biome_languages::{CssFileSource, DocumentFileSource, JsFileSource, JsonFileSource};
use biome_rowan::TokenText;

/// Language that can be embedded inside HTML-like files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GuestLanguage {
    JsModule,
    JsScript,
    Jsx,
    Ts,
    Tsx,
    Css,
    Json,
}

impl From<GuestLanguage> for DocumentFileSource {
    fn from(value: GuestLanguage) -> Self {
        match value {
            GuestLanguage::JsModule => JsFileSource::js_module().into(),
            GuestLanguage::JsScript => JsFileSource::js_script().into(),
            GuestLanguage::Jsx => JsFileSource::jsx().into(),
            GuestLanguage::Ts => JsFileSource::ts().into(),
            GuestLanguage::Tsx => JsFileSource::tsx().into(),
            GuestLanguage::Css => CssFileSource::css().into(),
            GuestLanguage::Json => JsonFileSource::json().into(),
        }
    }
}

impl GuestLanguage {
    pub(crate) fn from_js_source(source: &JsFileSource) -> Self {
        match source.language() {
            Language::JavaScript => {
                if source.is_script() {
                    Self::JsScript
                } else if source.is_jsx() {
                    Self::Jsx
                } else {
                    Self::JsModule
                }
            }
            Language::TypeScript { .. } => {
                if source.is_jsx() {
                    Self::Tsx
                } else {
                    Self::Ts
                }
            }
        }
    }
}

/// Describes where a potential embedded language can be found in an HTML-like file.
pub(crate) enum EmbedCandidate {
    Element {
        tag_name: TokenText,
        attributes: Vec<(TokenText, Option<TokenText>)>,
        content: EmbedContent,
        is_global: bool,
    },
    Frontmatter {
        content: EmbedContent,
    },
    TextExpression {
        content: EmbedContent,
        block_kind: EmbedBlockKind,
    },
    Directive {
        content: EmbedContent,
        is_event_handler: bool,
        is_class_attribute: bool,
    },
}

#[derive(Debug, Default)]
pub(crate) enum EmbedBlockKind {
    Svelte(SvelteBlockKind),
    #[default]
    Neutral,
}

#[derive(Debug)]
pub(crate) enum SvelteBlockKind {
    Render,
    Snippet,
    Const,
    Declaration(SvelteVariableKind),
}

impl From<&AnySvelteBlock> for EmbedBlockKind {
    fn from(value: &AnySvelteBlock) -> Self {
        match value {
            AnySvelteBlock::SvelteAwaitBlock(_)
            | AnySvelteBlock::SvelteBogusBlock(_)
            | AnySvelteBlock::SvelteDebugBlock(_)
            | AnySvelteBlock::SvelteEachBlock(_)
            | AnySvelteBlock::SvelteHtmlBlock(_)
            | AnySvelteBlock::SvelteIfBlock(_)
            | AnySvelteBlock::SvelteKeyBlock(_) => Self::Neutral,
            AnySvelteBlock::SvelteConstBlock(_) => Self::Svelte(SvelteBlockKind::Const),
            AnySvelteBlock::SvelteDeclarationBlock(block) => {
                let kind = match block.keyword_token().map(|token| token.kind()) {
                    Ok(HtmlSyntaxKind::CONST_KW) => SvelteVariableKind::Const,
                    _ => SvelteVariableKind::Let,
                };
                Self::Svelte(SvelteBlockKind::Declaration(kind))
            }
            AnySvelteBlock::SvelteRenderBlock(_) => Self::Svelte(SvelteBlockKind::Render),
            AnySvelteBlock::SvelteSnippetBlock(_) => Self::Svelte(SvelteBlockKind::Snippet),
        }
    }
}

impl EmbedCandidate {
    pub fn content(&self) -> EmbedContent {
        match self {
            Self::Element { content, .. }
            | Self::Frontmatter { content }
            | Self::TextExpression { content, .. }
            | Self::Directive { content, .. } => content.clone(),
        }
    }

    pub fn attribute(&self, name: &str) -> Option<&TokenText> {
        match self {
            Self::Element { attributes, .. } => attributes
                .iter()
                .find(|(key, _)| key.text().eq_ignore_ascii_case(name))
                .and_then(|(_, value)| value.as_ref()),
            _ => None,
        }
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        match self {
            Self::Element { attributes, .. } => attributes
                .iter()
                .any(|(key, _)| key.text().eq_ignore_ascii_case(name)),
            _ => false,
        }
    }

    pub fn has_attribute_value(&self, name: &str, value: &str) -> bool {
        match self {
            Self::Element { attributes, .. } => attributes.iter().any(|(key, attr_value)| {
                key.text().eq_ignore_ascii_case(name)
                    && attr_value
                        .as_ref()
                        .is_some_and(|attr_value| attr_value.text().eq_ignore_ascii_case(value))
            }),
            _ => false,
        }
    }

    pub fn is_css_global(&self) -> bool {
        match self {
            Self::Element { is_global, .. } => *is_global,
            _ => false,
        }
    }
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
        for detector in HTML_DETECTORS.iter() {
            if let Some(guest) = detector.try_match(candidate, file_source) {
                return Some(EmbedMatch { guest });
            }
        }
        None
    }
}

enum EmbedDetector {
    Element {
        tag: &'static str,
        target: EmbedTarget,
    },
    Frontmatter {
        target: EmbedTarget,
    },
    TextExpression {
        target: EmbedTarget,
    },
    Directive {
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
            (Self::Element { tag, target }, EmbedCandidate::Element { tag_name, .. }) => {
                if tag_name.text().eq_ignore_ascii_case(tag) {
                    target.resolve(candidate, file_source)
                } else {
                    None
                }
            }
            (Self::Frontmatter { target }, EmbedCandidate::Frontmatter { .. }) => {
                target.resolve(candidate, file_source)
            }
            (Self::TextExpression { target }, EmbedCandidate::TextExpression { .. }) => {
                target.resolve(candidate, file_source)
            }
            (Self::Directive { target }, EmbedCandidate::Directive { .. }) => {
                target.resolve(candidate, file_source)
            }
            _ => None,
        }
    }
}

enum EmbedTarget {
    Static(GuestLanguage),
    Dynamic {
        resolver: fn(&EmbedCandidate, &DocumentFileSource) -> Option<GuestLanguage>,
        fallback: Option<GuestLanguage>,
    },
}

impl EmbedTarget {
    fn resolve(
        &self,
        candidate: &EmbedCandidate,
        file_source: &DocumentFileSource,
    ) -> Option<GuestLanguage> {
        match self {
            Self::Static(guest) => Some(*guest),
            Self::Dynamic { resolver, fallback } => resolver(candidate, file_source).or(*fallback),
        }
    }
}

static HTML_DETECTORS: [EmbedDetector; 5] = [
    EmbedDetector::Element {
        tag: "script",
        target: EmbedTarget::Dynamic {
            resolver: resolve_script_language,
            fallback: None,
        },
    },
    EmbedDetector::Element {
        tag: "style",
        target: EmbedTarget::Dynamic {
            resolver: resolve_style_language,
            fallback: None,
        },
    },
    EmbedDetector::Frontmatter {
        target: EmbedTarget::Static(GuestLanguage::Ts),
    },
    EmbedDetector::TextExpression {
        target: EmbedTarget::Dynamic {
            resolver: resolve_text_expression_language,
            fallback: None,
        },
    },
    EmbedDetector::Directive {
        target: EmbedTarget::Dynamic {
            resolver: resolve_directive_language,
            fallback: None,
        },
    },
];

fn resolve_script_language(
    candidate: &EmbedCandidate,
    file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    let html_source = file_source.to_html_file_source()?;

    if let Some(type_value) = candidate.attribute("type") {
        let script_type = ScriptType::from_type_value(type_value.text());
        if script_type.is_json() {
            return Some(GuestLanguage::Json);
        }
        if !script_type.is_javascript() {
            return None;
        }
    }

    if html_source.is_vue() || html_source.is_svelte() {
        if candidate.has_attribute_value("lang", "ts") {
            Some(GuestLanguage::Ts)
        } else if candidate.has_attribute_value("lang", "tsx") {
            Some(GuestLanguage::Tsx)
        } else if candidate.has_attribute_value("lang", "jsx") {
            Some(GuestLanguage::Jsx)
        } else {
            Some(GuestLanguage::JsModule)
        }
    } else if html_source.is_astro() {
        Some(GuestLanguage::Ts)
    } else if candidate.has_attribute_value("type", "module") {
        Some(GuestLanguage::JsModule)
    } else {
        Some(GuestLanguage::JsScript)
    }
}

fn resolve_style_language(
    candidate: &EmbedCandidate,
    _file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    if candidate.has_attribute_value("lang", "scss") {
        None
    } else {
        Some(GuestLanguage::Css)
    }
}

fn resolve_text_expression_language(
    _candidate: &EmbedCandidate,
    file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    let html_source = file_source.to_html_file_source()?;
    if html_source.is_astro() {
        Some(GuestLanguage::Tsx)
    } else {
        Some(GuestLanguage::JsModule)
    }
}

fn resolve_directive_language(
    _candidate: &EmbedCandidate,
    _file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    Some(GuestLanguage::JsModule)
}
