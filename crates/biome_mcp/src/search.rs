//! The `search` tool, which finds code by its syntax with GritQL patterns.
//!
//! The tool searches the files that the project scan indexed, so the project's
//! configuration decides which files are searched, and the workspace's file
//! watcher keeps their content current while the agent edits them.

use crate::project::Project;
use biome_diagnostics::display::SourceFile;
use biome_diagnostics::{PrintDescription, SourceCode};
use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    DropPatternParams, FeaturesBuilder, FileFeaturesResult, GetFileContentParams,
    GetProjectFilesParams, ParsePatternParams, PatternId, SearchCapture, SearchLanguage,
    SearchPatternParams, SupportsFeatureParams,
};
use biome_service::{Workspace, WorkspaceError};
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use schemars::JsonSchema;
use serde::Deserialize;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// The number of matches reported when the agent doesn't set `maxMatches`.
const DEFAULT_MAX_MATCHES: usize = 1000;

/// The maximum number of characters of a line of code in the report.
const MAX_LINE_CHARS: usize = 200;

/// The maximum number of lines of code shown for a match with the
/// [SearchOutput::Matches] output.
const MAX_MATCH_LINES: usize = 5;

/// The maximum number of characters of a captured value with the
/// [SearchOutput::Captures] output.
const MAX_CAPTURE_CHARS: usize = 100;

/// The arguments of the `search` tool.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    /// The GritQL pattern to find. Write the code between backticks, and use
    /// `$name` variables for the parts that vary, for example
    /// `` `console.log($message)` `` or `` `import $names from "react"` ``.
    pub pattern: String,

    /// The files or directories to search, relative to the project root.
    /// Defaults to the whole project.
    #[serde(default)]
    pub paths: Vec<String>,

    /// The language of the pattern and of the files to search. Defaults to
    /// `javascript`, which also covers TypeScript and JSX.
    #[serde(default)]
    pub language: PatternLanguage,

    /// What the report shows. Defaults to `lines`.
    #[serde(default)]
    pub output: SearchOutput,

    /// The maximum number of matches to report, or of files with the `files`
    /// output, from 1. Defaults to 1000.
    pub max_matches: Option<usize>,
}

/// What the report of a search shows. Each form lists the matches grouped by
/// file, except `files`.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SearchOutput {
    /// The files that contain matches, with the number of matches in each.
    Files,
    /// The line and column where each match starts, with that line of code.
    #[default]
    Lines,
    /// The line and column where each match starts, with the code of the
    /// match, up to 5 lines.
    Matches,
    /// The line and column where each match starts, with the code that each
    /// `$variable` of the pattern captured, on one line. A match without
    /// captured code shows its line of code instead.
    Captures,
}

/// The language of a search pattern.
#[derive(Clone, Copy, Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PatternLanguage {
    /// JavaScript, TypeScript, and JSX files.
    #[default]
    Javascript,
    /// CSS files.
    Css,
    /// JSON files.
    Json,
}

impl PatternLanguage {
    const fn search_language(self) -> SearchLanguage {
        match self {
            Self::Javascript => SearchLanguage::Js,
            Self::Css => SearchLanguage::Css,
            Self::Json => SearchLanguage::Json,
        }
    }

    const fn can_search(self, file_source: &DocumentFileSource) -> bool {
        matches!(
            (self, file_source),
            (Self::Javascript, DocumentFileSource::Js(_))
                | (Self::Css, DocumentFileSource::Css(_))
                | (Self::Json, DocumentFileSource::Json(_))
        )
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Javascript => "JavaScript",
            Self::Css => "CSS",
            Self::Json => "JSON",
        }
    }
}

/// Searches `project` with `params` and returns the report for the agent: the
/// matches grouped by file, or the files, in the form that `params.output` asks
/// for, then a summary.
///
/// ## Errors
///
/// Returns a message for the agent when the configuration has errors, when the
/// pattern is invalid, or when a path is outside the project root or doesn't
/// exist.
pub(crate) fn search(project: &Project, params: SearchParams) -> Result<String, String> {
    let project_key = project.ensure_loaded()?;
    let workspace = project.workspace();
    let paths = resolve_paths(project.root(), &params.paths)?;
    if let Some(missing) = params
        .paths
        .iter()
        .zip(&paths)
        .find(|(_, path)| workspace.fs().path_kind(path).is_err())
    {
        return Err(format!("The path `{}` doesn't exist.", missing.0));
    }
    let max_matches = params.max_matches.unwrap_or(DEFAULT_MAX_MATCHES).max(1);

    let project_files = workspace
        .get_project_files(GetProjectFilesParams { project_key })
        .map_err(|error| PrintDescription(&error).to_string())?
        .paths;
    let pattern = workspace
        .parse_pattern(ParsePatternParams {
            pattern: params.pattern,
            default_language: params.language.search_language(),
        })
        .map_err(|error| format!("The pattern is invalid: {}", PrintDescription(&error)))?
        .pattern_id;

    let mut report = Report::default();
    // The number of matches that the report still has room for. The `files`
    // output only counts matches.
    let mut room = match params.output {
        SearchOutput::Files => 0,
        _ => max_matches,
    };
    for path in project_files.iter().filter(|path| {
        paths.iter().any(|requested| path.starts_with(requested))
            && is_searchable(project, project_key, params.language, path)
    }) {
        report.searched_files += 1;
        // A panic while searching one file, for example in the parser, is
        // reported for that file instead of failing the whole search.
        match catch_unwind(AssertUnwindSafe(|| {
            search_file(project, project_key, &pattern, path, params.output, room)
        })) {
            Ok(Ok(Some(found))) => {
                room -= found.matches.len();
                report.files.push(found);
            }
            Ok(Ok(None)) => {}
            Ok(Err(error)) => report.errors.push(format!(
                "{}: {}",
                relative_path(project, path),
                PrintDescription(&error)
            )),
            Err(_) => report.errors.push(format!(
                "{}: Biome failed unexpectedly while searching this file.",
                relative_path(project, path)
            )),
        }
    }

    // Dropping the pattern only frees memory, so a failure doesn't affect the
    // report.
    let _ = workspace.drop_pattern(DropPatternParams { pattern });

    Ok(report.render(params.output, max_matches, params.language))
}

/// Returns the paths to search: `paths` resolved against `root`, or `root`
/// itself when `paths` is empty.
fn resolve_paths(root: &Utf8Path, paths: &[String]) -> Result<Vec<Utf8PathBuf>, String> {
    if paths.is_empty() {
        return Ok(vec![root.to_path_buf()]);
    }

    paths
        .iter()
        .map(|path| {
            let resolved = root.join(path);
            let escapes_root = !resolved.starts_with(root)
                || resolved
                    .components()
                    .any(|component| component == Utf8Component::ParentDir);
            if escapes_root {
                Err(format!(
                    "The path `{path}` is outside the project root `{root}`."
                ))
            } else {
                Ok(resolved)
            }
        })
        .collect()
}

/// Returns whether the file at `path` is written in `language` and its
/// configuration, including nested configurations and overrides, lets Biome
/// search it.
fn is_searchable(
    project: &Project,
    project_key: ProjectKey,
    language: PatternLanguage,
    path: &BiomePath,
) -> bool {
    let file_source = DocumentFileSource::from_path(path, false);
    language.can_search(&file_source)
        && project
            .workspace()
            .file_features(SupportsFeatureParams {
                project_key,
                path: path.clone(),
                features: FeaturesBuilder::new().with_search().build(),
                inline_config: None,
                skip_ignore_check: false,
                not_requested_features: FeaturesBuilder::new().with_all().without_search().build(),
            })
            .is_ok_and(|FileFeaturesResult { features_supported }| {
                features_supported.supports_search()
            })
}

/// Searches the workspace's document of the file at `path`, and returns its
/// matches if there are any: all of them are counted, and the first `room`
/// ones are prepared for the report in the form that `output` asks for.
fn search_file(
    project: &Project,
    project_key: ProjectKey,
    pattern: &PatternId,
    path: &BiomePath,
    output: SearchOutput,
    room: usize,
) -> Result<Option<FileMatches>, WorkspaceError> {
    let workspace = project.workspace();
    let results = workspace.search_pattern(SearchPatternParams {
        project_key,
        path: path.clone(),
        pattern: pattern.clone(),
        include_captures: output == SearchOutput::Captures && room > 0,
    })?;
    if results.matches.is_empty() {
        return Ok(None);
    }

    let mut found = FileMatches {
        path: relative_path(project, path),
        count: results.matches.len(),
        matches: Vec::new(),
    };
    if room == 0 {
        return Ok(Some(found));
    }

    let content = workspace.get_file_content(GetFileContentParams {
        project_key,
        path: path.clone(),
    })?;
    let source = SourceFile::new(SourceCode {
        text: content.as_str(),
        line_starts: None,
    });
    found.matches = results
        .matches
        .iter()
        .zip(&results.captures)
        .filter_map(|(range, captures)| {
            // The watcher can replace the document between the search and the
            // read of its content, so a range that no longer fits the content
            // is skipped instead of being sliced.
            let start = usize::from(range.start());
            let end = usize::from(range.end());
            content.get(start..end)?;
            let location = source.location(range.start()).ok()?;
            let text = match output {
                SearchOutput::Matches => match_lines(&content, start, end),
                SearchOutput::Captures => vec![
                    captured_values(&content, captures)
                        .unwrap_or_else(|| shorten_line(&content, start)),
                ],
                SearchOutput::Files | SearchOutput::Lines => vec![shorten_line(&content, start)],
            };
            Some(ReportedMatch {
                line: location.line_number.get(),
                column: location.column_number.get(),
                text,
            })
        })
        .take(room)
        .collect();

    Ok(Some(found))
}

/// Returns `path` relative to the project root.
fn relative_path(project: &Project, path: &Utf8Path) -> String {
    path.strip_prefix(project.root())
        .unwrap_or(path)
        .to_string()
}

/// The matches of the pattern in one file.
struct FileMatches {
    /// The path of the file, relative to the project root.
    path: String,
    /// The number of matches in the file.
    count: usize,
    /// The first matches of the file, as many as the report has room for.
    matches: Vec<ReportedMatch>,
}

/// A match, ready to be reported.
struct ReportedMatch {
    line: usize,
    column: usize,
    /// The lines that the report shows for the match. There's more than one
    /// only with the [SearchOutput::Matches] output.
    text: Vec<String>,
}

/// What a search found, before it's written for the agent.
#[derive(Default)]
struct Report {
    files: Vec<FileMatches>,
    searched_files: usize,
    errors: Vec<String>,
}

impl Report {
    /// Returns the report for the agent in the form that `output` asks for,
    /// with at most `max_entries` files for the `files` output. The other
    /// outputs show the matches that [search_file] prepared, which are at most
    /// `max_entries` too.
    fn render(
        &self,
        output: SearchOutput,
        max_entries: usize,
        language: PatternLanguage,
    ) -> String {
        let total_matches: usize = self.files.iter().map(|file| file.count).sum();

        let mut lines = Vec::new();
        let (shown_entries, total_entries, entry) = if output == SearchOutput::Files {
            for file in self.files.iter().take(max_entries) {
                lines.push(format!("{} ({})", file.path, file.count));
            }
            (lines.len(), self.files.len(), ("file", "files"))
        } else {
            let mut shown_matches = 0;
            for file in self.files.iter().filter(|file| !file.matches.is_empty()) {
                if !lines.is_empty() {
                    lines.push(String::new());
                }
                lines.push(file.path.clone());
                for found in &file.matches {
                    shown_matches += 1;
                    let position = format!("{}:{}: ", found.line, found.column);
                    // The other lines of a match are aligned with its first
                    // line of code.
                    let alignment = " ".repeat(position.len());
                    let mut text = found.text.iter();
                    lines.push(format!(
                        "{position}{}",
                        text.next().map_or("", String::as_str)
                    ));
                    lines.extend(text.map(|line| format!("{alignment}{line}")));
                }
            }
            (shown_matches, total_matches, ("match", "matches"))
        };

        let mut summary = Vec::new();
        if total_matches == 0 && self.searched_files == 0 {
            summary.push(format!(
                "No files were searched. Check that the paths contain {} files and that the Biome configuration doesn't ignore them.",
                language.name()
            ));
        } else if total_matches == 0 {
            summary.push(format!(
                "No matches in {}.",
                plural(self.searched_files, "searched file", "searched files")
            ));
        } else {
            summary.push(format!(
                "Found {} in {} ({} searched).",
                plural(total_matches, "match", "matches"),
                plural(self.files.len(), "file", "files"),
                self.searched_files
            ));
            if shown_entries < total_entries {
                summary.push(format!(
                    "Showing the first {}. Narrow the pattern or the paths, or raise `maxMatches`, to see the rest.",
                    plural(shown_entries, entry.0, entry.1)
                ));
            }
        }
        if !self.errors.is_empty() {
            summary.push(format!(
                "Couldn't search {}:",
                plural(self.errors.len(), "file", "files")
            ));
            summary.extend(self.errors.iter().map(|error| format!("- {error}")));
        }

        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.extend(summary);
        lines.join("\n")
    }
}

/// Returns the line of `source` that contains `offset`, without leading and
/// trailing whitespace.
///
/// A line longer than [MAX_LINE_CHARS], as in minified code, is shortened to
/// that many characters starting at `offset`, and `…` marks each end where text
/// was removed.
fn shorten_line(source: &str, offset: usize) -> String {
    let line_start = source[..offset].rfind('\n').map_or(0, |index| index + 1);
    let line_end = source[offset..]
        .find('\n')
        .map_or(source.len(), |index| offset + index);

    let line = source[line_start..line_end].trim();
    if line.chars().count() <= MAX_LINE_CHARS {
        return line.to_string();
    }

    let from_offset = source[offset..line_end].trim_end();
    let mut shortened = String::new();
    if !source[line_start..offset].trim().is_empty() {
        shortened.push('…');
    }
    shortened.extend(from_offset.chars().take(MAX_LINE_CHARS));
    if from_offset.chars().count() > MAX_LINE_CHARS {
        shortened.push('…');
    }
    shortened
}

/// Returns the lines of code that the match from `start` to `end` in `source`
/// spans: the first one as [shorten_line] returns it, then at most
/// [MAX_MATCH_LINES] - 1 others, without the indentation of the first one.
/// When the match spans more lines, a last line says how many were left out.
fn match_lines(source: &str, start: usize, end: usize) -> Vec<String> {
    let mut lines = vec![shorten_line(source, start)];
    let Some(first_line_end) = source[start..end].find('\n').map(|index| start + index) else {
        return lines;
    };

    let line_start = source[..start].rfind('\n').map_or(0, |index| index + 1);
    let before_match = &source[line_start..start];
    let indentation = &before_match[..before_match.len() - before_match.trim_start().len()];
    let last_line_end = source[end..]
        .find('\n')
        .map_or(source.len(), |index| end + index);
    let other_lines: Vec<_> = source[first_line_end + 1..last_line_end].lines().collect();

    let shown = other_lines.len().min(MAX_MATCH_LINES - 1);
    lines.extend(other_lines[..shown].iter().map(|line| {
        let line = line
            .strip_prefix(indentation)
            .unwrap_or_else(|| line.trim_start());
        truncate(line.trim_end(), MAX_LINE_CHARS)
    }));
    if other_lines.len() > shown {
        lines.push(format!(
            "… {}",
            plural(other_lines.len() - shown, "more line", "more lines")
        ));
    }
    lines
}

/// Returns the code that `captures` captured in `source`, as `$name = value`
/// for each variable, or `None` when there's none.
///
/// Each value is on one line: every run of whitespace is replaced by a space,
/// and it's shortened to [MAX_CAPTURE_CHARS] characters.
fn captured_values(source: &str, captures: &[SearchCapture]) -> Option<String> {
    let values: Vec<_> = captures
        .iter()
        .filter_map(|capture| {
            let value =
                source.get(usize::from(capture.range.start())..usize::from(capture.range.end()))?;
            let value: Vec<_> = value.split_whitespace().collect();
            Some(format!(
                "{} = {}",
                capture.name,
                truncate(&value.join(" "), MAX_CAPTURE_CHARS)
            ))
        })
        .collect();
    // A value never contains two spaces in a row, so they separate the values
    // without ambiguity.
    (!values.is_empty()).then(|| values.join("  "))
}

/// Returns `text` shortened to `max_chars` characters, with `…` at the end when
/// characters were removed.
fn truncate(text: &str, max_chars: usize) -> String {
    let mut shortened: String = text.chars().take(max_chars).collect();
    if text.chars().nth(max_chars).is_some() {
        shortened.push('…');
    }
    shortened
}

fn plural(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

#[cfg(test)]
#[path = "search.tests.rs"]
mod tests;
