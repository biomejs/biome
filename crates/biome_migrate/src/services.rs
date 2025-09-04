use biome_analyze::{FromServices, RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct IsRoot(pub bool);

impl IsRoot {
    pub fn is_root(&self) -> bool {
        self.0
    }
}

impl FromServices for IsRoot {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let is_root: &Arc<bool> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["IsRoot"]))?;
        Ok(Self(*is_root.deref()))
    }
}
