#[path = "generated/data.rs"]
mod data;

pub use data::*;

/// Finds a module replacement mapping by module name.
///
/// Returns `Some` with a reference to the `ModuleReplacementMapping` if found, `None` otherwise.
pub fn find_mapping(module_name: &str) -> Option<&'static ModuleReplacementMapping> {
    MODULE_REPLACEMENTS_MAPPINGS.get(module_name)
}

/// Finds a module replacement by replacement ID.
///
/// Returns `Some` with a reference to the `ModuleReplacement` if found, `None` otherwise.
pub fn find_replacement(replacement_id: &str) -> Option<&'static ModuleReplacement> {
    MODULE_REPLACEMENTS.get(replacement_id)
}

/// Resolves a documentation URL to its string representation.
///
/// Returns `Some` with the resolved URL string if a URL is provided, `None` otherwise.
pub fn resolve_doc_url(url: Option<KnownUrl>) -> Option<String> {
    let url = url?;
    match url {
        KnownUrl::Raw(raw) => Some(raw.to_string()),
        KnownUrl::Descriptor { url_type, id } => match url_type {
            KnownUrlType::Mdn => Some(format!("https://developer.mozilla.org/en-US/docs/{id}")),
            KnownUrlType::Node => Some(format!("https://nodejs.org/{id}")),
            KnownUrlType::E18e => Some(format!("https://e18e.dev/docs/replacements/{id}")),
        },
    }
}
