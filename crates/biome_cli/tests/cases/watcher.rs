use crate::run_cli_with_watcher_factory;
use crate::snap_test::markup_to_string;
use biome_cli::{MockWatcher, WatcherEvent};
use biome_console::{BufferConsole, markup};
use biome_fs::TemporaryFs;
use bpaf::Args;
use std::sync::Mutex;

#[test]
fn lint_watch_reruns_on_event() {
    let mut fs = TemporaryFs::new("lint_watch_reruns_on_event");
    fs.create_file(
        "biome.json",
        r#"{"linter":{"rules":{"recommended":false,"suspicious":{"noVar":"error"}}}}"#,
    );
    fs.create_file("good.js", "const x = 1;\n");
    let bad_path = fs.create_file("bad.js", "var y = 2;\n");

    let mut console = BufferConsole::default();

    let seed = Mutex::new(Some(vec![WatcherEvent::Changed(vec![bad_path.clone()])]));
    let watcher_factory = Box::new(move || {
        let events = seed.lock().unwrap().take().unwrap_or_default();
        Box::new(MockWatcher::with_events(events)) as Box<dyn biome_cli::Watcher>
    });

    let result = run_cli_with_watcher_factory(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--watch", fs.cli_path()].as_slice()),
        watcher_factory,
    );

    let out = console
        .out_buffer
        .iter()
        .map(|m| markup_to_string(markup! { {m.content} }))
        .collect::<Vec<_>>()
        .join("\n");

    assert!(result.is_ok(), "run_cli returned {result:?}\n---\n{out}");

    assert!(
        out.contains("Watching for changes"),
        "expected watch banner in output, got:\n{out}"
    );

    assert_eq!(
        out.matches("noVar").count(),
        2,
        "expected lint to run twice on bad.js, got:\n{out}"
    );
}
