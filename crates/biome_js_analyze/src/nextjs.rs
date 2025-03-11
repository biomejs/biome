use biome_js_semantic::Binding;
use biome_js_syntax::JsImport;
use biome_rowan::AstNode;

/// Represent Next.js built-in utility.
#[derive(Debug, Clone, Copy)]
pub(crate) enum NextUtility {
    Script,
    // you can add more here
}

impl NextUtility {
    pub const fn import_names(self) -> &'static [&'static str] {
        match self {
            NextUtility::Script => &["next/script"],
        }
    }
}

/// Check if the binding is an import from a specific Next.js built-in utility.
pub(crate) fn is_next_import(binding: &Binding, lib: NextUtility) -> bool {
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| lib.import_names().contains(&source.text()))
}
