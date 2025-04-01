use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use biome_diagnostics::Diagnostic;
use biome_diagnostics::diagnostic::Severity;
use crossbeam::channel::unbounded;

use super::*;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "lint/style/noShoutyConstants",
    tags(FIXABLE),
    message = "Test diagnostic message",
    severity = Warning
)]
struct WarningDiagnostic {}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "lint/style/noShoutyConstants",
    tags(FIXABLE),
    message = "Test diagnostic message",
    severity = Error
)]
struct ErrorDiagnostic {}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "lint/style/noShoutyConstants",
    tags(FIXABLE),
    message = "Test diagnostic message",
    severity = Information
)]
struct InformationDiagnostic {}

#[test]
fn test_diagnostics_collector_sorting() {
    let collector = DiagnosticsCollector{
        diagnostic_level: Severity::Hint,
        verbose: false
    };
    let (sender, receiver) = unbounded();



    // Send diagnostics with different severities in random order
    let warning = SerdeDiagnostic::new(WarningDiagnostic {});
    let error = SerdeDiagnostic::new(ErrorDiagnostic {});
    let info = SerdeDiagnostic::new(InformationDiagnostic {});

    // Send in an order different from severity
    sender.send(info).unwrap();
    sender.send(warning).unwrap();
    sender.send(error).unwrap();
    drop(sender);

    let result = collector.run(receiver);

    // Verify diagnostics are sorted by severity (error > warning > info)
    assert_eq!(result[0].severity(), Severity::Error);
    assert_eq!(result[1].severity(), Severity::Warning);
    assert_eq!(result[2].severity(), Severity::Information);
}