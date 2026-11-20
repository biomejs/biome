use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallExpression, JsSyntaxToken,
    JsVariableDeclarator, T,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_canvas_to_image::NoCanvasToImageOptions;
use biome_string_case::StrLikeExtension;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow converting a canvas to an image only to draw it onto another canvas.
    ///
    /// [`drawImage()`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    /// accepts a canvas as the image to draw.
    /// Turning a canvas into a data URL and loading that URL as an image does extra work
    /// that `drawImage()` doesn't need: the browser encodes the pixels into an image file
    /// and then decodes the file back into pixels.
    /// Copying pixels with `getImageData()` and writing them back with `putImageData()` also
    /// copies every pixel through JavaScript, when `drawImage()` can copy them directly.
    ///
    /// The rule reports two patterns:
    ///
    /// - the first argument of `drawImage()` is the result of `loadImage(canvas.toDataURL())`.
    ///   The rule only checks a `toDataURL()` call on a variable or property whose name contains `canvas`,
    ///   in any letter case, such as `canvas`, `sourceCanvas`, or `this.canvas`.
    /// - the first argument of `putImageData()` is the result of a `getImageData()` call.
    ///
    /// The result can be passed directly, after `await`, or through a `const` variable that
    /// is declared before the call and used only once.
    /// The rule recognizes these patterns by their names; it doesn't check whether the objects
    /// really are canvases.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const image = await loadImage(canvas.toDataURL());
    /// context.drawImage(image, 0, 0);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const imageData = sourceContext.getImageData(0, 0, width, height);
    /// targetContext.putImageData(imageData, 0, 0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// context.drawImage(canvas, 0, 0);
    /// targetContext.drawImage(sourceCanvas, 0, 0);
    ///
    /// // The pixels are changed before they're written back.
    /// const imageData = context.getImageData(0, 0, width, height);
    /// imageData.data[0] = 0;
    /// context.putImageData(imageData, 0, 0);
    /// ```
    ///
    pub NoCanvasToImage {
        version: "next",
        name: "noCanvasToImage",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-canvas-to-image").same()],
        recommended: false,
    }
}

impl Rule for NoCanvasToImage {
    type Query = Semantic<JsCallExpression>;
    type State = CanvasCopy;
    type Signals = Option<Self::State>;
    type Options = NoCanvasToImageOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let kind = match method_name(call)?.text_trimmed() {
            "drawImage" => CanvasCopy::DataUrl,
            "putImageData" => CanvasCopy::ImageData,
            _ => return None,
        };
        let image = first_argument(call)?;
        let source = resolve_source(image, model)?;
        let source = source.as_js_call_expression()?;
        let is_copy = match kind {
            CanvasCopy::DataUrl => is_load_image_from_canvas_data_url(source),
            CanvasCopy::ImageData => is_method_call(source, "getImageData"),
        };
        is_copy.then_some(kind)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = first_argument(ctx.query())?.range();
        Some(match state {
            CanvasCopy::DataUrl => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This image is loaded from a canvas only to be drawn with "<Emphasis>"drawImage()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Converting the canvas to a data URL and loading it as an image encodes and decodes every pixel, but "<Emphasis>"drawImage()"</Emphasis>" accepts the canvas itself."
            })
            .note(markup! {
                "Pass the canvas to "<Emphasis>"drawImage()"</Emphasis>" instead."
            }),
            CanvasCopy::ImageData => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "These pixels are copied from a canvas with "<Emphasis>"getImageData()"</Emphasis>" only to be written to a canvas with "<Emphasis>"putImageData()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Copying pixels through "<Emphasis>"getImageData()"</Emphasis>" is slower than drawing one canvas onto another with "<Emphasis>"drawImage()"</Emphasis>"."
            })
            .note(markup! {
                "Pass the source canvas to "<Emphasis>"drawImage()"</Emphasis>" instead."
            }),
        })
    }
}

/// The kind of unnecessary canvas copy passed as the first argument of the queried call.
#[derive(Clone, Copy)]
pub enum CanvasCopy {
    /// `drawImage(loadImage(canvas.toDataURL()))`
    DataUrl,
    /// `putImageData(context.getImageData())`
    ImageData,
}

/// Returns the method name of `call` if it's a plain method call such as `object.method()`.
///
/// Returns `None` for optional calls such as `object?.method()` or `object.method?.()`,
/// computed members such as `object["method"]()`, and private methods.
fn method_name(call: &JsCallExpression) -> Option<JsSyntaxToken> {
    if call.optional_chain_token().is_some() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    if member.operator_token().ok()?.kind() == T![?.] {
        return None;
    }
    member.member().ok()?.as_js_name()?.value_token().ok()
}

fn is_method_call(call: &JsCallExpression, method: &str) -> bool {
    method_name(call).is_some_and(|name| name.text_trimmed() == method)
}

/// Returns the first argument of `call` unless it's missing or a spread argument.
fn first_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    match call.arguments().ok()?.args().first()?.ok()? {
        AnyJsCallArgument::AnyJsExpression(expression) => Some(expression),
        AnyJsCallArgument::JsSpread(_) => None,
    }
}

/// Follows the argument back to the expression that produced its value.
///
/// Removes any `await` around the argument. If the argument is then a variable, follows it to
/// its initializer, and removes any `await` around the initializer. The variable is followed
/// only if it's declared with `const`, isn't destructured, is declared before the argument,
/// and the argument is its only reference. Otherwise this returns `None`.
fn resolve_source(expression: AnyJsExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    let expression = omit_await(expression)?;
    let AnyJsExpression::JsIdentifierExpression(identifier) = &expression else {
        return Some(expression);
    };
    let reference = identifier.name().ok()?;
    let binding = model.binding(&reference)?;
    // Other uses of the variable may depend on the image or pixels it holds.
    if binding.all_references().nth(1).is_some() {
        return None;
    }
    if binding.syntax().text_trimmed_range().start()
        > reference.syntax().text_trimmed_range().start()
    {
        return None;
    }
    let declarator = JsVariableDeclarator::cast(binding.syntax().parent()?)?;
    if !declarator.declaration()?.is_const() {
        return None;
    }
    omit_await(declarator.initializer()?.expression().ok()?)
}

fn omit_await(mut expression: AnyJsExpression) -> Option<AnyJsExpression> {
    loop {
        expression = expression.omit_parentheses();
        let AnyJsExpression::JsAwaitExpression(await_expression) = expression else {
            return Some(expression);
        };
        expression = await_expression.argument().ok()?;
    }
}

/// Whether `call` is `loadImage(canvas.toDataURL())`, where the object of `toDataURL()`
/// is a variable or a non-computed property whose name contains `canvas` in any letter case.
fn is_load_image_from_canvas_data_url(call: &JsCallExpression) -> bool {
    if call.optional_chain_token().is_some() {
        return false;
    }
    let is_load_image = call.callee().is_ok_and(|callee| {
        callee
            .omit_parentheses()
            .as_js_identifier_expression()
            .and_then(|identifier| identifier.name().ok())
            .is_some_and(|name| name.has_name("loadImage"))
    });
    if !is_load_image {
        return false;
    }
    let Some(AnyJsExpression::JsCallExpression(data_url_call)) =
        first_argument(call).map(AnyJsExpression::omit_parentheses)
    else {
        return false;
    };
    if !is_method_call(&data_url_call, "toDataURL") {
        return false;
    }
    data_url_call
        .callee()
        .ok()
        .and_then(|callee| {
            callee
                .omit_parentheses()
                .as_js_static_member_expression()?
                .object()
                .ok()
        })
        .is_some_and(|object| is_canvas_like(&object.omit_parentheses()))
}

/// Whether `expression` is a variable such as `sourceCanvas`, or a non-computed property such as
/// `this.canvas`, whose name contains `canvas` in any letter case.
fn is_canvas_like(expression: &AnyJsExpression) -> bool {
    let name = match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .ok()
            .and_then(|name| name.value_token().ok()),
        AnyJsExpression::JsStaticMemberExpression(member) => member
            .member()
            .ok()
            .and_then(|member| member.as_js_name()?.value_token().ok()),
        _ => None,
    };
    name.is_some_and(|name| name.text_trimmed().contains_ignore_ascii_case("canvas"))
}
