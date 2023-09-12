use crate::prelude::*;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody,
    AnyJsLiteralExpression, AnyJsName, AnyJsTemplateElement, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsSyntaxNode, JsTemplateExpression,
};
use biome_rowan::{SyntaxResult, TokenText};

/// Returns `Ok(true)` if `maybe_argument` is an argument of a [test call expression](is_test_call_expression).
pub(crate) fn is_test_call_argument(maybe_argument: &JsSyntaxNode) -> SyntaxResult<bool> {
    let call_expression = maybe_argument
        .parent()
        .and_then(JsCallArgumentList::cast)
        .and_then(|args| args.syntax().grand_parent())
        .and_then(JsCallExpression::cast);

    call_expression.map_or(Ok(false), |call| is_test_call_expression(&call))
}

/// This is a specialised function that checks if the current [call expression]
/// resembles a call expression usually used by a testing frameworks.
///
/// If the [call expression] matches the criteria, a different formatting is applied.
///
/// To evaluable the eligibility of a  [call expression] to be a test framework like,
/// we need to check its [callee] and its [arguments].
///
/// 1. The [callee] must contain a name or a chain of names that belongs to the
/// test frameworks, for example: `test()`, `test.only()`, etc.
/// 2. The [arguments] should be at the least 2
/// 3. The first argument has to be a string literal
/// 4. The third argument, if present, has to be a number literal
/// 5. The second argument has to be an [arrow function expression] or [function expression]
/// 6. Both function must have zero or one parameters
///
/// [call expression]: crate::biome_js_syntax::JsCallExpression
/// [callee]: crate::biome_js_syntax::AnyJsExpression
/// [arguments]: crate::biome_js_syntax::JsCallArgumentList
/// [arrow function expression]: crate::biome_js_syntax::JsArrowFunctionExpression
/// [function expression]: crate::biome_js_syntax::JsCallArgumentList
pub(crate) fn is_test_call_expression(call_expression: &JsCallExpression) -> SyntaxResult<bool> {
    use AnyJsExpression::*;

    let callee = call_expression.callee()?;
    let arguments = call_expression.arguments()?;

    let mut args = arguments.args().iter();

    match (args.next(), args.next(), args.next()) {
        (Some(Ok(argument)), None, None) if arguments.args().len() == 1 => {
            if is_angular_test_wrapper(&call_expression.clone().into())
                && call_expression
                    .parent::<JsCallArgumentList>()
                    .and_then(|arguments_list| arguments_list.parent::<JsCallArguments>())
                    .and_then(|arguments| arguments.parent::<self::JsCallExpression>())
                    .map_or(Ok(false), |parent| is_test_call_expression(&parent))?
            {
                return Ok(matches!(
                    argument,
                    AnyJsCallArgument::AnyJsExpression(
                        JsArrowFunctionExpression(_) | JsFunctionExpression(_)
                    )
                ));
            }

            if is_unit_test_set_up_callee(&callee) {
                return Ok(argument
                    .as_any_js_expression()
                    .map_or(false, is_angular_test_wrapper));
            }

            Ok(false)
        }

        // it("description", ..)
        (
            Some(Ok(AnyJsCallArgument::AnyJsExpression(
                JsTemplateExpression(_)
                | AnyJsLiteralExpression(self::AnyJsLiteralExpression::JsStringLiteralExpression(_)),
            ))),
            Some(Ok(second)),
            third,
        ) if arguments.args().len() <= 3 && contains_a_test_pattern(&callee)? => {
            // it('name', callback, duration)
            if !matches!(
                third,
                None | Some(Ok(AnyJsCallArgument::AnyJsExpression(
                    AnyJsLiteralExpression(
                        self::AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                    )
                )))
            ) {
                return Ok(false);
            }

            if second
                .as_any_js_expression()
                .map_or(false, is_angular_test_wrapper)
            {
                return Ok(true);
            }

            let (parameters, has_block_body) = match second {
                AnyJsCallArgument::AnyJsExpression(JsFunctionExpression(function)) => (
                    function
                        .parameters()
                        .map(AnyJsArrowFunctionParameters::from),
                    true,
                ),
                AnyJsCallArgument::AnyJsExpression(JsArrowFunctionExpression(arrow)) => (
                    arrow.parameters(),
                    arrow.body().map_or(false, |body| {
                        matches!(body, AnyJsFunctionBody::JsFunctionBody(_))
                    }),
                ),
                _ => return Ok(false),
            };

            Ok(arguments.args().len() == 2 || (parameters?.len() <= 1 && has_block_body))
        }
        _ => Ok(false),
    }
}

/// Note: `inject` is used in AngularJS 1.x, `async` and `fakeAsync` in
/// Angular 2+, although `async` is deprecated and replaced by `waitForAsync`
/// since Angular 12.
///
/// example: https://docs.angularjs.org/guide/unit-testing#using-beforeall-
///
/// @param {CallExpression} node
/// @returns {boolean}
///
fn is_angular_test_wrapper(expression: &AnyJsExpression) -> bool {
    use AnyJsExpression::*;
    match expression {
        JsCallExpression(call_expression) => match call_expression.callee() {
            Ok(JsIdentifierExpression(identifier)) => identifier
                .name()
                .and_then(|name| name.value_token())
                .map_or(false, |name| {
                    matches!(
                        name.text_trimmed(),
                        "async" | "inject" | "fakeAsync" | "waitForAsync"
                    )
                }),
            _ => false,
        },
        _ => false,
    }
}

/// Tests if the callee is a `beforeEach`, `beforeAll`, `afterEach` or `afterAll` identifier
/// that is commonly used in test frameworks.
fn is_unit_test_set_up_callee(callee: &AnyJsExpression) -> bool {
    match callee {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .and_then(|name| name.value_token())
            .map_or(false, |name| {
                matches!(
                    name.text_trimmed(),
                    "beforeEach" | "beforeAll" | "afterEach" | "afterAll"
                )
            }),
        _ => false,
    }
}

pub(crate) fn is_test_each_pattern(template: &JsTemplateExpression) -> bool {
    is_test_each_pattern_callee(template) && is_test_each_pattern_elements(template)
}

fn is_test_each_pattern_elements(template: &JsTemplateExpression) -> bool {
    let mut iter = template.elements().into_iter();

    // the table must have a header as JsTemplateChunkElement
    // e.g. a | b | expected
    if !matches!(
        iter.next(),
        Some(AnyJsTemplateElement::JsTemplateChunkElement(_))
    ) {
        return false;
    }

    // Guarding against skipped token trivia on elements that we remove.
    // Because that would result in the skipped token trivia being emitted before the template.
    for element in template.elements() {
        if let AnyJsTemplateElement::JsTemplateChunkElement(element) = element {
            if let Some(leading_trivia) = element.syntax().first_leading_trivia() {
                if leading_trivia.has_skipped() {
                    return false;
                }
            }
        }
    }

    true
}

/// This function checks if a call expressions has one of the following members:
/// - `describe.each`
/// - `describe.only.each`
/// - `describe.skip.each`
/// - `test.concurrent.each`
/// - `test.concurrent.only.each`
/// - `test.concurrent.skip.each`
/// - `test.each`
/// - `test.only.each`
/// - `test.skip.each`
/// - `test.failing.each`
/// - `it.concurrent.each`
/// - `it.concurrent.only.each`
/// - `it.concurrent.skip.each`
/// - `it.each`
/// - `it.only.each`
/// - `it.skip.each`
/// - `it.failing.each`
///
/// - `xdescribe.each`
/// - `xdescribe.only.each`
/// - `xdescribe.skip.each`
/// - `xtest.concurrent.each`
/// - `xtest.concurrent.only.each`
/// - `xtest.concurrent.skip.each`
/// - `xtest.each`
/// - `xtest.only.each`
/// - `xtest.skip.each`
/// - `xtest.failing.each`
/// - `xit.concurrent.each`
/// - `xit.concurrent.only.each`
/// - `xit.concurrent.skip.each`
/// - `xit.each`
/// - `xit.only.each`
/// - `xit.skip.each`
/// - `xit.failing.each`
///
/// - `fdescribe.each`
/// - `fdescribe.only.each`
/// - `fdescribe.skip.each`
/// - `ftest.concurrent.each`
/// - `ftest.concurrent.only.each`
/// - `ftest.concurrent.skip.each`
/// - `ftest.each`
/// - `ftest.only.each`
/// - `ftest.skip.each`
/// - `ftest.failing.each`
/// - `fit.concurrent.each`
/// - `fit.concurrent.only.each`
/// - `fit.concurrent.skip.each`
/// - `fit.each`
/// - `fit.only.each`
/// - `fit.skip.each`
/// - `xit.failing.each`
///
/// Based on this [article]
///
/// [article]: https://craftinginterpreters.com/scanning-on-demand.html#tries-and-state-machines
fn is_test_each_pattern_callee(template: &JsTemplateExpression) -> bool {
    if let Some(tag) = template.tag() {
        let mut members = CalleeNamesIterator::new(tag);

        let texts: [Option<TokenText>; 5] = [
            members.next(),
            members.next(),
            members.next(),
            members.next(),
            members.next(),
        ];

        let mut rev = texts.iter().rev().flatten();

        let first = rev.next().map(|t| t.text());
        let second = rev.next().map(|t| t.text());
        let third = rev.next().map(|t| t.text());
        let fourth = rev.next().map(|t| t.text());
        let fifth = rev.next().map(|t| t.text());

        match first {
            Some("describe" | "xdescribe" | "fdescribe") => match second {
                Some("each") => third.is_none(),
                Some("skip" | "only") => match third {
                    Some("each") => fourth.is_none(),
                    _ => false,
                },
                _ => false,
            },
            Some("test" | "xtest" | "ftest" | "it" | "xit" | "fit") => match second {
                Some("each") => third.is_none(),
                Some("skip" | "only" | "failing") => match third {
                    Some("each") => fourth.is_none(),
                    _ => false,
                },
                Some("concurrent") => match third {
                    Some("each") => fourth.is_none(),
                    Some("only" | "skip") => match fourth {
                        Some("each") => fifth.is_none(),
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    } else {
        false
    }
}

/// This function checks if a call expressions has one of the following members:
/// - `it`
/// - `it.only`
/// - `it.skip`
/// - `describe`
/// - `describe.only`
/// - `describe.skip`
/// - `test`
/// - `test.only`
/// - `test.skip`
/// - `test.step`
/// - `test.describe`
/// - `test.describe.only`
/// - `test.describe.parallel`
/// - `test.describe.parallel.only`
/// - `test.describe.serial`
/// - `test.describe.serial.only`
/// - `skip`
/// - `xit`
/// - `xdescribe`
/// - `xtest`
/// - `fit`
/// - `fdescribe`
/// - `ftest`
///
/// Based on this [article]
///
/// [article]: https://craftinginterpreters.com/scanning-on-demand.html#tries-and-state-machines
fn contains_a_test_pattern(callee: &AnyJsExpression) -> SyntaxResult<bool> {
    let mut members = CalleeNamesIterator::new(callee.clone());

    let texts: [Option<TokenText>; 5] = [
        members.next(),
        members.next(),
        members.next(),
        members.next(),
        members.next(),
    ];

    let mut rev = texts.iter().rev().flatten();

    let first = rev.next().map(|t| t.text());
    let second = rev.next().map(|t| t.text());
    let third = rev.next().map(|t| t.text());
    let fourth = rev.next().map(|t| t.text());
    let fifth = rev.next().map(|t| t.text());

    Ok(match first {
        Some("it" | "describe") => match second {
            None => true,
            Some("only" | "skip") => third.is_none(),
            _ => false,
        },
        Some("test") => match second {
            None => true,
            Some("only" | "skip" | "step") => third.is_none(),
            Some("describe") => match third {
                None => true,
                Some("only") => true,
                Some("parallel" | "serial") => match fourth {
                    None => true,
                    Some("only") => fifth.is_none(),
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
        Some("skip" | "xit" | "xdescribe" | "xtest" | "fit" | "fdescribe" | "ftest") => true,
        _ => false,
    })
}

/// Iterator that returns the callee names in "top down order".
///
/// # Examples
///
/// ```javascript
/// it.only() -> [`only`, `it`]
/// ```
struct CalleeNamesIterator {
    next: Option<AnyJsExpression>,
}

impl CalleeNamesIterator {
    fn new(callee: AnyJsExpression) -> Self {
        Self { next: Some(callee) }
    }
}

impl Iterator for CalleeNamesIterator {
    type Item = TokenText;

    fn next(&mut self) -> Option<Self::Item> {
        use AnyJsExpression::*;

        let current = self.next.take()?;

        match current {
            JsIdentifierExpression(identifier) => identifier
                .name()
                .and_then(|reference| reference.value_token())
                .ok()
                .map(|value| value.token_text_trimmed()),
            JsStaticMemberExpression(member_expression) => match member_expression.member() {
                Ok(AnyJsName::JsName(name)) => {
                    self.next = member_expression.object().ok();
                    name.value_token()
                        .ok()
                        .map(|name| name.token_text_trimmed())
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{contains_a_test_pattern, is_test_each_pattern_callee};
    use biome_js_parser::{parse, JsParserOptions};
    use biome_js_syntax::{JsCallExpression, JsFileSource, JsTemplateExpression};
    use biome_rowan::AstNodeList;

    fn extract_call_expression(src: &str) -> JsCallExpression {
        let source_type = JsFileSource::js_module();
        let result = parse(src, source_type, JsParserOptions::default());
        let module = result
            .tree()
            .as_js_module()
            .unwrap()
            .items()
            .first()
            .unwrap();

        module
            .as_any_js_statement()
            .unwrap()
            .as_js_expression_statement()
            .unwrap()
            .expression()
            .unwrap()
            .as_js_call_expression()
            .unwrap()
            .clone()
    }

    fn extract_template(src: &str) -> JsTemplateExpression {
        let source_type = JsFileSource::js_module();
        let result = parse(src, source_type, JsParserOptions::default());
        let module = result
            .tree()
            .as_js_module()
            .unwrap()
            .items()
            .first()
            .unwrap();

        module
            .as_any_js_statement()
            .unwrap()
            .as_js_expression_statement()
            .unwrap()
            .expression()
            .unwrap()
            .as_js_template_expression()
            .unwrap()
            .clone()
    }

    #[test]
    fn matches_simple_call() {
        let call_expression = extract_call_expression("test();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );

        let call_expression = extract_call_expression("it();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression() {
        let call_expression = extract_call_expression("test.only();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn doesnt_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only.AHAHA();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(false)
        );
    }

    #[test]
    fn matches_simple_each() {
        let template = extract_template("describe.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("test.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xdescribe.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fdescribe.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.each``");
        assert!(is_test_each_pattern_callee(&template));
    }

    #[test]
    fn matches_skip_each() {
        let template = extract_template("describe.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("test.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xdescribe.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fdescribe.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.skip.each``");
        assert!(is_test_each_pattern_callee(&template));
    }

    #[test]
    fn matches_only_each() {
        let template = extract_template("describe.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("test.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xdescribe.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fdescribe.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.only.each``");
        assert!(is_test_each_pattern_callee(&template));
    }

    #[test]
    fn matches_failing_each() {
        let template = extract_template("test.failing.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.failing.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.failing.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.failing.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.failing.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.failing.each``");
        assert!(is_test_each_pattern_callee(&template));
    }

    #[test]
    fn matches_concurrent_each() {
        let template = extract_template("test.concurrent.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.concurrent.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.concurrent.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.concurrent.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.concurrent.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.concurrent.each``");
        assert!(is_test_each_pattern_callee(&template));
    }

    #[test]
    fn matches_concurrent_only_each() {
        let template = extract_template("test.concurrent.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.concurrent.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.concurrent.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.concurrent.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.concurrent.only.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.concurrent.only.each``");
        assert!(is_test_each_pattern_callee(&template));
    }

    #[test]
    fn matches_concurrent_skip_each() {
        let template = extract_template("test.concurrent.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("it.concurrent.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xtest.concurrent.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("xit.concurrent.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("ftest.concurrent.skip.each``");
        assert!(is_test_each_pattern_callee(&template));

        let template = extract_template("fit.concurrent.skip.each``");
        assert!(is_test_each_pattern_callee(&template));
    }
}
