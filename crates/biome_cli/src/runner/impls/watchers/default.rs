use std::sync::mpsc::{Receiver, channel};

use crate::runner::diagnostics::WatcherDiagnostic;
use crate::runner::watcher::{Watcher, WatcherEvent};
use biome_diagnostics::{Error, NotifyError};
use camino::Utf8PathBuf;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify::{Event, EventKind, RecursiveMode, Result, recommended_watcher};
use tracing::warn;

pub(crate) struct DefaultWatcher {
    rx: Receiver<Result<Event>>,
    watcher: Box<dyn notify::Watcher>,
}

impl DefaultWatcher {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            rx,
            watcher: Box::new(recommended_watcher(tx).expect("watcher created")),
        }
    }
}

impl Watcher for DefaultWatcher {
    fn watch(&mut self, paths: Vec<Utf8PathBuf>) {
        let mut watched_paths = self.watcher.paths_mut();

        for path in paths {
            if let Err(e) = watched_paths.add(path.as_std_path(), RecursiveMode::Recursive) {
                warn!("Failed to watch path {}: {}", path, e);
            }
        }
        if let Err(e) = watched_paths.commit() {
            warn!("Failed to commit watched paths: {}", e);
        }
    }

    fn poll(&mut self) -> Option<WatcherEvent> {
        self.rx.iter().find_map(|event| {
            match event {
                Err(err) => Some(WatcherEvent::Error(WatcherDiagnostic {
                    source: Some(Error::from(NotifyError::from(err))),
                })),
                Ok(event) => {
                    // Modifying folder or metadata is ignored as it can unlikely affect the results.
                    // Any event types are necessary for some platforms to catch events.
                    if matches!(
                        event.kind,
                        EventKind::Create(CreateKind::File | CreateKind::Any)
                            | EventKind::Modify(
                                ModifyKind::Name(_) | ModifyKind::Data(_) | ModifyKind::Any
                            )
                            | EventKind::Remove(RemoveKind::File | RemoveKind::Any)
                            | EventKind::Any
                    ) {
                        Some(WatcherEvent::Changed(
                            event
                                .paths
                                .into_iter()
                                .filter_map(|path| Utf8PathBuf::from_path_buf(path).ok())
                                .collect(),
                        ))
                    } else {
                        None
                    }
                }
            }
        })
    }
}
