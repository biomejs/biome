use crate::{services::control_flow::AnyJsControlFlowRoot, services::semantic::SemanticServices};
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding},
    AnyJsExportNamedSpecifier, AnyJsIdentifierUsage,
};
use biome_rowan::{AstNode, SyntaxNodeOptionExt, TextRange};

declare_lint_rule! {
    /// Disallow the use of variables and function parameters before their declaration
    ///
    /// JavaScript doesn't allow the use of block-scoped variables (`let`, `const`) and function parameters before their declaration.
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
    ///     const x;
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
    /// ### Valid
    ///
    /// ```js
    /// f();
    /// function f() {}
    ///
    /// new C();
    /// class C {}
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
    pub NoInvalidUseBeforeDeclaration {
        version: "1.5.0",
        name: "noInvalidUseBeforeDeclaration",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-use-before-define"),
            RuleSource::EslintTypeScript("no-use-before-define"),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoInvalidUseBeforeDeclaration {
    type Query = SemanticServices;
    type State = InvalidUseBeforeDeclaration;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let mut result = vec![];
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
            let declaration_end = declaration.range().end();
            let declaration_control_flow_root =
                if let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration
                    .parent_binding_pattern_declaration()
                    .unwrap_or(declaration)
                {
                    declarator
                        .syntax()
                        .ancestors()
                        .skip(1)
                        .find(|ancestor| AnyJsControlFlowRoot::can_cast(ancestor.kind()))
                } else {
                    None
                };
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
                        && (declaration_control_flow_root.is_none() ||
                            declaration_control_flow_root == reference_syntax
                                .ancestors()
                                .skip(1)
                                .find(|ancestor| AnyJsControlFlowRoot::can_cast(ancestor.kind()))
                        )
                        // ignore when used as a type.
                        // For example:
                        //
                        // ```js
                        // type Y = typeof X;
                        // const X = 0;
                        // ```
                        && !AnyJsIdentifierUsage::cast_ref(reference_syntax)
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

#[derive(Debug, Copy, Clone)]
pub enum DeclarationKind {
    EnumMember,
    Parameter,
    Variable,
}

impl TryFrom<&AnyJsBindingDeclaration> for DeclarationKind {
    type Error = ();

    fn try_from(value: &AnyJsBindingDeclaration) -> Result<Self, Self::Error> {
        match value {
            AnyJsBindingDeclaration::TsEnumMember(_) => Ok(DeclarationKind::EnumMember),
            // Variable declaration
            AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
            | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_)
            | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_)
            | AnyJsBindingDeclaration::JsVariableDeclarator(_) => Ok(DeclarationKind::Variable),
            // Parameters
            AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_)
            | AnyJsBindingDeclaration::TsPropertyParameter(_) => Ok(DeclarationKind::Parameter),
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
            | AnyJsBindingDeclaration::JsClassDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExpression(_)
            | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
            | AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)
            | AnyJsBindingDeclaration::TsEnumDeclaration(_)
            | AnyJsBindingDeclaration::TsModuleDeclaration(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
            | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::JsCatchDeclaration(_) => Err(()),
        }
    }
}
