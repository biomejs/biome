use biome_analyze::RuleCategory;
use proc_macro::TokenStream;
use quote::quote;

mod group_struct;
mod visitors;

use crate::group_struct::generate_group_struct;
use crate::visitors::{AssistActionsVisitor, LintRulesVisitor};

#[proc_macro]
pub fn lint_group_structs(_input: TokenStream) -> TokenStream {
    let input_lint = collect_lint_rules();

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
        let Some(group) = input_lint.groups.get(group_name) else {
            return TokenStream::from(quote! {
                compile_error!(concat!("no such lint rule group found: ", group_name));
            });
        };
        let group_struct = generate_group_struct(group_name, group, RuleCategory::Lint);
        group_structs.push(group_struct);
    }

    let expanded = quote! {
        #(#group_structs)*
    };
    TokenStream::from(expanded)
}

fn collect_lint_rules() -> LintRulesVisitor {
    let mut lint_visitor = LintRulesVisitor::default();
    biome_js_analyze::visit_registry(&mut lint_visitor);
    biome_json_analyze::visit_registry(&mut lint_visitor);
    biome_css_analyze::visit_registry(&mut lint_visitor);
    biome_graphql_analyze::visit_registry(&mut lint_visitor);
    biome_html_analyze::visit_registry(&mut lint_visitor);

    lint_visitor
}

#[proc_macro]
pub fn assist_group_structs(_input: TokenStream) -> TokenStream {
    let input_assist = collect_assist_rules();

    let group_names = ["source"];
    let mut group_structs = vec![];
    for group_name in group_names {
        let Some(group) = input_assist.groups.get(group_name) else {
            return TokenStream::from(quote! {
                compile_error!(concat!("no such lint rule group found: ", group_name));
            });
        };
        let group_struct = generate_group_struct(group_name, group, RuleCategory::Action);
        group_structs.push(group_struct);
    }

    let expanded = quote! {
        #(#group_structs)*
    };
    TokenStream::from(expanded)
}

fn collect_assist_rules() -> AssistActionsVisitor {
    let mut assist_visitor = AssistActionsVisitor::default();
    biome_js_analyze::visit_registry(&mut assist_visitor);
    biome_json_analyze::visit_registry(&mut assist_visitor);
    biome_css_analyze::visit_registry(&mut assist_visitor);
    biome_graphql_analyze::visit_registry(&mut assist_visitor);
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
