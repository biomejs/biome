use biome_console::Console;
use biome_fs::{FileSystem, OsFileSystem};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub mod documentation;
pub mod file_handlers;

pub mod matcher;
pub mod settings;
pub mod workspace;

pub mod configuration;
pub mod diagnostics;
pub mod dome;
#[cfg(feature = "schema")]
pub mod workspace_types;

pub use crate::matcher::Matcher;

pub use crate::diagnostics::{TransportError, WorkspaceError};
/// Exports only for this crate
pub use crate::file_handlers::JsFormatterSettings;
pub use crate::workspace::Workspace;
pub use diagnostics::extension_error;

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
        Self::new(console, WorkspaceRef::Owned(workspace::server(fs)))
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

    // False positive
    #[allow(clippy::explicit_auto_deref)]
    fn deref(&self) -> &Self::Target {
        match self {
            WorkspaceRef::Owned(inner) => &**inner,
            WorkspaceRef::Borrowed(inner) => *inner,
        }
    }
}
