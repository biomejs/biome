use crate::{services::semantic::SemanticServices, utils::restricted_regex::RestrictedRegex};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize::DeserializationContext;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    binding_ext::AnyJsIdentifierBinding, AnyJsIdentifierUsage, JsExportNamedSpecifier,
};
use biome_rowan::{AstNode, TextRange};
use biome_string_case::{Case, Cases};
use smallvec::SmallVec;
use std::{hash::Hash, str::FromStr};

declare_lint_rule! {
    /// Enforce naming conventions for JavaScript and TypeScript filenames.
    ///
    /// Enforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent.
    ///
    /// A filename consists of two parts: a name and a set of consecutive extensions.
    /// For instance, `my-filename.test.js` has `my-filename` as name, and two consecutive extensions: `.test` and `.js`.
    ///
    /// By default, the rule ensures that the name is either in [`camelCase`], [`kebab-case`], [`snake_case`],
    /// or equal to the name of one export in the file.
    /// By default, the rule ensures that the extensions are either in [`camelCase`], [`kebab-case`], or [`snake_case`].
    ///
    /// The rule supports the following exceptions:
    ///
    /// - The name of the file can start with a dot or a plus sign, be prefixed and suffixed by underscores `_`.
    ///   For example, `.filename.js`, `+filename.js`, `__filename__.js`, or even `.__filename__.js`.
    ///
    ///   The convention of prefixing a filename with a plus sign is used by [Sveltekit](https://kit.svelte.dev/docs/routing#page) and [Vike](https://vike.dev/route).
    ///
    /// - Also, the rule supports dynamic route syntaxes of [Next.js](https://nextjs.org/docs/pages/building-your-application/routing/dynamic-routes#catch-all-segments), [SolidStart](https://docs.solidjs.com/solid-start/building-your-application/routing#renaming-index), [Nuxt](https://nuxt.com/docs/guide/directory-structure/server#catch-all-route), and [Astro](https://docs.astro.build/en/guides/routing/#rest-parameters).
    ///   For example `[...slug].js` and `[[...slug]].js` are valid filenames.
    ///
    /// Note that if you specify the `match' option, the previous exceptions will no longer be handled.
    ///
    /// ## Ignoring some files
    ///
    /// Sometimes you want to completely ignore some files.
    /// Biome ignore comments cannot be used because the rule applies on filenames not file contents.
    /// To ignore files, you can use [`overrides`](https://biomejs.dev/reference/configuration/#overrides).
    /// If you want to ignore all files in the `test` directory, then you can disable the rule for those files only:
    ///
    /// ```json
    /// {
    ///   "overrides": [
    ///     {
    ///        "includes": ["test/**/*"],
    ///        "linter": {
    ///          "rules": {
    ///            "style": {
    ///              "useFilenamingConvention": "off"
    ///            }
    ///          }
    ///        }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides several options that are detailed in the following subsections.
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "strictCase": false,
    ///         "requireAscii": true,
    ///         "match": "%?(.+?)[.](.+)", // Since v2.0.0
    ///         "filenameCases": ["camelCase", "export"]
    ///     }
    /// }
    /// ```
    ///
    /// ### strictCase
    ///
    /// When this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`] and [`PascalCase`].
    /// For instance,  when the option is set to `true`, `agentID` will throw an error.
    /// This name should be renamed to `agentId`.
    ///
    /// When the option is set to `false`, consecutive uppercase characters are allowed.
    /// `agentID` is so valid.
    ///
    /// Default: `true`
    ///
    /// ### requireAscii
    ///
    /// When this option is set to `true`, it forbids names that include non-ASCII characters.
    /// For instance,  when the option is set to `true`, `café` or `안녕하세요` will throw an error.
    ///
    /// When the option is set to `false`, a name may include non-ASCII characters.
    /// `café` and `안녕하세요` are so valid.
    ///
    /// Default: `true`
    ///
    /// ### match (Since v2.0.0)
    ///
    /// `match` defines a regular expression that the filename must match.
    /// If the regex has capturing groups, then the first capture is considered as the filename
    /// and the second one as file extensions separated by dots.
    ///
    /// For example, given the regular expression `%?(.+?)\.(.+)` and the filename `%index.d.ts`,
    /// the filename matches the regular expression with two captures: `index` and `d.ts`.
    /// The captures are checked against `filenameCases`.
    /// Note that we use the non-greedy quantifier `+?` to stop capturing as soon as we met the next character (`.`).
    /// If we use the greedy quantifier `+` instead, then the captures could be `index.d` and `ts`.
    ///
    /// The regular expression supports the following syntaxes:
    ///
    /// - Greedy quantifiers `*`, `?`, `+`, `{n}`, `{n,m}`, `{n,}`, `{m}`
    /// - Non-greedy quantifiers `*?`, `??`, `+?`, `{n}?`, `{n,m}?`, `{n,}?`, `{m}?`
    /// - Any character matcher `.`
    /// - Character classes `[a-z]`, `[xyz]`, `[^a-z]`
    /// - Alternations `|`
    /// - Capturing groups `()`
    /// - Non-capturing groups `(?:)`
    /// - Case-insensitive groups `(?i:)` and case-sensitive groups `(?-i:)`
    /// - A limited set of escaped characters including all special characters
    ///   and regular string escape characters `\f`, `\n`, `\r`, `\t`, `\v`.
    ///   Note that you can also escape special characters using character classes.
    ///   For example, `\$` and `[$]` are two valid patterns that escape `$`.
    ///
    /// ### filenameCases
    ///
    /// By default, the rule enforces that the filename  is either in [`camelCase`], [`kebab-case`], [`snake_case`], or equal to the name of one export in the file.
    ///
    /// You can enforce a stricter convention by setting `filenameCases` option.
    /// `filenameCases` accepts an array of cases among the following cases: [`camelCase`], [`kebab-case`], [`PascalCase`], [`snake_case`], and `export`.
    ///
    /// This option also applies to the file extensions.
    /// Extensions in lowercase are always allowed regardless of how `filenameCases` is set.
    ///
    /// [case]: https://en.wikipedia.org/wiki/Naming_convention_(programming)#Examples_of_multiple-word_identifier_formats
    /// [`camelCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`kebab-case`]: https://en.wikipedia.org/wiki/Letter_case#Kebab_case
    /// [`PascalCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`snake_case`]: https://en.wikipedia.org/wiki/Snake_case
    pub UseFilenamingConvention {
        version: "1.5.0",
        name: "useFilenamingConvention",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("filename-case")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

impl Rule for UseFilenamingConvention {
    type Query = SemanticServices;
    type State = FileNamingConventionState;
    type Signals = Option<Self::State>;
    type Options = Box<FilenamingConventionOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let file_name = ctx.file_path().file_name()?;
        let options = ctx.options();
        if options.require_ascii && !file_name.is_ascii() {
            return Some(FileNamingConventionState::Ascii);
        }
        let first_char = file_name.bytes().next()?;
        let (name, mut extensions) = if let Some(matching) = &options.matching {
            let Some(captures) = matching.captures(file_name) else {
                return Some(FileNamingConventionState::Match);
            };
            let mut captures = captures.iter().skip(1).flatten();
            let Some(first_capture) = captures.next() else {
                // Match without any capture implies a valid case
                return None;
            };
            let name = first_capture.as_str();
            if name.is_empty() {
                // Empty string are always valid.
                return None;
            }
            let split = captures.next().map_or("", |x| x.as_str()).split('.');
            (name, split)
        } else if matches!(first_char, b'(' | b'[') {
            // Support [Next.js](https://nextjs.org/docs/pages/building-your-application/routing/dynamic-routes#catch-all-segments),
            // [SolidStart](https://docs.solidjs.com/solid-start/building-your-application/routing#renaming-index),
            // [Nuxt](https://nuxt.com/docs/guide/directory-structure/server#catch-all-route),
            // and [Astro](https://docs.astro.build/en/guides/routing/#rest-parameters)
            // dynamic routes. Some examples:
            //
            // - `(slug).js`
            // - `[slug].js`
            // - `[[slug]].js`
            // - `[...slug].js`
            // - `[[...slug]].js`
            let count = if file_name.starts_with("[[") { 2 } else { 1 };
            let to_split = if first_char != b'(' && file_name[count..].starts_with("...") {
                &file_name[count + 3..]
            } else {
                &file_name[count..]
            };
            let mut split = to_split.split('.');
            let Some(name) = split.next() else {
                return Some(FileNamingConventionState::Filename);
            };
            let ends = if count == 2 {
                "]]"
            } else if first_char == b'[' {
                "]"
            } else {
                ")"
            };
            if !name.ends_with(ends)
                || !name[..name.len() - count]
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
            {
                return Some(FileNamingConventionState::Filename);
            }
            ("", split)
        } else {
            // Support UNIX hidden files (filenames starting with a dot).
            //
            // Support [Sveltekit](https://kit.svelte.dev/docs/routing#page) and
            // [Vike](https://vike.dev/route) routing conventions where page name starts with `+`.
            let file_name = if matches!(first_char, b'.' | b'+') {
                &file_name[1..]
            } else {
                file_name
            };
            let mut split = file_name.split('.');
            let Some(name) = split.next().filter(|name| !name.is_empty()) else {
                return Some(FileNamingConventionState::Filename);
            };
            (name, split)
        };
        let allowed_cases = options.filename_cases.cases;
        let allowed_extension_cases = allowed_cases | Case::Lower;
        // Check extension case
        if extensions.any(|extension| {
            !allowed_extension_cases.contains(Case::identify(extension, options.strict_case))
        }) {
            return Some(FileNamingConventionState::Extension);
        }
        if name.is_empty() {
            return None;
        }
        // Check filename case
        if !allowed_cases.is_empty() {
            let trimmed_name = name.trim_matches('_');
            let case = Case::identify(trimmed_name, options.strict_case);
            if (allowed_cases | Case::Uni).contains(case) {
                return None;
            }
        }
        if options.filename_cases.allow_export {
            // If no exported binding has the file name, then reports the filename
            ctx.model()
                .all_exported_bindings()
                .all(|exported_binding| {
                    exported_binding
                        .exports()
                        .filter_map(|export| match AnyJsIdentifierBinding::try_cast(export) {
                            Ok(id) => id.name_token().ok(),
                            Err(export) => match JsExportNamedSpecifier::cast(export.parent()?) {
                                Some(specifier) => specifier.exported_name().ok()?.value().ok(),
                                None => AnyJsIdentifierUsage::cast(export)?.value_token().ok(),
                            },
                        })
                        .all(|exported_name_token| exported_name_token.text_trimmed() != name)
                })
                .then_some(FileNamingConventionState::Filename)
        } else {
            Some(FileNamingConventionState::Filename)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let file_name = ctx.file_path().file_name()?;
        let options = ctx.options();
        match state {
            FileNamingConventionState::Ascii => {
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    None as Option<TextRange>,
                    markup! {
                        "The filename should be in ASCII because "<Emphasis>"requireAscii"</Emphasis>" is set to `true`."
                    },
                ).note(markup! {
                    "If you want to use non-ASCII filenames, then set the "<Emphasis>"requireAscii"</Emphasis>" option to `false`.\nSee the rule "<Hyperlink href="https://biomejs.dev/linter/rules/use-filenaming-convention#options">"options"</Hyperlink>" for more details."
                }))
            },
            FileNamingConventionState::Filename => {
                let allowed_cases = options.filename_cases.cases;
                let allowed_case_names = allowed_cases.into_iter().map(|case| case.to_string());
                let allowed_case_names = if options.filename_cases.allow_export {
                    allowed_case_names
                        .chain(["equal to the name of an export".to_string()])
                        .collect::<SmallVec<[_; 4]>>()
                        .join(" or ")
                } else {
                    allowed_case_names
                        .collect::<SmallVec<[_; 4]>>()
                        .join(" or ")
                };
                let mut split = file_name.split('.');
                let name = split.next()?;
                let name = if name.is_empty() {
                    // The filename starts with a dot
                    split.next()?
                } else if let Some(stripped_name) = name.strip_prefix('+') {
                    stripped_name
                } else {
                    name
                };
                let trimmed_name = name.trim_matches('_');
                let trimmed_info = if name != trimmed_name {
                    markup! {" trimmed as `"{trimmed_name}"`"}.to_owned()
                } else {
                    markup! {""}.to_owned()
                };
                if options.strict_case && options.filename_cases.cases.contains(Case::Camel) {
                    let case_type = Case::identify(trimmed_name, false);
                    let case_strict = Case::identify(trimmed_name, true);
                    if case_type == Case::Camel && case_strict == Case::Unknown {
                        return Some(RuleDiagnostic::new(
                            rule_category!(),
                            None as Option<TextRange>,
                            markup! {
                                "The filename"{trimmed_info}" is in camelCase, however, two consecutive uppercase characters are not allowed because `strictCase` is set to `true`."
                            },
                        ).note(markup! {
                            "If you want to use consecutive uppercase characters in camelCase then consider setting `strictCase` option to `false`.\n Check rule "<Hyperlink href="https://biomejs.dev/linter/rules/use-filenaming-convention#options">"options"</Hyperlink>" for more information."
                        }));
                    }
                }
                let mut suggested_filenames = allowed_cases
                    .into_iter()
                    .map(|case| case.convert(trimmed_name).into_boxed_str())
                    .filter(|new_trimmed_name| allowed_cases.contains(Case::identify(new_trimmed_name, options.strict_case)))
                    .collect::<SmallVec<[_; 4]>>();
                // We sort and deduplicate the suggested names
                suggested_filenames.sort();
                suggested_filenames.dedup();
                for i in 0..suggested_filenames.len() {
                    suggested_filenames[i] = file_name.replacen(trimmed_name, &suggested_filenames[i], 1).into_boxed_str();
                }
                let suggested_filenames = suggested_filenames.join("\n");
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    None as Option<TextRange>,
                    markup! {
                        "The filename"{trimmed_info}" should be in "<Emphasis>{allowed_case_names}</Emphasis>"."
                    },
                );
                if suggested_filenames.is_empty() {
                    return Some(diagnostic);
                }
                Some(diagnostic.note(markup! {
                    "The filename could be renamed to one of the following names:\n"{suggested_filenames}
                }))
            },
            FileNamingConventionState::Extension => {
                let allowed_cases = options.filename_cases.cases | Case::Lower;
                let allowed_case_names = allowed_cases.into_iter().map(|case| case.to_string());
                let allowed_case_names = allowed_case_names.collect::<SmallVec<[_; 4]>>().join(" or ");
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    None as Option<TextRange>,
                    markup! {
                        "The file extension should be in "<Emphasis>{allowed_case_names}</Emphasis>"."
                    },
                ))
            },
            FileNamingConventionState::Match => {
                let matching = options.matching.as_ref()?.as_str();
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    None as Option<TextRange>,
                    markup! {
                        "This filename should match the following regex "<Emphasis>"/"{matching}"/"</Emphasis>"."
                    },
                ))
            }
        }
    }
}

#[derive(Debug)]
pub enum FileNamingConventionState {
    /// The name is not in ASCII while `reuireAscii` is enabled.
    Ascii,
    /// The filename doesn't match the expected case
    Filename,
    /// An extension is not in lowercase
    Extension,
    /// The filename doesn't match the provided regex
    Match,
}

/// Rule's options.
#[derive(Clone, Debug, serde::Deserialize, Deserializable, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FilenamingConventionOptions {
    /// If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
    /// This does not affect other [Case].
    #[serde(default = "enabled", skip_serializing_if = "bool::clone")]
    pub strict_case: bool,

    /// If `false`, then non-ASCII characters are allowed.
    #[serde(default = "enabled", skip_serializing_if = "bool::clone")]
    pub require_ascii: bool,

    /// Regular expression to enforce
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    pub matching: Option<RestrictedRegex>,

    /// Allowed cases for file names.
    #[serde(default, skip_serializing_if = "is_default")]
    pub filename_cases: FilenameCases,
}

const fn enabled() -> bool {
    true
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

impl Default for FilenamingConventionOptions {
    fn default() -> Self {
        Self {
            strict_case: true,
            require_ascii: true,
            matching: None,
            filename_cases: FilenameCases::default(),
        }
    }
}

#[derive(
    Clone, Copy, Debug, Deserializable, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(
    from = "SmallVec<[FilenameCase; 5]>",
    into = "SmallVec<[FilenameCase; 5]>"
)]
#[deserializable(with_validator)]
pub struct FilenameCases {
    cases: Cases,
    /// `true` is the filename can be equal to the name of an export.
    allow_export: bool,
}
impl From<SmallVec<[FilenameCase; 5]>> for FilenameCases {
    fn from(values: SmallVec<[FilenameCase; 5]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<FilenameCase> for FilenameCases {
    fn from_iter<T: IntoIterator<Item = FilenameCase>>(values: T) -> Self {
        let mut result = Self {
            cases: Cases::empty(),
            allow_export: false,
        };
        for filename_case in values {
            if let Ok(case) = Case::try_from(filename_case) {
                result.cases |= case;
            } else {
                result.allow_export = true;
            }
        }
        result
    }
}
impl From<FilenameCases> for SmallVec<[FilenameCase; 5]> {
    fn from(value: FilenameCases) -> Self {
        let maybe_export = if value.allow_export {
            &[FilenameCase::Export][..]
        } else {
            &[]
        };
        value
            .cases
            .into_iter()
            .filter_map(|case| FilenameCase::try_from(case).ok())
            .chain(maybe_export.iter().copied())
            .collect()
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for FilenameCases {
    fn schema_name() -> String {
        "FilenameCases".to_string()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <std::collections::HashSet<FilenameCase>>::json_schema(gen)
    }
}
impl Default for FilenameCases {
    fn default() -> Self {
        Self {
            cases: Case::Camel | Case::Kebab | Case::Snake,
            allow_export: true,
        }
    }
}

impl biome_deserialize::DeserializableValidator for FilenameCases {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        name: &str,
        range: TextRange,
    ) -> bool {
        if !self.allow_export && self.cases.is_empty() {
            ctx.report(
                biome_deserialize::DeserializationDiagnostic::new(markup! {
                    ""<Emphasis>{name}</Emphasis>" cannot be an empty array."
                })
                .with_range(range),
            );
            false
        } else {
            true
        }
    }
}

/// Supported cases for file names.
#[derive(
    Clone, Copy, Debug, serde::Deserialize, Deserializable, Eq, Hash, PartialEq, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum FilenameCase {
    /// camelCase
    #[serde(rename = "camelCase")]
    Camel,

    /// Match an export name
    #[serde(rename = "export")]
    Export,

    /// kebab-case
    #[serde(rename = "kebab-case")]
    Kebab,

    /// PascalCase
    #[serde(rename = "PascalCase")]
    Pascal,

    /// snake_case
    #[serde(rename = "snake_case")]
    Snake,
}
impl FilenameCase {
    pub const ALLOWED_VARIANTS: &'static [&'static str] = &[
        "camelCase",
        "export",
        "kebab-case",
        "PascalCase",
        "snake_case",
    ];
}
impl FromStr for FilenameCase {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "camelCase" => Ok(Self::Camel),
            "export" => Ok(Self::Export),
            "kebab-case" => Ok(Self::Kebab),
            "PascalCase" => Ok(Self::Pascal),
            "snake_case" => Ok(Self::Snake),
            _ => Err("Value not supported for enum member case"),
        }
    }
}
impl TryFrom<FilenameCase> for Case {
    type Error = &'static str;

    fn try_from(case: FilenameCase) -> Result<Self, Self::Error> {
        match case {
            FilenameCase::Camel => Ok(Self::Camel),
            FilenameCase::Export => Err("`export` is not a valid case"),
            FilenameCase::Kebab => Ok(Self::Kebab),
            FilenameCase::Pascal => Ok(Self::Pascal),
            FilenameCase::Snake => Ok(Self::Snake),
        }
    }
}
impl TryFrom<Case> for FilenameCase {
    type Error = &'static str;

    fn try_from(value: Case) -> Result<Self, Self::Error> {
        match value {
            Case::Camel => Ok(FilenameCase::Camel),
            Case::Kebab => Ok(FilenameCase::Kebab),
            Case::Pascal => Ok(FilenameCase::Pascal),
            Case::Snake => Ok(FilenameCase::Snake),
            Case::Constant
            | Case::Lower
            | Case::Number
            | Case::NumberableCapital
            | Case::Uni
            | Case::Upper
            | Case::Unknown => Err("Unsupported case"),
        }
    }
}
