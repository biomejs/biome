use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMemberName, AnyJsExportNamedSpecifier, AnyJsFunction, AnyJsIdentifierReference,
    AnyJsPropertyModifier,
    JsClassDeclaration, JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember,
    JsLanguage, JsMethodClassMember, JsMethodObjectMember, JsModule, JsPropertyClassMember, JsScript,
    JsSetterClassMember, JsSetterObjectMember, JsStaticInitializationBlockClassMember,
    JsVariableDeclarationClause,
    TsDeclareStatement, TsModuleDeclaration, TsPropertySignatureTypeMember,
    binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding},
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, SyntaxNode, SyntaxNodeOptionExt, TextRange, declare_node_union};
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
        let declaration_scope = declaration
            .syntax()
            .ancestors()
            .skip(1)
            .find(|ancestor| AnyJsVariableScope::can_cast(ancestor.kind()));
        let binding = model.as_binding(id);
        for reference in binding.all_references() {
            let reference_syntax = reference.syntax();
            // A class definition evaluates its heritage clause, its computed member
            // names, its static field initializers and its static blocks before it
            // initializes the class binding, so a reference to a binding declared
            // later in the enclosing scope is a use before the declaration even when
            // it follows the class name. A reference to the class being defined is
            // already covered by `declaration_end`.
            let is_eager_class_position = matches!(declaration_kind, DeclarationKind::Class)
                && AnyJsIdentifierReference::cast_ref(&reference_syntax).is_some_and(|reference| {
                    reference_syntax
                        .ancestors()
                        .skip(1)
                        .any(|ancestor| is_class_eager_position(&reference, &ancestor, id))
                });
            if reference.range_start() < declaration_end || is_eager_class_position {
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
                        // Don't report variables used in another control flow root (function, classes, ...)
                        // For example:
                        //
                        // ```js
                        // function f() { X; }
                        // const X = 0;
                        // ```
                        //
                        // A class body is a control flow root, so an instance field
                        // initializer or a member body keeps the exemption even when it
                        // refers to a binding declared later.
                        && (declaration_scope
                            == reference_syntax
                                .ancestors()
                                .skip(1)
                                .find(|ancestor| {
                                    AnyJsVariableScope::can_cast(ancestor.kind())
                                })
                            || is_eager_class_position)
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

/// Reports whether `reference` sits in a position that a class definition
/// evaluates eagerly, before it initializes the binding of the class.
///
/// The heritage clause and every computed member name are evaluated while the
/// class definition is created, and a static field initializer or static block
/// is evaluated before the class binding becomes available. A reference in one
/// of these positions runs in the scope that contains the class, so a use of a
/// binding declared later in that scope throws a `ReferenceError`.
///
/// Instance field initializers and the bodies of methods, accessors and
/// constructors are deferred, so a reference there does not depend on the class
/// definition order and keeps the control flow root exemption.
fn is_class_eager_position(
    reference: &AnyJsIdentifierReference,
    scope: &SyntaxNode<JsLanguage>,
    declared_id: &AnyJsIdentifierBinding,
) -> bool {
    let reference_range = reference.syntax().text_trimmed_range();

    // A reference to the class being defined does not depend on the declaration
    // order: `class Class { static SINGLETON = new Class(); }` runs after the class
    // binding is initialized. The range check in the caller already covers the
    // references that appear before the class name.
    if !is_reference_to_class_member(reference, declared_id) {
        // A static block runs while the class definition is being created.
        if JsStaticInitializationBlockClassMember::cast_ref(scope)
            .is_some_and(|block| contains_range(block.syntax(), reference_range))
        {
            return true;
        }

        // A static field initializer runs while the class definition is being
        // created, while an instance field initializer runs when an instance is
        // constructed.
        if let Some(property) = JsPropertyClassMember::cast_ref(scope)
            && is_static_property(&property)
            && let Some(value) = property.value()
            && contains_range(value.syntax(), reference_range)
        {
            return true;
        }
    }

    let Some(class) = JsClassDeclaration::cast_ref(scope) else {
        return false;
    };

    // The heritage clause runs before the class body is evaluated.
    if class
        .extends_clause()
        .is_some_and(|heritage| contains_range(heritage.syntax(), reference_range))
    {
        return true;
    }

    // A computed member name runs while the class definition is created.
    class
        .members()
        .iter()
        .filter_map(|member| member.name().ok().flatten())
        .filter_map(|name| match name {
            AnyJsClassMemberName::JsComputedMemberName(computed) => Some(computed),
            _ => None,
        })
        .filter_map(|computed| computed.expression().ok())
        .any(|expression| contains_range(expression.syntax(), reference_range))
}

/// Returns whether `reference` belongs to the class declared by `declared_id`.
fn is_reference_to_class_member(
    reference: &AnyJsIdentifierReference,
    declared_id: &AnyJsIdentifierBinding,
) -> bool {
    reference
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(|ancestor| JsClassDeclaration::cast_ref(&ancestor))
        .and_then(|class| class.id().ok())
        .is_some_and(|id| id.syntax() == declared_id.syntax())
}

/// Returns whether `property` is declared with the `static` modifier.
fn is_static_property(property: &JsPropertyClassMember) -> bool {
    property
        .modifiers()
        .iter()
        .any(|modifier| matches!(modifier, AnyJsPropertyModifier::JsStaticModifier(_)))
}

/// Returns whether `outer` contains all of `inner`.
fn contains_range(outer: &SyntaxNode<JsLanguage>, inner: TextRange) -> bool {
    outer.text_trimmed_range().contains_range(inner)
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
        | TsModuleDeclaration
        | TsPropertySignatureTypeMember
}
