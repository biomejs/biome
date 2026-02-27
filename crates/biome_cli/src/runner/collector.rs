use crate::runner::execution::Execution;
use crate::runner::process_file::Message;
use biome_diagnostics::{DiagnosticTags, Severity};
use camino::Utf8PathBuf;
use crossbeam::channel::Receiver;
use std::time::Duration;

pub(crate) trait Collector: Send + Sync {
    type Result: Send + Sync;

    fn should_collect(&self) -> bool;

    fn diagnostic_level(&self) -> Severity;

    fn verbose(&self) -> bool;

    /// Checks if the diagnostic we received from the thread should be considered or not. Logic:
    /// - it should not be considered if its severity level is lower than the one provided via CLI;
    /// - it should not be considered if it's a verbose diagnostic and the CLI **didn't** request a `--verbose` option.
    fn should_skip_diagnostic(&self, severity: Severity, diagnostic_tags: DiagnosticTags) -> bool {
        if severity < self.diagnostic_level() {
            return true;
        }

        if diagnostic_tags.is_verbose() && !self.verbose() {
            return true;
        }

        false
    }
    fn run(
        &self,
        _receiver: Receiver<Message>,
        _interner: Receiver<Utf8PathBuf>,
        _execution: &dyn Execution,
    );

    fn result(self, _duration: Duration) -> Self::Result;
}

impl Collector for () {
    type Result = ();

    fn should_collect(&self) -> bool {
        false
    }

    fn diagnostic_level(&self) -> Severity {
        Severity::Hint
    }

    fn verbose(&self) -> bool {
        false
    }

    fn run(
        &self,
        _receiver: Receiver<Message>,
        _interner: Receiver<Utf8PathBuf>,
        _execution: &dyn Execution,
    ) {
    }

    fn result(self, _duration: Duration) -> Self::Result {}
}
