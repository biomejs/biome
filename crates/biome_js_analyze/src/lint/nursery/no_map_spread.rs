use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsMemberExpression,
    AnyJsObjectMember, JsArrayExpression, JsArrowFunctionExpression, JsCallExpression,
    JsIdentifierExpression, JsObjectExpression, JsParenthesizedExpression, JsReturnStatement, T,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind, WalkEvent,
    declare_node_union,
};
use biome_rule_options::no_map_spread::NoMapSpreadOptions;

use crate::JsRuleAction;
use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow spreading objects or arrays in the values returned by `map()` and `flatMap()` callbacks.
    ///
    /// A spread (`...`) inside an object or array literal creates a new object or
    /// array and copies every property or element into it. When the callback of
    /// `map()` or `flatMap()` does this, the copy happens once for each item in the
    /// array, which is slow and uses extra memory for large arrays.
    ///
    /// When it's fine to change the original items, use
    /// [`Object.assign()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign)
    /// or assign the new properties directly. For arrays, add the new elements with
    /// [`push()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push).
    ///
    /// Copying is still useful when other code needs the original items to stay
    /// unchanged. To avoid reporting those cases, the rule ignores `map()` and
    /// `flatMap()` calls on:
    ///
    /// - arrays read from `this`, such as `this.items.map(...)`;
    /// - variables that are read again after the call (see the `ignoreRereads` option);
    /// - function parameters (see the `ignoreArgs` option).
    ///
    /// The rule follows variables that the callback declares and returns, so it also
    /// reports `const copy = { ...item }; return copy;`.
    ///
    /// The fix replaces an object that starts with a spread and has at least one
    /// other property with a call to `Object.assign()`. This changes the first
    /// spread object in place, so the fix is unsafe. The fix isn't offered for an
    /// object that only contains a spread, such as `{ ...item }`, or for an object
    /// that contains comments.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const users = getUsers();
    /// const rows = users.map((user) => ({ ...user, role: getRole(user) }));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const groups = getGroups();
    /// const lists = groups.flatMap((group) => [...group.members, group.owner]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const users = getUsers();
    /// const rows = users.map((user) => Object.assign(user, { role: getRole(user) }));
    ///
    /// class UsersStore {
    ///   #users = [];
    ///   get users() {
    ///     // Each caller gets its own copies, so changing them can't change the store.
    ///     return this.#users.map((user) => ({ ...user }));
    ///   }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreRereads`
    ///
    /// Default: `true`
    ///
    /// Ignores `map()` and `flatMap()` calls on a variable that is read again after
    /// the call. Code that reads the original array later may rely on the copies to
    /// keep the original items unchanged.
    ///
    /// ```js
    /// const users = getUsers();
    /// const rows = users.map((user) => ({ ...user, role: getRole(user) }));
    /// console.log(users);
    /// ```
    ///
    /// Set it to `false` to report these calls too.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreRereads": false
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const users = getUsers();
    /// const rows = users.map((user) => ({ ...user, role: getRole(user) }));
    /// console.log(users);
    /// ```
    ///
    /// ### `ignoreArgs`
    ///
    /// Default: `true`
    ///
    /// Ignores `map()` and `flatMap()` calls on a function parameter. The caller
    /// owns the array, so changing its items in place could affect code outside the
    /// function.
    ///
    /// ```js
    /// function withRoles(users) {
    ///   return users.map((user) => ({ ...user, role: getRole(user) }));
    /// }
    /// ```
    ///
    /// Set it to `false` to report these calls too.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreArgs": false
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// function withRoles(users) {
    ///   return users.map((user) => ({ ...user, role: getRole(user) }));
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow spreading the accumulator of `reduce()`, see [`noAccumulatingSpread`](https://biomejs.dev/linter/rules/no-accumulating-spread/).
    ///
    pub NoMapSpread {
        version: "next",
        name: "noMapSpread",
        language: "js",
        sources: &[RuleSource::Oxc("no-map-spread").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    /// An object or array literal that contains a spread.
    pub AnyJsSpreadLiteral = JsObjectExpression | JsArrayExpression
}

/// A literal with a spread that the `map()` callback returns.
pub struct ReturnedSpread {
    literal: AnyJsSpreadLiteral,
    /// The range of the returned variable that holds `literal`, or `None` when
    /// the callback returns `literal` directly.
    returned_variable: Option<TextRange>,
}

impl Rule for NoMapSpread {
    type Query = Semantic<JsCallExpression>;
    type State = ReturnedSpread;
    type Signals = Box<[Self::State]>;
    type Options = NoMapSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        find_returned_spreads(ctx)
            .unwrap_or_default()
            .into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let callee = AnyJsMemberExpression::cast(ctx.query().callee().ok()?.into_syntax())?;
        let method = callee.member_name()?;
        let method_range = match &callee {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.range()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.member().ok()?.range()
            }
        };
        let method = method.text();

        let diagnostic = match &state.literal {
            AnyJsSpreadLiteral::JsObjectExpression(_) => RuleDiagnostic::new(
                rule_category!(),
                state.literal.range(),
                markup! {
                    "This object spread copies an object for each item in the "<Emphasis>{method}"()"</Emphasis>" callback."
                },
            )
            .detail(method_range, markup! { "The callback runs once for each item in the array." })
            .note(markup! {
                "A spread creates a new object and copies every property into it. Doing this for each item is slow and uses extra memory for large arrays."
            })
            .note(markup! {
                "If it's fine to change the original items, add the properties with "<Emphasis>"Object.assign()"</Emphasis>" or assign them directly. Keep the spread if other code needs the original items to stay unchanged."
            }),
            AnyJsSpreadLiteral::JsArrayExpression(_) => RuleDiagnostic::new(
                rule_category!(),
                state.literal.range(),
                markup! {
                    "This array spread copies an array for each item in the "<Emphasis>{method}"()"</Emphasis>" callback."
                },
            )
            .detail(method_range, markup! { "The callback runs once for each item in the array." })
            .note(markup! {
                "A spread creates a new array and copies every element into it. Doing this for each item is slow and uses extra memory for large arrays."
            })
            .note(markup! {
                "If it's fine to change the original arrays, add the elements with "<Emphasis>"push()"</Emphasis>". Keep the spread if other code needs the original arrays to stay unchanged."
            }),
        };

        Some(match state.returned_variable {
            Some(range) => {
                diagnostic.detail(range, markup! { "The callback returns the copy here." })
            }
            None => diagnostic,
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let AnyJsSpreadLiteral::JsObjectExpression(object) = &state.literal else {
            return None;
        };
        // The parentheses around an object returned by an arrow function are
        // only there to tell it apart from a block body, so they can go too.
        let replaced: AnyJsExpression = match object.parent::<JsParenthesizedExpression>() {
            Some(parenthesized)
                if parenthesized
                    .parent::<JsArrowFunctionExpression>()
                    .is_some() =>
            {
                parenthesized.into()
            }
            _ => object.clone().into(),
        };
        let members = object.members();
        if members.len() < 2
            || !matches!(members.first()?.ok()?, AnyJsObjectMember::JsSpread(_))
            || replaced.syntax().has_inner_comments()
            || ctx
                .model()
                .scope(object.syntax())
                .ancestors()
                .any(|scope| scope.get_binding("Object").is_some())
        {
            return None;
        }

        let mut arguments = Vec::new();
        let mut properties = Vec::new();
        for member in members.iter() {
            match member.ok()? {
                AnyJsObjectMember::JsSpread(spread) => {
                    if !properties.is_empty() {
                        arguments.push(object_argument(std::mem::take(&mut properties)));
                    }
                    arguments.push(AnyJsCallArgument::AnyJsExpression(
                        spread.argument().ok()?.trim_trivia()?,
                    ));
                }
                member => properties.push(member.trim_trivia()?),
            }
        }
        if !properties.is_empty() {
            arguments.push(object_argument(properties));
        }

        let separators = vec![make::token(T![,]).with_trailing_space(); arguments.len() - 1];
        let call = make::js_call_expression(
            make::js_static_member_expression(
                make::js_identifier_expression(make::js_reference_identifier(make::ident(
                    "Object",
                )))
                .into(),
                make::token(T![.]),
                make::js_name(make::ident("assign")).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list(arguments, separators),
                make::token(T![')']),
            ),
        )
        .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(replaced, call.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"Object.assign()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the spread literals returned by the callback of a `map()` or
/// `flatMap()` call. Returns `None` when the call isn't one, the callback
/// returns no spread, or [`is_ignored_array`] ignores the array.
fn find_returned_spreads(ctx: &RuleContext<NoMapSpread>) -> Option<Vec<ReturnedSpread>> {
    let call = ctx.query();
    let callee = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
    if !matches!(callee.member_name()?.text(), "map" | "flatMap") {
        return None;
    }

    let mut arguments = call.arguments().ok()?.args().iter();
    let argument = arguments.next()?.ok()?;
    if arguments.next().is_some() {
        return None;
    }
    let callback = match argument.as_any_js_expression()?.inner_expression()? {
        AnyJsExpression::JsArrowFunctionExpression(function) => AnyJsFunction::from(function),
        AnyJsExpression::JsFunctionExpression(function) => AnyJsFunction::from(function),
        _ => return None,
    };

    let mut finder = SpreadFinder {
        model: ctx.model(),
        callback_range: callback.range(),
        followed_variables: Vec::new(),
        spreads: Vec::new(),
    };
    match callback.body().ok()? {
        AnyJsFunctionBody::AnyJsExpression(expression) => finder.visit(&expression, None),
        AnyJsFunctionBody::JsFunctionBody(body) => {
            let mut preorder = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
            while let Some(event) = preorder.next() {
                let WalkEvent::Enter(node) = event else {
                    continue;
                };
                if let Some(argument) = JsReturnStatement::cast(node)
                    .as_ref()
                    .and_then(JsReturnStatement::argument)
                {
                    finder.visit(&argument, None);
                }
                preorder.skip_subtree();
            }
        }
    }
    if finder.spreads.is_empty() || is_ignored_array(ctx, &callee.object().ok()?) {
        return None;
    }
    Some(finder.spreads)
}

/// Returns `true` if the array that `map()` is called on likely needs its items
/// copied: it comes from `this`, or it's a variable that the options ignore.
fn is_ignored_array(ctx: &RuleContext<NoMapSpread>, array: &AnyJsExpression) -> bool {
    let mut expression = array.clone();
    let reference = loop {
        let Some(inner) = expression.omit_type_wrappers() else {
            return false;
        };
        expression = match inner {
            AnyJsExpression::JsStaticMemberExpression(member) => match member.object() {
                Ok(object) => object,
                Err(_) => return false,
            },
            AnyJsExpression::JsComputedMemberExpression(member) => match member.object() {
                Ok(object) => object,
                Err(_) => return false,
            },
            AnyJsExpression::JsThisExpression(_) => return true,
            AnyJsExpression::JsIdentifierExpression(identifier) => match identifier.name() {
                Ok(reference) => break reference,
                Err(_) => return false,
            },
            _ => return false,
        };
    };

    let Some(binding) = ctx.model().binding(&reference) else {
        return false;
    };
    let options = ctx.options();
    if options.ignore_args()
        && binding.tree().declaration().is_some_and(|declaration| {
            declaration
                .parent_binding_pattern_declaration()
                .unwrap_or(declaration)
                .is_parameter_like()
        })
    {
        return true;
    }

    let call_end = ctx.query().range().end();
    options.ignore_rereads()
        && binding
            .all_reads()
            .any(|read| read.range_start() >= call_end)
}

/// Collects the spread literals that an expression can evaluate to.
struct SpreadFinder<'a> {
    model: &'a SemanticModel,
    callback_range: TextRange,
    /// The variables currently being followed, to stop on self-references such
    /// as `const { a = b, b = a } = item`.
    followed_variables: Vec<TextRange>,
    spreads: Vec<ReturnedSpread>,
}

impl SpreadFinder<'_> {
    /// Records each spread literal that `expression` can evaluate to.
    /// `returned_variable` is the range of the returned variable that led to
    /// `expression`, if any.
    fn visit(&mut self, expression: &AnyJsExpression, returned_variable: Option<TextRange>) {
        let Some(expression) = expression.inner_expression() else {
            return;
        };
        match expression {
            AnyJsExpression::JsObjectExpression(object) => {
                let has_spread = object
                    .members()
                    .iter()
                    .any(|member| matches!(member, Ok(AnyJsObjectMember::JsSpread(_))));
                if has_spread {
                    self.record(object.into(), returned_variable);
                }
            }
            AnyJsExpression::JsArrayExpression(array) => {
                let has_spread = array
                    .elements()
                    .iter()
                    .any(|element| element.is_ok_and(|element| element.as_js_spread().is_some()));
                if has_spread {
                    self.record(array.into(), returned_variable);
                }
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                if let Ok(consequent) = conditional.consequent() {
                    self.visit(&consequent, returned_variable);
                }
                if let Ok(alternate) = conditional.alternate() {
                    self.visit(&alternate, returned_variable);
                }
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                if let Ok(left) = logical.left() {
                    self.visit(&left, returned_variable);
                }
                if let Ok(right) = logical.right() {
                    self.visit(&right, returned_variable);
                }
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                self.visit_variable(&identifier, returned_variable);
            }
            _ => {}
        }
    }

    /// Follows a variable declared inside the callback to the value or default
    /// value it's declared with.
    fn visit_variable(
        &mut self,
        identifier: &JsIdentifierExpression,
        returned_variable: Option<TextRange>,
    ) {
        let Some(binding) = identifier
            .name()
            .ok()
            .and_then(|reference| self.model.binding(&reference))
        else {
            return;
        };
        let binding = binding.tree();
        let binding_range = binding.range();
        if !self.callback_range.contains_range(binding_range)
            || self.followed_variables.contains(&binding_range)
        {
            return;
        }

        // Callback parameters hold the array items themselves. A default value
        // only applies when an item is missing, so it isn't treated as a copy.
        let declaration = binding.declaration();
        if declaration.as_ref().is_some_and(|declaration| {
            declaration
                .parent_binding_pattern_declaration()
                .unwrap_or_else(|| declaration.clone())
                .is_parameter_like()
        }) {
            return;
        }
        let initializer = match declaration {
            Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
                declarator.initializer()
            }
            Some(AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property)) => {
                property.init()
            }
            Some(AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property)) => {
                property.init()
            }
            Some(AnyJsBindingDeclaration::JsArrayBindingPatternElement(element)) => element.init(),
            _ => None,
        };
        let Some(value) = initializer.and_then(|initializer| initializer.expression().ok()) else {
            return;
        };

        self.followed_variables.push(binding_range);
        self.visit(
            &value,
            returned_variable.or_else(|| Some(identifier.range())),
        );
        self.followed_variables.pop();
    }

    fn record(&mut self, literal: AnyJsSpreadLiteral, returned_variable: Option<TextRange>) {
        if !self.spreads.iter().any(|spread| spread.literal == literal) {
            self.spreads.push(ReturnedSpread {
                literal,
                returned_variable,
            });
        }
    }
}

/// Wraps object members in a new object literal, for use as an argument of
/// `Object.assign()`.
fn object_argument(properties: Vec<AnyJsObjectMember>) -> AnyJsCallArgument {
    let separators = vec![make::token(T![,]).with_trailing_space(); properties.len() - 1];
    AnyJsCallArgument::AnyJsExpression(
        make::js_object_expression(
            make::token(T!['{']).with_trailing_space(),
            make::js_object_member_list(properties, separators),
            make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )
        .into(),
    )
}

/// The nodes the return search stops at: `return` statements, and the
/// functions whose `return` statements don't belong to the callback.
const RETURN_SEARCH_KINDS: biome_rowan::SyntaxKindSet<biome_js_syntax::JsLanguage> =
    AnyJsControlFlowRoot::KIND_SET.union(JsReturnStatement::KIND_SET);
