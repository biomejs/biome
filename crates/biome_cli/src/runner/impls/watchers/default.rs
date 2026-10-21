use std::sync::mpsc::{Receiver, channel};

use crate::runner::watcher::{Watcher, WatcherEvent};
use camino::{Utf8Path, Utf8PathBuf};
use tracing::warn;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify::{Event, EventKind, RecursiveMode, Result, recommended_watcher};

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

/// Returns `true` if the path lives inside a directory that Biome should not
/// surface to the user even when it appears in a watcher event.
///
/// Today this only covers the `.git` directory. It exists so that transient
/// VCS bookkeeping (such as `.git/index.lock` updates) does not produce
/// `internalError/io` diagnostics in `--watch` mode, even when the project's
/// `.gitignore` does not exclude `.git` or VCS integration is disabled.
fn is_internal_vcs_path(path: &Utf8Path) -> bool {
    path.components().any(|c| match c {
        camino::Utf8Component::Normal(part) => part == ".git",
        _ => false,
    })
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
                Err(err) => {
                    // `notify` surfaces filesystem errors that occur on the
                    // watched paths (for example, transient permission
                    // failures while reading a file inside `.git`). Those
                    // events should never reach Biome's error reporter, since
                    // they do not reflect problems with the user's code or
                    // configuration. Drop the event so the watcher keeps
                    // running.
                    warn!("Watcher event error (suppressed): {err}");
                    None
                }
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
                        let paths: Vec<Utf8PathBuf> = event
                            .paths
                            .into_iter()
                            .filter_map(|path| Utf8PathBuf::from_path_buf(path).ok())
                            // `.git/` paths come from VCS bookkeeping (e.g.
                            // `index.lock` churn) and should not trigger a
                            // re-crawl. Filtering at the watcher level keeps
                            // the workspace scanner's ignore contract intact
                            // while removing the spurious diagnostics that
                            // otherwise surface in watch mode.
                            .filter(|path| !is_internal_vcs_path(path))
                            .collect();
                        if paths.is_empty() {
                            None
                        } else {
                            Some(WatcherEvent::Changed(paths))
                        }
                    } else {
                        None
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::is_internal_vcs_path;

    #[test]
    fn detects_path_inside_dot_git_directory() {
        assert!(is_internal_vcs_path(Utf8Path::new(
            "/repo/.git/index.lock"
        )));
        assert!(is_internal_vcs_path(Utf8Path::new("repo/.git/HEAD")));
        assert!(is_internal_vcs_path(Utf8Path::new(
            "/repo/sub/.git/index.lock"
        )));
    }

    #[test]
    fn leaves_non_git_paths_alone() {
        assert!(!is_internal_vcs_path(Utf8Path::new("/repo/src/index.js")));
        assert!(!is_internal_vcs_path(Utf8Path::new(
            "/repo/.github/workflows/ci.yml"
        )));
        assert!(!is_internal_vcs_path(Utf8Path::new(
            "/repo/some/path/git-notes.txt"
        )));
    }
}
