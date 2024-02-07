use crate::execute::diagnostics::{ResultExt, ResultIoExt};
use crate::execute::process_file::SharedTraversalOptions;
use biome_diagnostics::{category, Error};
use biome_fs::{File, OpenOptions, RomePath};
use biome_service::file_handlers::Language;
use biome_service::workspace::{FileGuard, OpenFileParams};
use biome_service::{Workspace, WorkspaceError};
use std::path::{Path, PathBuf};

/// Small wrapper that holds information and operations around the current processed file
pub(crate) struct WorkspaceFile<'ctx, 'app> {
    guard: FileGuard<'app, dyn Workspace + 'ctx>,
    file: Box<dyn File>,
    pub(crate) path: PathBuf,
}

impl<'ctx, 'app> WorkspaceFile<'ctx, 'app> {
    /// It attempts to read the file from disk, creating a [FileGuard] and
    /// saving these information internally
    pub(crate) fn new(
        ctx: &SharedTraversalOptions<'ctx, 'app>,
        path: &Path,
    ) -> Result<Self, Error> {
        let rome_path = RomePath::new(path);
        let open_options = OpenOptions::default()
            .read(true)
            .write(ctx.execution.requires_write_access());
        let mut file = ctx
            .fs
            .open_with_options(path, open_options)
            .with_file_path(path.display().to_string())?;

        let mut input = String::new();
        file.read_to_string(&mut input)
            .with_file_path(path.display().to_string())?;

        let guard = FileGuard::open(
            ctx.workspace,
            OpenFileParams {
                path: rome_path,
                version: 0,
                content: input.clone(),
                language_hint: Language::default(),
            },
        )
        .with_file_path_and_code(path.display().to_string(), category!("internalError/fs"))?;

        Ok(Self {
            file,
            guard,
            path: PathBuf::from(path),
        })
    }

    pub(crate) fn guard(&self) -> &FileGuard<'app, dyn Workspace + 'ctx> {
        &self.guard
    }

    pub(crate) fn input(&self) -> Result<String, WorkspaceError> {
        self.guard().get_file_content()
    }

    pub(crate) fn as_extension(&self) -> Option<&str> {
        self.path.extension().and_then(|s| s.to_str())
    }

    /// It updates the workspace file with `new_content`
    pub(crate) fn update_file(&mut self, new_content: impl Into<String>) -> Result<(), Error> {
        let new_content = new_content.into();

        self.file
            .set_content(new_content.as_bytes())
            .with_file_path(self.path.display().to_string())?;
        self.guard
            .change_file(self.file.file_version(), new_content)?;
        Ok(())
    }
}
