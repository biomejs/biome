use std::{path::PathBuf, str::FromStr};

use case::CaseExt;
use convert_case::{Case, Casing};

pub fn generate_new_lintrule(path: &str, rule_name: &str) {
    let rule_folder = PathBuf::from_str(path).unwrap();
    match rule_folder.file_stem().and_then(|x| x.to_str()) {
        Some("nursery") => {}
        _ => {
            panic!("all new rules must be at a nursery folder");
        }
    }

    let rule_name_upper_camel = rule_name.to_camel();
    let rule_name_snake = rule_name.to_snake();
    let rule_name_lower_camel = rule_name_snake.to_camel_lowercase();

    // Generate rule code
    let code = format!(
        r#"use crate::semantic_services::Semantic;
use biome_analyze::{{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic,
}};
use biome_console::markup;
use biome_js_semantic::{{Reference, ReferencesExtensions}};
use biome_js_syntax::JsIdentifierBinding;

declare_rule! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Semantic<JsIdentifierBinding>;
    type State = Reference;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
        let binding = ctx.query();
        let model = ctx.model();
        binding.all_references(model).collect()
    }}

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {{
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
                markup! {{
                    "Variable is read here."
                }},
            )
            .note(markup! {{
                "This note will give you more information."
            }}),
        )
    }}
}}
"#
    );
    let file_name = format!("{path}/{rule_name_snake}.rs");
    std::fs::write(file_name, code).unwrap();

    let categories_path = "crates/biome_diagnostics_categories/src/categories.rs";
    let mut categories = std::fs::read_to_string(categories_path).unwrap();

    if !categories.contains(&rule_name_lower_camel) {
        let kebab_case_rule = rule_name_lower_camel.to_case(Case::Kebab);
        // We sort rules to reduce conflicts between contributions made in parallel.
        let rule_line = format!(
            r#"    "lint/nursery/{rule_name_lower_camel}": "https://biomejs.dev/linter/rules/{kebab_case_rule}","#
        );
        let lint_start = "define_categories! {\n";
        let lint_end = "\n    ;\n";
        debug_assert!(categories.contains(lint_start));
        debug_assert!(categories.contains(lint_end));
        let lint_start_index = categories.find(lint_start).unwrap() + lint_start.len();
        let lint_end_index = categories.find(lint_end).unwrap();
        let lint_rule_text = &categories[lint_start_index..lint_end_index];
        let mut lint_rules: Vec<_> = lint_rule_text.lines().chain(Some(&rule_line[..])).collect();
        lint_rules.sort_unstable();
        let new_lint_rule_text = lint_rules.join("\n");
        categories.replace_range(lint_start_index..lint_end_index, &new_lint_rule_text);
        std::fs::write(categories_path, categories).unwrap();
    }

    // Generate test code
    let tests_path = format!("crates/biome_js_analyze/tests/specs/nursery/{rule_name_lower_camel}");
    let _ = std::fs::create_dir_all(tests_path);

    let test_file =
        format!("crates/biome_js_analyze/tests/specs/nursery/{rule_name_lower_camel}/valid.js");
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(
            test_file,
            "/* should not generate diagnostics */\n\n var a = 1;",
        );
    }

    let test_file =
        format!("crates/biome_js_analyze/tests/specs/nursery/{rule_name_lower_camel}/invalid.js");
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, "\n\n var a = 1;\na = 2;\n a = 3;");
    }
}
