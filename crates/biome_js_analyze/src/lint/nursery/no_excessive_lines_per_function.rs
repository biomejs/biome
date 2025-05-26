use crate::services::semantic::Semantic;
use ::serde::{Deserialize, Serialize};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsFunction, JsCallExpression, JsParenthesizedExpression,
};
use biome_rowan::AstNode;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use std::num::NonZeroU8;

declare_lint_rule! {
    /// Restrict a maximum number of lines of code in a function.
    ///
    /// Some people consider large functions a code smell. Large functions tend to do a lot of things and can make it hard following what’s going on. Many coding style guides dictate a limit of the number of lines that a function can comprise of. This rule can help enforce that style.
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "max": 4,
    ///        "skipBlankLines": true,
    ///        "iifes": true
    ///     }
    /// }
    /// ```
    ///
    /// - `max` (positive integer, default: 50): The maximum number of lines allowed in a function.
    /// - `skip_blank_lines` (bool, default: false): A boolean value which indicates whether blank lines are counted or not.
    /// - `iifes` (bool, default: false): A boolean value which indicates whether IIFEs (Immediately Invoked Function Expression) are checked or not.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// function foo () {
    ///   const x = 0;
    ///   const y = 1;
    ///   const z = 2;
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const bar = () => {
    ///   const x = 0;
    ///
    ///   const y = 1;
    ///   const z = 2;
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// class Baz {
    ///	  foo() {
    ///     const x = 0;
    ///	    const y = 0;
    ///     const z = 2;
    ///	  };
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// (() => {
    ///	 const x = 0;
    ///	 const y = 0;
    ///	 const z = 0;
    /// })();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    ///  function foo () {
    ///     const x = 0;
    ///     const y = 1;
    /// };
    ///
    /// const bar = () => {
    ///   const x = 0;
    ///
    ///   const y = 1;
    /// };
    ///
    /// class Baz {
    ///	  foo() {
    ///     const x = 0;
    ///	    const y = 0;
    ///	  };
    /// };
    ///
    /// (() => {
    ///	 const x = 0;
    ///	 const y = 0;
    /// })();
    /// ```
    ///
    pub NoExcessiveLinesPerFunction {
        version: "2.0.0",
        name: "noExcessiveLinesPerFunction",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("max-lines-per-function")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for NoExcessiveLinesPerFunction {
    type Query = Semantic<AnyFunctionLike>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveLinesPerFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let options = ctx.options();

        if let AnyFunctionLike::AnyJsFunction(func) = binding {
            if is_iife(func) && !options.iifes {
                return None;
            }
        };

        let func_string = binding.to_string();
        let func_lines = func_string.trim().lines();

        let function_line_count = if options.skip_blank_lines {
            func_lines
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .len()
        } else {
            func_lines.collect::<Vec<_>>().len()
        };

        if function_line_count <= options.max.get().into() {
            return None;
        }

        Some(State {
            function_line_count,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let options = ctx.options();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This function has too many lines ("{state.function_line_count}"). Maximum allowed is "{options.max.to_string()}"."
                },
            )
            .note(markup! {
                "Consider refactoring this function to split it into smaller functions."
            }),
        )
    }
}

fn is_iife(func: &AnyJsFunction) -> bool {
    func.parent::<JsParenthesizedExpression>()
        .and_then(|expr| expr.parent::<JsCallExpression>())
        .is_some()
}

pub struct State {
    function_line_count: usize,
}

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveLinesPerFunctionOptions {
    pub max: NonZeroU8,
    pub skip_blank_lines: bool,
    pub iifes: bool,
}

impl Default for NoExcessiveLinesPerFunctionOptions {
    fn default() -> Self {
        Self {
            max: NonZeroU8::new(50).unwrap(),
            skip_blank_lines: false,
            iifes: false,
        }
    }
}
