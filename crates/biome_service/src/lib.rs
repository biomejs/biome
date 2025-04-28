#![deny(clippy::use_self)]

pub mod documentation;
pub mod file_handlers;

pub mod projects;
pub mod settings;
pub mod workspace;
mod workspace_watcher;

pub mod configuration;
pub mod diagnostics;
pub mod dome;
#[cfg(feature = "schema")]
pub mod workspace_types;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use biome_console::Console;
use biome_fs::{FileSystem, OsFileSystem};

pub use diagnostics::{TransportError, WorkspaceError, extension_error};
pub use file_handlers::JsFormatterSettings;
pub use workspace::{Workspace, WorkspaceServer};
pub use workspace_watcher::{WatcherInstruction, WorkspaceWatcher};

/// Path entries that should be ignored in the workspace, even by the scanner.
///
/// These cannot (yet) be configured.
pub const IGNORE_ENTRIES: &[&[u8]] = &[
    b".cache",
    b".git",
    b".hg",
    b".netlify",
    b".output",
    b".svn",
    b".yarn",
    b".timestamp",
    b".turbo",
    b".vercel",
    b".DS_Store",
];

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
        Self::with_filesystem_and_console(Box::new(OsFileSystem::default()), console)
    }

    /// Create a new instance of the app using the specified [FileSystem] and [Console] implementation
    pub fn with_filesystem_and_console(
        fs: Box<dyn FileSystem>,
        console: &'app mut dyn Console,
    ) -> Self {
        Self::new(console, WorkspaceRef::Owned(workspace::server(fs, None)))
    }

    /// Create a new instance of the app using the specified [FileSystem], [Console] and [Workspace] implementation
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
