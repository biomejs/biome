use crate::CliDiagnostic;
use crate::runner::collector::Collector;
use crate::runner::execution::Execution;
use crate::runner::handler::Handler;
use crate::runner::process_file::{ChangedFile, Message, MessageStat, ProcessFile};
use biome_diagnostics::{Error, Severity};
use biome_fs::{BiomePath, FileSystem, PathInterner, PathKind, TraversalContext, TraversalScope};
use biome_service::Workspace;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{ChangeFileParams, FeaturesSupported};
use camino::Utf8PathBuf;
use crossbeam::channel::{Sender, unbounded};
use papaya::{HashMap, HashSet, HashSetRef, LocalGuard};
use std::hash::RandomState;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use tracing::instrument;

pub enum CrawlPath {
    String(String),
    Path(Utf8PathBuf),
}

/// Records a processed path and whether the command wrote new content to it.
#[derive(Clone, Debug)]
pub(crate) struct EvaluatedPath {
    /// The logical filesystem path that was processed.
    path: BiomePath,
    /// Whether processing wrote new content to the path.
    was_written: bool,
}

impl EvaluatedPath {
    /// Creates a processed-path result.
    fn new(path: BiomePath, was_written: bool) -> Self {
        Self { path, was_written }
    }

    /// Returns the processed logical path.
    pub(crate) fn path(&self) -> &BiomePath {
        &self.path
    }

    /// Returns whether processing wrote new content to the path.
    pub(crate) fn was_written(&self) -> bool {
        self.was_written
    }
}

pub trait Crawler<Output> {
    type Handler: Handler;
    type ProcessFile: ProcessFile;
    type Collector: Collector;

    fn output(
        collector_result: <Self::Collector as Collector>::Result,
        evaluated_paths: Vec<EvaluatedPath>,
        duration: Duration,
    ) -> Output;

    #[expect(clippy::too_many_arguments)]
    fn crawl(
        execution: &dyn Execution,
        workspace: &dyn Workspace,
        fs: &dyn FileSystem,
        project_key: ProjectKey,
        use_server: bool,
        inputs: Vec<CrawlPath>,
        collector: Self::Collector,
        max_diagnostics: u32,
        diagnostic_level: Severity,
    ) -> Result<Output, CliDiagnostic> {
        let (interner, recv_files) = PathInterner::new();
        let (sender, receiver) = unbounded();

        let (duration, evaluated_paths) = thread::scope(|s| {
            let handler = thread::Builder::new()
                .name(String::from("biome::console"))
                .spawn_scoped(s, || collector.run(receiver, recv_files, execution))
                .expect("failed to spawn console thread");

            let ctx = CrawlerOptions::new(
                fs,
                workspace,
                project_key,
                use_server,
                interner,
                sender,
                execution,
                max_diagnostics,
                diagnostic_level,
            );
            let (elapsed, evaluated_paths) = Self::crawl_inputs(fs, inputs, &ctx);

            if use_server {
                for changed_file in ctx.take_changed_files() {
                    let result = workspace.change_file(ChangeFileParams {
                        project_key,
                        path: changed_file.path,
                        content: changed_file.content,
                        version: changed_file.version,
                        inline_config: None,
                        editor_features: None,
                    })?;
                    for diagnostic in result.diagnostics {
                        ctx.push_message(Message::from(diagnostic));
                    }
                }
            }

            // Close the message sender before waiting for the collector.
            drop(ctx);
            // wait for the main thread to finish
            handler.join().unwrap();

            Ok::<_, CliDiagnostic>((elapsed, evaluated_paths))
        })?;

        execution.on_post_crawl(workspace)?;
        let result = collector.result(duration);
        Ok(Self::output(result, evaluated_paths, duration))
    }

    /// Walks the input paths and processes every selected file, returning the duration of the
    /// process and the processed paths in processing order.
    fn crawl_inputs<'a>(
        fs: &'a dyn FileSystem,
        inputs: Vec<CrawlPath>,
        ctx: &'a CrawlerOptions<Self::Handler, Self::ProcessFile>,
    ) -> (Duration, Vec<EvaluatedPath>) {
        let start = Instant::now();
        fs.traversal(Box::new(move |scope: &dyn TraversalScope| {
            for input in inputs {
                let input = match input {
                    CrawlPath::Path(input) => input,
                    CrawlPath::String(input) => Utf8PathBuf::from(input),
                };
                scope.evaluate_and_handle(ctx, input);
            }
        }));

        let written_paths = ctx.written_paths.pin();
        let mut evaluated_paths: Vec<_> = ctx
            .evaluated_paths()
            .into_iter()
            .cloned()
            .map(|path| {
                let was_written = written_paths.contains(&path);
                EvaluatedPath::new(path, was_written)
            })
            .collect();
        evaluated_paths
            .sort_unstable_by(|left, right| left.path().cmp_processing_priority(right.path()));

        (start.elapsed(), evaluated_paths)
    }
}

pub trait CrawlerContext: TraversalContext {
    fn increment_changed(&self, path: &BiomePath, changed_file: ChangedFile);
    fn increment_unchanged(&self);
    fn increment_matches(&self, num_matches: usize);
    fn increment_skipped(&self);
    /// Send a message to the display thread
    fn push_message(&self, msg: Message);
    fn fs(&self) -> &dyn FileSystem;
    fn workspace(&self) -> &dyn Workspace;
    fn project_key(&self) -> ProjectKey;
    fn execution(&self) -> &dyn Execution;
    fn insert_file_features(&self, path: BiomePath, features: FeaturesSupported);
    fn get_file_features(&self, path: &BiomePath) -> Option<FeaturesSupported>;
}

/// Context object shared between directory traversal tasks
pub(crate) struct CrawlerOptions<'ctx, 'app, H, P> {
    /// Shared instance of [FileSystem]
    pub(crate) fs: &'app dyn FileSystem,
    /// Instance of [Workspace] used by this instance of the CLI
    pub(crate) workspace: &'ctx dyn Workspace,
    /// Key of the project in which we're traversing.
    pub(crate) project_key: ProjectKey,
    /// Logical path interner used by the filesystem traversal.
    interner: PathInterner,
    /// Channel sending messages to the display thread
    pub(crate) messages: Sender<Message>,
    /// Paths handed to the processor by the traversal.
    pub(crate) evaluated_paths: papaya::HashSet<BiomePath>,
    /// Paths whose contents were changed by processing.
    pub(crate) written_paths: papaya::HashSet<BiomePath>,
    /// File features already computed during path evaluation.
    pub(crate) file_features: papaya::HashMap<BiomePath, FeaturesSupported>,
    /// Final contents written by crawler workers.
    pub(crate) changed_files: Mutex<Vec<ChangedFile>>,
    /// Whether final contents must be synchronized with a persistent workspace.
    synchronize_workspace: bool,
    /// Maximum number of diagnostics to pull from the workspace.
    pub(crate) max_diagnostics: u32,
    /// Minimum severity for diagnostics to be included.
    pub(crate) diagnostic_level: Severity,

    execution: &'app dyn Execution,

    handler: H,

    _p: PhantomData<P>,
}

impl<'ctx, 'app, H, P> CrawlerContext for CrawlerOptions<'ctx, 'app, H, P>
where
    H: Handler,
    P: ProcessFile,
{
    fn increment_changed(&self, path: &BiomePath, changed_file: ChangedFile) {
        if self.synchronize_workspace {
            self.changed_files.lock().unwrap().push(changed_file);
        }
        self.written_paths.pin().insert(path.clone());
        self.push_message(Message::Stats(MessageStat::Changed));
    }
    fn increment_unchanged(&self) {
        self.push_message(Message::Stats(MessageStat::Unchanged));
    }

    fn increment_matches(&self, num_matches: usize) {
        self.push_message(Message::Stats(MessageStat::Matches(num_matches)));
    }

    fn increment_skipped(&self) {
        self.push_message(Message::Stats(MessageStat::Skipped));
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

    fn insert_file_features(&self, path: BiomePath, features: FeaturesSupported) {
        self.file_features.pin().insert(path, features);
    }

    fn get_file_features(&self, path: &BiomePath) -> Option<FeaturesSupported> {
        self.file_features.pin().get(path).cloned()
    }
}

impl<'ctx, 'app, I, P> CrawlerOptions<'ctx, 'app, I, P>
where
    I: Handler,
    P: ProcessFile,
{
    #[expect(clippy::too_many_arguments)]
    pub(crate) fn new(
        fs: &'app dyn FileSystem,
        workspace: &'ctx dyn Workspace,
        project_key: ProjectKey,
        synchronize_workspace: bool,
        interner: PathInterner,
        sender: Sender<Message>,
        execution: &'app dyn Execution,
        max_diagnostics: u32,
        diagnostic_level: Severity,
    ) -> Self {
        Self {
            fs,
            workspace,
            project_key,
            interner,
            messages: sender,
            evaluated_paths: HashSet::default(),
            written_paths: HashSet::default(),
            file_features: HashMap::default(),
            changed_files: Mutex::new(Vec::new()),
            synchronize_workspace,
            handler: I::default(),
            execution,
            max_diagnostics,
            diagnostic_level,
            _p: PhantomData::<P>,
        }
    }

    fn take_changed_files(&self) -> Vec<ChangedFile> {
        std::mem::take(&mut *self.changed_files.lock().unwrap())
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

    fn evaluated_paths(&self) -> HashSetRef<'_, BiomePath, RandomState, LocalGuard<'_>> {
        self.evaluated_paths.pin()
    }

    fn push_diagnostic(&self, diagnostic: Error) {
        let is_traversal_error = diagnostic.severity() >= Severity::Error;
        self.push_message(Message::Error {
            diagnostic,
            is_traversal_error,
        });
    }

    #[instrument(level = "debug", skip(self, biome_path))]
    fn can_handle_with_kind(
        &self,
        biome_path: &BiomePath,
        path_kind: PathKind,
        ancestors_checked: bool,
    ) -> bool {
        self.handler
            .can_handle(biome_path, path_kind, ancestors_checked, self)
    }

    fn handle_path(&self, path: BiomePath) {
        self.evaluated_paths.pin().insert(path.clone());
        self.handler.handle_path::<P, Self>(
            &path,
            self.max_diagnostics,
            self.diagnostic_level,
            self,
        )
    }

    fn store_path(&self, path: BiomePath) {
        self.evaluated_paths.pin().insert(path);
    }
}
