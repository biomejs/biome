use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsNamedImportSpecifier, JsArrayExpression, JsArrowFunctionExpression,
    JsBinaryOperator, JsCallExpression, JsClassExpression, JsFunctionExpression,
    JsIdentifierExpression, JsImport, JsLanguage, JsObjectExpression, JsStaticMemberExpression,
    JsSyntaxToken, JsUnaryOperator, binding_ext::AnyJsBindingDeclaration, inner_string_text,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, TextRange};
use biome_rule_options::no_unnecessary_string_trim::NoUnnecessaryStringTrimOptions;

use crate::{JsRuleAction, services::semantic::Semantic, utils::same_reference::string_value};

declare_lint_rule! {
    /// Disallow `trim()` before `startsWith()` or `endsWith()` when only one side of the string matters.
    ///
    /// `startsWith()` only looks at the start of a string, so whitespace removed from the end by `trim()`
    /// can't change its result. The same is true for `endsWith()` and whitespace at the start.
    /// Use `trimStart()` before `startsWith()` and `trimEnd()` before `endsWith()` to remove only the
    /// whitespace that matters.
    ///
    /// The rule only reports calls where the searched text is known and the change can't affect the
    /// result. It ignores these calls:
    ///
    /// - `startsWith()` calls whose searched text ends with whitespace, and `endsWith()` calls whose
    ///   searched text starts with whitespace.
    /// - Calls whose searched text is a variable or expression that the rule can't work out.
    /// - Calls with a second argument, which sets the position to search from.
    /// - Calls that use optional chaining (`?.`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// value.trim().startsWith("-");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// value.trim().endsWith("-");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// value.trimStart().startsWith("-");
    /// value.trimEnd().endsWith("-");
    /// value.trim().startsWith("-", 1);
    /// value.trim().startsWith("foo ");
    /// value.trim().endsWith(suffix);
    /// ```
    ///
    pub NoUnnecessaryStringTrim {
        version: "next",
        name: "noUnnecessaryStringTrim",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-unnecessary-string-trim").same()],
        recommended: true,
        // Unsafe because the receiver of `trim()` may be an object with its own methods.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUnnecessaryStringTrim {
    type Query = Semantic<JsCallExpression>;
    type State = SearchMethod;
    type Signals = Option<Self::State>;
    type Options = NoUnnecessaryStringTrimOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional_chain() {
            return None;
        }
        let member = call.callee().ok()?;
        let member = member.as_js_static_member_expression()?;
        let method = match member_name(member)?.text_trimmed() {
            "startsWith" => SearchMethod::StartsWith,
            "endsWith" => SearchMethod::EndsWith,
            _ => return None,
        };

        let arguments = call.arguments().ok()?.args();
        if arguments.len() > 1 {
            return None;
        }
        let search = match arguments.first() {
            Some(argument) => Some(argument.ok()?.as_any_js_expression()?.clone()),
            None => None,
        };

        let trim_call = trim_call(call)?;
        if trim_call.arguments().ok()?.args().len() != 0 {
            return None;
        }
        let trim_member = trim_call.callee().ok()?;
        let trim_member = trim_member.as_js_static_member_expression()?;
        if member_name(trim_member)?.text_trimmed() != "trim" {
            return None;
        }

        let model = ctx.model();
        let receiver = trim_member.object().ok()?;
        if is_known_non_string(&receiver, model) || is_zod_string_schema(&receiver, model) {
            return None;
        }

        if let Some(search) = search {
            let mut visits_left = MAX_VISITED_EXPRESSIONS;
            let form = string_form(&search, model, &mut visits_left)?;
            let edge = match method {
                SearchMethod::StartsWith => form.last,
                SearchMethod::EndsWith => form.first,
            };
            if edge == Edge::Whitespace {
                return None;
            }
        }

        Some(method)
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let trim_call = trim_call(ctx.query())?;
        let trim_name = member_name(trim_call.callee().ok()?.as_js_static_member_expression()?)?;
        let range = TextRange::new(
            trim_name.text_trimmed_range().start(),
            trim_call.range().end(),
        );
        let (checked_side, ignored_side) = match method {
            SearchMethod::StartsWith => ("start", "end"),
            SearchMethod::EndsWith => ("end", "start"),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"trim()"</Emphasis>" call removes whitespace from both ends of the string, but "<Emphasis>{method}</Emphasis>" only needs it removed from the "{checked_side}"."
                },
            )
            .note(markup! {
                "Whitespace at the "{ignored_side}" of the string can't change the result of "<Emphasis>{method}</Emphasis>" here."
            })
            .note(markup! {
                "Use "<Emphasis>{method.replacement()}"()"</Emphasis>" to remove whitespace from the "{checked_side}" only."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, method: &Self::State) -> Option<JsRuleAction> {
        let trim_call = trim_call(ctx.query())?;
        let trim_name = member_name(trim_call.callee().ok()?.as_js_static_member_expression()?)?;
        let replacement = method.replacement();

        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(trim_name, make::ident(replacement));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{replacement}"()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// The string method called on the result of `trim()`.
#[derive(Clone, Copy, Debug)]
pub enum SearchMethod {
    StartsWith,
    EndsWith,
}

impl SearchMethod {
    /// Returns the trim method that removes only the whitespace this method can observe.
    const fn replacement(self) -> &'static str {
        match self {
            Self::StartsWith => "trimStart",
            Self::EndsWith => "trimEnd",
        }
    }
}

impl biome_console::fmt::Display for SearchMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::StartsWith => "startsWith()",
            Self::EndsWith => "endsWith()",
        })
    }
}

/// The number of expressions [`string_form`] looks at before giving up.
///
/// The limit is shared by the whole search, so it bounds the work done on
/// declarations that refer to each other, such as `const a = b + ""; const b = a + "";`,
/// and on `const` variables built by adding other `const` variables many times over.
const MAX_VISITED_EXPRESSIONS: u8 = 32;

/// Syntax kinds of expressions whose value is never a string.
const NON_STRING_KINDS: SyntaxKindSet<JsLanguage> = JsArrayExpression::KIND_SET
    .union(JsObjectExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET);

/// Returns the `trim()` call that `call`, a `startsWith()` or `endsWith()` call, is made on.
fn trim_call(call: &JsCallExpression) -> Option<JsCallExpression> {
    call.callee()
        .ok()?
        .as_js_static_member_expression()?
        .object()
        .ok()?
        .omit_parentheses()
        .as_js_call_expression()
        .cloned()
}

/// Returns the name token of `member`, such as `trim` in `value.trim`.
///
/// Returns `None` for private names such as `value.#trim`.
fn member_name(member: &JsStaticMemberExpression) -> Option<JsSyntaxToken> {
    member.member().ok()?.as_js_name()?.value_token().ok()
}

/// Returns `true` if `receiver` is a value that is never a string, such as an
/// array or a number, either written directly or stored in a `const` variable.
fn is_known_non_string(receiver: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(mut expression) = receiver.clone().omit_type_wrappers() else {
        return false;
    };
    if let AnyJsExpression::JsIdentifierExpression(identifier) = &expression {
        let Some(initializer) = const_initializer(identifier, model) else {
            return false;
        };
        expression = initializer;
    }
    if NON_STRING_KINDS.matches(expression.syntax().kind()) {
        return true;
    }
    expression
        .as_any_js_literal_expression()
        .is_some_and(|literal| literal.as_js_string_literal_expression().is_none())
}

/// Returns `true` if `receiver` is a `z.string()` call, where `z` is imported from
/// Zod. Zod schemas have their own `trim()`, `startsWith()`, and `endsWith()` methods,
/// and `trimStart()` and `trimEnd()` don't exist on them.
fn is_zod_string_schema(receiver: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(AnyJsExpression::JsCallExpression(call)) = receiver.clone().omit_type_wrappers()
    else {
        return false;
    };
    if call.is_optional_chain()
        || call
            .arguments()
            .is_ok_and(|arguments| arguments.args().len() != 0)
    {
        return false;
    }
    let Some(AnyJsExpression::JsStaticMemberExpression(member)) = call.callee().ok() else {
        return false;
    };
    if member_name(&member).is_none_or(|name| name.text_trimmed() != "string") {
        return false;
    }
    let Some(AnyJsExpression::JsIdentifierExpression(identifier)) = member.object().ok() else {
        return false;
    };
    let Some(declaration) = identifier
        .name()
        .ok()
        .and_then(|reference| model.binding(&reference))
        .and_then(|binding| binding.tree().declaration())
    else {
        return false;
    };
    let is_zod_namespace = match &declaration {
        AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => true,
        _ => AnyJsNamedImportSpecifier::cast_ref(declaration.syntax())
            .and_then(|specifier| specifier.imported_name())
            .is_some_and(|name| inner_string_text(&name) == "z"),
    };
    is_zod_namespace
        && declaration
            .syntax()
            .ancestors()
            .find_map(JsImport::cast)
            .and_then(|import| import.source_text().ok())
            .is_some_and(|source| source.text() == "zod" || source.text().starts_with("zod/"))
}

/// Returns the initializer of the `const` variable that `identifier` refers to.
///
/// Returns `None` if the variable isn't declared with `const`, has no initializer,
/// or is declared through destructuring, such as `const { a } = object`.
fn const_initializer(
    identifier: &JsIdentifierExpression,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    let binding = model.binding(&identifier.name().ok()?)?;
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = binding.tree().declaration()?
    else {
        return None;
    };
    if !declarator.declaration()?.is_const() {
        return None;
    }
    declarator
        .initializer()?
        .expression()
        .ok()?
        .omit_type_wrappers()
}

/// The first or last character of a value's string form.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Edge {
    /// The string form is empty.
    Empty,
    Whitespace,
    Other,
}

impl Edge {
    fn from_char(character: Option<char>) -> Self {
        match character {
            None => Self::Empty,
            // `trim()` also removes the byte order mark, which Rust doesn't treat as whitespace.
            Some(character) if character.is_whitespace() || character == '\u{feff}' => {
                Self::Whitespace
            }
            Some(_) => Self::Other,
        }
    }
}

/// What the rule knows about the string that `startsWith()` or `endsWith()`
/// converts its search argument to.
#[derive(Clone, Copy, Debug)]
struct StringForm {
    /// Whether the value is a string. Other values, such as `1`, `true`, or
    /// `undefined`, convert to text that is never empty and has no whitespace.
    is_string: bool,
    first: Edge,
    last: Edge,
}

impl StringForm {
    const NON_STRING: Self = Self {
        is_string: false,
        first: Edge::Other,
        last: Edge::Other,
    };

    fn from_text(text: &str) -> Self {
        Self {
            is_string: true,
            first: Edge::from_char(text.chars().next()),
            last: Edge::from_char(text.chars().next_back()),
        }
    }

    /// Returns the string form of `left + right`.
    fn concat(left: Self, right: Self) -> Self {
        if !left.is_string && !right.is_string {
            // Adding two values that aren't strings produces a number, a bigint,
            // or `NaN`, none of which contain whitespace.
            return Self::NON_STRING;
        }
        Self {
            is_string: true,
            first: if left.first == Edge::Empty {
                right.first
            } else {
                left.first
            },
            last: if right.last == Edge::Empty {
                left.last
            } else {
                right.last
            },
        }
    }
}

/// Returns the string form of `expression` if it can be determined without running the code.
///
/// Handles literals, template literals without `${}`, `undefined`, `void` expressions,
/// `+` concatenations, and `const` variables initialized with any of these.
///
/// Returns `None` once it has looked at `visits_left` expressions, and for strings with
/// escape sequences that can't be decoded, such as the legacy octal escape `\40`.
fn string_form(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    visits_left: &mut u8,
) -> Option<StringForm> {
    *visits_left = visits_left.checked_sub(1)?;
    let expression = expression.clone().omit_type_wrappers()?;
    match &expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let reference = identifier.name().ok()?;
            if model.binding(&reference).is_none() {
                return (reference.value_token().ok()?.text_trimmed() == "undefined")
                    .then_some(StringForm::NON_STRING);
            }
            string_form(&const_initializer(identifier, model)?, model, visits_left)
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            (unary.operator().ok()? == JsUnaryOperator::Void).then_some(StringForm::NON_STRING)
        }
        AnyJsExpression::JsBinaryExpression(binary) => {
            if binary.operator().ok()? != JsBinaryOperator::Plus {
                return None;
            }
            let left = string_form(&binary.left().ok()?, model, visits_left)?;
            let right = string_form(&binary.right().ok()?, model, visits_left)?;
            Some(StringForm::concat(left, right))
        }
        AnyJsExpression::JsTemplateExpression(template) if template.tag().is_some() => None,
        _ => match expression.as_static_value()? {
            StaticValue::String(token) => Some(StringForm::from_text(&string_value(
                inner_string_text(&token),
            )?)),
            StaticValue::EmptyString(_) => Some(StringForm::from_text("")),
            StaticValue::Boolean(_)
            | StaticValue::Null(_)
            | StaticValue::Undefined(_)
            | StaticValue::Number(_)
            | StaticValue::BigInt(_) => Some(StringForm::NON_STRING),
        },
    }
}
