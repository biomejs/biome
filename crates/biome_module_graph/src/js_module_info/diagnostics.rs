use crate::js_module_info::utils::MAX_NUM_TYPES;
use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_diagnostics::{
    Category, Diagnostic, DiagnosticTags, Location, LogCategory, Severity, Visit, category,
};

#[derive(Debug, Clone)]
pub enum JsModuleInfoDiagnostic {
    ExceededTypesLimit(ExceededTypesLimitDiagnostic),
}

impl Diagnostic for JsModuleInfoDiagnostic {
    fn severity(&self) -> Severity {
        match self {
            Self::ExceededTypesLimit(d) => d.severity(),
        }
    }
    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::ExceededTypesLimit(d) => d.message(fmt),
        }
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExceededTypesLimit(d) => d.description(fmt),
        }
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::ExceededTypesLimit(d) => d.source(),
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            Self::ExceededTypesLimit(d) => d.location(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            Self::ExceededTypesLimit(d) => d.tags(),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::ExceededTypesLimit(d) => d.advices(visitor),
        }
    }

    fn category(&self) -> Option<&'static Category> {
        match self {
            Self::ExceededTypesLimit(d) => d.category(),
        }
    }

    fn verbose_advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::ExceededTypesLimit(d) => d.verbose_advices(visitor),
        }
    }
}

impl JsModuleInfoDiagnostic {
    pub fn exceeded_types_limit(types_count: usize) -> Self {
        Self::ExceededTypesLimit(ExceededTypesLimitDiagnostic { types_count })
    }
}

#[derive(Debug, Clone)]
pub struct ExceededTypesLimitDiagnostic {
    pub types_count: usize,
}

impl Diagnostic for ExceededTypesLimitDiagnostic {
    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Option<&'static Category> {
        Some(category!("project"))
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! {
            "Biome encountered an unusually large amount of types ("{self.types_count}") which exceeded the limit of "{MAX_NUM_TYPES}"."
        })?;

        fmt.write_str("\n\n")?;

        fmt.write_markup(markup! {
            "Either you are analyzing "<Emphasis>"very"</Emphasis>" large files (did you make sure to \
        exclude your "<Emphasis>"build/"</Emphasis>" or "<Emphasis>"dist/"</Emphasis>" folder?), or you've encountered a bug in \
        Biome."
        })?;

        fmt.write_str("\n\n")?;

        fmt.write_markup(markup! {
            "Please follow these instructions to discover if you are accidentally \
        analyzing large files and what to do about them in "<Hyperlink href={"https://biomejs.dev/guides/investigate-slowness/"}>"the relative guide."</Hyperlink>
        })?;

        Ok(())
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        visitor.record_log(LogCategory::Info, &markup!{
            "In the meantime, you can ignore this file by adding its name or folder in the "<Emphasis>"files.experimentalScannerIgnores"</Emphasis>" option in your configuration file."
        })?;
        visitor.record_log(LogCategory::Info, &markup!{
            "Refer to the "<Hyperlink href={"https://biomejs.dev/reference/configuration/#filesexperimentalscannerignoress"}>"documentation"</Hyperlink>" for more information."
        })?;

        visitor.record_log(
            LogCategory::Info,
            &markup! {
                "If you think this is a bug, please report it and include the following information:"
            },
        )?;
        visitor.record_list(&[
            &"source code of the file;",
            &"how the file is imported in the project (by a test file, a dependency, etc.);",
            &"how the file/folder is excluded.",
        ])?;
        visitor.record_log(
            LogCategory::Warn,
            &markup! {"Failing to provide this information won't allow the team to fix the issue."},
        )?;

        Ok(())
    }
}

impl From<ExceededTypesLimitDiagnostic> for JsModuleInfoDiagnostic {
    fn from(d: ExceededTypesLimitDiagnostic) -> Self {
        Self::ExceededTypesLimit(d)
    }
}
