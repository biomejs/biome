use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExportNamedSpecifier, AnyJsFormalParameter, AnyJsFunction, AnyJsIdentifierReference,
    AnyJsPropertyModifier, JsConstructorParameters, JsFunctionBody, JsInitializerClause, JsModule,
    JsParameters, JsPropertyClassMember, JsScript, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclarationClause, TsDeclareStatement, TsModuleDeclaration,
    TsPropertySignatureTypeMember,
    binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding},
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, SyntaxNodeOptionExt, TextRange, declare_node_union};
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
    type Query = Semantic<AnyJsIdentifierBinding>;
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
        let id = ctx.query();
        if matches!(
            id,
            AnyJsIdentifierBinding::TsIdentifierBinding(_)
                | AnyJsIdentifierBinding::TsTypeParameterName(_)
        ) {
            // Ignore type declarations (interfaces, type-aliases, ...)
            return Box::default();
        };
        let Some(declaration) = id.declaration() else {
            return Box::default();
        };
        let Ok(declaration_kind) = DeclarationKind::try_from(&declaration) else {
            return Box::default();
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
        let declaration_scope = deferred_scope(declaration.syntax());
        let binding = model.as_binding(id);
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
                        .kind().as_ref().is_none_or(|parent_kind| !AnyJsExportNamedSpecifier::can_cast(*parent_kind))
                        // skip uses that only run later, e.g. `function f() { X; } const X = 0;`
                        && declaration_scope == deferred_scope(&reference_syntax)
                        // ignore when used as a type.
                        // For example:
                        //
                        // ```js
                        // type Y = typeof X;
                        // const X = 0;
                        // ```
                    && !AnyJsIdentifierReference::cast_ref(&reference_syntax)
                        .is_some_and(|reference| reference.is_only_type())
                {
                    result.push(InvalidUseBeforeDeclaration {
                        declaration_kind,
                        reference_range: reference_syntax.text_trimmed_range(),
                        binding_range: id.range(),
                    });
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
            .note(markup! {
                "Using a "{declaration_kind_text}" before it is declared makes the code depend on declaration order and hoisting behavior."
            })
            .detail(
                declaration_range,
                markup! { "The "{declaration_kind_text}" is declared here:" },
            )
            .note(markup! {
                "Move this use after the declaration, or move the declaration earlier."
            }),
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
    /// nodes whose code doesn't run together with the code around them
    AnyJsDeferredScope =
        JsScript
        | JsModule
        | AnyJsFunction
        | JsFunctionBody
        | JsParameters
        | JsConstructorParameters
        | TsModuleDeclaration
        | TsPropertySignatureTypeMember
}

/// closest ancestor whose code runs later than its surroundings, if any
fn deferred_scope(node: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    node.ancestors().skip(1).find(is_deferred)
}

fn is_deferred(node: &JsSyntaxNode) -> bool {
    if AnyJsDeferredScope::can_cast(node.kind()) {
        return true;
    }
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        // a setter's lone param isn't wrapped in JsParameters
        JsSyntaxKind::JS_SETTER_CLASS_MEMBER | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
            AnyJsFormalParameter::can_cast(node.kind())
        }
        // instance field initializers only run on `new`, static ones run with the class body
        JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
            JsInitializerClause::can_cast(node.kind()) && !is_static_field(&parent)
        }
        _ => false,
    }
}

fn is_static_field(node: &JsSyntaxNode) -> bool {
    JsPropertyClassMember::cast_ref(node).is_some_and(|field| {
        field
            .modifiers()
            .iter()
            .any(|modifier| matches!(modifier, AnyJsPropertyModifier::JsStaticModifier(_)))
    })
}
