//! Migrations for Biome v2

use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_successfully_migrate_knip() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(
        configuration_path.into(),
        r#"{
  "$schema": "https://biomejs.dev/schemas/1.8.3/schema.json",
  "files": {
    "ignore": [
      "**/dist",
      "**/tmp",
      "**/fixtures",
      "packages/knip/package.json",
      "packages/knip/vendor/bash-parser/index.js",
      "packages/docs/.astro"
    ]
  },
  "organizeImports": {
    "enabled": true
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true,
      "correctness": {
        "noUnusedVariables": "error",
        "noUnusedImports": "error"
      },
      "complexity": {
        "useLiteralKeys": "off"
      },
      "nursery": {
        "noRestrictedImports": {
          "level": "error",
          "options": {
            "paths": {
              "node:path": "Please use src/util/path.js instead.",
              "path": "Please use src/util/path.js instead."
            }
          }
        }
      },
      "style": {
        "noParameterAssign": "off",
        "useNodeAssertStrict": "error"
      },
      "suspicious": {
        "noConsoleLog": "error",
        "noExplicitAny": "off"
      }
    }
  },
  "formatter": {
    "enabled": true,
    "lineWidth": 120,
    "indentStyle": "space",
    "formatWithErrors": true
  },
  "javascript": {
    "formatter": {
      "quoteStyle": "single",
      "arrowParentheses": "asNeeded",
      "trailingCommas": "es5"
    }
  },
  "json": {
    "formatter": {
      "lineWidth": 80
    }
  },
  "css": {
    "formatter": {
      "quoteStyle": "single"
    }
  },
  "overrides": [
    {
      "include": ["*.astro"],
      "linter": {
        "rules": {
          "correctness": {
            "noUnusedVariables": "off",
            "noUnusedImports": "off"
          }
        }
      }
    },
    {
      "include": [
        "packages/docs",
        "packages/knip/scripts",
        "packages/knip/src/cli.ts",
        "packages/knip/src/reporters",
        "packages/knip/src/util/cli-arguments.ts",
        "packages/knip/src/util/debug.ts"
      ],
      "linter": {
        "rules": {
          "suspicious": {
            "noConsoleLog": "off"
          }
        }
      }
    },
    {
      "include": ["packages/knip/fixtures"],
      "organizeImports": {
        "enabled": false
      },
      "linter": {
        "rules": {
          "correctness": {
            "noUnusedVariables": "off",
            "noUnusedImports": "off"
          },
          "style": {
            "useImportType": "off"
          }
        }
      }
    },
    {
      "include": ["packages/knip/test/util/get-inputs-from-scripts.test.ts"],
      "formatter": {
        "lineWidth": 200
      }
    }
  ]
}"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["migrate", "--write"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_successfully_migrate_knip",
        fs,
        console,
        result,
    ));
}
