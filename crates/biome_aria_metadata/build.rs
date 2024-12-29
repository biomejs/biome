//! Generate ARAI metadata from `./aria-data.json`

use biome_deserialize::Merge;
use biome_string_case::Case;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::{env, fs, io};

const WAI_ARIA: &str = "../../packages/aria-data/wai-aria-1-3.json";
const GRAPHICS_ARIA: &str = "../../packages/aria-data/graphics-aria-1-0.json";
const DPUB_ARIA: &str = "../../packages/aria-data/dpub-aria-1-1.json";

const ISO_COUNTRIES: &[&str] = &[
    "AF", "AL", "DZ", "AS", "AD", "AO", "AI", "AQ", "AG", "AR", "AM", "AW", "AU", "AT", "AZ", "BS",
    "BH", "BD", "BB", "BY", "BE", "BZ", "BJ", "BM", "BT", "BO", "BA", "BW", "BR", "IO", "VG", "BN",
    "BG", "BF", "MM", "BI", "KH", "CM", "CA", "CV", "KY", "CF", "TD", "CL", "CN", "CX", "CC", "CO",
    "KM", "CK", "CR", "HR", "CU", "CY", "CZ", "CD", "DK", "DJ", "DM", "DO", "EC", "EG", "SV", "GQ",
    "ER", "EE", "ET", "FK", "FO", "FJ", "FI", "FR", "PF", "GA", "GM", "GE", "DE", "GH", "GI", "GR",
    "GL", "GD", "GU", "GT", "GN", "GW", "GY", "HT", "VA", "HN", "HK", "HU", "IS", "IN", "ID", "IR",
    "IQ", "IE", "IM", "IL", "IT", "CI", "JM", "JP", "JE", "JO", "KZ", "KE", "KI", "KW", "KG", "LA",
    "LV", "LB", "LS", "LR", "LY", "LI", "LT", "LU", "MO", "MK", "MG", "MW", "MY", "MV", "ML", "MT",
    "MH", "MR", "MU", "YT", "MX", "FM", "MD", "MC", "MN", "ME", "MS", "MA", "MZ", "NA", "NR", "NP",
    "NL", "AN", "NC", "NZ", "NI", "NE", "NG", "NU", "KP", "MP", "NO", "OM", "PK", "PW", "PA", "PG",
    "PY", "PE", "PH", "PN", "PL", "PT", "PR", "QA", "CG", "RO", "RU", "RW", "BL", "SH", "KN", "LC",
    "MF", "PM", "VC", "WS", "SM", "ST", "SA", "SN", "RS", "SC", "SL", "SG", "SK", "SI", "SB", "SO",
    "ZA", "KR", "ES", "LK", "SD", "SR", "SJ", "SZ", "SE", "CH", "SY", "TW", "TJ", "TZ", "TH", "TL",
    "TG", "TK", "TO", "TT", "TN", "TR", "TM", "TC", "TV", "UG", "UA", "AE", "GB", "US", "UY", "VI",
    "UZ", "VU", "VE", "VN", "WF", "EH", "YE", "ZM", "ZW",
];

const ISO_LANGUAGES: &[&str] = &[
    "ab", "aa", "af", "sq", "am", "ar", "an", "hy", "as", "ay", "az", "ba", "eu", "bn", "dz", "bh",
    "bi", "br", "bg", "my", "be", "km", "ca", "zh", "zh-Hans", "zh-Hant", "co", "hr", "cs", "da",
    "nl", "en", "eo", "et", "fo", "fa", "fj", "fi", "fr", "fy", "gl", "gd", "gv", "ka", "de", "el",
    "kl", "gn", "gu", "ht", "ha", "he", "iw", "hi", "hu", "is", "io", "id", "in", "ia", "ie", "iu",
    "ik", "ga", "it", "ja", "jv", "kn", "ks", "kk", "rw", "ky", "rn", "ko", "ku", "lo", "la", "lv",
    "li", "ln", "lt", "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mo", "mn", "na", "ne", "no", "oc",
    "or", "om", "ps", "pl", "pt", "pa", "qu", "rm", "ro", "ru", "sm", "sg", "sa", "sr", "sh", "st",
    "tn", "sn", "ii", "sd", "si", "ss", "sk", "sl", "so", "es", "su", "sw", "sv", "tl", "tg", "ta",
    "tt", "te", "th", "bo", "ti", "to", "ts", "tr", "tk", "tw", "ug", "uk", "ur", "uz", "vi", "vo",
    "wa", "cy", "wo", "xh", "yi", "ji", "yo", "zu",
];

#[derive(Debug, Default, biome_deserialize_macros::Merge, serde::Deserialize)]
struct Aria {
    roles: BTreeMap<String, AriaRole>,
    attributes: BTreeMap<String, AriaAttribute>,
}
impl Aria {
    /// Retuurns direct and indirect superclass roles.
    fn superclass_roles(&self, role_name: &str) -> Result<BTreeSet<String>, String> {
        let mut result = BTreeSet::new();
        let mut stack = vec![role_name];
        while let Some(role_name) = stack.pop() {
            let Some(role) = self.roles.get(role_name) else {
                return Err(format!("The role '{role_name}' doesn't exist"));
            };
            stack.extend(role.superclass_roles.iter().map(String::as_str));
            result.extend(role.superclass_roles.iter().map(Clone::clone));
        }
        Ok(result)
    }
}

#[derive(Debug, Default, biome_deserialize_macros::Merge, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct AriaRole {
    description: String,
    deprecated_in_version: Option<String>,
    is_abstract: bool,
    superclass_roles: BTreeSet<String>,
    subclass_roles: BTreeSet<String>,
    base_concepts: BTreeSet<Concept>,
    related_concepts: BTreeSet<Concept>,
    allowed_child_roles: BTreeSet<String>,
    required_parent_roles: BTreeSet<String>,
    required_attributes: BTreeSet<AriaAttributeReference>,
    supported_attributes: BTreeSet<AriaAttributeReference>,
    inherited_attributes: BTreeSet<AriaAttributeReference>,
    prohibited_attributes: BTreeSet<AriaAttributeReference>,
    name_from: BTreeSet<AriaNameFrom>,
    is_accessible_name_required: bool,
    has_presentational_children: bool,
    implicit_values_for_role: BTreeMap<String, String>,
}
impl AriaRole {
    fn all_attributes(&self) -> BTreeSet<&AriaAttributeReference> {
        self.supported_attributes
            .iter()
            .chain(self.required_attributes.iter())
            .chain(self.inherited_attributes.iter())
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum AriaNameFrom {
    Author,
    Contents,
    Prohibited,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Concept {
    /// An attribute or an element
    Any {
        name: String,
        module: ConceptModule,
    },
    Attribute {
        name: String,
        module: ConceptModule,
    },
    Element {
        name: String,
        #[serde(default)]
        attributes: BTreeMap<String, String>,
        module: ConceptModule,
    },
    Role {
        name: String,
        module: ConceptModule,
    },
    Text {
        name: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum ConceptModule {
    Aria,
    Daisy,
    Dom,
    #[serde(rename = "dublin core")]
    DublinCore,
    Html,
    Japi,
    Smil,
    Svg,
    Xforms,
    Xhtml,
}
impl ConceptModule {
    const fn is_html_like(self) -> bool {
        matches!(self, Self::Dom | Self::Html | Self::Xhtml)
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(from = "AriaAttributeReferenceShortcut")]
struct AriaAttributeReference {
    name: String,
    deprecated_in_version: Option<String>,
}

impl From<AriaAttributeReferenceShortcut> for AriaAttributeReference {
    fn from(value: AriaAttributeReferenceShortcut) -> Self {
        match value {
            AriaAttributeReferenceShortcut::Active(name) => Self {
                name,
                deprecated_in_version: None,
            },
            AriaAttributeReferenceShortcut::Deprecated {
                name,
                deprecated_in_version,
            } => Self {
                name,
                deprecated_in_version,
            },
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(untagged, rename_all_fields = "camelCase")]
enum AriaAttributeReferenceShortcut {
    Active(String),
    Deprecated {
        name: String,
        deprecated_in_version: Option<String>,
    },
}

#[derive(Debug, Default, biome_deserialize_macros::Merge, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct AriaAttribute {
    r#type: AriaAttributeType,
    description: String,
    deprecated_in_version: Option<String>,
    related_concepts: BTreeSet<Concept>,
    used_in_roles: BTreeSet<String>,
    inherits_into_roles: BTreeSet<String>,
    value_type: AriaValueType,
    values: BTreeMap<String, ValueDefinition>,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, biome_deserialize_macros::Merge, serde::Deserialize,
)]
enum AriaValueType {
    #[default]
    #[serde(rename = "true/false")]
    Boolean,
    #[serde(rename = "ID reference")]
    IdReference,
    #[serde(rename = "ID reference list")]
    IdReferenceList,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "true/false/undefined")]
    OptionalBoolean,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "token list")]
    TokenList,
    #[serde(rename = "tristate")]
    Tristate,
}

#[derive(Debug, Default, biome_deserialize_macros::Merge, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum AriaAttributeType {
    #[default]
    Property,
    State,
}

#[derive(Debug, Default, biome_deserialize_macros::Merge, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueDefinition {
    description: String,
    is_default: bool,
}

fn main() -> io::Result<()> {
    // CARGO instructions: rern if one of these files change
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={WAI_ARIA}");
    println!("cargo::rerun-if-changed={GRAPHICS_ARIA}");
    println!("cargo::rerun-if-changed={DPUB_ARIA}");

    let text = std::fs::read_to_string(WAI_ARIA)?;
    let wair_aria: Aria = serde_json::from_str(&text)?;

    let text = std::fs::read_to_string(DPUB_ARIA)?;
    let dpub_aria: Aria = serde_json::from_str(&text)?;

    let text = std::fs::read_to_string(GRAPHICS_ARIA)?;
    let graphics_aria: Aria = serde_json::from_str(&text)?;

    let mut aria = graphics_aria;
    aria.merge_with(dpub_aria);
    aria.merge_with(wair_aria);

    let aria_attributes = generate_aria_attributes(&aria.attributes);
    let aria_roles = generate_aria_roles(&aria);

    let iso_countries = generate_enums(ISO_COUNTRIES, "IsoCountries");
    let iso_languages = generate_enums(ISO_LANGUAGES, "IsoLanguages");

    let tokens = quote! {
        #aria_attributes
        #aria_roles
        #iso_countries
        #iso_languages
    };
    let ast = tokens.to_string();

    // Try to parse and then format code
    let ast = if let Ok(parsed) = syn::parse_file(&ast) {
        prettyplease::unparse(&parsed)
    } else {
        ast
    };

    // We print the code even if it cannot be parsed,
    // this allows to debug the code by directly looking at it.
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(PathBuf::from(out_dir).join("roles_and_attributes.rs"), ast)?;

    Ok(())
}

fn generate_enums(
    array: impl IntoIterator<Item = impl AsRef<str>>,
    enum_name: &str,
) -> TokenStream {
    let iter = array.into_iter();
    let enum_name = Ident::new(enum_name, Span::call_site());
    let mut enum_metadata = Vec::with_capacity(iter.size_hint().0);
    let mut from_enum_metadata = Vec::with_capacity(iter.size_hint().0);
    let mut from_string_metadata = Vec::with_capacity(iter.size_hint().0);
    for property in iter {
        let property = property.as_ref();
        let name = Ident::new(&Case::Pascal.convert(property), Span::call_site());
        let property = Literal::string(property);
        from_enum_metadata.push(quote! {
            Self::#name => #property
        });
        from_string_metadata.push(quote! {
            #property => Ok(Self::#name)
        });
        enum_metadata.push(name);
    }

    from_string_metadata.push(quote! {
        _ => Err(())
    });

    quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum #enum_name {
            #( #enum_metadata ),*
        }
        impl #enum_name {
            pub const fn as_str(self) -> &'static str {
                match self {
                    #( #from_enum_metadata ),*
                }
            }
        }
        impl std::str::FromStr for #enum_name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #from_string_metadata ),*
                }
            }
        }
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str(self.as_str())
            }
        }
    }
}

fn generate_aria_attributes(attributes: &BTreeMap<String, AriaAttribute>) -> TokenStream {
    let aria_attribute_enum = generate_enums(attributes.keys(), "AriaAttribute");
    let mut deprecated_variants = Vec::new();
    let mut kind_match_lines = Vec::with_capacity(attributes.len());
    let mut value_type_match_lines = Vec::with_capacity(attributes.len());
    for (name, data) in attributes {
        let name = Ident::new(&Case::Pascal.convert(name), Span::call_site());
        if data.deprecated_in_version.is_some() {
            deprecated_variants.push(name.clone());
        }
        let kind_variant_name = match data.r#type {
            AriaAttributeType::Property => quote! { Property },
            AriaAttributeType::State => quote! { State },
        };
        kind_match_lines.push(quote! {
            Self::#name => AriaAttributeKind::#kind_variant_name
        });
        let value_type_variant_name = match data.value_type {
            AriaValueType::Boolean => quote! { Boolean },
            AriaValueType::IdReference => quote! { IdReference },
            AriaValueType::IdReferenceList => quote! { IdReferenceList },
            AriaValueType::Integer => quote! { Integer },
            AriaValueType::Number => quote! { Number },
            AriaValueType::OptionalBoolean => quote! { OptionalBoolean },
            AriaValueType::String => quote! { String },
            AriaValueType::Token => quote! { Token },
            AriaValueType::TokenList => quote! { TokenList },
            AriaValueType::Tristate => quote! { Tristate },
        };
        let params = if !data.values.is_empty()
            && matches!(
                data.value_type,
                AriaValueType::Token | AriaValueType::TokenList
            ) {
            let default_value = data
                .values
                .iter()
                .find(|(_, data)| data.is_default)
                .map(|(name, _)| name);
            let other_values = data
                .values
                .iter()
                .filter(|(_, data)| !data.is_default)
                .map(|(name, _)| name);
            let values = default_value.into_iter().chain(other_values);
            quote! {
                (&[
                    #( #values ),*
                ])
            }
        } else {
            Default::default()
        };
        value_type_match_lines.push(quote! {
            Self::#name => AriaValueType::#value_type_variant_name #params
        });
    }
    quote! {
        #aria_attribute_enum
        impl AriaAttribute {
            pub fn is_deprecated(&self) -> bool {
                matches!(
                    self,
                    #( Self::#deprecated_variants )|*
                )
            }
            pub fn kind(self) -> AriaAttributeKind {
                match self {
                    #( #kind_match_lines ),*,
                }
            }
            pub fn value_type(&self) -> AriaValueType {
                match self {
                    #( #value_type_match_lines ),*,
                }
            }
        }
    }
}

fn generate_aria_roles(aria: &Aria) -> TokenStream {
    let aria_abstract_role_names = aria
        .roles
        .iter()
        .filter(|(_, data)| data.is_abstract)
        .map(|(name, _)| name);
    let aria_abstarct_role_enum = generate_enums(aria_abstract_role_names, "AriaAbstractRole");
    let aria_concrete_role_names = aria
        .roles
        .iter()
        .filter(|(_, data)| !data.is_abstract)
        .map(|(name, _)| name);
    let aria_concrete_role_enum = generate_enums(aria_concrete_role_names, "AriaRole");
    let mut html_element_names = BTreeSet::new();
    let mut html_attributes_names = BTreeSet::new();
    let mut deprecated_variants = Vec::new();
    let mut base_concepts_match_lines = Vec::new();
    let mut related_concepts_match_lines = Vec::new();
    let mut inherited_abstract_role_variants = Vec::new();
    let mut inherited_concrete_role_variants = Vec::new();
    let mut required_parent_roles_match_lines = Vec::new();
    let mut required_attributes_match_lines = Vec::new();
    let mut prohibited_attributes_match_lines = Vec::new();
    let mut attributes_match_lines = Vec::with_capacity(aria.roles.len());
    let mut global_attribute_variants = Vec::new();
    let mut variants = Vec::new();
    for (name, data) in &aria.roles {
        if data.is_abstract {
            if name == "roletype" {
                variants.clear();
                for global_attribute in &data.supported_attributes {
                    let variant = Ident::new(
                        &Case::Pascal.convert(&global_attribute.name),
                        Span::call_site(),
                    );
                    global_attribute_variants.push(variant);
                }
            }
            continue;
        }
        let superclass_roles = aria.superclass_roles(name).expect("All roles exist");
        let name = Ident::new(&Case::Pascal.convert(name), Span::call_site());
        variants.clear();
        for name in &superclass_roles {
            let role_data = aria
                .roles
                .get(name)
                .unwrap_or_else(|| panic!("The role '{name}' doesn't exist."));
            if role_data.is_abstract {
                let variant_name = Ident::new(&Case::Pascal.convert(name), Span::call_site());
                variants.push(variant_name);
            }
        }
        if !variants.is_empty() {
            inherited_abstract_role_variants.push(quote! {
                Self::#name => AriaAbstractRoles(&[
                    #( AriaAbstractRole::#variants ),*
                ])
            });
        }
        variants.clear();
        for name in &superclass_roles {
            let role_data = aria
                .roles
                .get(name)
                .unwrap_or_else(|| panic!("The role '{name}' doesn't exist."));
            if !role_data.is_abstract {
                let variant_name = Ident::new(&Case::Pascal.convert(name), Span::call_site());
                variants.push(variant_name);
            }
        }
        if !variants.is_empty() {
            inherited_concrete_role_variants.push(quote! {
                Self::#name => AriaRoles(&[
                    #( AriaRole::#variants ),*
                ])
            });
        }
        if data.deprecated_in_version.is_some() {
            deprecated_variants.push(name.clone());
        }
        let mut element_instances = Vec::new();
        for concept in &data.base_concepts {
            if let Concept::Element {
                name: elt_name,
                attributes,
                module,
            } = concept
            {
                if module.is_html_like() {
                    html_element_names.insert(elt_name.as_str());
                    variants.clear();
                    let mut attribute_instances = Vec::new();
                    for (attribute_name, value) in attributes {
                        html_attributes_names.insert(attribute_name.as_str());
                        let attribute_name =
                            format_ident!("{}", Case::Pascal.convert(attribute_name));
                        attribute_instances.push(quote! {
                            HtmlAttributeInstance {
                                attribute: HtmlAttribute::#attribute_name,
                                value: #value,
                            }
                        });
                    }
                    let elt_name = format_ident!("{}", Case::Pascal.convert(elt_name));
                    element_instances.push(quote! {
                        HtmlElementInstance {
                            element: HtmlElement::#elt_name,
                            attributes: &[ #( #attribute_instances ),* ],
                        }
                    });
                }
            }
        }
        if !element_instances.is_empty() {
            base_concepts_match_lines.push(quote! {
                Self::#name => &[ #( #element_instances ),* ]
            });
        }
        element_instances.clear();
        for concept in &data.related_concepts {
            if let Concept::Element {
                name: elt_name,
                attributes,
                module,
            } = concept
            {
                if module.is_html_like() {
                    html_element_names.insert(elt_name.as_str());
                    variants.clear();
                    let mut attribute_instances = Vec::new();
                    for (attribute_name, value) in attributes {
                        html_attributes_names.insert(attribute_name.as_str());
                        let attribute_name =
                            format_ident!("{}", Case::Pascal.convert(attribute_name));
                        attribute_instances.push(quote! {
                            HtmlAttributeInstance {
                                attribute: HtmlAttribute::#attribute_name,
                                value: #value,
                            }
                        });
                    }
                    let elt_name = format_ident!("{}", Case::Pascal.convert(elt_name));
                    element_instances.push(quote! {
                        HtmlElementInstance {
                            element: HtmlElement::#elt_name,
                            attributes: &[ #( #attribute_instances ),* ],
                        }
                    });
                }
            }
        }
        if !element_instances.is_empty() {
            related_concepts_match_lines.push(quote! {
                Self::#name => &[ #( #element_instances ),* ]
            });
        }
        variants.clear();
        for name in &data.required_parent_roles {
            let variant_name = Ident::new(&Case::Pascal.convert(name), Span::call_site());
            variants.push(variant_name);
        }
        if !variants.is_empty() {
            required_parent_roles_match_lines.push(quote! {
                Self::#name => AriaRoles(&[
                    #( AriaRole::#variants ),*
                ])
            });
        }
        variants.clear();
        for attribute in &data.required_attributes {
            // Ignore deprecated attributes
            if attribute.deprecated_in_version.is_none() {
                let variant = Ident::new(&Case::Pascal.convert(&attribute.name), Span::call_site());
                variants.push(variant);
            }
        }
        if !variants.is_empty() {
            required_attributes_match_lines.push(quote! {
                Self::#name => AriaAttributes(&[
                    #( AriaAttribute::#variants ),*
                ])
            });
        }
        variants.clear();
        for attribute in &data.prohibited_attributes {
            let variant = Ident::new(&Case::Pascal.convert(&attribute.name), Span::call_site());
            variants.push(variant);
        }
        if !variants.is_empty() {
            prohibited_attributes_match_lines.push(quote! {
                Self::#name => AriaAttributes(&[
                    #( AriaAttribute::#variants ),*
                ])
            });
        }
        variants.clear();
        for attribute in data.all_attributes() {
            let variant = Ident::new(&Case::Pascal.convert(&attribute.name), Span::call_site());
            variants.push(variant);
        }
        if !variants.is_empty() {
            attributes_match_lines.push(quote! {
                Self::#name => AriaAttributes(&[
                    #( AriaAttribute::#variants ),*
                ])
            });
        }
    }
    let html_element_enum = generate_enums(&html_element_names, "HtmlElement");
    let html_attribute_enum = generate_enums(&html_attributes_names, "HtmlAttribute");
    quote! {
        impl AriaAttribute {
            pub fn is_global(self) -> bool {
                matches!(
                    self,
                    #( Self::#global_attribute_variants )|*
                )
            }
        }
        #html_element_enum
        #html_attribute_enum
        #aria_abstarct_role_enum
        #aria_concrete_role_enum
        impl AriaRole {
            pub fn is_deprecated(self) -> bool {
                matches!(
                    self,
                    #( Self::#deprecated_variants )|*
                )
            }
            pub const fn base_html_elements(self) -> &'static [HtmlElementInstance] {
                match self {
                    #( #base_concepts_match_lines, )*
                    _ => &[],
                }
            }
            pub const fn related_html_elements(self) -> &'static [HtmlElementInstance] {
                match self {
                    #( #related_concepts_match_lines, )*
                    _ => &[],
                }
            }
            pub const fn inherited_abstract_roles(self) -> AriaAbstractRoles {
                match self {
                    #( #inherited_abstract_role_variants, )*
                    _ => AriaAbstractRoles::empty(),
                }
            }
            pub const fn inherited_roles(self) -> AriaRoles {
                match self {
                    #( #inherited_concrete_role_variants, )*
                    _ => AriaRoles::empty(),
                }
            }
            pub const fn required_parent_roles(self) -> AriaRoles {
                match self {
                    #( #required_parent_roles_match_lines, )*
                    _ => AriaRoles::empty(),
                }
            }
            pub const fn required_attributes(self) -> AriaAttributes {
                match self {
                    #( #required_attributes_match_lines, )*
                    _ => AriaAttributes::empty(),
                }
            }
            pub const fn attributes(self) -> AriaAttributes {
                match self {
                    #( #attributes_match_lines, )*
                    _ => AriaAttributes::empty(),
                }
            }
            pub const fn prohibited_attributes(self) -> AriaAttributes {
                match self {
                    #( #prohibited_attributes_match_lines, )*
                    _ => AriaAttributes::empty(),
                }
            }
        }
    }
}
