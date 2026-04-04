use crate::snapshot_builder::SnapshotBuilder;
use crate::utils::strip_rome_placeholders;
use biome_configuration::Configuration;
use biome_deserialize::json::deserialize_from_str;
use biome_diagnostics::print_diagnostic_to_string;
use biome_formatter::LineWidth;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_rowan::{TextRange, TextSize};
use biome_service::workspace::{
    ChangeFileParams, DocumentFileSource, FileContent, FormatFileParams, FormatRangeParams,
    GetFormatterIRParams, OpenFileParams, OpenProjectParams, OpenProjectResult,
    UpdateSettingsParams, server,
};
use camino::{Utf8Path, Utf8PathBuf};
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug)]
pub struct SpecTestFile<'a> {
    input_file: BiomePath,
    root_path: &'a Utf8Path,
    input_code: String,
    range_start_index: Option<usize>,
    range_end_index: Option<usize>,
}

impl<'a> SpecTestFile<'a> {
    pub fn try_from_file(input_file: &'a str, root_path: &'a Utf8Path) -> Option<Self> {
        if input_file.ends_with("options.json") {
            return None;
        }

        let spec_input_file = Utf8Path::new(input_file);
        assert!(
            spec_input_file.is_file(),
            "The input '{spec_input_file}' must exist and be a file.",
        );

        let mut input_file = BiomePath::new(input_file);
        let mut input_code = input_file.get_buffer_from_file();
        let (_, range_start_index, range_end_index) = strip_rome_placeholders(&mut input_code);

        Some(SpecTestFile {
            input_file,
            root_path,
            input_code,
            range_start_index,
            range_end_index,
        })
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

    fn range(&self) -> Option<TextRange> {
        match (self.range_start_index, self.range_end_index) {
            (Some(start), Some(end)) => Some(TextRange::new(
                TextSize::try_from(start).unwrap(),
                TextSize::try_from(end).unwrap(),
            )),
            _ => None,
        }
    }
}

pub struct SpecSnapshot<'a> {
    test_file: SpecTestFile<'a>,
    test_directory: Utf8PathBuf,
    initial_configuration: Configuration,
    document_file_source: Option<DocumentFileSource>,
}

impl<'a> SpecSnapshot<'a> {
    pub fn new(
        test_file: SpecTestFile<'a>,
        test_directory: &str,
        initial_configuration: Configuration,
    ) -> Self {
        let test_directory = Utf8PathBuf::from(test_directory);
        SpecSnapshot {
            test_file,
            test_directory,
            initial_configuration,
            document_file_source: None,
        }
    }

    pub fn with_document_file_source(mut self, source: DocumentFileSource) -> Self {
        self.document_file_source = Some(source);
        self
    }

    pub fn test(self) {
        let input_file = self.test_file.input_file().as_path();
        let fs = MemoryFileSystem::default();
        let workspace = server(Arc::new(fs), None);

        // 1. Open project
        let OpenProjectResult { project_key } = workspace
            .open_project(OpenProjectParams {
                path: BiomePath::new(""),
                open_uninitialized: true,
            })
            .unwrap();

        // 2. Apply initial configuration
        workspace
            .update_settings(UpdateSettingsParams {
                project_key,
                configuration: self.initial_configuration,
                workspace_directory: None,
                extended_configurations: vec![],
                module_graph_resolution_kind: Default::default(),
            })
            .unwrap();

        // 3. Load and apply options.json if present
        let options_path = self.test_directory.join("options.json");
        let options_json_content = if options_path.exists() {
            let mut options_path = BiomePath::new(&options_path);
            let content = options_path.get_buffer_from_file();
            let (test_options, diagnostics) =
                deserialize_from_str::<Configuration>(content.as_str()).consume();

            if !diagnostics.is_empty() {
                for diagnostic in diagnostics {
                    println!("{:?}", print_diagnostic_to_string(&diagnostic));
                }
                panic!("Configuration is invalid");
            }

            let options_config = test_options.unwrap_or_default();

            // Extract line_width and line_ending before merging
            let line_width = options_config
                .formatter
                .as_ref()
                .and_then(|f| f.line_width)
                .map(|w| w.value() as usize);
            let has_non_auto_line_ending = options_config
                .formatter
                .as_ref()
                .and_then(|f| f.line_ending)
                .is_some_and(|le| !le.is_auto());

            // Apply options on top of initial configuration
            workspace
                .update_settings(UpdateSettingsParams {
                    project_key,
                    configuration: options_config,
                    workspace_directory: None,
                    extended_configurations: vec![],
                    module_graph_resolution_kind: Default::default(),
                })
                .unwrap();

            Some((content, line_width, has_non_auto_line_ending))
        } else {
            None
        };

        let line_width = options_json_content
            .as_ref()
            .and_then(|(_, lw, _)| *lw)
            .unwrap_or(LineWidth::default().value() as usize);
        let has_non_auto_line_ending = options_json_content
            .as_ref()
            .is_some_and(|(_, _, has)| *has);

        // 4. Open file with input content
        let file_path = BiomePath::new(input_file);
        workspace
            .open_file(OpenFileParams {
                project_key,
                path: file_path.clone(),
                content: FileContent::FromClient {
                    content: self.test_file.input_code().to_string(),
                    version: 0,
                },
                document_file_source: self.document_file_source,
                persist_node_cache: false,
                inline_config: None,
            })
            .unwrap();

        // 5. Format — try without formatWithErrors first to detect parse errors
        let text_range = self.test_file.range();
        let is_range_format = text_range.is_some();

        let format_result = if let Some(range) = text_range {
            workspace.format_range(FormatRangeParams {
                project_key,
                path: file_path.clone(),
                range,
                inline_config: None,
            })
        } else {
            workspace.format_file(FormatFileParams {
                project_key,
                path: file_path.clone(),
                inline_config: None,
            })
        };

        let (printed, has_errors) = match format_result {
            Ok(printed) => (printed, false),
            Err(_) => {
                // File has parse errors — enable formatWithErrors and retry
                workspace
                    .update_settings(UpdateSettingsParams {
                        project_key,
                        configuration: Configuration {
                            formatter: Some(
                                biome_configuration::formatter::FormatterConfiguration {
                                    format_with_errors: Some(true.into()),
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        },
                        workspace_directory: None,
                        extended_configurations: vec![],
                        module_graph_resolution_kind: Default::default(),
                    })
                    .unwrap();

                let printed = if let Some(range) = text_range {
                    workspace
                        .format_range(FormatRangeParams {
                            project_key,
                            path: file_path.clone(),
                            range,
                            inline_config: None,
                        })
                        .expect("formatting range failed even with formatWithErrors")
                } else {
                    workspace
                        .format_file(FormatFileParams {
                            project_key,
                            path: file_path.clone(),
                            inline_config: None,
                        })
                        .expect("formatting failed even with formatWithErrors")
                };
                (printed, true)
            }
        };

        let mut output_code = if is_range_format {
            let range = printed
                .range()
                .expect("the result of format_range should have a range");
            let mut output_code = self.test_file.input_code().to_string();
            output_code.replace_range(Range::<usize>::from(range), printed.as_code());
            output_code
        } else {
            printed.as_code().to_string()
        };

        // 6. Idempotency check (only for non-range, error-free files)
        if !is_range_format && !has_errors {
            // Get IR of original formatting
            let original_ir = workspace
                .get_formatter_ir(GetFormatterIRParams {
                    project_key,
                    path: file_path.clone(),
                })
                .ok();

            // Change file content to the formatted output
            workspace
                .change_file(ChangeFileParams {
                    project_key,
                    path: file_path.clone(),
                    content: output_code.clone(),
                    version: 1,
                    inline_config: None,
                })
                .unwrap();

            // Re-format and compare
            let re_printed = match workspace.format_file(FormatFileParams {
                project_key,
                path: file_path.clone(),
                inline_config: None,
            }) {
                Ok(printed) => printed,
                Err(_) => return, // Re-formatting failed (e.g., file with errors), skip check
            };

            if re_printed.as_code() != output_code && !re_printed.as_code().is_empty() {
                // Show IR diff if available
                if let Some(original_ir) = &original_ir
                    && let Ok(reformat_ir) = workspace.get_formatter_ir(GetFormatterIRParams {
                        project_key,
                        path: file_path.clone(),
                    })
                {
                    let diff = similar_asserts::SimpleDiff::from_str(
                        original_ir,
                        &reformat_ir,
                        "input",
                        "output",
                    );
                    println!("{diff}");
                }

                similar_asserts::assert_eq!(
                    re_printed.as_code(),
                    output_code.as_str(),
                    "Formatter is not idempotent: re-formatting produced different output"
                );
            }
        }

        // 7. Apply line ending normalization
        if has_non_auto_line_ending {
            const CRLF_PATTERN: &str = "\r\n";
            const CR_PATTERN: &str = "\r";
            output_code = output_code
                .replace(CRLF_PATTERN, "<CRLF>\n")
                .replace(CR_PATTERN, "<CR>\n");
        }

        // 8. Build snapshot
        let mut snapshot_builder =
            SnapshotBuilder::new(input_file).with_input(self.test_file.input_code());

        if let Some((raw_json, _, _)) = &options_json_content {
            snapshot_builder = snapshot_builder.with_options_json(raw_json);
        }

        snapshot_builder = snapshot_builder
            .with_formatted(&output_code)
            .with_unimplemented(&printed)
            .with_lines_exceeding_max_width(&output_code, line_width);

        snapshot_builder.finish(self.test_file.relative_file_name());
    }
}
