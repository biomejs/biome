//! Experimental bridge from Biome to the [tsrs](https://github.com/oxc-project/tsrs) type checker.
//!
//! tsrs parses every file itself: its checker re-scans source text (lazy JSDoc, token positions, identifier
//! text), so it needs the AST its own parser produces. Biome nodes are matched to tsrs nodes by byte range.
//! Both sides use UTF-8 byte offsets, but a tsrs node's `pos` includes its leading trivia, so a Biome range is
//! compared against the tsrs token start.

use std::sync::{Arc, LazyLock, Mutex, RwLock};
use std::time::SystemTime;

use biome_rowan::TextRange;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;
use tsrs_ast::{Node, SourceFile};
use tsrs_checker::{Checker, Type, TypeFacts, TypeFlags};
use tsrs_compiler::{CompilerHost, Program, ProgramOptions, new_compiler_host, new_program};
use tsrs_core::{
    CompilerOptions, Context, ModuleDetectionKind, ModuleKind, ModuleResolutionKind, P,
    ScriptTarget, Tristate, tspath::ComparePathsOptions,
};
use tsrs_tsoptions::{ParseConfigHost, ParsedCommandLine, new_parsed_command_line};
use tsrs_vfs::{Entries, FS, FileInfo, bundled, osvfs};

/// One project checked by tsrs: a program over all of its files, kept in sync with the text Biome analyzes.
///
/// Every program the project builds, and everything it allocates, lives for the rest of the process.
pub struct TsrsProject {
    cwd: String,
    overlay: Arc<OverlayFs>,
    program: Mutex<&'static Program>,
}

/// The project's program at a moment when one file had the text Biome analyzed.
///
/// A snapshot stays valid when the project moves on to a newer program.
#[derive(Clone, Copy)]
pub struct TsrsSnapshot {
    program: &'static Program,
    file: P<SourceFile>,
}

/// A kind of value whose possibility decides whether a branch is reachable.
#[derive(Clone, Copy, Debug)]
pub enum ValueKind {
    Truthy,
    Falsy,
    Nullish,
}

struct ConfigHost {
    fs: Arc<dyn FS>,
    cwd: String,
}

impl ParseConfigHost for ConfigHost {
    fn fs(&self) -> &dyn FS {
        &*self.fs
    }
    fn get_current_directory(&self) -> &str {
        &self.cwd
    }
}

impl TsrsProject {
    /// Builds the project a `tsconfig.json` describes, reading files from disk.
    pub fn from_tsconfig(tsconfig: &Utf8Path) -> Result<Self, String> {
        let cwd = tsconfig.parent().map_or("/", Utf8Path::as_str).to_string();
        let overlay = Arc::new(OverlayFs::new(os_fs()));
        let host: &'static ConfigHost = Box::leak(Box::new(ConfigHost {
            fs: overlay.clone(),
            cwd: cwd.clone(),
        }));
        let (config, diagnostics) = tsrs_tsoptions::get_parsed_command_line_of_config_file(
            tsconfig.as_str(),
            None,
            None,
            host,
            None,
        );
        let config = config.ok_or_else(|| format!("{} config diagnostics", diagnostics.len()))?;
        Ok(Self::new(P::new(config), &cwd, overlay))
    }

    /// Builds a project over every JavaScript and TypeScript file under `directory`, outside `node_modules`,
    /// with Biome's default options.
    pub fn from_directory(directory: &Utf8Path) -> Self {
        let mut root_files = Vec::new();
        collect_source_files(directory, &mut root_files);
        let root_files: Vec<_> = root_files.iter().map(Utf8PathBuf::as_path).collect();
        Self::from_root_files_in(directory, &root_files, os_fs())
    }

    /// Builds a project over `root_files` with Biome's default options, reading files from `fs`.
    ///
    /// Every file is a module, as Biome parses it, and JavaScript files are checked too. The bundled
    /// `lib.*.d.ts` files are always available, whatever `fs` contains.
    pub fn from_root_files_in(cwd: &Utf8Path, root_files: &[&Utf8Path], fs: Arc<dyn FS>) -> Self {
        let options = CompilerOptions {
            allow_js: Tristate::True,
            module_detection: ModuleDetectionKind::Force,
            strict: Tristate::True,
            target: ScriptTarget::ESNext,
            module: ModuleKind::ESNext,
            module_resolution: ModuleResolutionKind::Bundler,
            skip_lib_check: Tristate::True,
            no_emit: Tristate::True,
            ..Default::default()
        };
        let config = new_parsed_command_line(
            P::new(options),
            root_files.iter().map(|path| path.to_string()).collect(),
            Vec::new(),
            ComparePathsOptions {
                use_case_sensitive_file_names: fs.use_case_sensitive_file_names(),
                current_directory: cwd.to_string(),
            },
        );
        Self::new(P::new(config), cwd.as_str(), Arc::new(OverlayFs::new(fs)))
    }

    fn new(config: P<ParsedCommandLine>, cwd: &str, overlay: Arc<OverlayFs>) -> Self {
        let host = new_host(cwd, &overlay);
        let program = run_on_tsrs_pool(move || {
            let program = new_program(ProgramOptions::new(config, host));
            prepare(program);
            program
        });
        Self {
            cwd: cwd.to_string(),
            overlay,
            program: Mutex::new(program),
        }
    }

    /// Returns a snapshot of the project in which `file` has exactly `text`.
    ///
    /// When the program read other text for `file`, `text` replaces it and the program is updated; the
    /// other files are reused unless the change affects them (new imports, for example). Returns `None`
    /// when `file` isn't part of the project.
    pub fn snapshot(&self, file: &Utf8Path, text: &str) -> Option<TsrsSnapshot> {
        let mut program = self.program.lock().unwrap();
        let source_file = program.get_source_file(file.as_str())?;
        if source_file.text() == text {
            return Some(TsrsSnapshot {
                program: *program,
                file: source_file,
            });
        }
        if std::env::var_os("BIOME_TSRS").is_some_and(|value| value == "verbose") {
            eprintln!("tsrs: updating {file}");
        }
        self.overlay.set(file.as_str(), text);
        let path = program.to_path(file.as_str());
        let host = new_host(&self.cwd, &self.overlay);
        let current = *program;
        let updated = run_on_tsrs_pool(move || {
            let (updated, _, _) = current.update_program(&path, host, None, None);
            prepare(updated);
            updated
        });
        *program = updated;
        Some(TsrsSnapshot {
            program: updated,
            file: updated.get_source_file(file.as_str())?,
        })
    }
}

impl TsrsSnapshot {
    /// Runs `f` with the checker of the snapshot's file and the type of the expression whose trimmed range is
    /// `range`.
    ///
    /// Returns `None` when no expression has exactly that range.
    pub fn with_type_of_expression<R>(
        &self,
        range: TextRange,
        f: impl FnOnce(&mut Checker, P<Type>) -> R,
    ) -> Option<R> {
        let node = find_expression(self.file, range)?;
        let mut checker = self
            .program
            .get_type_checker_for_file(&Context::background(), self.file);
        let ty = checker.get_type_at_location(node);
        Some(f(&mut checker, ty))
    }

    /// Prints the type of the expression at `range` as TypeScript would in a diagnostic.
    pub fn type_to_string(&self, range: TextRange) -> Option<String> {
        self.with_type_of_expression(range, |checker, ty| checker.type_to_string_exported(ty))
    }

    /// Whether the expression at `range` may evaluate to a value of the given kind, as the checker decides
    /// whether a `&&`, `||`, `??` or `?:` operand is reachable.
    pub fn may_be(&self, range: TextRange, kind: ValueKind) -> Option<bool> {
        let facts = match kind {
            ValueKind::Truthy => TypeFacts::Truthy,
            ValueKind::Falsy => TypeFacts::Falsy,
            ValueKind::Nullish => TypeFacts::EQUndefinedOrNull,
        };
        self.with_type_of_expression(range, |checker, ty| checker.has_type_facts(ty, facts))
    }

    /// Whether the expression at `range` is a Promise, as typescript-eslint's `no-floating-promises` decides
    /// with its default options.
    pub fn is_promise_like(&self, range: TextRange) -> Option<bool> {
        self.with_type_of_expression(range, |checker, ty| self.is_promise_like_type(checker, ty))
    }

    /// Whether the expression at `range` is an array or tuple whose elements are Promises.
    pub fn is_promise_array(&self, range: TextRange) -> Option<bool> {
        self.with_type_of_expression(range, |checker, ty| {
            let apparent = checker.get_apparent_type(ty);
            union_constituents(apparent).iter().any(|&part| {
                if checker.is_array_type(part) {
                    let [element] = checker.get_type_arguments(part) else {
                        return false;
                    };
                    self.is_promise_like_type(checker, *element)
                } else if checker.is_array_or_tuple_type(part) {
                    let elements = checker.get_type_arguments(part);
                    elements
                        .iter()
                        .any(|&element| self.is_promise_like_type(checker, element))
                } else {
                    false
                }
            })
        })
    }

    fn is_promise_like_type(&self, checker: &mut Checker, ty: P<Type>) -> bool {
        let apparent = checker.get_apparent_type(ty);
        union_constituents(apparent)
            .iter()
            .any(|&part| self.is_builtin_symbol_like(checker, part, "Promise"))
    }

    /// typescript-eslint's `isBuiltinSymbolLike`: `ty` is the lib type `name`, or derives from it.
    fn is_builtin_symbol_like(&self, checker: &mut Checker, ty: P<Type>, name: &str) -> bool {
        let flags = ty.flags();
        if flags.intersects(TypeFlags::UnionOrIntersection) {
            let parts = ty.types();
            return if flags.intersects(TypeFlags::Union) {
                parts
                    .iter()
                    .all(|&part| self.is_builtin_symbol_like(checker, part, name))
            } else {
                parts
                    .iter()
                    .any(|&part| self.is_builtin_symbol_like(checker, part, name))
            };
        }
        if let Some(symbol) = ty.symbol()
            && symbol.name() == name
            && self.is_declared_in_default_lib(symbol.declarations())
        {
            return true;
        }
        if flags.intersects(TypeFlags::Object) {
            let target = if ty
                .object_flags()
                .intersects(tsrs_checker::ObjectFlags::Reference)
            {
                ty.as_type_reference().target.get().unwrap_or(ty)
            } else {
                ty
            };
            if target
                .object_flags()
                .intersects(tsrs_checker::ObjectFlags::ClassOrInterface)
            {
                let bases = checker.get_base_types(target);
                return bases
                    .iter()
                    .any(|&base| self.is_builtin_symbol_like(checker, base, name));
            }
        }
        false
    }

    fn is_declared_in_default_lib(&self, declarations: &[P<Node>]) -> bool {
        declarations.iter().any(|&declaration| {
            tsrs_ast::get_source_file_of_node(declaration)
                .is_some_and(|file| self.program.is_source_file_default_library(file.path()))
        })
    }
}

/// The threads that build and update programs.
///
/// tsrs parses, binds and creates checkers with rayon. A rayon worker that waits for that work, even on another
/// pool, runs other jobs of its own pool meanwhile, and deadlocks when one of them waits for a lock the worker
/// holds (a project's program, or the `OnceLock` behind a program's checkers). Callers send the work here and
/// block on a channel instead, which runs nothing else.
static TSRS_POOL: LazyLock<rayon::ThreadPool> = LazyLock::new(|| {
    rayon::ThreadPoolBuilder::new()
        .thread_name(|index| format!("tsrs-{index}"))
        .build()
        .expect("the tsrs thread pool starts")
});

/// Binds `program` and creates its checkers, the last work on a program that uses rayon. Queries only lock a
/// checker afterwards, so they can run on any thread.
fn prepare(program: &'static Program) {
    program.bind_source_files();
    drop(program.get_type_checker(&Context::background()));
}

fn run_on_tsrs_pool<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    TSRS_POOL.spawn(move || {
        let _ = sender.send(work());
    });
    receiver.recv().expect("tsrs work doesn't panic")
}

fn os_fs() -> Arc<dyn FS> {
    Arc::new(bundled::wrap_fs(osvfs::fs()))
}

fn new_host(cwd: &str, overlay: &Arc<OverlayFs>) -> Arc<dyn CompilerHost> {
    new_compiler_host(
        cwd,
        overlay.clone(),
        &bundled::default_lib_path(),
        None,
        None,
        None,
    )
}

const SOURCE_EXTENSIONS: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];

fn collect_source_files(directory: &Utf8Path, files: &mut Vec<Utf8PathBuf>) {
    let Ok(entries) = directory.read_dir_utf8() else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            if entry.file_name() != "node_modules" && !entry.file_name().starts_with('.') {
                collect_source_files(path, files);
            }
        } else if path
            .extension()
            .is_some_and(|extension| SOURCE_EXTENSIONS.contains(&extension))
        {
            files.push(path.to_path_buf());
        }
    }
}

/// A file system that serves the text Biome analyzed in place of what's on disk.
struct OverlayFs {
    base: Arc<dyn FS>,
    files: RwLock<FxHashMap<String, String>>,
}

impl OverlayFs {
    fn new(base: Arc<dyn FS>) -> Self {
        Self {
            base,
            files: RwLock::default(),
        }
    }

    fn set(&self, path: &str, text: &str) {
        self.files
            .write()
            .unwrap()
            .insert(path.to_string(), text.to_string());
    }
}

impl FS for OverlayFs {
    fn use_case_sensitive_file_names(&self) -> bool {
        self.base.use_case_sensitive_file_names()
    }
    fn file_exists(&self, path: &str) -> bool {
        self.files.read().unwrap().contains_key(path) || self.base.file_exists(path)
    }
    fn read_file(&self, path: &str) -> Option<String> {
        match self.files.read().unwrap().get(path) {
            Some(text) => Some(text.clone()),
            None => self.base.read_file(path),
        }
    }
    fn write_file(&self, path: &str, data: &str) -> Result<(), String> {
        self.base.write_file(path, data)
    }
    fn append_file(&self, path: &str, data: &str) -> Result<(), String> {
        self.base.append_file(path, data)
    }
    fn remove(&self, path: &str) -> Result<(), String> {
        self.base.remove(path)
    }
    fn chtimes(&self, path: &str, a_time: SystemTime, m_time: SystemTime) -> Result<(), String> {
        self.base.chtimes(path, a_time, m_time)
    }
    fn directory_exists(&self, path: &str) -> bool {
        self.base.directory_exists(path)
    }
    fn get_accessible_entries(&self, path: &str) -> Entries {
        self.base.get_accessible_entries(path)
    }
    fn stat(&self, path: &str) -> Option<FileInfo> {
        self.base.stat(path)
    }
    fn realpath(&self, path: &str) -> String {
        self.base.realpath(path)
    }
}

fn union_constituents(ty: P<Type>) -> Vec<P<Type>> {
    if ty.flags().intersects(TypeFlags::Union) {
        ty.types().to_vec()
    } else {
        vec![ty]
    }
}

/// Finds the outermost expression node of `file` whose token start and end match `range`.
fn find_expression(file: P<SourceFile>, range: TextRange) -> Option<P<Node>> {
    let start = u32::from(range.start());
    let end = u32::from(range.end());
    let mut current = file.as_node();
    loop {
        let mut next = None;
        current.for_each_child(&mut |child| {
            let (Some(pos), Some(child_end)) =
                (child.pos().source_offset(), child.end().source_offset())
            else {
                return false;
            };
            if pos > start || child_end < end {
                return false;
            }
            next = Some(child);
            true
        });
        let child = next?;
        if child.end().source_offset() == Some(end)
            && tsrs_scanner::get_token_pos_of_node(child, file, false) == start
            && tsrs_ast::is_expression_node(child)
        {
            return Some(child);
        }
        current = child;
    }
}

#[cfg(test)]
mod tests;
