//! Parsed files shared by every program in the process.
//!
//! Without sharing, each project parses its own copy of every file it reaches, and projects of the same
//! repository reach mostly the same `lib.*.d.ts` and `node_modules` declarations. A parsed file depends only
//! on its parse options and text, and tsrs binds it once whichever program asks first; type information lives
//! in each program's checkers. tsgo's project system shares files between programs the same way.

use std::fmt::Display;
use std::sync::{Arc, LazyLock, Mutex};

use rustc_hash::FxHashMap;
use tsrs_ast::{SourceFile, SourceFileParseOptions};
use tsrs_compiler::CompilerHost;
use tsrs_contentmapper::{self as contentmapper, Mapper, Project};
use tsrs_core::tspath::Path;
use tsrs_core::{P, ensure_script_kind_from_file_name};
use tsrs_diagnostics::Message;
use tsrs_tsoptions::ParsedCommandLine;
use tsrs_vfs::{FS, bundled};

/// The latest parse of each file. A file whose text changed is parsed again and replaces its entry; programs
/// that hold the older parse keep it.
static PARSED_FILES: LazyLock<Mutex<FxHashMap<SourceFileParseOptions, P<SourceFile>>>> =
    LazyLock::new(Default::default);

/// A compiler host that takes parsed files from [`PARSED_FILES`] and delegates everything else.
pub(crate) struct SharedParseHost {
    inner: Arc<dyn CompilerHost>,
}

impl SharedParseHost {
    pub(crate) fn new(inner: Arc<dyn CompilerHost>) -> Self {
        Self { inner }
    }

    fn cached(options: &SourceFileParseOptions, text: Option<&str>) -> Option<P<SourceFile>> {
        let file = *PARSED_FILES.lock().unwrap().get(options)?;
        text.is_none_or(|text| file.text() == text).then_some(file)
    }

    /// Stores `file` unless another thread stored a parse of the same text first, and returns the stored parse.
    fn store(options: SourceFileParseOptions, file: P<SourceFile>) -> P<SourceFile> {
        let mut files = PARSED_FILES.lock().unwrap();
        match files.get(&options) {
            Some(&existing) if existing.text() == file.text() => existing,
            _ => {
                files.insert(options, file);
                file
            }
        }
    }
}

impl CompilerHost for SharedParseHost {
    fn fs(&self) -> &dyn FS {
        self.inner.fs()
    }

    fn default_library_path(&self) -> &str {
        self.inner.default_library_path()
    }

    fn get_current_directory(&self) -> &str {
        self.inner.get_current_directory()
    }

    fn trace(&self, msg: &'static Message, args: &[&dyn Display]) {
        self.inner.trace(msg, args);
    }

    fn get_source_file(&self, options: SourceFileParseOptions) -> Option<P<SourceFile>> {
        // Bundled libs are text in the binary, which never changes.
        if bundled::embedded_file(&options.file_name).is_some() {
            if let Some(file) = Self::cached(&options, None) {
                return Some(file);
            }
            let file = self.inner.get_source_file(options.clone())?;
            return Some(Self::store(options, file));
        }
        let text = self.inner.fs().read_file(&options.file_name)?;
        if let Some(file) = Self::cached(&options, Some(&text)) {
            return Some(file);
        }
        let script_kind = ensure_script_kind_from_file_name(&options.file_name);
        let file = tsrs_parser::parse_source_file_owned(options.clone(), text, script_kind);
        Some(Self::store(options, file))
    }

    fn get_content_mapped_source_files(
        &self,
        parse_options: SourceFileParseOptions,
        mapper: &Mapper,
    ) -> Result<contentmapper::SourceFiles, contentmapper::Error> {
        self.inner
            .get_content_mapped_source_files(parse_options, mapper)
    }

    fn content_mapper_project(&self) -> Option<Arc<dyn Project>> {
        self.inner.content_mapper_project()
    }

    fn get_resolved_project_reference(
        &self,
        file_name: &str,
        path: Path,
    ) -> Option<P<ParsedCommandLine>> {
        self.inner.get_resolved_project_reference(file_name, path)
    }
}
