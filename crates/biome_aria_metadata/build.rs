//! Generate ARAI metadata from `./aria-data.json`

use biome_string_case::Case;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::{env, fs, io};

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

#[derive(Debug, Default, serde::Deserialize)]
struct Aria {
    roles: BTreeMap<String, AriaRole>,
    attributes: BTreeMap<String, AriaAttribute>,
}

impl Aria {
    fn inherits_role(&self, role_name: &str, candidate_role_name: &str) -> Result<bool, String> {
        if !self.roles.contains_key(candidate_role_name) {
            return Err(format!("The role '{candidate_role_name}' doesn't exist"));
        }
        let mut stack = vec![role_name];
        while let Some(role_name) = stack.pop() {
            let Some(role) = self.roles.get(role_name) else {
                return Err(format!("The role '{role_name}' doesn't exist"));
            };
            if role.superclass_roles.contains(candidate_role_name) {
                return Ok(true);
            }
            stack.extend(role.superclass_roles.iter().map(String::as_str));
        }
        Ok(false)
    }
}

#[derive(Debug, Default, serde::Deserialize)]
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

#[derive(Debug, Default, serde::Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
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

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum AriaAttributeType {
    #[default]
    Property,
    State,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ValueDefinition {
    description: String,
    is_default: bool,
}

fn main() -> io::Result<()> {
    // CARGO instructions: rern if one of these files change
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=aria-data.json");

    let text = std::fs::read_to_string("aria-data.json")?;
    let data: Aria = serde_json::from_str(&text)?;

    let aria_attribute_names: Vec<_> = data.attributes.keys().map(|name| name.as_str()).collect();
    let abstract_aria_roles: BTreeMap<_, _> = data
        .roles
        .iter()
        .filter(|(_, role)| role.is_abstract)
        .collect();
    let abstract_aria_role_names: Vec<_> = abstract_aria_roles
        .keys()
        .map(|name| name.as_str())
        .collect();
    let structure_aria_roles: Vec<_> = data
        .roles
        .keys()
        .filter(|name| data.inherits_role(name, "structure").is_ok_and(|b| b))
        .map(|name| name.as_str())
        .collect();
    let widget_aria_roles: Vec<_> = data
        .roles
        .keys()
        .filter(|name| data.inherits_role(name, "widget").is_ok_and(|b| b))
        .map(|name| name.as_str())
        .collect();

    let aria_properties = generate_enums(&aria_attribute_names[..], "AriaPropertiesEnum");

    let abstract_roles = generate_enums(&abstract_aria_role_names[..], "AriaAbstractRolesEnum");
    let structure_roles =
        generate_enums(&structure_aria_roles[..], "AriaDocumentStructureRolesEnum");
    let widget_roles = generate_enums(&widget_aria_roles[..], "AriaWidgetRolesEnum");

    let iso_countries = generate_enums(ISO_COUNTRIES, "IsoCountries");
    let iso_languages = generate_enums(ISO_LANGUAGES, "IsoLanguages");

    let tokens = quote! {
        #aria_properties
        #abstract_roles
        #structure_roles
        #widget_roles
        #iso_countries
        #iso_languages
    };
    let ast = tokens.to_string();

    // Format code
    let ast = syn::parse_file(&ast).unwrap();
    let ast = prettyplease::unparse(&ast);

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(PathBuf::from(out_dir).join("roles_and_properties.rs"), ast)?;

    Ok(())
}

fn generate_enums(array: &[&str], enum_name: &str) -> TokenStream {
    let enum_name = Ident::new(enum_name, Span::call_site());
    let mut enum_metadata = Vec::with_capacity(array.len());
    let mut from_enum_metadata = Vec::with_capacity(array.len());
    let mut from_string_metadata = Vec::with_capacity(array.len());
    for property in array {
        let name = Ident::new(&Case::Pascal.convert(property), Span::call_site());
        let property = Literal::string(property);
        from_enum_metadata.push(quote! {
            #enum_name::#name => #property
        });
        from_string_metadata.push(quote! {
            #property => Ok(#enum_name::#name)
        });
        enum_metadata.push(name);
    }

    from_string_metadata.push(quote! {
        _ => Err("aria property not implemented".to_string())
    });

    quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum #enum_name {
            #( #enum_metadata ),*
        }

        impl std::str::FromStr for #enum_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #from_string_metadata ),*
                }
            }
        }

        impl #enum_name {
            pub fn as_str(&self) -> &str {
                match self {
                    #( #from_enum_metadata ),*
                }
            }
        }
    }
}
