use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{JsCallExpression, JsLanguage, JsReferenceIdentifier, JsVariableDeclarator};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_vue_duplicate_keys::NoVueDuplicateKeysOptions;
use biome_vue_semantic::{AnalyzedVueModel, Origin, Symbol, SymbolKind};
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
    type Query = VueComponent;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoVueDuplicateKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let vue_model = ctx.vue_model();
        let model = ctx.model();

        // The variables that hold the props object, as in `const props = defineProps(['foo'])`.
        let props_objects: Vec<_> = component
            .declarations()
            .filter(|symbol| symbol.origin() == Origin::DefineProps)
            .map(|symbol| symbol.range())
            .collect();

        let mut declarations: Vec<_> = component
            .declarations()
            .filter(|symbol| symbol.parent().is_none())
            .filter(|symbol| match symbol.kind() {
                SymbolKind::Prop
                | SymbolKind::Data
                | SymbolKind::AsyncData
                | SymbolKind::Computed
                | SymbolKind::Method
                | SymbolKind::SetupReturn => true,
                // A setup variable derived from props is the same value as the prop it comes
                // from, so it doesn't compete with it.
                SymbolKind::Local | SymbolKind::Import => {
                    !symbol.is_type_only()
                        && !is_derived_from_props(symbol, vue_model, model, &props_objects)
                }
                _ => false,
            })
            .map(|symbol| (Box::<str>::from(symbol.name()), symbol.range()))
            .collect();
        declarations.sort_by_key(|(_, range)| range.start());

        let mut key_declarations: FxHashMap<Box<str>, Vec<TextRange>> = FxHashMap::default();
        for (name, range) in declarations {
            key_declarations.entry(name).or_default().push(range);
        }
        key_declarations
            .into_iter()
            .filter(|(_, ranges)| ranges.len() > 1)
            .map(|(key, ranges)| RuleState { key, ranges })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_range, other_ranges) = state.ranges.split_first()?;
        let key = &*state.key;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first_range,
            markup! {
                "Duplicate key "<Emphasis>{key}</Emphasis>" found in Vue component."
            },
        );

        // Add related information for other occurrences
        for range in other_ranges {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "Key "<Emphasis>{key}</Emphasis>" is also defined here."
                },
            );
        }

        diagnostic = diagnostic.note(markup! {
            "Keys defined in different Vue component options (props, data, methods, computed) can conflict when accessed in the template. Rename the key to avoid conflicts."
        });

        Some(diagnostic)
    }
}

pub struct RuleState {
    key: Box<str>,
    /// The ranges of the names of the declarations, in source order.
    ranges: Vec<TextRange>,
}

/// Returns `true` when `symbol`, a top-level variable of `<script setup>`, takes its value from
/// the props: it destructures the props, holds the props object, or its initializer calls
/// `defineProps()` or reads a variable in `props_objects`.
///
/// ```js
/// const props = defineProps(['foo']);
/// const foo = toRef(props, 'foo'); // derived from `props`
/// const bar = 42;                  // not derived from `props`
/// ```
fn is_derived_from_props(
    symbol: &Symbol,
    vue_model: &AnalyzedVueModel<JsLanguage>,
    model: &SemanticModel,
    props_objects: &[TextRange],
) -> bool {
    if matches!(
        symbol.origin(),
        Origin::DefineProps | Origin::PropsDestructure
    ) {
        return true;
    }
    let Some(statement) = vue_model.node(symbol) else {
        return false;
    };
    let Some(initializer) = statement
        .descendants()
        .filter_map(JsVariableDeclarator::cast)
        .find(|declarator| declarator.range().contains_range(symbol.range()))
        .and_then(|declarator| declarator.initializer()?.expression().ok())
    else {
        return false;
    };
    initializer.syntax().descendants().any(|node| {
        if let Some(call) = JsCallExpression::cast_ref(&node) {
            return is_vue_compiler_macro_call(&call, model, "defineProps");
        }
        JsReferenceIdentifier::cast(node)
            .and_then(|reference| model.binding(&reference))
            .is_some_and(|binding| props_objects.contains(&binding.syntax().text_trimmed_range()))
    })
}
