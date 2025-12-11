use crate::{LanguageRoot, Manifest};
use biome_deserialize::{
    Deserialized,
    json::{deserialize_from_json_ast, deserialize_from_json_str},
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Error;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;
use std::sync::OnceLock;

/// Cached environment variable patterns, separated into positive and negation patterns.
#[derive(Clone, Debug, Default)]
struct EnvVarsCache {
    /// Positive patterns (exact matches and wildcards like `FOO_*`)
    positive: FxHashSet<String>,
    /// Negation patterns without the `!` prefix (e.g., `SECRET_KEY` from `!SECRET_KEY`)
    negations: FxHashSet<String>,
}

/// Represents the `turbo.json` configuration file used by Turborepo.
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub struct TurboJson {
    /// Path to `turbo.json`. Contains the `turbo.json` filename.
    #[deserializable(skip)]
    pub path: Utf8PathBuf,

    /// Global environment variables that affect all tasks.
    /// These are environment variables that, when changed, will invalidate
    /// the cache for all tasks.
    #[deserializable(rename = "globalEnv")]
    pub global_env: Option<Vec<String>>,

    /// Global pass-through environment variables.
    /// These environment variables are available to all tasks but do not
    /// contribute to the task's cache key. Useful for variables that should
    /// be available at runtime but shouldn't invalidate caches when changed.
    #[deserializable(rename = "globalPassThroughEnv")]
    pub global_pass_through_env: Option<Vec<String>>,

    /// Global dependencies that affect all tasks.
    #[deserializable(rename = "globalDependencies")]
    pub global_dependencies: Option<Vec<String>>,

    /// Task definitions.
    pub tasks: Option<TurboTasks>,

    /// Legacy pipeline field (deprecated but still supported).
    pub pipeline: Option<TurboTasks>,

    /// Cached environment variable patterns (lazily initialized).
    /// Separated into positive patterns and negation patterns for single-pass matching.
    #[deserializable(skip)]
    env_vars_cache: OnceLock<EnvVarsCache>,
}

impl Clone for TurboJson {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            global_env: self.global_env.clone(),
            global_pass_through_env: self.global_pass_through_env.clone(),
            global_dependencies: self.global_dependencies.clone(),
            tasks: self.tasks.clone(),
            pipeline: self.pipeline.clone(),
            // Clone the cached value if present, otherwise start fresh
            env_vars_cache: self
                .env_vars_cache
                .get()
                .cloned()
                .map(OnceLock::from)
                .unwrap_or_default(),
        }
    }
}

/// A map of task names to their configurations.
pub type TurboTasks = rustc_hash::FxHashMap<String, TurboTask>;

/// Configuration for a single task in turbo.json.
#[derive(Clone, Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub struct TurboTask {
    /// Environment variables that this task depends on.
    /// Changes to these variables will invalidate the cache for this task.
    pub env: Option<Vec<String>>,

    /// Environment variables that are passed through to this task without
    /// affecting the cache key. These are allowed but not used for cache invalidation.
    #[deserializable(rename = "passThroughEnv")]
    pub pass_through_env: Option<Vec<String>>,

    /// Task dependencies.
    #[deserializable(rename = "dependsOn")]
    pub depends_on: Option<Vec<String>>,

    /// Output files/directories for caching.
    pub outputs: Option<Vec<String>>,

    /// Input files for cache key calculation.
    pub inputs: Option<Vec<String>>,

    /// Whether this task can be cached.
    pub cache: Option<bool>,

    /// Whether this task is persistent (long-running).
    pub persistent: Option<bool>,
}

impl Manifest for TurboJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(
        root: &LanguageRoot<Self::Language>,
        path: &Utf8Path,
    ) -> Deserialized<Self> {
        let deserialized = deserialize_from_json_ast::<Self>(root, "");
        let (mut turbo_json, errors) = deserialized.consume();
        if let Some(config) = turbo_json.as_mut() {
            config.path = path.to_path_buf();
        }

        Deserialized::new(turbo_json, errors)
    }

    fn read_manifest(fs: &dyn biome_fs::FileSystem, path: &Utf8Path) -> Deserialized<Self> {
        match fs.read_file_from_path(path) {
            Ok(content) => {
                let (manifest, errors) = Self::parse(path, &content);
                Deserialized::new(Some(manifest), errors)
            }
            Err(error) => Deserialized::new(None, vec![Error::from(error)]),
        }
    }
}

impl TurboJson {
    fn parse(path: &Utf8Path, json: &str) -> (Self, Vec<Error>) {
        let (turbo_json, diagnostics) = deserialize_from_json_str(
            json,
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            path.file_name().unwrap_or("turbo.json"),
        )
        .consume();

        let mut turbo_json: Self = turbo_json.unwrap_or_default();
        turbo_json.path = path.to_path_buf();

        (turbo_json, diagnostics)
    }

    /// Returns the cached environment variable patterns from this turbo.json.
    ///
    /// This includes patterns from:
    /// - Global environment variables from `globalEnv`
    /// - Global pass-through environment variables from `globalPassThroughEnv`
    /// - Task-specific environment variables from `tasks.*.env`
    /// - Task-specific pass-through environment variables from `tasks.*.passThroughEnv`
    /// - Legacy pipeline environment variables from `pipeline.*.env`
    /// - Legacy pipeline pass-through environment variables from `pipeline.*.passThroughEnv`
    ///
    /// The result is cached for subsequent calls, with patterns separated into
    /// positive and negation sets for efficient single-pass matching.
    fn env_vars_cache(&self) -> &EnvVarsCache {
        self.env_vars_cache.get_or_init(|| {
            let mut cache = EnvVarsCache::default();

            // Helper to process env vars and separate into positive/negation sets
            let mut process_var = |var: &String| {
                if let Some(negated) = var.strip_prefix('!') {
                    cache.negations.insert(negated.to_string());
                } else {
                    cache.positive.insert(var.clone());
                }
            };

            // Add global env vars
            if let Some(global_env) = &self.global_env {
                for var in global_env {
                    process_var(var);
                }
            }

            // Add global pass-through env vars
            if let Some(global_pass_through_env) = &self.global_pass_through_env {
                for var in global_pass_through_env {
                    process_var(var);
                }
            }

            // Add task-specific env vars and passThroughEnv
            if let Some(tasks) = &self.tasks {
                for task in tasks.values() {
                    if let Some(env) = &task.env {
                        for var in env {
                            process_var(var);
                        }
                    }
                    if let Some(pass_through_env) = &task.pass_through_env {
                        for var in pass_through_env {
                            process_var(var);
                        }
                    }
                }
            }

            // Add legacy pipeline env vars and passThroughEnv
            if let Some(pipeline) = &self.pipeline {
                for task in pipeline.values() {
                    if let Some(env) = &task.env {
                        for var in env {
                            process_var(var);
                        }
                    }
                    if let Some(pass_through_env) = &task.pass_through_env {
                        for var in pass_through_env {
                            process_var(var);
                        }
                    }
                }
            }

            cache
        })
    }

    /// Returns all declared environment variables from this turbo.json.
    ///
    /// This includes:
    /// - Global environment variables from `globalEnv`
    /// - Global pass-through environment variables from `globalPassThroughEnv`
    /// - Task-specific environment variables from `tasks.*.env`
    /// - Task-specific pass-through environment variables from `tasks.*.passThroughEnv`
    /// - Legacy pipeline environment variables from `pipeline.*.env`
    /// - Legacy pipeline pass-through environment variables from `pipeline.*.passThroughEnv`
    ///
    /// Note: This returns only positive patterns, not negation patterns.
    pub fn all_env_vars(&self) -> &FxHashSet<String> {
        &self.env_vars_cache().positive
    }

    /// Checks if the given environment variable is declared in this turbo.json.
    ///
    /// This handles both exact matches and wildcard patterns (e.g., `FOO_*`),
    /// as well as negation patterns (e.g., `!SECRET_KEY`).
    ///
    /// Negation patterns take precedence: if `["*", "!SECRET_KEY"]` is declared,
    /// `SECRET_KEY` will NOT be considered declared because the negation excludes it.
    pub fn is_env_var_declared(&self, env_var: &str) -> bool {
        let cache = self.env_vars_cache();

        // Check negations first (they take precedence)
        // Exact negation match
        if cache.negations.contains(env_var) {
            return false;
        }
        // Wildcard negation (e.g., !SECRET_* excludes SECRET_KEY)
        for negated in &cache.negations {
            if let Some(prefix) = negated.strip_suffix('*')
                && env_var.starts_with(prefix)
            {
                return false;
            }
        }

        // Check for positive matches
        if cache.positive.contains(env_var) {
            return true;
        }
        // Wildcard pattern (e.g., FOO_* matches FOO_BAR)
        for pattern in &cache.positive {
            if let Some(prefix) = pattern.strip_suffix('*')
                && env_var.starts_with(prefix)
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_turbo_json_with_global_env(env_vars: Vec<&str>) -> TurboJson {
        TurboJson {
            path: Utf8PathBuf::from("turbo.json"),
            global_env: Some(env_vars.into_iter().map(String::from).collect()),
            global_pass_through_env: None,
            global_dependencies: None,
            tasks: None,
            pipeline: None,
            env_vars_cache: OnceLock::new(),
        }
    }

    #[test]
    fn test_exact_match() {
        let turbo = create_turbo_json_with_global_env(vec!["API_KEY", "DATABASE_URL"]);
        assert!(turbo.is_env_var_declared("API_KEY"));
        assert!(turbo.is_env_var_declared("DATABASE_URL"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_wildcard_match() {
        let turbo = create_turbo_json_with_global_env(vec!["NEXT_PUBLIC_*"]);
        assert!(turbo.is_env_var_declared("NEXT_PUBLIC_API_URL"));
        assert!(turbo.is_env_var_declared("NEXT_PUBLIC_"));
        assert!(!turbo.is_env_var_declared("NEXT_PUBLICX"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_global_wildcard() {
        let turbo = create_turbo_json_with_global_env(vec!["*"]);
        assert!(turbo.is_env_var_declared("ANY_VAR"));
        assert!(turbo.is_env_var_declared("SECRET_KEY"));
    }

    #[test]
    fn test_negation_excludes_exact_match() {
        let turbo = create_turbo_json_with_global_env(vec!["*", "!SECRET_KEY"]);
        assert!(turbo.is_env_var_declared("ANY_VAR"));
        assert!(!turbo.is_env_var_declared("SECRET_KEY"));
    }

    #[test]
    fn test_negation_excludes_wildcard() {
        let turbo = create_turbo_json_with_global_env(vec!["*", "!SECRET_*"]);
        assert!(turbo.is_env_var_declared("ANY_VAR"));
        assert!(!turbo.is_env_var_declared("SECRET_KEY"));
        assert!(!turbo.is_env_var_declared("SECRET_TOKEN"));
        assert!(turbo.is_env_var_declared("SECRETX")); // Doesn't match !SECRET_*
    }

    #[test]
    fn test_negation_without_wildcard() {
        // Negation alone shouldn't declare anything
        let turbo = create_turbo_json_with_global_env(vec!["!SECRET_KEY"]);
        assert!(!turbo.is_env_var_declared("SECRET_KEY"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_multiple_negations() {
        let turbo = create_turbo_json_with_global_env(vec!["*", "!SECRET_KEY", "!API_TOKEN"]);
        assert!(turbo.is_env_var_declared("ANY_VAR"));
        assert!(!turbo.is_env_var_declared("SECRET_KEY"));
        assert!(!turbo.is_env_var_declared("API_TOKEN"));
    }

    #[test]
    fn test_negation_order_independent() {
        // Negation should work regardless of order in the list
        let turbo1 = create_turbo_json_with_global_env(vec!["*", "!SECRET_KEY"]);
        let turbo2 = create_turbo_json_with_global_env(vec!["!SECRET_KEY", "*"]);
        assert!(!turbo1.is_env_var_declared("SECRET_KEY"));
        assert!(!turbo2.is_env_var_declared("SECRET_KEY"));
        assert!(turbo1.is_env_var_declared("OTHER_VAR"));
        assert!(turbo2.is_env_var_declared("OTHER_VAR"));
    }

    fn create_turbo_json_with_task_pass_through_env(pass_through_env: Vec<&str>) -> TurboJson {
        let mut tasks = TurboTasks::default();
        tasks.insert(
            "build".to_string(),
            TurboTask {
                env: None,
                pass_through_env: Some(pass_through_env.into_iter().map(String::from).collect()),
                depends_on: None,
                outputs: None,
                inputs: None,
                cache: None,
                persistent: None,
            },
        );
        TurboJson {
            path: Utf8PathBuf::from("turbo.json"),
            global_env: None,
            global_pass_through_env: None,
            global_dependencies: None,
            tasks: Some(tasks),
            pipeline: None,
            env_vars_cache: OnceLock::new(),
        }
    }

    fn create_turbo_json_with_global_pass_through_env(env_vars: Vec<&str>) -> TurboJson {
        TurboJson {
            path: Utf8PathBuf::from("turbo.json"),
            global_env: None,
            global_pass_through_env: Some(env_vars.into_iter().map(String::from).collect()),
            global_dependencies: None,
            tasks: None,
            pipeline: None,
            env_vars_cache: OnceLock::new(),
        }
    }

    #[test]
    fn test_global_pass_through_env_exact_match() {
        let turbo =
            create_turbo_json_with_global_pass_through_env(vec!["AWS_SECRET_KEY", "DATABASE_URL"]);
        assert!(turbo.is_env_var_declared("AWS_SECRET_KEY"));
        assert!(turbo.is_env_var_declared("DATABASE_URL"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_global_pass_through_env_wildcard_match() {
        let turbo = create_turbo_json_with_global_pass_through_env(vec!["AWS_*"]);
        assert!(turbo.is_env_var_declared("AWS_SECRET_KEY"));
        assert!(turbo.is_env_var_declared("AWS_ACCESS_KEY"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_global_pass_through_env_combined_with_global_env() {
        // Test that both globalEnv and globalPassThroughEnv are considered
        let turbo = TurboJson {
            path: Utf8PathBuf::from("turbo.json"),
            global_env: Some(vec!["API_KEY".to_string()]),
            global_pass_through_env: Some(vec!["AWS_SECRET".to_string()]),
            global_dependencies: None,
            tasks: None,
            pipeline: None,
            env_vars_cache: OnceLock::new(),
        };
        assert!(turbo.is_env_var_declared("API_KEY"));
        assert!(turbo.is_env_var_declared("AWS_SECRET"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_global_pass_through_env_negation() {
        // Test that negation works with globalPassThroughEnv
        let turbo = TurboJson {
            path: Utf8PathBuf::from("turbo.json"),
            global_env: Some(vec!["*".to_string()]),
            global_pass_through_env: Some(vec!["!SECRET_KEY".to_string()]),
            global_dependencies: None,
            tasks: None,
            pipeline: None,
            env_vars_cache: OnceLock::new(),
        };
        assert!(turbo.is_env_var_declared("ANY_VAR"));
        assert!(!turbo.is_env_var_declared("SECRET_KEY"));
    }

    #[test]
    fn test_pass_through_env_exact_match() {
        let turbo =
            create_turbo_json_with_task_pass_through_env(vec!["AWS_SECRET_KEY", "DATABASE_URL"]);
        assert!(turbo.is_env_var_declared("AWS_SECRET_KEY"));
        assert!(turbo.is_env_var_declared("DATABASE_URL"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_pass_through_env_wildcard_match() {
        let turbo = create_turbo_json_with_task_pass_through_env(vec!["AWS_*"]);
        assert!(turbo.is_env_var_declared("AWS_SECRET_KEY"));
        assert!(turbo.is_env_var_declared("AWS_ACCESS_KEY"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_pass_through_env_combined_with_env() {
        // Test that both env and passThroughEnv are considered
        let mut tasks = TurboTasks::default();
        tasks.insert(
            "build".to_string(),
            TurboTask {
                env: Some(vec!["API_KEY".to_string()]),
                pass_through_env: Some(vec!["AWS_SECRET".to_string()]),
                depends_on: None,
                outputs: None,
                inputs: None,
                cache: None,
                persistent: None,
            },
        );
        let turbo = TurboJson {
            path: Utf8PathBuf::from("turbo.json"),
            global_env: None,
            global_pass_through_env: None,
            global_dependencies: None,
            tasks: Some(tasks),
            pipeline: None,
            env_vars_cache: OnceLock::new(),
        };
        assert!(turbo.is_env_var_declared("API_KEY"));
        assert!(turbo.is_env_var_declared("AWS_SECRET"));
        assert!(!turbo.is_env_var_declared("OTHER_VAR"));
    }
}
