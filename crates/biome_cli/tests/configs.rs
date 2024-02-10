pub const CONFIG_FORMAT: &str = r#"{
  "formatter": {
    "enabled": true,
    "lineWidth": 160,
    "indentStyle": "space",
    "indentSize": 6
  }
}
"#;

pub const CONFIG_FORMAT_JSONC: &str = r#"{
  // Formatting options
  "formatter": {
    "lineWidth": 10,
    "indentStyle": "space",
    "indentWidth": 8
  }
}
"#;

pub const CONFIG_INIT_DEFAULT: &str = r#"{
	"organizeImports": {
	 	 "enabled": true
	},
	"linter": {
		"enabled": true,
		"rules": {
			"recommended": true
		}
	}
}
"#;

pub const CONFIG_INIT_DEFAULT_WHEN_INSTALLED: &str = r#"{
	"$schema": "./node_modules/@biomejs/biome/configuration_schema.json",
    "organizeImports": {
	 	 "enabled": true
	},
	"linter": {
		"enabled": true,
		"rules": {
			"recommended": true
		}
	}
}
"#;

pub const CONFIG_DISABLED_FORMATTER: &str = r#"{
  "formatter": {
    "enabled": false
  }
}
"#;

pub const CONFIG_DISABLED_FORMATTER_JSONC: &str = r#"{
 // I am a comment
  "formatter": {
    "enabled": false
  }
}
"#;

pub const CONFIG_ALL_FIELDS: &str = r#"{
  "formatter": {
    "enabled": true,
    "formatWithErrors": true,
    "indentStyle": "tab",
    "indentSize": 2,
    "lineWidth": 80
  },
  "linter": {
    "enabled": true,
    "rules": {
        "complexity": {
            "useSimplifiedLogicExpression": "warn"
        },
        "style": {
            "useTemplate": {
                "level": "error"
            }
        },
        "correctness": {
            "noUnreachable": "off"
        },
        "suspicious": {
            "noCatchAssign": "error",
            "noLabelVar": {
                "level": "warn"
            }
        }
    }
  },
  "javascript": {
    "globals": ["$"],
    "formatter": {
      "quoteStyle": "double",
      "jsxQuoteStyle": "double",
      "quoteProperties": "asNeeded"
    }
  }
}"#;

pub const CONFIG_BAD_LINE_WIDTH: &str = r#"{
  "formatter": {
    "lineWidth": 500
  }
}"#;

pub const CONFIG_LINTER_DISABLED: &str = r#"{
  "linter": {
    "enabled": false
  }
}"#;

pub const CONFIG_LINTER_DISABLED_JSONC: &str = r#"{
  // I am a comment
  "linter": {
    "enabled": false
  }
}"#;

pub const CONFIG_LINTER_WRONG_RULE: &str = r#"{
  "linter": {
    "enabled": true,
    "rules": {
        "correctness": {
            "foo_rule": "off"
        },
        "style": {
            "what_the_hell": "off"
        }
    }
  }
}"#;

pub const CONFIG_INCORRECT_GLOBALS: &str = r#"{
  "linter": {
    "enabled": false
  },
  "javascript": {
    "globals": [false]
  }
}"#;

pub const CONFIG_LINTER_SUPPRESSED_RULE: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noDebugger": "off"
        }
    }
  }
}"#;

pub const CONFIG_LINTER_SUPPRESSED_GROUP: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "recommended": false
        }
    }
  }
}"#;

pub const CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noDebugger": "warn"
        }
    }
  }
}"#;

pub const CONFIG_LINTER_UPGRADE_DIAGNOSTIC: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "style": {
            "noNegationElse": "error"
        }
    }
  }
}"#;

pub const CONFIG_RECOMMENDED_GROUP: &str = r#"{
  "linter": {
    "rules": {
        "recommended": false,
        "correctness": {
            "recommended": true
        }
    }
  }
}"#;

pub const CONFIG_INCORRECT_GLOBALS_V2: &str = r#"{
    "javascript": {
      "formatter": {
        "quoteStyle": "single"
      }
  }
}"#;

pub const CONFIG_ISSUE_3175_1: &str = r#"{
  "formatter": {
    "indentStyle": "space",
    "indentSize": 2,
    "lineWidth": 120
  }
}"#;

pub const CONFIG_ISSUE_3175_2: &str = r#"{
  "javascript": {
    "formatter": {
        "quoteStyle": "single"
    }
  }
}"#;

pub const CONFIG_FORMATTER_IGNORED_FILES: &str = r#"{
  "formatter": {
    "enabled": true,
    "ignore": ["test.js"]
  }
}
"#;

pub const CONFIG_FORMATTER_AND_FILES_IGNORE: &str = r#"{
  "files": {
    "ignore": ["test1.js"]
  },
  "formatter": {
    "enabled": true,
    "ignore": ["test2.js"]
  }
}"#;

pub const CONFIG_FORMATTER_IGNORED_DIRECTORIES: &str = r#"{
  "formatter": {
    "ignore": [
      "test1.js",
      "test2.js",
      "test3/**/*",
      "/test4/**/*",
      "test5/**/*",
      "**/test6/*.js",
      "*.test7.js"
    ]
  }
}
"#;

pub const CONFIG_LINTER_IGNORED_FILES: &str = r#"{
  "linter": {
    "enabled": true,
    "ignore": ["test.js"]
  }
}
"#;

pub const CONFIG_LINTER_AND_FILES_IGNORE: &str = r#"{
  "files": {
    "ignore": ["test1.js"]
  },
  "linter": {
    "enabled": true,
    "ignore": ["test2.js"]
  }
}"#;

pub const CONFIG_FILE_SIZE_LIMIT: &str = r#"{
  "files": {
    "maxSize": 16
  }
}"#;

pub const CONFIG_IGNORE_SYMLINK: &str = r#"{
  "files": {
    "ignore": [
      "symlink_testcase2/**/*.ts"
    ]
  }
}"#;
