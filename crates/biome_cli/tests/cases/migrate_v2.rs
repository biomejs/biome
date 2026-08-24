//! Migrations for Biome v2

use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{run_cli, run_cli_with_dyn_fs};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_successfully_migrate_knip() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(
        configuration_path.into(),
        r#"{
  "$schema": "https://biomejs.dev/schemas/1.8.3/schema.json",
  "files": {
    "ignore": [
      "**/dist",
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

#[test]
fn should_successfully_migrate_ariakit() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(
        configuration_path.into(),
        r#"{
  "$schema": "./node_modules/@biomejs/biome/configuration_schema.json",
  "vcs": {
    "enabled": true,
    "clientKind": "git",
    "useIgnoreFile": false
  },
  "files": {
    "ignoreUnknown": true,
    "ignore": ["website/.next/**", "website/.pages/**", "**/*.css"]
  },
  "organizeImports": {
    "enabled": true
  },
  "formatter": {
    "enabled": true,
    "indentStyle": "space"
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true,
      "security": {
        "noDangerouslySetInnerHtml": "off"
      },
      "a11y": {
        "noSvgWithoutTitle": "off",
        "useButtonType": "off",
        "useAnchorContent": "off",
        "useValidAnchor": "off",
        "useKeyWithClickEvents": "off",
        "noAutofocus": "off",
        "noLabelWithoutControl": "off",
        "useSemanticElements": "off",
        "useFocusableInteractive": "off"
      },
      "suspicious": {
        "noExplicitAny": "off",
        "noShadowRestrictedNames": "off",
        "noConfusingVoidType": "off",
        "noArrayIndexKey": "off",
        "noAssignInExpressions": "off"
      },
      "correctness": {
        "useExhaustiveDependencies": "off",
        "useJsxKeyInIterable": "off"
      },
      "style": {
        "noParameterAssign": "off",
        "noUnusedTemplateLiteral": "off",
        "noNonNullAssertion": "off",
        "noUselessElse": "off"
      }
    }
  }
}
"#
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
        "should_successfully_migrate_ariakit",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_successfully_migrate_sentry() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(
        configuration_path.into(),
        r#"{
  "$schema": "./node_modules/@biomejs/biome/configuration_schema.json",
  "vcs": {
    "enabled": true,
    "clientKind": "git",
    "useIgnoreFile": false,
    "defaultBranch": "master"
  },
  "organizeImports": {
    "enabled": false
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": false,
      "a11y": {
        "noBlankTarget": "error"
      },
      "correctness": {
        "noGlobalObjectCalls": "error",
        "noUnreachable": "error",
        "useIsNan": "error",
        "noUnusedPrivateClassMembers": "error",
        "noInvalidUseBeforeDeclaration": "error",
        "noNodejsModules": "error"
      },
      "complexity": {
        "useFlatMap": "error",
        "useOptionalChain": "error",
        "noEmptyTypeParameters": "error",
        "noUselessLoneBlockStatements": "error",
        "noUselessEmptyExport": "error",
        "noUselessConstructor": "error",
        "noUselessTypeConstraint": "error",
        "noExcessiveNestedTestSuites": "error"
      },
      "nursery": {
        "noRestrictedImports": {
          "level": "warn",
          "options": {
            "paths": {}
          }
        }
      },
      "performance": {
        "noBarrelFile": "error"
      },
      "security": {
        "noDangerouslySetInnerHtmlWithChildren": "error"
      },
      "suspicious": {
        "noDebugger": "error",
        "noDoubleEquals": "error",
        "noDuplicateJsxProps": "error",
        "noDuplicateObjectKeys": "error",
        "noDuplicateParameters": "error",
        "noDuplicateCase": "error",
        "noFallthroughSwitchClause": "error",
        "noRedeclare": "error",
        "noSparseArray": "error",
        "noUnsafeDeclarationMerging": "error",
        "noUnsafeNegation": "error",
        "useIsArray": "error",
        "noApproximativeNumericConstant": "error",
        "noMisrefactoredShorthandAssign": "error",
        "useAwait": "error",
        "useNamespaceKeyword": "error",
        "noFocusedTests": "error",
        "noDuplicateTestHooks": "error"
      },
      "style": {
        "noCommaOperator": "error",
        "noShoutyConstants": "error",
        "noParameterProperties": "error",
        "noVar": "error",
        "useConst": "error",
        "useShorthandFunctionType": "error",
        "useExportType": "error",
        "useImportType": "error",
        "useNodejsImportProtocol": "error",
        "useLiteralEnumMembers": "error",
        "useEnumInitializers": "error",
        "useAsConstAssertion": "error",
        "useBlockStatements": "error"
      }
    }
  },
  "files": {
    "ignoreUnknown": true,
    "ignore": [
      "**/*/trace.json",
      "static/app/data/world.json",
      "**/*.sourcemap.js",
      "**/*.min.js",
      "fixtures",
      ".devenv",
      "package.json"
    ]
  },
  "css": {
    "formatter": {
      "enabled": false
    },
    "linter": {
      "enabled": false
    }
  },
  "formatter": {
    "enabled": true,
    "formatWithErrors": true,
    "indentStyle": "space",
    "indentWidth": 2,
    "lineEnding": "lf",
    "ignore": ["tests/**/*.json"]
  },
  "javascript": {
    "formatter": {
      "enabled": false,
      "lineWidth": 90,
      "quoteStyle": "single",
      "jsxQuoteStyle": "double",
      "quoteProperties": "asNeeded",
      "trailingCommas": "es5",
      "semicolons": "always",
      "arrowParentheses": "asNeeded",
      "bracketSpacing": false,
      "bracketSameLine": false
    }
  },
  "json": {
    "formatter": {
      "enabled": true
    },
    "parser": {
      "allowComments": true,
      "allowTrailingCommas": true
    }
  },
  "overrides": [
    {
      "include": [
        "api-docs/*.ts",
        "build-utils/*.ts",
        "config/*.ts",
        "scripts",
        "tests/js/sentry-test/loadFixtures.ts",
        "tests/js/jest-pegjs-transform.js",
        "tests/js/setup.ts",
        "tests/js/test-balancer/index.js",
        "*.config.ts"
      ],
      "linter": {
        "rules": {
          "correctness": {
            "noNodejsModules": "off"
          }
        }
      }
    },
    {
      "include": ["src/sentry/templates/sentry/error-page-embed.js"],
      "linter": {
        "rules": {
          "style": {
            "noVar": "off"
          }
        }
      }
    },
    {
      "include": [
        "static/app/utils/replays/types.tsx",
        "static/app/utils/queryClient.tsx",
        "static/app/views/performance/traceDetails/styles.tsx",
        "static/app/icons/index.tsx",
        "static/app/components/tabs/index.tsx",
        "static/app/components/sparklines/line.tsx",
        "static/app/types/index.tsx",
        "tests/js/sentry-test/reactTestingLibrary.tsx",
        "tests/js/sentry-test/index.tsx"
      ],
      "linter": {
        "rules": {
          "performance": {
            "noBarrelFile": "off"
          }
        }
      }
    }
  ]
}

"#
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
        "should_successfully_migrate_sentry",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_migrate_issue_5465() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(
        configuration_path.into(),
        r#"{
	"linter": {
		"enabled": true,
		"rules": {
			"suspicious": {
				"noDebugger": "error",
				"noShadowRestrictedNames": "off",
				"noExplicitAny": "off",
				"useValidTypeof": "error",
				"noAsyncPromiseExecutor": "off",
				"noEmptyInterface": "error",
				"noAssignInExpressions": "error",
				"noConsole": {
					"level": "error",
					"options": {
						"allow": ["info", "error", "warn", "time", "timeEnd"]
					}
				},
				"noPrototypeBuiltins": "error"
			},
			"style": {
				"noVar": "error",
				"useNamingConvention": {
					"level": "error",
					"options": {
						"strictCase": false
					}
				},
				"noDefaultExport": "error"
			},
			"correctness": {
				"noConstantCondition": "error",
				"noInvalidConstructorSuper": "error",
				"noUnusedVariables": "error",
				"noUnreachable": "error",
				"noUnsafeFinally": "error",
				"noSwitchDeclarations": "off",
				"noSelfAssign": "off"
			}
		}
	}
}
"#
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
        "should_migrate_issue_5465",
        fs,
        console,
        result,
    ));
}
#[test]
fn should_migrate_aws_config() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration_path = Utf8Path::new("biome.json");
    fs.insert(
        configuration_path.into(),
        r#"{
  "$schema": "https://biomejs.dev/schemas/1.8.0/schema.json",
  "files": {
    "maxSize": 5242880,
    "ignore": ["**/__fixtures__/**", "package.json"]
  },
  "formatter": {
    "enabled": true,
    "indentStyle": "space",
    "lineWidth": 100
  },
  "linter": {
    "rules": {
      "recommended": true,
      "complexity": {
        "noForEach": "off"
      },
      "correctness": {
        "noNewSymbol": "error",
        "noUndeclaredVariables": "error",
        "noUnusedVariables": "error"
      },
      "style": {
        "noNamespace": "error",
        "useConsistentArrayType": {
          "level": "error",
          "options": { "syntax": "shorthand" }
        }
      },
      "suspicious": {
        "noEmptyBlockStatements": "error"
      }
    }
  },
  "javascript": { "formatter": { "trailingCommas": "es5" } },
  "vcs": {
    "enabled": true,
    "clientKind": "git",
    "useIgnoreFile": false
  }
}

"#
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
        "should_migrate_aws_config",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_migrate_nested_config() {
    let mut fs = TemporaryFs::new("should_migrate_nested_config");
    let mut console = BufferConsole::default();
    fs.create_file(
        "biome.json",
        r#"{ "linter": { "rules": { "recommended": true } } }"#,
    );
    fs.create_file(
        "foo/biome.json",
        r#"{ "linter": { "rules": { "recommended": true } } }"#,
    );
    fs.create_file(
        "bar/biome.json",
        r#"{ "linter": { "rules": { "recommended": true } } }"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["migrate"].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_migrate_nested_config",
        fs.create_mem(),
        console,
        result,
    ));
}
