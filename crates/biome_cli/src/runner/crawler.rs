use crate::CliDiagnostic;
use crate::runner::collector::Collector;
use crate::runner::execution::Execution;
use crate::runner::inspector::Inspector;
use crate::runner::process_file::Message;
use biome_diagnostics::Error;
use biome_fs::{BiomePath, FileSystem, PathInterner, TraversalContext, TraversalScope};
use biome_service::Workspace;
use biome_service::projects::ProjectKey;
use camino::Utf8PathBuf;
use crossbeam::channel::{Sender, unbounded};
use std::collections::BTreeSet;
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use tracing::instrument;

pub trait Crawler {
    type Output;
    type CollectorOutput;
    type Inspector: Inspector;

    fn output<C>(
        collector_result: C::Result,
        evaluated_paths: BTreeSet<BiomePath>,
        duration: Duration,
    ) -> Self::Output
    where
        C: Collector;

    fn crawl<C>(
        execution: Box<dyn Execution>,
        workspace: &dyn Workspace,
        fs: &dyn FileSystem,
        project_key: ProjectKey,
        inputs: Vec<String>,
        configuration_files: Vec<BiomePath>,
        collector: C,
    ) -> Result<Self::Output, CliDiagnostic>
    where
        C: Collector,
    {
        let (interner, recv_files) = PathInterner::new();
        let (sender, receiver) = unbounded();

        // TODO implement this
        // let max_diagnostics = execution.get_max_diagnostics();
        //
        // let working_directory = fs.working_directory();
        // let collector = crate::execute::traverse::DiagnosticsCollector::new(execution, working_directory.as_deref())
        //     .with_verbose(cli_options.verbose)
        //     .with_diagnostic_level(cli_options.diagnostic_level)
        //     .with_max_diagnostics(max_diagnostics);

        let ctx = CrawlerOptions::new(
            fs,
            workspace,
            project_key,
            interner,
            sender,
            execution.as_ref(),
        );

        let (duration, evaluated_paths) = thread::scope(|s| {
            let handler = thread::Builder::new()
                .name(String::from("biome::console"))
                .spawn_scoped(s, || {
                    collector.run(receiver, recv_files, execution.as_ref())
                })
                .expect("failed to spawn console thread");

            // The traversal context is scoped to ensure all the channels it
            // contains are properly closed once the traversal finishes
            let (elapsed, evaluated_paths) = Self::crawl_inputs(fs, inputs, &ctx);
            // wait for the main thread to finish
            handler.join().unwrap();

            (elapsed, evaluated_paths)
        });

        // TODO implement this
        // Make sure patterns are always cleaned up at the end of traversal.
        // if let TraversalMode::Search { pattern, .. } = execution.traversal_mode() {
        //     let _ = session.app.workspace.drop_pattern(DropPatternParams {
        //         pattern: pattern.clone(),
        //     });
        // }

        execution.on_post_crawl(workspace)?;
        Ok(Self::output::<C>(
            collector.result(duration.clone(), &ctx),
            evaluated_paths,
            duration,
        ))
    }

    /// Initiate the filesystem traversal tasks with the provided input paths and
    /// run it to completion, returning the duration of the process and the evaluated paths
    fn crawl_inputs(
        fs: &dyn FileSystem,
        inputs: Vec<String>,
        ctx: &CrawlerOptions<Self::Inspector>,
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

    fn process_file() -> Result<(), CliDiagnostic> {
        // ProcessFile::default().process()
        Ok(())
    }
}

pub trait CrawlerContext: TraversalContext {
    fn increment_changed(&self, path: &BiomePath);
    fn increment_unchanged(&self);
    fn increment_matches(&self, num_matches: usize);
    /// Send a message to the display thread
    fn push_message(&self, msg: Message);
    fn fs(&self) -> &dyn FileSystem;
    fn workspace(&self) -> &dyn Workspace;
    fn project_key(&self) -> ProjectKey;
    fn execution(&self) -> &dyn Execution;

    fn changed(&self) -> usize;
    fn unchanged(&self) -> usize;
    fn matches(&self) -> usize;
    fn skipped(&self) -> usize;
}

/// Context object shared between directory traversal tasks
pub(crate) struct CrawlerOptions<'ctx, 'app, I> {
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

    inspector: I,
}

impl<'ctx, 'app, I> CrawlerContext for CrawlerOptions<'ctx, 'app, I>
where
    I: Inspector,
{
    fn increment_changed(&self, path: &BiomePath) {
        self.changed.fetch_add(1, Ordering::Relaxed);
        self.evaluated_paths
            .write()
            .unwrap()
            .replace(path.to_written());
    }
    fn increment_unchanged(&self) {
        self.unchanged.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_matches(&self, num_matches: usize) {
        self.matches.fetch_add(num_matches, Ordering::Relaxed);
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

    fn changed(&self) -> usize {
        self.changed.load(Ordering::Relaxed)
    }

    fn unchanged(&self) -> usize {
        self.unchanged.load(Ordering::Relaxed)
    }

    fn matches(&self) -> usize {
        self.matches.load(Ordering::Relaxed)
    }

    fn skipped(&self) -> usize {
        self.skipped.load(Ordering::Relaxed)
    }
}

impl<'ctx, 'app, I> CrawlerOptions<'ctx, 'app, I>
where
    I: Inspector,
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
            inspector: I::default(),
            changed: AtomicUsize::new(0),
            unchanged: AtomicUsize::new(0),
            matches: AtomicUsize::new(0),
            skipped: AtomicUsize::new(0),
            execution,
        }
    }
}

impl<I> TraversalContext for CrawlerOptions<'_, '_, I>
where
    I: Inspector,
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
        self.inspector.can_handle(biome_path, self)
    }

    fn handle_path(&self, path: BiomePath) {
        self.inspector.handle_path(&path, self)
    }

    fn store_path(&self, path: BiomePath) {
        self.evaluated_paths
            .write()
            .unwrap()
            .insert(BiomePath::new(path.as_path()));
    }
}
