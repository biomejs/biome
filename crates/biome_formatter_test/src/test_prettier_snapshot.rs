use crate::TestFormatLanguage;
use crate::check_reformat::{CheckReformat, ReformatError};
use crate::snapshot_builder::{SnapshotBuilder, SnapshotOutput};
use crate::utils::{PrettierDiff, get_prettier_diff, strip_prettier_placeholders};
use biome_formatter::{FormatLanguage, FormatOptions, Printed};
use biome_parser::AnyParse;
use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use std::{fmt, fs::read_to_string, ops::Range};

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
            "The input '{input_file}' must exist and be a file.",
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

enum FormatAttempt {
    Formatted(String),
    Failed(PrettierSnapshotError),
}

#[derive(Debug)]
enum PrettierSnapshotError {
    Format(String),
    Reformat(ReformatError),
}

impl fmt::Display for PrettierSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(message) => write!(f, "format: {message}"),
            Self::Reformat(error) => write!(f, "{error}"),
        }
    }
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

    fn format_for_snapshot(&self, parsed: &AnyParse) -> Option<FormatAttempt> {
        let syntax = parsed.syntax();
        let printed = match self.run_formatter(&syntax) {
            Some(Ok(printed)) => printed,
            Some(Err(error)) => return Some(FormatAttempt::Failed(error)),
            None => return None,
        };

        let formatted = match self.finish_formatted_output(parsed, &syntax, printed) {
            Ok(formatted) => formatted,
            Err(error) => return Some(FormatAttempt::Failed(error)),
        };

        Some(FormatAttempt::Formatted(
            formatted.replace(BIOME_IGNORE, PRETTIER_IGNORE),
        ))
    }

    fn run_formatter(
        &self,
        syntax: &biome_rowan::SyntaxNode<L::ServiceLanguage>,
    ) -> Option<Result<Printed, PrettierSnapshotError>> {
        match self.test_file.range() {
            (Some(start), Some(end)) => self.format_range_once(syntax, start, end),
            _ => Some(self.format_node_once(syntax)),
        }
    }

    fn format_range_once(
        &self,
        syntax: &biome_rowan::SyntaxNode<L::ServiceLanguage>,
        start: usize,
        end: usize,
    ) -> Option<Result<Printed, PrettierSnapshotError>> {
        // Skip reversed range tests because TextRange cannot represent them.
        if end < start {
            return None;
        }

        Some(
            self.language
                .format_range(
                    self.format_language.clone(),
                    syntax,
                    TextRange::new(
                        TextSize::try_from(start).unwrap(),
                        TextSize::try_from(end).unwrap(),
                    ),
                )
                .map_err(|err| PrettierSnapshotError::Format(err.to_string())),
        )
    }

    fn format_node_once(
        &self,
        syntax: &biome_rowan::SyntaxNode<L::ServiceLanguage>,
    ) -> Result<Printed, PrettierSnapshotError> {
        self.language
            .format_node(self.format_language.clone(), syntax)
            .map_err(|err| PrettierSnapshotError::Format(err.to_string()))
            .and_then(|formatted| {
                formatted
                    .print()
                    .map_err(|err| PrettierSnapshotError::Format(err.to_string()))
            })
    }

    fn finish_formatted_output(
        &self,
        parsed: &AnyParse,
        syntax: &biome_rowan::SyntaxNode<L::ServiceLanguage>,
        printed: Printed,
    ) -> Result<String, PrettierSnapshotError> {
        match self.test_file.range() {
            (Some(_), Some(_)) => Ok(self.apply_range_format(printed)),
            _ => self.finish_file_format(parsed, syntax, printed),
        }
    }

    fn apply_range_format(&self, printed: Printed) -> String {
        let range = printed
            .range()
            .expect("the result of format_range should have a range");

        let formatted = printed.as_code();
        let mut output_code = self.test_file.parse_input.clone();
        output_code.replace_range(Range::<usize>::from(range), formatted);
        output_code
    }

    fn finish_file_format(
        &self,
        parsed: &AnyParse,
        syntax: &biome_rowan::SyntaxNode<L::ServiceLanguage>,
        printed: Printed,
    ) -> Result<String, PrettierSnapshotError> {
        let formatted = printed.into_code();

        if !parsed.has_errors() {
            self.verify_reformat(syntax, &formatted)?;
        }

        Ok(formatted)
    }

    fn verify_reformat(
        &self,
        syntax: &biome_rowan::SyntaxNode<L::ServiceLanguage>,
        formatted: &str,
    ) -> Result<(), PrettierSnapshotError> {
        let check_reformat = CheckReformat::new(
            syntax,
            formatted,
            self.test_file.file_name(),
            &self.language,
            self.format_language.clone(),
        );

        check_reformat
            .check_reformat()
            .map_err(PrettierSnapshotError::Reformat)
    }

    pub fn test(self) {
        let parsed = self.language.parse(self.test_file().parse_input());
        let attempt = match self.format_for_snapshot(&parsed) {
            Some(attempt) => attempt,
            None => return,
        };

        let relative_file_name = self.test_file().relative_file_name();
        let input_file = self.test_file().input_file();

        match attempt {
            FormatAttempt::Formatted(formatted) => {
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
            FormatAttempt::Failed(error) => {
                SnapshotBuilder::new(input_file)
                    .with_input(&self.test_file().input_code)
                    .with_errors(&parsed, &self.test_file().parse_input)
                    .with_error(&error.to_string())
                    .finish(relative_file_name);
            }
        }
    }

    fn test_file(&self) -> &PrettierTestFile<'_> {
        &self.test_file
    }
}
