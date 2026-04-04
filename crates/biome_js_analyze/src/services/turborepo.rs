//! Service for checking environment variable declarations against Turborepo configuration.
//!
//! This module provides [`TurborepoServices`] which aggregates all applicable `turbo.json`
//! configurations (root and package-level) and enables lint rules to verify that environment
//! variables accessed in code are properly declared for Turborepo caching.

use std::sync::Arc;

use crate::services::semantic::SemanticModelBuilderVisitor;
use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_package::TurboJson;
use biome_rowan::AstNode;

/// Holds all turbo.json(c) configurations that apply to a file.
///
/// In a Turborepo monorepo, environment variables can be declared in:
/// 1. The root `turbo.json` at the repository root
/// 2. A package-level `turbo.json` in the package directory
///
#[derive(Debug, Clone)]
pub struct TurborepoServices {
    /// All turbo.json configurations that apply to the current file,
    /// ordered from closest (package-level) to furthest (root).
    pub(crate) turborepo_configs: Vec<Arc<TurboJson>>,

    /// Semantic model for resolving bindings.
    model: SemanticModel,
}

impl TurborepoServices {
    /// Checks if the given environment variable is declared in any turbo.json.
    ///
    /// This method checks all turbo.json configurations that apply to the current file,
    /// including both the root turbo.json and any package-level turbo.json files.
    ///
    /// While Turborepo treats `globalEnv` and `globalPassThroughEnv` as root-only,
    /// task-level `env` and `passThroughEnv` can be declared in package-level turbo.json files.
    /// Therefore, we need to check all applicable configs.
    ///
    /// Returns `false` if no turbo.json configs exist or the env var is not declared in any of them.
    pub fn is_env_var_declared(&self, env_var: &str) -> bool {
        self.turborepo_configs
            .iter()
            .any(|config| config.is_env_var_declared(env_var))
    }

    /// Returns whether any turbo.json file was found for the current file.
    pub fn has_turborepo_config(&self) -> bool {
        !self.turborepo_configs.is_empty()
    }

    /// Returns a reference to the semantic model.
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for TurborepoServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let turborepo_configs: &Vec<Arc<TurboJson>> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["TurboJson"]))?;

        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;

        Ok(Self {
            turborepo_configs: turborepo_configs.clone(),
            model: model.clone(),
        })
    }
}

impl Phase for TurborepoServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type usable by lint rules **that use the turbo.json config** and matches on specific [AstNode] types.
/// Also provides access to the semantic model for resolving bindings.
#[derive(Clone)]
pub struct Turborepo<N>(pub N);

impl<N> Queryable for Turborepo<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = TurborepoServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsFileSource;

    fn create_turbo_json(json: &str) -> TurboJson {
        let (turbo_json, _diagnostics) =
            deserialize_from_json_str::<TurboJson>(json, Default::default(), "turbo.json")
                .consume();
        turbo_json.unwrap_or_default()
    }

    fn create_services(configs: Vec<Arc<TurboJson>>) -> TurborepoServices {
        // Create a minimal semantic model for testing
        let tree = biome_js_parser::parse("", JsFileSource::tsx(), Default::default());
        let model = semantic_model(&tree.tree(), SemanticModelOptions::default());
        TurborepoServices {
            turborepo_configs: configs,
            model,
        }
    }

    #[test]
    fn test_all_configs_are_checked() {
        // Root config declares ROOT_VAR, package config declares PACKAGE_VAR
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR", "SHARED_VAR"] }"#);
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["PACKAGE_VAR"] }"#);

        // Configs are ordered from closest (package) to furthest (root)
        let services = create_services(vec![Arc::new(package_turbo), Arc::new(root_turbo)]);

        // ROOT_VAR is declared in root turbo.json
        assert!(services.is_env_var_declared("ROOT_VAR"));
        // SHARED_VAR is declared in root turbo.json
        assert!(services.is_env_var_declared("SHARED_VAR"));
        // PACKAGE_VAR is in package config, should be found
        assert!(services.is_env_var_declared("PACKAGE_VAR"));
        // UNDECLARED is not in any config
        assert!(!services.is_env_var_declared("UNDECLARED"));
    }

    #[test]
    fn test_task_env_from_all_configs() {
        // Root has both globalEnv and task-specific env
        let root_turbo = create_turbo_json(
            r#"{ "globalEnv": ["ROOT_VAR"], "tasks": { "build": { "env": ["BUILD_OUTPUT"] } } }"#,
        );
        let package_turbo =
            create_turbo_json(r#"{ "tasks": { "build": { "env": ["PACKAGE_BUILD_VAR"] } } }"#);

        let services = create_services(vec![Arc::new(package_turbo), Arc::new(root_turbo)]);

        // ROOT_VAR is in root globalEnv
        assert!(services.is_env_var_declared("ROOT_VAR"));
        // BUILD_OUTPUT is in root task env
        assert!(services.is_env_var_declared("BUILD_OUTPUT"));
        // PACKAGE_BUILD_VAR is in package config task env, should be found
        assert!(services.is_env_var_declared("PACKAGE_BUILD_VAR"));
    }

    #[test]
    fn test_wildcard_pattern_from_all_configs() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["NEXT_PUBLIC_*"] }"#);
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["DATABASE_URL"] }"#);

        let services = create_services(vec![Arc::new(package_turbo), Arc::new(root_turbo)]);

        // Wildcard from root applies
        assert!(services.is_env_var_declared("NEXT_PUBLIC_API_URL"));
        // DATABASE_URL is in package config, should be found
        assert!(services.is_env_var_declared("DATABASE_URL"));
        // Non-matching var is undeclared
        assert!(!services.is_env_var_declared("ACME_SECRET"));
    }

    #[test]
    fn test_negation_and_positive_in_different_configs() {
        // Root allows all vars except SECRET_KEY
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["*", "!SECRET_KEY"] }"#);
        // Package explicitly declares SECRET_KEY
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["SECRET_KEY"] }"#);

        let services = create_services(vec![Arc::new(package_turbo), Arc::new(root_turbo)]);

        // SECRET_KEY is negated in root config but declared in package config
        // Since we check all configs and any positive match wins, this is declared
        assert!(services.is_env_var_declared("SECRET_KEY"));
        // Regular vars are allowed by root
        assert!(services.is_env_var_declared("ACME_TOKEN"));
    }

    #[test]
    fn test_empty_configs_returns_false() {
        let services = create_services(vec![]);

        // No configs means no root config exists, should return false
        assert!(!services.has_turborepo_config());
        assert!(!services.is_env_var_declared("ANY_VAR"));
    }

    #[test]
    fn test_single_config_is_root() {
        // Single config is considered the root
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR"] }"#);

        let services = create_services(vec![Arc::new(root_turbo)]);

        assert!(services.is_env_var_declared("ROOT_VAR"));
        assert!(!services.is_env_var_declared("OTHER_VAR"));
    }

    #[test]
    fn test_multiple_levels_all_checked() {
        // Simulates: root -> packages/shared -> packages/shared/utils
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR"] }"#);
        let shared_turbo = create_turbo_json(r#"{ "globalEnv": ["SHARED_VAR"] }"#);
        let utils_turbo = create_turbo_json(r#"{ "globalEnv": ["UTILS_VAR"] }"#);

        // Closest to furthest (root is last)
        let services = create_services(vec![
            Arc::new(utils_turbo),
            Arc::new(shared_turbo),
            Arc::new(root_turbo),
        ]);

        // All vars from all configs should be found
        assert!(services.is_env_var_declared("ROOT_VAR"));
        assert!(services.is_env_var_declared("SHARED_VAR"));
        assert!(services.is_env_var_declared("UTILS_VAR"));
        assert!(!services.is_env_var_declared("UNDECLARED"));
    }

    /// This test explicitly validates the monorepo use case where:
    /// - Root turbo.json declares global env vars
    /// - Package-level turbo.json declares task-specific env vars
    /// - A file in the package should have access to BOTH
    ///
    /// This is a regression test for the bug where only root config was checked.
    #[test]
    fn test_monorepo_package_env_vars_are_recognized() {
        // Root turbo.json at /project/turbo.json
        // Declares global env vars that apply to all packages
        let root_turbo = create_turbo_json(
            r#"{ "globalEnv": ["CI", "NODE_ENV"], "tasks": { "build": { "env": ["BUILD_ID"] } } }"#,
        );

        // Package turbo.json at /project/packages/web/turbo.json
        // Declares package-specific task env vars
        let package_turbo = create_turbo_json(
            r#"{ "tasks": { "build": { "env": ["NEXT_PUBLIC_API_URL", "DATABASE_URL"] } } }"#,
        );

        // When linting /project/packages/web/src/index.ts, both configs apply
        // Configs are ordered closest-to-furthest, so package config comes first
        let services = create_services(vec![Arc::new(package_turbo), Arc::new(root_turbo)]);

        // Vars from root turbo.json should be recognized
        assert!(
            services.is_env_var_declared("CI"),
            "CI from root globalEnv should be declared"
        );
        assert!(
            services.is_env_var_declared("NODE_ENV"),
            "NODE_ENV from root globalEnv should be declared"
        );
        assert!(
            services.is_env_var_declared("BUILD_ID"),
            "BUILD_ID from root task env should be declared"
        );

        // CRITICAL: Vars from package turbo.json MUST also be recognized
        // This was the bug - package-level env vars were being ignored
        assert!(
            services.is_env_var_declared("NEXT_PUBLIC_API_URL"),
            "NEXT_PUBLIC_API_URL from package task env should be declared"
        );
        assert!(
            services.is_env_var_declared("DATABASE_URL"),
            "DATABASE_URL from package task env should be declared"
        );

        // Undeclared vars should still be rejected
        assert!(
            !services.is_env_var_declared("UNDECLARED_VAR"),
            "UNDECLARED_VAR should not be declared"
        );
    }
}
