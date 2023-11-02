use biome_console::markup;
use biome_diagnostics::console::fmt::Display;
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{Diagnostic, DiagnosticTags, MessageAndDescription};
use biome_text_size::TextRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Diagnostic, Deserialize, Serialize, Clone)]
#[diagnostic(
	category = "project",
	severity = Error
)]
pub struct ProjectDiagnostic {
    #[message]
    #[description]
    message: MessageAndDescription,
    #[tags]
    tags: DiagnosticTags,

    #[location(span)]
    range: Option<TextRange>,
}

impl ProjectDiagnostic {
    fn new(message: impl Display) -> Self {
        Self {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
            range: None,
            tags: DiagnosticTags::empty(),
        }
    }

    // pub fn new_failed_deserialization(manifest_path: &str, diagnostics: Vec<Error>) -> Self {
    //     let mut diagnostic = Self::new(markup!("Failed to deserialize the manifest"));
    //     diagnostic.verbose_advice = Advice {
    //         diagnostics: diagnostics
    //             .into_iter()
    //             .map(|diagnostic| diagnostic.with_file_path(manifest_path))
    //             .collect(),
    //     };
    //     diagnostic
    // }

    pub fn new_internal() -> Self {
        Self::new("Failed to retrieve the project during internal operations")
            .with_tag(DiagnosticTags::INTERNAL)
    }

    pub fn with_tag(mut self, tag: DiagnosticTags) -> Self {
        self.tags.insert(tag);
        self
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "project")]
pub struct ProjectAnalyzeDiagnostic {
    #[message]
    #[description]
    message: MessageAndDescription,

    #[location(span)]
    range: Option<TextRange>,
}

impl ProjectAnalyzeDiagnostic {
    fn new(message: impl Display) -> Self {
        Self {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
            range: None,
        }
    }

    pub fn new_invalid_license(licence: impl Display) -> Self {
        Self::new(markup! {
            "The license "<Emphasis>{licence}</Emphasis>" is invalid"
        })
    }

    pub fn new_deprecated_license(licence: impl Display) -> Self {
        Self::new(markup! {
            "The license "<Emphasis>{licence}</Emphasis>" is deprecated"
        })
    }
    pub fn with_range(mut self, range: impl AsSpan) -> Self {
        self.range = range.as_span();
        self
    }
}
