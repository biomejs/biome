use crate::CliDiagnostic;
use crate::runner::collector::Collector;
use crate::runner::execution::Execution;
use crate::runner::handler::Handler;
use crate::runner::process_file::{Message, MessageStat, ProcessFile};
use biome_diagnostics::Error;
use biome_fs::{BiomePath, FileSystem, PathInterner, TraversalContext, TraversalScope};
use biome_service::Workspace;
use biome_service::projects::ProjectKey;
use camino::Utf8PathBuf;
use crossbeam::channel::{Sender, unbounded};
use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use tracing::instrument;

pub trait Crawler<Output> {
    type Handler: Handler;
    type ProcessFile: ProcessFile;
    type Collector: Collector;

    fn output(
        collector_result: <Self::Collector as Collector>::Result,
        evaluated_paths: BTreeSet<BiomePath>,
        duration: Duration,
    ) -> Output;

    fn crawl(
        execution: &dyn Execution,
        workspace: &dyn Workspace,
        fs: &dyn FileSystem,
        project_key: ProjectKey,
        inputs: Vec<String>,
        collector: Self::Collector,
    ) -> Result<Output, CliDiagnostic> {
        let (interner, recv_files) = PathInterner::new();
        let (sender, receiver) = unbounded();

        let (duration, evaluated_paths) = thread::scope(|s| {
            let handler = thread::Builder::new()
                .name(String::from("biome::console"))
                .spawn_scoped(s, || collector.run(receiver, recv_files, execution))
                .expect("failed to spawn console thread");

            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once the traversal finishes
            let (elapsed, evaluated_paths) = Self::crawl_inputs(
                fs,
                inputs,
                // Don't move it. If ctx is declared outside of this function, it doesn't
                // go out of scope, causing a deadlock because the main thread waits for
                // ctx to be dropped
                &CrawlerOptions::new(fs, workspace, project_key, interner, sender, execution),
            );
            // wait for the main thread to finish
            handler.join().unwrap();

            (elapsed, evaluated_paths)
        });

        execution.on_post_crawl(workspace)?;
        let result = collector.result(duration);
        Ok(Self::output(result, evaluated_paths, duration))
    }

    /// Initiate the filesystem traversal tasks with the provided input paths and
    /// run it to completion, returning the duration of the process and the evaluated paths
    fn crawl_inputs(
        fs: &dyn FileSystem,
        inputs: Vec<String>,
        ctx: &CrawlerOptions<Self::Handler, Self::ProcessFile>,
    ) -> (Duration, BTreeSet<BiomePath>) {
        let start = Instant::now();
        fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
            for input in inputs {
                scope.evaluate(ctx, Utf8PathBuf::from(input));
            }
        }));

        let paths = ctx.evaluated_paths();
        fs.traversal(Box::new(|scope: &dyn TraversalScope| {
            for path in paths {
                scope.handle(ctx, path.to_path_buf());
            }
        }));

        (start.elapsed(), ctx.evaluated_paths())
    }
}

pub trait CrawlerContext: TraversalContext {
    fn increment_changed(&self, path: &BiomePath);
    fn increment_unchanged(&self);
    fn increment_matches(&self, num_matches: usize);
    fn increment_skipped(&self);
    /// Send a message to the display thread
    fn push_message(&self, msg: Message);
    fn fs(&self) -> &dyn FileSystem;
    fn workspace(&self) -> &dyn Workspace;
    fn project_key(&self) -> ProjectKey;
    fn execution(&self) -> &dyn Execution;
}

/// Context object shared between directory traversal tasks
pub(crate) struct CrawlerOptions<'ctx, 'app, H, P> {
    /// Shared instance of [FileSystem]
    pub(crate) fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    pub(crate) workspace: &'ctx dyn Workspace,
    /// Key of the project in which we're traversing.
    pub(crate) project_key: ProjectKey,
    /// File paths interner cache used by the filesystem traversal
    interner: PathInterner,
    /// Shared atomic counter storing the number of changed files
    changed: AtomicUsize,
    /// Shared atomic counter storing the number of unchanged files
    unchanged: AtomicUsize,
    /// Shared atomic counter storing the number of unchanged files
    matches: AtomicUsize,
    /// Shared atomic counter storing the number of skipped files
    skipped: AtomicUsize,
    /// Channel sending messages to the display thread
    pub(crate) messages: Sender<Message>,
    /// List of paths that should be processed
    pub(crate) evaluated_paths: RwLock<BTreeSet<BiomePath>>,

    execution: &'app dyn Execution,

    handler: H,

    _p: PhantomData<P>,
}

impl<'ctx, 'app, H, P> CrawlerContext for CrawlerOptions<'ctx, 'app, H, P>
where
    H: Handler,
    P: ProcessFile,
{
    fn increment_changed(&self, path: &BiomePath) {
        self.changed.fetch_add(1, Ordering::Relaxed);
        self.evaluated_paths
            .write()
            .unwrap()
            .replace(path.to_written());
        self.push_message(Message::Stats(MessageStat::Changed));
    }
    fn increment_unchanged(&self) {
        self.push_message(Message::Stats(MessageStat::Unchanged));
        self.unchanged.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_matches(&self, num_matches: usize) {
        self.push_message(Message::Stats(MessageStat::Matches));
        self.matches.fetch_add(num_matches, Ordering::Relaxed);
    }

    fn increment_skipped(&self) {
        self.push_message(Message::Stats(MessageStat::Skipped));
        self.skipped.fetch_add(1, Ordering::Relaxed);
    }

    /// Send a message to the display thread
    fn push_message(&self, msg: Message) {
        self.messages.send(msg).ok();
    }

    fn fs(&self) -> &dyn FileSystem {
        self.fs
    }

    fn workspace(&self) -> &dyn Workspace {
        self.workspace
    }

    fn project_key(&self) -> ProjectKey {
        self.project_key
    }

    fn execution(&self) -> &dyn Execution {
        self.execution
    }
}

impl<'ctx, 'app, I, P> CrawlerOptions<'ctx, 'app, I, P>
where
    I: Handler,
    P: ProcessFile,
{
    pub(crate) fn new(
        fs: &'app dyn FileSystem,
        workspace: &'ctx dyn Workspace,
        project_key: ProjectKey,
        interner: PathInterner,
        sender: Sender<Message>,
        execution: &'app dyn Execution,
    ) -> Self {
        Self {
            fs,
            workspace,
            project_key,
            interner,
            messages: sender,
            evaluated_paths: RwLock::default(),
            handler: I::default(),
            changed: AtomicUsize::new(0),
            unchanged: AtomicUsize::new(0),
            matches: AtomicUsize::new(0),
            skipped: AtomicUsize::new(0),
            execution,
            _p: PhantomData::<P>,
        }
    }
}

impl<I, P> TraversalContext for CrawlerOptions<'_, '_, I, P>
where
    I: Handler,
    P: ProcessFile,
{
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn evaluated_paths(&self) -> BTreeSet<BiomePath> {
        self.evaluated_paths.read().unwrap().clone()
    }

    fn push_diagnostic(&self, error: Error) {
        self.push_message(error.into());
    }

    #[instrument(level = "debug", skip(self, biome_path))]
    fn can_handle(&self, biome_path: &BiomePath) -> bool {
        self.handler.can_handle(biome_path, self)
    }

    fn handle_path(&self, path: BiomePath) {
        self.handler.handle_path::<P, Self>(&path, self)
    }

    fn store_path(&self, path: BiomePath) {
        self.evaluated_paths
            .write()
            .unwrap()
            .insert(BiomePath::new(path.as_path()));
    }
}
