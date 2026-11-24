use super::SemanticModel;
use biome_js_syntax::{JsReferenceIdentifier, unescape_js_identifier};
use biome_languages::JsFileSource;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SemanticFlavor {
    /// Regular JavaScript/TypeScript semantics.
    #[default]
    Vanilla,
    /// Svelte component scripts and template expressions support runes and store subscriptions.
    Svelte,
    /// Standalone `.svelte.js` and `.svelte.ts` modules support runes but not store subscriptions.
    SvelteSourceModule,
}

pub const SVELTE_RUNES: [&str; 7] = [
    "$bindable",
    "$derived",
    "$effect",
    "$host",
    "$inspect",
    "$props",
    "$state",
];

impl SemanticFlavor {
    pub fn is_svelte_rune(self, name: &str) -> bool {
        matches!(self, Self::Svelte | Self::SvelteSourceModule) && SVELTE_RUNES.contains(&name)
    }

    /// Returns the store binding name for Svelte auto-subscription references (`$store` -> `store`).
    /// It only matches a single leading `$` and excludes runes.
    pub fn store_reference_name(self, reference_name: &str) -> Option<&str> {
        if self != Self::Svelte {
            return None;
        }
        if self.is_svelte_rune(reference_name) {
            return None;
        }
        let store_name = reference_name.strip_prefix('$')?;
        if store_name.is_empty() || store_name.starts_with('$') {
            return None;
        }
        Some(store_name)
    }
}

impl SemanticModel {
    /// A lexically bound identifier takes precedence over the Svelte rune of the same name.
    pub fn is_svelte_rune_reference(&self, reference: &JsReferenceIdentifier, rune: &str) -> bool {
        self.flavor().is_svelte_rune(rune)
            && reference
                .value_token()
                .is_ok_and(|name| unescape_js_identifier(name.text_trimmed()) == rune)
            && self.binding(reference).is_none()
    }
}

impl From<&JsFileSource> for SemanticFlavor {
    fn from(source_type: &JsFileSource) -> Self {
        if source_type.is_svelte_source_module() {
            Self::SvelteSourceModule
        } else if source_type.is_svelte_component() {
            Self::Svelte
        } else {
            Self::Vanilla
        }
    }
}
