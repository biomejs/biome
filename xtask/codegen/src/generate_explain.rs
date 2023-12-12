use std::collections::BTreeMap;

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

pub(crate) fn generate_explain(mode: Mode) -> Result<()> {
    let explain_root = project_root().join("crates/biome_service/src/explain");

    let mut visitor = RulesVisitor::new();

    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);

    let (rule_docs_constants, rule_match_arms): (Vec<_>, Vec<_>) = visitor
        .rules
        .values()
        .map(generate_rule_match_arm_and_docs_constant)
        .unzip();

    let nl = Punct::new('\n', Spacing::Alone);

    let rules = quote! {
        use biome_analyze::{RuleMetadata, FixKind};

        pub(super) fn get_rule_metadata(s: &str) -> Option<RuleMetadata> {
            match s {
                #( #rule_match_arms ),*,
                _ => None,
            }
        }

        #( #rule_docs_constants #nl )*

    };

    update(
        &explain_root.join("rules.rs"),
        &xtask::reformat(rules.to_string())?,
        &mode,
    )?;

    Ok(())
}

fn generate_rule_match_arm_and_docs_constant(
    metadata: &RuleMetadata,
) -> (TokenStream, TokenStream) {
    let name = Literal::string(metadata.name);

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

    let docs = Literal::string(&format_docs(metadata.docs));
    let docs_ident = Ident::new(&to_upper_snake_case(metadata.name), Span::call_site());

    (
        quote! {
            const #docs_ident: &str = #docs;
        },
        quote! {
            #name => Some(RuleMetadata{
                name: #name,
                version: #version,
                fix_kind: #fix_kind,
                recommended: #recommended,
                deprecated: #deprecated,
                docs: #docs_ident,
            })
        },
    )
}

fn format_docs(docs: &str) -> String {
    docs.lines()
        .map(|line| line.trim_start())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn to_upper_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_ascii_uppercase() {
            out.push('_');
        }

        out.push(c.to_ascii_uppercase());
    }

    out
}

#[cfg(test)]
mod test {
    use super::to_upper_snake_case;

    #[test]
    fn to_upper_snake_case_works() {
        assert_eq!(to_upper_snake_case("noAccessKey"), "NO_ACCESS_KEY");
    }
}
