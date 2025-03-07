use std::io;

use biome_console::{ConsoleExt, EnvConsole, fmt, markup};
use biome_diagnostics::{
    Advices, Diagnostic, Location, LogCategory, PrintDiagnostic, Resource, SourceCode, Visit,
};
use biome_rowan::{TextRange, TextSize};

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "args/fileNotFound", message = "No matching files found")]
struct NotFoundDiagnostic {
    #[location(resource)]
    path: String,
    #[advice]
    advices: NotFoundAdvices,
}

#[derive(Debug)]
struct NotFoundAdvices {
    pattern_list: Vec<String>,
    configuration_path: String,
    configuration_span: TextRange,
    configuration_source_code: String,
}

impl Advices for NotFoundAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(LogCategory::Info, &"The following files were ignored")?;

        let pattern_list: Vec<_> = self
            .pattern_list
            .iter()
            .map(|pattern| pattern as &dyn fmt::Display)
            .collect();

        visitor.record_list(&pattern_list)?;

        visitor.record_log(LogCategory::Info, &"Ignore patterns were defined here")?;
        visitor.record_frame(Location {
            resource: Some(Resource::File(&self.configuration_path)),
            span: Some(self.configuration_span),
            source_code: Some(SourceCode {
                text: &self.configuration_source_code,
                line_starts: None,
            }),
        })
    }
}

pub fn main() {
    let diag = NotFoundDiagnostic {
        path: String::from("dist/bundle.js"),
        advices: NotFoundAdvices {
            pattern_list: vec![String::from("dist/**/*.js"), String::from("build/**/*.js")],
            configuration_path: String::from("biome.json"),
            configuration_span: TextRange::new(TextSize::from(29), TextSize::from(106)),
            configuration_source_code: String::from(
                "{
    \"formatter\": {
        \"ignore\": [
            \"dist/**/*.js\",
            \"build/**/*.js\"
        ]
    }
}",
            ),
        },
    };

    EnvConsole::default().error(markup!({ PrintDiagnostic::verbose(&diag) }));
}
