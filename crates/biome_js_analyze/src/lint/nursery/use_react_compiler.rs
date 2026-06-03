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

// TODO: probably useless to have an enum with only one variant. refactor when cleaning up.
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
            Ok(output) => {
                let mut diagnostics = Vec::new();
                for diagnostic in &output.diagnostics {
                    let Some(kind) = diagnostic_kind(diagnostic) else {
                        continue;
                    };
                    let diagnostic = ReactCompilerDiagnostic {
                        range: diagnostic_range(diagnostic).unwrap_or(root_range),
                        kind,
                    };
                    if !diagnostics
                        .iter()
                        .any(|existing| same_diagnostic(existing, &diagnostic))
                    {
                        diagnostics.push(diagnostic);
                    }
                }
                diagnostics
            }
            Err(_) => Vec::new(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match &state.kind {
            ReactCompilerDiagnosticKind::Compiler { detail } => {
                compiler_diagnostic(state.range, detail)
            }
        }
    }
}

fn same_diagnostic(left: &ReactCompilerDiagnostic, right: &ReactCompilerDiagnostic) -> bool {
    left.range == right.range
        && match (&left.kind, &right.kind) {
            (
                ReactCompilerDiagnosticKind::Compiler { detail: left },
                ReactCompilerDiagnosticKind::Compiler { detail: right },
            ) => {
                left.category == right.category
                    && left.reason == right.reason
                    && left.description == right.description
            }
        }
}

fn diagnostic_kind(error: &ReactCompilerError) -> Option<ReactCompilerDiagnosticKind> {
    match error {
        ReactCompilerError::CompilerDiagnostic { detail, .. } => {
            if !is_reportable_compiler_detail(detail) {
                return None;
            }
            Some(ReactCompilerDiagnosticKind::Compiler {
                detail: detail.clone(),
            })
        }
        _ => None,
    }
}

fn is_reportable_compiler_detail(detail: &CompilerErrorDetailInfo) -> bool {
    matches!(
        detail.category.as_str(),
        "Hooks"
            | "CapitalizedCalls"
            | "StaticComponents"
            | "UseMemo"
            | "VoidUseMemo"
            | "PreserveManualMemo"
            | "MemoDependencies"
            | "EffectDependencies"
            | "EffectExhaustiveDependencies"
            | "EffectSetState"
            | "EffectDerivationsOfState"
            | "ErrorBoundaries"
            | "Immutability"
            | "Globals"
            | "Purity"
            | "IncompatibleLibrary"
            | "Refs"
            | "RenderSetState"
    )
}

fn compiler_diagnostic(
    range: TextRange,
    detail: &CompilerErrorDetailInfo,
) -> Option<RuleDiagnostic> {
    let category = detail.category.as_str();
    let reason = detail.reason.as_str();
    match reason {
        "useMemo() callbacks must return a value" => Some(RuleDiagnostic::new(
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
        })),
        "useMemo() result is unused" => Some(RuleDiagnostic::new(
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
        })),
        "useMemo() callbacks may not accept parameters" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This " <Emphasis>"useMemo"</Emphasis> " callback has parameters."
            },
        )
        .note(markup! {
            <Emphasis>"useMemo"</Emphasis> " callbacks are called by React and should read props, state, or local variables directly."
        })
        .note(markup! {
            "Remove the parameters and reference the needed values from the surrounding scope."
        })),
        "useMemo() callbacks may not be async or generator functions" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This " <Emphasis>"useMemo"</Emphasis> " callback is async or a generator."
            },
        )
        .note(markup! {
            <Emphasis>"useMemo"</Emphasis> " must compute its cached value synchronously during render."
        })
        .note(markup! {
            "Make the callback synchronous, or move asynchronous work into an effect or event handler."
        })),
        "useMemo() callbacks may not reassign variables declared outside of the callback" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This " <Emphasis>"useMemo"</Emphasis> " callback reassigns a variable from outside the callback."
            },
        )
        .note(markup! {
            <Emphasis>"useMemo"</Emphasis> " callbacks must be pure computations so React can safely cache their result."
        })
        .note(markup! {
            "Return a computed value from the callback, and move assignments or other side effects outside " <Emphasis>"useMemo"</Emphasis> "."
        })),
        _ if reason.starts_with("Expected a callback function to be passed to ") => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This manual memoization hook is missing an inline callback."
            },
        )
        .note(markup! {
            <Emphasis>"useMemo"</Emphasis> " and " <Emphasis>"useCallback"</Emphasis> " need a callback so React can compute or cache the value."
        })
        .note(markup! {
            "Pass an inline function as the first argument."
        })),
        "Expected the first argument to be an inline function expression" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This manual memoization hook uses a non-inline callback."
            },
        )
        .note(markup! {
            "React Compiler needs to inspect the callback body to preserve existing manual memoization."
        })
        .note(markup! {
            "Pass the callback inline, or remove the manual memoization if it is unnecessary."
        })),
        _ if reason.starts_with("Expected the dependency list for ") => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This dependency list is not an array literal."
            },
        )
        .note(markup! {
            "A dynamic dependency list makes it unclear when React should reuse the previous value or compute a new one."
        })
        .note(markup! {
            "Pass an inline array literal as the dependency list, and move conditional logic outside of the dependency argument."
        })),
        "Expected the dependency list to be an array of simple expressions (e.g. `x`, `x.y.z`, `x?.y?.z`)" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This dependency list contains a complex expression."
            },
        )
        .note(markup! {
            "React dependencies should name the values that affect the memoized computation. Complex expressions can hide what actually changed."
        })
        .note(markup! {
            "Move the expression into a local variable, then include that variable or a simple property chain in the dependency list."
        })),
        _ if category == "Hooks" => Some(RuleDiagnostic::new(
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
        })),
        _ if category == "CapitalizedCalls" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This capitalized function is called directly."
            },
        )
        .note(markup! {
            "React treats capitalized functions as components, and components should be rendered with JSX so their hooks and lifecycle are handled correctly."
        })
        .note(markup! {
            "Render it as JSX, rename it to start with a lowercase letter, or allowlist it in the React Compiler configuration if it is not a component."
        })),
        _ if category == "StaticComponents" => Some(RuleDiagnostic::new(
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
        })),
        _ if category == "Immutability" => Some(RuleDiagnostic::new(
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
        })),
        _ if category == "Globals" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This component reassigns a variable declared outside it."
            },
        )
        .note(markup! {
            "Changing external variables during render makes component output depend on mutable shared state."
        })
        .note(markup! {
            "Keep render logic local and immutable, or move shared updates into state, refs, or event handlers."
        })),
        _ if category == "Purity" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This render calls an impure function."
            },
        )
        .note(markup! {
            "Render should be pure so React can safely repeat, interrupt, and optimize it."
        })
        .note(markup! {
            "Move the impure work into an effect or event handler, or replace it with a pure calculation."
        })),
        _ if category == "IncompatibleLibrary" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This code uses a library pattern that React Compiler cannot optimize safely."
            },
        )
        .note(markup! {
            "Some libraries rely on mutable or runtime behavior that conflicts with React Compiler's assumptions."
        })
        .note(markup! {
            "Avoid this library pattern in compiled components, or opt this component out of React Compiler."
        })),
        _ if category == "Refs" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This render reads or writes a ref."
            },
        )
        .note(markup! {
            "Refs are mutable escape hatches. Accessing them during render can make output depend on mutable state outside React's render model."
        })
        .note(markup! {
            "Read or write refs in effects, event handlers, or callbacks instead of during render."
        })),
        _ if category == "RenderSetState" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This render synchronously updates state."
            },
        )
        .note(markup! {
            "Updating state during render can cause cascading renders or infinite loops."
        })
        .note(markup! {
            "Derive the value during render, initialize state lazily, or move the update into an event handler or effect."
        })),
        _ if category == "MemoDependencies" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This memoization dependency list is incomplete or contains unnecessary values."
            },
        )
        .note(markup! {
            "Missing dependencies can produce stale values, while extra dependencies can cause avoidable recomputation."
        })
        .note(markup! {
            "Include every value used by the memoized callback, and remove values that do not affect it."
        })),
        _ if category == "EffectExhaustiveDependencies" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This effect dependency list is incomplete or contains unnecessary values."
            },
        )
        .note(markup! {
            "Missing dependencies can make effects observe stale values, while extra dependencies can make effects run more often than needed."
        })
        .note(markup! {
            "Include every value used by the effect, and remove values that do not affect it."
        })),
        _ if category == "EffectDependencies" => Some(RuleDiagnostic::new(
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
        })),
        _ if category == "EffectSetState" => Some(RuleDiagnostic::new(
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
        })),
        _ if category == "EffectDerivationsOfState" => Some(RuleDiagnostic::new(
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
        })),
        _ if category == "PreserveManualMemo" => Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "React Compiler cannot preserve this existing manual memoization."
            },
        )
        .note(markup! {
            "The memoized value depends on patterns that prevent React Compiler from proving the memoization is equivalent after optimization."
        })
        .note(markup! {
            "Simplify the memoized callback and dependency list, or remove the manual memoization if it is unnecessary."
        })),
        _ if category == "ErrorBoundaries" => Some(RuleDiagnostic::new(
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
        })),
        _ => None,
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
