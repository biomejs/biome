use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot, assert_file_contents};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const UNFORMATTED: &str = "#   Heading\n\n\n##   Section";
const FORMATTED: &str = "# Heading\n\n## Section\n";
const UNFORMATTED_PROSE: &str = "This is a long paragraph with enough words to exceed the configured line width and require wrapping.";
const PROSE_WRAP_NEVER: &str = "This is a long paragraph with enough words to exceed the configured line width and require wrapping.\n";
const PROSE_WRAP_ALWAYS: &str = "This is a long paragraph with enough\nwords to exceed the configured line\nwidth and require wrapping.\n";

#[test]
fn format_markdown_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_and_write_markdown_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_and_write_markdown_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_markdown_files_with_nursery_rules() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "markdown": {
        "linter": {
            "enabled": true
        }
    },
    "linter": {
        "enabled": false,
        "rules": {
            "nursery": {
                "useTopLevelHeading": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), b"## Second level heading\n");

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_files_with_nursery_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_markdown_linter_override_disables_matching_files() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "linter": {
            "enabled": true
        }
    },
    "linter": {
        "rules": {
            "nursery": {
                "useTopLevelHeading": "error"
            }
        }
    },
    "overrides": [
        {
            "includes": ["special/**"],
            "markdown": {
                "linter": {
                    "enabled": false
                }
            }
        }
    ]
}"#
        .as_bytes(),
    );
    fs.insert(
        Utf8Path::new("file.md").into(),
        b"## Second level heading\n",
    );
    fs.insert(
        Utf8Path::new("special/file.md").into(),
        b"## Second level heading\n",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "file.md", "special/file.md"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_linter_override_disables_matching_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_markdown_linter_override_enables_with_global_disabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "linter": {
        "enabled": false,
        "rules": {
            "nursery": {
                "useTopLevelHeading": "error"
            }
        }
    },
    "overrides": [
        {
            "includes": ["special/**"],
            "linter": {
                "enabled": false
            },
            "markdown": {
                "linter": {
                    "enabled": true
                }
            }
        }
    ]
}"#
        .as_bytes(),
    );
    fs.insert(
        Utf8Path::new("file.md").into(),
        b"## Second level heading\n",
    );
    fs.insert(
        Utf8Path::new("special/file.md").into(),
        b"## Second level heading\n",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "file.md", "special/file.md"].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_linter_override_enables_with_global_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_files_with_prose_wrap_override() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let config_path = Utf8Path::new("biome.json");
    fs.insert(
        config_path.into(),
        r#"{
    "markdown": {
        "formatter": {
            "proseWrap": "never"
        }
    },
    "overrides": [
        {
            "includes": ["special/**"],
            "markdown": {
                "formatter": {
                    "lineWidth": 40,
                    "proseWrap": "always"
                }
            }
        }
    ]
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), UNFORMATTED_PROSE.as_bytes());

    let overridden_file_path = Utf8Path::new("special/file.md");
    fs.insert(overridden_file_path.into(), UNFORMATTED_PROSE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                file_path.as_str(),
                overridden_file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, PROSE_WRAP_NEVER);
    assert_file_contents(&fs, overridden_file_path, PROSE_WRAP_ALWAYS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_files_with_prose_wrap_override",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_files_with_prose_wrap_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), UNFORMATTED_PROSE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--md-formatter-line-width",
                "40",
                "--md-formatter-prose-wrap",
                "always",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, PROSE_WRAP_ALWAYS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_files_with_prose_wrap_cli_option",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_keeps_embeds_with_syntax_errors() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "parser": {
            "frontmatter": true
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"---
items: [
---

#   Embeds

```js
function () {}
```

```json
{
```

<div>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_keeps_embeds_with_syntax_errors",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_markdown_skips_embedded_code_blocks() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"# Embeds

```js
debugger;
```

```css
a {
  color: red;
  color: blue;
}
```

```html
<img src="code-block.png">
```

<img src="html-block.png">
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_markdown_skips_embedded_code_blocks",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_with_embeds() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "parser": {
            "frontmatter": true
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"---
title: Biome
---

#   Embeds

```js
const value = 1;
```

<div>content</div>

- item

    ```js
    const =
    ```

Paragraph with <span>inline HTML</span>.
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_with_embeds",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_reformats_embeds() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "parser": {
            "frontmatter": true
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"---
title:    Biome
items: [a,   b]
---

# Embeds

```js
const   value   =   {a:1,b:2}
```

```css
a{color:red}
```

```rust
fn   main() {}
```

<div><span>content</span>
</div>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_reformats_embeds",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_embeds_keep_markdown_layout() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "html": {
        "formatter": {
            "enabled": true
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"<div><p>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb</p></div>

<span>
label
</span>

   <div><span>indented</span>
   </div>

- item

  <div><span>listed</span>
  </div>

- item

  ```js
  const   listed = 1
  ```

> ```js
> const   quoted = 2
> ```

  ```js
  const   indented = `a
  b`;
  ```

````md
#   Nested

```js
const   nested = 3
```
````

```md
#   Tilde fence

~~~js
const   nested = 4
~~~
```

```js
const   unclosed = 4
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_embeds_keep_markdown_layout",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_keeps_embeds_when_format_embeds_disabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "parser": {
            "frontmatter": true
        },
        "formatter": {
            "formatEmbeds": false
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"---
title:    Biome
---

#   Embeds

```js
const   value   =   {a:1,b:2}
```

```js
function () {}
```
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_keeps_embeds_when_format_embeds_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_ignores_embed_errors_when_only_analyze_embeds_enabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "formatter": {
            "formatEmbeds": false
        },
        "analyzeEmbeds": true
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"#   Embeds

```js
function () {}
```
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_ignores_embed_errors_when_only_analyze_embeds_enabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_embeds_with_override() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "overrides": [
        {
            "includes": ["docs/**"],
            "markdown": {
                "formatter": {
                    "formatEmbeds": false
                }
            }
        }
    ]
}"#
        .as_bytes(),
    );

    let content = r#"```js
const   value   =   {a:1,b:2}
```
"#;
    let formatted_path = Utf8Path::new("file.md");
    let kept_path = Utf8Path::new("docs/file.md");
    fs.insert(formatted_path.into(), content.as_bytes());
    fs.insert(kept_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                formatted_path.as_str(),
                kept_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, kept_path, content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_embeds_with_override",
        fs,
        console,
        result,
    ));
}

#[test]
fn format_markdown_embeds_with_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let content = r#"```js
const   value   =   {a:1,b:2}
```
"#;
    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--md-formatter-format-embeds=false",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_markdown_embeds_with_cli_option",
        fs,
        console,
        result,
    ));
}

const EMBEDS_TO_LINT: &str = r#"---
title: Biome
title: Duplicate
---

# Embeds

```js
debugger;
```

```js
function () {}
```

```css
a {
  color: red;
  color: blue;
}
```

```html
<img src="code-block.png">
```

<img src="html-block.png">

<!-- biome-ignore-start lint/suspicious/noDebugger: example -->

```js
debugger;
```

<!-- biome-ignore-end lint/suspicious/noDebugger: example -->

```js
// biome-ignore lint/suspicious/noDebugger: example
debugger;
```
"#;

#[test]
fn lint_markdown_embeds_when_analyze_embeds_enabled() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "parser": {
            "frontmatter": true
        },
        "analyzeEmbeds": true
    },
    "linter": {
        "rules": {
            "nursery": {
                "noDuplicateMapKeys": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), EMBEDS_TO_LINT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_markdown_embeds_when_analyze_embeds_enabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_markdown_skips_embeds_by_default() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "parser": {
            "frontmatter": true
        }
    },
    "linter": {
        "rules": {
            "nursery": {
                "noDuplicateMapKeys": "error"
            }
        }
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), EMBEDS_TO_LINT.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_markdown_skips_embeds_by_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_markdown_embeds_with_override() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "overrides": [
        {
            "includes": ["docs/**"],
            "markdown": {
                "analyzeEmbeds": true
            }
        }
    ]
}"#
        .as_bytes(),
    );

    let content = r#"```js
debugger;
```
"#;
    let skipped_path = Utf8Path::new("file.md");
    let linted_path = Utf8Path::new("docs/file.md");
    fs.insert(skipped_path.into(), content.as_bytes());
    fs.insert(linted_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", skipped_path.as_str(), linted_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_markdown_embeds_with_override",
        fs,
        console,
        result,
    ));
}

#[test]
fn lint_markdown_embeds_with_cli_option() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"```js
debugger;
```
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--md-analyze-embeds=true", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_markdown_embeds_with_cli_option",
        fs,
        console,
        result,
    ));
}

const EMBEDS_TO_FIX: &str = r#"#   Embeds

```js
debugger;
console.log(   1)
```
"#;

#[test]
fn check_markdown_fixes_and_formats_embeds() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "analyzeEmbeds": true
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), EMBEDS_TO_FIX.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_fixes_and_formats_embeds",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_markdown_fixes_embeds_without_formatting_them() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "formatter": {
            "formatEmbeds": false
        },
        "analyzeEmbeds": true
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(file_path.into(), EMBEDS_TO_FIX.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_fixes_embeds_without_formatting_them",
        fs,
        console,
        result,
    ));
}

#[test]
fn check_markdown_runs_assist_on_embeds() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        r#"{
    "markdown": {
        "analyzeEmbeds": true
    }
}"#
        .as_bytes(),
    );

    let file_path = Utf8Path::new("file.md");
    fs.insert(
        file_path.into(),
        r#"# Embeds

```js
import { b, a } from "./module.js";

export { a, b };
```
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["check", "--write", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "check_markdown_runs_assist_on_embeds",
        fs,
        console,
        result,
    ));
}
