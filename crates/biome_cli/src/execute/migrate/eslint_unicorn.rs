/// Configuration related to the
/// [Unicorn Eslint plugin](https://github.com/sindresorhus/eslint-plugin-unicorn).
///
/// ALso, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::lint::style::use_filenaming_convention;
use smallvec::SmallVec;

#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct FilenameCaseOptions {
    case: FilenameCase,
    cases: FilenameCases,
    ignore: Vec<String>,
    multiple_file_extensions: bool,
}
impl From<FilenameCaseOptions> for use_filenaming_convention::FilenamingConventionOptions {
    fn from(val: FilenameCaseOptions) -> Self {
        let filename_cases: Option<use_filenaming_convention::FilenameCases> = val.cases.into();
        use_filenaming_convention::FilenamingConventionOptions {
            strict_case: true,
            require_ascii: true,
            matching: None,
            filename_cases: filename_cases.unwrap_or_else(|| {
                use_filenaming_convention::FilenameCases::from_iter([val.case.into()])
            }),
        }
    }
}
#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) enum FilenameCase {
    #[default]
    #[deserializable(rename = "kebabCase")]
    Kebab,
    #[deserializable(rename = "camelCase")]
    Camel,
    #[deserializable(rename = "snakeCase")]
    Snake,
    #[deserializable(rename = "pascalCase")]
    Pascal,
}
impl From<FilenameCase> for use_filenaming_convention::FilenameCase {
    fn from(val: FilenameCase) -> Self {
        match val {
            FilenameCase::Kebab => use_filenaming_convention::FilenameCase::Kebab,
            FilenameCase::Camel => use_filenaming_convention::FilenameCase::Camel,
            FilenameCase::Snake => use_filenaming_convention::FilenameCase::Snake,
            FilenameCase::Pascal => use_filenaming_convention::FilenameCase::Pascal,
        }
    }
}
#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct FilenameCases {
    kebab_case: bool,
    camel_case: bool,
    snake_case: bool,
    pascal_case: bool,
}
impl From<FilenameCases> for Option<use_filenaming_convention::FilenameCases> {
    fn from(val: FilenameCases) -> Self {
        let mut cases: SmallVec<[use_filenaming_convention::FilenameCase; 4]> = SmallVec::new();
        if val.kebab_case {
            cases.push(use_filenaming_convention::FilenameCase::Kebab);
        }
        if val.camel_case {
            cases.push(use_filenaming_convention::FilenameCase::Camel);
        }
        if val.snake_case {
            cases.push(use_filenaming_convention::FilenameCase::Snake);
        }
        if val.pascal_case {
            cases.push(use_filenaming_convention::FilenameCase::Pascal);
        }
        if cases.is_empty() {
            None
        } else {
            Some(use_filenaming_convention::FilenameCases::from_iter(cases))
        }
    }
}
