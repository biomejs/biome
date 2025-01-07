use crate::{RuleKey, TextRange};
use biome_diagnostics::{Diagnostic, LineIndexBuf, Resource, Result, SourceCode};
use rustc_hash::FxHashMap;
use std::any::{Any, TypeId};

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "internalError/io", tags(INTERNAL))]
pub struct MissingServicesDiagnostic {
    #[message]
    message: String,
    #[description]
    description: String,
    #[location(resource)]
    path: Resource<&'static str>,
    #[location(span)]
    span: Option<TextRange>,
    #[location(source_code)]
    source_code: Option<SourceCode<String, LineIndexBuf>>,
}

impl MissingServicesDiagnostic {
    pub fn new(rule_name: &str, missing_services: &'static [&'static str]) -> Self {
        let description = missing_services.join(", ");
        Self {
            message: format!("Errors emitted while attempting run the rule: {rule_name}"),
            description: format!("Missing services: {description}"),
            source_code: None,
            path: Resource::Memory,
            span: None,
        }
    }
}

pub trait FromServices: Sized {
    #[expect(clippy::result_large_err)]
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic>;
}

#[derive(Debug, Default)]
pub struct ServiceBag {
    services: FxHashMap<TypeId, Box<dyn Any>>,
}

impl ServiceBag {
    pub fn insert_service<T: 'static>(&mut self, service: T) {
        let id = TypeId::of::<T>();
        self.services.insert(id, Box::new(service));
    }

    pub fn get_service<T: 'static>(&self) -> Option<&T> {
        let id = TypeId::of::<T>();
        let svc = self.services.get(&id)?;
        svc.downcast_ref()
    }
}

impl FromServices for () {
    fn from_services(_: &RuleKey, _: &ServiceBag) -> Result<Self, MissingServicesDiagnostic> {
        Ok(())
    }
}
