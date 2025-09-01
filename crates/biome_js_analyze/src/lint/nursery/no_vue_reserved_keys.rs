use crate::frameworks::vue::vue_component::{
    VueComponent, VueComponentDeclarations, VueComponentQuery, VueDeclaration,
    VueDeclarationCollectionFilter, VueDeclarationName,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::no_vue_reserved_keys::NoVueReservedKeysOptions;
use enumflags2::BitFlag;

declare_lint_rule! {
    /// Disallow reserved keys in Vue component data and computed properties.
    ///
    /// Vue reserves certain keys for its internal use. Using these reserved keys
    /// in data properties, computed properties, methods, or other component options
    /// can cause conflicts and unpredictable behavior in your Vue components.
    ///
    /// This rule prevents the use of Vue reserved keys such as:
    /// - Keys starting with `$` (e.g., `$el`, `$data`, `$props`, `$refs`, etc.)
    /// - Keys starting with `_` in data properties (reserved for Vue internals)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     data: {
    ///         $el: '',
    ///     },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     data() {
    ///         return {
    ///             _foo: 'bar',
    ///         };
    ///     },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     computed: {
    ///         $data() {
    ///             return this.someData;
    ///         },
    ///     },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     methods: {
    ///         $emit() {
    ///             // This conflicts with Vue's built-in $emit
    ///         },
    ///     },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///     data() {
    ///         return {
    ///             message: 'Hello Vue!',
    ///             count: 0,
    ///         };
    ///     },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///     computed: {
    ///         displayMessage() {
    ///             return this.message;
    ///         },
    ///     },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueReservedKeys {
        version: "2.1.3",
        name: "noVueReservedKeys",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-reserved-keys").same()],
    }
}

impl Rule for NoVueReservedKeys {
    type Query = VueComponentQuery;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoVueReservedKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return Box::new([]);
        };
        component
            .declarations(VueDeclarationCollectionFilter::all())
            .into_iter()
            .filter_map(|declaration| {
                if let Some(name) = declaration.declaration_name() {
                    if matches!(
                        declaration,
                        VueDeclaration::Data(_) | VueDeclaration::AsyncData(_)
                    ) && name.text().starts_with('_')
                    {
                        return Some(RuleState::StartsWithUnderscore(declaration));
                    }
                    if RESERVED_KEYS.binary_search(&name.text()).is_ok() {
                        return Some(RuleState::Reserved(declaration));
                    }
                }
                None
            })
            .collect::<Box<[_]>>()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            RuleState::Reserved(declaration) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        declaration.declaration_name_range()?,
                        markup! {
                            "Key "<Emphasis>{declaration.declaration_name()?.text()}</Emphasis>" is reserved in Vue."
                        },
                    )
                    .note(markup! {
                        "Rename the key to avoid conflicts with Vue reserved keys."
                    }),
                )
            }
            RuleState::StartsWithUnderscore(declaration) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        declaration.declaration_name_range()?,
                        markup! {
                            "Keys starting with an underscore are reserved in Vue."
                        },
                    )
                    .note(markup! {
                        "Rename the key to avoid conflicts with Vue reserved keys."
                    }),
                )
            }
        }
    }
}

pub enum RuleState {
    Reserved(VueDeclaration),
    StartsWithUnderscore(VueDeclaration),
}

const RESERVED_KEYS: &[&str] = &[
    "$attrs",
    "$children",
    "$data",
    "$delete",
    "$destroy",
    "$el",
    "$emit",
    "$forceUpdate",
    "$isServer",
    "$listeners",
    "$mount",
    "$nextTick",
    "$off",
    "$on",
    "$once",
    "$options",
    "$parent",
    "$props",
    "$refs",
    "$root",
    "$scopedSlots",
    "$set",
    "$slots",
    "$watch",
];

#[cfg(test)]
mod tests {
    use super::RESERVED_KEYS;

    #[test]
    fn reserved_keys_should_be_sorted() {
        assert!(
            RESERVED_KEYS.is_sorted(),
            "RESERVED_KEYS should be sorted for binary search."
        );
    }
}
