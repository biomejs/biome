use crate::execute::diagnostics::ResultExt;
use crate::execute::diagnostics::ResultIoExt;
use crate::execute::process_file::SharedTraversalOptions;
use biome_diagnostics::{Error, category};
use biome_fs::{BiomePath, File, OpenOptions};
use biome_service::workspace::{FileContent, FileExitsParams, FileGuard, OpenFileParams};
use biome_service::{Workspace, WorkspaceError};
/// Small wrapper that holds information and operations around the current processed file
pub(crate) struct WorkspaceFile<'ctx, 'app> {
    guard: FileGuard<'app, dyn Workspace + 'ctx>,
    file: Box<dyn File>,
    pub(crate) path: BiomePath,
}

impl<'ctx, 'app> WorkspaceFile<'ctx, 'app> {
    /// A wrapper that knows how to read and write a file on the file system.
    /// It the file doesn't exist in the workspace, it opens it.
    ///
    /// If you need to operate with the Workspace, call [WorkspaceFile::guard], which returns
    /// a type that allows to operate with the workspace without operating with the file system.
    pub(crate) fn new(
        ctx: &SharedTraversalOptions<'ctx, 'app>,
        path: BiomePath,
    ) -> Result<Self, Error> {
        let open_options = OpenOptions::default()
            .read(true)
            .write(ctx.execution.requires_write_access());
        let mut file = ctx
            .fs
            .open_with_options(path.as_path(), open_options)
            .with_file_path(path.to_string())?;

        let guard = FileGuard::new(ctx.workspace, ctx.project_key, path.clone())
            .with_file_path_and_code(path.to_string(), category!("internalError/fs"))?;

        if ctx.workspace.file_exists(FileExitsParams {
            file_path: path.clone(),
        })? {
            Ok(Self { guard, path, file })
        } else {
            let mut input = String::new();
            file.read_to_string(&mut input)
                .with_file_path(path.to_string())?;

            ctx.workspace.open_file(OpenFileParams {
                project_key: ctx.project_key,
                document_file_source: None,
                path: path.clone(),
                content: FileContent::from_client(&input),
                persist_node_cache: false,
            })?;

            Ok(Self { guard, path, file })
        }
    }

    pub(crate) fn guard(&self) -> &FileGuard<'app, dyn Workspace + 'ctx> {
        &self.guard
    }

    pub(crate) fn input(&self) -> Result<String, WorkspaceError> {
        self.guard().get_file_content()
    }

    pub(crate) fn as_extension(&self) -> Option<&str> {
        self.path.extension()
    }

    /// It updates the workspace file with `new_content`
    pub(crate) fn update_file(&mut self, new_content: impl Into<String>) -> Result<(), Error> {
        let new_content = new_content.into();

        self.file
            .set_content(new_content.as_bytes())
            .with_file_path(self.path.to_string())?;
        self.guard
            .change_file(self.file.file_version(), new_content)?;
        Ok(())
    }
}
