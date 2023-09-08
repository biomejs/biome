//! Metadata of:
//! - ARIA properties
//! - ARIA property types
//! - ARIA roles

use case::CaseExt;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::path::PathBuf;
use std::{env, fs, io};

pub const ARIA_PROPERTIES: [&str; 48] = [
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-checked",
    "aria-colcount",
    "aria-colindex",
    "aria-colspan",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-expanded",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-modal",
    "aria-multiline",
    "aria-multiselectable",
    "aria-orientation",
    "aria-owns",
    "aria-placeholder",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-roledescription",
    "aria-rowcount",
    "aria-rowindex",
    "aria-rowspan",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "aria-valuetext",
];

pub const ARIA_PROPERTY_TYPE: [&str; 9] = [
    "boolean",
    "id",
    "idlist",
    "integer",
    "number",
    "string",
    "token",
    "tokenlist",
    "tristate",
];

pub const ARIA_WIDGET_ROLES: [&str; 27] = [
    "alert",
    "alertdialog",
    "button",
    "checkbox",
    "dialog",
    "gridcell",
    "link",
    "log",
    "marquee",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "option",
    "progressbar",
    "radio",
    "scrollbar",
    "searchbox",
    "slider",
    "spinbutton",
    "status",
    "switch",
    "tab",
    "tabpanel",
    "textbox",
    "timer",
    "tooltip",
    "treeitem",
];

pub const ARIA_ABSTRACT_ROLES: [&str; 12] = [
    "command",
    "composite",
    "input",
    "landmark",
    "range",
    "roletype",
    "section",
    "sectionhead",
    "select",
    "structure",
    "widget",
    "window",
];

pub const ARIA_DOCUMENT_STRUCTURE_ROLES: [&str; 25] = [
    "article",
    "cell",
    "columnheader",
    "definition",
    "directory",
    "document",
    "feed",
    "figure",
    "group",
    "heading",
    "img",
    "list",
    "listitem",
    "math",
    "none",
    "note",
    "presentation",
    "region",
    "row",
    "rowgroup",
    "rowheader",
    "separator",
    "table",
    "term",
    "toolbar",
];

const ISO_COUNTRIES: [&str; 233] = [
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

const ISO_LANGUAGES: [&str; 150] = [
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

fn main() -> io::Result<()> {
    let aria_properties = generate_properties();
    let aria_roles = generate_roles();
    let tokens = quote! {
        use std::str::FromStr;


        #aria_properties
        #aria_roles
    };
    let ast = tokens.to_string();

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(PathBuf::from(out_dir).join("roles_and_properties.rs"), ast)?;

    Ok(())
}

fn generate_properties() -> TokenStream {
    let properties = generate_enums(
        ARIA_PROPERTIES.len(),
        ARIA_PROPERTIES.iter(),
        "AriaPropertiesEnum",
    );

    let property_types = generate_enums(
        ARIA_PROPERTY_TYPE.len(),
        ARIA_PROPERTY_TYPE.iter(),
        "AriaPropertyTypeEnum",
    );

    quote! {
        #properties
        #property_types
    }
}

fn generate_roles() -> TokenStream {
    let widget_roles = generate_enums(
        ARIA_WIDGET_ROLES.len(),
        ARIA_WIDGET_ROLES.iter(),
        "AriaWidgetRolesEnum",
    );
    let abstract_roles = generate_enums(
        ARIA_ABSTRACT_ROLES.len(),
        ARIA_ABSTRACT_ROLES.iter(),
        "AriaAbstractRolesEnum",
    );

    let document_structure_roles = generate_enums(
        ARIA_DOCUMENT_STRUCTURE_ROLES.len(),
        ARIA_DOCUMENT_STRUCTURE_ROLES.iter(),
        "AriaDocumentStructureRolesEnum",
    );

    let iso_countries = generate_enums(ISO_COUNTRIES.len(), ISO_COUNTRIES.iter(), "IsoCountries");

    let iso_languages = generate_enums(ISO_LANGUAGES.len(), ISO_LANGUAGES.iter(), "IsoLanguages");

    quote! {
        #widget_roles
        #abstract_roles
        #document_structure_roles
        #iso_countries
        #iso_languages
    }
}

fn generate_enums(len: usize, array: std::slice::Iter<&str>, enum_name: &str) -> TokenStream {
    let enum_name = Ident::new(enum_name, Span::call_site());
    let mut enum_metadata = Vec::with_capacity(len);
    let mut from_enum_metadata = Vec::with_capacity(len);
    let mut from_string_metadata = Vec::with_capacity(len);
    for property in array {
        let name = Ident::new(&property.replace('-', "_").to_camel(), Span::call_site());
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

        #[derive(Debug, Eq, PartialEq)]
        pub enum #enum_name {
            #( #enum_metadata ),*
        }

        impl From<#enum_name> for &str {
            fn from(property: #enum_name) -> Self {
                match property {
                    #( #from_enum_metadata ),*
                }
            }
        }

        impl FromStr for #enum_name {
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
