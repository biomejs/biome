use crate::check_reformat::CheckReformat;
use crate::snapshot_builder::{SnapshotBuilder, SnapshotOutput};
use crate::utils::{get_prettier_diff, strip_prettier_placeholders, PrettierDiff};
use crate::TestFormatLanguage;
use biome_formatter::{FormatLanguage, FormatOptions};
use biome_parser::AnyParse;
use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use std::{fs::read_to_string, ops::Range};

const PRETTIER_IGNORE: &str = "prettier-ignore";
const BIOME_IGNORE: &str = "biome-ignore format: prettier ignore";

pub struct PrettierTestFile<'a> {
    input_file: &'static Utf8Path,
    root_path: &'a Utf8Path,

    input_code: String,
    parse_input: String,

    range_start_index: Option<usize>,
    range_end_index: Option<usize>,
}

impl<'a> PrettierTestFile<'a> {
    pub fn new(input: &'static str, root_path: &'a Utf8Path) -> Self {
        let input_file = Utf8Path::new(input);

        assert!(
            input_file.is_file(),
            "The input '{}' must exist and be a file.",
            input_file
        );

        let mut input_code = read_to_string(input_file)
            .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

        let (_, range_start_index, range_end_index) = strip_prettier_placeholders(&mut input_code);
        let parse_input = input_code.replace(PRETTIER_IGNORE, BIOME_IGNORE);

        PrettierTestFile {
            input_file,
            root_path,

            input_code,
            parse_input,

            range_start_index,
            range_end_index,
        }
    }

    fn range(&self) -> (Option<usize>, Option<usize>) {
        (self.range_start_index, self.range_end_index)
    }

    pub fn input_file(&self) -> &Utf8Path {
        self.input_file
    }

    pub fn parse_input(&self) -> &str {
        self.parse_input.as_str()
    }

    pub fn file_name(&self) -> &str {
        self.input_file
            .file_name()
            .expect("failed to get file name")
    }

    pub fn file_extension(&self) -> &str {
        self.input_file
            .extension()
            .expect("failed to get file extension")
    }

    pub fn relative_file_name(&self) -> &'static str {
        self.input_file
            .strip_prefix(self.root_path)
            .unwrap_or_else(|_| {
                panic!(
                    "failed to strip prefix {:?} from {:?}",
                    self.root_path, self.input_file
                )
            })
            .as_str()
    }
}

pub struct PrettierSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    test_file: PrettierTestFile<'a>,
    language: L,
    // options: <L::ServiceLanguage as ServiceLanguage>::FormatOptions,
    format_language: L::FormatLanguage,
}

impl<'a, L> PrettierSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        test_file: PrettierTestFile<'a>,
        language: L,
        format_language: L::FormatLanguage,
    ) -> Self {
        PrettierSnapshot {
            test_file,
            language,
            format_language,
        }
    }

    fn formatted(&self, parsed: &AnyParse) -> Option<String> {
        let has_errors = parsed.has_errors();
        let syntax = parsed.syntax();

        let range = self.test_file.range();

        let result = match range {
            (Some(start), Some(end)) => {
                // Skip the reversed range tests as its impossible
                // to create a reversed TextRange anyway
                if end < start {
                    return None;
                }

                self.language.format_range(
                    self.format_language.clone(),
                    &syntax,
                    TextRange::new(
                        TextSize::try_from(start).unwrap(),
                        TextSize::try_from(end).unwrap(),
                    ),
                )
            }
            _ => self
                .language
                .format_node(self.format_language.clone(), &syntax)
                .map(|formatted| formatted.print().unwrap()),
        };

        let formatted = result.expect("formatting failed");
        let formatted = match range {
            (Some(_), Some(_)) => {
                let range = formatted
                    .range()
                    .expect("the result of format_range should have a range");

                let formatted = formatted.as_code();
                let mut output_code = self.test_file.parse_input.clone();
                output_code.replace_range(Range::<usize>::from(range), formatted);
                output_code
            }
            _ => {
                let formatted = formatted.into_code();

                if !has_errors {
                    let check_reformat = CheckReformat::new(
                        &syntax,
                        &formatted,
                        self.test_file.file_name(),
                        &self.language,
                        self.format_language.clone(),
                    );
                    check_reformat.check_reformat();
                }

                formatted
            }
        };

        let formatted = formatted.replace(BIOME_IGNORE, PRETTIER_IGNORE);

        Some(formatted)
    }

    pub fn test(self) {
        let parsed = self.language.parse(self.test_file().parse_input());

        let formatted = match self.formatted(&parsed) {
            Some(formatted) => formatted,
            None => return,
        };

        let relative_file_name = self.test_file().relative_file_name();
        let input_file = self.test_file().input_file();

        let prettier_diff = get_prettier_diff(input_file, relative_file_name, &formatted);

        let prettier_diff = match prettier_diff {
            PrettierDiff::Diff(prettier_diff) => prettier_diff,
            PrettierDiff::Same => return,
        };

        let mut builder = SnapshotBuilder::new(input_file)
            .with_input(&self.test_file().input_code)
            .with_prettier_diff(&prettier_diff)
            .with_output(SnapshotOutput::new(&formatted))
            .with_errors(&parsed, &self.test_file().parse_input);

        let max_width = self.format_language.options().line_width().value() as usize;
        builder = builder.with_lines_exceeding_max_width(&formatted, max_width);

        builder.finish(relative_file_name);
    }

    fn test_file(&self) -> &PrettierTestFile {
        &self.test_file
    }
}
