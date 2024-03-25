use biome_string_case::Case;
use bpaf::Bpaf;
use case::CaseExt;
use std::str::FromStr;
use xtask::project_root;

#[derive(Debug, Clone, Bpaf)]
pub enum RuleKind {
    Js,
    Json,
    Css,
}

impl FromStr for RuleKind {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "js" => Ok(Self::Js),
            "json" => Ok(Self::Json),
            "css" => Ok(Self::Css),
            _ => Err("Unsupported value"),
        }
    }
}

fn generate_rule_template(
    kind: &RuleKind,
    rule_name_upper_camel: &str,
    rule_name_lower_camel: &str,
) -> String {
    match kind {
        RuleKind::Js => {
            format!(
                r#"use biome_analyze::{{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic, Ast
}};
use biome_console::markup;
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
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
        let binding = ctx.query();
        None
    }}

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {{
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
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
            )
        }
        RuleKind::Css => {
            format!(
                r#"use biome_analyze::{{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic}};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_rowan::AstNode;

declare_rule! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {{}}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {{
    ///   color: red;
    /// }}
    /// ```
    ///
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = CssDeclarationOrRuleBlock;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {{
        let node = ctx.query();
        if node.items().into_iter().next().is_none() {{
            return Some(node.clone());
        }}
        None
    }}

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {{
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {{
                    "Unexpected empty block is not allowed"
                }},
            )
            .note(markup! {{
                    "This note will give you more information."
            }}),
        )
    }}
}}
"#
            )
        }
        RuleKind::Json => {
            unimplemented!("JSON variant not implemented yet.")
        }
    }
}

pub fn generate_new_lintrule(kind: RuleKind, rule_name: &str) {
    let crate_folder = match kind {
        RuleKind::Js => project_root().join("crates/biome_js_analyze"),
        RuleKind::Json => project_root().join("crates/biome_js_analyze"),
        RuleKind::Css => project_root().join("crates/biome_js_analyze"),
    };
    let rule_folder = crate_folder.join("src/lint/nursery");
    let test_folder = crate_folder.join("tests/specs/nursery");
    let rule_name_upper_camel = rule_name.to_camel();
    let rule_name_snake = rule_name.to_snake();
    let rule_name_lower_camel = rule_name_snake.to_camel_lowercase();

    // Generate rule code
    let code = generate_rule_template(
        &kind,
        rule_name_upper_camel.as_str(),
        rule_name_lower_camel.as_str(),
    );
    let file_name = format!("{}/{rule_name_snake}.rs", rule_folder.display());
    std::fs::write(file_name, code).unwrap();

    let categories_path = "crates/biome_diagnostics_categories/src/categories.rs";
    let mut categories = std::fs::read_to_string(categories_path).unwrap();

    if !categories.contains(&rule_name_lower_camel) {
        let kebab_case_rule = Case::Kebab.convert(&rule_name_lower_camel);
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
    let tests_path = format!("{}/{rule_name_lower_camel}", test_folder.display());
    let _ = std::fs::create_dir_all(tests_path);

    let test_file = format!("{}/{rule_name_lower_camel}/valid.js", test_folder.display());
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(
            test_file,
            "/* should not generate diagnostics */\n\n var a = 1;",
        );
    }

    let test_file = format!(
        "{}/{rule_name_lower_camel}/invalid.js",
        test_folder.display()
    );
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, "\n\n var a = 1;\na = 2;\n a = 3;");
    }
}
