use biome_diagnostics::{Diagnostic, PrintDescription, Severity};
use grit_util::{AnalysisLog, Position, Range};
use std::path::{Path, PathBuf};

pub trait GritAnalysisExt {
    fn to_log(&self, path: Option<&Path>) -> AnalysisLog;
}

impl<T> GritAnalysisExt for T
where
    T: Diagnostic,
{
    fn to_log(&self, path: Option<&Path>) -> AnalysisLog {
        let location = self.location();
        let source = location.source_code;
        let range =
            match (location.span, source) {
                (Some(range), Some(source)) => source.text[..range.start().into()]
                    .lines()
                    .enumerate()
                    .last()
                    .map(|(i, line)| {
                        let start = Position {
                            line: (i + 1) as u32,
                            column: line.len() as u32,
                        };
                        let end = source.text[range].lines().enumerate().last().map_or(
                            start,
                            |(j, line)| Position {
                                line: start.line + j as u32,
                                column: if j == 0 {
                                    start.column + line.len() as u32
                                } else {
                                    line.len() as u32
                                },
                            },
                        );
                        Range {
                            start,
                            end,
                            start_byte: range.start().into(),
                            end_byte: range.end().into(),
                        }
                    }),
                _ => None,
            };

        AnalysisLog {
            engine_id: Some("biome".to_owned()),
            file: path.map(Path::to_path_buf).or_else(|| {
                location
                    .resource
                    .and_then(|r| r.as_file().map(PathBuf::from))
            }),
            level: Some(match self.severity() {
                Severity::Hint => 1,
                Severity::Information => 2,
                Severity::Warning => 3,
                Severity::Error => 4,
                Severity::Fatal => 5,
            }),
            message: PrintDescription(self).to_string(),
            position: range.as_ref().map(|r| r.start),
            range,
            syntax_tree: None,
            source: source.map(|s| s.text.to_owned()),
        }
    }
}
