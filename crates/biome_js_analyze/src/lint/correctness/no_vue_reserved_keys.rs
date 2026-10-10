use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::no_vue_reserved_keys::NoVueReservedKeysOptions;
use biome_vue_semantic::{Symbol, SymbolKind};

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
    type Query = VueComponent;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoVueReservedKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query()
            .declarations()
            .filter_map(|declaration| {
                let is_data = match declaration.kind() {
                    SymbolKind::Data | SymbolKind::AsyncData => true,
                    SymbolKind::Prop
                    | SymbolKind::Computed
                    | SymbolKind::Method
                    | SymbolKind::Watcher => false,
                    _ => return None,
                };
                // Only the top-level members of data become keys of the instance.
                if declaration.parent().is_some() {
                    return None;
                }
                if is_data && declaration.name().starts_with('_') {
                    return Some(RuleState::StartsWithUnderscore(declaration));
                }
                if RESERVED_KEYS.binary_search(&declaration.name()).is_ok() {
                    return Some(RuleState::Reserved(declaration));
                }
                None
            })
            .collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        let (RuleState::Reserved(declaration) | RuleState::StartsWithUnderscore(declaration)) =
            state;
        Some(declaration.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            RuleState::Reserved(declaration) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        declaration.range(),
                        markup! {
                            "Key "<Emphasis>{declaration.name()}</Emphasis>" is reserved in Vue."
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
                        declaration.range(),
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
    Reserved(Symbol),
    StartsWithUnderscore(Symbol),
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
