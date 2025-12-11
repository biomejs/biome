use std::sync::Arc;

use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_package::TurboJson;
use biome_rowan::AstNode;

use crate::services::semantic::SemanticModelBuilderVisitor;

/// Holds all turbo.json configurations that apply to a file.
///
/// In a Turborepo monorepo, environment variables can be declared in:
/// 1. The root `turbo.json` at the repository root
/// 2. A package-level `turbo.json` in the package directory
///
/// Both are checked when validating environment variable declarations.
#[derive(Debug, Clone)]
pub struct TurboServices {
    /// All turbo.json configurations that apply to the current file,
    /// ordered from closest (package-level) to furthest (root).
    pub(crate) turbo_configs: Vec<Arc<TurboJson>>,

    /// Semantic model for resolving bindings.
    model: SemanticModel,
}

impl TurboServices {
    /// Checks if the given environment variable is declared in any turbo.json.
    ///
    /// This checks all turbo.json files in the hierarchy (package-level and root).
    /// The env var is considered declared if it appears in ANY of the configs.
    pub fn is_env_var_declared(&self, env_var: &str) -> bool {
        self.turbo_configs
            .iter()
            .any(|config| config.is_env_var_declared(env_var))
    }

    /// Returns whether any turbo.json file was found for the current file.
    pub fn has_turbo_config(&self) -> bool {
        !self.turbo_configs.is_empty()
    }

    /// Returns a reference to the semantic model.
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for TurboServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let turbo_configs: &Vec<Arc<TurboJson>> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["TurboJson"]))?;

        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;

        Ok(Self {
            turbo_configs: turbo_configs.clone(),
            model: model.clone(),
        })
    }
}

impl Phase for TurboServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type usable by lint rules **that use the turbo.json config** and matches on specific [AstNode] types.
/// Also provides access to the semantic model for resolving bindings.
#[derive(Clone)]
pub struct Turbo<N>(pub N);

impl<N> Queryable for Turbo<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = TurboServices;

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

    fn create_turbo_json(json: &str) -> TurboJson {
        let (turbo_json, _diagnostics) =
            deserialize_from_json_str::<TurboJson>(json, Default::default(), "turbo.json")
                .consume();
        turbo_json.unwrap_or_default()
    }

    /// Tests monorepo scenario: env var declared in root turbo.json only
    #[test]
    fn test_monorepo_env_var_in_root_only() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR", "SHARED_VAR"] }"#);
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["PACKAGE_VAR"] }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // ROOT_VAR is declared in root turbo.json
        assert!(configs.iter().any(|c| c.is_env_var_declared("ROOT_VAR")));
        // PACKAGE_VAR is declared in package turbo.json
        assert!(configs.iter().any(|c| c.is_env_var_declared("PACKAGE_VAR")));
        // SHARED_VAR is declared in root turbo.json
        assert!(configs.iter().any(|c| c.is_env_var_declared("SHARED_VAR")));
        // UNDECLARED is not in any config
        assert!(!configs.iter().any(|c| c.is_env_var_declared("UNDECLARED")));
    }

    /// Tests monorepo scenario: env var declared in package turbo.json task
    #[test]
    fn test_monorepo_env_var_in_package_task() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR"] }"#);
        let package_turbo =
            create_turbo_json(r#"{ "tasks": { "build": { "env": ["BUILD_SECRET"] } } }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // BUILD_SECRET is declared in package turbo.json's task
        assert!(configs
            .iter()
            .any(|c| c.is_env_var_declared("BUILD_SECRET")));
        // ROOT_VAR is in root only
        assert!(configs.iter().any(|c| c.is_env_var_declared("ROOT_VAR")));
    }

    /// Tests monorepo scenario: wildcard in root, specific vars in package
    #[test]
    fn test_monorepo_wildcard_inheritance() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["NEXT_PUBLIC_*"] }"#);
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["DATABASE_URL"] }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // Wildcard from root applies
        assert!(configs
            .iter()
            .any(|c| c.is_env_var_declared("NEXT_PUBLIC_API_URL")));
        // Specific var from package applies
        assert!(configs
            .iter()
            .any(|c| c.is_env_var_declared("DATABASE_URL")));
        // Non-matching var is undeclared
        assert!(!configs.iter().any(|c| c.is_env_var_declared("SECRET_KEY")));
    }

    /// Tests monorepo scenario: negation in package doesn't affect root declarations
    #[test]
    fn test_monorepo_negation_scope() {
        // Root allows all vars
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["*"] }"#);
        // Package excludes SECRET_KEY
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["*", "!SECRET_KEY"] }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // SECRET_KEY is excluded in package but allowed in root
        // Since we use `any()`, if ANY config allows it, it's considered declared
        // This matches the current implementation behavior
        assert!(configs.iter().any(|c| c.is_env_var_declared("SECRET_KEY")));

        // Regular vars are allowed by both
        assert!(configs.iter().any(|c| c.is_env_var_declared("API_KEY")));
    }

    /// Tests monorepo scenario: empty configs
    #[test]
    fn test_monorepo_empty_configs() {
        let configs: Vec<Arc<TurboJson>> = vec![];

        // No configs means no turbo.json found
        assert!(configs.is_empty());
        assert!(!configs.iter().any(|c| c.is_env_var_declared("ANY_VAR")));
    }

    /// Tests monorepo scenario: deeply nested package structure
    #[test]
    fn test_monorepo_multiple_levels() {
        // Simulates: root -> packages/shared -> packages/shared/utils
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR"] }"#);
        let shared_turbo = create_turbo_json(r#"{ "globalEnv": ["SHARED_VAR"] }"#);
        let utils_turbo = create_turbo_json(r#"{ "globalEnv": ["UTILS_VAR"] }"#);

        // Closest to furthest
        let configs = vec![
            Arc::new(utils_turbo),
            Arc::new(shared_turbo),
            Arc::new(root_turbo),
        ];

        // All vars from all levels should be accessible
        assert!(configs.iter().any(|c| c.is_env_var_declared("ROOT_VAR")));
        assert!(configs.iter().any(|c| c.is_env_var_declared("SHARED_VAR")));
        assert!(configs.iter().any(|c| c.is_env_var_declared("UTILS_VAR")));
        assert!(!configs.iter().any(|c| c.is_env_var_declared("UNDECLARED")));
    }
}
