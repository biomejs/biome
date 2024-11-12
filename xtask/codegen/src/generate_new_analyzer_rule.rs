use biome_string_case::Case;
use bpaf::Bpaf;
use std::str::FromStr;
use xtask::project_root;

#[derive(Debug, Clone, Bpaf)]
pub enum LanguageKind {
    Js,
    Json,
    Css,
    Graphql,
}

impl LanguageKind {
    fn as_str(&self) -> &str {
        match self {
            Self::Js => "js",
            Self::Json => "json",
            Self::Css => "css",
            Self::Graphql => "graphql",
        }
    }
}

impl FromStr for LanguageKind {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "js" => Ok(Self::Js),
            "json" => Ok(Self::Json),
            "css" => Ok(Self::Css),
            "graphql" => Ok(Self::Graphql),
            _ => Err("Unsupported value"),
        }
    }
}

#[derive(Debug, Clone, Bpaf)]
pub enum Category {
    /// Lint rules
    Lint,
    /// Assist rules
    Assist,
    /// Syntax rule
    Syntax,
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "lint" => Ok(Self::Lint),
            "assist" => Ok(Self::Assist),
            _ => Err("Not supported"),
        }
    }
}

fn generate_rule_template(
    kind: &LanguageKind,
    category: &Category,
    rule_name_upper_camel: &str,
    rule_name_lower_camel: &str,
) -> String {
    let macro_name = match category {
        Category::Lint => "declare_lint_rule",
        Category::Assist => "declare_assist_rule",
        Category::Syntax => "declare_syntax_rule",
    };
    match kind {
        LanguageKind::Js => {
            format!(
                r#"use biome_analyze::{{
    context::RuleContext, {macro_name}, Rule, RuleDiagnostic, Ast
}};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
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
    /// // var a = 1;
    /// ```
    ///
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        language: "js",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
        let _binding = ctx.query();
        Some(())
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
        LanguageKind::Css => {
            format!(
                r#"use biome_analyze::{{context::RuleContext, {macro_name}, Ast, Rule, RuleDiagnostic}};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_rowan::AstNode;

{macro_name}! {{
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
        language: "css",
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
        LanguageKind::Json => {
            format!(
                r#"use biome_analyze::{{context::RuleContext, {macro_name}, Ast, Rule, RuleDiagnostic}};
use biome_console::markup;
use biome_json_syntax::JsonMember;
use biome_rowan::AstNode;

{macro_name}! {{
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
        language: "json",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Ast<JsonMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {{
        let _node = ctx.query();
        None
    }}

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {{
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = ctx.query().range();
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
        LanguageKind::Graphql => {
            format!(
                r#"use biome_analyze::{{context::RuleContext, {macro_name}, Ast, Rule, RuleDiagnostic}};
use biome_console::markup;
use biome_graphql_syntax::GraphqlRoot;
use biome_rowan::AstNode;

{macro_name}! {{
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
    /// ```graphql,expect_diagnostic
    /// quer {{}}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {{
    ///   field
    /// }}
    /// ```
    ///
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        language: "graphql",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Ast<GraphqlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {{
        let _node = ctx.query();
        None
    }}

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {{
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = ctx.query().range();
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
    }
}

pub fn generate_new_analyzer_rule(kind: LanguageKind, category: Category, rule_name: &str) {
    let rule_name_camel = Case::Camel.convert(rule_name);
    let rule_kind = kind.as_str();
    let crate_folder = project_root().join(format!("crates/biome_{rule_kind}_analyze"));
    let test_folder = crate_folder.join("tests/specs/nursery");
    let rule_folder = match &category {
        Category::Lint => crate_folder.join("src/lint/nursery"),
        Category::Assist => crate_folder.join("src/assists/nursery"),
        Category::Syntax => crate_folder.join("src/syntax/nursery"),
    };
    // Generate rule code
    let code = generate_rule_template(
        &kind,
        &category,
        Case::Pascal.convert(rule_name).as_str(),
        rule_name_camel.as_str(),
    );
    if !rule_folder.exists() {
        std::fs::create_dir(rule_folder.clone()).expect("To create the rule folder");
    }
    let file_name = format!(
        "{}/{}.rs",
        rule_folder.display(),
        Case::Snake.convert(rule_name)
    );
    std::fs::write(file_name.clone(), code).unwrap_or_else(|_| panic!("To write {}", &file_name));

    let categories_path = "crates/biome_diagnostics_categories/src/categories.rs";
    let mut categories = std::fs::read_to_string(categories_path).unwrap();

    if !categories.contains(&rule_name_camel) {
        let kebab_case_rule = Case::Kebab.convert(&rule_name_camel);
        // We sort rules to reduce conflicts between contributions made in parallel.
        let rule_line = match category {
            Category::Lint => format!(
                r#"    "lint/nursery/{rule_name_camel}": "https://biomejs.dev/linter/rules/{kebab_case_rule}","#
            ),
            Category::Assist => format!(
                r#"    "assists/nursery/{rule_name_camel}": "https://biomejs.dev/assists/{kebab_case_rule}","#
            ),
            Category::Syntax => format!(r#"    "syntax/nursery/{rule_name_camel}","#),
        };
        let lint_start = match category {
            Category::Lint => "define_categories! {\n",
            Category::Assist => "    // start assist actions\n",
            Category::Syntax => "    // start syntax rules\n",
        };
        let lint_end = match category {
            Category::Lint => "\n    // end lint rules\n",
            Category::Assist => "\n    // end assist actions\n",
            Category::Syntax => "\n  ;  // end syntax rules\n",
        };
        debug_assert!(categories.contains(lint_start), "{}", lint_start);
        debug_assert!(categories.contains(lint_end), "{}", lint_end);
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
    let tests_path = format!("{}/{rule_name_camel}", test_folder.display());
    let _ = std::fs::create_dir_all(tests_path);

    let test_file = format!(
        "{}/{rule_name_camel}/valid.{rule_kind}",
        test_folder.display()
    );
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(
            test_file,
            "/* should not generate diagnostics */\n// var a = 1;",
        );
    }

    let test_file = format!(
        "{}/{rule_name_camel}/invalid.{rule_kind}",
        test_folder.display()
    );
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, "var a = 1;\na = 2;\na = 3;");
    }
}
