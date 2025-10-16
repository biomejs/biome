use std::str::FromStr;

use anyhow::{Result, bail};
use biome_analyze::AnalyzerOptions;
use biome_configuration::Configuration;
use biome_fs::BiomePath;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

/// Represents a single code block used for evaluating doc tests.
///
/// Code blocks can be either evaluated stand-alone as a self-contained test
/// snippet, contain configuration to be applied to another code block, or be
/// part of an in-memory file system used to run examples for rules with
/// multi-file analysis.
#[derive(Default)]
pub struct CodeBlock {
    /// The language tag of this code block.
    pub tag: String,

    /// Whether this is an invalid example that should trigger a diagnostic.
    pub expect_diagnostic: bool,

    /// Whether to expect a code diff.
    pub expect_diff: bool,

    /// If a file path is provided using the `file=<path>` attribute, it will be
    /// used as the code block's title in the MDX output. It will also be used
    /// in generating an in-memory file system for multi-file analysis.
    file_path: Option<String>,

    /// Whether to ignore this code block.
    pub ignore: bool,

    /// Whether this is a block of configuration options instead of a
    /// valid/invalid code example, and if yes, how that block of configuration
    /// options should be parsed:
    pub options: OptionsParsingMode,

    /// Whether to use the last code block that was marked with `options` as the
    /// configuration settings for this code block.
    pub use_options: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OptionsParsingMode {
    /// This code block does not contain configuration options.
    #[default]
    NoOptions,

    /// This code block contains the options for a single rule only.
    RuleOptionsOnly,

    /// This code block contains JSON that adheres to the full `biome.json`
    /// schema.
    FullConfiguration,
}

impl CodeBlock {
    pub fn create_analyzer_options<L>(
        &self,
        config: Option<Configuration>,
    ) -> Result<AnalyzerOptions>
    where
        L: ServiceLanguage,
    {
        let mut settings = Settings::default();

        if self.use_options {
            // Load settings from the preceding `json,options` block if requested
            let Some(config) = config else {
                bail!(
                    "Code blocks tagged with 'use_options' must be preceded by a valid 'json,options' code block."
                );
            };

            settings.merge_with_configuration(config, None)?;
        }

        let language_settings = &L::lookup_settings(&settings.languages).linter;
        let environment = L::resolve_environment(&settings);
        let suppression_reason = None;

        Ok(L::resolve_analyzer_options(
            &settings,
            language_settings,
            environment,
            &BiomePath::new(self.file_path()),
            &self.document_file_source(),
            suppression_reason,
        ))
    }

    pub fn document_file_source(&self) -> DocumentFileSource {
        DocumentFileSource::from_extension(&self.tag, false)
    }

    /// Returns the block's file path, but only if one was set explicitly using
    /// the `file=<path>` attribute.
    pub fn explicit_file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    /// Returns the block's file path, with a fallback in case one was not set
    /// explicitly.
    pub fn file_path(&self) -> String {
        self.explicit_file_path()
            .map_or_else(|| format!("code-block.{}", self.tag), ToString::to_string)
    }
}

impl FromStr for CodeBlock {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let tokens = input
            .split([',', ' ', '\t'])
            .map(str::trim)
            .filter(|token| !token.is_empty());

        let mut code_block = Self::default();

        for token in tokens {
            match token {
                "expect_diagnostic" => code_block.expect_diagnostic = true,
                "expect_diff" => code_block.expect_diff = true,
                "full_options" => code_block.options = OptionsParsingMode::FullConfiguration,
                "ignore" => code_block.ignore = true,
                "options" => code_block.options = OptionsParsingMode::RuleOptionsOnly,
                "use_options" => code_block.use_options = true,
                _ => {
                    if let Some(path) = token.strip_prefix("file=") {
                        if path.is_empty() {
                            bail!("The 'file' attribute must be followed by a file path");
                        }

                        code_block.file_path = Some(normalize_file_path(path));
                    } else {
                        if DocumentFileSource::from_extension(token, false)
                            == DocumentFileSource::Unknown
                        {
                            bail!("Unrecognised attribute in code block: {token}");
                        }

                        if code_block.document_file_source() != DocumentFileSource::Unknown {
                            bail!(
                                "Only one language tag is accepted per code block. Found '{}' and '{}'",
                                code_block.tag,
                                token
                            );
                        }

                        code_block.tag = token.to_string();
                    }
                }
            }
        }

        Ok(code_block)
    }
}

/// Normalizes a file path to an absolute path for easier module graph path
/// resolution.
fn normalize_file_path(path: &str) -> String {
    let path = path.trim_start_matches("./").trim_start_matches("../");
    format!("/{path}")
}
