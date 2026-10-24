use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rule_options::no_vue_template_shadow::NoVueTemplateShadowOptions;
use biome_vue_semantic::{Namespace, Symbol, SymbolId, SymbolKind};

declare_lint_rule! {
    /// Disallow template variables that reuse the name of a variable from an outer scope.
    ///
    /// `v-for` and `v-slot` create variables that only exist inside their element. When
    /// one of these variables has the same name as a variable the template can already
    /// see, the new variable hides the old one. Inside the element, the name always refers
    /// to the new variable, so the outer value can't be used there, and readers can easily
    /// confuse the two.
    ///
    /// The variables that can be hidden are:
    ///
    /// - variables created by an enclosing `v-for` or `v-slot`;
    /// - top-level variables and imports of `<script setup>`;
    /// - what the component exposes to its template: props, models created with
    ///   `defineModel()`, `data`, `asyncData`, computed properties, methods, injected
    ///   values, and the values returned by `setup()`.
    ///
    /// Types declared in `<script setup lang="ts">` are not variables, so a template
    /// variable can reuse their names.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-for="item in groups">
    ///     <span v-for="item in item.children">{{ item }}</span>
    ///   </div>
    /// </template>
    /// ```
    ///
    /// ```vue,ignore
    /// <script setup>
    /// const item = "selected";
    /// </script>
    ///
    /// <template>
    ///   <div v-for="item in items">{{ item }}</div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-for="group in groups">
    ///     <span v-for="child in group.children">{{ child }}</span>
    ///   </div>
    ///   <div v-for="group in otherGroups">{{ group }}</div>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allow`
    ///
    /// Type: `string[]`
    ///
    /// Default: `[]`
    ///
    /// A list of names that template variables may reuse. A variable with one of these
    /// names is never reported.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allow": ["item"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid (using `allow`)
    ///
    /// ```vue,use_options
    /// <template>
    ///   <div v-for="item in groups">
    ///     <span v-for="item in item.children">{{ item }}</span>
    ///   </div>
    /// </template>
    /// ```
    ///
    pub NoVueTemplateShadow {
        version: "next",
        name: "noVueTemplateShadow",
        language: "html",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-template-shadow").same()],
    }
}

impl Rule for NoVueTemplateShadow {
    type Query = VueSemantic<HtmlRoot>;
    type State = Shadowing;
    type Signals = Box<[Self::State]>;
    type Options = NoVueTemplateShadowOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let allow = ctx.options().allow();
        ctx.model()
            .symbols()
            .filter(|symbol| TEMPLATE_VARIABLE_KINDS.contains(&symbol.kind()))
            .filter(|symbol| !allow.iter().any(|name| name.as_ref() == symbol.name()))
            .filter_map(|variable| {
                let shadowed = earlier_in_same_scope(&variable)
                    .or_else(|| visible_from_outer_scopes(&variable))?;
                Some(Shadowing {
                    variable: variable.id(),
                    shadowed: shadowed.id(),
                })
            })
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let model = ctx.model();
        let variable = model.symbol(state.variable);
        let shadowed = model.symbol(state.shadowed);
        let name = variable.name();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                variable.host_range(),
                markup! {
                    "The template variable "<Emphasis>{name}</Emphasis>" hides a variable with the same name."
                },
            )
            .detail(
                shadowed.host_range(),
                markup! {
                    "The hidden "<Emphasis>{name}</Emphasis>" is declared here."
                },
            )
            .note(markup! {
                "Inside this element, "<Emphasis>{name}</Emphasis>" refers to the template variable, so the other "<Emphasis>{name}</Emphasis>" can't be used, and the two are easy to confuse."
            })
            .note(markup! {
                "Rename the template variable."
            }),
        )
    }
}

/// A template variable, and the variable with the same name it hides.
pub struct Shadowing {
    variable: SymbolId,
    shadowed: SymbolId,
}

/// Returns a variable with the same name declared before `variable` by the same
/// element, as in `v-for="(item, item) in items"`.
fn earlier_in_same_scope(variable: &Symbol) -> Option<Symbol> {
    variable
        .scope()
        .symbols()
        .take_while(|symbol| symbol.id() != variable.id())
        .find(|symbol| symbol.name() == variable.name())
}

/// Returns the innermost variable with the same name as `variable` that the
/// template can use outside the element declaring `variable`.
///
/// Types and the names Vue provides to every template, such as `$emit`, are
/// skipped: a type can't be used as a value, and a name starting with `$`
/// isn't a variable the user declared.
fn visible_from_outer_scopes(variable: &Symbol) -> Option<Symbol> {
    variable.scope().parent()?.ancestors().find_map(|scope| {
        scope.symbols().find(|symbol| {
            symbol.namespace() == Namespace::Binding
                && symbol.name() == variable.name()
                && symbol.kind() != SymbolKind::Builtin
                && !symbol.is_type_only()
        })
    })
}

/// The kinds of variables a template declares.
const TEMPLATE_VARIABLE_KINDS: [SymbolKind; 2] = [SymbolKind::VForAlias, SymbolKind::SlotParam];
