use crate::context::trailing_commas::FormatTrailingCommas;
use crate::js::bindings::parameters::has_only_simple_parameters;
use crate::js::expressions::call_arguments::GroupedCallArgumentLayout;
use crate::prelude::*;
use crate::utils::function_body::{FormatMaybeCachedFunctionBody, FunctionBodyCacheMode};
use crate::utils::AssignmentLikeLayout;

use biome_formatter::{
    format_args, write, CstFormatContext, FormatRuleWithOptions, RemoveSoftLinesBuffer,
};
use biome_js_syntax::expression_left_side::AnyJsExpressionLeftSide;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{
    is_test_call_argument, AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsExpression,
    AnyJsFormalParameter, AnyJsFunctionBody, AnyJsParameter, AnyJsTemplateElement,
    JsArrowFunctionExpression, JsFormalParameter, JsSyntaxKind, JsTemplateExpression,
};
use biome_rowan::{SyntaxNodeOptionExt, SyntaxResult};
use std::iter::once;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsArrowFunctionExpression {
    options: FormatJsArrowFunctionExpressionOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatJsArrowFunctionExpressionOptions {
    pub assignment_layout: Option<AssignmentLikeLayout>,
    pub call_arg_layout: Option<GroupedCallArgumentLayout>,
    pub body_cache_mode: FunctionBodyCacheMode,
}

impl FormatRuleWithOptions<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    type Options = FormatJsArrowFunctionExpressionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let layout =
            ArrowFunctionLayout::for_arrow(node.clone(), f.context().comments(), &self.options)?;

        match layout {
            ArrowFunctionLayout::Chain(chain) => {
                write!(f, [chain])
            }
            ArrowFunctionLayout::Single(arrow) => {
                use self::AnyJsExpression::*;
                use AnyJsFunctionBody::*;

                let body = arrow.body()?;

                let formatted_signature = format_with(|f| {
                    write!(
                        f,
                        [
                            format_signature(&arrow, self.options.call_arg_layout.is_some(), true),
                            space(),
                            arrow.fat_arrow_token().format()
                        ]
                    )
                });

                let format_body = FormatMaybeCachedFunctionBody {
                    body: &body,
                    mode: self.options.body_cache_mode,
                };

                // With arrays, arrow selfs and objects, they have a natural line breaking strategy:
                // Arrays and objects become blocks:
                //
                //    [
                //      100000,
                //      200000,
                //      300000
                //    ]
                //
                // Arrow selfs get line broken after the `=>`:
                //
                //  (foo) => (bar) =>
                //     (foo + bar) * (foo + bar)
                //
                // Therefore if our body is an arrow self, array, or object, we
                // do not have a soft line break after the arrow because the body is
                // going to get broken anyways.
                let body_has_soft_line_break = match &body {
                    JsFunctionBody(_)
                    | AnyJsExpression(
                        JsArrowFunctionExpression(_) | JsArrayExpression(_) | JsObjectExpression(_),
                    ) => !f.comments().has_leading_own_line_comment(body.syntax()),
                    AnyJsExpression(JsxTagExpression(_)) => true,
                    AnyJsExpression(JsTemplateExpression(template)) => {
                        is_multiline_template_starting_on_same_line(template)
                    }
                    AnyJsExpression(JsSequenceExpression(sequence)) => {
                        let has_comment = f.context().comments().has_comments(sequence.syntax());
                        if has_comment {
                            return write!(
                                f,
                                [group(&format_args![
                                    formatted_signature,
                                    group(&format_args![indent(&format_args![
                                        hard_line_break(),
                                        text("("),
                                        soft_block_indent(&format_body),
                                        text(")")
                                    ]),])
                                ])]
                            );
                        }
                        return write!(
                            f,
                            [group(&format_args![
                                formatted_signature,
                                group(&format_args![
                                    space(),
                                    text("("),
                                    soft_block_indent(&format_body),
                                    text(")")
                                ])
                            ])]
                        );
                    }
                    _ => false,
                };
                let body_is_condition_type =
                    matches!(body, AnyJsExpression(JsConditionalExpression(_)));
                if body_has_soft_line_break {
                    write![f, [formatted_signature, space(), format_body]]
                } else {
                    let should_add_parens = should_add_parens(&body);

                    let is_last_call_arg = matches!(
                        self.options.call_arg_layout,
                        Some(GroupedCallArgumentLayout::GroupedLastArgument)
                    );

                    let should_add_soft_line = (is_last_call_arg
                        // if it's inside a JSXExpression (e.g. an attribute) we should align the expression's closing } with the line with the opening {.
                        || matches!(node.syntax().parent().kind(), Some(JsSyntaxKind::JSX_EXPRESSION_CHILD | JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE)))
                        && !f.context().comments().has_comments(node.syntax());
                    if body_is_condition_type {
                        write!(
                            f,
                            [
                                formatted_signature,
                                group(&format_args![
                                    soft_line_indent_or_hard_space(&format_with(|f| {
                                        if should_add_parens {
                                            write!(f, [if_group_fits_on_line(&text("("))])?;
                                        }

                                        write!(f, [format_body])?;

                                        if should_add_parens {
                                            write!(f, [if_group_fits_on_line(&text(")"))])?;
                                        }

                                        Ok(())
                                    })),
                                    is_last_call_arg
                                        .then_some(format_args![FormatTrailingCommas::All,]),
                                    should_add_soft_line.then_some(format_args![soft_line_break()])
                                ])
                            ]
                        )
                    } else {
                        write!(
                            f,
                            [
                                formatted_signature,
                                group(&format_args![
                                    soft_line_indent_or_space(&format_with(|f| {
                                        if should_add_parens {
                                            write!(f, [if_group_fits_on_line(&text("("))])?;
                                        }

                                        write!(f, [format_body])?;

                                        if should_add_parens {
                                            write!(f, [if_group_fits_on_line(&text(")"))])?;
                                        }

                                        Ok(())
                                    })),
                                    is_last_call_arg
                                        .then_some(format_args![FormatTrailingCommas::All,]),
                                    should_add_soft_line.then_some(format_args![soft_line_break()])
                                ])
                            ]
                        )
                    }
                }
            }
        }
    }

    fn needs_parentheses(&self, item: &JsArrowFunctionExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsArrowFunctionExpression,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// Writes the arrow function type parameters, parameters, and return type annotation.
///
/// Formats the parameters and return type annotation without any soft line breaks if `is_first_or_last_call_argument` is `true`
/// so that the parameters and return type are kept on the same line.
///
/// # Errors
///
/// Returns [`FormatError::PoorLayout`] if `is_first_or_last_call_argument` is `true` but the parameters
/// or return type annotation contain any content that forces a [*group to break](FormatElements::will_break).
///
/// This error gets captured by [FormatJsCallArguments].
fn format_signature(
    arrow: &JsArrowFunctionExpression,
    is_first_or_last_call_argument: bool,
    is_first_in_chain: bool,
) -> impl Format<JsFormatContext> + '_ {
    format_with(move |f| {
        let formatted_async_token = format_with(|f: &mut JsFormatter| {
            if let Some(async_token) = arrow.async_token() {
                write!(f, [async_token.format(), space()])?;
                Ok(())
            } else {
                Ok(())
            }
        });

        let formatted_parameters = format_with(|f: &mut JsFormatter| {
            write!(f, [arrow.type_parameters().format()])?;

            match arrow.parameters()? {
                AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                    let should_hug =
                        is_test_call_argument(arrow.syntax())? || is_first_or_last_call_argument;
                    let parentheses_not_needed = can_avoid_parentheses(arrow, f);

                    if !parentheses_not_needed {
                        write!(f, [text("(")])?;
                    }

                    if should_hug || parentheses_not_needed {
                        write!(f, [binding.format()])?;
                    } else {
                        write!(
                            f,
                            [&soft_block_indent(&format_args![
                                binding.format(),
                                FormatTrailingCommas::All
                            ])]
                        )?
                    }

                    if !parentheses_not_needed {
                        write!(f, [text(")")])?;
                    }
                }
                AnyJsArrowFunctionParameters::JsParameters(params) => {
                    write!(f, [params.format()])?;
                }
            };

            Ok(())
        });

        if is_first_or_last_call_argument {
            let mut buffer = RemoveSoftLinesBuffer::new(f);
            let mut recording = buffer.start_recording();

            write!(
                recording,
                [group(&format_args![
                    maybe_space(!is_first_in_chain),
                    formatted_async_token,
                    group(&formatted_parameters),
                    group(&arrow.return_type_annotation().format())
                ])]
            )?;

            if recording.stop().will_break() {
                return Err(FormatError::PoorLayout);
            }
        } else {
            write!(
                f,
                [
                    // This soft break is placed outside of the group to ensure
                    // that the parameter group only tries to write on a single
                    // line and can't break pre-emptively without also causing
                    // the parent (i.e., this ArrowChain) to break first.
                    (!is_first_in_chain).then_some(soft_line_break_or_space()),
                    group(&format_args![
                        formatted_async_token,
                        formatted_parameters,
                        arrow.return_type_annotation().format()
                    ])
                ]
            )?;
        }

        if f.comments().has_dangling_comments(arrow.syntax()) {
            write!(f, [space(), format_dangling_comments(arrow.syntax())])?;
        }

        Ok(())
    })
}

/// Returns a `true` result if the arrow function contains any elements which
/// should force the chain to break onto multiple lines. This includes any kind
/// of return type annotation if the function also takes parameters (e.g.,
/// `(a, b): bool => ...`), any kind of rest/object/array binding parameter
/// (e.g., `({a, b: foo}) => ...`), and any kind of initializer for a parameter
/// (e.g., `(a = 2) => ...`).
///
/// The complexity of these expressions limits their legibility when printed
/// inline, so they force the chain to break to preserve clarity. Any other
/// cases are considered simple enough to print in a single line.
fn should_break_chain(arrow: &JsArrowFunctionExpression) -> SyntaxResult<bool> {
    if arrow.type_parameters().is_some() {
        return Ok(true);
    }

    let parameters = arrow.parameters()?;

    let has_parameters = match &parameters {
        AnyJsArrowFunctionParameters::AnyJsBinding(_) => true,
        AnyJsArrowFunctionParameters::JsParameters(parameters) => {
            // This matches Prettier, which allows type annotations when
            // grouping arrow expressions, but disallows them when grouping
            // normal function expressions.
            if !has_only_simple_parameters(parameters, true) {
                return Ok(true);
            }
            !parameters.items().is_empty()
        }
    };

    let has_type_and_parameters = arrow.return_type_annotation().is_some() && has_parameters;

    Ok(has_type_and_parameters || has_rest_object_or_array_parameter(&parameters))
}

fn should_add_parens(body: &AnyJsFunctionBody) -> bool {
    // Add parentheses to avoid confusion between `a => b ? c : d` and `a <= b ? c : d`
    // but only if the body isn't an object/function or class expression because parentheses are always required in that
    // case and added by the object expression itself
    match &body {
        AnyJsFunctionBody::AnyJsExpression(
            expression @ AnyJsExpression::JsConditionalExpression(_),
        ) => {
            let var_name = matches!(
                AnyJsExpressionLeftSide::leftmost(expression.clone()),
                AnyJsExpressionLeftSide::AnyJsExpression(
                    AnyJsExpression::JsObjectExpression(_)
                        | AnyJsExpression::JsFunctionExpression(_)
                        | AnyJsExpression::JsClassExpression(_)
                )
            );
            let are_parentheses_mandatory = var_name;

            !are_parentheses_mandatory
        }
        _ => false,
    }
}

fn has_rest_object_or_array_parameter(parameters: &AnyJsArrowFunctionParameters) -> bool {
    match parameters {
        AnyJsArrowFunctionParameters::AnyJsBinding(_) => false,
        AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters
            .items()
            .iter()
            .flatten()
            .any(|parameter| match parameter {
                AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                    parameter,
                )) => {
                    matches!(
                        parameter.binding(),
                        Ok(AnyJsBindingPattern::JsArrayBindingPattern(_)
                            | AnyJsBindingPattern::JsObjectBindingPattern(_))
                    )
                }
                AnyJsParameter::AnyJsFormalParameter(
                    AnyJsFormalParameter::JsBogusParameter(_)
                    | AnyJsFormalParameter::JsMetavariable(_),
                ) => false,
                AnyJsParameter::TsThisParameter(_) => false,
                AnyJsParameter::JsRestParameter(_) => true,
            }),
    }
}

/// Returns `true` if parentheses can be safely avoided and the `arrow_parentheses` formatter option allows it
pub fn can_avoid_parentheses(arrow: &JsArrowFunctionExpression, f: &mut JsFormatter) -> bool {
    arrow.parameters().is_ok_and(|parameters| {
        f.options().arrow_parentheses().is_as_needed()
            && parameters.len() == 1
            && arrow.type_parameters().is_none()
            && arrow.return_type_annotation().is_none()
            && !has_rest_object_or_array_parameter(&parameters)
            && !parameters
                .as_js_parameters()
                .and_then(|p| p.items().first()?.ok())
                .and_then(|p| JsFormalParameter::cast(p.into_syntax()))
                .is_some_and(|p| {
                    f.context().comments().has_comments(p.syntax())
                        || p.initializer().is_some()
                        || p.question_mark_token().is_some()
                        || p.type_annotation().is_some()
                })
    })
}

#[derive(Clone, Debug)]
enum ArrowFunctionLayout {
    /// Arrow function with a non-arrow function body
    Single(JsArrowFunctionExpression),

    /// A chain of at least two arrow functions.
    ///
    /// An arrow function is part of the chain when it is the body of the parent arrow function.
    ///
    /// The idea of arrow chains is that they break after the `=>` token
    ///
    /// ```javascript
    /// const x =
    ///   (a): string =>
    ///   (b) =>
    ///   (c) =>
    ///   (d) =>
    ///   (e) =>
    ///     f;
    /// ```
    Chain(ArrowChain),
}

#[derive(Clone, Debug)]
struct ArrowChain {
    /// The top most arrow function in the chain
    head: JsArrowFunctionExpression,

    /// The arrow functions in the chain that are neither the first nor the last.
    /// Empty for chains consisting only of two arrow functions.
    middle: Vec<JsArrowFunctionExpression>,

    /// The last arrow function in the chain
    tail: JsArrowFunctionExpression,

    options: FormatJsArrowFunctionExpressionOptions,

    /// Whether the group wrapping the signatures should be expanded or not.
    expand_signatures: bool,
}

impl ArrowChain {
    /// Returns an iterator over all arrow functions in this chain
    fn arrows(&self) -> impl Iterator<Item = &JsArrowFunctionExpression> {
        once(&self.head)
            .chain(self.middle.iter())
            .chain(once(&self.tail))
    }
}

impl Format<JsFormatContext> for ArrowChain {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let ArrowChain {
            head,
            tail,
            expand_signatures,
            ..
        } = self;

        let head_parent = head.syntax().parent();
        let tail_body = tail.body()?;
        let is_assignment_rhs = self.options.assignment_layout.is_some();
        let is_grouped_call_arg_layout = self.options.call_arg_layout.is_some();

        // If this chain is the callee in a parent call expression, then we
        // want it to break onto a new line to clearly show that the arrow
        // chain is distinct and the _result_ is what's being called.
        // Example:
        //      (() => () => a)()
        // becomes
        //      (
        //        () => () =>
        //          a
        //      )();
        let is_callee = head_parent.as_ref().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
            )
        });

        // With arrays, objects, sequence expressions, and block function bodies,
        // the opening brace gives a convenient boundary to insert a line break,
        // allowing that token to live immediately after the last arrow token
        // and save a line from being printed with just the punctuation.
        //
        // (foo) => (bar) => [a, b]
        //
        // (foo) => (bar) => [
        //   a,
        //   b
        // ]
        //
        // If the body is _not_ one of those kinds, then we'll want to insert a
        // soft line break before the body so that it prints on a separate line
        // in its entirety.
        let body_on_separate_line = !matches!(
            tail_body,
            AnyJsFunctionBody::JsFunctionBody(_)
                | AnyJsFunctionBody::AnyJsExpression(
                    AnyJsExpression::JsObjectExpression(_)
                        | AnyJsExpression::JsArrayExpression(_)
                        | AnyJsExpression::JsSequenceExpression(_)
                        | AnyJsExpression::JsxTagExpression(_)
                )
        );

        // If the arrow chain will break onto multiple lines, either because
        // it's a callee or because the body is printed on its own line, then
        // the signatures should be expanded first.
        let break_signatures = (is_callee && body_on_separate_line)
            || matches!(
                self.options.assignment_layout,
                Some(AssignmentLikeLayout::ChainTailArrowFunction)
            );

        // Arrow chains as callees or as the right side of an assignment
        // indent the entire signature chain a single level and do _not_
        // indent a second level for additional signatures after the first:
        //   const foo =
        //     (a) =>
        //     (b) =>
        //     (c) =>
        //       0;
        // This tracks that state and is used to prevent the insertion of
        // additional indents under `format_arrow_signatures`, then also to
        // add the outer indent under `format_inner`.
        let has_initial_indent = is_callee || is_assignment_rhs;

        let format_arrow_signatures = format_with(|f| {
            let join_signatures = format_with(|f: &mut JsFormatter| {
                let mut is_first_in_chain = true;
                for arrow in self.arrows() {
                    // The first comment in the chain gets formatted by the
                    // parent (the FormatJsArrowFunctionExpression), but the
                    // rest of the arrows in the chain need to format their
                    // comments manually, since they won't have their own
                    // Format node to handle it.
                    let should_format_comments = !is_first_in_chain
                        && f.context().comments().has_leading_comments(arrow.syntax());
                    let is_first = is_first_in_chain;

                    let formatted_signature = format_with(|f: &mut JsFormatter| {
                        if should_format_comments {
                            // A grouped layout implies that the arrow chain is trying to be rendered
                            // in a condensend, single-line format (at least the signatures, not
                            // necessarily the body). In that case, we _need_ to prevent the leading
                            // comments from inserting line breaks. But if it's _not_ a grouped layout,
                            // then we want to _force_ the line break so that the leading comments
                            // don't inadvertently end up on the previous line after the fat arrow.
                            if is_grouped_call_arg_layout {
                                write!(f, [space(), format_leading_comments(arrow.syntax())])?;
                            } else {
                                write!(
                                    f,
                                    [
                                        soft_line_break_or_space(),
                                        format_leading_comments(arrow.syntax())
                                    ]
                                )?;
                            }
                        }

                        write!(
                            f,
                            [format_signature(
                                arrow,
                                is_grouped_call_arg_layout,
                                is_first
                            )]
                        )
                    });

                    // Arrow chains indent a second level for every item other than the first:
                    //   (a) =>
                    //     (b) =>
                    //     (c) =>
                    //       0
                    // Because the chain is printed as a flat list, each entry needs to set
                    // its own indention. This ensures that the first item keeps the same
                    // level as the surrounding content, and then each subsequent item has
                    // one additional level, as shown above.
                    if is_first_in_chain || has_initial_indent {
                        is_first_in_chain = false;
                        write!(f, [formatted_signature])?;
                    } else {
                        write!(f, [indent(&formatted_signature)])?;
                    };

                    // The arrow of the tail is formatted outside of the group to ensure it never
                    // breaks from the body
                    if arrow != tail {
                        write!(f, [space(), arrow.fat_arrow_token().format()])?;
                    }
                }

                Ok(())
            });

            write!(
                f,
                [group(&join_signatures).should_expand(*expand_signatures)]
            )
        });

        let has_comment = matches!(
          &tail_body,
          AnyJsFunctionBody::AnyJsExpression(AnyJsExpression::JsSequenceExpression(sequence))
          if f.context().comments().has_comments(sequence.syntax())
        );

        let format_tail_body_inner = format_with(|f| {
            let format_tail_body = FormatMaybeCachedFunctionBody {
                body: &tail_body,
                mode: self.options.body_cache_mode,
            };

            // Ensure that the parens of sequence expressions end up on their own line if the
            // body breaks
            if matches!(
                tail_body,
                AnyJsFunctionBody::AnyJsExpression(AnyJsExpression::JsSequenceExpression(_))
            ) {
                if has_comment {
                    write!(
                        f,
                        [group(&format_args![indent(&format_args![
                            hard_line_break(),
                            text("("),
                            soft_block_indent(&format_tail_body),
                            text(")")
                        ]),])]
                    )?;
                } else {
                    write!(
                        f,
                        [group(&format_args![
                            text("("),
                            soft_block_indent(&format_tail_body),
                            text(")")
                        ])]
                    )?;
                }
            } else {
                let should_add_parens = should_add_parens(&tail_body);
                if should_add_parens {
                    write!(
                        f,
                        [
                            if_group_fits_on_line(&text("(")),
                            format_tail_body,
                            if_group_fits_on_line(&text(")"))
                        ]
                    )?;
                } else {
                    write!(f, [format_tail_body])?;
                }
            }

            // Format the trailing comments of all arrow function EXCEPT the first one because
            // the comments of the head get formatted as part of the `FormatJsArrowFunctionExpression` call.
            for arrow in self.arrows().skip(1) {
                write!(f, [format_trailing_comments(arrow.syntax())])?;
            }

            Ok(())
        });

        let format_tail_body = format_with(|f| {
            // if it's inside a JSXExpression (e.g. an attribute) we should align the expression's closing } with the line with the opening {.
            let should_add_soft_line = matches!(
                head_parent.kind(),
                Some(
                    JsSyntaxKind::JSX_EXPRESSION_CHILD
                        | JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE
                )
            );

            if body_on_separate_line {
                write!(
                    f,
                    [
                        indent(&format_args![
                            soft_line_break_or_space(),
                            format_tail_body_inner
                        ]),
                        should_add_soft_line.then_some(soft_line_break())
                    ]
                )
            } else {
                write!(f, [space(), format_tail_body_inner])
            }
        });

        let group_id = f.group_id("arrow-chain");

        let format_inner = format_once(|f| {
            if has_initial_indent {
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break(),
                        format_arrow_signatures
                    ]))
                    .with_group_id(Some(group_id))
                    .should_expand(break_signatures)]
                )?;
            } else {
                write!(
                    f,
                    [group(&format_arrow_signatures)
                        .with_group_id(Some(group_id))
                        .should_expand(break_signatures)]
                )?;
            };

            write!(f, [space(), tail.fat_arrow_token().format()])?;

            if is_grouped_call_arg_layout {
                write!(f, [group(&format_tail_body)])?;
            } else {
                write!(f, [indent_if_group_breaks(&format_tail_body, group_id)])?;
            }

            if is_callee {
                write!(
                    f,
                    [if_group_breaks(&soft_line_break()).with_group_id(Some(group_id))]
                )?;
            }

            Ok(())
        });

        write!(f, [group(&format_inner)])
    }
}

impl ArrowFunctionLayout {
    /// Determines the layout for the passed arrow function. See [ArrowFunctionLayout] for a description
    /// of the different layouts.
    fn for_arrow(
        arrow: JsArrowFunctionExpression,
        comments: &JsComments,
        options: &FormatJsArrowFunctionExpressionOptions,
    ) -> SyntaxResult<ArrowFunctionLayout> {
        let mut head = None;
        let mut middle = Vec::new();
        let mut current = arrow;
        let mut should_break = false;

        let result = loop {
            match current.body()? {
                AnyJsFunctionBody::AnyJsExpression(AnyJsExpression::JsArrowFunctionExpression(
                    next,
                )) if matches!(
                    options.call_arg_layout,
                    None | Some(GroupedCallArgumentLayout::GroupedLastArgument)
                ) && !comments.is_suppressed(next.syntax()) =>
                {
                    should_break = should_break || should_break_chain(&current)?;

                    if let Some(body) = JsArrowFunctionExpression::cast_ref(next.syntax()) {
                        should_break = should_break || should_break_chain(&body)?;
                    }

                    if head.is_none() {
                        head = Some(current);
                    } else {
                        middle.push(current);
                    }

                    current = next;
                }
                _ => {
                    break match head {
                        None => ArrowFunctionLayout::Single(current),
                        Some(head) => ArrowFunctionLayout::Chain(ArrowChain {
                            head,
                            middle,
                            tail: current,
                            expand_signatures: should_break,
                            options: *options,
                        }),
                    }
                }
            }
        };

        Ok(result)
    }
}

/// Returns `true` if the template contains any new lines inside of its text chunks.
fn template_literal_contains_new_line(template: &JsTemplateExpression) -> bool {
    template.elements().iter().any(|element| match element {
        AnyJsTemplateElement::JsTemplateChunkElement(chunk) => chunk
            .template_chunk_token()
            .is_ok_and(|chunk| chunk.text().contains('\n')),
        AnyJsTemplateElement::JsTemplateElement(_) => false,
    })
}

/// Returns `true` for a template that starts on the same line as the previous token and contains a line break.
///
///
/// # Examples
///
/// ```javascript
/// "test" + `
///   some content
/// `;
/// ```
///
/// Returns `true` because the template starts on the same line as the `+` token and its text contains a line break.
///
/// ```javascript
/// "test" + `no line break`
/// ```
///
/// Returns `false` because the template text contains no line break.
///
/// ```javascript
/// "test" +
///     `template
///     with line break`;
/// ```
///
/// Returns `false` because the template isn't on the same line as the '+' token.
pub(crate) fn is_multiline_template_starting_on_same_line(template: &JsTemplateExpression) -> bool {
    let contains_new_line = template_literal_contains_new_line(template);

    let starts_on_same_line = template.syntax().first_token().is_some_and(|token| {
        for piece in token.leading_trivia().pieces() {
            if let Some(comment) = piece.as_comments() {
                if comment.has_newline() {
                    return false;
                }
            } else if piece.is_newline() {
                return false;
            }
        }

        true
    });

    contains_new_line && starts_on_same_line
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsArrowFunctionExpression, JsFileSource};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (a => test)()`", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => test)()", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => test).member", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => test)[member]", JsArrowFunctionExpression);
        assert_not_needs_parentheses!("object[a => a]", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) as Function", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a)!", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a)`template`", JsArrowFunctionExpression);
        assert_needs_parentheses!("+(a => a)", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) && b", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) instanceof b", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) in b", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) + b", JsArrowFunctionExpression);
        assert_needs_parentheses!("await (a => a)", JsArrowFunctionExpression);
        assert_needs_parentheses!(
            "<Function>(a => a)",
            JsArrowFunctionExpression,
            JsFileSource::ts()
        );
        assert_needs_parentheses!("(a => a) ? b : c", JsArrowFunctionExpression);
        assert_not_needs_parentheses!("a ? b => b : c", JsArrowFunctionExpression);
        assert_not_needs_parentheses!("a ? b : c => c", JsArrowFunctionExpression);
        assert_needs_parentheses!("class Test extends (a => a) {}", JsArrowFunctionExpression);
    }
}
