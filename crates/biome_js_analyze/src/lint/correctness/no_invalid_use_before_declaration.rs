use crate::services::semantic::SemanticServices;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExportNamedSpecifier, AnyJsFunction, AnyJsIdentifierUsage, JsClassDeclaration,
    JsConstructorClassMember, JsFileSource, JsGetterClassMember, JsGetterObjectMember,
    JsMethodClassMember, JsMethodObjectMember, JsModule, JsScript, JsSetterClassMember,
    JsSetterObjectMember, JsStaticInitializationBlockClassMember, JsVariableDeclarationClause,
    TsDeclareStatement, TsModuleDeclaration, TsPropertySignatureTypeMember,
    binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding},
};
use biome_rowan::{AstNode, SyntaxNodeOptionExt, TextRange, declare_node_union};
use biome_rule_options::no_invalid_use_before_declaration::NoInvalidUseBeforeDeclarationOptions;

declare_lint_rule! {
    /// Disallow the use of variables, function parameters, classes, and enums before their declaration
    ///
    /// JavaScript doesn't allow the use of block-scoped variables (`let`, `const`), function parameters, and classes before their declaration.
    /// Similarly TypeScript doesn't allow the use of enums before their declaration.
    /// A `ReferenceError` will be thrown with any attempt to access the variable or the parameter before its declaration.
    ///
    /// The rule also reports the use of variables declared with `var` before their declarations.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     console.log(x);
    ///     let x;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     console.log(x);
    ///     var x = 0;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(a = b, b = 0) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new C();
    /// class C {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// f();
    /// function f() {}
    /// ```
    ///
    /// ```js
    /// // An export can reference a variable before its declaration.
    /// export { CONSTANT };
    /// const CONSTANT = 0;
    /// ```
    ///
    /// ```js
    /// function f() { return CONSTANT; }
    /// const CONSTANT = 0;
    /// ```
    ///
    /// ```ts
    /// function f() {
    ///     new C();
    /// }
    /// let c: C;
    /// class C {}
    /// ```
    pub NoInvalidUseBeforeDeclaration {
        version: "1.5.0",
        name: "noInvalidUseBeforeDeclaration",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-use-before-define").same(),
            RuleSource::EslintTypeScript("no-use-before-define").same(),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoInvalidUseBeforeDeclaration {
    type Query = SemanticServices;
    type State = InvalidUseBeforeDeclaration;
    type Signals = Box<[Self::State]>;
    type Options = NoInvalidUseBeforeDeclarationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let mut result = vec![];
        let is_declaration_file = ctx
            .source_type::<JsFileSource>()
            .language()
            .is_definition_file();
        if is_declaration_file {
            return Box::default();
        }
        for binding in model.all_bindings() {
            let id = binding.tree();
            if matches!(
                id,
                AnyJsIdentifierBinding::TsIdentifierBinding(_)
                    | AnyJsIdentifierBinding::TsTypeParameterName(_)
            ) {
                // Ignore type declarations (interfaces, type-aliases, ...)
                continue;
            };
            let Some(declaration) = id.declaration() else {
                continue;
            };
            let Ok(declaration_kind) = DeclarationKind::try_from(&declaration) else {
                continue;
            };
            let declaration_end = if matches!(
                declaration_kind,
                DeclarationKind::Class | DeclarationKind::Enum
            ) {
                // A class can be instantiated by its properties.
                // Enum members can be qualified by the enum name.
                id.range().end()
            } else {
                declaration.range().end()
            };
            let declaration_scope = declaration
                .syntax()
                .ancestors()
                .skip(1)
                .find(|ancestor| AnyJsVariableScope::can_cast(ancestor.kind()));
            for reference in binding.all_references() {
                if reference.range_start() < declaration_end {
                    let reference_syntax = reference.syntax();
                    // References that are exports, such as `export { a }` are always valid,
                    // even when they appear before the declaration.
                    // For example:
                    //
                    // ```js
                    // export { X };
                    // const X = 0;
                    // ```
                    if reference_syntax
                        .parent()
                        .kind()
                        .filter(|parent_kind| AnyJsExportNamedSpecifier::can_cast(*parent_kind))
                        .is_none()
                        // Don't report variables used in another control flow root (function, classes, ...)
                        // For example:
                        //
                        // ```js
                        // function f() { X; }
                        // const X = 0;
                        // ```
                        && declaration_scope == reference_syntax
                                .ancestors()
                                .skip(1)
                                .find(|ancestor| AnyJsVariableScope::can_cast(ancestor.kind())
                        )
                        // ignore when used as a type.
                        // For example:
                        //
                        // ```js
                        // type Y = typeof X;
                        // const X = 0;
                        // ```
                        && !AnyJsIdentifierUsage::cast_ref(&reference_syntax)
                            .is_some_and(|usage| usage.is_only_type())
                    {
                        result.push(InvalidUseBeforeDeclaration {
                            declaration_kind,
                            reference_range: reference_syntax.text_trimmed_range(),
                            binding_range: id.range(),
                        });
                    }
                }
            }
        }
        result.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let InvalidUseBeforeDeclaration {
            declaration_kind,
            reference_range,
            binding_range: declaration_range,
        } = state;
        let declaration_kind_text = match declaration_kind {
            DeclarationKind::Class => "class",
            DeclarationKind::Enum => "enum",
            DeclarationKind::EnumMember => "enum member",
            DeclarationKind::Parameter => "parameter",
            DeclarationKind::Variable => "variable",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference_range,
                markup! { "This "{declaration_kind_text}" is used before its declaration." },
            )
            .detail(
                declaration_range,
                markup! { "The "{declaration_kind_text}" is declared here:" },
            ),
        )
    }
}

#[derive(Debug)]
pub struct InvalidUseBeforeDeclaration {
    declaration_kind: DeclarationKind,
    reference_range: TextRange,
    binding_range: TextRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeclarationKind {
    Class,
    Enum,
    EnumMember,
    Parameter,
    Variable,
}

impl TryFrom<&AnyJsBindingDeclaration> for DeclarationKind {
    type Error = ();

    fn try_from(value: &AnyJsBindingDeclaration) -> Result<Self, Self::Error> {
        match value {
            AnyJsBindingDeclaration::TsEnumMember(_) => Ok(Self::EnumMember),
            // Variable declaration
            AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
            | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
            | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => Ok(Self::Variable),
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                if let Some(var_decl) = declarator.declaration()
                    && let Some(var_decl_clause) = var_decl.parent::<JsVariableDeclarationClause>()
                    && var_decl_clause.parent::<TsDeclareStatement>().is_some()
                {
                    // Ambient variables, such as `declare const c;`,
                    // can be used before their declarations.
                    Err(())
                } else {
                    Ok(Self::Variable)
                }
            }
            // Parameters
            AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_)
            | AnyJsBindingDeclaration::TsPropertyParameter(_) => Ok(Self::Parameter),
            AnyJsBindingDeclaration::JsClassDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_) => {
                if value.parent::<TsDeclareStatement>().is_some() {
                    Err(())
                } else {
                    Ok(Self::Class)
                }
            }
            AnyJsBindingDeclaration::TsEnumDeclaration(_) => {
                if value.parent::<TsDeclareStatement>().is_some() {
                    Err(())
                } else {
                    Ok(Self::Enum)
                }
            }
            // Other declarations allow use before definition
            AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
            | AnyJsBindingDeclaration::JsBogusParameter(_)
            | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
            | AnyJsBindingDeclaration::TsInferType(_)
            | AnyJsBindingDeclaration::TsMappedType(_)
            | AnyJsBindingDeclaration::TsTypeParameter(_)
            | AnyJsBindingDeclaration::JsFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExpression(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExpression(_)
            | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
            | AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)
            | AnyJsBindingDeclaration::TsExternalModuleDeclaration(_)
            | AnyJsBindingDeclaration::TsModuleDeclaration(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::JsCatchDeclaration(_) => Err(()),
        }
    }
}

declare_node_union! {
    AnyJsVariableScope =
        JsScript
        | JsModule
        | AnyJsFunction
        | JsClassDeclaration
        | JsConstructorClassMember
        | JsGetterClassMember
        | JsGetterObjectMember
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsSetterClassMember
        | JsSetterObjectMember
        | JsStaticInitializationBlockClassMember
        | TsModuleDeclaration
        | TsPropertySignatureTypeMember
}
