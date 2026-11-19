use crate::runner::diagnostics::WatcherDiagnostic;
use camino::Utf8PathBuf;

pub enum WatcherEvent {
    Changed(Vec<Utf8PathBuf>),
    Error(WatcherDiagnostic),
}

pub trait Watcher {
    /// Start watching file changes under the paths recursively.
    fn watch(&mut self, paths: Vec<Utf8PathBuf>);

    /// Wait for the first event from the watcher.
    /// Returns [`None`] if the watcher is no longer available.
    fn poll(&mut self) -> Option<WatcherEvent>;
}
