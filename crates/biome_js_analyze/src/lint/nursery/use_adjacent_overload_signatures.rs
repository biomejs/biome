use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsModuleItem, JsClassDeclaration, JsFunctionDeclaration, JsModule, JsModuleItemList,
    TsDeclarationModule, TsDeclareStatement, TsInterfaceDeclaration, TsTypeAliasDeclaration,
    TsTypeMemberList,
};
use biome_rowan::{declare_node_union, AstNode, TextRange, TokenText};
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Disallow the use of overload signatures that are not next to each other.
    ///
    /// Overload signatures must be adjacent.
    /// If a key is defined multiple times, only the last definition takes effect. Previous definitions are ignored.
    /// This rule is useful for preventing accidental overloads that are not adjacent.
    /// It is recommended to keep the overload signatures adjacent to make the code easier to read and maintain.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type Foo = {
    ///   foo_type(s: string): void;
    ///   foo_type(n: number): void;
    ///   bar_type(): void;
    ///   foo_type(sn: string | number): void;
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface Foo {
    ///   foo_interface(s: string): void;
    ///   foo_interface(n: number): void;
    ///   bar_interface(): void;
    ///   foo_interface(sn: string | number): void;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
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
    /// ```ts
    /// declare namespace Foo {
    ///   export function foo_declare(s: string): void;
    ///   export function foo_declare(n: number): void;
    ///   export function foo_declare(sn: string | number): void;
    ///   export function bar_declare(): void;
    /// }
    /// ```
    ///
    /// ```ts
    /// type Foo = {
    ///   foo_type(s: string): void;
    ///   foo_type(n: number): void;
    ///   foo_type(sn: string | number): void;
    ///   bar_type(): void;
    /// };
    /// ```
    ///
    /// ```ts
    /// interface Foo {
    ///   foo_interface(s: string): void;
    ///   foo_interface(n: number): void;
    ///   foo_interface(sn: string | number): void;
    ///   bar_interface(): void;
    /// }
    /// ```
    ///
    /// ```ts
    /// class A {
    ///   fooA(s: string): void;
    ///   fooA(n: number): void;
    ///   fooA(sn: string | number): void {}
    ///   barA(): void {}
    /// }
    /// ```
    ///
    pub UseAdjacentOverloadSignatures {
        version: "1.9.0",
        name: "useAdjacentOverloadSignatures",
        language: "js",
        sources: &[
            RuleSource::EslintTypeScript("adjacent-overload-signatures")
        ],
        recommended: false,
    }
}

impl Rule for UseAdjacentOverloadSignatures {
    type Query = Ast<DeclarationOrModuleNode>;
    type State = Box<[(TokenText, TextRange)]>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let methods = match ctx.query() {
            // Handle export function foo() {} in declare namespace Foo {}
            DeclarationOrModuleNode::TsDeclareStatement(node) => {
                let declaration = node.declaration().ok()?;
                let items = declaration.as_ts_module_declaration()?.body().ok()?.items();
                collect_exports(&items)
            }
            // Handle interface Foo {}
            DeclarationOrModuleNode::TsInterfaceDeclaration(node) => {
                collect_type_member_list(&node.members())
            }
            // Handle type Foo = {}
            DeclarationOrModuleNode::TsTypeAliasDeclaration(node) => {
                let members = node
                    .ty()
                    .ok()
                    .and_then(|ty| ty.as_ts_object_type().cloned())?
                    .members();
                collect_type_member_list(&members)
            }
            // Handle class Foo {}
            DeclarationOrModuleNode::JsClassDeclaration(node) => collect_class(node),
            // Handle export function foo() {}
            DeclarationOrModuleNode::JsFunctionDeclaration(node) => collect_function(node),
            // Handle export function foo() {}
            DeclarationOrModuleNode::JsModule(node) => collect_exports(&node.items()),
            DeclarationOrModuleNode::TsDeclarationModule(node) => collect_exports(&node.items()),
        };

        if !methods.is_empty() {
            Some(methods.into_boxed_slice())
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
                "All "{text_ranges[0].0.text()}" signatures must be adjacent."
            },
        );
        for text_range in text_ranges.iter().skip(1) {
            diagnostic = diagnostic.detail(
                text_range.1,
                markup! {
                    "All "{text_range.0.text()}" signatures must be adjacent."
                },
            );
        }

        Some(diagnostic)
    }
}

fn collect_type_member_list(node: &TsTypeMemberList) -> Vec<(TokenText, TextRange)> {
    let mut methods: Vec<(TokenText, TextRange)> = Vec::new();
    let mut seen_methods = FxHashSet::default();
    let mut last_method = None;
    for member in node {
        if let Some(ts_method_signature) = member.as_ts_method_signature_type_member() {
            if let Ok(method_member) = ts_method_signature.name() {
                if let Some(text) = method_member.name() {
                    let range = method_member.range();
                    check_method(
                        text,
                        range,
                        &mut methods,
                        &mut seen_methods,
                        &mut last_method,
                    );
                }
            }
        }
    }
    methods
}

fn collect_class(node: &JsClassDeclaration) -> Vec<(TokenText, TextRange)> {
    let mut methods: Vec<(TokenText, TextRange)> = Vec::new();
    let mut seen_methods = FxHashSet::default();
    let mut last_method = None;
    let members = node.members();
    for member in members {
        if let Some(method_class) = member
            .as_js_method_class_member()
            .or_else(|| member.as_js_method_class_member())
        {
            if let Ok(method_member) = method_class.name() {
                if let Some(text) = method_member.name() {
                    let range = method_member.range();
                    check_method(
                        text,
                        range,
                        &mut methods,
                        &mut seen_methods,
                        &mut last_method,
                    );
                }
            }
        } else if let Some(method_class) = member.as_ts_method_signature_class_member() {
            if let Ok(method_member) = method_class.name() {
                if let Some(text) = method_member.name() {
                    let range = method_member.range();
                    check_method(
                        text,
                        range,
                        &mut methods,
                        &mut seen_methods,
                        &mut last_method,
                    );
                }
            }
        }
    }
    methods
}

fn collect_function(node: &JsFunctionDeclaration) -> Vec<(TokenText, TextRange)> {
    let mut methods: Vec<(TokenText, TextRange)> = Vec::new();
    let mut seen_methods = FxHashSet::default();
    let mut last_method = None;
    if let Some(return_type_annotation) = node.return_type_annotation() {
        if let Some(ty) = return_type_annotation
            .ty()
            .ok()
            .and_then(|ty| ty.as_any_ts_type().cloned())
        {
            if let Some(ts_object) = ty.as_ts_object_type() {
                let members = ts_object.members();
                for member in members {
                    if let Some(method_member) = member
                        .as_ts_method_signature_type_member()
                        .and_then(|m| m.name().ok())
                    {
                        if let Some(text) = method_member.name() {
                            let range = method_member.range();
                            check_method(
                                text,
                                range,
                                &mut methods,
                                &mut seen_methods,
                                &mut last_method,
                            );
                        }
                    }
                }
            }
        }
    }
    methods
}

fn collect_exports(items: &JsModuleItemList) -> Vec<(TokenText, TextRange)> {
    let mut methods: Vec<(TokenText, TextRange)> = Vec::new();
    let mut seen_methods = FxHashSet::default();
    let mut last_method = None;
    for item in items {
        if let AnyJsModuleItem::JsExport(node) = item {
            if let Ok(export) = node.export_clause() {
                if let Some(declaration_clause) = export.as_any_js_declaration_clause() {
                    if let Some(ts_declare) =
                        declaration_clause.as_ts_declare_function_declaration()
                    {
                        if let Some(name_token) = ts_declare.id().ok().and_then(|id| {
                            id.as_js_identifier_binding()
                                .and_then(|id| id.name_token().ok())
                        }) {
                            let text = name_token.token_text_trimmed();
                            let range = name_token.text_range();
                            check_method(
                                text,
                                range,
                                &mut methods,
                                &mut seen_methods,
                                &mut last_method,
                            );
                        }
                    }
                }
            }
        }
    }
    methods
}

// Check if the method is already seen and add it to the list of methods
fn check_method<T: Clone + Eq + std::hash::Hash + Into<TokenText>>(
    text: T,
    range: TextRange,
    methods: &mut Vec<(TokenText, TextRange)>,
    seen_methods: &mut FxHashSet<T>,
    last_method: &mut Option<T>,
) {
    if let Some(last) = last_method {
        if last != &text && seen_methods.contains(&text) {
            methods.push((text.clone().into(), range));
        } else {
            seen_methods.insert(text.clone());
        }
    } else {
        seen_methods.insert(text.clone());
    }
    *last_method = Some(text);
}

declare_node_union! {
    pub DeclarationOrModuleNode =
        JsClassDeclaration
        | JsFunctionDeclaration
        | TsInterfaceDeclaration
        | TsDeclareStatement
        | TsTypeAliasDeclaration
        | JsModule
        | TsDeclarationModule
}
