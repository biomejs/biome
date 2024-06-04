use biome_diagnostics::{display::SourceFile, Diagnostic, PrintDescription, Severity};
use grit_util::AnalysisLog;
use std::path::{Path, PathBuf};

use crate::source_location_ext::SourceFileExt;

pub trait GritAnalysisExt {
    fn to_log(&self, path: Option<&Path>) -> AnalysisLog;
}

impl<T> GritAnalysisExt for T
where
    T: Diagnostic,
{
    fn to_log(&self, path: Option<&Path>) -> AnalysisLog {
        let location = self.location();
        let source = location.source_code.map(SourceFile::new);

        let range = match (location.span, source) {
            (Some(range), Some(source)) => source.to_grit_range(range),
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
            source: location.source_code.map(|s| s.text.to_string()),
        }
    }
}
