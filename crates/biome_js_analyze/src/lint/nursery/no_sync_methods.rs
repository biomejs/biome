use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsClass, AnyJsExpression, AnyJsMemberExpression, JsArrowFunctionExpression,
    JsCallExpression, JsConstructorParameters, JsDecorator, JsFormalParameter, JsFunctionBody,
    JsInitializerClause, JsLanguage, JsParameters, JsPropertyClassMember,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, AstNodeList, SyntaxKindSet};
use biome_rule_options::no_sync_methods::NoSyncMethodsOptions;

declare_lint_rule! {
    /// Disallow calling synchronous methods, such as `fs.readFileSync()`.
    ///
    /// Many Node.js functions come in two versions. For example, `fs.readFile()` reads a file
    /// in the background and lets the program keep running, while `fs.readFileSync()` stops
    /// everything else until the file is read. By convention, the names of the blocking versions
    /// end with `Sync`.
    ///
    /// Blocking calls are fine in a command-line script that does one thing at a time. In a web
    /// server, a single blocking call holds up every other request until it finishes.
    ///
    /// This rule reports every call to a function or method whose name ends with `Sync`,
    /// including calls made through `.call()` and `.apply()`. It only looks at names, so it also
    /// reports functions that don't come from Node.js.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// fs.existsSync(somePath);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     var contents = fs.readFileSync(somePath).toString();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// obj.sync();
    ///
    /// const contents = await fs.promises.readFile(somePath);
    ///
    /// const readFile = fs.readFileSync;
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowAtRootLevel`
    ///
    /// Default: `false`
    ///
    /// When `true`, the rule allows synchronous calls at the top level of a file, outside of
    /// any function. Code at the top level runs once, when the file is first loaded, so blocking
    /// there doesn't slow down later work. Calls inside functions and methods are still
    /// reported, and so are calls in the values of class properties that aren't `static`,
    /// because those values are computed each time the class creates an object.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowAtRootLevel": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// const read = (file) => fs.readFileSync(file);
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// const config = fs.readFileSync("config.json").toString();
    /// ```
    ///
    /// ### `ignore`
    ///
    /// Default: `[]`
    ///
    /// A list of function names that the rule doesn't report.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignore": ["readFileSync"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// fs.readdirSync(somePath);
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// fs.readFileSync(somePath);
    /// ```
    ///
    pub NoSyncMethods {
        version: "next",
        name: "noSyncMethods",
        language: "js",
        sources: &[RuleSource::EslintN("no-sync").same()],
        recommended: false,
    }
}

impl Rule for NoSyncMethods {
    type Query = Ast<JsCallExpression>;
    /// The name of the synchronous function being called.
    type State = StaticValue;
    type Signals = Option<Self::State>;
    type Options = NoSyncMethodsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let name = sync_function_name(call.callee().ok()?)?;
        let options = ctx.options();
        if options
            .ignore()
            .iter()
            .any(|ignored| ignored.as_ref() == name.text())
        {
            return None;
        }
        if options.allow_at_root_level() && !is_inside_function(call) {
            return None;
        }
        Some(name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected call to the synchronous function "<Emphasis>{name.text()}</Emphasis>"."
                },
            )
            .note(markup! {
                "Synchronous functions block the program until they finish, so nothing else can run in the meantime."
            })
            .note(markup! {
                "Use the asynchronous version of this function instead."
            }),
        )
    }
}

/// Returns the name of the called function when it ends with `Sync`.
///
/// Calls made through `.call()` and `.apply()` return the name of the function they call:
/// `fooSync.call(null)` returns `fooSync`.
fn sync_function_name(callee: AnyJsExpression) -> Option<StaticValue> {
    let mut callee = callee.omit_type_wrappers()?;
    if let Some(member) = AnyJsMemberExpression::cast_ref(callee.syntax())
        && member
            .member_name()
            .is_some_and(|name| matches!(name.text(), "call" | "apply"))
    {
        callee = member.object().ok()?.omit_type_wrappers()?;
    }

    let name = match callee {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            StaticValue::String(identifier.name().ok()?.value_token().ok()?)
        }
        _ => AnyJsMemberExpression::cast(callee.into_syntax())?.member_name()?,
    };
    name.text().ends_with("Sync").then_some(name)
}

/// Returns `true` when the call runs as part of a function rather than when the file loads.
///
/// Only the body and the parameters of a function count. Decorators and the computed name of
/// a method run once, when the class or object is defined, so a call inside a decorator is
/// checked from the position of its class instead.
///
/// Values of class properties that aren't `static` count as part of a function, because they
/// run each time the class creates an object.
fn is_inside_function(call: &JsCallExpression) -> bool {
    let mut in_decorator = false;
    for ancestor in call.syntax().ancestors().skip(1) {
        let kind = ancestor.kind();
        if in_decorator {
            in_decorator = !AnyJsClass::can_cast(kind);
        } else if JsDecorator::can_cast(kind) {
            in_decorator = true;
        } else if FUNCTION_PART_KINDS.matches(kind)
            || JsInitializerClause::cast(ancestor)
                .and_then(|initializer| initializer.parent::<JsPropertyClassMember>())
                .is_some_and(|property| {
                    !property
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some())
                })
        {
            return true;
        }
    }
    false
}

/// Nodes that run only when their function is called. Arrow functions are included as a
/// whole because their body can be an expression rather than a block. Formal parameters are
/// included because a setter's parameter isn't wrapped in a parameter list.
const FUNCTION_PART_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsParameters::KIND_SET)
    .union(JsConstructorParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET);
