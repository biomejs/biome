use biome_string_case::Case;
use bpaf::Bpaf;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use xtask_glue::project_root;

#[derive(Debug, Clone, Bpaf)]
pub enum LanguageKind {
    Js,
    Json,
    Css,
    Graphql,
    Html,
    HtmlVue,
}

impl LanguageKind {
    fn as_str(&self) -> &str {
        match self {
            Self::Js => "js",
            Self::Json => "json",
            Self::Css => "css",
            Self::Graphql => "graphql",
            Self::Html => "html",
            Self::HtmlVue => "html",
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
            "html" => Ok(Self::Html),
            "html-vue" => Ok(Self::HtmlVue),
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

fn test_group_name(category: &Category) -> &'static str {
    match category {
        Category::Assist => "source",
        Category::Lint | Category::Syntax => "nursery",
    }
}

fn rule_folder(crate_folder: &Path, category: &Category) -> PathBuf {
    match category {
        Category::Lint => crate_folder.join("src/lint/nursery"),
        Category::Assist => crate_folder.join("src/assist/source"),
        Category::Syntax => crate_folder.join("src/syntax/nursery"),
    }
}

fn category_entry(category: &Category, rule_name_camel: &str, kebab_case_rule: &str) -> String {
    match category {
        Category::Lint => format!(
            r#"    "lint/nursery/{rule_name_camel}": "https://biomejs.dev/linter/rules/{kebab_case_rule}","#
        ),
        Category::Assist => format!(
            r#"    "assist/source/{rule_name_camel}": "https://biomejs.dev/assist/actions/{kebab_case_rule}","#
        ),
        Category::Syntax => format!(r#"    "syntax/nursery/{rule_name_camel}","#),
    }
}

fn category_markers(category: &Category) -> (&'static str, &'static str) {
    match category {
        Category::Lint => ("define_categories! {\n", "\n    // end lint rules\n"),
        Category::Assist => (
            "    // start assist actions\n",
            "\n    // end assist actions\n",
        ),
        Category::Syntax => (
            "    // start syntax rules\n",
            "\n  ;  // end syntax rules\n",
        ),
    }
}

fn generate_rule_template(
    kind: &LanguageKind,
    category: &Category,
    rule_name_upper_camel: &str,
    rule_name_lower_camel: &str,
    rule_name_snake_case: &str,
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
use biome_rule_options::{rule_name_snake_case}::{rule_name_upper_camel}Options;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// You can use asides to highlight important information:
    /// :::note
    /// Important information for users.
    /// :::
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
    type Options = {rule_name_upper_camel}Options;

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
use biome_rule_options::{rule_name_snake_case}::{rule_name_upper_camel}Options;

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
    /// You can use asides to highlight important information:
    /// :::note
    /// Important information for users.
    /// :::
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
    type Options = {rule_name_upper_camel}Options;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
        let node = ctx.query();
        if node.items().into_iter().next().is_none() {{
            return Some(node.clone());
        }}
        None
    }}

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {{
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = state.range();
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
use biome_rule_options::{rule_name_snake_case}::{rule_name_upper_camel}Options;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// You can use asides to highlight important information:
    /// :::note
    /// Important information for users.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {{
    ///     "test": true,
    ///     "test": true
    /// }}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {{
    ///     "test": true
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
    type Options = {rule_name_upper_camel}Options;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
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
use biome_rule_options::{rule_name_snake_case}::{rule_name_upper_camel}Options;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// You can use asides to highlight important information:
    /// :::note
    /// Important information for users.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {{}}
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
    type Options = {rule_name_upper_camel}Options;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
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
        LanguageKind::HtmlVue => {
            format!(
                r#"use biome_analyze::{{context::RuleContext, {macro_name}, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource}};
use biome_console::markup;
use biome_html_syntax::HtmlRoot;
use biome_rowan::AstNode;
use biome_rule_options::{rule_name_snake_case}::{rule_name_upper_camel}Options;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// You can use asides to highlight important information:
    /// :::note
    /// Important information for users.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div>foo</div>
    /// ```
    ///
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("rule-name").same()],
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Ast<HtmlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = {rule_name_upper_camel}Options;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
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
        LanguageKind::Html => {
            format!(
                r#"use biome_analyze::{{context::RuleContext, {macro_name}, Ast, Rule, RuleDiagnostic}};
use biome_console::markup;
use biome_html_syntax::HtmlRoot;
use biome_rowan::AstNode;
use biome_rule_options::{rule_name_snake_case}::{rule_name_upper_camel}Options;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// You can use asides to highlight important information:
    /// :::note
    /// Important information for users.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div>foo</div>
    /// ```
    ///
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        language: "html",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Ast<HtmlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = {rule_name_upper_camel}Options;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
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
    let root = project_root();
    generate_new_analyzer_rule_at(root.as_path(), kind, category, rule_name);
}

fn generate_new_analyzer_rule_at(
    root: &Path,
    kind: LanguageKind,
    category: Category,
    rule_name: &str,
) {
    let rule_name_camel = Case::Camel.convert(rule_name);
    let rule_kind = kind.as_str();
    let test_group = test_group_name(&category);
    let test_extension = if matches!(kind, LanguageKind::HtmlVue) {
        "vue"
    } else {
        rule_kind
    };
    let valid_contents = match kind {
        LanguageKind::Json => "{\n\t\"test\": \"value\"\n}",
        LanguageKind::Css => "/* should not generate diagnostics */\np {\n\tcolor: red;\n}",
        LanguageKind::Graphql => "# should not generate diagnostics\nquery {\n\tfield\n}",
        LanguageKind::Html | LanguageKind::HtmlVue => {
            "<!-- should not generate diagnostics -->\n<div>ok</div>"
        }
        _ => "/* should not generate diagnostics */\n// var a = 1;",
    };
    let invalid_contents = match kind {
        LanguageKind::Json => "{\n\t\"test\": \"value\",\n\t\"test\": \"value\"\n}",
        LanguageKind::Css => "/* should generate diagnostics */\np {}",
        LanguageKind::Graphql => "# should generate diagnostics\nquery {}",
        LanguageKind::Html | LanguageKind::HtmlVue => {
            "<!-- should generate diagnostics -->\n<div></div>"
        }
        _ => "/* should generate diagnostics */\nvar a = 1;\na = 2;\na = 3;",
    };
    let crate_folder = root.join(format!("crates/biome_{rule_kind}_analyze"));
    let test_folder = crate_folder.join(format!("tests/specs/{test_group}"));
    let rule_folder = rule_folder(&crate_folder, &category);
    // Generate rule code
    let code = generate_rule_template(
        &kind,
        &category,
        Case::Pascal.convert(rule_name).as_str(),
        rule_name_camel.as_str(),
        Case::Snake.convert(rule_name).as_str(),
    );
    if !rule_folder.exists() {
        std::fs::create_dir_all(rule_folder.clone()).expect("To create the rule folder");
    }
    let file_name = format!(
        "{}/{}.rs",
        rule_folder.display(),
        Case::Snake.convert(rule_name)
    );
    std::fs::write(file_name.clone(), code).unwrap_or_else(|_| panic!("To write {}", &file_name));

    let categories_path = root.join("crates/biome_diagnostics_categories/src/categories.rs");
    let mut categories = std::fs::read_to_string(&categories_path).unwrap();

    if !categories.contains(&rule_name_camel) {
        let kebab_case_rule = Case::Kebab.convert(&rule_name_camel);
        // We sort rules to reduce conflicts between contributions made in parallel.
        let rule_line = category_entry(&category, &rule_name_camel, &kebab_case_rule);
        let (category_start, category_end) = category_markers(&category);
        debug_assert!(categories.contains(category_start), "{}", category_start);
        debug_assert!(categories.contains(category_end), "{}", category_end);
        let category_start_index = categories.find(category_start).unwrap() + category_start.len();
        let category_end_index = categories.find(category_end).unwrap();
        let category_rule_text = &categories[category_start_index..category_end_index];
        let mut category_rules: Vec<_> = category_rule_text
            .lines()
            .chain(Some(&rule_line[..]))
            .collect();
        category_rules.sort_unstable();
        let new_category_rule_text = category_rules.join("\n");
        categories.replace_range(
            category_start_index..category_end_index,
            &new_category_rule_text,
        );
        std::fs::write(categories_path, categories).unwrap();
    }

    // Generate test code
    let tests_path = format!("{}/{rule_name_camel}", test_folder.display());
    let _ = std::fs::create_dir_all(tests_path);

    let test_file = format!(
        "{}/{rule_name_camel}/valid.{test_extension}",
        test_folder.display()
    );
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, valid_contents);
    }

    let test_file = format!(
        "{}/{rule_name_camel}/invalid.{test_extension}",
        test_folder.display()
    );
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, invalid_contents);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new(name: &str) -> Self {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("biome-codegen-{name}-{id}"));
            if path.exists() {
                let _ = fs::remove_dir_all(&path);
            }
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn write_categories_fixture(root: &Path) {
        let categories_dir = root.join("crates/biome_diagnostics_categories/src");
        fs::create_dir_all(&categories_dir).unwrap();
        fs::write(
            categories_dir.join("categories.rs"),
            r#"define_categories! {
    // start assist actions

    // end assist actions
    // start syntax rules

  ;  // end syntax rules
    // end lint rules
}
"#,
        )
        .unwrap();
    }

    fn write_html_analyzer_fixture(root: &Path) {
        fs::create_dir_all(root.join("crates/biome_html_analyze/src/assist")).unwrap();
        fs::create_dir_all(root.join("crates/biome_html_analyze/tests/specs")).unwrap();
    }

    #[test]
    fn html_assist_rules_use_the_source_group() {
        let temp = TestDir::new("html-assist");
        write_categories_fixture(temp.path());
        write_html_analyzer_fixture(temp.path());

        generate_new_analyzer_rule_at(
            temp.path(),
            LanguageKind::Html,
            Category::Assist,
            "useSortedAttributes",
        );

        assert!(
            temp.path()
                .join("crates/biome_html_analyze/src/assist/source/use_sorted_attributes.rs")
                .exists()
        );
        assert!(
            temp.path()
                .join("crates/biome_html_analyze/tests/specs/source/useSortedAttributes/valid.html")
                .exists()
        );

        let categories = fs::read_to_string(
            temp.path()
                .join("crates/biome_diagnostics_categories/src/categories.rs"),
        )
        .unwrap();
        assert!(categories.contains(
            r#""assist/source/useSortedAttributes": "https://biomejs.dev/assist/actions/use-sorted-attributes","#,
        ));
        assert!(!categories.contains("assists/nursery/useSortedAttributes"));
    }
}
