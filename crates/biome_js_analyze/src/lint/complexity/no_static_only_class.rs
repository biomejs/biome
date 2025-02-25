use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClass, AnyJsClassMember, JsGetterClassMember, JsMethodClassMember, JsPropertyClassMember,
    JsSetterClassMember, TsGetterSignatureClassMember, TsIndexSignatureClassMember,
    TsInitializedPropertySignatureClassMember, TsMethodSignatureClassMember,
    TsPropertySignatureClassMember, TsSetterSignatureClassMember,
};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
    ///
    /// Users who come from a [OOP](https://en.wikipedia.org/wiki/Object-oriented_programming) paradigm may wrap their utility functions in an extra class,
    /// instead of putting them at the top level of an ECMAScript module. Doing so is generally unnecessary in JavaScript and TypeScript projects.
    ///
    /// - Wrapper classes add extra cognitive complexity to code without adding any structural improvements
    /// 	- Whatever would be put on them, such as utility functions, are already organized by virtue of being in a module.
    /// 	- As an alternative, you can import * as ... the module to get all of them in a single object.
    /// - IDEs can't provide as good suggestions for static class or namespace imported properties when you start typing property names
    /// - It's more difficult to statically analyze code for unused variables, etc. when they're all on the class (see: [Finding dead code (and dead types) in TypeScript](https://effectivetypescript.com/2020/10/20/tsprune)).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class X {
    ///   static foo = false;
    ///   static bar() {};
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// class StaticConstants {
    ///   static readonly version = 42;
    ///
    ///   static isProduction() {
    ///     return process.env.NODE_ENV === 'production';
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const X = {
    ///   foo: false,
    ///   bar() {}
    /// };
    /// ```
    /// ```js
    /// export const version = 42;
    ///
    /// export function isProduction() {
    ///   return process.env.NODE_ENV === 'production';
    /// }
    ///
    /// function logHelloWorld() {
    ///   console.log('Hello, world!');
    /// }
    /// ```
    /// ```js
    /// class Empty {}
    /// ```
    ///
    /// ## Notes on Mutating Variables
    /// One case you need to be careful of is exporting mutable variables. While class properties can be mutated externally, exported variables are always constant. This means that importers can only ever read the first value they are assigned and cannot write to the variables.
    ///
    /// Needing to write to an exported variable is very rare and is generally considered a code smell. If you do need it you can accomplish it using getter and setter functions:
    /// ```js,expect_diagnostic
    /// export class Utilities {
    ///   static mutableCount = 1;
    ///   static incrementCount() {
    ///     Utilities.mutableCount += 1;
    ///   }
    /// }
    /// ```
    ///
    /// Do this instead:
    /// ```js
    /// let mutableCount = 1;
    ///
    /// export function getMutableCount() {
    ///   return mutableField;
    /// }
    ///
    /// export function incrementCount() {
    ///   mutableField += 1;
    /// }
    /// ```
    pub NoStaticOnlyClass {
        version: "1.0.0",
        name: "noStaticOnlyClass",
        language: "js",
        sources: &[
            RuleSource::EslintTypeScript("no-extraneous-class"),
            RuleSource::EslintUnicorn("no-static-only-class"),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

trait HasStaticModifier {
    fn has_static_modifier(&self) -> bool;
}

macro_rules! impl_has_static_modifiers {
    ($type:ty) => {
        impl HasStaticModifier for $type {
            fn has_static_modifier(&self) -> bool {
                self.modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some())
            }
        }
    };
}

impl_has_static_modifiers!(JsGetterClassMember);
impl_has_static_modifiers!(JsSetterClassMember);
impl_has_static_modifiers!(JsMethodClassMember);
impl_has_static_modifiers!(JsPropertyClassMember);
impl_has_static_modifiers!(TsGetterSignatureClassMember);
impl_has_static_modifiers!(TsIndexSignatureClassMember);
impl_has_static_modifiers!(TsInitializedPropertySignatureClassMember);
impl_has_static_modifiers!(TsMethodSignatureClassMember);
impl_has_static_modifiers!(TsPropertySignatureClassMember);
impl_has_static_modifiers!(TsSetterSignatureClassMember);

impl Rule for NoStaticOnlyClass {
    type Query = Ast<AnyJsClass>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class_declaration = ctx.query();

        // If the class is empty, we can't be sure that the user can use a module-based approach instead
        // Also, the user might be in the process of writing a class, so we won't warn them yet
        if class_declaration.members().is_empty() {
            return None;
        }

        // If the class has decorators, we can't be sure that the user can use a module-based approach instead
        if !class_declaration.decorators().is_empty() {
            return None;
        }

        if class_declaration.extends_clause().is_some() {
            return None;
        }

        let all_members_static = class_declaration
            .members()
            .iter()
            .filter_map(|member| match member {
                AnyJsClassMember::JsBogusMember(_)
                | AnyJsClassMember::JsMetavariable(_)
                | AnyJsClassMember::JsEmptyClassMember(_) => None,
                AnyJsClassMember::JsConstructorClassMember(_) => Some(false), // See GH#4482: Constructors are not regarded as static
                AnyJsClassMember::TsConstructorSignatureClassMember(_) => Some(false), // See GH#4482: Constructors are not regarded as static
                AnyJsClassMember::JsGetterClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::JsMethodClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::JsPropertyClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::JsSetterClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => Some(true), // See GH#4482: Static initialization blocks are regarded as static
                AnyJsClassMember::TsGetterSignatureClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::TsIndexSignatureClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::TsInitializedPropertySignatureClassMember(m) => {
                    Some(m.has_static_modifier())
                }
                AnyJsClassMember::TsMethodSignatureClassMember(m) => Some(m.has_static_modifier()),
                AnyJsClassMember::TsPropertySignatureClassMember(m) => {
                    Some(m.has_static_modifier())
                }
                AnyJsClassMember::TsSetterSignatureClassMember(m) => Some(m.has_static_modifier()),
            })
            .all(|is_static| is_static);

        if all_members_static {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let class_declaration = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                class_declaration.syntax().text_trimmed_range(),
                markup! {
                    "Avoid classes that contain only static members."
                },
            )
            .note(markup! {
                "Prefer using simple functions instead of classes with only static members."
            }),
        )
    }
}
