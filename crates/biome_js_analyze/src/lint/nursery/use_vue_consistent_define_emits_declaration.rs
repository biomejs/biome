use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyTsType, JsBogusMember, JsCallExpression, JsLanguage, JsMetavariable,
    TsCallSignatureTypeMember, TsPropertySignatureTypeMember,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TextRange};
use biome_rule_options::use_vue_consistent_define_emits_declaration::{
    DefineEmitsDeclarationStyle, UseVueConsistentDefineEmitsDeclarationOptions,
};

use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce a consistent declaration style for Vue's `defineEmits` macro.
    ///
    /// In `<script setup lang="ts">`, Vue lets you declare the events a component emits in two
    /// ways: with a TypeScript type passed between angle brackets, or with an argument, such as a
    /// list of event names. A type lets TypeScript check the name and the arguments of every
    /// event that the component emits. This rule enforces one style throughout the project.
    ///
    /// Only `<script setup lang="ts">` blocks are checked, because the type style requires
    /// TypeScript.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const emit = defineEmits(["change", "update"]);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const emit = defineEmits({
    ///   change: (id) => typeof id === "number",
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineEmits<{
    ///   (e: "change", id: number): void;
    ///   (e: "update", value: string): void;
    /// }>();
    ///
    /// defineEmits<{
    ///   change: [id: number];
    ///   update: [value: string];
    /// }>();
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Default: `"type"`
    ///
    /// The style that `defineEmits` calls must use:
    ///
    /// - `"type"`: declare events with a type.
    /// - `"typeLiteral"`: declare events with a type in which each property is an event name,
    ///   and the type of the property lists the arguments of the event, such as
    ///   `{ change: [id: number] }`. This style is shorter than function signatures such as
    ///   `(e: "change", id: number): void`, and requires Vue 3.3 or later.
    /// - `"runtime"`: declare events with an argument, such as `defineEmits(["change"])`.
    ///
    /// #### `typeLiteral`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "typeLiteral"
    ///     }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script setup lang="ts">
    /// const emit = defineEmits<{
    ///   (e: "change", id: number): void;
    /// }>();
    /// </script>
    /// ```
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script setup lang="ts">
    /// const emit = defineEmits(["change"]);
    /// </script>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <script setup lang="ts">
    /// const emit = defineEmits<{
    ///   change: [id: number];
    /// }>();
    /// </script>
    /// ```
    ///
    /// #### `runtime`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "runtime"
    ///     }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script setup lang="ts">
    /// const emit = defineEmits<{
    ///   change: [id: number];
    /// }>();
    /// </script>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <script setup lang="ts">
    /// defineEmits(["change", "update"]);
    ///
    /// defineEmits({
    ///   change: (id) => typeof id === "number",
    /// });
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - To enforce a declaration style for `defineProps`, see [`useVueConsistentDefinePropsDeclaration`](https://biomejs.dev/linter/rules/use-vue-consistent-define-props-declaration/).
    ///
    pub UseVueConsistentDefineEmitsDeclaration {
        version: "next",
        name: "useVueConsistentDefineEmitsDeclaration",
        language: "ts",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("define-emits-declaration").same()],
    }
}

impl Rule for UseVueConsistentDefineEmitsDeclaration {
    type Query = Semantic<JsCallExpression>;
    type State = Violation;
    type Signals = Box<[Self::State]>;
    type Options = UseVueConsistentDefineEmitsDeclarationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.is_typescript() || !source_type.as_embedding_kind().is_vue_setup() {
            return Box::default();
        }

        let call = ctx.query();
        if !is_vue_compiler_macro_call(call, ctx.model(), "defineEmits") {
            return Box::default();
        }

        let has_arguments = call
            .arguments()
            .is_ok_and(|arguments| !arguments.args().is_empty());
        if has_arguments && call.type_arguments().is_some() {
            return Box::new([Violation::MixedDeclaration]);
        }

        match ctx.options().style() {
            DefineEmitsDeclarationStyle::Type if has_arguments => {
                Box::new([Violation::RuntimeDeclaration])
            }
            DefineEmitsDeclarationStyle::Type => Box::default(),
            DefineEmitsDeclarationStyle::TypeLiteral if has_arguments => {
                Box::new([Violation::RuntimeDeclaration])
            }
            DefineEmitsDeclarationStyle::TypeLiteral => find_non_property_events(call),
            DefineEmitsDeclarationStyle::Runtime if call.type_arguments().is_some() => {
                Box::new([Violation::TypeDeclaration])
            }
            DefineEmitsDeclarationStyle::Runtime => Box::default(),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            Violation::RuntimeDeclaration => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"defineEmits"</Emphasis>" call declares events with an argument instead of a type."
                },
            )
            .note(markup! {
                "A type lets TypeScript check the name and the arguments of every event that the component emits."
            })
            .note(markup! {
                "Pass the events as a type between angle brackets, for example "<Emphasis>"defineEmits<{ change: [id: number] }>()"</Emphasis>"."
            }),
            Violation::TypeDeclaration => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"defineEmits"</Emphasis>" call declares events with a type instead of an argument."
                },
            )
            .note(markup! {
                "This project declares events with an argument, so that every component declares its events the same way."
            })
            .note(markup! {
                "Pass the events as an argument, for example "<Emphasis>"defineEmits(['change'])"</Emphasis>"."
            }),
            Violation::MixedDeclaration => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"defineEmits"</Emphasis>" call declares events with both a type and an argument."
                },
            )
            .note(markup! {
                "Vue doesn't accept a type and an argument in the same "<Emphasis>"defineEmits"</Emphasis>" call."
            })
            .note(if ctx.options().style() == DefineEmitsDeclarationStyle::Runtime {
                markup! { "Remove the type between angle brackets and keep the argument." }
            } else {
                markup! { "Remove the argument and keep the type between angle brackets." }
            }),
            Violation::FunctionSignature(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This declares events with a function signature instead of a property."
                },
            )
            .note(markup! {
                "Declaring each event as a property is shorter: the name of the property is the name of the event, and its type lists the arguments of the event."
            })
            .note(markup! {
                "Declare the event as a property, for example "<Emphasis>"change: [id: number]"</Emphasis>"."
            }),
            Violation::NonPropertyMember(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This member of the events type isn't a property."
                },
            )
            .note(markup! {
                "Declaring each event as a property keeps the events easy to read: the name of the property is the name of the event, and its type lists the arguments of the event."
            })
            .note(markup! {
                "Declare the event as a property, for example "<Emphasis>"change: [id: number]"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }
}

/// A part of a `defineEmits` call that doesn't follow the configured style.
pub enum Violation {
    /// The call passes an argument, but the style requires a type.
    RuntimeDeclaration,
    /// The call passes a type, but the style requires an argument.
    TypeDeclaration,
    /// The call passes both a type and an argument, whatever the style.
    MixedDeclaration,
    /// The range of a function type or call signature that declares events.
    FunctionSignature(TextRange),
    /// The range of a member of the events type that is neither a property nor a call signature.
    NonPropertyMember(TextRange),
}

/// Returns the parts of the type passed to `defineEmits` that don't declare an event as a
/// property: a whole function type, or each member of an object type that isn't a property.
fn find_non_property_events(call: &JsCallExpression) -> Box<[Violation]> {
    let Some(events_type) = call
        .type_arguments()
        .and_then(|arguments| arguments.ts_type_argument_list().iter().next()?.ok())
    else {
        return Box::default();
    };

    match events_type.omit_parentheses() {
        AnyTsType::TsFunctionType(function) => {
            Box::new([Violation::FunctionSignature(function.range())])
        }
        AnyTsType::TsObjectType(object) => object
            .members()
            .into_iter()
            .filter(|member| !ALLOWED_MEMBER_KINDS.matches(member.syntax().kind()))
            .map(|member| {
                if TsCallSignatureTypeMember::can_cast(member.syntax().kind()) {
                    Violation::FunctionSignature(member.range())
                } else {
                    Violation::NonPropertyMember(member.range())
                }
            })
            .collect(),
        _ => Box::default(),
    }
}

/// The members of an events type that the `typeLiteral` style accepts. Bogus members and
/// metavariables are skipped because they aren't events written by the user.
const ALLOWED_MEMBER_KINDS: SyntaxKindSet<JsLanguage> = TsPropertySignatureTypeMember::KIND_SET
    .union(JsBogusMember::KIND_SET)
    .union(JsMetavariable::KIND_SET);
