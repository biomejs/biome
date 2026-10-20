//! Experimental bridge from Biome to the [tsrs](https://github.com/oxc-project/tsrs) type checker.
//!
//! tsrs parses every file itself: its checker re-scans source text (lazy JSDoc, token positions, identifier
//! text), so it needs the AST its own parser produces. Biome nodes are matched to tsrs nodes by byte range.
//! Both sides use UTF-8 byte offsets, but a tsrs node's `pos` includes its leading trivia, so a Biome range is
//! compared against the tsrs token start.

use std::sync::Arc;

use biome_rowan::TextRange;
use camino::Utf8Path;
use tsrs_ast::{Node, SourceFile};
use tsrs_checker::{Checker, Type, TypeFacts, TypeFlags};
use tsrs_compiler::{CompilerHost, Program, ProgramOptions, new_compiler_host, new_program};
use tsrs_core::{
    CompilerOptions, Context, ModuleDetectionKind, ModuleKind, ModuleResolutionKind, P,
    ScriptTarget, Tristate, tspath::ComparePathsOptions,
};
use tsrs_tsoptions::{ParseConfigHost, ParsedCommandLine, new_parsed_command_line};
use tsrs_vfs::{FS, bundled, osvfs};

/// A tsrs program: the parsed, bound files of one project plus its checkers.
///
/// The program and everything it allocates live for the rest of the process.
pub struct TsrsProgram {
    program: &'static Program,
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

impl TsrsProgram {
    /// Builds the program a `tsconfig.json` describes, reading files from disk.
    pub fn from_tsconfig(tsconfig: &Utf8Path) -> Result<Self, String> {
        let cwd = tsconfig.parent().map_or("/", Utf8Path::as_str).to_string();
        let fs = os_fs();
        let host: &'static ConfigHost = Box::leak(Box::new(ConfigHost {
            fs: Arc::clone(&fs),
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
        Ok(Self::new(P::new(config), &cwd, fs))
    }

    /// Builds a program over `root_files` with Biome's default options, reading files from disk.
    ///
    /// Every file is a module, as Biome parses it, and JavaScript files are checked too.
    pub fn from_root_files(cwd: &Utf8Path, root_files: &[&Utf8Path]) -> Self {
        Self::from_root_files_in(cwd, root_files, os_fs())
    }

    /// Builds a program over `root_files` with Biome's default options, reading files from `fs`.
    ///
    /// The bundled `lib.*.d.ts` files are always available, whatever `fs` contains.
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
        Self::new(P::new(config), cwd.as_str(), fs)
    }

    fn new(config: P<ParsedCommandLine>, cwd: &str, fs: Arc<dyn FS>) -> Self {
        let host: Arc<dyn CompilerHost> =
            new_compiler_host(cwd, fs, &bundled::default_lib_path(), None, None, None);
        let mut options = ProgramOptions::new(config, host);
        options.single_threaded = Tristate::True;
        let program = new_program(options);
        program.bind_source_files();
        Self { program }
    }

    /// Runs `f` with the checker of `file` and the type of the expression whose trimmed range is `range`.
    ///
    /// Returns `None` when the file isn't part of the program or no expression has exactly that range.
    pub fn with_type_of_expression<R>(
        &self,
        file: &Utf8Path,
        range: TextRange,
        f: impl FnOnce(&mut Checker, P<Type>) -> R,
    ) -> Option<R> {
        let source_file = self.program.get_source_file(file.as_str())?;
        let node = find_expression(source_file, range)?;
        let mut checker = self
            .program
            .get_type_checker_for_file(&Context::background(), source_file);
        let ty = checker.get_type_at_location(node);
        Some(f(&mut checker, ty))
    }

    /// Prints the type of the expression at `range` as TypeScript would in a diagnostic.
    pub fn type_to_string(&self, file: &Utf8Path, range: TextRange) -> Option<String> {
        self.with_type_of_expression(file, range, |checker, ty| {
            checker.type_to_string_exported(ty)
        })
    }

    /// Whether the expression at `range` may evaluate to a value of the given kind, as the checker decides
    /// whether a `&&`, `||`, `??` or `?:` operand is reachable.
    pub fn may_be(&self, file: &Utf8Path, range: TextRange, kind: ValueKind) -> Option<bool> {
        let facts = match kind {
            ValueKind::Truthy => TypeFacts::Truthy,
            ValueKind::Falsy => TypeFacts::Falsy,
            ValueKind::Nullish => TypeFacts::EQUndefinedOrNull,
        };
        self.with_type_of_expression(file, range, |checker, ty| checker.has_type_facts(ty, facts))
    }

    /// Whether the expression at `range` is a Promise, as typescript-eslint's `no-floating-promises` decides
    /// with its default options.
    pub fn is_promise_like(&self, file: &Utf8Path, range: TextRange) -> Option<bool> {
        self.with_type_of_expression(file, range, |checker, ty| {
            self.is_promise_like_type(checker, ty)
        })
    }

    /// Whether the expression at `range` is an array or tuple whose elements are Promises.
    pub fn is_promise_array(&self, file: &Utf8Path, range: TextRange) -> Option<bool> {
        self.with_type_of_expression(file, range, |checker, ty| {
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

fn os_fs() -> Arc<dyn FS> {
    Arc::new(bundled::wrap_fs(osvfs::fs()))
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
