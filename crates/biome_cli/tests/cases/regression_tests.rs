use bpaf::Args;
use camino::Utf8Path;

use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};

use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{run_cli, run_cli_with_dyn_fs, run_cli_with_server_workspace};

/// Regression test for https://github.com/biomejs/biome/issues/9180
///
/// This issue was caused by noRedundantUseStrict's fix replacing the directive with a directive that was syntactically blank.
/// When fixes are applied, they also get formatted by our formatter. The formatter would try to format the
/// directive, but it expects the directive to have quotes, and since it was syntactically blank, it would panic
/// when trying to trim the quotes off.
///
/// Our unit tests didn't pick it up because linter unit tests don't include the formatting step.
///
/// This issue was fixed by changing the rule's fix to remove the node and transfer the trivia to the next token.
#[test]
fn issue_9180() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("test.js");
    fs.insert(
        js_file.into(),
        "// foo\n'use strict';\r\nconsole.log('test');\n".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_9180",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_9180_2() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let js_file = Utf8Path::new("test.js");
    fs.insert(js_file.into(), "// foo\n'use strict';\n".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", js_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_9180_2",
        fs,
        console,
        result,
    ));
}

/// Regression test for https://github.com/biomejs/biome/issues/9300
///
/// This issue affects Tanstack Form users who use `<form.Field>` as their default API.
/// In Biome 2.4.5, lowercase component member expressions like `<form.Field>` were
/// incorrectly formatted as `<form .Field>` (with an extra space before the dot),
/// which breaks the code.
///
/// The official Tanstack Form docs https://tanstack.com/form/latest/docs/framework/svelte/quick-start
///
/// This test ensures that lowercase component member expressions in Svelte and Astro
/// files are formatted correctly without adding extra spaces.
#[test]
fn issue_9300() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let svelte_file = Utf8Path::new("form.svelte");
    fs.insert(svelte_file.into(), "<form.Field></form.Field>".as_bytes());

    let astro_file = Utf8Path::new("form.astro");
    fs.insert(astro_file.into(), "<form.Field></form.Field>".as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--write",
                svelte_file.as_str(),
                astro_file.as_str(),
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_9300",
        fs,
        console,
        result,
    ));
}

/// Regression test for https://github.com/biomejs/biome/issues/10885
#[test]
fn issue_10885() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    let mut temp_fs = TemporaryFs::new("issue_10885");

    for (path, content) in [
        (
            "biome.json",
            r#"{
    "linter": {
        "domains": {
            "types": "recommended"
        },
        "rules": {
            "nursery": {
                "noFloatingPromises": "error"
            }
        }
    }
}
"#,
        ),
        (
            "router.ts",
            r#"import { protectedProcedure } from "./procedures";

export const routerProcedure = protectedProcedure.mutation;
"#,
        ),
        (
            "procedures.ts",
            r#"declare const trpc: unknown;

export const protectedProcedure = trpc.baseProcedure.use;
"#,
        ),
    ] {
        temp_fs.create_file(path, content);
    }

    let result = run_cli_with_dyn_fs(
        Box::new(temp_fs.create_os()),
        &mut console,
        Args::from(["check", temp_fs.cli_path(), "--formatter-enabled=false"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_10885",
        fs,
        console,
        result,
    ));
}

/// Regression test for https://github.com/biomejs/biome/issues/9196
#[test]
fn issue_9196() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let jsx_file = Utf8Path::new("test.jsx");
    fs.insert(
        jsx_file.into(),
        "<div>\n\ttext // first\n\t{/* ok */}\n\ttail /* second */ more\n</div>;\n".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", jsx_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_9196",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_6427() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_file = Utf8Path::new("biome.json");
    fs.insert(
        config_file.into(),
        br#"{
    "plugins": ["noDirectReactComponentCall.grit"],
    "formatter": {
        "enabled": false
    }
}"#,
    );

    let plugin_file = Utf8Path::new("noDirectReactComponentCall.grit");
    fs.insert(
        plugin_file.into(),
        br#"language js

call_expression(function=$f) where {
    $f <: r"[A-Z].*",
    register_diagnostic(
        span = $f,
        message = "Don't call React Components directly"
    )
}
"#,
    );

    let js_file = Utf8Path::new("test.js");
    fs.insert(js_file.into(), br#"Component();"#);

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["check", js_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_6427",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_6571() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        br#"{
    "plugins": ["detectButtonLinkImport.grit"],
    "linter": {
        "rules": {
            "recommended": false
        }
    }
}"#,
    );
    fs.insert(
        "detectButtonLinkImport.grit".into(),
        br#"language js

`import { $imports } from "geist/components"` as $import where {
    $imports <: contains `ButtonLink`,
    register_diagnostic(
        span = $import,
        message = "ButtonLink import matched",
        severity = "error"
    )
}
"#,
    );

    let js_file = Utf8Path::new("test.js");
    fs.insert(
        js_file.into(),
        br#"import { ButtonLink } from "geist/components";
import { Button, ButtonLink, Card } from "geist/components";
import { Button } from "geist/components";
import { ButtonLink } from "other/components";
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", js_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_6571",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_7363() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        br#"{
    "plugins": ["interface.grit"],
    "formatter": {
        "enabled": false
    },
    "linter": {
        "rules": {
            "recommended": false
        }
    }
}"#,
    );
    fs.insert(
        "interface.grit".into(),
        br#"`interface $name { $body }` where {
    register_diagnostic(span=$name, severity="warn", message="found interface")
}
"#,
    );

    let ts_file = Utf8Path::new("interface.ts");
    fs.insert(
        ts_file.into(),
        br#"interface Zero {}

interface Single {
    f1: string;
}

interface Multi {
    id: number;
    name: string;
    email: string;
}
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["check", ts_file.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_7363",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_6782() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        br#"{
    "plugins": ["fragment.grit"],
    "linter": {
        "rules": {
            "recommended": false
        }
    }
}"#,
    );
    fs.insert(
        "fragment.grit".into(),
        br#"`<$component $attrs>$children</$component>` as $fragment where {
    $component <: `React.Fragment`,
    register_diagnostic(span=$fragment, message=`Prefer importing <Fragment /> instead of relying on auto import ($component)`)
}
"#,
    );

    let js_file = Utf8Path::new("main.tsx");
    fs.insert(
        js_file.into(),
        br#"const Component = () => {
  return <React.Fragment>
    <div>Hello</div>
    <span>World</span>
  </React.Fragment>
}

export default Component

const keyed = <React.Fragment key="item"><span>Child</span></React.Fragment>;
const empty = <React.Fragment></React.Fragment>;
const other = <Other.Fragment><span>Child</span></Other.Fragment>;
const suspense = <React.Suspense><span>Child</span></React.Suspense>;
const imported = <Fragment><span>Child</span></Fragment>;
"#,
    );

    let (fs, result) = run_cli_with_server_workspace(
        fs,
        &mut console,
        Args::from(["lint", js_file.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "issue_6782",
        fs,
        console,
        result,
    ));
}

#[test]
fn issue_7771() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        "biome.json".into(),
        br#"{
    "assist": {
        "actions": {
            "source": {
                "organizeImports": {
                    "level": "on",
                    "options": {
                        "groups": [
                            "@app", "@app/**",
                            "@pages", "@pages/**",
                            "@widgets", "@widgets/**",
                            "@features", "@features/**",
                            "@entities", "@entities/**",
                            "@shared", "@shared/**",
                            ":PACKAGE:", ":ALIAS:", ":PATH:"
                        ]
                    }
                }
            }
        }
    },
    "plugins": [
        "./tools/biome/plugins/fsd-depth.grit",
        "./tools/biome/plugins/fsd-deps.grit"
    ]
}"#,
    );
    fs.insert(
        "tools/biome/plugins/fsd-depth.grit".into(),
        br#"engine biome(1.0)
language js(typescript,jsx)

sequential {
    `import $what from $src` where {
        $src <: r"^\"@(app|pages|widgets|features|entities|shared)\/[^\/]+\/.+\"$",
        register_diagnostic(
            span = $src,
            message = "Only 2 level paths are allowed (e.g. @layer/name).",
            severity = "error"
        )
    },

    `import $src` where {
        $src <: r"^\"@(app|pages|widgets|features|entities|shared)\/[^\/]+\/.+\"$",
        register_diagnostic(
            span = $src,
            message = "Only 2 level paths are allowed (e.g. @layer/name).",
            severity = "error"
        )
    }
}"#,
    );
    fs.insert(
        "tools/biome/plugins/fsd-deps.grit".into(),
        br#"engine biome(1.0)
language js(typescript,jsx)

sequential {
    `import $w from $src` where {
        $filename <: r".*/pages/.*",
        $src <: r"^\"@app(?:/.*)?\"$",
        register_diagnostic(
            span = $src,
            message = "pages cannot import from higher layer @app.",
            severity = "error"
        )
    },

    `import $w from $src` where {
        $filename <: r".*/widgets/.*",
        $src <: r"^\"@(pages|app)(?:/.*)?\"$",
        register_diagnostic(
            span = $src,
            message = "widgets cannot import from @pages or @app.",
            severity = "error"
        )
    },

    `import $w from $src` where {
        $filename <: r".*/features/.*",
        $src <: r"^\"@(widgets|pages|app)(?:/.*)?\"$",
        register_diagnostic(
            span = $src,
            message = "features cannot import from @widgets, @pages, or @app.",
            severity = "error"
        )
    },

    `import $w from $src` where {
        $filename <: r".*/entities/.*",
        $src <: r"^\"@(features|widgets|pages|app)(?:/.*)?\"$",
        register_diagnostic(
            span = $src,
            message = "entities cannot import from higher layers.",
            severity = "error"
        )
    },

    `import $w from $src` where {
        $filename <: r".*/shared/.*",
        $src <: r"^\"@(entities|features|widgets|pages|app)(?:/.*)?\"$",
        register_diagnostic(
            span = $src,
            message = "shared cannot import from higher layers.",
            severity = "error"
        )
    },

    `import $src` where {
        $filename <: r".*/shared/.*",
        $src <: r"^\"@(entities|features|widgets|pages|app)(?:/.*)?\"$",
        register_diagnostic(
            span = $src,
            message = "shared cannot import from higher layers (side-effect import).",
            severity = "error"
        )
    }
}"#,
    );
    fs.insert("src/pages/a.ts".into(), br#"import "@app/foo";"#);

    let _ = run_cli_with_server_workspace(fs, &mut console, Args::from(["check", "."].as_slice()));

    assert!(
        !console
            .out_buffer
            .iter()
            .flat_map(|message| &message.content.0)
            .any(|node| node.content.contains("processing panicked")),
        "Grit plugin processing panicked: {:?}",
        console.out_buffer
    );
}
