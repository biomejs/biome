use biome_diagnostics::Diagnostic;
use biome_diagnostics::diagnostic::Severity;
use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use crossbeam::channel::unbounded;

use super::*;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "lint/style/noShoutyConstants",
    tags(FIXABLE),
    message = "Test diagnostic message"
)]
struct TestDiagnostic {
    #[severity]
    severity: Severity,
}

#[test]
fn test_diagnostics_collector_sorting() {
    let collector = DiagnosticsCollector::new();
    let (sender, receiver) = unbounded();

    // Send diagnostics with different severities
    let warning = SerdeDiagnostic::new(TestDiagnostic { severity: Severity::Warning });
    let error = SerdeDiagnostic::new(TestDiagnostic { severity: Severity::Error });
    let info = SerdeDiagnostic::new(TestDiagnostic { severity: Severity::Information });

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
