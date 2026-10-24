use crate::JsRuleAction;
use crate::services::vue::VueComponent;
use crate::utils::rename::RenameSymbolExtensions;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsIdentifierBinding, JsSyntaxKind};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::use_vue_macro_variable_name::UseVueMacroVariableNameOptions;
use biome_unicode_table::is_js_ident;
use biome_vue_semantic::{ComponentKind, Origin, Symbol};

declare_lint_rule! {
    /// Require specific variable names for the results of `defineProps()`, `defineEmits()`, `defineSlots()`, `useSlots()` and `useAttrs()`.
    ///
    /// Inside `<script setup>`, the results of `defineProps()`, `defineEmits()`, `defineSlots()`,
    /// `useSlots()` and `useAttrs()` are stored in variables. When every component uses the same
    /// names for these variables, components are easier to read, and searching the codebase for
    /// them finds every use.
    ///
    /// The variable that stores `withDefaults(defineProps(...), ...)` is checked like the one
    /// that stores `defineProps()`. Destructured results, such as `const { title } = defineProps()`,
    /// are ignored.
    ///
    /// The fix also renames every use of the variable in the script. It is only offered when Biome
    /// can tell that nothing outside the script uses the variable, such as the template, and when
    /// no other variable in the same component would get the same name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const propsDefined = defineProps({ msg: String });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const emits = defineEmits(["update:msg"]);
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { useAttrs, useSlots } from "vue";
    ///
    /// const props = defineProps({ msg: String });
    /// const emit = defineEmits(["update:msg"]);
    /// const slots = useSlots();
    /// const attrs = useAttrs();
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// Each option sets the required variable name for the result of one macro or function.
    ///
    /// ### `defineProps`
    ///
    /// Default: `"props"`
    ///
    /// The required name of the variable that stores the result of `defineProps()`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "defineProps": "properties"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup>
    /// const props = defineProps({ msg: String });
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// const properties = defineProps({ msg: String });
    /// </script>
    /// ```
    ///
    /// ### `defineEmits`
    ///
    /// Default: `"emit"`
    ///
    /// The required name of the variable that stores the result of `defineEmits()`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "defineEmits": "emits"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup>
    /// const emit = defineEmits(["change"]);
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// const emits = defineEmits(["change"]);
    /// </script>
    /// ```
    ///
    /// ### `defineSlots`
    ///
    /// Default: `"slots"`
    ///
    /// The required name of the variable that stores the result of `defineSlots()`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "defineSlots": "componentSlots"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup lang="ts">
    /// const slots = defineSlots<{ default(): any }>();
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup lang="ts">
    /// const componentSlots = defineSlots<{ default(): any }>();
    /// </script>
    /// ```
    ///
    /// ### `useSlots`
    ///
    /// Default: `"slots"`
    ///
    /// The required name of the variable that stores the result of `useSlots()`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "useSlots": "componentSlots"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup>
    /// import { useSlots } from "vue";
    ///
    /// const slots = useSlots();
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// import { useSlots } from "vue";
    ///
    /// const componentSlots = useSlots();
    /// </script>
    /// ```
    ///
    /// ### `useAttrs`
    ///
    /// Default: `"attrs"`
    ///
    /// The required name of the variable that stores the result of `useAttrs()`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "useAttrs": "attributes"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <script setup>
    /// import { useAttrs } from "vue";
    ///
    /// const attrs = useAttrs();
    /// </script>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script setup>
    /// import { useAttrs } from "vue";
    ///
    /// const attributes = useAttrs();
    /// </script>
    /// ```
    ///
    pub UseVueMacroVariableName {
        version: "next",
        name: "useVueMacroVariableName",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-macro-variable-name").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueMacroVariableName {
    type Query = VueComponent;
    /// A top-level `<script setup>` variable that stores the result of a checked macro under
    /// another name than the configured one.
    type State = Symbol;
    type Signals = Box<[Self::State]>;
    type Options = UseVueMacroVariableNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        if component.kind() != ComponentKind::Setup {
            return Box::default();
        }
        let options = ctx.options();
        component
            .setup_bindings()
            .filter(|symbol| {
                Macro::from_origin(symbol.origin())
                    .is_some_and(|source| symbol.name() != source.expected_name(options))
            })
            .collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, symbol: &Self::State) -> Option<TextRange> {
        Some(symbol.range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, symbol: &Self::State) -> Option<RuleDiagnostic> {
        let source = Macro::from_origin(symbol.origin())?;
        let expected = source.expected_name(ctx.options());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                symbol.range(),
                markup! {
                    "The result of "<Emphasis>{source}</Emphasis>" is stored in "<Emphasis>{symbol.name()}</Emphasis>" instead of "<Emphasis>{expected}</Emphasis>"."
                },
            )
            .note(markup! {
                "Using the same variable name for this result in every component makes components easier to read and search."
            })
            .note(markup! {
                "Rename the variable to "<Emphasis>{expected}</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, symbol: &Self::State) -> Option<JsRuleAction> {
        // The rename only reaches the script, so a use in the template, or a
        // use the model can't follow, would be left with the old name.
        if !symbol.has_complete_uses()
            || symbol
                .uses()
                .any(|reference| reference.snippet() != symbol.snippet())
        {
            return None;
        }
        let options = ctx.options();
        let expected = Macro::from_origin(symbol.origin())?.expected_name(options);
        if !can_be_variable_name(expected) {
            return None;
        }
        // Renaming two variables to the same name, such as the results of
        // `defineSlots()` and `useSlots()`, would declare that name twice
        // once all fixes are applied together.
        if ctx.query().setup_bindings().any(|other| {
            other != *symbol
                && Macro::from_origin(other.origin())
                    .is_some_and(|source| source.expected_name(options) == expected)
        }) {
            return None;
        }
        let binding = ctx
            .root()
            .syntax()
            .covering_element(symbol.range())
            .parent()
            .and_then(JsIdentifierBinding::cast)?;
        let mut mutation = ctx.root().begin();
        if !mutation.rename_node_declaration(ctx.model(), &binding, expected) {
            return None;
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Rename the variable to "<Emphasis>{expected}</Emphasis>"."
            },
            mutation,
        ))
    }
}

/// A macro or function whose result must be stored in a variable with a
/// configured name.
#[derive(Clone, Copy)]
enum Macro {
    DefineProps,
    DefineEmits,
    DefineSlots,
    UseSlots,
    UseAttrs,
}

impl Macro {
    fn from_origin(origin: Origin) -> Option<Self> {
        match origin {
            Origin::DefineProps => Some(Self::DefineProps),
            Origin::DefineEmits => Some(Self::DefineEmits),
            Origin::DefineSlots => Some(Self::DefineSlots),
            Origin::UseSlots => Some(Self::UseSlots),
            Origin::UseAttrs => Some(Self::UseAttrs),
            _ => None,
        }
    }

    fn expected_name(self, options: &UseVueMacroVariableNameOptions) -> &str {
        match self {
            Self::DefineProps => options.define_props(),
            Self::DefineEmits => options.define_emits(),
            Self::DefineSlots => options.define_slots(),
            Self::UseSlots => options.use_slots(),
            Self::UseAttrs => options.use_attrs(),
        }
    }
}

impl biome_console::fmt::Display for Macro {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::DefineProps => "defineProps()",
            Self::DefineEmits => "defineEmits()",
            Self::DefineSlots => "defineSlots()",
            Self::UseSlots => "useSlots()",
            Self::UseAttrs => "useAttrs()",
        })
    }
}

/// Returns `true` if `name` can be declared as a variable in a module, where
/// strict mode reserves words such as `let` and `static`.
fn can_be_variable_name(name: &str) -> bool {
    is_js_ident(name)
        && !JsSyntaxKind::from_keyword(name).is_some_and(|keyword| keyword.is_non_contextual_keyword())
}
