use biome_js_syntax::JsFileSource;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SemanticFlavor {
    /// Regular JavaScript/TypeScript semantics.
    #[default]
    Vanilla,
    /// Svelte-specific semantics, such as `$store` dereference syntax.
    Svelte,
}

const SVELTE_RUNES: [&str; 7] = [
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
        self == Self::Svelte && SVELTE_RUNES.contains(&name)
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

impl From<&JsFileSource> for SemanticFlavor {
    fn from(source_type: &JsFileSource) -> Self {
        if source_type.as_embedding_kind().is_svelte() {
            Self::Svelte
        } else {
            Self::Vanilla
        }
    }
}

pub fn svelte_runes() -> &'static [&'static str; 7] {
    &SVELTE_RUNES
}
