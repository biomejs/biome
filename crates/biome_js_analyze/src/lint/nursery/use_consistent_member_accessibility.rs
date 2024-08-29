use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    JsConstructorClassMember, JsGetterClassMember, JsMethodClassMember, JsPropertyClassMember,
    JsSetterClassMember, TsAccessibilityModifier, TsConstructorSignatureClassMember,
    TsGetterSignatureClassMember, TsMethodSignatureClassMember, TsPropertyParameter,
    TsPropertySignatureClassMember, TsSetterSignatureClassMember,
};
use biome_rowan::{declare_node_union, AstNode, TextRange};
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Require consistent accessibility modifiers on class properties and methods.
    ///
    /// TypeScript allows placing explicit `public`, `protected`, and `private` accessibility modifiers in front of class members.
    /// The modifiers exist solely in the type system and just serve to describe who is allowed to access those members.
    /// Leaving off accessibility modifiers makes for less code to read and write. Members are public by default.
    ///
    /// However, adding in consistent accessibility modifiers can be helpful in codebases with many classes for enforcing proper privacy of members.
    /// Some developers also find it preferable for code readability to keep member publicity explicit.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following patterns are considered incorrect code with the default options `noPublic`:
    ///
    /// ```js,ignore
    /// class Animal {
    ///   public constructor(
    ///     public breed,
    ///     name,
    ///   ) {
    ///     // Parameter property and constructor
    ///     this.animalName = name;
    ///   }
    ///   public animalName: string; // Property
    ///   public get name(): string {
    ///     // get accessor
    ///     return this.animalName;
    ///   }
    ///   public set name(value: string) {
    ///     // set accessor
    ///     this.animalName = value;
    ///   }
    ///   public walk() {
    ///     // method
    ///   }
    /// }
    /// ```
    ///
    /// The following patterns are considered incorrect code with the accessibility set to `explicit`:
    ///
    /// ```ts,ignore
    /// class Animal {
    ///   // Constructor is not set accessibility modifier
    ///   constructor(
    ///     public breed,
    ///     name,
    ///   ) {
    ///     // Parameter property and constructor
    ///     this.animalName = name;
    ///   }
    ///   private animalName: string; // Property
    ///   public get name(): string {
    ///     // get accessor
    ///     return this.animalName;
    ///   }
    ///   public set name(value: string) {
    ///     // set accessor
    ///     this.animalName = value;
    ///   }
    ///   protected walk() {
    ///     // method
    ///   }
    /// }
    /// ```
    ///
    /// The following patterns are considered incorrect code with the accessibility set to `none`:
    ///
    /// ```ts,ignore
    /// class Animal {
    ///   constructor(
    ///     protected breed,
    ///     name,
    ///   ) {
    ///     // Parameter property and constructor
    ///     this.name = name;
    ///   }
    ///   // Property is set accessibility modifier
    ///   private animalName: string; // Property
    ///   get name(): string {
    ///     // get accessor
    ///     return this.animalName;
    ///   }
    ///   // set accessor is set accessibility modifier
    ///   set name(value: string) {
    ///     // set accessor
    ///     this.animalName = value;
    ///   }
    ///   // walk() is set accessibility modifier
    ///   protected walk() {
    ///     // method
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// The following patterns are considered correct code with the default options `noPublic`:
    ///
    /// ```ts,ignore
    /// class Animal {
    ///   constructor(
    ///     public breed,
    ///     name,
    ///   ) {
    ///     // Parameter property and constructor
    ///     this.animalName = name;
    ///   }
    ///   private animalName: string; // Property
    ///   get name(): string {
    ///     // get accessor
    ///     return this.animalName;
    ///   }
    ///   set name(value: string) {
    ///     // set accessor
    ///     this.animalName = value;
    ///   }
    ///   protected walk() {
    ///     // method
    ///   }
    /// }
    /// ```
    ///
    /// The following patterns are considered correct code with the accessibility set to `explicit`:
    ///
    /// ```ts,ignore
    /// class Animal {
    ///   public constructor(
    ///     public breed,
    ///     name,
    ///   ) {
    ///     // Parameter property and constructor
    ///     this.animalName = name;
    ///   }
    ///   private animalName: string; // Property
    ///   public get name(): string {
    ///     // get accessor
    ///     return this.animalName;
    ///   }
    ///   public set name(value: string) {
    ///     // set accessor
    ///     this.animalName = value;
    ///   }
    ///   protected walk() {
    ///     // method
    ///   }
    /// }
    /// ```
    ///
    /// The following patterns are considered correct code with the accessibility set to `none`:
    ///
    /// ```ts,ignore
    /// class Animal {
    ///   constructor(
    ///     breed,
    ///     name,
    ///   ) {
    ///     // Parameter property and constructor
    ///     this.name = name;
    ///   }
    ///   animalName: string; // Property
    ///   get name(): string {
    ///     // get accessor
    ///     return this.animalName;
    ///   }
    ///   set name(value: string) {
    ///     // set accessor
    ///     this.animalName = value;
    ///   }
    ///   walk() {
    ///     // method
    ///   }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "accessibility": "explicit"
    ///     }
    /// }
    /// ```
    ///
    /// ### `accessibility`
    ///
    /// This option determines the required accessibility modifiers on class properties and methods.
    /// It can be set to one of the following values:
    ///
    /// - `noPublic` - forbid the use of public (a safe fix will remove it).
    /// - `explicit` - requires an accessibility modifier for every member that allows that (a safe fix will add public).
    /// - `none` - forbid all accessibility modifiers (public, protected, private).
    ///
    /// Default: `noPublic`.
    ///
    pub UseConsistentMemberAccessibility {
        version: "next",
        name: "useConsistentMemberAccessibility",
        language: "ts",
        recommended: true,
        sources: &[RuleSource::EslintTypeScript("explicit-member-accessibility")],
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConsistentMemberAccessibilityOptions {
    pub accessibility: Accessibility,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Accessibility {
    #[default]
    NoPublic,
    Explicit,
    None,
}

impl Rule for UseConsistentMemberAccessibility {
    type Query = Ast<AnyJsMemberWithAccessibility>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ConsistentMemberAccessibilityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let accessibility = node.accessibility_modifier();
        match ctx.options().accessibility {
            Accessibility::NoPublic => accessibility
                .filter(|accessibility| accessibility.is_public())
                .map(|accessibility| accessibility.range()),
            Accessibility::Explicit => accessibility.is_none().then_some(node.range()),
            Accessibility::None => accessibility.map(|accessibility| accessibility.range()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let (diag_msg, note_msg) = match ctx.options().accessibility {
            Accessibility::NoPublic => (
                markup! {
                    "The "<Emphasis>"public"</Emphasis>" modifier is disallowed."
                },
                markup! {"Remove the accessibility modifier."},
            ),
            Accessibility::Explicit => (
                markup! {
                    "Missing accessibility modifier on this member."
                },
                markup! {
                    "Use "<Emphasis>"public"</Emphasis>" to explicitly make a member public."
                },
            ),
            Accessibility::None => (
                markup! {
                    "Accessibility modifiers are disallowed."
                },
                markup! {"Remove the accessibility modifier."},
            ),
        };
        Some(RuleDiagnostic::new(rule_category!(), range, diag_msg).note(note_msg))
    }
}

declare_node_union! {
    pub AnyJsMemberWithAccessibility = JsConstructorClassMember
        | JsPropertyClassMember
        | JsMethodClassMember
        | TsPropertyParameter
        | JsGetterClassMember
        | JsSetterClassMember
        | TsConstructorSignatureClassMember
        | TsMethodSignatureClassMember
        | TsPropertySignatureClassMember
        | TsGetterSignatureClassMember
        | TsSetterSignatureClassMember
}

impl AnyJsMemberWithAccessibility {
    fn accessibility_modifier(&self) -> Option<TsAccessibilityModifier> {
        match self {
            Self::JsConstructorClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::JsPropertyClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::JsMethodClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::TsPropertyParameter(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::JsGetterClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::JsSetterClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::TsConstructorSignatureClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::TsMethodSignatureClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::TsPropertySignatureClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::TsGetterSignatureClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
            Self::TsSetterSignatureClassMember(member) => member
                .modifiers()
                .into_iter()
                .map(AstNode::into_syntax)
                .find_map(TsAccessibilityModifier::cast),
        }
    }
}
