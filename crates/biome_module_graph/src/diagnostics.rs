use crate::js_module_info::JsModuleInfoDiagnostic;
use biome_diagnostics::Diagnostic;

#[derive(Debug, Clone, Diagnostic)]
pub enum ModuleDiagnostic {
    JsInfo(JsModuleInfoDiagnostic),
}

impl From<JsModuleInfoDiagnostic> for ModuleDiagnostic {
    fn from(d: JsModuleInfoDiagnostic) -> Self {
        Self::JsInfo(d)
    }
}
