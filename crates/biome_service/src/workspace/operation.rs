//! The workspace operations: the request-shaped behaviors of a
//! [Workspace](super::Workspace), with their parameters, their results and the
//! name of the method that carries them over a transport.
//!
//! [workspace_operations] declares every operation once, with the
//! documentation of its [Workspace](super::Workspace) method. Each consumer
//! passes the name of a macro that it defines, and that macro receives every
//! operation as a row:
//!
//! ```ignore
//! /// Documentation of the operation.
//! open_file(OpenFileParams) -> OpenFileResult = "biome/open_file";
//! ```
//!
//! The rows generate the declarations of the [Workspace](super::Workspace)
//! trait and the implementations that only forward each operation. The
//! implementation that performs the operations stays written by hand, so a new
//! operation fails to compile until it has a body. How an implementation
//! executes an operation, such as whether it retries it, is up to the
//! implementation and isn't part of the operation.

/// Calls `$emit!` with one row per workspace operation, in the order of the
/// methods of the [Workspace](super::Workspace) trait. See the
/// [module documentation](self) for the format of a row.
macro_rules! workspace_operations {
    ($emit:ident) => {
        $emit! {
            // #region PROJECT-LEVEL METHODS
            /// Opens a project within the workspace.
            ///
            /// The path given as part of the `params` is used to determine the root of
            /// the Biome project, but it may not have to be the root itself. The first
            /// ancestor directory of the path (including the path itself) that contains
            /// a **top-level** `biome.json` is determined to be the project root and
            /// will be used for opening the project.
            ///
            /// If no `biome.json` file can be found in any of the ancestors, and
            /// `open_uninitialized` is `true` the directory will be opened as a Biome
            /// project will default settings.
            ///
            /// Note: Opening a project does not mean the project is ready for use. You
            /// probably want to follow it up with a call to `scan_project()` or
            /// explicitly load settings into the project using `update_settings()`.
            ///
            /// Returns the key of the opened project and the [ScanKind] of this project.
            ///
            /// The key can be used with follow-up methods to perform actions related to the project,
            /// such as opening files or querying them.
            ///
            /// The `scan_kind` can be used to tell the scanner how it should scan the project.
            open_project(OpenProjectParams) -> OpenProjectResult = "biome/open_project";

            /// Scans the given project, and initializes all settings and service data.
            ///
            /// The first time you call this method, it may take a long time since it
            /// will traverse the entire project folder recursively, parse all included
            /// files (and possibly their dependencies), and perform processing for
            /// extracting the service data.
            ///
            /// Follow-up calls may be much faster as they can reuse cached data.
            ///
            /// If [`ScanProjectParams::watch`] is `true`, this method also
            /// registers file watchers to make sure the cache remains up-to-date.
            scan_project(ScanProjectParams) -> ScanProjectResult = "biome/scan_project";

            /// Updates the global settings for the given project.
            ///
            /// TODO: This method should not be used in combination with
            /// `scan_project()`. When scanning is enabled, the server should
            /// manage project settings on its own.
            update_settings(UpdateSettingsParams) -> UpdateSettingsResult = "biome/update_settings";

            /// Closes the project with the given key.
            ///
            /// Releases the calling client's use of the project. The project is unloaded only after every
            /// client releases it. Filesystem-backed documents that belong only to the unloaded project are
            /// closed. Virtual documents remain open until their clients close them, and later operations
            /// must use another open project as their processing context.
            ///
            /// Projects should normally **not** be closed, so that follow-up
            /// invocations to Biome can reuse the loaded settings and open files. Only
            /// when the user explicitly asks for a project to be closed, will we do so
            /// in order to allow the user to reclaim memory. (Although in reality
            /// they'll probably just kill the daemon anyway.)
            ///
            /// If a file watcher was registered as a result of a call to
            /// `scan_project()`, it will also be unregistered.
            close_project(CloseProjectParams) -> () = "biome/close_project";

            // #endregion
            // #region FILE-LEVEL METHODS
            /// Opens a new file in the workspace.
            ///
            /// Content sent by the client belongs to that client: changes on disk and
            /// requests from other clients don't replace it until the client closes
            /// the file. When several clients have the file open, the services they
            /// share, such as the module graph, use the content of the client that
            /// opened it first.
            ///
            /// If a filesystem path is under a folder that belongs to an opened project
            /// other than the current one, the current project is changed accordingly.
            /// Virtual documents use the project's root settings and do not participate
            /// in filesystem ignore or override matching.
            open_file(OpenFileParams) -> OpenFileResult = "biome/open_file";

            /// Checks if `file_path` exists in the workspace for this client: the
            /// client opened it, or Biome read it from the file system, for example
            /// while scanning the project. A file that only other clients opened
            /// doesn't exist for this client.
            ///
            /// This method is useful to avoid unexpected errors before using the file method and avoid errors.
            ///
            /// ### Error
            ///
            /// It throws an error only if there's an issue with the client transport.
            file_exists(FileExistsParams) -> bool = "biome/file_exists";

            /// Checks whether a certain feature is supported for the given file path.
            ///
            /// There are different conditions:
            /// - Biome doesn't recognize a file, so it can't provide the feature;
            /// - the feature is disabled inside the configuration;
            /// - the file is ignored
            file_features(SupportsFeatureParams) -> FileFeaturesResult = "biome/file_features";

            /// Checks if the given path is ignored by the project for a specific
            /// feature.
            ///
            /// If the file path matches, `true` is returned, and it should be
            /// considered ignored.
            is_path_ignored(PathIsIgnoredParams) -> bool = "biome/is_path_ignored";

            /// Returns the content of a file as this client sees it: the content that
            /// the client sent, if it opened the file, otherwise the content on disk.
            get_file_content(GetFileContentParams) -> String = "biome/get_file_content";

            /// Returns the size of a given file, as well as the allowed maximum file
            /// size for that file.
            check_file_size(CheckFileSizeParams) -> CheckFileSizeResult = "biome/check_file_size";

            /// Changes the content of a file.
            ///
            /// When the client opened the file, the change applies to the content of
            /// the client. Otherwise, the change records the content on disk, for
            /// example after the client wrote the file.
            change_file(ChangeFileParams) -> ChangeFileResult = "biome/change_file";

            /// Processes a file by applying linting, formatting, and assist without changing the open workspace document.
            /// This is a stateless operation, which means that the new document, after the fixes/fomatting, isn't saved in the workspace.
            process_file(ProcessFileParams) -> ProcessFileResult = "biome/process_file";

            /// Retrieves the list of diagnostics associated with a file.
            pull_diagnostics(PullDiagnosticsParams) -> PullDiagnosticsResult = "biome/pull_diagnostics";

            /// Retrieves the list of code actions available for a given cursor
            /// position within a file.
            pull_actions(PullActionsParams) -> PullActionsResult = "biome/pull_actions";

            /// Pulls diagnostics with their relative code actions
            pull_diagnostics_and_actions(PullDiagnosticsAndActionsParams) -> PullDiagnosticsAndActionsResult = "biome/pull_diagnostics_and_actions";

            /// Runs the given file through the formatter using the provided options
            /// and returns the resulting source code.
            format_file(FormatFileParams) -> Printed = "biome/format_file";

            /// Runs a range of an open document through the formatter.
            format_range(FormatRangeParams) -> Printed = "biome/format_range";

            /// Runs a "block" ending at the specified character of an open document
            /// through the formatter.
            format_on_type(FormatOnTypeParams) -> Printed = "biome/format_on_type";

            /// Returns the content of the file with all safe code actions applied.
            fix_file(FixFileParams) -> FixFileResult = "biome/fix_file";

            /// Returns the content of the file after renaming a symbol.
            rename(RenameParams) -> RenameResult = "biome/rename";

            /// Navigates to the definition of the symbol at the given cursor position.
            ///
            /// Returns `None` if the symbol cannot be resolved (e.g., external
            /// dependency, cursor not on an identifier, or no definition found).
            go_to_definition(GoToDefinitionParams) -> Option<GoToDefinitionResult> = "biome/go_to_definition";

            /// Closes a file that this client opened.
            ///
            /// Other clients that have the file open keep their content. When no
            /// client has the file open anymore, the file is unloaded from the
            /// workspace, unless the scanner indexed it: it may still be required for
            /// multi-file analysis, so its content on disk is used again.
            close_file(CloseFileParams) -> () = "biome/close_file";

            /// Updates the internal module graph using the provided path.
            ///
            /// ## Errors
            ///
            /// An error is emitted if the path doesn't exist inside the workspace. Use
            /// the method [Workspace::open_file] before updating the module graph.
            update_module_graph(UpdateModuleGraphParams) -> () = "biome/update_module_graph";

            // #endregion
            // #region SEARCH-RELATED METHODS
            /// Parses a pattern to be used in follow-up [`Self::search_pattern`]
            /// requests.
            ///
            /// Clients should call [`Self::drop_pattern()`] when they no need longer
            /// need it.
            parse_pattern(ParsePatternParams) -> ParsePatternResult = "biome/parse_pattern";

            /// Searches a file for matches of the given pattern.
            search_pattern(SearchPatternParams) -> SearchResults = "biome/search_pattern";

            /// Used to indicate a client no longer needs a specific pattern.
            drop_pattern(DropPatternParams) -> () = "biome/drop_pattern";

            // #endregion
            // #region DEBUGGING METHODS
            /// Returns a textual, debug representation of the syntax tree for a given
            /// document.
            get_syntax_tree(GetSyntaxTreeParams) -> GetSyntaxTreeResult = "biome/get_syntax_tree";

            /// Returns a textual, debug representation of the control flow graph at a
            /// given position in the document.
            get_control_flow_graph(GetControlFlowGraphParams) -> String = "biome/get_control_flow_graph";

            /// Returns a textual, debug representation of the formatter IR for a given
            /// document.
            get_formatter_ir(GetFormatterIRParams) -> String = "biome/get_formatter_ir";

            /// Returns an IR of the type information of the document
            get_type_info(GetTypeInfoParams) -> String = "biome/get_type_info";

            /// Returns the registered types of the document
            get_registered_types(GetRegisteredTypesParams) -> String = "biome/get_registered_types";

            /// Returns a textual, debug representation of the semantic model for the document.
            get_semantic_model(GetSemanticModelParams) -> String = "biome/get_semantic_model";

            /// Returns a serializable version of the module graph
            get_module_graph(GetModuleGraphParams) -> GetModuleGraphResult = "biome/get_module_graph";

            /// Returns debug information about this workspace.
            rage(RageParams) -> RageResult = "biome/rage";

            // #endregion
        }
    };
}

pub(crate) use workspace_operations;
