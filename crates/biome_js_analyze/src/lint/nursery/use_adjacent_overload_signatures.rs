use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsStatement,
    JsModuleItemList, SyntaxNodeText,
};
use biome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Disallow the use of overload signatures that are not next to each other.
    ///
    /// Overload signatures must be adjacent.
    /// If a key is defined multiple times, only the last definition takes effect. Previous definitions are ignored.
    /// This rule is useful for preventing accidental overloads that are not adjacent.
    /// It is recommended to keep the overload signatures adjacent to make the code easier to read and maintain.
    ///
    /// Source: https://typescript-eslint.io/rules/adjacent-overload-signatures/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// type Foo = {
    ///   foo_type(s: string): void;
    ///   foo_type(n: number): void;
    ///   bar_type(): void;
    ///   foo_type(sn: string | number): void;
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// interface Foo {
    ///   foo_interface(s: string): void;
    ///   foo_interface(n: number): void;
    ///   bar_interface(): void;
    ///   foo_interface(sn: string | number): void;
    /// }
    /// ``
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///   fooA(s: string): void;
    ///   fooA(n: number): void;
    ///   barA(): void {};
    ///   fooA(sn: string | number): void {};
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// declare namespace Foo {
    ///   export function foo_declare(s: string): void;
    ///   export function foo_declare(n: number): void;
    ///   export function foo_declare(sn: string | number): void;
    ///   export function bar_declare(): void;
    /// }
    /// ````
    ///
    /// ```js
    /// type Foo = {
    ///   foo_type(s: string): void;
    ///   foo_type(n: number): void;
    ///   foo_type(sn: string | number): void;
    ///   bar_type(): void;
    /// };
    /// ```
    ///
    /// ```js
    /// interface Foo {
    ///   foo_interface(s: string): void;
    ///   foo_interface(n: number): void;
    ///   foo_interface(sn: string | number): void;
    ///   bar_interface(): void;
    /// }
    /// ```
    ///
    /// ```js
    /// class A {
    ///   fooA(s: string): void;
    ///   fooA(n: number): void;
    ///   fooA(sn: string | number): void {}
    ///   barA(): void {}
    /// }
    /// ```
    ///
    pub UseAdjacentOverloadSignatures {
        version: "next",
        name: "useAdjacentOverloadSignatures",
        sources: &[
            RuleSource::EslintTypeScript("adjacent-overload-signatures")
        ],
        recommended: false,
    }
}

impl Rule for UseAdjacentOverloadSignatures {
    type Query = Ast<JsModuleItemList>;
    type State = Vec<(String, TextRange)>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut methods: Vec<Vec<(SyntaxNodeText, usize, TextRange)>> = Vec::new();
        let mut index = 0;
        let mut export_vec = vec![];

        for item in node {
            let AnyJsModuleItem::AnyJsStatement(node) = item else {
                continue;
            };
            match node {
                // type
                AnyJsStatement::TsTypeAliasDeclaration(node) => {
                    let ty = node.ty().ok()?;
                    let ts_object = ty.as_ts_object_type()?;
                    let members = ts_object.members();
                    let mut type_vec = vec![];
                    for (type_index, member) in members.into_iter().enumerate() {
                        let method_name = member.as_ts_method_signature_type_member()?.name().ok()?;
                        let text = method_name.syntax().text_trimmed();
                        let range = method_name.syntax().text_range();
                        type_vec.push((text, type_index, range));
                    }
                    methods.push(type_vec.clone());
                }
                // interface
                AnyJsStatement::TsInterfaceDeclaration(node) => {
                    let members = node.members();
                    let mut interface_vec = vec![];
                    for (interface_index, member) in members.into_iter().enumerate() {
                        let ts_method_signature = member.as_ts_method_signature_type_member()?;
                        let method_name = ts_method_signature.name().ok()?;
                        let text = method_name.syntax().text_trimmed();
                        let range = method_name.syntax().text_range();
                        interface_vec.push((
                            text,
                            interface_index,
                            range,
                        ));
                    }
                    methods.push(interface_vec.clone());
                }
                // class
                AnyJsStatement::JsClassDeclaration(node) => {
                    let members = node.members();
                    let mut class_vec = vec![];
                    let mut class_index = 0;
                    for member in members {
                        if let Some(method_class) = member.as_js_method_class_member() {
                            let method_name = method_class.name().ok()?;
                            let text = method_name.syntax().text_trimmed();
                            let range = method_name.syntax().text_range();
                            class_vec.push((text, class_index, range));
                            class_index += 1;
                        } else if let Some(method_class) = member.as_ts_method_signature_class_member() {
                            let method_name = method_class.name().ok()?;
                            let text = method_name.syntax().text_trimmed();
                            let range = method_name.syntax().text_range();
                            class_vec.push((text, class_index, range));
                            class_index += 1;
                        }
                    }
                    methods.push(class_vec.clone());
                }
                _ => {}
            }
        }
        // AnyJsModuleItem::JsExport() matches with `export function` even in declare namespace
        // So if I catch with AnyJsStatement::TsDeclareFunction() it also catches the export function in declare namespace
        for item in node {
            let AnyJsModuleItem::JsExport(node) = item else {
                continue;
            };
            let export = node.export_clause().ok()?;
            let declaration_clause = export.as_any_js_declaration_clause()?;
            let ts_declare = declaration_clause.as_ts_declare_function_declaration()?;
            let name_token = ts_declare.id().ok()?.as_js_identifier_binding()?.name_token().ok()?;
            let parent = name_token.parent()?;
            let text = parent.text_trimmed();
            let range = parent.text_range();
            export_vec.push((text, index, range));
            index += 1;
        }
        methods.push(export_vec.clone());
        let adjacent_overload_violations = check_adjacent_overload_violations(&methods);
        let violation_ranges: Vec<(String, TextRange)> = adjacent_overload_violations
            .iter()
            .map(|(text, range)| (text.to_string(), *range))
            .collect();

        if !violation_ranges.is_empty() {
            Some(violation_ranges)
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let text_ranges = state;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            text_ranges[0].1,
            markup! {
                "All "{text_ranges[0].0}" signatures must be adjacent."
            },
        );
        for text_range in text_ranges.iter().skip(1) {
            diagnostic = diagnostic.detail(
                text_range.1,
                markup! {
                    "All "{text_range.0}" signatures must be adjacent."
                },
            );
        }

        Some(diagnostic)
    }
}

fn check_adjacent_overload_violations(
    groups: &[Vec<(SyntaxNodeText, usize, TextRange)>],
) -> Vec<(SyntaxNodeText, TextRange)> {
    let mut violations: Vec<(SyntaxNodeText, TextRange)> = Vec::new();

    for group in groups.iter() {
        let mut method_positions: Vec<(SyntaxNodeText, Vec<usize>, TextRange)> = Vec::new();

        for (name, position, range) in group {
            if let Some((_, positions, last_range)) =
                method_positions.iter_mut().find(|(n, _, _)| *n == *name)
            {
                positions.push(*position);
                *last_range = *range;
            } else {
                method_positions.push((name.clone(), vec![*position], *range));
            }
        }

        for (method, ref positions, last_range) in &method_positions {
            if positions.len() > 1 {
                let mut sorted_positions = positions.clone();
                sorted_positions.sort_unstable();
                let expected: Vec<usize> =
                    (sorted_positions[0]..=sorted_positions[sorted_positions.len() - 1]).collect();
                if sorted_positions != expected {
                    violations.push((method.clone(), *last_range));
                }
            }
        }
    }

    violations
}
