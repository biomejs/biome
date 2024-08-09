use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyJsClassMemberName, JsConstructorClassMember, JsGetterClassMember, JsMethodClassMember,
    JsPropertyClassMember, JsSetterClassMember, TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, TextRange};
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
    /// ```jsx,ignore
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
    /// ```jsx,ignore
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
    /// ```jsx,ignore
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
    /// ```jsx,ignore
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
    /// ```jsx,ignore
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
    /// ```jsx,ignore
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
        sources: &[RuleSource::Eslint("explicit-member-accessibility")],
        recommended: false,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseConsistentMemberAccessibilityOptions {
    accessibility: Accessibility,
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
    type Query = Ast<AnyMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentMemberAccessibilityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match ctx.options().accessibility {
            Accessibility::NoPublic => {
                if node.is_no_public() == Some(false) {
                    Some(())
                } else {
                    None
                }
            }
            Accessibility::Explicit => {
                if node.is_explicit() == Some(false) {
                    Some(())
                } else {
                    None
                }
            }
            Accessibility::None => {
                if node.is_none() == Some(false) {
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match ctx.options().accessibility {
            Accessibility::NoPublic => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.property_range(),
                    markup! {
                        "Missing accessibility modifier on member."
                    },
                )
                .note(markup! {
                    "Public accessibility modifier cannot be used."
                }),
            ),
            Accessibility::Explicit => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.syntax().text_trimmed_range(),
                    markup! {
                        "Missing accessibility modifier on member."
                    },
                )
                .note(markup! {
                    "Accessibility modifier is required."
                }),
            ),
            Accessibility::None => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.property_range(),
                    markup! {
                        "Missing accessibility modifier on member."
                    },
                )
                .note(markup! {
                    "Accessibility modifier cannot be used."
                }),
            ),
        }
    }
}

declare_node_union! {
    pub AnyJsMemberWithAccessibility = JsConstructorClassMember | JsPropertyClassMember | JsMethodClassMember | TsPropertyParameter | JsGetterClassMember | JsSetterClassMember
    | TsConstructorSignatureClassMember | TsMethodSignatureClassMember | TsPropertySignatureClassMember | TsGetterSignatureClassMember | TsSetterSignatureClassMember
    | TsPropertyParameter
}

impl AnyMember {
    fn is_no_public(&self) -> Option<bool> {
        match self {
            AnyMember::JsConstructorClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_no_public(&accessibility)),
            ),
            AnyMember::JsPropertyClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_no_public(&accessibility)),
            ),
            AnyMember::JsMethodClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_no_public(&accessibility)),
            ),
            AnyMember::JsGetterClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_no_public(&accessibility)),
            ),
            AnyMember::TsPropertyParameter(params) => Some(
                params
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_no_public(&accessibility)),
            ),
            AnyMember::JsSetterClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_no_public(&accessibility)),
            ),
        }
    }

    fn is_explicit(&self) -> Option<bool> {
        match self {
            AnyMember::JsConstructorClassMember(member) => {
                if matches!(
                    AnyJsClassMemberName::cast(member.name().ok()?.syntax().clone()),
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                ) {
                    return None;
                }
                Some(member.modifiers().iter().any(|x| {
                    if let Some(accessibility) = TsAccessibilityModifier::cast_ref(x.syntax()) {
                        is_explicit(&accessibility)
                    } else {
                        false
                    }
                }))
            }
            AnyMember::JsPropertyClassMember(member) => {
                if matches!(
                    AnyJsClassMemberName::cast(member.name().ok()?.syntax().clone()),
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                ) {
                    return None;
                }
                Some(member.modifiers().iter().any(|x| {
                    if let Some(accessibility) = TsAccessibilityModifier::cast_ref(x.syntax()) {
                        is_explicit(&accessibility)
                    } else {
                        false
                    }
                }))
            }
            AnyMember::JsMethodClassMember(member) => {
                if matches!(
                    AnyJsClassMemberName::cast(member.name().ok()?.syntax().clone()),
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                ) {
                    return None;
                }
                Some(member.modifiers().iter().any(|x| {
                    if let Some(accessibility) = TsAccessibilityModifier::cast_ref(x.syntax()) {
                        is_explicit(&accessibility)
                    } else {
                        false
                    }
                }))
            }
            AnyMember::JsGetterClassMember(member) => {
                if matches!(
                    AnyJsClassMemberName::cast(member.name().ok()?.syntax().clone()),
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                ) {
                    return None;
                }
                Some(member.modifiers().iter().any(|x| {
                    if let Some(accessibility) = TsAccessibilityModifier::cast_ref(x.syntax()) {
                        is_explicit(&accessibility)
                    } else {
                        false
                    }
                }))
            }
            AnyMember::TsPropertyParameter(params) => Some(params.modifiers().iter().any(|x| {
                if let Some(accessibility) = TsAccessibilityModifier::cast_ref(x.syntax()) {
                    is_explicit(&accessibility)
                } else {
                    false
                }
            })),

            AnyMember::JsSetterClassMember(member) => {
                if matches!(
                    AnyJsClassMemberName::cast(member.name().ok()?.syntax().clone()),
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                ) {
                    return None;
                }
                Some(member.modifiers().iter().any(|x| {
                    if let Some(accessibility) = TsAccessibilityModifier::cast_ref(x.syntax()) {
                        is_explicit(&accessibility)
                    } else {
                        false
                    }
                }))
            }
        }
    }

    fn is_none(&self) -> Option<bool> {
        match self {
            AnyMember::JsConstructorClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_none(&accessibility)),
            ),
            AnyMember::JsPropertyClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_none(&accessibility)),
            ),
            AnyMember::JsMethodClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_none(&accessibility)),
            ),
            AnyMember::JsGetterClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_none(&accessibility)),
            ),
            AnyMember::TsPropertyParameter(params) => Some(
                params
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_none(&accessibility)),
            ),
            AnyMember::JsSetterClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .all(|accessibility| is_none(&accessibility)),
            ),
        }
    }

    fn property_range(&self) -> Option<TextRange> {
        match self {
            AnyMember::JsConstructorClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .map(|accessibility| accessibility.syntax().text_trimmed_range())
                    .next()?,
            ),
            AnyMember::JsPropertyClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .map(|accessibility| accessibility.syntax().text_trimmed_range())
                    .next()?,
            ),
            AnyMember::JsMethodClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .map(|accessibility| accessibility.syntax().text_trimmed_range())
                    .next()?,
            ),
            AnyMember::TsPropertyParameter(params) => Some(
                params
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .map(|accessibility| accessibility.syntax().text_trimmed_range())
                    .next()?,
            ),
            AnyMember::JsGetterClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .map(|accessibility| accessibility.syntax().text_trimmed_range())
                    .next()?,
            ),
            AnyMember::JsSetterClassMember(member) => Some(
                member
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast_ref(x.syntax()))
                    .map(|accessibility| accessibility.syntax().text_trimmed_range())
                    .next()?,
            ),
        }
    }
}

fn is_no_public(accessibility: &TsAccessibilityModifier) -> bool {
    !accessibility.is_public()
}

fn is_explicit(accessibility: &TsAccessibilityModifier) -> bool {
    accessibility.is_private() || accessibility.is_protected() || accessibility.is_public()
}

fn is_none(accessibility: &TsAccessibilityModifier) -> bool {
    !(accessibility.is_private() || accessibility.is_protected() || accessibility.is_public())
}
