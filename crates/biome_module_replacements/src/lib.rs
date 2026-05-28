#[path = "generated/data.rs"]
mod data;

pub use data::*;

pub fn find_mapping(module_name: &str) -> Option<&'static ModuleReplacementMapping> {
    MODULE_REPLACEMENTS_MAPPINGS
        .iter()
        .find_map(|(name, mapping)| (*name == module_name).then_some(mapping))
}

pub fn find_replacement(replacement_id: &str) -> Option<&'static ModuleReplacement> {
    MODULE_REPLACEMENTS
        .iter()
        .find_map(|(name, replacement)| (*name == replacement_id).then_some(replacement))
}

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
