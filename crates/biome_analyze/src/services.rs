use crate::{RuleKey, RuleMetadata};
use biome_console::markup;
use biome_diagnostics::{Diagnostic, MessageAndDescription, Result};
use rustc_hash::FxHashMap;
use std::any::{Any, TypeId};

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "internalError/io", tags(INTERNAL))]
pub struct ServicesDiagnostic {
    #[message]
    #[description]
    message: MessageAndDescription,
}

impl ServicesDiagnostic {
    pub fn new(rule_name: &str, missing_services: &'static [&'static str]) -> Self {
        let services = missing_services.join(", ");
        Self {
            message: MessageAndDescription::from(
                markup! {
                    "Missing services ["{services}"] for the rule " {rule_name}
                }
                .to_owned(),
            ),
        }
    }
}

pub trait FromServices: Sized {
    fn from_services(
        rule_key: &RuleKey,
        rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic>;
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
    fn from_services(
        _: &RuleKey,
        _: &RuleMetadata,
        _: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        Ok(())
    }
}
