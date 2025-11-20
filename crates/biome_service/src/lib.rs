#![deny(clippy::use_self)]

pub mod configuration;
pub mod diagnostics;
pub mod documentation;
pub mod file_handlers;
pub mod projects;
pub mod settings;
pub mod workspace;

#[cfg(feature = "schema")]
pub mod workspace_types;

mod scanner;
mod utils;

#[cfg(test)]
mod test_utils;

use camino::Utf8Path;
use std::ops::Deref;
use std::sync::Arc;

use biome_console::Console;
use biome_fs::OsFileSystem;
use biome_resolver::FsWithResolverProxy;

pub use diagnostics::{TransportError, WorkspaceError, extension_error};
pub use file_handlers::JsFormatterSettings;
pub use scanner::{Watcher, WatcherInstruction};
pub use workspace::{Workspace, WorkspaceServer};

/// This is the main entrypoint of the application.
pub struct App<'app> {
    /// A reference to the internal workspace
    pub workspace: WorkspaceRef<'app>,
    /// A reference to the internal console, where its buffer will be used to write messages and
    /// errors
    pub console: &'app mut dyn Console,
}

impl<'app> App<'app> {
    pub fn with_console(console: &'app mut dyn Console) -> Self {
        Self::with_filesystem_and_console(Arc::new(OsFileSystem::default()), console)
    }

    /// Create a new instance of the app using the specified [FsWithResolverProxy] and [Console] implementation
    pub fn with_filesystem_and_console(
        fs: Arc<dyn FsWithResolverProxy>,
        console: &'app mut dyn Console,
    ) -> Self {
        Self::new(console, WorkspaceRef::Owned(workspace::server(fs, None)))
    }

    /// Create a new instance of the app using the specified [Console] and [Workspace] implementation
    pub fn new(console: &'app mut dyn Console, workspace: WorkspaceRef<'app>) -> Self {
        Self { console, workspace }
    }
}

pub enum WorkspaceRef<'app> {
    Owned(Box<dyn Workspace>),
    Borrowed(&'app dyn Workspace),
}

impl<'app> Deref for WorkspaceRef<'app> {
    type Target = dyn Workspace + 'app;

    fn deref(&self) -> &Self::Target {
        match self {
            WorkspaceRef::Owned(inner) => &**inner,
            WorkspaceRef::Borrowed(inner) => *inner,
        }
    }
}

/// Returns `true` if `path` is a directory or
/// if it is a symlink that resolves to a directory.
fn is_dir(path: &Utf8Path) -> bool {
    path.is_dir() || (path.is_symlink() && path.read_link_utf8().is_ok_and(|path| path.is_dir()))
}
