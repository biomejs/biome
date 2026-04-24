use crate::{AnyAstroDirective, AnySvelteDirective, HtmlAttributeInitializerClause};

impl AnyAstroDirective {
    /// Returns the initializer from an Astro directive's value, if available.
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        match self {
            Self::AstroClassDirective(dir) => dir.value().ok()?.initializer(),
            Self::AstroClientDirective(dir) => dir.value().ok()?.initializer(),
            Self::AstroDefineDirective(dir) => dir.value().ok()?.initializer(),
            Self::AstroIsDirective(dir) => dir.value().ok()?.initializer(),
            Self::AstroServerDirective(dir) => dir.value().ok()?.initializer(),
            Self::AstroSetDirective(dir) => dir.value().ok()?.initializer(),
        }
    }
}

impl AnySvelteDirective {
    /// Returns the initializer from a Svelte directive's value, if available.
    pub fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        match self {
            Self::SvelteBindDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteTransitionDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteInDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteOutDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteUseDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteAnimateDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteStyleDirective(dir) => dir.value().ok()?.initializer(),
            Self::SvelteClassDirective(dir) => dir.value().ok()?.initializer(),
        }
    }
}
