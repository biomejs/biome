/// Represents the [Case] of a string.
///
/// Note that some cases are superset of others.
/// For example, `Case::Camel` includes `Case::Lower`.
/// See [Case::is_compatible_with] for more details.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Case {
    /// Unknown case
    #[default]
    Unknown,
    /// camelCase
    Camel,
    // CONSTANT_CASE
    Constant,
    /// kebab-case
    Kebab,
    /// lowercase
    Lower,
    /// A, B1, C42
    NumberableCapital,
    /// PascalCase
    Pascal,
    /// snake_case
    Snake,
    /// Alphanumeric Characters that cannot be in lowercase or uppercase (numbers and syllabary)
    Uni,
    /// UPPERCASE
    Upper,
}

impl Case {
    /// Returns the [Case] of `value`.
    ///
    /// If `strict` is `true`, then two consecutive uppercase characters are not
    /// allowed in camelCase and PascalCase.
    /// For instance, `HTTPServer` is not considered in _PascalCase_ when `strict` is `true`.
    ///
    /// A figure is considered both uppercase and lowercase.
    /// Thus, `V8_ENGINE` is in _CONSTANt_CASE_ and `V8Engine` is in _PascalCase_.
    ///
    /// ### Examples
    ///
    /// ```
    /// use biome_js_analyze::utils::case::Case;
    ///
    /// assert_eq!(Case::identify("aHttpServer", /* no effect */ true), Case::Camel);
    /// assert_eq!(Case::identify("aHTTPServer", true), Case::Unknown);
    /// assert_eq!(Case::identify("aHTTPServer", false), Case::Camel);
    /// assert_eq!(Case::identify("v8Engine", /* no effect */ true), Case::Camel);
    ///
    /// assert_eq!(Case::identify("HTTP_SERVER", /* no effect */ true), Case::Constant);
    /// assert_eq!(Case::identify("V8_ENGINE", /* no effect */ true), Case::Constant);
    ///
    /// assert_eq!(Case::identify("http-server", /* no effect */ true), Case::Kebab);
    ///
    /// assert_eq!(Case::identify("httpserver", /* no effect */ true), Case::Lower);
    ///
    /// assert_eq!(Case::identify("T", /* no effect */ true), Case::NumberableCapital);
    /// assert_eq!(Case::identify("T1", /* no effect */ true), Case::NumberableCapital);
    ///
    /// assert_eq!(Case::identify("HttpServer", /* no effect */ true), Case::Pascal);
    /// assert_eq!(Case::identify("HTTPServer", true), Case::Unknown);
    /// assert_eq!(Case::identify("HTTPServer", false), Case::Pascal);
    /// assert_eq!(Case::identify("V8Engine", /* no effect */ true), Case::Pascal);
    ///
    /// assert_eq!(Case::identify("http_server", /* no effect */ true), Case::Snake);
    ///
    /// assert_eq!(Case::identify("HTTPSERVER", /* no effect */ true), Case::Upper);
    ///
    /// assert_eq!(Case::identify("100", /* no effect */ true), Case::Uni);
    /// assert_eq!(Case::identify("안녕하세요", /* no effect */ true), Case::Uni);
    ///
    /// assert_eq!(Case::identify("", /* no effect */ true), Case::Unknown);
    /// assert_eq!(Case::identify("_", /* no effect */ true), Case::Unknown);
    /// assert_eq!(Case::identify("안녕하세요abc", /* no effect */ true), Case::Unknown);
    /// ```
    pub fn identify(value: &str, strict: bool) -> Case {
        let mut chars = value.chars();
        let Some(first_char) = chars.next() else {
            return Case::Unknown;
        };
        let mut result = if first_char.is_uppercase() {
            Case::NumberableCapital
        } else if first_char.is_lowercase() {
            Case::Lower
        } else if first_char.is_alphanumeric() {
            Case::Uni
        } else {
            return Case::Unknown;
        };
        let mut previous_char = first_char;
        let mut has_consecutive_uppercase = false;
        for current_char in chars {
            result = match current_char {
                '-' => match result {
                    Case::Kebab | Case::Lower if previous_char != '-' => Case::Kebab,
                    _ => return Case::Unknown,
                },
                '_' => match result {
                    Case::Constant | Case::Snake if previous_char != '_' => result,
                    Case::NumberableCapital | Case::Upper => Case::Constant,
                    Case::Lower => Case::Snake,
                    _ => return Case::Unknown,
                },
                _ if current_char.is_uppercase() => {
                    has_consecutive_uppercase |= previous_char.is_uppercase();
                    match result {
                        Case::Camel | Case::Pascal if strict && has_consecutive_uppercase => {
                            return Case::Unknown
                        }
                        Case::Camel | Case::Constant | Case::Pascal => result,
                        Case::Lower => Case::Camel,
                        Case::NumberableCapital | Case::Upper => Case::Upper,
                        _ => return Case::Unknown,
                    }
                }
                _ if current_char.is_lowercase() => match result {
                    Case::Camel | Case::Kebab | Case::Lower | Case::Snake => result,
                    Case::Pascal | Case::NumberableCapital => Case::Pascal,
                    Case::Upper if !strict || !has_consecutive_uppercase => Case::Pascal,
                    _ => return Case::Unknown,
                },
                _ if current_char.is_numeric() => result,
                _ if current_char.is_alphabetic() => match result {
                    Case::Uni => Case::Uni,
                    _ => return Case::Unknown,
                },
                _ => return Case::Unknown,
            };
            previous_char = current_char;
        }
        // The last char cannot be a delimiter
        if matches!(previous_char, '-' | '_') {
            return Case::Unknown;
        }
        result
    }

    /// Returns `true` if a name that respects `self` also respects `other`.
    ///
    /// For example, a name in [Case::Lower] is also in [Case::Camel], [Case::Kebab] , and [Case::Snake].
    /// Thus [Case::Lower] is compatible with [Case::Camel], [Case::Kebab] , and [Case::Snake].
    ///
    /// Any [Case] is compatible with `Case::Unknown` and with itself.
    ///
    /// The compatibility relation between cases is depicted in the following diagram.
    /// The arrow means "is compatible with".
    ///
    /// ```svgbob
    ///                    ┌──► Pascal ────────────┐
    /// NumberableCapital ─┤                       │
    ///                    └──► Upper ─► Constant ─┤
    ///                                            ├──► Unknown
    ///                    ┌──► Kebab ─────────────┤
    ///             Lower ─┤                       │
    ///                    └──► Camel ─────────────┤
    ///                                            │
    ///               Uni ─────────────────────────┘
    /// ```
    ///
    /// ### Examples
    ///
    /// ```
    /// use biome_js_analyze::utils::case::Case;
    ///
    /// assert!(Case::Lower.is_compatible_with(Case::Camel));
    /// assert!(Case::Lower.is_compatible_with(Case::Kebab));
    /// assert!(Case::Lower.is_compatible_with(Case::Lower));
    /// assert!(Case::Lower.is_compatible_with(Case::Snake));
    ///
    /// assert!(Case::NumberableCapital.is_compatible_with(Case::Constant));
    /// assert!(Case::NumberableCapital.is_compatible_with(Case::Pascal));
    /// assert!(Case::NumberableCapital.is_compatible_with(Case::Upper));
    ///
    /// assert!(Case::Upper.is_compatible_with(Case::Constant));
    /// ```
    pub fn is_compatible_with(self, other: Case) -> bool {
        self == other
            || matches!(other, Case::Unknown)
            || matches!((self, other), |(
                Case::Lower,
                Case::Camel | Case::Kebab | Case::Snake,
            )| (
                Case::NumberableCapital,
                Case::Constant | Case::Pascal | Case::Upper
            ) | (
                Case::Upper,
                Case::Constant
            ))
    }

    /// Convert `value` to the `self` [Case].
    ///
    /// ### Examples
    ///
    /// ```
    /// use biome_js_analyze::utils::case::Case;
    ///
    /// assert_eq!(Case::Camel.convert("Http_SERVER"), "httpServer");
    /// assert_eq!(Case::Camel.convert("v8-Engine"), "v8Engine");
    ///
    /// assert_eq!(Case::Constant.convert("HttpServer"), "HTTP_SERVER");
    /// assert_eq!(Case::Constant.convert("v8-Engine"), "V8_ENGINE");
    ///
    /// assert_eq!(Case::Kebab.convert("Http_SERVER"), "http-server");
    /// assert_eq!(Case::Kebab.convert("v8Engine"), "v8-engine");
    ///
    /// assert_eq!(Case::Lower.convert("Http_SERVER"), "httpserver");
    ///
    /// assert_eq!(Case::NumberableCapital.convert("LONG"), "L");
    ///
    /// assert_eq!(Case::Pascal.convert("http_SERVER"), "HttpServer");
    ///
    /// assert_eq!(Case::Snake.convert("HttpServer"), "http_server");
    ///
    /// assert_eq!(Case::Upper.convert("Http_SERVER"), "HTTPSERVER");
    /// ```
    pub fn convert(self, value: &str) -> String {
        if value.is_empty() || matches!(self, Case::Unknown) {
            return value.to_string();
        }
        let mut word_separator = matches!(self, Case::Pascal);
        let mut output = String::with_capacity(value.len());
        for ((i, current), next) in value
            .char_indices()
            .zip(value.chars().skip(1).map(Some).chain(Some(None)))
        {
            if !current.is_alphanumeric()
                || (matches!(self, Case::Uni) && (current.is_lowercase() || current.is_uppercase()))
            {
                word_separator = true;
                continue;
            }
            if let Some(next) = next {
                if i != 0 && current.is_uppercase() && next.is_lowercase() {
                    word_separator = true;
                }
            }
            if word_separator {
                match self {
                    Case::Camel
                    | Case::Lower
                    | Case::NumberableCapital
                    | Case::Pascal
                    | Case::Unknown
                    | Case::Uni
                    | Case::Upper => (),
                    Case::Constant | Case::Snake => {
                        output.push('_');
                    }
                    Case::Kebab => {
                        output.push('-');
                    }
                }
            }
            match self {
                Case::Camel | Case::Pascal => {
                    if word_separator {
                        output.extend(current.to_uppercase())
                    } else {
                        output.extend(current.to_lowercase())
                    }
                }
                Case::Constant | Case::Upper => output.extend(current.to_uppercase()),
                Case::NumberableCapital => {
                    if i == 0 {
                        output.extend(current.to_uppercase());
                    }
                }
                Case::Kebab | Case::Snake | Case::Lower => output.extend(current.to_lowercase()),
                Case::Uni => output.extend(Some(current)),
                Case::Unknown => (),
            }
            word_separator = false;
            if let Some(next) = next {
                if current.is_lowercase() && next.is_uppercase() {
                    word_separator = true;
                }
            }
        }
        output
    }
}

impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Case::Unknown => "unknown case",
            Case::Camel => "camelCase",
            Case::Constant => "CONSTANT_CASE",
            Case::Kebab => "kebab-case",
            Case::Lower => "lowercase",
            Case::NumberableCapital => "numberable capital case",
            Case::Pascal => "PascalCase",
            Case::Snake => "snake_case",
            Case::Uni => "unicase",
            Case::Upper => "UPPERCASE",
        };
        write!(f, "{}", repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_identify() {
        let no_effect = true;

        assert_eq!(Case::identify("aHttpServer", no_effect), Case::Camel);
        assert_eq!(Case::identify("aHTTPServer", true), Case::Unknown);
        assert_eq!(Case::identify("aHTTPServer", false), Case::Camel);
        assert_eq!(Case::identify("v8Engine", no_effect), Case::Camel);

        assert_eq!(Case::identify("HTTP_SERVER", no_effect), Case::Constant);
        assert_eq!(Case::identify("V8_ENGINE", no_effect), Case::Constant);

        assert_eq!(Case::identify("http-server", no_effect), Case::Kebab);

        assert_eq!(Case::identify("httpserver", no_effect), Case::Lower);

        assert_eq!(Case::identify("T", no_effect), Case::NumberableCapital);
        assert_eq!(Case::identify("T1", no_effect), Case::NumberableCapital);

        assert_eq!(Case::identify("HttpServer", no_effect), Case::Pascal);
        assert_eq!(Case::identify("HTTPServer", true), Case::Unknown);
        assert_eq!(Case::identify("HTTPServer", false), Case::Pascal);
        assert_eq!(Case::identify("V8Engine", true), Case::Pascal);

        assert_eq!(Case::identify("http_server", no_effect), Case::Snake);

        assert_eq!(Case::identify("HTTPSERVER", no_effect), Case::Upper);

        assert_eq!(Case::identify("100", no_effect), Case::Uni);
        assert_eq!(Case::identify("안녕하세요", no_effect), Case::Uni);

        // don't allow identifier that starts/ends with a delimiter
        assert_eq!(Case::identify("-a", no_effect), Case::Unknown);
        assert_eq!(Case::identify("_a", no_effect), Case::Unknown);
        assert_eq!(Case::identify("a-", no_effect), Case::Unknown);
        assert_eq!(Case::identify("a_", no_effect), Case::Unknown);

        // don't allow identifier that use consecutive delimiters
        assert_eq!(Case::identify("a--a", no_effect), Case::Unknown);
        assert_eq!(Case::identify("a__a", no_effect), Case::Unknown);

        assert_eq!(Case::identify("", no_effect), Case::Unknown);
        assert_eq!(Case::identify("-", no_effect), Case::Unknown);
        assert_eq!(Case::identify("_", no_effect), Case::Unknown);
        assert_eq!(Case::identify("안녕하세요ABC", no_effect), Case::Unknown);
        assert_eq!(Case::identify("안녕하세요abc", no_effect), Case::Unknown);
        assert_eq!(Case::identify("안녕하세요_ABC", no_effect), Case::Unknown);
        assert_eq!(Case::identify("안녕하세요_abc", no_effect), Case::Unknown);
        assert_eq!(Case::identify("안녕하세요-abc", no_effect), Case::Unknown);
    }

    #[test]
    fn test_case_is_compatible() {
        assert!(Case::Unknown.is_compatible_with(Case::Unknown));
        assert!(!Case::Unknown.is_compatible_with(Case::Camel));
        assert!(!Case::Unknown.is_compatible_with(Case::Constant));
        assert!(!Case::Unknown.is_compatible_with(Case::Kebab));
        assert!(!Case::Unknown.is_compatible_with(Case::Lower));
        assert!(!Case::Unknown.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Unknown.is_compatible_with(Case::Pascal));
        assert!(!Case::Unknown.is_compatible_with(Case::Snake));
        assert!(!Case::Unknown.is_compatible_with(Case::Uni));
        assert!(!Case::Unknown.is_compatible_with(Case::Upper));

        assert!(Case::Camel.is_compatible_with(Case::Unknown));
        assert!(Case::Camel.is_compatible_with(Case::Camel));
        assert!(!Case::Camel.is_compatible_with(Case::Constant));
        assert!(!Case::Camel.is_compatible_with(Case::Kebab));
        assert!(!Case::Camel.is_compatible_with(Case::Lower));
        assert!(!Case::Camel.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Camel.is_compatible_with(Case::Pascal));
        assert!(!Case::Camel.is_compatible_with(Case::Snake));
        assert!(!Case::Camel.is_compatible_with(Case::Uni));
        assert!(!Case::Camel.is_compatible_with(Case::Upper));

        assert!(Case::Constant.is_compatible_with(Case::Unknown));
        assert!(!Case::Constant.is_compatible_with(Case::Camel));
        assert!(Case::Constant.is_compatible_with(Case::Constant));
        assert!(!Case::Constant.is_compatible_with(Case::Kebab));
        assert!(!Case::Constant.is_compatible_with(Case::Lower));
        assert!(!Case::Constant.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Constant.is_compatible_with(Case::Pascal));
        assert!(!Case::Constant.is_compatible_with(Case::Snake));
        assert!(!Case::Constant.is_compatible_with(Case::Uni));
        assert!(!Case::Constant.is_compatible_with(Case::Upper));

        assert!(Case::Kebab.is_compatible_with(Case::Unknown));
        assert!(!Case::Kebab.is_compatible_with(Case::Camel));
        assert!(!Case::Kebab.is_compatible_with(Case::Constant));
        assert!(Case::Kebab.is_compatible_with(Case::Kebab));
        assert!(!Case::Kebab.is_compatible_with(Case::Lower));
        assert!(!Case::Kebab.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Kebab.is_compatible_with(Case::Pascal));
        assert!(!Case::Kebab.is_compatible_with(Case::Snake));
        assert!(!Case::Kebab.is_compatible_with(Case::Uni));
        assert!(!Case::Kebab.is_compatible_with(Case::Upper));

        assert!(Case::Lower.is_compatible_with(Case::Unknown));
        assert!(Case::Lower.is_compatible_with(Case::Camel));
        assert!(!Case::Lower.is_compatible_with(Case::Constant));
        assert!(Case::Lower.is_compatible_with(Case::Kebab));
        assert!(Case::Lower.is_compatible_with(Case::Lower));
        assert!(!Case::Lower.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Lower.is_compatible_with(Case::Pascal));
        assert!(Case::Lower.is_compatible_with(Case::Snake));
        assert!(!Case::Lower.is_compatible_with(Case::Uni));
        assert!(!Case::Lower.is_compatible_with(Case::Upper));

        assert!(Case::NumberableCapital.is_compatible_with(Case::Unknown));
        assert!(!Case::NumberableCapital.is_compatible_with(Case::Camel));
        assert!(Case::NumberableCapital.is_compatible_with(Case::Constant));
        assert!(!Case::NumberableCapital.is_compatible_with(Case::Kebab));
        assert!(!Case::NumberableCapital.is_compatible_with(Case::Lower));
        assert!(Case::NumberableCapital.is_compatible_with(Case::NumberableCapital));
        assert!(Case::NumberableCapital.is_compatible_with(Case::Pascal));
        assert!(!Case::NumberableCapital.is_compatible_with(Case::Snake));
        assert!(!Case::NumberableCapital.is_compatible_with(Case::Uni));
        assert!(Case::NumberableCapital.is_compatible_with(Case::Upper));

        assert!(Case::Pascal.is_compatible_with(Case::Unknown));
        assert!(!Case::Pascal.is_compatible_with(Case::Camel));
        assert!(!Case::Pascal.is_compatible_with(Case::Constant));
        assert!(!Case::Pascal.is_compatible_with(Case::Kebab));
        assert!(!Case::Pascal.is_compatible_with(Case::Lower));
        assert!(!Case::Pascal.is_compatible_with(Case::NumberableCapital));
        assert!(Case::Pascal.is_compatible_with(Case::Pascal));
        assert!(!Case::Pascal.is_compatible_with(Case::Snake));
        assert!(!Case::Pascal.is_compatible_with(Case::Uni));
        assert!(!Case::Pascal.is_compatible_with(Case::Upper));

        assert!(Case::Snake.is_compatible_with(Case::Unknown));
        assert!(!Case::Snake.is_compatible_with(Case::Camel));
        assert!(!Case::Snake.is_compatible_with(Case::Constant));
        assert!(!Case::Snake.is_compatible_with(Case::Kebab));
        assert!(!Case::Snake.is_compatible_with(Case::Lower));
        assert!(!Case::Snake.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Snake.is_compatible_with(Case::Pascal));
        assert!(Case::Snake.is_compatible_with(Case::Snake));
        assert!(!Case::Snake.is_compatible_with(Case::Uni));
        assert!(!Case::Snake.is_compatible_with(Case::Upper));

        assert!(Case::Uni.is_compatible_with(Case::Unknown));
        assert!(!Case::Uni.is_compatible_with(Case::Camel));
        assert!(!Case::Uni.is_compatible_with(Case::Constant));
        assert!(!Case::Uni.is_compatible_with(Case::Kebab));
        assert!(!Case::Uni.is_compatible_with(Case::Lower));
        assert!(!Case::Uni.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Uni.is_compatible_with(Case::Pascal));
        assert!(!Case::Uni.is_compatible_with(Case::Snake));
        assert!(Case::Uni.is_compatible_with(Case::Uni));
        assert!(!Case::Uni.is_compatible_with(Case::Upper));

        assert!(Case::Upper.is_compatible_with(Case::Unknown));
        assert!(!Case::Upper.is_compatible_with(Case::Camel));
        assert!(Case::Upper.is_compatible_with(Case::Constant));
        assert!(!Case::Upper.is_compatible_with(Case::Kebab));
        assert!(!Case::Upper.is_compatible_with(Case::Lower));
        assert!(!Case::Upper.is_compatible_with(Case::NumberableCapital));
        assert!(!Case::Upper.is_compatible_with(Case::Pascal));
        assert!(!Case::Upper.is_compatible_with(Case::Snake));
        assert!(!Case::Upper.is_compatible_with(Case::Uni));
        assert!(Case::Upper.is_compatible_with(Case::Upper));
    }

    #[test]
    fn test_case_convert() {
        assert_eq!(Case::Camel.convert("camelCase"), "camelCase");
        assert_eq!(Case::Camel.convert("CONSTANT_CASE"), "constantCase");
        assert_eq!(Case::Camel.convert("kebab-case"), "kebabCase");
        assert_eq!(Case::Camel.convert("PascalCase"), "pascalCase");
        assert_eq!(Case::Camel.convert("snake_case"), "snakeCase");
        assert_eq!(Case::Camel.convert("Unknown_Style"), "unknownStyle");

        assert_eq!(Case::Constant.convert("camelCase"), "CAMEL_CASE");
        assert_eq!(Case::Constant.convert("CONSTANT_CASE"), "CONSTANT_CASE");
        assert_eq!(Case::Constant.convert("kebab-case"), "KEBAB_CASE");
        assert_eq!(Case::Constant.convert("PascalCase"), "PASCAL_CASE");
        assert_eq!(Case::Constant.convert("snake_case"), "SNAKE_CASE");
        assert_eq!(Case::Constant.convert("Unknown_Style"), "UNKNOWN_STYLE");

        assert_eq!(Case::Kebab.convert("camelCase"), "camel-case");
        assert_eq!(Case::Kebab.convert("CONSTANT_CASE"), "constant-case");
        assert_eq!(Case::Kebab.convert("kebab-case"), "kebab-case");
        assert_eq!(Case::Kebab.convert("PascalCase"), "pascal-case");
        assert_eq!(Case::Kebab.convert("snake_case"), "snake-case");
        assert_eq!(Case::Kebab.convert("Unknown_Style"), "unknown-style");

        assert_eq!(Case::Lower.convert("camelCase"), "camelcase");
        assert_eq!(Case::Lower.convert("CONSTANT_CASE"), "constantcase");
        assert_eq!(Case::Lower.convert("kebab-case"), "kebabcase");
        assert_eq!(Case::Lower.convert("PascalCase"), "pascalcase");
        assert_eq!(Case::Lower.convert("snake_case"), "snakecase");
        assert_eq!(Case::Lower.convert("Unknown_Style"), "unknownstyle");

        assert_eq!(Case::NumberableCapital.convert("LONG"), "L");

        assert_eq!(Case::Pascal.convert("camelCase"), "CamelCase");
        assert_eq!(Case::Pascal.convert("CONSTANT_CASE"), "ConstantCase");
        assert_eq!(Case::Pascal.convert("kebab-case"), "KebabCase");
        assert_eq!(Case::Pascal.convert("PascalCase"), "PascalCase");
        assert_eq!(Case::Pascal.convert("V8Engine"), "V8Engine");
        assert_eq!(Case::Pascal.convert("snake_case"), "SnakeCase");
        assert_eq!(Case::Pascal.convert("Unknown_Style"), "UnknownStyle");

        assert_eq!(Case::Snake.convert("camelCase"), "camel_case");
        assert_eq!(Case::Snake.convert("CONSTANT_CASE"), "constant_case");
        assert_eq!(Case::Snake.convert("kebab-case"), "kebab_case");
        assert_eq!(Case::Snake.convert("PascalCase"), "pascal_case");
        assert_eq!(Case::Snake.convert("snake_case"), "snake_case");
        assert_eq!(Case::Snake.convert("Unknown_Style"), "unknown_style");

        assert_eq!(Case::Upper.convert("camelCase"), "CAMELCASE");
        assert_eq!(Case::Upper.convert("CONSTANT_CASE"), "CONSTANTCASE");
        assert_eq!(Case::Upper.convert("kebab-case"), "KEBABCASE");
        assert_eq!(Case::Upper.convert("PascalCase"), "PASCALCASE");
        assert_eq!(Case::Upper.convert("snake_case"), "SNAKECASE");
        assert_eq!(Case::Upper.convert("Unknown_Style"), "UNKNOWNSTYLE");

        assert_eq!(Case::Uni.convert("안녕하세요"), "안녕하세요");
        assert_eq!(Case::Uni.convert("a안b녕c하_세D요E"), "안녕하세요");

        assert_eq!(Case::Unknown.convert("Unknown_Style"), "Unknown_Style");
    }
}
