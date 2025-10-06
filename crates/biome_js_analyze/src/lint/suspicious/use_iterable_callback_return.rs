use crate::ControlFlowGraph;
use crate::services::control_flow::JsControlFlowGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_control_flow::builder::ROOT_BLOCK_ID;
use biome_control_flow::{ExceptionHandlerKind, InstructionKind};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsFunctionBody, JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsFunctionExpression, JsReturnStatement, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, NodeOrToken, TextRange};
use biome_rule_options::use_iterable_callback_return::UseIterableCallbackReturnOptions;
use roaring::RoaringBitmap;
use rustc_hash::FxHashMap;
use std::io;
use std::sync::LazyLock;

declare_lint_rule! {
    /// Enforce consistent return values in iterable callbacks.
    ///
    /// This rule ensures that callbacks passed to certain iterable methods either always return a
    /// value or never return a value, depending on the method's requirements.
    ///
    /// Note that async and generator callbacks are ignored as they always return `Promise` or
    /// `Generator` respectively.
    ///
    /// ## Methods and Their Requirements
    ///
    /// The following methods require a return in their callback:
    ///
    /// - `every`
    /// - `filter`
    /// - `find`
    /// - `findIndex`
    /// - `findLast`
    /// - `findLastIndex`
    /// - `flatMap`
    /// - `map`
    /// - `reduce`
    /// - `reduceRight`
    /// - `some`
    /// - `sort`
    /// - `toSorted`
    /// — `from` (when called on `Array`)
    ///
    /// A return value is disallowed in the method `forEach`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// [].map(() => {
    ///     // Missing return value
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// [].forEach(() => {
    ///     return 1; // Should not return a value
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// [].map(() => {
    ///     return 1; // Correctly returns a value
    /// });
    /// ```
    ///
    /// ```js
    /// [].forEach(() => {
    ///     // No return value, which is correct
    /// });
    /// ```
    ///
    /// ```js
    /// [].forEach(() => void null); // Void return value, which doesn't trigger the rule
    /// ```
    pub UseIterableCallbackReturn {
        version: "2.0.0",
        name: "useIterableCallbackReturn",
        language: "js",
        sources: &[RuleSource::Eslint("array-callback-return").same()],
        severity: Severity::Error,
        recommended: true,
    }
}

impl Rule for UseIterableCallbackReturn {
    type Query = ControlFlowGraph;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseIterableCallbackReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();

        if let Some(function) = JsFunctionExpression::cast_ref(&cfg.node) {
            // Async and generator function callbacks are ignored.
            if function.async_token().is_some() || function.star_token().is_some() {
                return None;
            }
        } else if let Some(function) = JsArrowFunctionExpression::cast_ref(&cfg.node) {
            // Async arrow callbacks are ignored.
            if function.async_token().is_some() {
                return None;
            }
        } else {
            return None;
        }

        let parent_node = cfg.node.parent()?;
        let argument_list = JsCallArgumentList::cast_ref(&parent_node)?;
        let call_expression = argument_list
            .parent::<JsCallArguments>()?
            .parent::<JsCallExpression>()?;

        let callee = call_expression.callee().ok()?;

        let member_expression = callee.as_js_static_member_expression()?;
        let member_name = member_expression
            .member()
            .ok()
            .and_then(|member| member.as_js_name().cloned())
            .and_then(|name| name.value_token().ok())?;

        let method_config = ITERABLE_METHOD_INFOS.get(member_name.text_trimmed())?;

        let arg_position = argument_list
            .elements()
            .position(|arg| arg.node.is_ok() && arg.node.unwrap().syntax().eq(&cfg.node))?;

        if arg_position != method_config.callback_argument_position {
            return None;
        }

        if let Some(global_name) = method_config.global_name {
            let (_, name) = global_identifier(&member_expression.object().ok()?)?;
            if name.text() != global_name {
                return None;
            }
        }

        let returns_info = get_function_returns_info(cfg);

        let mut problems: Vec<RuleProblemKind> = Vec::new();
        let member_range = member_expression.member().ok()?.range();
        if method_config.return_value_required {
            if returns_info.has_paths_without_returns {
                if returns_info.returns_with_value.is_empty() {
                    problems.push(RuleProblemKind::MissingReturnWithValue);
                } else {
                    problems.push(RuleProblemKind::NotAllPathsReturnValue);
                }
            } else if !returns_info.returns_without_value.is_empty() {
                if !returns_info.returns_with_value.is_empty() {
                    for return_range in returns_info.returns_without_value {
                        problems.push(RuleProblemKind::UnexpectedEmptyReturn(return_range));
                    }
                } else {
                    problems.push(RuleProblemKind::MissingReturnWithValue);
                }
            }
        } else {
            for return_range in returns_info.returns_with_value {
                problems.push(RuleProblemKind::UnexpectedReturnWithValue(return_range));
            }
        }

        if !problems.is_empty() {
            Some(RuleState {
                problems: problems.into_boxed_slice(),
                callee_range: member_range,
                method_info: method_config,
            })
        } else {
            None
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let function_return_requirement = if state.method_info.return_value_required {
            "should always"
        } else {
            "should not"
        };
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.callee_range,
            markup! {
                "This "<Emphasis>"callback"</Emphasis>" passed to "<Emphasis>{state.method_info}</Emphasis>" "{function_return_requirement}" "<Emphasis>"return"</Emphasis>" a value."
            },
        );
        for problem in state.problems.iter() {
            match problem {
                RuleProblemKind::NotAllPathsReturnValue => {
                    diagnostic = diagnostic.note(
                        markup! {
                            "Add missing "<Emphasis>"return"</Emphasis>" statements so that this "<Emphasis>"callback"</Emphasis>" returns a value on all execution paths."
                        },
                    );
                }
                RuleProblemKind::MissingReturnWithValue => {
                    diagnostic = diagnostic.note(markup! {
                        "Add a "<Emphasis>"return"</Emphasis>" with a value to this callback."
                    });
                }
                RuleProblemKind::UnexpectedEmptyReturn(return_range) => {
                    diagnostic = diagnostic.detail(
                        *return_range,
                        markup! {
                            "Change this "<Emphasis>"return"</Emphasis>" so that it returns a value."
                        },
                    );
                }
                RuleProblemKind::UnexpectedReturnWithValue(return_range) => {
                    diagnostic = diagnostic.detail(
                        *return_range,
                        markup! {
                            "Either remove this "<Emphasis>"return"</Emphasis>" or remove the returned value."
                        },
                    );
                }
            }
        }

        Some(diagnostic)
    }
}

#[derive(Debug)]
pub struct RuleState {
    /// The kind of problem detected.
    problems: Box<[RuleProblemKind]>,
    /// The range of the problematic code.
    callee_range: TextRange,
    /// Reference to the relevant iterable method information.
    method_info: &'static IterableMethodInfo,
}

#[derive(Debug)]
enum RuleProblemKind {
    /// The function has paths in control flow that do not return a value.
    NotAllPathsReturnValue,
    /// Missing `return` statement, though expected in this case.
    MissingReturnWithValue,
    /// An unexpected `return` statement without value.
    UnexpectedEmptyReturn(TextRange),
    /// An unexpected `return` statement with value.
    UnexpectedReturnWithValue(TextRange),
}

/// This struct holds information about the return statements in a function.
/// It includes the count of blocks without return statements,
#[derive(Debug)]
struct FunctionReturnsInfo {
    /// The number of blocks that do not have any return statements.
    has_paths_without_returns: bool,
    /// The ranges of return keywords that return a value.
    returns_with_value: Vec<TextRange>,
    /// The ranges of return keywords that do not return a value.
    returns_without_value: Vec<TextRange>,
}

/// This function analyzes the control flow graph of a function and collects information about
/// the return statements. It also counts the number of blocks that do not have any return
/// statements.
fn get_function_returns_info(cfg: &JsControlFlowGraph) -> FunctionReturnsInfo {
    let mut function_returns_info = FunctionReturnsInfo {
        has_paths_without_returns: false,
        returns_with_value: Vec::new(),
        returns_without_value: Vec::new(),
    };

    if let Some(arrow_expression) = JsArrowFunctionExpression::cast_ref(&cfg.node)
        && let Ok(AnyJsFunctionBody::AnyJsExpression(expression)) = arrow_expression.body()
    {
        let is_void_expression = expression
            .as_js_unary_expression()
            .and_then(|unary| unary.is_void().ok())
            .unwrap_or(false);

        if is_void_expression {
            function_returns_info
                .returns_without_value
                .push(expression.range())
        } else {
            function_returns_info
                .returns_with_value
                .push(expression.range())
        }

        return function_returns_info;
    }

    // stack of blocks to process
    let mut block_stack = vec![ROOT_BLOCK_ID];
    let mut visited_blocks = RoaringBitmap::new();
    visited_blocks.insert(ROOT_BLOCK_ID.index());
    while let Some(block_id) = block_stack.pop() {
        let block = cfg.get(block_id);
        for handler in block.exception_handlers.iter() {
            if matches!(handler.kind, ExceptionHandlerKind::Catch) {
                // Avoid cycles and redundant checks.
                if visited_blocks.insert(handler.target.index()) {
                    block_stack.push(handler.target);
                }
            }
        }
        for instruction in block.instructions.iter() {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block: jump_block_id,
                    ..
                } => {
                    // Avoid cycles and redundant checks.
                    if visited_blocks.insert(jump_block_id.index()) {
                        block_stack.push(jump_block_id);
                    }
                    if !conditional {
                        // The next instructions are unreachable.
                        break;
                    }
                }
                InstructionKind::Return => {
                    match &instruction.node {
                        Some(NodeOrToken::Node(node)) => {
                            if let Some(return_stmt) = JsReturnStatement::cast_ref(node) {
                                let range = return_stmt
                                    .return_token()
                                    .map(|token| token.text_range())
                                    .unwrap_or(return_stmt.range());
                                if return_stmt.argument().is_none() {
                                    function_returns_info.returns_without_value.push(range);
                                } else {
                                    function_returns_info.returns_with_value.push(range);
                                }
                            }
                            // Ignore execution paths ending with `throw` statements.
                        }
                        _ => {
                            function_returns_info.has_paths_without_returns = true;
                        }
                    }
                    break;
                }
            }
        }
    }
    function_returns_info
}

/// This struct holds information about iterable methods to be used in the lint rule.
#[derive(Debug)]
struct IterableMethodInfo {
    /// The name of the iterable method (e.g., "map", "filter").
    method_name: &'static str,
    /// The name of the global object (e.g., "Array") that contains the method.
    global_name: Option<&'static str>,
    /// The position of the callback argument in the method's argument list.
    callback_argument_position: usize,
    /// Indicates whether the method requires a return value from the callback.
    return_value_required: bool,
}

/// A static map that holds information about iterable methods.
static ITERABLE_METHOD_INFOS: LazyLock<FxHashMap<&'static str, IterableMethodInfo>> =
    LazyLock::new(|| {
        let mut map: FxHashMap<&'static str, IterableMethodInfo> = FxHashMap::default();
        for method_name in [
            "every",
            "filter",
            "find",
            "findIndex",
            "findLast",
            "findLastIndex",
            "flatMap",
            "map",
            "reduce",
            "reduceRight",
            "some",
            "sort",
            "toSorted",
        ] {
            map.insert(
                method_name,
                IterableMethodInfo {
                    method_name,
                    global_name: None,
                    callback_argument_position: 0,
                    return_value_required: true,
                },
            );
        }
        map.insert(
            "forEach",
            IterableMethodInfo {
                method_name: "forEach",
                global_name: None,
                callback_argument_position: 0,
                return_value_required: false,
            },
        );
        map.insert(
            "from",
            IterableMethodInfo {
                method_name: "from",
                global_name: Some("Array"),
                callback_argument_position: 1,
                return_value_required: true,
            },
        );
        map
    });

impl Display for IterableMethodInfo {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        if let Some(global_name) = self.global_name {
            write!(fmt, "{}.{}() method", global_name, self.method_name)
        } else {
            write!(fmt, "{}() iterable method", self.method_name)
        }
    }
}
