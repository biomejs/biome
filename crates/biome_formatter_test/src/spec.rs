use crate::check_reformat::CheckReformat;
use crate::snapshot_builder::{SnapshotBuilder, SnapshotOutput};
use crate::utils::strip_rome_placeholders;
use crate::TestFormatLanguage;
use biome_configuration::Configuration;
use biome_console::EnvConsole;
use biome_deserialize::json::deserialize_from_str;
use biome_diagnostics::print_diagnostic_to_string;
use biome_formatter::{FormatLanguage, FormatOptions, Printed};
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_rowan::{TextRange, TextSize};
use biome_service::projects::ProjectKey;
use biome_service::settings::Settings;
use biome_service::workspace::{
    DocumentFileSource, FeaturesBuilder, OpenProjectParams, SupportsFeatureParams,
    UpdateSettingsParams,
};
use biome_service::App;
use camino::{Utf8Path, Utf8PathBuf};
use std::ops::Range;

#[derive(Debug)]
pub struct SpecTestFile<'a> {
    input_file: BiomePath,
    root_path: &'a Utf8Path,

    input_code: String,

    range_start_index: Option<usize>,
    range_end_index: Option<usize>,
}

impl<'a> SpecTestFile<'a> {
    pub fn try_from_file(
        input_file: &'a str,
        root_path: &'a Utf8Path,
        settings_fn: impl FnOnce(ProjectKey) -> Option<UpdateSettingsParams>,
    ) -> Option<SpecTestFile<'a>> {
        if input_file.ends_with("options.json") {
            return None;
        }
        let mut console = EnvConsole::default();
        let app = App::with_console(&mut console);
        let file_path = &input_file;
        let spec_input_file = Utf8Path::new(input_file);

        assert!(
            spec_input_file.is_file(),
            "The input '{spec_input_file}' must exist and be a file.",
        );

        let project_key = app
            .workspace
            .open_project(OpenProjectParams {
                path: BiomePath::new(""),
                open_uninitialized: true,
            })
            .unwrap();

        if let Some(settings) = settings_fn(project_key) {
            app.workspace.update_settings(settings).unwrap();
        }
        let mut input_file = BiomePath::new(file_path);
        let can_format = app
            .workspace
            .file_features(SupportsFeatureParams {
                project_key,
                path: input_file.clone(),
                features: FeaturesBuilder::new().with_formatter().build(),
            })
            .unwrap();

        if can_format.supports_format() {
            let mut input_code = input_file.get_buffer_from_file();

            let (_, range_start_index, range_end_index) = strip_rome_placeholders(&mut input_code);

            Some(SpecTestFile {
                input_file,
                root_path,

                input_code,

                range_start_index,
                range_end_index,
            })
        } else {
            None
        }
    }

    pub fn input_code(&self) -> &str {
        &self.input_code
    }

    pub fn file_name(&self) -> &str {
        self.input_file.file_name().unwrap()
    }

    pub fn input_file(&self) -> &BiomePath {
        &self.input_file
    }

    pub fn relative_file_name(&self) -> &str {
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

    fn range(&self) -> (Option<usize>, Option<usize>) {
        (self.range_start_index, self.range_end_index)
    }
}

pub struct SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    test_file: SpecTestFile<'a>,
    test_directory: Utf8PathBuf,
    language: L,
    format_language: L::FormatLanguage,
}

impl<'a, L> SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        test_file: SpecTestFile<'a>,
        test_directory: &str,
        language: L,
        format_language: L::FormatLanguage,
    ) -> Self {
        let test_directory = Utf8PathBuf::from(test_directory);

        SpecSnapshot {
            test_file,
            test_directory,
            language,
            format_language,
        }
    }

    fn formatted(
        &self,
        parsed: &AnyParse,
        format_language: L::FormatLanguage,
    ) -> (String, Printed) {
        let has_errors = parsed.has_errors();
        let syntax = parsed.syntax();

        let range = self.test_file.range();

        let result = match range {
            (Some(start), Some(end)) => self.language.format_range(
                format_language.clone(),
                &syntax,
                TextRange::new(
                    TextSize::try_from(start).unwrap(),
                    TextSize::try_from(end).unwrap(),
                ),
            ),
            _ => self
                .language
                .format_node(format_language.clone(), &syntax)
                .map(|formatted| formatted.print().unwrap()),
        };
        let formatted = result.expect("formatting failed");

        let output_code = match range {
            (Some(_), Some(_)) => {
                let range = formatted
                    .range()
                    .expect("the result of format_range should have a range");

                let mut output_code = self.test_file.input_code.clone();
                output_code.replace_range(Range::<usize>::from(range), formatted.as_code());

                // Check if output code is a valid syntax
                let parsed = self.language.parse(&output_code);

                if parsed.has_errors() {
                    panic!(
                        "{:?} format range produced an invalid syntax tree: {:?}",
                        self.test_file.input_file, output_code
                    )
                }

                output_code
            }
            _ => {
                let output_code = formatted.as_code();

                if !has_errors {
                    let check_reformat = CheckReformat::new(
                        &syntax,
                        output_code,
                        self.test_file.file_name(),
                        &self.language,
                        format_language,
                    );
                    check_reformat.check_reformat();
                }

                output_code.to_string()
            }
        };

        (output_code, formatted)
    }

    pub fn test(self) {
        let input_file = self.test_file().input_file().as_path();

        let mut snapshot_builder = SnapshotBuilder::new(input_file)
            .with_input(self.test_file.input_code())
            .with_separator()
            .with_multiple_outputs();

        let parsed = self.language.parse(self.test_file.input_code());

        let (output_code, printed) = self.formatted(&parsed, self.format_language.clone());

        let max_width = self.format_language.options().line_width().value() as usize;

        snapshot_builder = snapshot_builder
            .with_output_and_options(
                SnapshotOutput::new(&output_code).with_index(1),
                self.format_language.options().clone(),
            )
            .with_unimplemented(&printed)
            .with_lines_exceeding_max_width(&output_code, max_width);

        let options_path = self.test_directory.join("options.json");
        if options_path.exists() {
            let mut options_path = BiomePath::new(&options_path);

            let mut settings = Settings::default();
            // SAFETY: we checked its existence already, we assume we have rights to read it
            let (test_options, diagnostics) =
                deserialize_from_str::<Configuration>(options_path.get_buffer_from_file().as_str())
                    .consume();
            settings
                .merge_with_configuration(test_options.unwrap_or_default(), None, None, &[])
                .unwrap();

            if !diagnostics.is_empty() {
                for diagnostic in diagnostics {
                    println!("{:?}", print_diagnostic_to_string(&diagnostic));
                }

                panic!("Configuration is invalid");
            }

            let format_language = self
                .language
                .to_format_language(&settings, &DocumentFileSource::from_path(input_file));

            let (mut output_code, printed) = self.formatted(&parsed, format_language.clone());

            let max_width = format_language.options().line_width().value() as usize;

            // There are some logs that print different line endings, and we can't snapshot those
            // otherwise we risk automatically having them replaced with LF by git.
            //
            // This is a workaround, and it might not work for all cases.
            const CRLF_PATTERN: &str = "\r\n";
            const CR_PATTERN: &str = "\r";
            output_code = output_code
                .replace(CRLF_PATTERN, "<CRLF>\n")
                .replace(CR_PATTERN, "<CR>\n");

            snapshot_builder = snapshot_builder
                .with_output_and_options(
                    SnapshotOutput::new(&output_code).with_index(1),
                    format_language.options(),
                )
                .with_unimplemented(&printed)
                .with_lines_exceeding_max_width(&output_code, max_width);
        }

        snapshot_builder.finish(self.test_file.relative_file_name());
    }

    fn test_file(&self) -> &SpecTestFile {
        &self.test_file
    }
}
