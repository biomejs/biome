use crate::embed::detector::{EmbedDetector, EmbedTarget};
use crate::embed::types::{EmbedCandidate, GuestLanguage, HostLanguage};
use crate::workspace::DocumentFileSource;
use biome_html_syntax::ScriptType;

/// Returned by the registry when a detector matches.
pub(crate) struct EmbedMatch {
    pub guest: GuestLanguage,
}

pub(crate) struct EmbedDetectorsRegistry;

impl EmbedDetectorsRegistry {
    /// Look up the detectors for a given host language.
    pub const fn detectors(host: HostLanguage) -> &'static [EmbedDetector] {
        match host {
            HostLanguage::Html => &HTML_DETECTORS,
            HostLanguage::JavaScript => &JS_DETECTORS,
            HostLanguage::Markdown => &[], // Future: MARKDOWN_DETECTORS
        }
    }

    /// Find the first matching detector for a candidate.
    pub fn detect_match(
        host: HostLanguage,
        candidate: &EmbedCandidate,
        file_source: &DocumentFileSource,
    ) -> Option<EmbedMatch> {
        let detectors = Self::detectors(host);
        for detector in detectors {
            if let Some(guest) = detector.try_match(candidate, file_source) {
                return Some(EmbedMatch { guest });
            }
        }
        None
    }
}

static HTML_DETECTORS: [EmbedDetector; 5] = [
    // <script> → JS/TS/JSON (dynamic: depends on type/lang attributes + framework)
    //
    // A single detector handles all <script> variants via the dynamic resolver:
    //   - <script>                               → JsScript (classic HTML)
    //   - <script type="module">                 → JsModule (ES module)
    //   - <script lang="ts">                     → Ts (Vue/Svelte)
    //   - <script lang="tsx">                    → Tsx (Vue/Svelte)
    //   - <script lang="jsx">                    → Jsx (Vue/Svelte)
    //   - <script type="importmap">              → Json
    //   - <script type="application/json">       → Json
    //   - Astro <script> (no frontmatter)        → Ts
    //   - Vue/Svelte default                     → JsModule
    //   - <script type="speculationrules">       → skipped (unsupported type)
    //   - <script type="application/ld+json">    → skipped (unsupported type)
    //   - <script type="text/x-handlebars-*">    → skipped (unsupported type)
    //
    // The handler dispatches on embed_match.guest to call the right parser
    // (parse JS vs parse JSON). No separate detector needed for JSON scripts.
    EmbedDetector::Element {
        tag: "script",
        target: EmbedTarget::Dynamic {
            resolver: resolve_script_language,
            // No fallback: the resolver handles all supported cases explicitly.
            // Returning None means "skip this element" (e.g. unsupported type
            // like speculationrules or application/ld+json).
            fallback: None,
        },
    },
    // <style> → CSS (dynamic: skips SCSS via resolver returning None)
    EmbedDetector::Element {
        tag: "style",
        target: EmbedTarget::Dynamic {
            resolver: resolve_style_language,
            // No fallback — the resolver handles all cases explicitly.
            // Returning None from the resolver means "skip this element."
            fallback: None,
        },
    },
    // Astro frontmatter (--- ... ---) → TypeScript
    // Only matches EmbedCandidate::Frontmatter, which is only built for Astro.
    EmbedDetector::Frontmatter {
        target: EmbedTarget::Static(GuestLanguage::Ts),
    },
    // Template text expressions: { expr }, {{ expr }}, {#if expr}, etc.
    // Guest language depends on framework:
    //   - Standard HTML: JsModule
    //   - Astro: Tsx (with Astro embedding kind)
    //   - Vue/Svelte: JsModule (overridden by embedded_file_source in handler)
    EmbedDetector::TextExpression {
        target: EmbedTarget::Dynamic {
            resolver: resolve_text_expression_language,
            fallback: None,
        },
    },
    // Directive attribute values: @click="handler()", v-if="cond",
    // on:click={handler}, bind:value={x}, etc.
    // Always JS — framework determines EmbeddingKind in handler.
    EmbedDetector::Directive {
        target: EmbedTarget::Dynamic {
            resolver: resolve_directive_language,
            fallback: None,
        },
    },
];

/// Resolves the guest language for a <script> tag based on attributes and host framework.
/// Mirrors current logic in html.rs parse_embedded_script (978-1065).
fn resolve_script_language(
    candidate: &EmbedCandidate,
    file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    let html_source = file_source.to_html_file_source()?;

    // Check script type attribute first
    if let Some(type_value) = candidate.attribute("type") {
        let script_type = ScriptType::from_type_value(type_value.text());
        if script_type.is_json() {
            return Some(GuestLanguage::Json);
        }
        if !script_type.is_javascript() {
            return None; // unsupported type (e.g. text/x-handlebars-template)
        }
    }

    // Framework-specific resolution
    if html_source.is_vue() || html_source.is_svelte() {
        // Check lang attribute
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
        // Astro <script> tags (not frontmatter) default to TypeScript
        Some(GuestLanguage::Ts)
    } else {
        // Plain HTML
        if candidate.has_attribute_value("type", "module") {
            Some(GuestLanguage::JsModule)
        } else {
            Some(GuestLanguage::JsScript)
        }
    }
}

fn resolve_style_language(
    candidate: &EmbedCandidate,
    _file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    // SCSS is not supported — return None to skip this element
    if candidate.has_attribute_value("lang", "scss") {
        None
    } else {
        Some(GuestLanguage::Css)
    }
}

/// Resolves the guest language for template text expressions.
/// Astro expressions use TSX; everything else uses JsModule as a base
/// (the handler overrides with `embedded_file_source` for Vue/Svelte).
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

/// Resolves the guest language for directive attribute values.
/// Always JS — the handler applies framework-specific EmbeddingKind.
fn resolve_directive_language(
    _candidate: &EmbedCandidate,
    _file_source: &DocumentFileSource,
) -> Option<GuestLanguage> {
    Some(GuestLanguage::JsModule)
}

static JS_DETECTORS: [EmbedDetector; 5] = [
    // css`` → CSS
    EmbedDetector::TemplateTag {
        tag: "css",
        target: EmbedTarget::Static(GuestLanguage::Css),
    },
    // styled.div``, styled(Comp)`` → CSS
    EmbedDetector::TemplateExpression {
        object: "styled",
        target: EmbedTarget::Static(GuestLanguage::Css),
    },
    // gql`` → GraphQL
    EmbedDetector::TemplateTag {
        tag: "gql",
        target: EmbedTarget::Static(GuestLanguage::GraphQL),
    },
    // graphql`` → GraphQL
    EmbedDetector::TemplateTag {
        tag: "graphql",
        target: EmbedTarget::Static(GuestLanguage::GraphQL),
    },
    // graphql()`` → GraphQL
    EmbedDetector::TemplateExpression {
        object: "graphql",
        target: EmbedTarget::Static(GuestLanguage::GraphQL),
    },
];
