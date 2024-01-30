use std::str::FromStr;

use crate::{
    control_flow::AnyJsControlFlowRoot,
    semantic_services::Semantic,
    utils::case::Case,
    utils::rename::{AnyJsRenamableDeclaration, RenameSymbolExtensions},
    JsRuleAction,
};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Applicability;
use biome_js_semantic::CanBeImportedExported;
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, inner_string_text, AnyJsClassMember, AnyJsObjectMember,
    AnyJsVariableDeclaration, AnyTsTypeMember, JsIdentifierBinding, JsLiteralExportName,
    JsLiteralMemberName, JsPrivateClassMemberName, JsSyntaxKind, JsSyntaxToken,
    JsVariableDeclarator, JsVariableKind, TsEnumMember, TsIdentifierBinding, TsTypeParameterName,
};
use biome_rowan::{
    declare_node_union, AstNode, AstNodeList, BatchMutationExt, SyntaxResult, TokenText,
};
use biome_unicode_table::is_js_ident;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Enforce naming conventions for everything across a codebase.
    ///
    /// Enforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent,
    /// and reduces overhead when thinking about the name [case] of a variable.
    ///
    /// ## Naming conventions
    ///
    /// All names can be prefixed and suffixed by underscores `_` and dollar signs `$`.
    ///
    /// ### Variable names
    ///
    /// All variables, including function parameters and catch parameters, are in [`camelCase`].
    ///
    /// Additionally, top-level variables declared as `const` or `var` may be in [`CONSTANT_CASE`] or [`PascalCase`].
    /// Top-level variables are declared at module or script level.
    /// Variables declared in a TypeScript `module` or `namespace` are also considered top-level.
    ///
    /// ```js
    /// function f(param, _unusedParam) {
    ///     let localValue = 0;
    ///     try {
    ///         /* ... */
    ///     } catch (customError) {
    ///         /* ... */
    ///     }
    /// }
    ///
    /// export const A_CONSTANT = 5;
    ///
    /// export const Person = class {}
    ///
    /// let aVariable = 0;
    ///
    /// export namespace ns {
    ///     export const ANOTHER_CONSTANT = "";
    /// }
    /// ```
    ///
    /// Examples of incorrect names:
    ///
    /// ```js,expect_diagnostic
    /// let a_value = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const fooYPosition = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(FirstParam) {}
    /// ```
    ///
    /// ### Function names
    ///
    /// A `function` name is in [`camelCase`] or [`PascalCase`].
    ///
    /// ```jsx
    /// function trimString(s) { /*...*/ }
    ///
    /// function Component() {
    ///     return <div></div>;
    /// }
    /// ```
    ///
    /// ### TypeScript `enum` names
    ///
    /// A _TypeScript_ `enum` name is in [`PascalCase`].
    ///
    /// `enum` members are by default in [`PascalCase`].
    /// However, you can configure the [case] of `enum` members.
    /// See [options](#options) for more details.
    ///
    /// ```ts
    /// enum Status {
    ///     Open,
    ///     Close,
    /// }
    /// ```
    ///
    /// ### Classes
    ///
    /// - A class name is in [`PascalCase`].
    ///
    /// - Static property and static getter names are in [`camelCase`] or [`CONSTANT_CASE`].
    ///
    /// - Class property and method names are in [`camelCase`].
    ///
    /// ```js
    /// class Person {
    ///     static MAX_FRIEND_COUNT = 256;
    ///
    ///     static get SPECIAL_PERSON_INSTANCE() { /*...*/ }
    ///
    ///     initializedProperty = 0;
    ///
    ///     specialMethod() {}
    /// }
    /// ```
    ///
    /// ### TypeScript `type` aliases and `interface`
    ///
    /// - A `type` alias or an interface name are in [`PascalCase`].
    ///
    /// - Property and method names in a type are in [`camelCase`].
    ///
    /// - `readonly` property and getter names can also be in [`CONSTANT_CASE`].
    ///
    /// ```ts
    /// type Named = {
    ///     readonly fullName: string;
    ///
    ///     specialMethod(): void;
    /// };
    ///
    /// interface Named {
    ///     readonly fullName: string;
    ///
    ///     specialMethod(): void;
    /// }
    ///
    /// interface PersonConstructor {
    ///     readonly MAX_FRIEND_COUNT: number;
    ///
    ///     get SPECIAL_PERSON_INSTANCE(): Person;
    ///
    ///     new(): Person;
    /// }
    /// ```
    ///
    /// Examples of an incorrect type alias:
    ///
    /// ```ts,expect_diagnostic
    /// type person = { fullName: string };
    /// ```
    ///
    /// ### Literal object property and method names
    ///
    /// Literal object property and method names are in [`camelCase`].
    ///
    /// ```js
    /// const alice = {
    ///     fullName: "Alice",
    /// }
    /// ```
    ///
    /// Example of an incorrect name:
    ///
    /// ```js,expect_diagnostic
    /// const alice = {
    ///     FULL_NAME: "Alice",
    /// }
    /// ```
    ///
    /// ### Imported and exported module aliases
    ///
    /// Imported and exported module aliases are in [`camelCase`] or [`PascalCase`].
    ///
    /// ```js
    /// import * as myLib from "my-lib";
    /// import * as Framework from "framework";
    ///
    /// export * as myLib from "my-lib";
    /// export * as Framework from "framework";
    /// ```
    ///
    /// `import` and `export` aliases are in [`camelCase`], [`PascalCase`], or [`CONSTANT_CASE`]:
    ///
    /// ```js
    /// import assert, {
    ///     deepStrictEqual as deepEqual,
    ///     AssertionError as AssertError
    /// } from "node:assert";
    /// ```
    ///
    /// Examples of an incorrect name:
    ///
    /// ```ts,expect_diagnostic
    /// import * as MY_LIB from "my-lib";
    /// ```
    ///
    /// ### TypeScript type parameter names
    ///
    /// A _TypeScript_ type parameter name is in [`PascalCase`].
    ///
    /// ```ts
    /// function id<Val>(value: Val): Val { /* ... */}
    /// ```
    ///
    /// ### TypeScript `namespace` names
    ///
    /// A _TypeScript_ `namespace` name is in [`camelCase`] or in [`PascalCase`].
    ///
    /// ```ts
    /// namespace mathExtra {
    ///     /*...*/
    /// }
    ///
    /// namespace MathExtra {
    ///     /*...*/
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides two options that are detailed in the following subsections.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "strictCase": false,
    ///         "enumMemberCase": "CONSTANT_CASE"
    ///     }
    /// }
    /// ```
    ///
    /// ### strictCase
    ///
    /// When this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`] and [`PascalCase`].
    /// For instance,  when the option is set to `true`, `HTTPServer` or `aHTTPServer` will throw an error.
    /// These names should be renamed to `HttpServer` and `aHttpServer`
    ///
    /// When the option is set to `false`, consecutive uppercase characters are allowed.
    /// `HTTPServer` and `aHTTPServer` are so valid.
    ///
    /// Default: `true`
    ///
    /// ### enumMemberCase
    ///
    /// By default, the rule enforces the naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):
    /// an `enum` member is in [`PascalCase`].
    ///
    /// You can enforce another convention by setting `enumMemberCase` option.
    /// The supported cases are: [`PascalCase`], [`CONSTANT_CASE`], and [`camelCase`].
    ///
    /// [case]: https://en.wikipedia.org/wiki/Naming_convention_(programming)#Examples_of_multiple-word_identifier_formats
    /// [`camelCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`PascalCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`CONSTANT_CASE`]: https://en.wikipedia.org/wiki/Snake_case
    pub(crate)  UseNamingConvention {
        version: "1.0.0",
        name: "useNamingConvention",
        source: RuleSource::EslintTypeScript("naming-convention"),
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNamingConvention {
    type Query = Semantic<AnyIdentifierBindingLike>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NamingConventionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let element = Named::from_name(node)?;
        let allowed_cases = element.allowed_cases(options);
        if allowed_cases.is_empty() {
            // No naming convention to verify.
            return None;
        }
        let name = node.name().ok()?;
        let name = name.text();
        if !is_js_ident(name) {
            // ignore non-identifier strings
            return None;
        }
        let trimmed_name = trim_underscore_dollar(name);
        let actual_case = Case::identify(trimmed_name, options.strict_case);
        if trimmed_name.is_empty()
            || allowed_cases
                .iter()
                .any(|&expected_style| actual_case.is_compatible_with(expected_style))
        {
            // Valid case
            return None;
        }
        let preferred_case = element.allowed_cases(ctx.options())[0];
        let new_trimmed_name = preferred_case.convert(trimmed_name);
        let suggested_name = name.replace(trimmed_name, &new_trimmed_name);
        Some(State {
            element,
            suggested_name,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let State {
            element,
            suggested_name,
        } = state;
        let options = ctx.options();
        let name = ctx.query().name().ok()?;
        let name = name.text();
        let trimmed_name = trim_underscore_dollar(name);
        let allowed_cases = element.allowed_cases(ctx.options());
        let allowed_case_names = allowed_cases
            .iter()
            .map(|style| style.to_string())
            .collect::<SmallVec<[_; 3]>>()
            .join(" or ");
        let trimmed_info = if name != trimmed_name {
            markup! {" trimmed as `"{trimmed_name}"`"}.to_owned()
        } else {
            markup! {""}.to_owned()
        };

        if options.strict_case {
            let case_type = Case::identify(name, false);
            let case_strict = Case::identify(name, true);
            if case_type == Case::Camel && case_strict == Case::Unknown {
                return Some(RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().syntax().text_trimmed_range(),
                    markup! {
                        "Two consecutive uppercase characters are not allowed in camelCase and PascalCase because `strictCase` is set to `true`."
                    },
                ).note(markup! {
                    "If you want to use consecutive uppercase characters in camelCase and PascalCase then consider setting `strictCase` option to `false`.\n Check rule "<Hyperlink href="https://biomejs.dev/linter/rules/use-naming-convention#options">"options"</Hyperlink>" for more inforamtion."
                }));
            }
        }

        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().syntax().text_trimmed_range(),
            markup! {
                "This "<Emphasis>{element.to_string()}</Emphasis>" name"{trimmed_info}" should be in "<Emphasis>{allowed_case_names}</Emphasis>"."
            },
        ).note(markup! {
            "The name could be renamed to `"{suggested_name}"`."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let model = ctx.model();
        let mut mutation = ctx.root().begin();
        let State {
            element,
            suggested_name,
        } = state;
        let renamable = match node {
            AnyIdentifierBindingLike::JsIdentifierBinding(binding) => {
                if binding.is_exported(model) {
                    return None;
                }
                if let Some(AnyJsBindingDeclaration::TsPropertyParameter(_)) = binding.declaration()
                {
                    // Property parameters are also class properties.
                    return None;
                }
                Some(AnyJsRenamableDeclaration::JsIdentifierBinding(
                    binding.clone(),
                ))
            }
            AnyIdentifierBindingLike::TsIdentifierBinding(binding) => {
                if binding.is_exported(model) {
                    return None;
                }
                Some(AnyJsRenamableDeclaration::TsIdentifierBinding(
                    binding.clone(),
                ))
            }
            _ => None,
        };
        if let Some(renamable) = renamable {
            let preferred_case = element.allowed_cases(ctx.options())[0];
            let renamed = mutation.rename_any_renamable_node(model, renamable, &suggested_name[..]);
            if renamed {
                return Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! { "Rename this symbol in "<Emphasis>{preferred_case.to_string()}</Emphasis>"." }.to_owned(),
                    mutation,
                });
            }
        }
        None
    }
}

declare_node_union! {
    /// Ast nodes that defines a name.
    pub(crate) AnyIdentifierBindingLike =
        JsIdentifierBinding |
        JsLiteralMemberName |
        JsPrivateClassMemberName |
        JsLiteralExportName |
        TsIdentifierBinding |
        TsTypeParameterName
}

impl AnyIdentifierBindingLike {
    fn name_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyIdentifierBindingLike::JsIdentifierBinding(binding) => binding.name_token(),
            AnyIdentifierBindingLike::JsLiteralMemberName(member_name) => member_name.value(),
            AnyIdentifierBindingLike::JsPrivateClassMemberName(member_name) => {
                member_name.id_token()
            }
            AnyIdentifierBindingLike::JsLiteralExportName(export_name) => export_name.value(),
            AnyIdentifierBindingLike::TsIdentifierBinding(binding) => binding.name_token(),
            AnyIdentifierBindingLike::TsTypeParameterName(type_parameter) => {
                type_parameter.ident_token()
            }
        }
    }

    fn name(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.name_token()?))
    }
}

#[derive(Debug)]
pub(crate) struct State {
    element: Named,
    suggested_name: String,
}

/// Rule's options.
#[derive(Debug, Clone, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamingConventionOptions {
    /// If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
    /// This does not affect other [Case].
    #[serde(
        default = "default_strict_case",
        skip_serializing_if = "is_default_strict_case"
    )]
    pub strict_case: bool,

    /// Allowed cases for _TypeScript_ `enum` member names.
    #[serde(default, skip_serializing_if = "is_default")]
    pub enum_member_case: EnumMemberCase,
}

const fn default_strict_case() -> bool {
    true
}

const fn is_default_strict_case(strict_case: &bool) -> bool {
    *strict_case == default_strict_case()
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

impl Default for NamingConventionOptions {
    fn default() -> Self {
        Self {
            strict_case: default_strict_case(),
            enum_member_case: EnumMemberCase::default(),
        }
    }
}

/// Supported cases for TypeScript `enum` member names.
#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum EnumMemberCase {
    /// PascalCase
    #[serde(rename = "PascalCase")]
    #[default]
    Pascal,

    /// CONSTANT_CASE
    #[serde(rename = "CONSTANT_CASE")]
    Constant,

    /// camelCase
    #[serde(rename = "camelCase")]
    Camel,
}

impl FromStr for EnumMemberCase {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PascalCase" => Ok(Self::Pascal),
            "CONSTANT_CASE" => Ok(Self::Constant),
            "camelCase" => Ok(Self::Camel),
            _ => Err("Value not supported for enum member case"),
        }
    }
}

impl From<EnumMemberCase> for Case {
    fn from(case: EnumMemberCase) -> Case {
        match case {
            EnumMemberCase::Pascal => Case::Pascal,
            EnumMemberCase::Constant => Case::Constant,
            EnumMemberCase::Camel => Case::Camel,
        }
    }
}

/// Named elements with an attached naming convention.
///
/// [Named::from_name] enables to get the element from an [AnyName].
/// [Named::allowed_cases] enables to get a list of allowed cases for a given element.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Named {
    CatchParameter,
    Class,
    ClassGetter,
    ClassMethod,
    ClassProperty,
    ClassSetter,
    ClassStaticGetter,
    ClassStaticMethod,
    ClassStaticProperty,
    ClassStaticSetter,
    Enum,
    EnumMember,
    ExportAlias,
    ExportNamespace,
    ExportSource,
    Function,
    FunctionParameter,
    ImportAlias,
    ImportNamespace,
    ImportSource,
    IndexParameter,
    Interface,
    LocalConst,
    LocalLet,
    LocalVar,
    LocalUsing,
    Namespace,
    ObjectGetter,
    ObjectMethod,
    ObjectProperty,
    ObjectSetter,
    ParameterProperty,
    TopLevelConst,
    TopLevelLet,
    TopLevelVar,
    TypeAlias,
    TypeGetter,
    TypeMethod,
    TypeProperty,
    TypeReadonlyProperty,
    TypeSetter,
    TypeParameter,
}

impl Named {
    fn from_name(js_name: &AnyIdentifierBindingLike) -> Option<Named> {
        match js_name {
            AnyIdentifierBindingLike::JsIdentifierBinding(binding) => {
                Named::from_binding_declaration(&binding.declaration()?)
            }
            AnyIdentifierBindingLike::TsIdentifierBinding(binding) => {
                Named::from_binding_declaration(&binding.declaration()?)
            }
            AnyIdentifierBindingLike::JsLiteralMemberName(member_name) => {
                if let Some(member) = member_name.parent::<AnyJsClassMember>() {
                    Named::from_class_member(&member)
                } else if let Some(member) = member_name.parent::<AnyTsTypeMember>() {
                    Named::from_type_member(&member)
                } else if let Some(member) = member_name.parent::<AnyJsObjectMember>() {
                    Named::from_object_member(&member)
                } else if member_name.parent::<TsEnumMember>().is_some() {
                    Some(Named::EnumMember)
                } else {
                    None
                }
            }
            AnyIdentifierBindingLike::JsPrivateClassMemberName(member_name) => {
                Named::from_class_member(&member_name.parent::<AnyJsClassMember>()?)
            }
            AnyIdentifierBindingLike::JsLiteralExportName(export_name) => {
                let parent = export_name.syntax().parent()?;
                match parent.kind() {
                    JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => Some(Named::ImportSource),
                    JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => Some(Named::ExportSource),
                    JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => Some(Named::ExportAlias),
                    JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                        if parent.parent()?.kind() == JsSyntaxKind::JS_EXPORT_FROM_CLAUSE {
                            Some(Named::ExportNamespace)
                        } else {
                            Some(Named::ExportAlias)
                        }
                    }
                    _ => None,
                }
            }
            AnyIdentifierBindingLike::TsTypeParameterName(_) => Some(Named::TypeParameter),
        }
    }

    fn from_class_member(member: &AnyJsClassMember) -> Option<Named> {
        match member {
            AnyJsClassMember::JsBogusMember(_)
            | AnyJsClassMember::JsConstructorClassMember(_)
            | AnyJsClassMember::TsConstructorSignatureClassMember(_)
            | AnyJsClassMember::JsEmptyClassMember(_)
            | AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => None,
            AnyJsClassMember::TsIndexSignatureClassMember(_) => Some(Named::IndexParameter),
            AnyJsClassMember::JsGetterClassMember(getter) => {
                let is_static = getter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticGetter
                } else {
                    Named::ClassGetter
                })
            }
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
                let is_static = getter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticGetter
                } else {
                    Named::ClassGetter
                })
            }
            AnyJsClassMember::JsMethodClassMember(method) => {
                let is_static = method
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticMethod
                } else {
                    Named::ClassMethod
                })
            }
            AnyJsClassMember::TsMethodSignatureClassMember(method) => {
                let is_static = method
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticMethod
                } else {
                    Named::ClassMethod
                })
            }
            AnyJsClassMember::JsPropertyClassMember(property) => {
                let is_static = property
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::TsPropertySignatureClassMember(property) => {
                let is_static = property
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
                let is_static = property
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::JsSetterClassMember(setter) => {
                let is_static = setter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticSetter
                } else {
                    Named::ClassSetter
                })
            }
            AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
                let is_static = setter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticSetter
                } else {
                    Named::ClassSetter
                })
            }
        }
    }

    fn from_binding_declaration(decl: &AnyJsBindingDeclaration) -> Option<Named> {
        match decl {
            AnyJsBindingDeclaration::JsVariableDeclarator(var) => {
                Named::from_variable_declarator(var)
            }
            AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
            | AnyJsBindingDeclaration::JsBogusParameter(_)
            | AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_) => Some(Named::FunctionParameter),
            AnyJsBindingDeclaration::JsCatchDeclaration(_) => Some(Named::CatchParameter),
            AnyJsBindingDeclaration::TsPropertyParameter(_) => Some(Named::ParameterProperty),
            AnyJsBindingDeclaration::TsIndexSignatureParameter(_) => Some(Named::IndexParameter),
            AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => Some(Named::ImportNamespace),
            AnyJsBindingDeclaration::JsFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExpression(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
                Some(Named::Function)
            }
            AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => Some(Named::ImportAlias),
            AnyJsBindingDeclaration::TsModuleDeclaration(_) => Some(Named::Namespace),
            AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) => Some(Named::TypeAlias),
            AnyJsBindingDeclaration::JsClassDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExpression(_)
            | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_) => Some(Named::Class),
            AnyJsBindingDeclaration::TsInterfaceDeclaration(_) => Some(Named::Interface),
            AnyJsBindingDeclaration::TsEnumDeclaration(_) => Some(Named::Enum),
            AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => {
                Some(Named::ImportSource)
            }
            AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
            // Type parameters should be handled at call site
            | AnyJsBindingDeclaration::TsInferType(_)
            | AnyJsBindingDeclaration::TsMappedType(_)
            | AnyJsBindingDeclaration::TsTypeParameter(_) => None,
        }
    }

    fn from_variable_declarator(var: &JsVariableDeclarator) -> Option<Named> {
        let is_top_level_level = var
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .is_some_and(|x| {
                matches!(
                    x.kind(),
                    JsSyntaxKind::JS_MODULE
                        | JsSyntaxKind::JS_SCRIPT
                        | JsSyntaxKind::TS_MODULE_DECLARATION
                        | JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION
                )
            });
        let var_declaration = var
            .syntax()
            .ancestors()
            .find_map(AnyJsVariableDeclaration::cast)?;
        let var_kind = var_declaration.variable_kind().ok()?;
        Some(match (var_kind, is_top_level_level) {
            (JsVariableKind::Const, false) => Named::LocalConst,
            (JsVariableKind::Let, false) => Named::LocalLet,
            (JsVariableKind::Var, false) => Named::LocalVar,
            (JsVariableKind::Using, false) => Named::LocalUsing,
            (JsVariableKind::Const, true) => Named::TopLevelConst,
            (JsVariableKind::Let, true) => Named::TopLevelLet,
            (JsVariableKind::Var, true) => Named::TopLevelVar,
            (JsVariableKind::Using, true) => Named::LocalUsing,
        })
    }

    fn from_object_member(member: &AnyJsObjectMember) -> Option<Named> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) | AnyJsObjectMember::JsSpread(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(_) => Some(Named::ObjectGetter),
            AnyJsObjectMember::JsMethodObjectMember(_) => Some(Named::ObjectMethod),
            AnyJsObjectMember::JsPropertyObjectMember(_)
            | AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => Some(Named::ObjectProperty),
            AnyJsObjectMember::JsSetterObjectMember(_) => Some(Named::ObjectSetter),
        }
    }

    fn from_type_member(member: &AnyTsTypeMember) -> Option<Named> {
        match member {
            AnyTsTypeMember::JsBogusMember(_)
            | AnyTsTypeMember::TsCallSignatureTypeMember(_)
            | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => None,
            AnyTsTypeMember::TsIndexSignatureTypeMember(_) => Some(Named::IndexParameter),
            AnyTsTypeMember::TsGetterSignatureTypeMember(_) => Some(Named::TypeGetter),
            AnyTsTypeMember::TsMethodSignatureTypeMember(_) => Some(Named::TypeMethod),
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                Some(if property.readonly_token().is_some() {
                    Named::TypeReadonlyProperty
                } else {
                    Named::TypeProperty
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_) => Some(Named::TypeSetter),
        }
    }

    /// Returns the list of allowed [Case] for `self`.
    /// The preferred case comes first in the list.
    fn allowed_cases(self, options: &NamingConventionOptions) -> SmallVec<[Case; 3]> {
        match self {
            Named::CatchParameter
            | Named::ClassGetter
            | Named::ClassMethod
            | Named::ClassProperty
            | Named::ClassSetter
            | Named::ClassStaticMethod
            | Named::ClassStaticSetter
            | Named::FunctionParameter
            | Named::IndexParameter
            | Named::LocalConst
            | Named::LocalLet
            | Named::LocalVar
            | Named::LocalUsing
            | Named::ObjectGetter
            | Named::ObjectMethod
            | Named::ObjectProperty
            | Named::ObjectSetter
            | Named::ParameterProperty
            | Named::TopLevelLet
            | Named::TypeMethod
            | Named::TypeProperty
            | Named::TypeSetter => SmallVec::from_slice(&[Case::Camel]),
            Named::Class
            | Named::Enum
            | Named::Interface
            | Named::TypeAlias
            | Named::TypeParameter => SmallVec::from_slice(&[Case::Pascal]),
            Named::ClassStaticGetter
            | Named::ClassStaticProperty
            | Named::TypeReadonlyProperty
            | Named::TypeGetter => SmallVec::from_slice(&[Case::Camel, Case::Constant]),
            Named::EnumMember => SmallVec::from_slice(&[options.enum_member_case.into()]),
            Named::ExportAlias | Named::ImportAlias | Named::TopLevelConst | Named::TopLevelVar => {
                SmallVec::from_slice(&[Case::Camel, Case::Pascal, Case::Constant])
            }
            Named::ExportSource | Named::ImportSource => SmallVec::new(),
            Named::ExportNamespace
            | Named::Function
            | Named::ImportNamespace
            | Named::Namespace => SmallVec::from_slice(&[Case::Camel, Case::Pascal]),
        }
    }
}

impl std::fmt::Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Named::CatchParameter => "catch parameter",
            Named::Class => "class",
            Named::ClassGetter => "class getter",
            Named::ClassMethod => "class method",
            Named::ClassProperty => "class property",
            Named::ClassSetter => "class setter",
            Named::ClassStaticGetter => "static getter",
            Named::ClassStaticMethod => "static method",
            Named::ClassStaticProperty => "static property",
            Named::ClassStaticSetter => "static setter",
            Named::Enum => "enum",
            Named::EnumMember => "enum member",
            Named::ExportAlias => "export alias",
            Named::ExportNamespace => "export namespace",
            Named::ExportSource => "export source",
            Named::Function => "function",
            Named::FunctionParameter => "function parameter",
            Named::ImportAlias => "import alias",
            Named::ImportNamespace => "import namespace",
            Named::ImportSource => "import source",
            Named::IndexParameter => "index parameter",
            Named::Interface => "interface",
            Named::LocalConst => "local const",
            Named::LocalLet => "local let",
            Named::LocalVar => "local var",
            Named::LocalUsing => "local using",
            Named::Namespace => "namespace",
            Named::ObjectGetter => "object getter",
            Named::ObjectMethod => "object method",
            Named::ObjectProperty => "object property",
            Named::ObjectSetter => "object setter",
            Named::ParameterProperty => "parameter property",
            Named::TopLevelConst => "top-level const",
            Named::TopLevelLet => "top-level let",
            Named::TopLevelVar => "top-level var",
            Named::TypeAlias => "type alias",
            Named::TypeGetter => "getter",
            Named::TypeMethod => "method",
            Named::TypeProperty => "property",
            Named::TypeReadonlyProperty => "readonly property",
            Named::TypeSetter => "setter",
            Named::TypeParameter => "type parameter",
        };
        write!(f, "{}", repr)
    }
}

/// trim underscores and dollar signs from `name`.
fn trim_underscore_dollar(name: &str) -> &str {
    name.trim_start_matches(|c| c == '_' || c == '$')
        .trim_end_matches(|c| c == '_' || c == '$')
}
