use crate::js_module_info::utils::MAX_NUM_TYPES;
use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_diagnostics::{
    Category, Diagnostic, DiagnosticTags, LogCategory, Severity, Visit, category,
};

#[derive(Debug, Clone, Diagnostic)]
pub enum JsModuleInfoDiagnostic {
    ExceededTypesLimit(ExceededTypesLimitDiagnostic),
}

impl JsModuleInfoDiagnostic {
    pub fn exceeded_types_limit() -> Self {
        Self::ExceededTypesLimit(ExceededTypesLimitDiagnostic)
    }
}

#[derive(Debug, Clone)]
pub struct ExceededTypesLimitDiagnostic;

impl Diagnostic for ExceededTypesLimitDiagnostic {
    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Option<&'static Category> {
        Some(category!("project"))
    }

    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::INTERNAL
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        // format with a thousand separators for readability:
        let num = MAX_NUM_TYPES
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|bytes| std::str::from_utf8(bytes).unwrap())
            .collect::<Vec<_>>()
            .join(",");

        fmt.write_markup(markup! {
            "Biome encountered an unusually large amount of types which exceeded the limit of "{num}"."
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
            "In the meantime, you can force this file to be ignored using a `!!` pattern in the "<Emphasis>"files.includes"</Emphasis>" option in your configuration file."
        })?;
        visitor.record_log(LogCategory::Info, &markup!{
            "Refer to the "<Hyperlink href={"https://biomejs.dev/reference/configuration/#filesincludes"}>"documentation"</Hyperlink>" for more information."
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
            &"if and how the file/folder is excluded.",
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
