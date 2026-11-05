use crate::JsRuleAction;
use crate::services::vue::VueComponentRegistration;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsObjectMember, AnyJsObjectMemberName, T};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_vue_registered_component_name_casing::{
    RegisteredComponentNameCasing, UseVueRegisteredComponentNameCasingOptions,
};
use biome_string_case::{Case, Cases};
use biome_unicode_table::is_js_ident;

declare_lint_rule! {
    /// Enforce a consistent casing style for the names of components registered in the `components` option of Vue components.
    ///
    /// A Vue component can make other components available to its template by listing them in
    /// its `components` option. The name of each entry is the name the template uses for that
    /// component. Using one naming style for all of these names makes components easier to find
    /// and to use.
    ///
    /// The naming style also decides how the template can refer to a component. A component
    /// registered as `TodoItem` can be written as both `<TodoItem>` and `<todo-item>` in a
    /// template. A component registered as `todo-item` can only be written as `<todo-item>`, and
    /// a component registered as `todoItem` can't be written as `<TodoItem>`.
    ///
    /// Names written only with characters from writing systems without uppercase and
    /// lowercase letters, such as `漢字`, follow every style. Names that contain symbols, such as
    /// `_` or `$`, don't follow any style.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   components: {
    ///     todoItem: TodoItem,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   components: {
    ///     "todo-item": TodoItem,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   components: {
    ///     TodoItem,
    ///     TodoList: List,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `casing`
    ///
    /// Default: `"PascalCase"`
    ///
    /// The casing that component names must follow: `"PascalCase"`, `"camelCase"`, or
    /// `"kebab-case"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "casing": "kebab-case"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script>
    /// export default {
    ///   components: {
    ///     TodoItem,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script>
    /// export default {
    ///   components: {
    ///     "todo-item": TodoItem,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub UseVueRegisteredComponentNameCasing {
        version: "next",
        name: "useVueRegisteredComponentNameCasing",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("component-options-name-casing").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueRegisteredComponentNameCasing {
    type Query = VueComponentRegistration;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueRegisteredComponentNameCasingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let casing = ctx.options().casing();
        (!accepted_cases(casing).contains(Case::identify(ctx.query().name(), false))).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let registration = ctx.query();
        let casing = CasingName(ctx.options().casing());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                registration.range(),
                markup! {
                    "The component name "<Emphasis>{registration.name()}</Emphasis>" isn't in "{casing}"."
                },
            )
            .note(markup! {
                "Using one naming style for all registered components makes them easier to find and to use."
            })
            .note(markup! {
                "Rename the component to "{casing}"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let registration = ctx.query();
        let casing = ctx.options().casing();
        let converted = target_case(casing).convert(registration.name());
        // The conversion drops symbols, so a name such as `$_` has no counterpart in the
        // configured style.
        if !accepted_cases(casing).contains(Case::identify(&converted, false)) {
            return None;
        }
        let token = if is_js_ident(&converted) {
            make::ident(&converted)
        } else if ctx.preferred_quote().is_double() {
            make::js_string_literal(&converted)
        } else {
            make::js_string_literal_single_quotes(&converted)
        };
        let new_name = AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(token));

        let member = AnyJsObjectMember::cast(ctx.vue_model().node(registration)?)?;
        let mut mutation = ctx.root().begin();
        match member {
            AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                // `{ todoItem }` becomes `{ TodoItem: todoItem }`, so that the value stays the
                // same variable. The value is a new identifier: reusing the old one would also
                // reuse its leading trivia, which `replace_node` already moves to the new member.
                let value = AnyJsExpression::JsIdentifierExpression(make::js_identifier_expression(
                    make::js_reference_identifier(make::ident(registration.name())),
                ));
                let property = make::js_property_object_member(
                    new_name,
                    make::token_with_trailing_space(T![:]),
                    value,
                );
                mutation.replace_node(
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand),
                    AnyJsObjectMember::JsPropertyObjectMember(property),
                );
            }
            member => {
                let old_name = member_name(&member)?;
                // A computed name such as `["todo-item"]` can't be replaced by a plain name
                // without changing how the entry is written.
                if !matches!(old_name, AnyJsObjectMemberName::JsLiteralMemberName(_)) {
                    return None;
                }
                mutation.replace_node(old_name, new_name);
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Rename the component to "<Emphasis>{converted.as_str()}</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// Returns the cases a name may be identified as and still follow `casing`.
///
/// Names without uppercase and lowercase letters, such as `漢字`, follow every casing.
/// An uppercase name, such as `QR`, follows PascalCase.
fn accepted_cases(casing: RegisteredComponentNameCasing) -> Cases {
    match casing {
        RegisteredComponentNameCasing::Pascal => Case::Pascal | Case::Upper | Case::Uni,
        RegisteredComponentNameCasing::Camel => Case::Camel | Case::Uni,
        RegisteredComponentNameCasing::Kebab => Case::Kebab | Case::Uni,
    }
}

/// Returns the case that names are converted to when they don't follow `casing`.
fn target_case(casing: RegisteredComponentNameCasing) -> Case {
    match casing {
        RegisteredComponentNameCasing::Pascal => Case::Pascal,
        RegisteredComponentNameCasing::Camel => Case::Camel,
        RegisteredComponentNameCasing::Kebab => Case::Kebab,
    }
}

/// Returns the name of an object member that has a separate name and value, such as a property
/// or a method.
///
/// Returns `None` for shorthand properties, spreads, and members that failed to parse.
fn member_name(member: &AnyJsObjectMember) -> Option<AnyJsObjectMemberName> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => property.name().ok(),
        AnyJsObjectMember::JsMethodObjectMember(method) => method.name().ok(),
        AnyJsObjectMember::JsGetterObjectMember(getter) => getter.name().ok(),
        AnyJsObjectMember::JsSetterObjectMember(setter) => setter.name().ok(),
        AnyJsObjectMember::JsShorthandPropertyObjectMember(_)
        | AnyJsObjectMember::JsSpread(_)
        | AnyJsObjectMember::JsBogusMember(_)
        | AnyJsObjectMember::JsMetavariable(_) => None,
    }
}

/// Writes the name of a casing style as users write it in the rule options.
struct CasingName(RegisteredComponentNameCasing);

impl biome_console::fmt::Display for CasingName {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.0 {
            RegisteredComponentNameCasing::Pascal => "PascalCase",
            RegisteredComponentNameCasing::Camel => "camelCase",
            RegisteredComponentNameCasing::Kebab => "kebab-case",
        })
    }
}
