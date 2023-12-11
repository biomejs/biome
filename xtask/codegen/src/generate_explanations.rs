#![allow(unused)]
use std::collections::{BTreeMap, HashMap};

use biome_analyze::{RegistryVisitor, RuleMetadata};
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;

use proc_macro2::{Ident, Literal, Punct, Spacing, Span, TokenStream};
use quote::quote;

use xtask::project_root;
use xtask_codegen::update;

use crate::{Mode, Result};

#[derive(Default)]
struct RulesVisitor {
    rules: BTreeMap<&'static str, RuleMetadata>,
}

impl RulesVisitor {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl RegistryVisitor<JsLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule + 'static,
        R::Query: biome_analyze::Queryable<Language = JsLanguage>,
        <R::Query as biome_analyze::Queryable>::Output: Clone,
    {
        self.rules.insert(R::METADATA.name, R::METADATA);
    }
}

impl RegistryVisitor<JsonLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule + 'static,
        R::Query: biome_analyze::Queryable<Language = JsonLanguage>,
        <R::Query as biome_analyze::Queryable>::Output: Clone,
    {
        self.rules.insert(R::METADATA.name, R::METADATA);
    }
}

pub(crate) fn generate_explanations(mode: Mode) -> Result<()> {
    let explanations_root = project_root().join("crates/biome_service/src/explanations");

    let mut visitor = RulesVisitor::new();

    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);

    let keys: Vec<_> = visitor.rules.keys().collect();

    let rule_match_arms = visitor.rules.values().map(generate_rule_match_arm);

    let nl = Punct::new('\n', Spacing::Alone);

    let rules = quote! {
        use biome_analyze::{RuleMetadata, FixKind};
        #nl #nl
        pub(super) fn get_rule_metadata(s: &str) -> Option<RuleMetadata> {
            match s {
                #( #rule_match_arms ),*,
                _ => None,
            }
        }




    };

    update(
        &explanations_root.join("rules.rs"),
        &xtask::reformat(rules.to_string())?,
        &mode,
    )?;

    Ok(())
}

fn generate_rule_match_arm(metadata: &RuleMetadata) -> TokenStream {
    let name = Literal::string(metadata.name);
    let name_ident = Ident::new(metadata.name, Span::call_site());
    let docs = Literal::string(metadata.docs);
    let version = Literal::string(metadata.version);
    let recommended = Ident::new(&metadata.recommended.to_string(), Span::call_site());

    let fix_kind = match &metadata.fix_kind {
        Some(kind) => {
            let kind = Ident::new(kind.to_string(), Span::call_site());
            quote! {Some(FixKind::#kind)}
        }
        None => quote! {None},
    };

    let deprecated = match &metadata.deprecated {
        Some(text) => {
            let text = Literal::string(text);
            quote! {Some(#text)}
        }
        None => quote! {None},
    };

    let nl = Punct::new('\n', Spacing::Alone);

    quote! {
        #name => Some(RuleMetadata{ #nl
            name: #name,
            version: #version,
            fix_kind: #fix_kind,
            recommended: #recommended,
            deprecated: #deprecated,
            docs: #docs,
        })
    }
}
