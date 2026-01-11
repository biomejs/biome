use camino::Utf8PathBuf;

pub(crate) struct WatcherEvent {
    pub(crate) paths: Vec<Utf8PathBuf>,
}

pub(crate) trait Watcher {
    /// Creates a new filesystem watcher.
    fn new() -> Self;

    /// Start watching file changes under the paths recursively.
    fn watch(&mut self, paths: impl IntoIterator<Item = Utf8PathBuf>);

    /// Wait for the first event from the watcher.
    /// Returns [`None`] if the watcher is no longer available.
    fn poll(&mut self) -> Option<WatcherEvent>;
}

impl Watcher for () {
    fn new() -> Self {}

    fn watch(&mut self, _paths: impl IntoIterator<Item = Utf8PathBuf>) {}

    fn poll(&mut self) -> Option<WatcherEvent> {
        None
    }
}
