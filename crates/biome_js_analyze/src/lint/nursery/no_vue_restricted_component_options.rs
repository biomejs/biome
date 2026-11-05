use crate::services::vue::{VueComponent, node_at_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsObjectMember, JsObjectExpression, TextRange};
use biome_rowan::AstNode;
use biome_rule_options::no_vue_restricted_component_options::{
    ComponentOptionPath, NoVueRestrictedComponentOptionsOptions, RestrictedComponentOption,
};

declare_lint_rule! {
    /// Disallow configured options in Vue components.
    ///
    /// A Vue component is described by an object of options, such as `props`, `data` or
    /// `mounted`. Some projects want to avoid certain options, for example options that a
    /// library no longer supports, or options the team decided to stop using.
    ///
    /// This rule reports the options listed in the [`componentOptions`](#componentoptions)
    /// option. It checks the object passed to `defineComponent()` or `createApp()`, the object
    /// exported by default from the `<script>` block of a `.vue` file, and the object passed to
    /// `defineOptions()` in `<script setup>`. It does nothing until you list at least one option.
    ///
    /// ## Examples
    ///
    /// The examples below use the following configuration:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "componentOptions": ["init", ["props", "*", "twoWay"]]
    ///     }
    /// }
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script>
    /// export default {
    ///   init() {},
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     name: { type: String, twoWay: true },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script setup>
    /// defineOptions({
    ///   init() {},
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,use_options
    /// <script>
    /// export default {
    ///   beforeCreate() {},
    ///   props: {
    ///     name: { type: String, required: true },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `componentOptions`
    ///
    /// Default: `[]`
    ///
    /// The component options that must not be used. Each entry is one of the following:
    ///
    /// - The name of an option, such as `"init"`.
    /// - A list of names that leads to an option inside other options. For example,
    ///   `["props", "name", "twoWay"]` matches `twoWay` in `props: { name: { twoWay: true } }`.
    ///   The name `"*"` matches any property, including spread properties such as `...other`
    ///   and properties whose name is computed, such as `[key]`.
    /// - An object with these properties:
    ///   - `name`: The name of the option, or a list of names as described above. This property
    ///     is required.
    ///   - `message`: A custom message to show in place of the default message.
    ///
    /// Names must match the property names exactly.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "componentOptions": [
    ///             { "name": "init", "message": "Use beforeCreate() instead." },
    ///             { "name": ["props", "*", "twoWay"], "message": "Two-way props are not supported." }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script>
    /// export default {
    ///   init() {},
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,use_options
    /// <script>
    /// export default {
    ///   beforeCreate() {},
    ///   props: {
    ///     name: { type: String },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueRestrictedComponentOptions {
        version: "next",
        name: "noVueRestrictedComponentOptions",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-restricted-component-options").inspired()],
    }
}

impl Rule for NoVueRestrictedComponentOptions {
    type Query = VueComponent;
    type State = RestrictedOptionUse;
    type Signals = Box<[Self::State]>;
    type Options = NoVueRestrictedComponentOptionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let restricted_options = ctx.options().component_options();
        if restricted_options.is_empty() {
            return [].into();
        }
        let Some(options_object) = ctx
            .query()
            .options_range()
            .and_then(|range| node_at_range::<JsObjectExpression>(&ctx.root(), range))
        else {
            return [].into();
        };

        let mut uses = Vec::new();
        for (index, restricted_option) in restricted_options.iter().enumerate() {
            for_each_member(&options_object, restricted_option.names(), |member| {
                uses.push(RestrictedOptionUse {
                    member,
                    option: index,
                });
            });
        }
        uses.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let restricted_option = ctx.options().component_options().get(state.option)?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member_report_range(&state.member),
                markup! { {Message(restricted_option)} },
            )
            .note(markup! {
                "Your project lists this component option under "<Emphasis>"componentOptions"</Emphasis>" in the configuration of this rule, so it isn't allowed."
            })
            .note(markup! {
                "Remove this option, or replace it with the alternative your project recommends."
            }),
        )
    }
}

pub struct RestrictedOptionUse {
    /// The object member that writes the restricted option.
    member: AnyJsObjectMember,
    /// The index of the matching entry in the `componentOptions` option.
    option: usize,
}

/// Displays the custom message of a restricted option, or the default message when it has none.
struct Message<'a>(&'a RestrictedComponentOption);

impl biome_console::fmt::Display for Message<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self.0.message() {
            Some(message) => fmt.write_str(message),
            None => fmt.write_markup(markup! {
                "The component option "<Emphasis>{OptionPath(self.0.names())}</Emphasis>" is not allowed."
            }),
        }
    }
}

/// Displays the names of a restricted option joined with dots, such as `props.*.twoWay`.
struct OptionPath<'a>(&'a [Box<str>]);

impl biome_console::fmt::Display for OptionPath<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for (index, name) in self.0.iter().enumerate() {
            if index > 0 {
                fmt.write_str(".")?;
            }
            fmt.write_str(name)?;
        }
        Ok(())
    }
}

/// Calls `f` with each member of `object` that `names` leads to.
///
/// Each name matches a member with that static name, and the name `*` matches any member.
/// Every name except the last one only leads further when the matched member is a property
/// whose value is an object literal.
fn for_each_member(
    object: &JsObjectExpression,
    names: &[Box<str>],
    mut f: impl FnMut(AnyJsObjectMember),
) {
    let mut stack = vec![(object.clone(), 0)];
    while let Some((object, depth)) = stack.pop() {
        let Some(name) = names.get(depth) else {
            continue;
        };
        let is_last = depth + 1 == names.len();
        for member in object.members().into_iter().flatten() {
            let is_match = name.as_ref() == ComponentOptionPath::WILDCARD
                || member
                    .name()
                    .is_some_and(|member_name| member_name.text() == name.as_ref());
            if !is_match {
                continue;
            }
            if is_last {
                f(member);
            } else if let AnyJsObjectMember::JsPropertyObjectMember(property) = &member
                && let Some(AnyJsExpression::JsObjectExpression(value)) = property
                    .value()
                    .ok()
                    .and_then(|value| value.inner_expression())
            {
                stack.push((value, depth + 1));
            }
        }
    }
}

/// Returns the range of the name of `member`, or the range of the whole member when it has no
/// name, as in `...other`.
fn member_report_range(member: &AnyJsObjectMember) -> TextRange {
    let name = match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            property.name().ok().map(|name| name.range())
        }
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            method.name().ok().map(|name| name.range())
        }
        AnyJsObjectMember::JsGetterObjectMember(getter) => {
            getter.name().ok().map(|name| name.range())
        }
        AnyJsObjectMember::JsSetterObjectMember(setter) => {
            setter.name().ok().map(|name| name.range())
        }
        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
            shorthand.name().ok().map(|name| name.range())
        }
        _ => None,
    };
    name.unwrap_or_else(|| member.range())
}
