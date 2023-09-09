use biome_diagnostics::Diagnostic;

#[derive(Clone, Debug, PartialEq, Eq, Diagnostic)]
#[diagnostic(category = "suppressions/parse")]
pub struct SuppressionDiagnostic {}
