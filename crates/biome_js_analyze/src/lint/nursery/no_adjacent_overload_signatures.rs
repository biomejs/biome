use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsModuleItem, AnyJsStatement, JsExport, JsIdentifierBinding,
    JsModuleItemList, JsSyntaxKind, SyntaxNodeText, TsMethodSignatureTypeMember,
};
use biome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Disallow non adjacent overload signatures.
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
    ///
    /// type Foo = {
    ///   foo_type(s: string): void;
    ///   foo_type(n: number): void;
    ///   bar_type(): void;
    ///   foo_type(sn: string | number): void;
    /// };
    ///
    /// interface Foo {
    ///   foo_interface(s: string): void;
    ///   foo_interface(n: number): void;
    ///   bar_interface(): void;
    ///   foo_interface(sn: string | number): void;
    /// }
    ///
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
    ///
    /// type Foo = {
    ///   foo_type(s: string): void;
    ///   foo_type(n: number): void;
    ///   foo_type(sn: string | number): void;
    ///   bar_type(): void;
    /// };
    ///
    /// interface Foo {
    ///   foo_interface(s: string): void;
    ///   foo_interface(n: number): void;
    ///   foo_interface(sn: string | number): void;
    ///   bar_interface(): void;
    /// }
    ///
    /// class A {
    ///   fooA(s: string): void;
    ///   fooA(n: number): void;
    ///   fooA(sn: string | number): void {}
    ///   barA(): void {}
    /// }
    /// ```
    ///
    pub NoAdjacentOverloadSignatures {
        version: "next",
        name: "noAdjacentOverloadSignatures",
        sources: &[
            RuleSource::EslintTypeScript("adjacent-overload-signatures")
        ],
        recommended: false,
    }
}

impl Rule for NoAdjacentOverloadSignatures {
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
            if let AnyJsModuleItem::AnyJsStatement(node) = item {
                match node {
                    AnyJsStatement::JsClassDeclaration(node) => {
                        let mut class_index = 0;
                        let mut class_vec = vec![];
                        let members = node.members();
                        for member in members {
                            if let AnyJsClassMember::JsBogusMember(m) = member {
                                let items = m.items();
                                for item in items {
                                    if item.kind() == JsSyntaxKind::JS_LITERAL_MEMBER_NAME {
                                        let name = item.as_node()?.text_trimmed();
                                        class_vec.push((
                                            name,
                                            class_index,
                                            item.as_node()?.text_range(),
                                        ));
                                        class_index += 1;
                                    }
                                }
                            }
                        }
                        methods.push(class_vec.clone());
                    }
                    AnyJsStatement::JsBogusStatement(node) => {
                        let items = node.items();
                        for item in items {
                            match item.kind() {
                                // declare namespace
                                JsSyntaxKind::TS_MODULE_DECLARATION => {
                                    let items = item.as_node()?.children_with_tokens();
                                    for item in items {
                                        if item.kind() == JsSyntaxKind::TS_MODULE_BLOCK {
                                            let item = item.as_node()?.descendants();
                                            let items = item.collect::<Vec<_>>();
                                            let mut module_vec = vec![];
                                            for item in items {
                                                if item.kind() == JsSyntaxKind::JS_MODULE_ITEM_LIST
                                                {
                                                    let mut module_index = 0;
                                                    let items = item.descendants();
                                                    for item in items {
                                                        let item = item.clone();
                                                        if item.kind() == JsSyntaxKind::JS_EXPORT {
                                                            if let Some(export) =
                                                                JsExport::cast(item)
                                                            {
                                                                if let Ok(export_from_clause) =
                                                                    export.export_clause()
                                                                {
                                                                    if let Some(export_from_clause) = export_from_clause.as_any_js_declaration_clause() {
                                                                                        let ts_declare = export_from_clause.as_ts_declare_function_declaration();
                                                                                        if let Some(ts_declare) = ts_declare {
                                                                                            let name_token = ts_declare.id().ok()?.as_js_identifier_binding()?.name_token().ok()?;
                                                                                            let parent = name_token.parent()?;
                                                                                            let text = parent.text_trimmed();
                                                                                            let range = parent.text_range();
                                                                                            module_vec.push((text, module_index, range));
                                                                                            module_index += 1;
                                                                                        }
                                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            methods.push(module_vec.clone());
                                        }
                                    }
                                }
                                // type
                                JsSyntaxKind::TS_OBJECT_TYPE => {
                                    let items = item.as_node()?.children_with_tokens();
                                    let mut type_vec = vec![];
                                    for item in items {
                                        if item.kind() == JsSyntaxKind::TS_TYPE_MEMBER_LIST {
                                            let mut type_index = 0;
                                            let items = item.as_node()?.descendants();
                                            for item in items {
                                                if item.kind()
                                                    == JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER
                                                {
                                                    let ts_method_signature =
                                                        TsMethodSignatureTypeMember::cast(item)?;
                                                    let method_name =
                                                        ts_method_signature.name().ok()?;
                                                    let text = method_name.syntax().text_trimmed();
                                                    let range = method_name.syntax().text_range();
                                                    type_vec.push((text, type_index, range));
                                                    type_index += 1;
                                                }
                                            }
                                        }
                                    }
                                    methods.push(type_vec.clone());
                                }
                                // interface
                                JsSyntaxKind::TS_TYPE_MEMBER_LIST => {
                                    let items = item.as_node()?.children_with_tokens();
                                    let mut interface_index = 0;
                                    let mut interface_vec = vec![];
                                    for item in items {
                                        if item.kind()
                                            == JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER
                                        {
                                            let items = item.as_node()?.descendants();
                                            for item in items {
                                                if item.kind()
                                                    == JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER
                                                {
                                                    let ts_method_signature =
                                                        TsMethodSignatureTypeMember::cast(item)?;
                                                    let method_name =
                                                        ts_method_signature.name().ok()?;
                                                    let text = method_name.syntax().text_trimmed();
                                                    let range = method_name.syntax().text_range();
                                                    interface_vec.push((
                                                        text,
                                                        interface_index,
                                                        range,
                                                    ));
                                                    interface_index += 1;
                                                }
                                            }
                                        }
                                    }
                                    methods.push(interface_vec.clone());
                                }
                                // export function
                                JsSyntaxKind::JS_BOGUS_STATEMENT => {
                                    let items = item.as_node()?.children_with_tokens();
                                    for item in items {
                                        if item.kind() == JsSyntaxKind::JS_IDENTIFIER_BINDING {
                                            let item_node = item.as_node()?;
                                            if let Some(ident) =
                                                JsIdentifierBinding::cast(item_node.clone())
                                            {
                                                let name_token = ident.name_token().ok()?;
                                                let node = name_token.parent()?;
                                                let text = node.text_trimmed();
                                                export_vec.push((
                                                    text,
                                                    index,
                                                    item_node.text_range(),
                                                ));
                                                index += 1;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
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
