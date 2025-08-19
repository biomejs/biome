use crate::js_module_info::JsModuleInfoDiagnostic;
use biome_diagnostics::{Category, Diagnostic, DiagnosticTags, Location, Severity, Visit};

#[derive(Debug, Clone)]
pub enum ModuleDiagnostic {
    JsInfo(JsModuleInfoDiagnostic),
}

impl From<JsModuleInfoDiagnostic> for ModuleDiagnostic {
    fn from(d: JsModuleInfoDiagnostic) -> Self {
        Self::JsInfo(d)
    }
}

impl Diagnostic for ModuleDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        match self {
            Self::JsInfo(d) => d.category(),
        }
    }
    fn verbose_advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::JsInfo(d) => d.verbose_advices(visitor),
        }
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsInfo(d) => d.description(fmt),
        }
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::JsInfo(d) => d.message(fmt),
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            Self::JsInfo(d) => d.location(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            Self::JsInfo(d) => d.tags(),
        }
    }

    fn severity(&self) -> Severity {
        match self {
            Self::JsInfo(d) => d.severity(),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::JsInfo(d) => d.advices(visitor),
        }
    }
}
