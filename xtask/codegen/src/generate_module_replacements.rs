use anyhow::Context;
use proc_macro2::Literal;
use quote::quote;
use serde::Deserialize;
use std::collections::BTreeMap;
use ureq::get;
use xtask_codegen::update;
use xtask_glue::{Mode, Result, project_root};

const E18E_MICRO_UTILITIES_DATA_URL: &str = "https://raw.githubusercontent.com/e18e/module-replacements/refs/heads/main/manifests/micro-utilities.json";
const E18E_NATIVE_DATA_URL: &str = "https://raw.githubusercontent.com/e18e/module-replacements/refs/heads/main/manifests/native.json";
const E18E_PREFERRED_DATA_URL: &str = "https://raw.githubusercontent.com/e18e/module-replacements/refs/heads/main/manifests/preferred.json";

#[derive(Debug, Deserialize)]
pub struct ManifestModule {
    mappings: BTreeMap<String, ModuleReplacementMapping>,
    replacements: BTreeMap<String, ModuleReplacement>,
}

#[derive(Debug, Deserialize)]
pub struct EngineConstraint {
    engine: String,
    #[serde(rename = "minVersion")]
    min_version: Option<String>,
    #[serde(rename = "maxVersion")]
    max_version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KnownUrlType {
    Mdn,
    Node,
    E18e,
}

#[derive(Debug, Deserialize)]
pub struct KnownUrlDescriptor {
    #[serde(rename = "type")]
    url_type: KnownUrlType,
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KnownUrl {
    Descriptor(KnownUrlDescriptor),
    Raw(String),
}

#[derive(Debug, Deserialize)]
pub struct ModuleReplacementLike {
    id: String,
    engines: Option<Vec<EngineConstraint>>,
    preferred: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentedModuleReplacement {
    #[serde(flatten)]
    common: ModuleReplacementLike,
    #[serde(rename = "replacementModule")]
    replacement_module: String,
    url: Option<KnownUrl>,
}

#[derive(Debug, Deserialize)]
pub struct NativeWebFeatureId {
    #[serde(rename = "featureId")]
    feature_id: String,
    #[serde(rename = "compatKey")]
    compat_key: String,
}

#[derive(Debug, Deserialize)]
pub struct NativeNodeFeatureId {
    #[serde(rename = "moduleName")]
    module_name: String,
    #[serde(rename = "exportName")]
    export_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NativeModuleReplacement {
    #[serde(flatten)]
    common: ModuleReplacementLike,
    url: KnownUrl,
    description: Option<String>,
    #[serde(rename = "webFeatureId")]
    web_feature_id: Option<NativeWebFeatureId>,
    #[serde(rename = "nodeFeatureId")]
    node_feature_id: Option<NativeNodeFeatureId>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleModuleReplacement {
    #[serde(flatten)]
    common: ModuleReplacementLike,
    description: String,
    example: Option<String>,
    url: Option<KnownUrl>,
}

#[derive(Debug, Deserialize)]
pub struct RemovalReplacement {
    #[serde(flatten)]
    common: ModuleReplacementLike,
    description: String,
    url: Option<KnownUrl>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ModuleReplacement {
    #[serde(rename = "documented")]
    Documented(DocumentedModuleReplacement),
    #[serde(rename = "native")]
    Native(NativeModuleReplacement),
    #[serde(rename = "simple")]
    Simple(SimpleModuleReplacement),
    #[serde(rename = "removal")]
    Removal(RemovalReplacement),
}

#[derive(Debug, Deserialize)]
pub struct ModuleReplacementMapping {
    #[serde(rename = "type")]
    mapping_type: String,
    #[serde(rename = "moduleName")]
    module_name: String,
    replacements: Vec<String>,
    url: Option<KnownUrl>,
}

fn fetch_manifest(url: &str) -> Result<ManifestModule> {
    let mut response = get(url)
        .call()
        .with_context(|| format!("Failed to fetch e18e data from {url}"))?;

    response
        .body_mut()
        .read_json()
        .with_context(|| format!("Failed to parse e18e JSON from {url}"))
}

fn mapping_entry_tokens(
    mapping_key: &str,
    mapping: &ModuleReplacementMapping,
) -> proc_macro2::TokenStream {
    let mapping_key = Literal::string(mapping_key);
    let mapping_type = Literal::string(&mapping.mapping_type);
    let module_name = Literal::string(&mapping.module_name);
    let replacements = mapping
        .replacements
        .iter()
        .map(|replacement| Literal::string(replacement));
    let url = option_known_url_tokens(mapping.url.as_ref());

    quote! {
        #mapping_key => ModuleReplacementMapping {
            mapping_type: #mapping_type,
            module_name: #module_name,
            replacements: &[#(#replacements),*],
            url: #url,
        }
    }
}

fn replacement_entry_tokens(
    replacement_key: &str,
    replacement: &ModuleReplacement,
) -> proc_macro2::TokenStream {
    let replacement_key = Literal::string(replacement_key);
    let replacement = module_replacement_tokens(replacement);

    quote! {
        #replacement_key => #replacement
    }
}

fn generate_code(manifests: BTreeMap<&str, ManifestModule>) -> proc_macro2::TokenStream {
    let mut mappings: BTreeMap<String, &ModuleReplacementMapping> = BTreeMap::new();
    let mut replacements: BTreeMap<String, &ModuleReplacement> = BTreeMap::new();

    for manifest in manifests.values() {
        for (mapping_key, mapping) in &manifest.mappings {
            if mappings.insert(mapping_key.clone(), mapping).is_some() {
                println!("Duplicate mapping id across manifests: {mapping_key}");
            }
        }

        for (replacement_key, replacement) in &manifest.replacements {
            if replacements
                .insert(replacement_key.clone(), replacement)
                .is_some()
            {
                println!("Duplicate replacement id across manifests: {replacement_key}");
            }
        }
    }

    let mappings = mappings
        .iter()
        .map(|(mapping_key, mapping)| mapping_entry_tokens(mapping_key, mapping));

    let replacements = replacements.iter().map(|(replacement_key, replacement)| {
        replacement_entry_tokens(replacement_key, replacement)
    });

    quote! {
        #[derive(Debug, Clone, Copy)]
        pub enum KnownUrlType {
            Mdn,
            Node,
            E18e,
        }

        #[derive(Debug, Clone, Copy)]
        pub enum KnownUrl {
            Descriptor {
                url_type: KnownUrlType,
                id: &'static str,
            },
            Raw(&'static str),
        }

        #[derive(Debug)]
        pub struct EngineConstraint {
            pub engine: &'static str,
            pub min_version: Option<&'static str>,
            pub max_version: Option<&'static str>,
        }

        #[derive(Debug)]
        pub struct ModuleReplacementLike {
            pub id: &'static str,
            pub engines: &'static [EngineConstraint],
            pub preferred: bool,
        }

        #[derive(Debug)]
        pub struct DocumentedModuleReplacement {
            pub common: ModuleReplacementLike,
            pub replacement_module: &'static str,
            pub url: Option<KnownUrl>,
        }

        #[derive(Debug)]
        pub struct NativeWebFeatureId {
            pub feature_id: &'static str,
            pub compat_key: &'static str,
        }

        #[derive(Debug)]
        pub struct NativeNodeFeatureId {
            pub module_name: &'static str,
            pub export_name: Option<&'static str>,
        }

        #[derive(Debug)]
        pub struct NativeModuleReplacement {
            pub common: ModuleReplacementLike,
            pub url: KnownUrl,
            pub description: Option<&'static str>,
            pub web_feature_id: Option<NativeWebFeatureId>,
            pub node_feature_id: Option<NativeNodeFeatureId>,
        }

        #[derive(Debug)]
        pub struct SimpleModuleReplacement {
            pub common: ModuleReplacementLike,
            pub description: &'static str,
            pub example: Option<&'static str>,
            pub url: Option<KnownUrl>,
        }

        #[derive(Debug)]
        pub struct RemovalReplacement {
            pub common: ModuleReplacementLike,
            pub description: &'static str,
            pub url: Option<KnownUrl>,
        }

        #[derive(Debug)]
        pub enum ModuleReplacement {
            Documented(DocumentedModuleReplacement),
            Native(NativeModuleReplacement),
            Simple(SimpleModuleReplacement),
            Removal(RemovalReplacement),
        }

        #[derive(Debug)]
        pub struct ModuleReplacementMapping {
            pub mapping_type: &'static str,
            pub module_name: &'static str,
            pub replacements: &'static [&'static str],
            pub url: Option<KnownUrl>,
        }

        pub static MODULE_REPLACEMENTS_MAPPINGS: phf::Map<&'static str, ModuleReplacementMapping> = phf::phf_map! {
            #( #mappings ),*
        };

        pub static MODULE_REPLACEMENTS: phf::Map<&'static str, ModuleReplacement> = phf::phf_map! {
            #( #replacements ),*
        };
    }
}

fn known_url_type_tokens(url_type: &KnownUrlType) -> proc_macro2::TokenStream {
    match url_type {
        KnownUrlType::Mdn => quote! { KnownUrlType::Mdn },
        KnownUrlType::Node => quote! { KnownUrlType::Node },
        KnownUrlType::E18e => quote! { KnownUrlType::E18e },
    }
}

fn known_url_tokens(url: &KnownUrl) -> proc_macro2::TokenStream {
    match url {
        KnownUrl::Descriptor(descriptor) => {
            let url_type = known_url_type_tokens(&descriptor.url_type);
            let id = Literal::string(&descriptor.id);
            quote! {
                KnownUrl::Descriptor {
                    url_type: #url_type,
                    id: #id,
                }
            }
        }
        KnownUrl::Raw(raw) => {
            let raw = Literal::string(raw);
            quote! { KnownUrl::Raw(#raw) }
        }
    }
}

fn option_known_url_tokens(url: Option<&KnownUrl>) -> proc_macro2::TokenStream {
    match url {
        Some(url) => {
            let url = known_url_tokens(url);
            quote! { Some(#url) }
        }
        None => quote! { None },
    }
}

fn option_string_tokens(value: Option<&str>) -> proc_macro2::TokenStream {
    match value {
        Some(value) => {
            let value = Literal::string(value);
            quote! { Some(#value) }
        }
        None => quote! { None },
    }
}

fn module_replacement_like_tokens(common: &ModuleReplacementLike) -> proc_macro2::TokenStream {
    let id = Literal::string(&common.id);
    let engines = common.engines.iter().flatten().map(|engine| {
        let engine_name = Literal::string(&engine.engine);
        let min_version = option_string_tokens(engine.min_version.as_deref());
        let max_version = option_string_tokens(engine.max_version.as_deref());
        quote! {
            EngineConstraint {
                engine: #engine_name,
                min_version: #min_version,
                max_version: #max_version,
            }
        }
    });
    let preferred = common.preferred.unwrap_or(false);

    quote! {
        ModuleReplacementLike {
            id: #id,
            engines: &[#(#engines),*],
            preferred: #preferred,
        }
    }
}

fn module_replacement_tokens(replacement: &ModuleReplacement) -> proc_macro2::TokenStream {
    match replacement {
        ModuleReplacement::Documented(documented) => {
            let common = module_replacement_like_tokens(&documented.common);
            let replacement_module = Literal::string(&documented.replacement_module);
            let url = option_known_url_tokens(documented.url.as_ref());

            quote! {
                ModuleReplacement::Documented(DocumentedModuleReplacement {
                    common: #common,
                    replacement_module: #replacement_module,
                    url: #url,
                })
            }
        }
        ModuleReplacement::Native(native) => {
            let common = module_replacement_like_tokens(&native.common);
            let url = known_url_tokens(&native.url);
            let description = option_string_tokens(native.description.as_deref());
            let web_feature_id = match native.web_feature_id.as_ref() {
                Some(web_feature_id) => {
                    let feature_id = Literal::string(&web_feature_id.feature_id);
                    let compat_key = Literal::string(&web_feature_id.compat_key);
                    quote! {
                        Some(NativeWebFeatureId {
                            feature_id: #feature_id,
                            compat_key: #compat_key,
                        })
                    }
                }
                None => quote! { None },
            };
            let node_feature_id = match native.node_feature_id.as_ref() {
                Some(node_feature_id) => {
                    let module_name = Literal::string(&node_feature_id.module_name);
                    let export_name = option_string_tokens(node_feature_id.export_name.as_deref());
                    quote! {
                        Some(NativeNodeFeatureId {
                            module_name: #module_name,
                            export_name: #export_name,
                        })
                    }
                }
                None => quote! { None },
            };

            quote! {
                ModuleReplacement::Native(NativeModuleReplacement {
                    common: #common,
                    url: #url,
                    description: #description,
                    web_feature_id: #web_feature_id,
                    node_feature_id: #node_feature_id,
                })
            }
        }
        ModuleReplacement::Simple(simple) => {
            let common = module_replacement_like_tokens(&simple.common);
            let description = Literal::string(&simple.description);
            let example = option_string_tokens(simple.example.as_deref());
            let url = option_known_url_tokens(simple.url.as_ref());

            quote! {
                ModuleReplacement::Simple(SimpleModuleReplacement {
                    common: #common,
                    description: #description,
                    example: #example,
                    url: #url,
                })
            }
        }
        ModuleReplacement::Removal(removal) => {
            let common = module_replacement_like_tokens(&removal.common);
            let description = Literal::string(&removal.description);
            let url = option_known_url_tokens(removal.url.as_ref());

            quote! {
                ModuleReplacement::Removal(RemovalReplacement {
                    common: #common,
                    description: #description,
                    url: #url,
                })
            }
        }
    }
}

pub fn generate_module_replacements(mode: Mode) -> Result<()> {
    println!("Fetching e18e data");

    let manifests = [
        ("native", E18E_NATIVE_DATA_URL),
        ("micro-utilities", E18E_MICRO_UTILITIES_DATA_URL),
        ("preferred", E18E_PREFERRED_DATA_URL),
    ];

    let mut manifests_data: BTreeMap<&str, ManifestModule> = BTreeMap::new();

    for (name, url) in manifests {
        let manifest = fetch_manifest(url)?;
        println!(
            "Loaded {name} manifest: {} mappings, {} replacements",
            manifest.mappings.len(),
            manifest.replacements.len()
        );
        manifests_data.insert(name, manifest);
    }

    let tokens = generate_code(manifests_data);
    let output_path = project_root().join("crates/biome_module_replacements/src/generated/data.rs");

    update(
        output_path.as_path(),
        &xtask_glue::reformat(tokens.to_string())?,
        &mode,
    )?;

    Ok(())
}
