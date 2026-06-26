use proc_macro::TokenStream;
use quote::quote;

mod group_struct;
mod visitors;

use crate::group_struct::generate_group_struct;
use crate::visitors::{AssistActionsVisitor, LintRulesVisitor};

#[proc_macro]
pub fn lint_group_structs(_input: TokenStream) -> TokenStream {
    let input_lint = collect_lint_rules();
    let empty_group: std::collections::BTreeMap<&'static str, biome_analyze::RuleMetadata> =
        std::collections::BTreeMap::new();

    let group_names = [
        "a11y",
        "complexity",
        "correctness",
        "nursery",
        "performance",
        "security",
        "style",
        "suspicious",
    ];
    let mut group_structs = vec![];
    for group_name in group_names {
        let group = input_lint.groups.get(group_name).unwrap_or(&empty_group);
        let group_struct =
            generate_group_struct(group_name, group, biome_analyze::RuleCategory::Lint);
        group_structs.push(group_struct);
    }

    let expanded = quote! {
        #(#group_structs)*
    };
    TokenStream::from(expanded)
}

fn collect_lint_rules() -> LintRulesVisitor {
    #[expect(
        clippy::allow_attributes,
        reason = "`unused_mut` is feature-dependent here; `expect(unused_mut)` is unfulfilled when language features mutate this visitor."
    )]
    #[allow(
        unused_mut,
        reason = "The visitor is mutated only when at least one language feature registers lint rules."
    )]
    let mut lint_visitor = LintRulesVisitor::default();
    #[cfg(feature = "lang_js")]
    biome_js_analyze::visit_registry(&mut lint_visitor);
    #[cfg(feature = "lang_json")]
    biome_json_analyze::visit_registry(&mut lint_visitor);
    #[cfg(feature = "lang_css")]
    biome_css_analyze::visit_registry(&mut lint_visitor);
    #[cfg(feature = "lang_graphql")]
    biome_graphql_analyze::visit_registry(&mut lint_visitor);
    #[cfg(feature = "lang_html")]
    biome_html_analyze::visit_registry(&mut lint_visitor);

    lint_visitor
}

#[proc_macro]
pub fn assist_group_structs(_input: TokenStream) -> TokenStream {
    let input_assist = collect_assist_rules();
    let empty_group: std::collections::BTreeMap<&'static str, biome_analyze::RuleMetadata> =
        std::collections::BTreeMap::new();

    let group_names = ["source"];
    let mut group_structs = vec![];
    for group_name in group_names {
        let group = input_assist.groups.get(group_name).unwrap_or(&empty_group);
        let group_struct =
            generate_group_struct(group_name, group, biome_analyze::RuleCategory::Action);
        group_structs.push(group_struct);
    }

    let expanded = quote! {
        #(#group_structs)*
    };
    TokenStream::from(expanded)
}

fn collect_assist_rules() -> AssistActionsVisitor {
    #[expect(
        clippy::allow_attributes,
        reason = "`unused_mut` is feature-dependent here; `expect(unused_mut)` is unfulfilled when language features mutate this visitor."
    )]
    #[allow(
        unused_mut,
        reason = "The visitor is mutated only when at least one language feature registers assist rules."
    )]
    let mut assist_visitor = AssistActionsVisitor::default();
    #[cfg(feature = "lang_js")]
    biome_js_analyze::visit_registry(&mut assist_visitor);
    #[cfg(feature = "lang_json")]
    biome_json_analyze::visit_registry(&mut assist_visitor);
    #[cfg(feature = "lang_css")]
    biome_css_analyze::visit_registry(&mut assist_visitor);
    #[cfg(feature = "lang_graphql")]
    biome_graphql_analyze::visit_registry(&mut assist_visitor);
    #[cfg(feature = "lang_html")]
    biome_html_analyze::visit_registry(&mut assist_visitor);

    assist_visitor
}

pub(crate) fn to_capitalized(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
