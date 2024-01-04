use biome_cli::CliDiagnostic;
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::{markup, BufferConsole, Markup};
use biome_diagnostics::termcolor::NoColor;
use biome_diagnostics::{print_diagnostic_to_string, Error};
use biome_formatter::IndentStyle;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_formatter::format_node;
use biome_json_parser::{parse_json, JsonParserOptions};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::env::{current_exe, temp_dir};
use std::fmt::Write as _;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

#[derive(Default)]
struct InMessages {
    stdin: Option<String>,
}

pub(crate) struct CliSnapshot {
    /// input messages, coming from different sources
    in_messages: InMessages,
    /// the configuration, if set
    pub configuration: Option<String>,
    /// file name -> content
    pub files: BTreeMap<String, String>,
    /// messages written in console
    pub messages: Vec<String>,
    /// possible termination error of the CLI
    pub termination: Option<Error>,
}

impl CliSnapshot {
    pub fn from_result(result: Result<(), CliDiagnostic>) -> Self {
        Self {
            in_messages: InMessages::default(),
            configuration: None,
            files: BTreeMap::default(),
            messages: Vec::new(),
            termination: result.err().map(Error::from),
        }
    }
}

impl CliSnapshot {
    pub fn emit_content_snapshot(&self) -> String {
        let mut content = String::new();

        if let Some(configuration) = &self.configuration {
            let redacted = redact_snapshot(configuration).unwrap_or(String::new().into());

            let parsed = parse_json(&redacted, JsonParserOptions::default());
            let formatted = format_node(
                JsonFormatOptions::default()
                    .with_indent_style(IndentStyle::Space)
                    .with_indent_width(2.into()),
                &parsed.syntax(),
            )
            .expect("formatted JSON")
            .print()
            .expect("printed JSON");

            content.push_str("## `biome.json`\n\n");
            content.push_str("```json");
            content.push('\n');
            content.push_str(formatted.as_code());
            content.push_str("```");
            content.push_str("\n\n")
        }

        for (name, file_content) in &self.files {
            if !name.starts_with("biome.json") {
                let extension = name.split('.').last().unwrap();

                let redacted_name = redact_snapshot(name).unwrap_or(String::new().into());
                let redacted_content =
                    redact_snapshot(file_content).unwrap_or(String::new().into());

                let _ = write!(content, "## `{}`\n\n", redacted_name);
                let _ = write!(content, "```{extension}");
                content.push('\n');
                content.push_str(&redacted_content);
                content.push('\n');
                content.push_str("```");
                content.push_str("\n\n")
            }
        }

        if let Some(stdin) = &self.in_messages.stdin {
            content.push_str("# Input messages\n\n");
            content.push_str("```block");
            content.push('\n');
            content.push_str(stdin);
            content.push('\n');
            content.push_str("```");
            content.push_str("\n\n")
        }

        if let Some(termination) = &self.termination {
            let message = print_diagnostic_to_string(termination);

            if let Some(redacted) = &redact_snapshot(&message) {
                content.push_str("# Termination Message\n\n");
                content.push_str("```block");
                content.push('\n');
                content.push_str(redacted);
                content.push('\n');
                content.push_str("```");
                content.push_str("\n\n");
            }
        }

        if !self.messages.is_empty() {
            content.push_str("# Emitted Messages\n\n");

            for message in &self.messages {
                let Some(redacted) = &redact_snapshot(message) else {
                    continue;
                };

                content.push_str("```block");
                content.push('\n');
                content.push_str(redacted);
                content.push('\n');
                content.push_str("```");
                content.push_str("\n\n")
            }
        }

        content
    }
}

fn redact_snapshot(input: &str) -> Option<Cow<'_, str>> {
    let mut output = Cow::Borrowed(input);

    // There are some logs that print the timing, and we can't snapshot that message
    // otherwise at each run we invalid the previous snapshot.
    //
    // This is a workaround, and it might not work for all cases.
    const PATTERN: &str = "file(s) in ";
    if let Some(start) = output.find(PATTERN) {
        output
            .to_mut()
            .replace_range(start + PATTERN.len().., "<TIME>");
    }

    // Normalize the name of the current executable to "biome"
    let current_exe = current_exe()
        .ok()
        .and_then(|path| Some(path.file_name()?.to_str()?.to_string()));

    if let Some(current_exe) = current_exe {
        replace(&mut output, &current_exe, "biome");
    }

    output = replace_temp_dir(output);
    output = replace_biome_dir(output);

    // Ref: https://docs.github.com/actions/learn-github-actions/variables#default-environment-variables
    let is_github = std::env::var("GITHUB_ACTIONS")
        .ok()
        .map(|value| value == "true")
        .unwrap_or(false);

    if is_github {
        // GitHub actions sets the env var GITHUB_ACTIONS=true in CI
        // Based on that, biome also has a feature that detects if it's running on GH Actions and
        // emits additional information in the output (workflow commands, see PR + discussion at #681)
        // To avoid polluting snapshots with that, we filter out those lines

        let lines: Vec<_> = output
            .split('\n')
            .filter(|line| {
                !line.starts_with("::notice ")
                    && !line.starts_with("::warning ")
                    && !line.starts_with("::error ")
            })
            .collect();

        output = Cow::Owned(lines.join("\n"));
        if output.is_empty() {
            return None;
        }
    }

    // Normalize Windows-specific path separators to "/"
    if cfg!(windows) {
        let mut rest = &*output;
        let mut result = String::new();

        while let Some(index) = rest.find(MAIN_SEPARATOR) {
            let (before, after) = rest.split_at(index);
            result.push_str(before);

            // Paths are recognized if they start with ".\",  ":\" (as in "C:\")
            // or ">\" (as in "<TEMP_DIR>\")
            if !before.ends_with(['.', ':', '>']) {
                let (sep, after) = after.split_at(1);
                result.push_str(sep);
                rest = after;
                continue;
            }

            // File paths are assumed to end at the first space or line breaks
            let path = if let Some(end) = after.find([' ', '\n']) {
                let (before, after) = after.split_at(end);
                rest = after;
                before
            } else {
                rest = "";
                after
            };

            result.push_str(&path.replace(MAIN_SEPARATOR, "/"));
        }

        if !result.is_empty() {
            result.push_str(&rest.replace(MAIN_SEPARATOR, "/"));
            output = Cow::Owned(result);
        }
    }

    Some(output)
}

/// Replace the path to the temporary directory with "<TEMP_DIR>"
/// And normalizes the count of `-` at the end of the diagnostic
fn replace_temp_dir(input: Cow<str>) -> Cow<str> {
    let mut result = String::new();
    let mut rest = input.as_ref();

    let temp_dir = temp_dir().display().to_string();
    let temp_dir = temp_dir.trim_end_matches(MAIN_SEPARATOR);

    while let Some(index) = rest.find(temp_dir) {
        let (before, after) = rest.split_at(index);

        result.push_str(before);
        result.push_str("<TEMP_DIR>");

        let after = after.split_at(temp_dir.len()).1;
        let header_line = after.lines().next().unwrap();

        match header_line.split_once('\u{2501}') {
            Some((between_temp_and_line, _)) => {
                // Diagnostic header line, normalize the horizontal line
                result.push_str(between_temp_and_line);
                result.push_str(&"\u{2501}".repeat(20));
                rest = after.split_at(header_line.len()).1;
            }
            None => {
                // Not a header line, only replace tempdir
                rest = after;
            }
        }
    }

    if result.is_empty() {
        input
    } else {
        result.push_str(rest);
        Cow::Owned(result)
    }
}

/// Replace the path to the temporary directory with "<TEMP_DIR>"
/// And normalizes the count of `-` at the end of the diagnostic
fn replace_biome_dir(input: Cow<str>) -> Cow<str> {
    let mut result = String::new();
    let mut rest = input.as_ref();

    let temp_dir = biome_fs::ensure_cache_dir().display().to_string();
    let temp_dir = temp_dir.trim_end_matches(MAIN_SEPARATOR);

    while let Some(index) = rest.find(temp_dir) {
        let (before, after) = rest.split_at(index);

        result.push_str(before);
        result.push_str("<BIOME_DIR>");

        let after = after.split_at(temp_dir.len()).1;
        let header_line = after.lines().next().unwrap();

        match header_line.split_once('\u{2501}') {
            Some((between_temp_and_line, _)) => {
                // Diagnostic header line, normalize the horizontal line
                result.push_str(between_temp_and_line);
                result.push_str(&"\u{2501}".repeat(20));
                rest = after.split_at(header_line.len()).1;
            }
            None => {
                // Not a header line, only replace tempdir
                rest = after;
            }
        }
    }

    if result.is_empty() {
        input
    } else {
        result.push_str(rest);
        Cow::Owned(result)
    }
}

fn replace(input: &mut Cow<str>, from: &str, to: &str) {
    let mut rest = &**input;
    let mut result = String::new();

    while let Some(index) = rest.find(from) {
        let (before, after) = rest.split_at(index);

        result.push_str(before);
        result.push_str(to);

        let (_, after) = after.split_at(from.len());
        rest = after;
    }

    if !result.is_empty() {
        result.push_str(rest);
        *input = Cow::Owned(result);
    }
}

impl From<SnapshotPayload<'_>> for CliSnapshot {
    fn from(payload: SnapshotPayload<'_>) -> Self {
        let SnapshotPayload {
            result,
            console,
            fs,
            test_name: _,
            module_path: _,
        } = payload;
        let mut cli_snapshot = CliSnapshot::from_result(result);
        let config_path = PathBuf::from("biome.json");
        let configuration = fs.open(&config_path).ok();
        if let Some(mut configuration) = configuration {
            let mut buffer = String::new();
            if configuration.read_to_string(&mut buffer).is_ok() {
                cli_snapshot.configuration = Some(buffer);
            }
        }

        let files: Vec<_> = fs
            .files()
            .map(|(file, entry)| {
                let content = entry.lock();
                let content = std::str::from_utf8(content.as_slice()).unwrap();
                (file.to_str().unwrap().to_string(), String::from(content))
            })
            .collect();

        for (file, content) in files {
            cli_snapshot.files.insert(file, content);
        }

        let in_buffer = &console.in_buffer;
        for (index, message) in in_buffer.iter().enumerate() {
            if index == 0 {
                cli_snapshot.in_messages.stdin = Some(message.to_string());
            }
        }

        for message in &console.out_buffer {
            let content = markup_to_string(markup! {
                {message.content}
            });
            cli_snapshot.messages.push(content)
        }

        cli_snapshot
    }
}

pub fn markup_to_string(markup: Markup) -> String {
    let mut buffer = Vec::new();
    let mut write = Termcolor(NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}

pub struct SnapshotPayload<'a> {
    pub module_path: &'a str,
    pub test_name: &'a str,
    pub fs: MemoryFileSystem,
    pub console: BufferConsole,
    pub result: Result<(), CliDiagnostic>,
}

impl<'a> SnapshotPayload<'a> {
    pub fn new(
        module_path: &'a str,
        test_name: &'a str,
        fs: MemoryFileSystem,
        console: BufferConsole,
        result: Result<(), CliDiagnostic>,
    ) -> Self {
        Self {
            module_path,
            test_name,
            fs,
            console,
            result,
        }
    }
}

/// Function used to snapshot a session test of the a CLI run.
pub fn assert_cli_snapshot(payload: SnapshotPayload<'_>) {
    let module_path = payload.module_path.to_owned();
    let test_name = payload.test_name;
    let cli_snapshot = CliSnapshot::from(payload);

    let content = cli_snapshot.emit_content_snapshot();

    let module_path = module_path.replace("::", "_");
    let snapshot_path = PathBuf::from("snapshots").join(module_path);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => snapshot_path
    }, {
        insta::assert_snapshot!(test_name, content);

    });
}

/// It checks if the contents of a file matches the passed `expected_content`
pub fn assert_file_contents(fs: &MemoryFileSystem, path: &Path, expected_content: &str) {
    let mut file = fs.open(path).expect("file was removed");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(
        content,
        expected_content,
        "file {} doesn't match the expected content (right)",
        path.display()
    );
}
