//! Identify string case and convert to various string cases.

/// Represents the [Case] of a string.
///
/// Note that some cases are superset of others.
/// For example, a name in [Case::Lower] is also in [Case::Camel], [Case::Kebab] , and [Case::Snake].
/// Thus [Case::Camel], [Case::Kebab], and [Case::Snake] are superset of [Case::Lower].
/// `Case::Unknown` is a superset of all [Case].
///
/// The relation between cases is depicted in the following diagram.
/// The arrow means "is subset of".
///
/// ```svgbob
///                    ┌──► Pascal ────────────┐
/// NumberableCapital ─┤                       │
///                    └──► Upper ─► Constant ─┤
///                                            ├──► Unknown
///                    ┌──► Camel ─────────────┤
///             Lower ─┤                       │
///                    └──► Kebab ─────────────┤
///                    │                       │
///                    └──► Snake ─────────────┤
///                                            │
///               Uni ─────────────────────────┘
/// ```
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Case {
    /// Alphanumeric Characters that cannot be in lowercase or uppercase (numbers and syllabary)
    Uni = 1 << 0,
    /// A, B1, C42
    NumberableCapital = 1 << 1,
    /// UPPERCASE
    Upper = Case::NumberableCapital as u16 | 1 << 2,
    // CONSTANT_CASE
    Constant = Case::Upper as u16 | 1 << 3,
    /// PascalCase
    Pascal = Case::NumberableCapital as u16 | 1 << 4,
    /// lowercase
    Lower = 1 << 5,
    /// snake_case
    Snake = Case::Lower as u16 | 1 << 6,
    /// kebab-case
    Kebab = Case::Lower as u16 | 1 << 7,
    // camelCase
    Camel = Case::Lower as u16 | 1 << 8,
    /// Unknown case
    #[default]
    Unknown = Case::Camel as u16
        | Case::Kebab as u16
        | Case::Snake as u16
        | Case::Pascal as u16
        | Case::Constant as u16
        | Case::Uni as u16
        | 1 << 9,
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
    /// ```
    /// use biome_string_case::Case;
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

    /// Convert `value` to the `self` [Case].
    ///
    /// ```
    /// use biome_string_case::Case;
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

/// Represents a set of cases.
///
/// An instance of [Cases] supports the binary operators `|` to unionize two sets or add a new [Case].
///
/// Note that some [Case] are already sets of [Case].
/// For example, [Case::Unknown] is a set that includes all [Case].
/// So adding [Case::Unknown] to a [Cases] will superseed all other cases.
///
/// A [Cases] is iterable.
/// A Cases iterator doesn't yield a [Case] that is covered by another [Case] in the set.
/// See [CasesIterator] for more details.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Cases(u16);

impl Cases {
    /// Create an empty set.
    ///
    /// You can also obtain an empty alias using [Cases::default()].
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns `true` if the set is empty.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if all cases of `other` are contained in the current set.
    ///
    /// ```
    /// use biome_string_case::{Cases, Case};
    ///
    /// let camel_or_kebab = (Case::Camel | Case::Kebab);
    ///
    /// assert!(camel_or_kebab.contains(Case::Camel));
    /// assert!(camel_or_kebab.contains(camel_or_kebab));
    /// ```
    pub fn contains(self, other: impl Into<Cases>) -> bool {
        let other = other.into();
        self.0 & other.0 == other.0
    }
}

impl IntoIterator for Cases {
    type Item = Case;
    type IntoIter = CasesIterator;
    fn into_iter(self) -> Self::IntoIter {
        CasesIterator { rest: self }
    }
}

impl FromIterator<Case> for Cases {
    fn from_iter<T: IntoIterator<Item = Case>>(iter: T) -> Self {
        iter.into_iter()
            .fold(Self::empty(), |result, case| result | case)
    }
}

impl From<Case> for Cases {
    fn from(value: Case) -> Self {
        Self(value as u16)
    }
}

impl<Rhs: Into<Cases>> core::ops::BitOr<Rhs> for Cases {
    type Output = Cases;
    fn bitor(self, rhs: Rhs) -> Self::Output {
        Self(self.0 | rhs.into().0)
    }
}
impl core::ops::BitOr for Case {
    type Output = Cases;
    fn bitor(self, rhs: Self) -> Self::Output {
        Cases::from(self) | rhs
    }
}
impl<Rhs: Into<Cases>> core::ops::BitOrAssign<Rhs> for Cases {
    fn bitor_assign(&mut self, rhs: Rhs) {
        self.0 |= rhs.into().0;
    }
}

/// An iterator of [Cases].
///
/// The iterator doesn't yield a [Case] that is covered by another [Case] in the iterated set.
/// For example, if the set includes [Case::Constant] and [Case::Upper],
/// the iterator only yields [Case::Constant] because [Case::Constant] covers [Case::Upper].
///
/// ```
/// use biome_string_case::{Cases, Case};
///
/// let cases = Case::Camel | Case::Kebab;
/// assert_eq!(cases.into_iter().collect::<Vec<_>>().as_slice(), &[Case::Camel, Case::Kebab]);
///
/// let cases = Case::Camel | Case::Kebab | Case::Lower;
/// assert_eq!(cases.into_iter().collect::<Vec<_>>().as_slice(), &[Case::Camel, Case::Kebab]);
/// ```
#[derive(Clone, Debug)]
pub struct CasesIterator {
    rest: Cases,
}
impl Iterator for CasesIterator {
    type Item = Case;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            None
        } else {
            let leading_bit_index = 15u16 - (self.rest.0.leading_zeros() as u16);
            let case = LEADING_BIT_INDEX_TO_CASE[leading_bit_index as usize];
            self.rest.0 &= !(case as u16);
            Some(case)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(6))
    }
}
impl std::iter::FusedIterator for CasesIterator {}

const LEADING_BIT_INDEX_TO_CASE: [Case; 10] = [
    Case::Uni,
    Case::NumberableCapital,
    Case::Upper,
    Case::Constant,
    Case::Pascal,
    Case::Lower,
    Case::Snake,
    Case::Kebab,
    Case::Camel,
    Case::Unknown,
];

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
    fn test_cases_contains() {
        // Individual cases
        assert!(Cases::from(Case::Unknown).contains(Case::Unknown));
        assert!(!Cases::from(Case::Camel).contains(Case::Unknown));
        assert!(!Cases::from(Case::Constant).contains(Case::Unknown));
        assert!(!Cases::from(Case::Kebab).contains(Case::Unknown));
        assert!(!Cases::from(Case::Lower).contains(Case::Unknown));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Unknown));
        assert!(!Cases::from(Case::Pascal).contains(Case::Unknown));
        assert!(!Cases::from(Case::Snake).contains(Case::Unknown));
        assert!(!Cases::from(Case::Uni).contains(Case::Unknown));
        assert!(!Cases::from(Case::Upper).contains(Case::Unknown));

        assert!(Cases::from(Case::Unknown).contains(Case::Camel));
        assert!(Cases::from(Case::Camel).contains(Case::Camel));
        assert!(!Cases::from(Case::Constant).contains(Case::Camel));
        assert!(!Cases::from(Case::Kebab).contains(Case::Camel));
        assert!(!Cases::from(Case::Lower).contains(Case::Camel));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Camel));
        assert!(!Cases::from(Case::Pascal).contains(Case::Camel));
        assert!(!Cases::from(Case::Snake).contains(Case::Camel));
        assert!(!Cases::from(Case::Uni).contains(Case::Camel));
        assert!(!Cases::from(Case::Upper).contains(Case::Camel));

        assert!(Cases::from(Case::Unknown).contains(Case::Constant));
        assert!(!Cases::from(Case::Camel).contains(Case::Constant));
        assert!(Cases::from(Case::Constant).contains(Case::Constant));
        assert!(!Cases::from(Case::Kebab).contains(Case::Constant));
        assert!(!Cases::from(Case::Lower).contains(Case::Constant));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Constant));
        assert!(!Cases::from(Case::Pascal).contains(Case::Constant));
        assert!(!Cases::from(Case::Snake).contains(Case::Constant));
        assert!(!Cases::from(Case::Uni).contains(Case::Constant));
        assert!(!Cases::from(Case::Upper).contains(Case::Constant));

        assert!(Cases::from(Case::Unknown).contains(Case::Kebab));
        assert!(!Cases::from(Case::Camel).contains(Case::Kebab));
        assert!(!Cases::from(Case::Constant).contains(Case::Kebab));
        assert!(Cases::from(Case::Kebab).contains(Case::Kebab));
        assert!(!Cases::from(Case::Lower).contains(Case::Kebab));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Kebab));
        assert!(!Cases::from(Case::Pascal).contains(Case::Kebab));
        assert!(!Cases::from(Case::Snake).contains(Case::Kebab));
        assert!(!Cases::from(Case::Uni).contains(Case::Kebab));
        assert!(!Cases::from(Case::Upper).contains(Case::Kebab));

        assert!(Cases::from(Case::Unknown).contains(Case::Lower));
        assert!(Cases::from(Case::Camel).contains(Case::Lower));
        assert!(!Cases::from(Case::Constant).contains(Case::Lower));
        assert!(Cases::from(Case::Kebab).contains(Case::Lower));
        assert!(Cases::from(Case::Lower).contains(Case::Lower));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Lower));
        assert!(!Cases::from(Case::Pascal).contains(Case::Lower));
        assert!(Cases::from(Case::Snake).contains(Case::Lower));
        assert!(!Cases::from(Case::Uni).contains(Case::Lower));
        assert!(!Cases::from(Case::Upper).contains(Case::Lower));

        assert!(Cases::from(Case::Unknown).contains(Case::NumberableCapital));
        assert!(!Cases::from(Case::Camel).contains(Case::NumberableCapital));
        assert!(Cases::from(Case::Constant).contains(Case::NumberableCapital));
        assert!(!Cases::from(Case::Kebab).contains(Case::NumberableCapital));
        assert!(!Cases::from(Case::Lower).contains(Case::NumberableCapital));
        assert!(Cases::from(Case::NumberableCapital).contains(Case::NumberableCapital));
        assert!(Cases::from(Case::Pascal).contains(Case::NumberableCapital));
        assert!(!Cases::from(Case::Snake).contains(Case::NumberableCapital));
        assert!(!Cases::from(Case::Uni).contains(Case::NumberableCapital));
        assert!(Cases::from(Case::Upper).contains(Case::NumberableCapital));

        assert!(Cases::from(Case::Unknown).contains(Case::Pascal));
        assert!(!Cases::from(Case::Camel).contains(Case::Pascal));
        assert!(!Cases::from(Case::Constant).contains(Case::Pascal));
        assert!(!Cases::from(Case::Kebab).contains(Case::Pascal));
        assert!(!Cases::from(Case::Lower).contains(Case::Pascal));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Pascal));
        assert!(Cases::from(Case::Pascal).contains(Case::Pascal));
        assert!(!Cases::from(Case::Snake).contains(Case::Pascal));
        assert!(!Cases::from(Case::Uni).contains(Case::Pascal));
        assert!(!Cases::from(Case::Upper).contains(Case::Pascal));

        assert!(Cases::from(Case::Unknown).contains(Case::Snake));
        assert!(!Cases::from(Case::Camel).contains(Case::Snake));
        assert!(!Cases::from(Case::Constant).contains(Case::Snake));
        assert!(!Cases::from(Case::Kebab).contains(Case::Snake));
        assert!(!Cases::from(Case::Lower).contains(Case::Snake));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Snake));
        assert!(!Cases::from(Case::Pascal).contains(Case::Snake));
        assert!(Cases::from(Case::Snake).contains(Case::Snake));
        assert!(!Cases::from(Case::Uni).contains(Case::Snake));
        assert!(!Cases::from(Case::Upper).contains(Case::Snake));

        assert!(Cases::from(Case::Unknown).contains(Case::Uni));
        assert!(!Cases::from(Case::Camel).contains(Case::Uni));
        assert!(!Cases::from(Case::Constant).contains(Case::Uni));
        assert!(!Cases::from(Case::Kebab).contains(Case::Uni));
        assert!(!Cases::from(Case::Lower).contains(Case::Uni));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Uni));
        assert!(!Cases::from(Case::Pascal).contains(Case::Uni));
        assert!(!Cases::from(Case::Snake).contains(Case::Uni));
        assert!(Cases::from(Case::Uni).contains(Case::Uni));
        assert!(!Cases::from(Case::Upper).contains(Case::Uni));

        assert!(Cases::from(Case::Unknown).contains(Case::Upper));
        assert!(!Cases::from(Case::Camel).contains(Case::Upper));
        assert!(Cases::from(Case::Constant).contains(Case::Upper));
        assert!(!Cases::from(Case::Kebab).contains(Case::Upper));
        assert!(!Cases::from(Case::Lower).contains(Case::Upper));
        assert!(!Cases::from(Case::NumberableCapital).contains(Case::Upper));
        assert!(!Cases::from(Case::Pascal).contains(Case::Upper));
        assert!(!Cases::from(Case::Snake).contains(Case::Upper));
        assert!(!Cases::from(Case::Uni).contains(Case::Upper));
        assert!(Cases::from(Case::Upper).contains(Case::Upper));

        // Set of cases
        assert!((Case::Camel | Case::Kebab | Case::Snake).contains(Case::Camel));
        assert!((Case::Camel | Case::Kebab | Case::Snake).contains(Case::Kebab));
        assert!((Case::Camel | Case::Kebab | Case::Snake).contains(Case::Snake));
        assert!((Case::Camel | Case::Kebab | Case::Snake).contains(Case::Lower));
        assert!(!(Case::Camel | Case::Kebab | Case::Snake).contains(Case::Unknown));
        assert!(!(Case::Camel | Case::Kebab | Case::Snake).contains(Case::Constant));
        assert!(!(Case::Camel | Case::Kebab | Case::Snake).contains(Case::Upper));
        assert!(!(Case::Camel | Case::Kebab | Case::Snake).contains(Case::NumberableCapital));
        assert!(!(Case::Camel | Case::Kebab | Case::Snake).contains(Case::Uni));

        assert!((Case::Constant | Case::Upper).contains(Case::Constant));
        assert!((Case::Constant | Case::Upper).contains(Case::Upper));
        assert!((Case::Constant | Case::Upper).contains(Case::NumberableCapital));
        assert!(!(Case::Constant | Case::Upper).contains(Case::Unknown));
        assert!(!(Case::Constant | Case::Upper).contains(Case::Camel));
        assert!(!(Case::Constant | Case::Upper).contains(Case::Kebab));
        assert!(!(Case::Constant | Case::Upper).contains(Case::Snake));
        assert!(!(Case::Constant | Case::Upper).contains(Case::Lower));
        assert!(!(Case::Constant | Case::Upper).contains(Case::Uni));
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

    #[test]
    fn test_cases_iter() {
        fn vec(value: impl Into<Cases>) -> Vec<Case> {
            value.into().into_iter().collect::<Vec<_>>()
        }

        assert_eq!(vec(Cases::empty()).as_slice(), &[]);
        assert_eq!(vec(Case::Unknown).as_slice(), &[Case::Unknown]);
        assert_eq!(vec(Case::Camel).as_slice(), &[Case::Camel]);
        assert_eq!(vec(Case::Kebab).as_slice(), &[Case::Kebab]);
        assert_eq!(vec(Case::Snake).as_slice(), &[Case::Snake]);
        assert_eq!(vec(Case::Lower).as_slice(), &[Case::Lower]);
        assert_eq!(vec(Case::Pascal).as_slice(), &[Case::Pascal]);
        assert_eq!(vec(Case::Constant).as_slice(), &[Case::Constant]);
        assert_eq!(vec(Case::Upper).as_slice(), &[Case::Upper]);
        assert_eq!(vec(Case::Uni).as_slice(), &[Case::Uni]);
        assert_eq!(
            vec(Case::NumberableCapital).as_slice(),
            &[Case::NumberableCapital]
        );

        assert_eq!(
            vec(Case::Unknown | Case::Camel).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::Kebab).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::Snake).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::Lower).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::Pascal).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::Constant).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::Upper).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(
            vec(Case::Unknown | Case::NumberableCapital).as_slice(),
            &[Case::Unknown]
        );
        assert_eq!(vec(Case::Unknown | Case::Uni).as_slice(), &[Case::Unknown]);
        assert_eq!(
            vec(Case::Unknown | Case::Pascal | Case::Camel).as_slice(),
            &[Case::Unknown]
        );

        assert_eq!(vec(Case::Camel | Case::Lower).as_slice(), &[Case::Camel]);
        assert_eq!(vec(Case::Kebab | Case::Lower).as_slice(), &[Case::Kebab]);
        assert_eq!(vec(Case::Snake | Case::Lower).as_slice(), &[Case::Snake]);

        assert_eq!(
            vec(Case::Constant | Case::Upper).as_slice(),
            &[Case::Constant]
        );

        assert_eq!(
            vec(Case::Pascal | Case::NumberableCapital).as_slice(),
            &[Case::Pascal]
        );
        assert_eq!(
            vec(Case::Constant | Case::NumberableCapital).as_slice(),
            &[Case::Constant]
        );
        assert_eq!(
            vec(Case::Upper | Case::NumberableCapital).as_slice(),
            &[Case::Upper]
        );

        assert_eq!(
            vec(Case::Pascal | Case::Camel).as_slice(),
            &[Case::Camel, Case::Pascal]
        );
        assert_eq!(
            vec(Case::NumberableCapital | Case::Uni).as_slice(),
            &[Case::NumberableCapital, Case::Uni]
        );

        assert_eq!(
            vec(Case::Pascal
                | Case::Constant
                | Case::Camel
                | Case::Kebab
                | Case::Snake
                | Case::Uni)
            .as_slice(),
            &[
                Case::Camel,
                Case::Kebab,
                Case::Snake,
                Case::Pascal,
                Case::Constant,
                Case::Uni
            ]
        );
    }

    #[test]
    fn test_leading_bit_to_case() {
        for (i, case) in LEADING_BIT_INDEX_TO_CASE.iter().enumerate() {
            assert_eq!(i as u16, 15u16 - (*case as u16).leading_zeros() as u16)
        }
    }

    #[test]
    fn test_size_hint_upper_limit() {
        let mut cases = Cases::empty();
        let mut max_count = 0;
        for case in LEADING_BIT_INDEX_TO_CASE {
            let count = (cases | case).into_iter().count();
            if count >= max_count {
                cases |= case;
                max_count = count;
            }
        }
        assert_eq!(cases.into_iter().size_hint().1, Some(max_count));
    }
}
