use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use crate::utils::can_be_variable_name;
use crate::utils::rename::RenameSymbolExtensions;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::{ReferencesExtensions, Scope, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody, AnyJsIdentifierReference,
    JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsCatchClause, JsCatchDeclaration, JsFormalParameter, JsFunctionExpression,
    JsIdentifierBinding, JsParameterList, JsParameters,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, WalkEvent, declare_node_union};
use biome_rule_options::use_catch_error_name::UseCatchErrorNameOptions;

declare_lint_rule! {
    /// Require a specific name for caught errors.
    ///
    /// When an error is caught, it is stored in a variable whose name you choose.
    /// Using the same name for this variable everywhere makes error handling code easier to read and to search.
    ///
    /// The rule checks the variable of:
    ///
    /// - `try...catch` statements;
    /// - functions passed to `promise.catch(...)`;
    /// - functions passed as the second argument of `promise.then(..., ...)`, which handle errors.
    ///
    /// The following names are always allowed:
    ///
    /// - the configured name, followed by any number of underscores, such as `error_`.
    ///   An underscore avoids a clash with another variable of the same name.
    /// - names that end with the configured name, such as `fsError` or `parse_error`.
    ///   These names describe what kind of error is stored.
    /// - `_`, when the variable is never used.
    /// - names listed in the [`ignore`](#ignore) option.
    ///
    /// The fix renames the variable and all its uses.
    /// When the configured name is already used by another variable, the fix adds underscores at the end of the name, such as `error_`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {} catch (err) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// promise.catch((e) => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// promise.then(undefined, function (reason) {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {} catch (_) {
    ///     console.log(_);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// try {} catch (error) {}
    ///
    /// promise.catch((error) => {});
    ///
    /// promise.then(undefined, function (error) {});
    ///
    /// try {} catch (_) {}
    ///
    /// try {} catch (fsError) {}
    ///
    /// try {} catch (error_) {
    ///     const error = new Error("Something went wrong");
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `name`
    ///
    /// Default: `"error"`
    ///
    /// The required name of caught errors.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "name": "exception"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// try {} catch (error) {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// try {} catch (exception) {}
    /// ```
    ///
    /// ### `ignore`
    ///
    /// Default: `[]`
    ///
    /// A list of names that are always allowed, in addition to the configured name.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": ["exception"]
    ///   }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// try {} catch (exception) {}
    /// ```
    ///
    pub UseCatchErrorName {
        version: "2.6.7",
        name: "useCatchErrorName",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("catch-error-name").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseCatchErrorName {
    type Query = Semantic<JsIdentifierBinding>;
    /// The number of underscores that the fix adds after the configured name,
    /// so that the new name doesn't clash with another variable.
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = UseCatchErrorNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let handler = AnyErrorHandler::from_error_parameter(binding)?;
        let name_token = binding.name_token().ok()?;
        let name = name_token.text_trimmed();
        let options = ctx.options();
        let expected = options.name();
        let ignore = options.ignore();
        if is_allowed_name(name, expected, ignore)
            || is_allowed_name(name.trim_end_matches('_'), expected, ignore)
        {
            return None;
        }
        let model = ctx.model();
        if name == "_" && binding.all_references(model).next().is_none() {
            return None;
        }
        if !can_be_variable_name(expected) {
            return Some(0);
        }
        let mut candidate = String::from(expected);
        let mut underscores = 0;
        while has_name_clash(model, binding, &handler, &candidate) {
            candidate.push('_');
            underscores += 1;
        }
        Some(underscores)
    }

    fn diagnostic(ctx: &RuleContext<Self>, underscores: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        let name_token = binding.name_token().ok()?;
        let expected = ctx.options().name();
        let fixed_name = FixedName {
            name: expected,
            underscores: *underscores,
        };
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            binding.range(),
            markup! {
                "The caught error is named "<Emphasis>{name_token.text_trimmed()}</Emphasis>" instead of "<Emphasis>{expected}</Emphasis>"."
            },
        )
        .note(markup! {
            "Using the same name for caught errors everywhere makes error handling code easier to read and search."
        });
        Some(if !can_be_variable_name(expected) {
            diagnostic.note(markup! {
                "The configured name "<Emphasis>{expected}</Emphasis>" can't be used as a variable name. Change the "<Emphasis>"name"</Emphasis>" option of this rule to a valid variable name."
            })
        } else if *underscores == 0 {
            diagnostic.note(markup! {
                "Rename the variable to "<Emphasis>{fixed_name}</Emphasis>"."
            })
        } else {
            diagnostic.note(markup! {
                "Rename the variable to "<Emphasis>{fixed_name}</Emphasis>", because the name "<Emphasis>{expected}</Emphasis>" is already used here."
            })
        })
    }

    fn action(ctx: &RuleContext<Self>, underscores: &Self::State) -> Option<JsRuleAction> {
        let expected = ctx.options().name();
        if !can_be_variable_name(expected) {
            return None;
        }
        let fixed_name = FixedName {
            name: expected,
            underscores: *underscores,
        };
        let new_name = format!("{expected}{}", "_".repeat(*underscores));
        let mut mutation = ctx.root().begin();
        if !mutation.rename_node_declaration(ctx.model(), ctx.query(), &new_name) {
            return None;
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Rename the variable to "<Emphasis>{fixed_name}</Emphasis>"."
            },
            mutation,
        ))
    }
}

declare_node_union! {
    /// A node that declares a variable for a caught error: a `catch` clause, or a function that
    /// handles a rejected promise.
    AnyErrorHandler = JsCatchClause | JsFunctionExpression | JsArrowFunctionExpression
}

impl AnyErrorHandler {
    /// Returns the node that declares `binding` as the variable of a caught error, or `None` if
    /// `binding` is any other variable.
    ///
    /// A function declares a caught error when `binding` is its first parameter and the function
    /// is the only argument of a `.catch()` call or the second argument of a `.then()` call.
    /// Calls with a spread argument, such as `promise.then(...handlers)`, and optional calls,
    /// such as `promise.catch?.(handler)`, are not checked.
    fn from_error_parameter(binding: &JsIdentifierBinding) -> Option<Self> {
        let parent = binding.syntax().parent()?;
        if let Some(declaration) = JsCatchDeclaration::cast_ref(&parent) {
            return declaration.parent::<JsCatchClause>().map(Self::from);
        }
        let function = if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&parent) {
            Self::from(arrow)
        } else {
            let parameter = JsFormalParameter::cast(parent)?;
            let list = parameter.parent::<JsParameterList>()?;
            if list.first()?.ok()?.syntax() != parameter.syntax() {
                return None;
            }
            Self::cast(list.parent::<JsParameters>()?.syntax().parent()?)?
        };
        let argument = AnyJsExpression::cast_ref(function.syntax())?.outer_expression()?;
        let arguments = argument.parent::<JsCallArgumentList>()?;
        let call = arguments
            .parent::<JsCallArguments>()?
            .parent::<JsCallExpression>()?;
        if call.is_optional() {
            return None;
        }
        let expected_arguments = match call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_static_member_expression()?
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
        {
            "catch" => 1,
            "then" => 2,
            _ => return None,
        };
        if arguments.len() != expected_arguments
            || arguments
                .iter()
                .any(|argument| matches!(argument, Ok(AnyJsCallArgument::JsSpread(_))))
        {
            return None;
        }
        let last = arguments.last()?.ok()?;
        (last.syntax() == argument.syntax()).then_some(function)
    }

    /// Returns the scope of the block that runs when the error is caught, if any.
    ///
    /// A variable declared with `let`, `const`, `class` or `function` directly in this block
    /// can't share its name with the caught error.
    fn body_scope(&self, model: &SemanticModel) -> Option<Scope> {
        let body = match self {
            Self::JsCatchClause(clause) => clause.body().ok()?.into_syntax(),
            Self::JsFunctionExpression(function) => function.body().ok()?.into_syntax(),
            Self::JsArrowFunctionExpression(arrow) => match arrow.body().ok()? {
                AnyJsFunctionBody::JsFunctionBody(body) => body.into_syntax(),
                AnyJsFunctionBody::AnyJsExpression(_) => return None,
            },
        };
        Some(model.scope(&body))
    }
}

/// Returns `true` if `name` doesn't need to be reported: it ends with `expected`, ends with
/// `expected` with its first letter in uppercase, or is one of the `ignore` names.
fn is_allowed_name(name: &str, expected: &str, ignore: &[Box<str>]) -> bool {
    name.ends_with(expected)
        || ends_with_capitalized(name, expected)
        || ignore.iter().any(|ignored| ignored.as_ref() == name)
}

/// Returns `true` if `name` ends with `suffix` with its first letter in uppercase, such as
/// `fsError` with the suffix `error`.
fn ends_with_capitalized(name: &str, suffix: &str) -> bool {
    let mut suffix_chars = suffix.chars();
    let Some(first) = suffix_chars.next() else {
        return false;
    };
    name.strip_suffix(suffix_chars.as_str())
        .and_then(|head| head.chars().next_back())
        .is_some_and(|last| first.to_uppercase().eq([last]))
}

/// Returns `true` if renaming `binding` to `name` would change which variable some code refers
/// to, or would declare `name` twice in the same block.
fn has_name_clash(
    model: &SemanticModel,
    binding: &JsIdentifierBinding,
    handler: &AnyErrorHandler,
    name: &str,
) -> bool {
    if model
        .scope(binding.syntax())
        .ancestors()
        .any(|scope| scope.get_binding(name).is_some())
        || handler
            .body_scope(model)
            .is_some_and(|scope| scope.get_binding(name).is_some())
        || binding.all_references(model).any(|reference| {
            reference
                .scope()
                .ancestors()
                .any(|scope| scope.get_binding(name).is_some())
        })
    {
        return true;
    }
    // A use of an undeclared variable, such as a global, would refer to the
    // renamed variable instead.
    handler
        .syntax()
        .preorder_of_kinds(AnyJsIdentifierReference::KIND_SET)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => AnyJsIdentifierReference::cast(node),
            WalkEvent::Leave(_) => None,
        })
        .any(|reference| {
            reference
                .value_token()
                .is_ok_and(|token| token.text_trimmed() == name)
                && model.binding(&reference).is_none()
        })
}

/// The name that the fix gives to a caught error: the configured name followed by underscores.
struct FixedName<'a> {
    name: &'a str,
    underscores: usize,
}

impl biome_console::fmt::Display for FixedName<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.name)?;
        for _ in 0..self.underscores {
            fmt.write_str("_")?;
        }
        Ok(())
    }
}
