use biome_diagnostics::Diagnostic;
use biome_rowan::TextRange;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(severity = Warning)]
pub struct CompilerDiagnostic {
    #[message]
    #[description]
    message: String,

    #[location(span)]
    range: TextRange,
}

impl CompilerDiagnostic {
    pub(crate) fn new_warning(message: impl Into<String>, range: TextRange) -> Self {
        Self {
            message: message.into(),
            range,
        }
    }
}
