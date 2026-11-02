use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier, JsCallExpression,
    JsDefaultImportSpecifier, JsIdentifierBinding, JsImport, JsNamespaceImportSpecifier,
    inner_string_text,
};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_vue_restricted_call_after_await::{
    NoVueRestrictedCallAfterAwaitOptions, RestrictedFunction,
};

use crate::frameworks::vue::vue_component::{VueComponent, VueComponentQuery};
use crate::frameworks::vue::vue_setup::{
    filter_map_vue_setup_calls_after_await, vue_setup_function_body,
};

declare_lint_rule! {
    /// Disallow calling configured functions after an `await` in `setup()`.
    ///
    /// Some functions, such as `useI18n()` from `vue-i18n`, need to know which component is
    /// being set up. Vue only keeps track of that while `setup()` runs without pausing. Once
    /// `setup()` pauses at an `await`, these functions can no longer find the component, so they
    /// fail or behave incorrectly.
    ///
    /// This rule reports calls to the functions listed in the [`functions`](#functions) option
    /// that come after an `await` in a component's `setup()` function. It does nothing until you
    /// list at least one function.
    ///
    /// Code inside `<script setup>` isn't checked, because Vue keeps track of the component
    /// across each `await` there.
    ///
    /// ## Examples
    ///
    /// The examples below use the following configuration:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": [{ "module": "vue-i18n", "path": "useI18n" }]
    ///     }
    /// }
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script>
    /// import { useI18n } from "vue-i18n";
    ///
    /// export default {
    ///   async setup() {
    ///     await fetchUser();
    ///     const { t } = useI18n();
    ///     return { t };
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,use_options
    /// <script>
    /// import { useI18n } from "vue-i18n";
    ///
    /// export default {
    ///   async setup() {
    ///     const { t } = useI18n();
    ///     await fetchUser();
    ///     return { t };
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `functions`
    ///
    /// Default: `[]`
    ///
    /// The functions that must not be called after an `await` in `setup()`. Each entry is an
    /// object with these properties:
    ///
    /// - `module`: The module that exports the function, written the same way as in the `import`
    ///   statement. This property is required.
    /// - `path`: The name the function is exported as. Use a list of names when the function is a
    ///   property of an exported object: `["bar", "baz"]` matches `bar.baz()` for
    ///   `import { bar } from "..."`. Defaults to `"default"`, which matches the module's default
    ///   export.
    /// - `message`: A custom message to show in place of the default message.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": [
    ///             { "module": "vue-i18n", "path": "useI18n" },
    ///             { "module": "my-store", "path": ["store", "use"], "message": "Call store.use() before the first await." },
    ///             { "module": "my-plugin" }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script>
    /// import { store } from "my-store";
    ///
    /// export default {
    ///   async setup() {
    ///     await fetchUser();
    ///     store.use();
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script>
    /// import usePlugin from "my-plugin";
    ///
    /// export default {
    ///   async setup() {
    ///     await fetchUser();
    ///     usePlugin();
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - [`noVueLifecycleAfterAwait`](https://biomejs.dev/linter/rules/no-vue-lifecycle-after-await/) reports Vue lifecycle hooks, such as `onMounted()`, that are registered after an `await` in `setup()`.
    ///
    pub NoVueRestrictedCallAfterAwait {
        version: "2.6.5",
        name: "noVueRestrictedCallAfterAwait",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-restricted-call-after-await").same()],
    }
}

impl Rule for NoVueRestrictedCallAfterAwait {
    type Query = VueComponentQuery;
    type State = RestrictedCallAfterAwait;
    type Signals = Box<[Self::State]>;
    type Options = NoVueRestrictedCallAfterAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let functions = ctx.options().functions();
        if functions.is_empty() {
            return [].into();
        }
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return [].into();
        };
        let Some(body) = vue_setup_function_body(component.kind()) else {
            return [].into();
        };

        filter_map_vue_setup_calls_after_await(&body, |call, first_await| {
            let function = find_restricted_function(&call, functions, ctx.model())?;
            Some(RestrictedCallAfterAwait {
                call,
                first_await,
                function,
            })
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let function = ctx.options().functions().get(state.function)?;
        let message = match &function.message {
            Some(message) => markup! { {message} }.to_owned(),
            None => markup! {
                "This call to "<Emphasis>{FunctionName(function)}</Emphasis>" from "<Emphasis>{function.module}</Emphasis>" comes after an "<Emphasis>"await"</Emphasis>" in "<Emphasis>"setup()"</Emphasis>"."
            }
            .to_owned(),
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.call.range(), message)
                .detail(
                    state.first_await,
                    markup! {
                        "The "<Emphasis>"setup()"</Emphasis>" function pauses here. After this point, Vue no longer knows which component is being set up."
                    },
                )
                .note(markup! {
                    "This function is listed in the "<Emphasis>"functions"</Emphasis>" option because it needs to know which component is being set up. Call it before the first "<Emphasis>"await"</Emphasis>" in "<Emphasis>"setup()"</Emphasis>"."
                }),
        )
    }
}

pub struct RestrictedCallAfterAwait {
    call: JsCallExpression,
    /// The range of the first `await` keyword in `setup()` that pauses before the call.
    first_await: TextRange,
    /// The index of the matching entry in the `functions` option.
    function: usize,
}

/// Displays the function of a [`RestrictedFunction`] as it is called, such as `bar.baz()`.
struct FunctionName<'a>(&'a RestrictedFunction);

impl biome_console::fmt::Display for FunctionName<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for (index, name) in self.0.path.names().iter().enumerate() {
            if index > 0 {
                fmt.write_str(".")?;
            }
            fmt.write_str(name)?;
        }
        fmt.write_str("()")
    }
}

/// How a binding is imported.
enum ImportedName {
    /// `import { name } from "module"` or `import { name as local } from "module"`.
    Named(TokenText),
    /// `import local from "module"`.
    Default,
    /// `import * as local from "module"`.
    Namespace,
}

/// Returns the index of the entry in `functions` that `call` calls, if any.
fn find_restricted_function(
    call: &JsCallExpression,
    functions: &[RestrictedFunction],
    model: &SemanticModel,
) -> Option<usize> {
    let callee = call.callee().ok()?.inner_expression()?;

    // Find the identifier at the start of the callee, such as `bar` in `bar.baz.qux()`.
    let mut member_count = 0;
    let mut object = callee.clone();
    while let Some(member) = AnyJsMemberExpression::cast_ref(object.syntax()) {
        object = member.object().ok()?.inner_expression()?;
        member_count += 1;
    }
    let binding = model.binding(&object.as_js_reference_identifier()?)?;
    let (source, imported_name) = imported_binding(&binding)?;

    functions.iter().position(|function| {
        if function.module.as_ref() != source.text() {
            return false;
        }
        let names = function.path.names();
        let member_names = match &imported_name {
            ImportedName::Namespace => names,
            ImportedName::Named(imported_name) => match names.split_first() {
                Some((first, rest)) if first.as_ref() == imported_name.text() => rest,
                _ => return false,
            },
            ImportedName::Default => match names.split_first() {
                Some((first, rest)) if first.as_ref() == "default" => rest,
                _ => return false,
            },
        };
        member_names.len() == member_count && callee_has_member_names(&callee, member_names)
    })
}

/// Returns the source of the import that declares `binding`, along with how it is imported.
fn imported_binding(binding: &Binding) -> Option<(TokenText, ImportedName)> {
    let identifier = JsIdentifierBinding::cast(binding.syntax().clone())?;
    let parent = identifier.syntax().parent()?;
    let imported_name = if let Some(specifier) = AnyJsNamedImportSpecifier::cast_ref(&parent) {
        if specifier.imports_only_types() {
            return None;
        }
        ImportedName::Named(inner_string_text(&specifier.imported_name()?))
    } else if JsDefaultImportSpecifier::can_cast(parent.kind()) {
        ImportedName::Default
    } else if JsNamespaceImportSpecifier::can_cast(parent.kind()) {
        ImportedName::Namespace
    } else {
        return None;
    };
    let import = parent.ancestors().find_map(JsImport::cast)?;
    if import.import_clause().ok()?.type_token().is_some() {
        return None;
    }
    Some((import.source_text().ok()?, imported_name))
}

/// Returns `true` if the members accessed in `callee` are `names`. For example, `ns.bar.baz` has
/// the members `["bar", "baz"]`.
fn callee_has_member_names(callee: &AnyJsExpression, names: &[Box<str>]) -> bool {
    let mut expression = callee.clone();
    for name in names.iter().rev() {
        let Some(member) = AnyJsMemberExpression::cast(expression.into_syntax()) else {
            return false;
        };
        if member
            .member_name()
            .is_none_or(|member_name| member_name.text() != name.as_ref())
        {
            return false;
        }
        let Some(object) = member.object().ok().and_then(|object| object.inner_expression())
        else {
            return false;
        };
        expression = object;
    }
    true
}
