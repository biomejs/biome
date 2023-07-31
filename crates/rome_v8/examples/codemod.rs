use bpaf::{construct, long, positional, OptionParser, Parser};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    thread,
};

use crossbeam::channel::{unbounded, Receiver, Sender};

use rome_console::{markup, ColorMode, Console, ConsoleExt, EnvConsole};
use rome_diagnostics::{Error, PrintDiagnostic};
use rome_fs::{
    self, FileSystem, FileSystemExt, OsFileSystem, PathInterner, RomePath, TraversalContext,
};
use rome_js_parser::{parse, JsParserOptions};
use rome_js_syntax::JsFileSource;
use rome_service::{workspace, App, DynRef, WorkspaceRef};
use rome_v8::{Instance, Script};

pub fn main() {
    let mut console = EnvConsole::default();
    console.set_color(ColorMode::Auto);
    let app = App::new(
        DynRef::Owned(Box::new(OsFileSystem)),
        &mut console,
        WorkspaceRef::Owned(workspace::server()),
    );
    let options = options().run();
    let (script_name, script) = load_script(&*app.fs, &options);

    let (send_diagnostics, recv_diagnostics) = unbounded();
    let (interner, recv_files) = PathInterner::new();

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
            for input in options.paths {
                scope.spawn(ctx, input);
            }
        }));
    });
}

struct CliOptions {
    script_path: PathBuf,
    paths: Vec<PathBuf>,
}

fn options() -> OptionParser<CliOptions> {
    let script_path = long("script")
        .help("Script to load at runtime")
        .argument::<PathBuf>("PATH");
    let paths = positional::<PathBuf>("PATH").many();

    construct!(CliOptions { script_path, paths }).to_options()
}

fn load_script(fs: &dyn FileSystem, options: &CliOptions) -> (String, String) {
    let mut script = fs.read(&options.script_path).unwrap_or_else(|err| {
        panic!(
            "could not open file {}: {err}",
            options.script_path.display()
        )
    });

    let mut buffer = String::new();
    script.read_to_string(&mut buffer).unwrap();

    (options.script_path.display().to_string(), buffer)
}

fn console_thread(
    console: &mut dyn Console,
    recv_diagnostics: Receiver<Error>,
    _recv_files: Receiver<PathBuf>,
) {
    while let Ok(error) = recv_diagnostics.recv() {
        console.error(markup! {
            {PrintDiagnostic::simple(&error)}
        });
    }
}

struct CodemodContext<'app> {
    fs: &'app dyn FileSystem,
    interner: PathInterner,
    diagnostics: Sender<Error>,
    script_name: String,
    script: String,
}

impl TraversalContext for CodemodContext<'_> {
    fn interner(&self) -> &PathInterner {
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

    fn handle_file(&self, path: &Path) {
        let mut file = self
            .fs
            .open(path)
            .unwrap_or_else(|err| panic!("could not open file {}: {err}", path.display()));

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let source_type = JsFileSource::try_from(path).unwrap();
        let parse = parse(&buffer, source_type, JsParserOptions::default());
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
