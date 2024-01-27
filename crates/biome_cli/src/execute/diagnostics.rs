use biome_diagnostics::adapters::{IoError, StdError};
use biome_diagnostics::{
    Advices, Category, Diagnostic, DiagnosticExt, DiagnosticTags, Error, Severity, Visit,
};
use biome_text_edit::TextEdit;
use std::io;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "format",
    message = "File content differs from formatting output"
)]
pub(crate) struct CIFormatDiffDiagnostic {
    #[location(resource)]
    pub(crate) file_name: String,
    #[advice]
    pub(crate) diff: ContentDiffAdvice,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "organizeImports",
    message = "Import statements differs from the output"
)]
pub(crate) struct CIOrganizeImportsDiffDiagnostic {
    #[location(resource)]
    pub(crate) file_name: String,
    #[advice]
    pub(crate) diff: ContentDiffAdvice,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
category = "format",
severity = Information,
message = "Formatter would have printed the following content:"
)]
pub(crate) struct FormatDiffDiagnostic {
    #[location(resource)]
    pub(crate) file_name: String,
    #[advice]
    pub(crate) diff: ContentDiffAdvice,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
	category = "organizeImports",
	severity = Information,
	message = "Import statements could be sorted:"
)]
pub(crate) struct OrganizeImportsDiffDiagnostic {
    #[location(resource)]
    pub(crate) file_name: String,
    #[advice]
    pub(crate) diff: ContentDiffAdvice,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
	category = "migrate",
	severity = Information,
	message = "Configuration file can be updated."
)]
pub(crate) struct MigrateDiffDiagnostic {
    #[location(resource)]
    pub(crate) file_name: String,
    #[advice]
    pub(crate) diff: ContentDiffAdvice,
}

#[derive(Debug)]
pub(crate) struct ContentDiffAdvice {
    pub(crate) old: String,
    pub(crate) new: String,
}

impl Advices for ContentDiffAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let diff = TextEdit::from_unicode_words(&self.old, &self.new);
        visitor.record_diff(&diff)
    }
}

#[derive(Debug, Diagnostic)]
pub(crate) struct TraversalDiagnostic<'a> {
    #[location(resource)]
    pub(crate) file_name: Option<&'a str>,
    #[severity]
    pub(crate) severity: Severity,
    #[category]
    pub(crate) category: &'static Category,
    #[message]
    #[description]
    pub(crate) message: &'a str,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "internalError/panic", tags(INTERNAL))]
pub(crate) struct PanicDiagnostic {
    #[description]
    #[message]
    pub(crate) message: String,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "files/missingHandler",
    message = "Biome doesn't know how to process this file",
	severity = Warning,
    tags(VERBOSE)
)]
pub(crate) struct UnhandledDiagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "parse", message = "Skipped file with syntax errors")]
pub(crate) struct SkippedDiagnostic;

/// Extension trait for turning [Display]-able error types into [TraversalError]
pub(crate) trait ResultExt {
    type Result;
    fn with_file_path_and_code(
        self,
        file_path: String,
        code: &'static Category,
    ) -> Result<Self::Result, Error>;

    fn with_file_path_and_code_and_tags(
        self,
        file_path: String,
        code: &'static Category,
        tags: DiagnosticTags,
    ) -> Result<Self::Result, Error>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    type Result = T;

    fn with_file_path_and_code_and_tags(
        self,
        file_path: String,
        code: &'static Category,
        diagnostic_tags: DiagnosticTags,
    ) -> Result<Self::Result, Error> {
        self.map_err(move |err| {
            StdError::from(err)
                .with_category(code)
                .with_file_path(file_path)
                .with_tags(diagnostic_tags)
        })
    }

    fn with_file_path_and_code(
        self,
        file_path: String,
        code: &'static Category,
    ) -> Result<Self::Result, Error> {
        self.map_err(move |err| {
            StdError::from(err)
                .with_category(code)
                .with_file_path(file_path)
        })
    }
}

/// Extension trait for turning [io::Error] into [Error]
pub(crate) trait ResultIoExt: ResultExt {
    fn with_file_path(self, file_path: String) -> Result<Self::Result, Error>;
}

impl<T> ResultIoExt for io::Result<T> {
    fn with_file_path(self, file_path: String) -> Result<Self::Result, Error> {
        self.map_err(|error| IoError::from(error).with_file_path(file_path))
    }
}
