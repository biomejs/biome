use crate::services::vue::VueReference;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{AnyJsMemberExpression, AnyJsRoot};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_vue_undeclared_properties::NoVueUndeclaredPropertiesOptions;
use biome_vue_semantic::{
    Access, Namespace, RefBase, RefSite, Reference, Resolution, SnippetHost,
};

declare_lint_rule! {
    /// Disallow using properties that a Vue component doesn't declare.
    ///
    /// A component's template and its `this` can only use what the component declares: its
    /// props, data, computed properties, methods, injected values, the values its `setup()`
    /// function returns, and the variables of `<script setup>`. Using any other name gives
    /// `undefined`. This usually means the name has a typo, or the declaration was removed or
    /// never written.
    ///
    /// This rule reports such names in the template, in `this.name` inside the component's
    /// options, and in `props.name` when `props` is the object returned by `defineProps()` or
    /// passed to `setup()`. It also reports names written as strings in the `watch` and `expose`
    /// options, and members that a data property doesn't have when its value is an object
    /// written in place, such as `user.nmae` when `data()` returns `{ user: { name: "" } }`.
    ///
    /// Names that start with `$`, such as `$route` or `$t`, are never reported, because plugins
    /// commonly add properties with these names to every component.
    ///
    /// :::note
    /// In `.vue` files, this rule needs to read the template, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule only checks the script.
    /// :::
    ///
    /// The rule reports nothing for a component when it can't know everything the component
    /// declares. That is the case when:
    ///
    /// - the component uses `mixins` or `extends`;
    /// - an object is copied into the options with `...`, as in `computed: { ...mapState(["user"]) }`;
    /// - an option such as `data` or `setup` returns something other than an object written
    ///   in place, as in `return toRefs(state)`;
    /// - the props are declared with a TypeScript type that is imported from another file;
    /// - the component has a `<script src>` block.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   data() {
    ///     return { count: 0 };
    ///   },
    ///   methods: {
    ///     increment() {
    ///       this.cnt++;
    ///     },
    ///   },
    /// });
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const props = defineProps(["name"]);
    /// console.log(props.title);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script setup>
    /// const count = 0;
    /// </script>
    ///
    /// <template>
    ///   <p>{{ cnt }}</p>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: ["name"],
    ///   data() {
    ///     return { count: 0 };
    ///   },
    ///   methods: {
    ///     increment() {
    ///       this.count++;
    ///     },
    ///   },
    /// };
    /// </script>
    ///
    /// <template>
    ///   <button @click="increment">{{ name }}: {{ count }} {{ $t("clicks") }}</button>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignore`
    ///
    /// A list of property names that are never reported. Use it for properties that a plugin
    /// adds to every component, when their names don't start with `$`.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignore": ["i18n"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   methods: {
    ///     greet() {
    ///       return this.i18n.translate("hello");
    ///     },
    ///   },
    /// });
    /// ```
    ///
    /// ## See Also
    ///
    /// - [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/)
    ///   reports variables that are not declared anywhere, including in Vue templates.
    ///
    pub NoVueUndeclaredProperties {
        version: "next",
        name: "noVueUndeclaredProperties",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-undef-properties").inspired()],
    }
}

impl Rule for NoVueUndeclaredProperties {
    type Query = VueReference;
    type State = UndeclaredProperty;
    type Signals = Option<Self::State>;
    type Options = NoVueUndeclaredPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = ctx.query();
        if reference.namespace() != Namespace::Binding
            || !matches!(reference.site(), RefSite::Identifier | RefSite::OptionName)
        {
            return None;
        }
        match reference.resolution() {
            Resolution::Unresolved => {
                let name = reference.name();
                if name.starts_with('$')
                    || ctx
                        .options()
                        .ignore()
                        .iter()
                        .any(|ignored| ignored.as_ref() == name)
                {
                    return None;
                }
                Some(if is_props_member(reference) {
                    UndeclaredProperty::Prop
                } else {
                    UndeclaredProperty::Property
                })
            }
            Resolution::Symbol(_) => {
                let (depth, member) = find_undeclared_member(reference)?;
                if OBJECT_PROTOTYPE_MEMBERS.binary_search(&member).is_ok() {
                    return None;
                }
                Some(UndeclaredProperty::Member {
                    depth,
                    range: member_range(&ctx.root(), reference, depth)
                        .unwrap_or_else(|| reference.range()),
                })
            }
            Resolution::Global | Resolution::Unknowable => None,
        }
    }

    fn text_range(ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(match state {
            UndeclaredProperty::Member { range, .. } => *range,
            UndeclaredProperty::Property | UndeclaredProperty::Prop => ctx.query().range(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let reference = ctx.query();
        let name = reference.name();
        let (range, message) = match state {
            UndeclaredProperty::Property => (
                reference.range(),
                markup! {
                    <Emphasis>{name}</Emphasis>" isn't declared by this component."
                }
                .to_owned(),
            ),
            UndeclaredProperty::Prop => (
                reference.range(),
                markup! {
                    <Emphasis>{name}</Emphasis>" isn't declared as a prop of this component."
                }
                .to_owned(),
            ),
            UndeclaredProperty::Member { depth, range } => {
                let path = std::iter::once(name)
                    .chain(reference.path().take(depth + 1))
                    .collect::<Vec<_>>()
                    .join(".");
                (
                    *range,
                    markup! {
                        <Emphasis>{path}</Emphasis>" isn't declared by this component."
                    }
                    .to_owned(),
                )
            }
        };
        let effect = if reference.access() == Access::Write {
            markup! {
                "Vue doesn't track changes to it, so the page doesn't update when it changes."
            }
            .to_owned()
        } else {
            markup! {
                "Its value is always "<Emphasis>"undefined"</Emphasis>"."
            }
            .to_owned()
        };
        Some(
            RuleDiagnostic::new(rule_category!(), range, message)
                .note(effect)
                .note(markup! {
                    "This often means the name has a typo, or its declaration is missing."
                })
                .note(markup! {
                    "Fix the name, or declare it in the component."
                }),
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UndeclaredProperty {
    /// A property of the component instance that nothing declares.
    Property,
    /// A member of the props object that no prop declares.
    Prop,
    /// A member that the object written as the value of a data property
    /// doesn't have, as in `user.nmae` for `data() { return { user: { name: '' } } }`.
    Member {
        /// The index in [`Reference::path`] of the missing member.
        depth: usize,
        /// The range from the property name to the end of the missing member.
        range: TextRange,
    },
}

/// Members that every object inherits, so reading them from an object
/// written in place is not a mistake.
const OBJECT_PROTOTYPE_MEMBERS: &[&str] = &[
    "constructor",
    "hasOwnProperty",
    "isPrototypeOf",
    "propertyIsEnumerable",
    "toLocaleString",
    "toString",
    "valueOf",
];

/// Returns `true` when `reference` reads a member of the props object.
fn is_props_member(reference: &Reference) -> bool {
    match reference.base() {
        RefBase::This => false,
        // `props.title` with `props` from `defineProps()`, or `$props.title`.
        RefBase::Via(_) | RefBase::Builtin => true,
        // A bare name in a template is a name of the component. In a script,
        // the only member recorded without a base is one read from the first
        // parameter of `setup()`.
        RefBase::Bare => reference
            .snippet()
            .is_none_or(|snippet| matches!(snippet.host(), SnippetHost::Script(_))),
    }
}

/// Follows the members `reference` reads through data properties written as
/// object literals, and returns the index and name of the first member that
/// the object doesn't have.
fn find_undeclared_member(reference: &Reference) -> Option<(usize, &str)> {
    let mut symbol = reference.symbol()?;
    // A mixin, a component it extends, or an object spread into its data can
    // replace the value of the property.
    if symbol
        .component()
        .is_none_or(|component| !component.is_complete(Namespace::Binding))
    {
        return None;
    }
    let depth = reference.path().count();
    for (index, member) in reference.path().enumerate() {
        if !symbol.has_known_members() {
            return None;
        }
        let found = symbol.members().find(|symbol| symbol.name() == member);
        match found {
            Some(found) => symbol = found,
            // Assigning a member that doesn't exist yet adds it.
            None if index + 1 == depth && reference.access() == Access::Write => return None,
            None => return Some((index, member)),
        }
    }
    None
}

/// Returns the range from the start of `reference` to the end of the member
/// at index `depth` of its path.
///
/// Returns `None` for a name written as a string, whose members are part of
/// the same string.
fn member_range(root: &AnyJsRoot, reference: &Reference, depth: usize) -> Option<TextRange> {
    if reference.site() != RefSite::Identifier {
        return None;
    }
    let start = reference.range();
    let element = root.syntax().covering_element(start);
    let member = element
        .ancestors()
        .filter(|node| {
            (AnyJsMemberExpression::can_cast(node.kind())
                || AnyJsMemberAssignment::can_cast(node.kind()))
                && node.text_trimmed_range().end() > start.end()
        })
        .nth(depth)?;
    Some(TextRange::new(start.start(), member.text_trimmed_range().end()))
}

#[cfg(test)]
mod tests {
    use super::OBJECT_PROTOTYPE_MEMBERS;

    #[test]
    fn object_prototype_members_are_sorted() {
        for pair in OBJECT_PROTOTYPE_MEMBERS.windows(2) {
            assert!(pair[0] < pair[1], "{} must come after {}", pair[0], pair[1]);
        }
    }
}
