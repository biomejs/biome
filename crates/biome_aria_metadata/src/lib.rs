include!(concat!(env!("OUT_DIR"), "/roles_and_attributes.rs"));

pub const ISO_COUNTRIES: [&str; 233] = [
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

pub const ISO_LANGUAGES: [&str; 150] = [
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AriaAttributeKind {
    Property,
    State,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AriaValueType {
    /// `false`/`true`
    Boolean,
    IdReference,
    IdReferenceList,
    Integer,
    Number,
    /// `undefined`/`false`/`true`
    OptionalBoolean,
    /// Non-empty string
    String,
    /// A token among the given slice of tokens
    Token(&'static [&'static str]),
    /// A list of tokens among the given slice of tokens
    TokenList(&'static [&'static str]),
    /// `false`/`true`/`mixed`
    Tristate,
}
impl AriaValueType {
    /// It checks if `value` is valid for the `self` type.
    ///
    /// [Source](https://www.w3.org/TR/wai-aria-1.2/#propcharacteristic_value)
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_aria_metadata::AriaAttribute;
    ///
    /// assert!(AriaAttribute::AriaCurrent.value_type().contains("true"));
    /// assert!(AriaAttribute::AriaCurrent.value_type().contains("step"));
    ///
    /// assert!(!AriaAttribute::AriaCurrent.value_type().contains("something_not_allowed"));
    /// ```
    pub fn contains(&self, value: &str) -> bool {
        if value.is_empty() {
            return false;
        }
        match self {
            Self::String => true,
            Self::IdReference => is_valid_html_id(value),
            Self::IdReferenceList => value.split_ascii_whitespace().all(is_valid_html_id),
            Self::Integer => value.parse::<u32>().is_ok(),
            Self::Number => value.parse::<f32>().is_ok(),
            Self::Boolean => matches!(value, "false" | "true"),
            Self::OptionalBoolean => matches!(value, "undefined" | "false" | "true"),
            Self::Token(tokens) => tokens.iter().any(|allowed_token| *allowed_token == value),
            Self::TokenList(tokens) => value.split_ascii_whitespace().all(|input_token| {
                tokens
                    .iter()
                    .any(|allowed_token| allowed_token.trim() == input_token)
            }),
            Self::Tristate => matches!(value, "false" | "true" | "mixed"),
        }
    }
}

/// Is `id` a valid HTML identifier?
///
/// Browsers allows arbitrary sequence of characters for identifiers.
/// However, it is commonly accepted that an identifier should not contain
/// characters recognized as whitespaces by HTML.
/// Whitespaces are usedd to separate two identifier in a list of identifiers.
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id#syntax
fn is_valid_html_id(id: &str) -> bool {
    !id.is_empty() && !id.bytes().any(|b| b.is_ascii_whitespace())
}

impl AriaRole {
    /// Returns `true` if the given role inherits of `AriaAbstractRole::Widget`.
    ///
    /// This corresponds to a role that defines a user interface widget (slider, tree control, ...)
    pub fn is_interactive(self) -> bool {
        self.inherited_abstract_roles()
            .contains(&AriaAbstractRole::Widget)
    }

    /// Returns `true` if the given role inherits of `AriaAbstractRole::Structure`.
    ///
    /// This corresponds to a role that defines the page structure (section, navigation, ...).
    pub fn is_presentational(self) -> bool {
        self.inherited_abstract_roles()
            .contains(&AriaAbstractRole::Structure)
    }

    /// Returns `true` if the given role inherits of `AriaAbstractRole::Composite`.
    pub fn is_composite(self) -> bool {
        self.inherited_abstract_roles()
            .contains(&AriaAbstractRole::Composite)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AriaAttributes(&'static [AriaAttribute]);
impl AriaAttributes {
    // Same as `Self::default`, but usable in `const` context.
    pub const fn empty() -> Self {
        Self(&[])
    }

    pub fn contains(self, value: &AriaAttribute) -> bool {
        self.0.contains(value)
    }

    pub fn iter(self) -> impl Iterator<Item = AriaAttribute> {
        self.0.iter().copied()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AriaAbstractRoles(&'static [AriaAbstractRole]);
impl AriaAbstractRoles {
    // Same as `Self::default`, but usable in `const` context.
    pub const fn empty() -> Self {
        Self(&[])
    }

    pub fn contains(self, value: &AriaAbstractRole) -> bool {
        self.0.contains(value)
    }

    pub fn iter(self) -> impl Iterator<Item = AriaAbstractRole> {
        self.0.iter().copied()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AriaRoles(&'static [AriaRole]);
impl AriaRoles {
    // Same as `Self::default`, but usable in `const` context.
    pub const fn empty() -> Self {
        Self(&[])
    }

    pub fn contains(self, value: &AriaRole) -> bool {
        self.0.contains(value)
    }

    pub fn iter(self) -> impl Iterator<Item = AriaRole> {
        self.0.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HtmlElementInstance {
    element: HtmlElement,
    attributes: &'static [HtmlAttributeInstance],
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HtmlAttributeInstance {
    attribute: HtmlAttribute,
    value: &'static str,
}
