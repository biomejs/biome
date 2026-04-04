use crate::workspace::DocumentFileSource;
use biome_css_syntax::CssFileSource;
use biome_graphql_syntax::GraphqlFileSource;
use biome_js_syntax::JsFileSource;
use biome_json_syntax::JsonFileSource;
use biome_rowan::{TextRange, TextSize, TokenText};

/// Language that can host embeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HostLanguage {
    Html,
    JavaScript,
    #[expect(dead_code)]
    Markdown,
}

/// Language that can be embedded inside a host. Returned by resolvers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GuestLanguage {
    JsModule,
    JsScript,
    Jsx,
    Ts,
    Tsx,
    Css,
    GraphQL,
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
            GuestLanguage::GraphQL => GraphqlFileSource::graphql().into(),
            GuestLanguage::Json => JsonFileSource::json().into(),
        }
    }
}

/// Describes where a potential embedded language can be found inside an host language.
///
/// A `<script>` tag inside an HTML file, a `css` tagged template inside
/// a JavaScript file, or a Vue directive like `@click="handler()"` are all
/// examples of places where embedded code can appear.
pub(crate) enum EmbedCandidate {
    /// An HTML element like `<script>` or `<style>`.
    /// Built from `HtmlElement` by the HTML handler.
    Element {
        tag_name: TokenText,
        /// Attribute (name, optional_value) pairs.
        /// Bare attributes like `setup` have `None` value.
        attributes: Vec<(TokenText, Option<TokenText>)>,
        content: EmbedContent,
    },

    /// Astro frontmatter block: `---\ncode\n---`.
    /// Built from `AstroEmbeddedContent` by the HTML handler.
    Frontmatter { content: EmbedContent },

    /// A JS tagged template literal like `css\`...\``, `styled.div\`...\``.
    /// Built from `JsTemplateExpression` by the JS handler.
    TaggedTemplate {
        tag: TemplateTagKind,
        content: EmbedContent,
    },

    /// An inline JS expression in template syntax.
    /// Covers `{ expr }` (single), `{{ expr }}` (double), and Svelte control
    /// flow blocks (`{#if}`, `{#each}`, `{#await}`, `{#key}`).
    /// Built from `HtmlTextExpression` by the HTML handler.
    TextExpression { content: EmbedContent },

    /// A directive attribute value containing JS.
    /// - Vue: `@click="handler()"`, `:prop="value"`, `v-if="cond"`
    /// - Svelte: `on:click={handler}`, `bind:value={x}`
    /// - Astro: `define:vars={{ x }}`,` class:list={[...]}`, `set:text={expr}`
    ///
    /// Built from `HtmlAttributeInitializerClause` by the HTML handler.
    Directive {
        content: EmbedContent,
        /// True for Vue `v-on:` / `@` event handler directives.
        /// Affects `EmbeddingKind::Vue { event_handler }`.
        is_event_handler: bool,
        /// True when the directive is a class-related attribute
        /// (e.g., `class:list={...}` in Astro, `class={...}` in plain HTML).
        /// Affects `EmbeddingKind::Astro { is_class_attribute }`.
        is_class_attribute: bool,
    },
}

/// The text content and position information for an embed site.
/// Shared across all `EmbedCandidate` variants.
pub(crate) struct EmbedContent {
    /// The text range of the entire host element (including tags/delimiters).
    pub element_range: TextRange,

    /// The text range of just the embedded content.
    pub content_range: TextRange,

    /// Offset where embedded content starts in the parent document.
    pub content_offset: TextSize,

    /// The raw text of the embedded content.
    pub text: TokenText,
}

impl EmbedCandidate {
    /// Access the content, regardless of variant.
    pub fn content(&self) -> &EmbedContent {
        match self {
            Self::Element { content, .. }
            | Self::Frontmatter { content }
            | Self::TaggedTemplate { content, .. }
            | Self::TextExpression { content }
            | Self::Directive { content, .. } => content,
        }
    }

    /// Lookup an attribute value by name (case-insensitive).
    /// Returns `None` for non-Element variants or bare attributes.
    pub fn attribute(&self, name: &str) -> Option<&TokenText> {
        match self {
            Self::Element { attributes, .. } => attributes
                .iter()
                .find(|(k, _)| k.text().eq_ignore_ascii_case(name))
                .and_then(|(_, v)| v.as_ref()),
            _ => None,
        }
    }

    /// Check if an attribute exists (case-insensitive).
    /// Returns `true` for bare attributes (no value) too.
    /// Returns `false` for non-Element variants.
    pub fn has_attribute(&self, name: &str) -> bool {
        match self {
            Self::Element { attributes, .. } => attributes
                .iter()
                .any(|(k, _)| k.text().eq_ignore_ascii_case(name)),
            _ => false,
        }
    }

    /// Check if an attribute has a specific value (case-insensitive for both).
    /// Returns `false` for bare attributes (no value) and non-Element variants.
    pub fn has_attribute_value(&self, name: &str, value: &str) -> bool {
        match self {
            Self::Element { attributes, .. } => attributes.iter().any(|(k, v)| {
                k.text().eq_ignore_ascii_case(name)
                    && v.as_ref()
                        .is_some_and(|v| v.text().eq_ignore_ascii_case(value))
            }),
            _ => false,
        }
    }
}

/// Describes how a JS template tag was classified.
/// Uses `TokenText` — zero-copy from the syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TemplateTagKind {
    /// Direct identifier tag: css``, gql``, graphql``
    Identifier(TokenText),
    /// Member expression: styled.div``
    MemberExpression {
        object: TokenText,
        property: TokenText,
    },
    /// Call expression: styled(Component)``, graphql(``)
    CallExpression { callee: TokenText },
}
