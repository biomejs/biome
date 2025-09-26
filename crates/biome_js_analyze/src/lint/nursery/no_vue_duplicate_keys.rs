use crate::frameworks::vue::vue_component::{
    VueComponent, VueComponentDeclarations, VueComponentQuery, VueDeclaration,
    VueDeclarationCollectionFilter, VueDeclarationName,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::no_vue_duplicate_keys::NoVueDuplicateKeysOptions;
use enumflags2::BitFlag;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Disallow duplicate keys in Vue component data, methods, computed properties, and other options.
    ///
    /// This rule prevents the use of duplicate keys across different Vue component options
    /// such as `props`, `data`, `computed`, `methods`, and `setup`. Even if keys don't conflict
    /// in the script tag, they may cause issues in the template since Vue allows direct
    /// access to these keys.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     props: ['foo'],
    ///     data() {
    ///         return {
    ///             foo: 'bar'
    ///         };
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     data() {
    ///         return {
    ///             message: 'hello'
    ///         };
    ///     },
    ///     methods: {
    ///         message() {
    ///             console.log('duplicate key');
    ///         }
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///     computed: {
    ///         count() {
    ///             return this.value * 2;
    ///         }
    ///     },
    ///     methods: {
    ///         count() {
    ///             this.value++;
    ///         }
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///     props: ['foo'],
    ///     data() {
    ///         return {
    ///             bar: 'baz'
    ///         };
    ///     },
    ///     methods: {
    ///         handleClick() {
    ///             console.log('unique key');
    ///         }
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///     computed: {
    ///         displayMessage() {
    ///             return this.message.toUpperCase();
    ///         }
    ///     },
    ///     methods: {
    ///         clearMessage() {
    ///             this.message = '';
    ///         }
    ///     }
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueDuplicateKeys {
        version: "2.2.5",
        name: "noVueDuplicateKeys",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-dupe-keys").same()],
    }
}

impl Rule for NoVueDuplicateKeys {
    type Query = VueComponentQuery;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoVueDuplicateKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return Box::new([]);
        };

        let mut key_declarations: FxHashMap<String, Vec<VueDeclaration>> = FxHashMap::default();

        // Collect all declarations across all Vue component sections
        for declaration in component.declarations(VueDeclarationCollectionFilter::all()) {
            if let Some(name) = declaration.declaration_name() {
                let key = name.text().to_string();
                key_declarations.entry(key).or_default().push(declaration);
            }
        }

        // Find duplicates
        key_declarations
            .into_iter()
            .filter_map(|(key, declarations)| {
                if declarations.len() > 1 {
                    Some(RuleState { key, declarations })
                } else {
                    None
                }
            })
            .collect::<Box<[_]>>()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut declarations_iterator = state.declarations.iter();
        let first_declaration = declarations_iterator.next()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first_declaration.declaration_name_range()?,
            markup! {
                "Duplicate key "<Emphasis>{&state.key}</Emphasis>" found in Vue component."
            },
        );

        // Add related information for other occurrences
        for declaration in declarations_iterator {
            if let Some(range) = declaration.declaration_name_range() {
                diagnostic = diagnostic.detail(
                    range,
                    markup! {
                        "Key "<Emphasis>{&state.key}</Emphasis>" is also defined here."
                    },
                );
            }
        }

        diagnostic = diagnostic.note(markup! {
            "Keys defined in different Vue component options (props, data, methods, computed) can conflict when accessed in the template. Rename the key to avoid conflicts."
        });

        Some(diagnostic)
    }
}

pub struct RuleState {
    key: String,
    declarations: Vec<VueDeclaration>,
}
