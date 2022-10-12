use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    thread,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use pico_args::Arguments;

use rome_console::{markup, Console, ConsoleExt, EnvConsole};
use rome_diagnostics::v2::{Error, FileId, PrintDiagnostic};
use rome_fs::{
    self, AtomicInterner, FileSystem, FileSystemExt, OsFileSystem, PathInterner, RomePath,
    TraversalContext,
};
use rome_js_parser::parse;
use rome_js_syntax::SourceType;
use rome_service::{workspace, App, DynRef, WorkspaceRef};
use rome_v8::{Instance, Script};

pub fn main() {
    let mut args = Arguments::from_env();

    let no_colors = args.contains("--no-colors");
    let mut app = App::new(
        DynRef::Owned(Box::new(OsFileSystem)),
        DynRef::Owned(Box::new(EnvConsole::new(no_colors))),
        WorkspaceRef::Owned(workspace::server()),
    );

    let (script_name, script) = load_script(&*app.fs, &mut args);

    // Check that at least one input file / directory was specified in the command line
    let mut inputs = vec![];

    for input in args.finish() {
        if let Some(maybe_arg) = input.to_str() {
            let without_dashes = maybe_arg.trim_start_matches('-');
            if without_dashes.is_empty() {
                // `-` or `--`
                continue;
            }
            // `--<some character>` or `-<some character>`
            if without_dashes != input {
                panic!("unexpected argument {input:?}");
            }
        }
        inputs.push(input);
    }

    if inputs.is_empty() {
        panic!("missing argument <INPUT>");
    }

    let (send_diagnostics, recv_diagnostics) = unbounded();
    let (interner, recv_files) = AtomicInterner::new();

    let console = &mut *app.console;
    let fs = &*app.fs;

    let ctx = CodemodContext {
        fs,
        interner,
        diagnostics: send_diagnostics,
        script_name,
        script,
    };

    thread::scope(move |scope| {
        scope.spawn(move || {
            console_thread(console, recv_diagnostics, recv_files);
        });

        let ctx = &ctx;
        fs.traversal(Box::new(move |scope| {
            for input in inputs {
                scope.spawn(ctx, PathBuf::from(input));
            }
        }));
    });
}

fn load_script(fs: &dyn FileSystem, args: &mut Arguments) -> (String, String) {
    let script_name: String = args
        .value_from_str("--script")
        .expect("missing argument --script");

    let script_path = Path::new(&script_name);
    let mut script = fs
        .read(script_path)
        .unwrap_or_else(|err| panic!("could not open file {}: {err}", script_path.display()));

    let mut buffer = String::new();
    script.read_to_string(&mut buffer).unwrap();

    (script_name, buffer)
}

fn console_thread(
    console: &mut dyn Console,
    recv_diagnostics: Receiver<Error>,
    _recv_files: Receiver<(FileId, PathBuf)>,
) {
    while let Ok(error) = recv_diagnostics.recv() {
        console.error(markup! {
            {PrintDiagnostic(&error)}
        });
    }
}

struct CodemodContext<'app> {
    fs: &'app dyn FileSystem,
    interner: AtomicInterner,
    diagnostics: Sender<Error>,
    script_name: String,
    script: String,
}

impl TraversalContext for CodemodContext<'_> {
    fn interner(&self) -> &dyn PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, error: Error) {
        self.diagnostics.send(error).ok();
    }

    fn can_handle(&self, path: &RomePath) -> bool {
        let ext = match path.extension().and_then(OsStr::to_str) {
            Some(ext) => ext,
            None => return false,
        };

        matches!(
            ext.to_lowercase().as_str(),
            "js" | "mjs" | "cjs" | "jsx" | "ts" | "mts" | "cts" | "tsx"
        )
    }

    fn handle_file(&self, path: &Path, file_id: FileId) {
        let mut file = self
            .fs
            .open(path)
            .unwrap_or_else(|err| panic!("could not open file {}: {err}", path.display()));

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let source_type = SourceType::try_from(path).unwrap();
        let parse = parse(&buffer, file_id, source_type);
        let root = parse.tree();

        thread_local! {
            static INSTANCE: RefCell<Instance> = RefCell::new(Instance::new().expect("failed to create V8 instance"));
            static SCRIPT_CACHE: RefCell<HashMap<String, Script>> = RefCell::new(HashMap::new());
        }

        let result = INSTANCE.with(|instance| {
            let mut instance = instance.borrow_mut();

            SCRIPT_CACHE.with(|script_cache| {
                let mut script_cache = script_cache.borrow_mut();

                let script = script_cache
                    .entry(self.script_name.clone())
                    .or_insert_with(|| instance.load(&self.script_name, &self.script).unwrap());

                instance.run(script, root).unwrap()
            })
        });

        let output = result.to_string();
        file.set_content(output.as_bytes()).unwrap();
    }
}
