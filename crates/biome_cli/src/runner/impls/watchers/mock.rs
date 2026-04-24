use crate::runner::watcher::{Watcher, WatcherEvent};
use camino::Utf8PathBuf;
use std::collections::VecDeque;

pub struct MockWatcher {
    events: VecDeque<WatcherEvent>,
    pub watched: Vec<Utf8PathBuf>,
}

impl MockWatcher {
    pub fn with_events(events: Vec<WatcherEvent>) -> Self {
        Self {
            events: VecDeque::from(events),
            watched: Vec::new(),
        }
    }
}

impl Watcher for MockWatcher {
    fn watch(&mut self, paths: Vec<Utf8PathBuf>) {
        self.watched.extend(paths);
    }

    fn poll(&mut self) -> Option<WatcherEvent> {
        self.events.pop_front()
    }
}
