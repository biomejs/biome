use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression,
    JsNewOrCallExpression, binding_ext::AnyJsBindingDeclaration, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_global_regex_in_replace_all::UseGlobalRegexInReplaceAllOptions;

declare_lint_rule! {
    /// Require regular expressions passed to `replaceAll()` to use the global flag.
    ///
    /// When `String.prototype.replaceAll()` receives a regular expression, that regular
    /// expression must have the `g` (global) flag. Without it, `replaceAll()` throws a
    /// `TypeError` when the code runs, instead of replacing any text.
    ///
    /// This rule checks regular expression literals, such as `/\s+/`, and calls to
    /// `RegExp()` or `new RegExp()` whose flags are written directly in the code.
    /// It also follows variables that are initialized with one of these and never reassigned.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// withSpaces.replaceAll(/\s+/, ",");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// withSpaces.replaceAll(new RegExp("\\s+", "i"), ",");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const spaces = /\s+/;
    /// withSpaces.replaceAll(spaces, ",");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// withSpaces.replaceAll(/\s+/g, ",");
    /// withSpaces.replaceAll(new RegExp("\\s+", "g"), ",");
    /// withSpaces.replaceAll(" ", ",");
    /// withSpaces.replace(/\s+/, ",");
    /// ```
    ///
    pub UseGlobalRegexInReplaceAll {
        version: "2.6.7",
        name: "useGlobalRegexInReplaceAll",
        language: "js",
        sources: &[RuleSource::Oxc("bad-replace-all-arg").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseGlobalRegexInReplaceAll {
    type Query = Semantic<JsCallExpression>;
    /// The regular expression without the `g` flag. It is either the first argument
    /// of the call, or the initializer of the variable passed as the first argument.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = UseGlobalRegexInReplaceAllOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        let callee = AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        if callee.member_name()?.text() != "replaceAll" {
            return None;
        }

        let argument = first_argument(call)?;
        let regex = resolve_variable(&argument, model).unwrap_or(argument);
        (regex_has_global_flag(&regex, model) == Some(false)).then_some(regex)
    }

    fn diagnostic(ctx: &RuleContext<Self>, regex: &Self::State) -> Option<RuleDiagnostic> {
        let argument = first_argument(ctx.query())?;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            argument.range(),
            markup! {
                "This regular expression passed to "<Emphasis>"replaceAll()"</Emphasis>" is missing the "<Emphasis>"g"</Emphasis>" flag."
            },
        );
        let diagnostic = if regex.range() == argument.range() {
            diagnostic
        } else {
            diagnostic.detail(
                regex.range(),
                markup! { "The regular expression is created here." },
            )
        };
        Some(
            diagnostic
                .note(markup! {
                    "When "<Emphasis>"replaceAll()"</Emphasis>" receives a regular expression without the "<Emphasis>"g"</Emphasis>" (global) flag, it throws a "<Emphasis>"TypeError"</Emphasis>" instead of replacing text."
                })
                .note(markup! {
                    "Add the "<Emphasis>"g"</Emphasis>" flag to the regular expression, or use "<Emphasis>"replace()"</Emphasis>" to replace only the first match."
                }),
        )
    }
}

/// Returns the first argument of `call`, without parentheses or TypeScript type wrappers.
/// Returns `None` if there is no argument or the first argument is a spread.
fn first_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let argument = call.arguments().ok()?.args().first()?.ok()?;
    argument.as_any_js_expression()?.inner_expression()
}

/// Returns the initializer of the variable that `expression` refers to, if the variable
/// is declared with an initializer and is never reassigned.
fn resolve_variable(expression: &AnyJsExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    let reference = expression.as_js_identifier_expression()?.name().ok()?;
    let binding = model.binding(&reference)?;
    if binding.all_writes().next().is_some() {
        return None;
    }
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = binding.tree().declaration()?
    else {
        return None;
    };
    declarator.initializer()?.expression().ok()?.inner_expression()
}

/// Returns whether `expression` creates a regular expression with the `g` flag.
///
/// Returns `None` if `expression` doesn't create a regular expression, or if its flags
/// can't be known without running the code.
fn regex_has_global_flag(expression: &AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    if let AnyJsExpression::AnyJsLiteralExpression(literal) = expression {
        let (_, flags) = literal.as_js_regex_literal_expression()?.decompose().ok()?;
        return Some(flags.text().contains('g'));
    }

    let constructor = JsNewOrCallExpression::cast_ref(expression.syntax())?;
    if !is_regexp_object(&constructor.callee().ok()?, model) {
        return None;
    }
    let Some(arguments) = constructor.arguments() else {
        // `new RegExp` without parentheses creates a regular expression without flags.
        return Some(false);
    };
    let mut arguments = arguments.args().iter();
    let Some(pattern) = arguments.next() else {
        return Some(false);
    };
    // A spread pattern hides the position of the flags argument.
    let AnyJsCallArgument::AnyJsExpression(pattern) = pattern.ok()? else {
        return None;
    };
    if let Some(flags) = arguments.next() {
        let flags = flags.ok()?;
        let flags = flags.as_any_js_expression()?.inner_expression()?;
        if is_tagged_template(&flags) {
            return None;
        }
        return Some(flags.as_static_value()?.as_string_constant()?.contains('g'));
    }

    // Without a flags argument, `RegExp` copies the flags of a regular expression
    // pattern, and uses no flags for a string pattern.
    let pattern = pattern.inner_expression()?;
    if let Some(template) = pattern.as_js_template_expression() {
        return template.tag().is_none().then_some(false);
    }
    if pattern.as_static_value().is_some() {
        Some(false)
    } else {
        regex_has_global_flag(&pattern, model)
    }
}

/// Returns whether `expression` is a tagged template, such as ``tag`g` ``.
/// The tag function decides the value, so the text of the template can't be trusted.
fn is_tagged_template(expression: &AnyJsExpression) -> bool {
    expression
        .as_js_template_expression()
        .is_some_and(|template| template.tag().is_some())
}

/// Returns whether `expression` refers to the global `RegExp`, and not to a local
/// variable with the same name.
fn is_regexp_object(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    match expression
        .clone()
        .omit_parentheses()
        .as_any_global_identifier_expression()
        .and_then(|expression| global_identifier(&expression))
    {
        Some((reference, name)) => match model.binding(&reference) {
            Some(_) if !reference.is_global_this() && !reference.has_name("window") => false,
            _ => name.text() == "RegExp",
        },
        None => false,
    }
}
