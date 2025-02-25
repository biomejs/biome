use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyJsParameters, AnyParameter};
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern,
    AnyJsObjectBindingPatternMember, JsIdentifierBinding,
};
use biome_rowan::AstNode;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    ///  Disallow duplicate function parameter name.
    ///
    /// If more than one parameter has the same name in a function definition,
    /// the last occurrence overrides the preceding occurrences.
    /// A duplicated name might be a typing error.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var f = function(a, b, b) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function b(a, b, b) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function i(i, b, c) {}
    /// var j = function (j, b, c) {};
    /// function k({ k, b }, { c, d }) {}
    /// function l([, l]) {}
    /// function foo([[a, b], [c, d]]) {}
    /// ```
    pub NoDuplicateParameters {
        version: "1.0.0",
        name: "noDuplicateParameters",
        language: "js",
        sources: &[RuleSource::Eslint("no-dupe-args")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoDuplicateParameters {
    type Query = Ast<AnyJsParameters>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let parameters = ctx.query();

        let list = match parameters {
            AnyJsParameters::JsParameters(parameters) => {
                AnyJsParameterList::from(parameters.items())
            }
            AnyJsParameters::JsConstructorParameters(parameters) => {
                AnyJsParameterList::from(parameters.parameters())
            }
        };

        let mut set = FxHashSet::default();
        // Traversing the parameters of the function in preorder and checking for duplicates,
        list.iter().find_map(|parameter| {
            let parameter = parameter.ok()?;
            traverse_parameter(&parameter, &mut set)
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding_syntax_node = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                binding_syntax_node.syntax().text_trimmed_range(),
                markup! {
                    "Duplicate parameter name."
                },
            )
            .note("The parameter overrides a preceding parameter by using the same name."),
        )
    }
}

/// Traverse the parameter recursively and check if it is duplicated.
/// Return `Some(JsIdentifierBinding)` if it is duplicated.
fn traverse_parameter(
    parameter: &AnyParameter,
    tracked_bindings: &mut FxHashSet<String>,
) -> Option<JsIdentifierBinding> {
    parameter
        .binding()
        .and_then(|binding| traverse_binding(binding, tracked_bindings))
}

/// Traverse a [JsAnyBindingPattern] in preorder and check if the name of [JsIdentifierBinding] has seem before.
/// If true then add the [JsIdentifierBinding] to the [duplicated_arguments].
/// If false then add the [JsIdentifierBinding] to the [tracked_bindings], mark it name as seen.
/// If it is not a [JsIdentifierBinding] then recursively call [traverse_binding] on its children.
fn traverse_binding(
    binding: AnyJsBindingPattern,
    tracked_bindings: &mut FxHashSet<String>,
) -> Option<JsIdentifierBinding> {
    match binding {
        AnyJsBindingPattern::AnyJsBinding(inner_binding) => match inner_binding {
            AnyJsBinding::JsIdentifierBinding(id_binding) => {
                if track_binding(&id_binding, tracked_bindings) {
                    return Some(id_binding);
                }
            }
            AnyJsBinding::JsBogusBinding(_) | AnyJsBinding::JsMetavariable(_) => {}
        },
        AnyJsBindingPattern::JsArrayBindingPattern(inner_binding) => {
            return inner_binding.elements().into_iter().find_map(|element| {
                let element = element.ok()?;
                match element {
                    AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                        binding_rest,
                    ) => {
                        let binding_pattern = binding_rest.pattern().ok()?;
                        traverse_binding(binding_pattern, tracked_bindings)
                    }
                    AnyJsArrayBindingPatternElement::JsArrayHole(_) => None,
                    AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                        binding_with_default,
                    ) => {
                        let pattern = binding_with_default.pattern().ok()?;
                        traverse_binding(pattern, tracked_bindings)
                    }
                }
            })
        }

        AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
            return pattern.properties().into_iter().find_map(|prop| {
                let prop = prop.ok()?;
                match prop {
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(pattern) => {
                        let pattern = pattern.pattern().ok()?;
                        traverse_binding(pattern, tracked_bindings)
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        let pattern = rest.binding().ok()?;
                        match pattern {
                            AnyJsBinding::JsIdentifierBinding(binding) => {
                                track_binding(&binding, tracked_bindings).then_some(binding)
                            }
                            AnyJsBinding::JsBogusBinding(_) | AnyJsBinding::JsMetavariable(_) => {
                                None
                            }
                        }
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        shorthand_binding,
                    ) => match shorthand_binding.identifier().ok()? {
                        AnyJsBinding::JsIdentifierBinding(id_binding) => {
                            track_binding(&id_binding, tracked_bindings).then_some(id_binding)
                        }
                        AnyJsBinding::JsBogusBinding(_) | AnyJsBinding::JsMetavariable(_) => None,
                    },
                    AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                    | AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
                }
            })
        }
    }
    None
}

#[inline]
/// If the name of binding has been seen in set, then we push the `JsIdentifierBinding` into `identifier_vec`.
/// Else we mark the name of binding as seen.
fn track_binding(
    id_binding: &JsIdentifierBinding,
    tracked_bindings: &mut FxHashSet<String>,
) -> bool {
    let binding_text = id_binding.to_trimmed_string();
    if tracked_bindings.contains(&binding_text) {
        true
    } else {
        tracked_bindings.insert(binding_text);
        false
    }
}
