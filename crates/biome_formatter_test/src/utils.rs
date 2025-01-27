use crate::diff_report::DiffReport;
use camino::Utf8Path;
use similar::TextDiff;
use std::fs::{read_to_string, remove_file};

struct StripPlaceholders {
    cursor: String,
    range_start_placeholder: String,
    range_end_placeholder: String,
}

/// Find and replace the cursor, range start and range end placeholders in a
/// snapshot tests and return their indices in the resulting string
impl StripPlaceholders {
    pub fn new(
        cursor: String,
        range_start_placeholder: String,
        range_end_placeholder: String,
    ) -> Self {
        StripPlaceholders {
            cursor,
            range_start_placeholder,
            range_end_placeholder,
        }
    }

    pub fn transform(
        &self,
        input_code: &mut String,
    ) -> (Option<usize>, Option<usize>, Option<usize>) {
        let mut cursor_index = None;
        let mut range_start_index = None;
        let mut range_end_index = None;

        if let Some(index) = input_code.find(&self.cursor) {
            input_code.replace_range(index..index + self.cursor.len(), "");
            cursor_index = Some(index);
        }

        if let Some(index) = input_code.find(&self.range_start_placeholder) {
            input_code.replace_range(index..index + self.range_start_placeholder.len(), "");
            range_start_index = Some(index);

            if let Some(cursor) = &mut cursor_index {
                if *cursor > index {
                    *cursor -= self.range_start_placeholder.len();
                }
            }
        }

        if let Some(index) = input_code.find(&self.range_end_placeholder) {
            input_code.replace_range(index..index + self.range_end_placeholder.len(), "");
            range_end_index = Some(index);

            if let Some(cursor) = &mut cursor_index {
                if *cursor > index {
                    *cursor -= self.range_end_placeholder.len();
                }
            }
            if let Some(cursor) = &mut range_start_index {
                // Prettier has tests for reversed ranges
                if *cursor > index {
                    *cursor -= self.range_end_placeholder.len();
                }
            }
        }

        (cursor_index, range_start_index, range_end_index)
    }
}

const PRETTIER_CURSOR_PLACEHOLDER: &str = "<|>";
const PRETTIER_RANGE_START_PLACEHOLDER: &str = "<<<PRETTIER_RANGE_START>>>";
const PRETTIER_RANGE_END_PLACEHOLDER: &str = "<<<PRETTIER_RANGE_END>>>";

pub(crate) fn strip_prettier_placeholders(
    input_code: &mut String,
) -> (Option<usize>, Option<usize>, Option<usize>) {
    StripPlaceholders::new(
        PRETTIER_CURSOR_PLACEHOLDER.to_string(),
        PRETTIER_RANGE_START_PLACEHOLDER.to_string(),
        PRETTIER_RANGE_END_PLACEHOLDER.to_string(),
    )
    .transform(input_code)
}

const ROME_CURSOR_PLACEHOLDER: &str = "<|>";
const ROME_RANGE_START_PLACEHOLDER: &str = "<<<ROME_RANGE_START>>>";
const ROME_RANGE_END_PLACEHOLDER: &str = "<<<ROME_RANGE_END>>>";

pub(crate) fn strip_rome_placeholders(
    input_code: &mut String,
) -> (Option<usize>, Option<usize>, Option<usize>) {
    StripPlaceholders::new(
        ROME_CURSOR_PLACEHOLDER.to_string(),
        ROME_RANGE_START_PLACEHOLDER.to_string(),
        ROME_RANGE_END_PLACEHOLDER.to_string(),
    )
    .transform(input_code)
}

pub enum PrettierDiff {
    Diff(String),
    Same,
}

pub fn get_prettier_diff(
    input_file: &Utf8Path,
    relative_file_name: &'static str,
    formatted: &str,
) -> PrettierDiff {
    let input_extension = input_file.extension();

    let prettier_snapshot_path = input_extension
        .map(|ext| input_file.with_extension(format!("{ext}.prettier-snap")))
        .filter(|path| path.exists());

    let prettier_snapshot_path = prettier_snapshot_path.expect("failed to find prettier snapshot");

    let mut prettier_snapshot = read_to_string(prettier_snapshot_path).unwrap();

    strip_prettier_placeholders(&mut prettier_snapshot);

    DiffReport::get().report(relative_file_name, formatted, &prettier_snapshot);

    if formatted == prettier_snapshot {
        // The output matches prettier's output. There's no need for a snapshot that duplicates the output.
        // Delete the snapshot file if it already exists, otherwise return early to not create a new snapshot.
        if let Some(input_extension) = input_extension {
            let snapshot_file_name = input_file.with_extension(format!("{input_extension}.snap"));

            if snapshot_file_name.exists() && snapshot_file_name.is_file() {
                remove_file(snapshot_file_name).ok(); // not the end of the world if it fails
            }

            let new_snapshot_file_name =
                input_file.with_extension(format!("{input_extension}.snap.new"));
            if new_snapshot_file_name.exists() && new_snapshot_file_name.is_file() {
                remove_file(new_snapshot_file_name).ok(); // not the end of the world if it fails
            }
        }

        PrettierDiff::Same
    } else {
        let mut prettier_differences = Vec::new();

        TextDiff::from_lines(prettier_snapshot.as_str(), formatted)
            .unified_diff()
            .header("Prettier", "Biome")
            .to_writer(&mut prettier_differences)
            .unwrap();

        PrettierDiff::Diff(
            String::from_utf8(prettier_differences).expect("Input file to be in UTF8"),
        )
    }
}
