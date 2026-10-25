use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsTemplateElement, JsCallExpression,
    JsStaticMemberExpression, JsSyntaxToken,
};
use biome_rowan::{AstNodeList, AstSeparatedList, BatchMutationExt, TextRange, TextSize};
use biome_rule_options::no_console_spaces::NoConsoleSpacesOptions;

declare_lint_rule! {
    /// Disallow leading and trailing spaces in `console` method arguments.
    ///
    /// `console.log()`, `console.debug()`, `console.info()`, `console.warn()`, and `console.error()`
    /// put a space between their arguments when printing them.
    /// A string argument that starts or ends with a space next to another argument
    /// prints two spaces instead of one.
    ///
    /// The first argument may start with a space, and the last argument may end with one,
    /// because nothing is printed before or after them.
    /// Strings that contain only one space, or that start or end with two or more spaces,
    /// are allowed because the extra spacing is likely intentional.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log("abc ", "def");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// console.error("abc", " def");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// console.warn(`abc `, "def");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// console.log("abc", "def");
    /// console.log(" abc", "def ");
    /// console.log("abc  ", "def");
    /// console.log("abc\t", "def");
    /// console.log("abc", " ", "def");
    /// ```
    ///
    pub NoConsoleSpaces {
        version: "next",
        name: "noConsoleSpaces",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-console-spaces").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConsoleSpaces {
    type Query = Ast<JsCallExpression>;
    type State = SpaceState;
    type Signals = Box<[Self::State]>;
    type Options = NoConsoleSpacesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if console_method_callee(call).is_none() {
            return Box::default();
        }
        let Ok(arguments) = call.arguments() else {
            return Box::default();
        };
        let arguments = arguments.args();
        let last_index = arguments.len().saturating_sub(1);
        let mut signals = Vec::new();
        for (index, argument) in arguments.iter().enumerate() {
            let Ok(AnyJsCallArgument::AnyJsExpression(argument)) = argument else {
                continue;
            };
            let Some(content) = ArgumentContent::from_expression(&argument.omit_parentheses())
            else {
                continue;
            };
            if index != 0
                && let Some(state) = content.leading_space()
            {
                signals.push(state);
            }
            if index != last_index
                && let Some(state) = content.trailing_space()
            {
                signals.push(state);
            }
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let method = console_method_callee(ctx.query())?
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?;
        let method = method.text_trimmed();
        let message = match state.position {
            SpacePosition::Leading => markup! { "This argument starts with a space." },
            SpacePosition::Trailing => markup! { "This argument ends with a space." },
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.range, message).note(markup! {
                <Emphasis>"console."{method}"()"</Emphasis>" already prints a space between its arguments, so this space adds a second one."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let token = &state.token;
        let offset = usize::from(state.range.start() - token.text_trimmed_range().start());
        let text = token.text_trimmed();
        let new_text = format!("{}{}", &text[..offset], &text[offset + 1..]);
        let new_token = JsSyntaxToken::new_detached(token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state.position {
                SpacePosition::Leading => markup! { "Remove the leading space." }.to_owned(),
                SpacePosition::Trailing => markup! { "Remove the trailing space." }.to_owned(),
            },
            mutation,
        ))
    }
}

/// A space to remove from a string or template argument.
pub struct SpaceState {
    /// The token that contains the space: a string literal or a template chunk.
    token: JsSyntaxToken,
    /// The range of the space.
    range: TextRange,
    position: SpacePosition,
}

#[derive(Clone, Copy)]
enum SpacePosition {
    Leading,
    Trailing,
}

const CONSOLE_METHODS: &[&str] = &["debug", "error", "info", "log", "warn"];

/// Returns the callee of `call` if it is a non-optional call to one of the
/// [`CONSOLE_METHODS`] on the `console` identifier, such as `console.log(...)`.
fn console_method_callee(call: &JsCallExpression) -> Option<JsStaticMemberExpression> {
    if call.is_optional() {
        return None;
    }
    let AnyJsExpression::JsStaticMemberExpression(callee) = call.callee().ok()?.omit_parentheses()
    else {
        return None;
    };
    if callee.is_optional() {
        return None;
    }
    let object = callee.object().ok()?.omit_parentheses();
    let object = object.as_js_identifier_expression()?.name().ok()?;
    if object.value_token().ok()?.text_trimmed() != "console" {
        return None;
    }
    let member = callee.member().ok()?;
    let member = member.as_js_name()?.value_token().ok()?;
    CONSOLE_METHODS
        .contains(&member.text_trimmed())
        .then_some(callee)
}

/// The first and last pieces of text in a string or untagged template argument.
///
/// For a string literal, both pieces are the content between the quotes.
/// For a template, they are the first and last chunks of text, if the template
/// starts or ends with text instead of a `${}` substitution.
struct ArgumentContent {
    first: Option<TextPiece>,
    last: Option<TextPiece>,
}

/// Text inside a string or template argument.
struct TextPiece {
    token: JsSyntaxToken,
    /// The byte offset of the text inside the token, after any opening quote.
    start: usize,
    /// The byte offset of the end of the text inside the token, before any closing quote.
    end: usize,
    /// Whether this text is a template chunk with a `${}` substitution on its
    /// inner side: after the first chunk, or before the last chunk.
    has_substitution: bool,
}

impl ArgumentContent {
    fn from_expression(expression: &AnyJsExpression) -> Option<Self> {
        match expression {
            AnyJsExpression::AnyJsLiteralExpression(literal) => {
                let token = literal
                    .as_js_string_literal_expression()?
                    .value_token()
                    .ok()?;
                let len = token.text_trimmed().len();
                if len < 2 {
                    return None;
                }
                let piece = || TextPiece {
                    token: token.clone(),
                    start: 1,
                    end: len - 1,
                    has_substitution: false,
                };
                Some(Self {
                    first: Some(piece()),
                    last: Some(piece()),
                })
            }
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    return None;
                }
                let elements = template.elements();
                let count = elements.len();
                let chunk_piece = |element: Option<AnyJsTemplateElement>| {
                    let token = element?
                        .as_js_template_chunk_element()?
                        .template_chunk_token()
                        .ok()?;
                    let end = token.text_trimmed().len();
                    Some(TextPiece {
                        token,
                        start: 0,
                        end,
                        has_substitution: count > 1,
                    })
                };
                Some(Self {
                    first: chunk_piece(elements.first()),
                    last: chunk_piece(elements.last()),
                })
            }
            _ => None,
        }
    }

    /// Returns the leading space if the argument starts with exactly one space
    /// and contains more than that space.
    fn leading_space(&self) -> Option<SpaceState> {
        let piece = self.first.as_ref()?;
        let text = piece.text().as_bytes();
        if text.first() != Some(&b' ') {
            return None;
        }
        let is_single_space = match text.get(1) {
            Some(next) => *next != b' ',
            None => piece.has_substitution,
        };
        is_single_space.then(|| piece.space_at(piece.start, SpacePosition::Leading))
    }

    /// Returns the trailing space if the argument ends with exactly one space,
    /// contains more than that space, and the space isn't escaped by a backslash.
    fn trailing_space(&self) -> Option<SpaceState> {
        let piece = self.last.as_ref()?;
        let text = piece.text().as_bytes();
        let (&last, rest) = text.split_last()?;
        if last != b' ' {
            return None;
        }
        let is_single_space = match rest.last() {
            Some(previous) => *previous != b' ',
            None => piece.has_substitution,
        };
        if !is_single_space {
            return None;
        }
        // `"a\ "` is an escaped space. Removing the space would leave a
        // backslash that escapes the closing quote.
        let backslashes = rest.iter().rev().take_while(|&&byte| byte == b'\\').count();
        if backslashes % 2 != 0 {
            return None;
        }
        Some(piece.space_at(piece.end - 1, SpacePosition::Trailing))
    }
}

impl TextPiece {
    fn text(&self) -> &str {
        &self.token.text_trimmed()[self.start..self.end]
    }

    fn space_at(&self, offset: usize, position: SpacePosition) -> SpaceState {
        let start = self.token.text_trimmed_range().start() + TextSize::from(offset as u32);
        SpaceState {
            token: self.token.clone(),
            range: TextRange::at(start, TextSize::from(1)),
            position,
        }
    }
}
