use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::{markup, MarkupBuf};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMember, AnyTsType, AnyTsTypeMember, ClassMemberName, JsClassDeclaration,
    JsSyntaxToken, TsDeclareStatement, TsInterfaceDeclaration, TsReferenceType,
    TsTypeAliasDeclaration,
};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
    /// Enforce proper usage of `new` and `constructor`.
    ///
    /// In JavaScript, classes utilize the `constructor` method to initialize a new instance. On the other hand, TypeScript interfaces can describe a class type with a `new()` method signature, though this pattern is not commonly seen in real-world code. Developers, especially those new to JavaScript or TypeScript, might occasionally confuse the use of `constructor` with `new`.
    /// This rule triggers warnings in the following scenarios:
    /// - When a class has a method named `new`.
    /// - When an interface defines a method named `constructor` or `new` that returns the interface type.
    /// - When a type alias has a `constructor` method.
    ///
    /// You should not use this rule if you intentionally want a class with a `new` method, and you're confident nobody working in your code will mistake it with an `constructor`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface I {
    ///   new (): I;
    ///   constructor(): void;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class C {
    ///   new(): C;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// declare class C {
    ///   constructor();
    /// }
    ///
    /// interface I {
    ///   new (): C;
    /// }
    /// ```
    pub NoMisleadingInstantiator {
        version: "1.3.0",
        name: "noMisleadingInstantiator",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-misused-new")],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub DeclarationQuery = TsInterfaceDeclaration |  TsTypeAliasDeclaration | JsClassDeclaration | TsDeclareStatement
}

pub enum RuleState {
    ClassMisleadingNew(TextRange),
    InterfaceMisleadingNew(TextRange),
    InterfaceMisleadingConstructor(TextRange),
    TypeAliasMisleadingConstructor(TextRange),
}

impl RuleState {
    fn message(&self) -> MarkupBuf {
        match self {
            RuleState::ClassMisleadingNew(_) => (markup! {
                "Don't use the "<Emphasis>"new"</Emphasis>" method in classes."
            })
            .to_owned(),
            RuleState::InterfaceMisleadingNew(_) => (markup! {
                "Don't use the "<Emphasis>"new"</Emphasis>" method in interfaces."
            })
            .to_owned(),
            RuleState::InterfaceMisleadingConstructor(_) => (markup! {
                "Don't use the "<Emphasis>"constructor"</Emphasis>" method in interfaces."
            })
            .to_owned(),
            RuleState::TypeAliasMisleadingConstructor(_) => (markup! {
                "Don't use the "<Emphasis>"constructor"</Emphasis>" method in type aliases."
            })
            .to_owned(),
        }
    }

    fn note(&self) -> MarkupBuf {
        match self {
            RuleState::ClassMisleadingNew(_) => (markup! {
                ""<Emphasis>"new"</Emphasis>" is typically used to instantiate objects. In classes, its usage can be misleading."
            })
            .to_owned(),
            RuleState::InterfaceMisleadingNew(_) => (markup! {
                ""<Emphasis>"new"</Emphasis>" in an interface suggests it's instantiable, which is incorrect. The returned type should different from the constructor's type."
            })
            .to_owned(),
            RuleState::InterfaceMisleadingConstructor(_) => (markup! {
                "Interfaces define a contract, not an implementation. Thus, including a "<Emphasis>"constructor"</Emphasis>"is not appropriate."
            })
            .to_owned(),
            RuleState::TypeAliasMisleadingConstructor(_) => (markup! {
                "Type aliases simply rename types. They don't execute code, so a "<Emphasis>"constructor"</Emphasis>"is misleading."
            })
            .to_owned(),
        }
    }

    fn range(&self) -> &TextRange {
        match self {
            RuleState::ClassMisleadingNew(range)
            | RuleState::InterfaceMisleadingNew(range)
            | RuleState::InterfaceMisleadingConstructor(range)
            | RuleState::TypeAliasMisleadingConstructor(range) => range,
        }
    }
}

impl Rule for NoMisleadingInstantiator {
    type Query = Ast<DeclarationQuery>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            DeclarationQuery::TsInterfaceDeclaration(decl) => check_interface_methods(decl),
            DeclarationQuery::TsTypeAliasDeclaration(decl) => check_type_alias(decl),
            DeclarationQuery::JsClassDeclaration(decl) => check_class_methods(decl),
            DeclarationQuery::TsDeclareStatement(decl) => {
                let decl = decl.declaration().ok()?;
                let decl = decl.as_js_class_declaration()?;
                check_class_methods(decl)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(rule_category!(), state.range(), state.message())
            .note(state.note());
        Some(diagnostic)
    }
}

/// Checks if the interface has a misleading constructor or new method.
fn check_interface_methods(decl: &TsInterfaceDeclaration) -> Option<RuleState> {
    let interface_ident = decl
        .id()
        .ok()?
        .as_ts_identifier_binding()?
        .name_token()
        .ok()?;
    for member in decl.members() {
        match member {
            AnyTsTypeMember::TsConstructSignatureTypeMember(construct)
                if construct.new_token().ok().is_some() =>
            {
                let any_ts_type = construct.type_annotation()?.ty().ok()?;
                match any_ts_type {
                    AnyTsType::TsReferenceType(ref_type) => {
                        let return_type_ident = extract_return_type_ident(&ref_type)?;
                        if interface_ident.text_trimmed() == return_type_ident.text_trimmed() {
                            return Some(RuleState::InterfaceMisleadingNew(construct.range()));
                        }
                    }
                    AnyTsType::TsThisType(this_type) if this_type.this_token().ok().is_some() => {
                        return Some(RuleState::InterfaceMisleadingNew(construct.range()));
                    }
                    _ => continue,
                }
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                let method_name = method.name().ok()?.name()?;
                if method_name == "constructor" {
                    return Some(RuleState::InterfaceMisleadingConstructor(method.range()));
                }
            }
            _ => continue,
        };
    }
    None
}

/// Checks if the class has a misleading new method.
fn check_class_methods(js_class_decl: &JsClassDeclaration) -> Option<RuleState> {
    let class_ident = js_class_decl
        .id()
        .ok()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()?;
    for member in js_class_decl.members() {
        if let AnyJsClassMember::TsMethodSignatureClassMember(method) = member {
            if let Some(ClassMemberName::Public(name)) = method.name().ok()?.name() {
                if name.text() == "new" {
                    let return_type = method.return_type_annotation()?.ty().ok()?;
                    match return_type.as_any_ts_type()? {
                        AnyTsType::TsReferenceType(ref_type) => {
                            let return_type_ident = extract_return_type_ident(ref_type)?;
                            if class_ident.text_trimmed() == return_type_ident.text_trimmed() {
                                return Some(RuleState::ClassMisleadingNew(method.range()));
                            }
                        }
                        AnyTsType::TsThisType(this_type)
                            if this_type.this_token().ok().is_some() =>
                        {
                            return Some(RuleState::ClassMisleadingNew(method.range()));
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
    None
}

/// Checks if the type alias has a misleading constructor method.
fn check_type_alias(decl: &TsTypeAliasDeclaration) -> Option<RuleState> {
    let any_ts_type = decl.ty().ok()?;
    let object = any_ts_type.as_ts_object_type()?;
    let method = object
        .members()
        .into_iter()
        .find_map(|member| member.as_ts_method_signature_type_member().cloned())?;
    if method.name().ok()?.name()? == "constructor" {
        return Some(RuleState::TypeAliasMisleadingConstructor(method.range()));
    }
    None
}

/// Extracts the identifier from a reference type.
fn extract_return_type_ident(reference_type: &TsReferenceType) -> Option<JsSyntaxToken> {
    reference_type
        .name()
        .ok()?
        .as_js_reference_identifier()?
        .value_token()
        .ok()
}
