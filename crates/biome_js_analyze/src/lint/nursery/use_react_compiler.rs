use crate::services::react_compiler::ReactCompilerServices;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::TextRange;
use biome_react_compiler::{CompilerErrorDetailInfo, ReactCompilerError};
use biome_rule_options::use_react_compiler::UseReactCompilerOptions;

declare_lint_rule! {
    /// Validate files with React Compiler.
    ///
    /// This rule runs React Compiler in lint mode and reports any compiler diagnostics.
    /// React Compiler validates whether components and hooks can be safely compiled.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { useState } from "react";
    ///
    /// function Component(props) {
    ///     if (props.enabled) {
    ///         useState(0);
    ///     }
    ///
    ///     return <div />;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function Component(props) {
    ///     return <div>{props.value}</div>;
    /// }
    /// ```
    pub UseReactCompiler {
        version: "next",
        name: "useReactCompiler",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::React],
    }
}

pub struct ReactCompilerDiagnostic {
    range: TextRange,
    kind: ReactCompilerDiagnosticKind,
}

pub enum ReactCompilerDiagnosticKind {
    Compiler { detail: CompilerErrorDetailInfo },
}

impl Rule for UseReactCompiler {
    type Query = ReactCompilerServices;
    type State = ReactCompilerDiagnostic;
    type Signals = Vec<Self::State>;
    type Options = UseReactCompilerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let root_range = query.range;

        match query.result.as_ref() {
            Ok(output) => output
                .diagnostics
                .iter()
                .filter_map(|diagnostic| {
                    Some(ReactCompilerDiagnostic {
                        range: diagnostic_range(diagnostic).unwrap_or(root_range),
                        kind: diagnostic_kind(diagnostic)?,
                    })
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match &state.kind {
            ReactCompilerDiagnosticKind::Compiler { detail } => compiler_diagnostic(state.range, detail),
        })
    }
}

fn diagnostic_kind(error: &ReactCompilerError) -> Option<ReactCompilerDiagnosticKind> {
    match error {
        ReactCompilerError::CompilerDiagnostic { detail, .. } => {
            Some(ReactCompilerDiagnosticKind::Compiler {
                detail: detail.clone(),
            })
        }
        _ => None,
    }
}

fn compiler_diagnostic(range: TextRange, detail: &CompilerErrorDetailInfo) -> RuleDiagnostic {
    let category = detail.category.as_str();
    let reason = detail.reason.as_str();
    match reason {
        "useMemo() callbacks must return a value" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This " <Emphasis>"useMemo"</Emphasis> " callback does not return a value."
            },
        )
        .note(markup! {
            <Emphasis>"useMemo"</Emphasis> " is for computing and caching a value, so its callback must return the value to memoize."
        })
        .note(markup! {
            "Return the computed value from the callback, or use " <Emphasis>"useEffect"</Emphasis> " if this code only performs a side effect."
        }),
        "useMemo() result is unused" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This " <Emphasis>"useMemo"</Emphasis> " result is unused."
            },
        )
        .note(markup! {
            <Emphasis>"useMemo"</Emphasis> " should compute a value that is used during render. An unused memo usually means the callback is being used for a side effect."
        })
        .note(markup! {
            "Use the memoized value, remove the " <Emphasis>"useMemo"</Emphasis> " call, or move side effects into " <Emphasis>"useEffect"</Emphasis> "."
        }),
        _ if category == "Hooks" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This hook usage does not follow React's rules."
            },
        )
        .note(markup! {
            "Hooks must be called directly at the top level of a component or custom hook, and in the same order on every render."
        })
        .note(markup! {
            "Move the hook call to the top level, and call a statically known hook directly instead of passing, storing, or selecting it dynamically."
        }),
        _ if category == "Immutability" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This component mutates a value that React treats as immutable."
            },
        )
        .note(markup! {
            "Props, state, and values derived from hooks should be treated as immutable during render. Mutating them can make renders inconsistent."
        })
        .note(markup! {
            "Create a new value instead of mutating this one, or move the update into state with the appropriate setter."
        }),
        _ if category == "MemoDependencies" || category == "EffectDependencies" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This hook dependency list is not an array literal."
            },
        )
        .note(markup! {
            "A dynamic dependency list makes it unclear when React should reuse the previous value or compute a new one."
        })
        .note(markup! {
            "Pass an inline array literal as the dependency list, and move conditional logic outside of the dependency argument."
        }),
        _ if category == "EffectSetState" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This effect synchronously updates state."
            },
        )
        .note(markup! {
            "Updating state directly inside an effect can trigger cascading renders and hurt performance. Effects should synchronize with external systems instead."
        })
        .note(markup! {
            "Derive the value during render, initialize state lazily, or update state from an external subscription callback instead."
        }),
        _ if category == "EffectDerivationsOfState" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This effect derives state that can be computed during render."
            },
        )
        .note(markup! {
            "Using an effect for derived data causes an extra render and can briefly show stale values."
        })
        .note(markup! {
            "Compute the derived value during render instead of storing it in state from an effect."
        }),
        _ if category == "StaticComponents" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This component is created during render."
            },
        )
        .note(markup! {
            "Components created during render are recreated on every render, which resets their state and prevents stable optimization."
        })
        .note(markup! {
            "Declare the component outside of the rendering component, or pass data through props instead."
        }),
        _ if category == "ErrorBoundaries" => RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This code constructs JSX inside " <Emphasis>"try"</Emphasis> "/" <Emphasis>"catch"</Emphasis> "."
            },
        )
        .note(markup! {
            "Creating JSX does not render the component immediately, so render errors from that JSX will not be caught by the surrounding " <Emphasis>"try"</Emphasis> "/" <Emphasis>"catch"</Emphasis> "."
        })
        .note(markup! {
            "Use an error boundary to catch render errors instead of wrapping JSX creation in " <Emphasis>"try"</Emphasis> "/" <Emphasis>"catch"</Emphasis> "."
        }),
        _ => {
            let diagnostic = RuleDiagnostic::new(rule_category!(), range, markup! { {reason} });
            let diagnostic = if let Some(description) = detail.description.as_deref() {
                diagnostic.note(markup! { {description} })
            } else {
                diagnostic.note(markup! {
                    "React Compiler could not prove that this code can be safely optimized."
                })
            };
            diagnostic.note(markup! {
                "Update the highlighted code to satisfy React Compiler's validation."
            })
        }
    }
}

fn diagnostic_range(error: &ReactCompilerError) -> Option<TextRange> {
    match error {
        ReactCompilerError::UnsupportedSyntax { range, .. }
        | ReactCompilerError::InvalidLiteral { range, .. } => Some(*range),
        ReactCompilerError::CompilerDiagnostic { range, .. } => *range,
        ReactCompilerError::MissingSyntax { .. } | ReactCompilerError::CompilerOutput(_) => None,
    }
}
