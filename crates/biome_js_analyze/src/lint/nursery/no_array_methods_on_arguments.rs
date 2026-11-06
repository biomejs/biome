use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsMemberExpression, AnyJsVariableDeclaration, JsCallExpression, JsComputedMemberName,
    JsConstructorClassMember, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsGetterClassMember,
    JsGetterObjectMember, JsLanguage, JsMethodClassMember, JsMethodObjectMember,
    JsReferenceIdentifier, JsSetterClassMember, JsSetterObjectMember,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, SyntaxKindSet};
use biome_rule_options::no_array_methods_on_arguments::NoArrayMethodsOnArgumentsOptions;

declare_lint_rule! {
    /// Disallow calling array methods on the `arguments` object.
    ///
    /// Inside a regular function, `arguments` holds the values the function was called with.
    /// It has a `length` and its values can be read by index, like an array, but it isn't an array.
    /// It doesn't have array methods such as `map()`, `filter()`, or `reduce()`,
    /// so calling one of them throws a `TypeError` when the code runs.
    ///
    /// Declare a rest parameter, such as `...args`, to receive the values as a real array.
    /// If you can't change the function's parameters, convert `arguments` to an array first,
    /// for example with `Array.from(arguments)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function sum() {
    ///     return arguments.reduce((total, value) => total + value, 0);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function first() {
    ///     return arguments["at"](0);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function sum(...values) {
    ///     return values.reduce((total, value) => total + value, 0);
    /// }
    ///
    /// function first() {
    ///     return Array.from(arguments).at(0);
    /// }
    ///
    /// function list() {
    ///     return Array.prototype.slice.call(arguments);
    /// }
    /// ```
    ///
    /// Outside strict mode, you can name a parameter or variable `arguments`.
    /// When it has a value, it replaces the `arguments` object, so the rule ignores it:
    ///
    /// ```cjs
    /// function count(arguments) {
    ///     return arguments.filter(Boolean).length;
    /// }
    /// ```
    ///
    /// A `var arguments;` declaration without a value doesn't replace the `arguments` object,
    /// so the rule still reports it:
    ///
    /// ```cjs,expect_diagnostic
    /// function count() {
    ///     var arguments;
    ///     return arguments.filter(Boolean).length;
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - To disallow the use of `arguments` entirely, see [`noArguments`](https://biomejs.dev/linter/rules/no-arguments/).
    ///
    pub NoArrayMethodsOnArguments {
        version: "next",
        name: "noArrayMethodsOnArguments",
        language: "js",
        sources: &[RuleSource::Oxc("bad-array-method-on-arguments").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoArrayMethodsOnArguments {
    type Query = Semantic<JsCallExpression>;
    /// The name of the array method that is called, as listed in [ARRAY_METHODS].
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = NoArrayMethodsOnArgumentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        // `arguments.map?.()` doesn't throw, because the call is skipped when `map` is missing.
        if call.optional_chain_token().is_some() {
            return None;
        }
        let callee =
            AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        let member_name = callee.member_name()?;
        let index = ARRAY_METHODS.binary_search(&member_name.text()).ok()?;

        let reference = callee
            .object()
            .ok()?
            .inner_expression()?
            .as_js_reference_identifier()?;
        if !reference.has_name("arguments")
            || !refers_to_implicit_arguments(ctx.model(), &reference)
        {
            return None;
        }

        Some(ARRAY_METHODS[index])
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let callee = ctx.query().callee().ok()?.omit_parentheses();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                callee.range(),
                markup! {
                    "The "<Emphasis>"arguments"</Emphasis>" object doesn't have a "<Emphasis>{method}"()"</Emphasis>" method."
                },
            )
            .note(markup! {
                <Emphasis>"arguments"</Emphasis>" looks like an array, but it isn't one, so this call throws a "<Emphasis>"TypeError"</Emphasis>" when the code runs."
            })
            .note(markup! {
                "Declare a rest parameter, such as "<Emphasis>"...args"</Emphasis>", and use it instead, or convert "<Emphasis>"arguments"</Emphasis>" to an array with "<Emphasis>"Array.from(arguments)"</Emphasis>"."
            }),
        )
    }
}

/// Returns `true` if `reference`, an identifier named `arguments`, refers to the `arguments`
/// object that a regular function creates for each call.
///
/// A function's own `arguments` object hides any variable named `arguments` declared outside
/// the function. Inside the function, a parameter or a declaration with a value replaces the
/// object, but `var arguments;` without a value keeps it.
fn refers_to_implicit_arguments(model: &SemanticModel, reference: &JsReferenceIdentifier) -> bool {
    let Some(binding) = model.binding(reference) else {
        return true;
    };
    // Arrow functions don't have their own `arguments` object, so they are skipped. A computed
    // name of a method, like `[arguments.at(0)]() {}`, runs outside the method, so the method is
    // skipped when the search comes from its name.
    let mut child = reference.syntax().clone();
    let Some(function) = reference.syntax().ancestors().skip(1).find(|node| {
        let is_owner = ARGUMENTS_OWNER_KINDS.matches(node.kind())
            && !JsComputedMemberName::can_cast(child.kind());
        child = node.clone();
        is_owner
    }) else {
        return false;
    };

    // Scopes are looked up by range, and when the function spans the whole file, the scope of
    // the file has the same range. Looking up the scope of `function` or of a parameter could
    // return the scope of the file, so the scope of the function is found from the reference.
    let function_range = function.text_trimmed_range();
    let Some(function_scope) = model
        .scope(reference.syntax())
        .ancestors()
        .find(|scope| scope.range() == function_range)
    else {
        return false;
    };
    // Parameters belong to the scope of the function, while declarations in its body,
    // including `var` declarations in nested blocks, belong to the scope of the body.
    let body_scope = function_scope
        .children()
        .find(|scope| JsFunctionBody::can_cast(scope.syntax().kind()));

    // The name of a function expression belongs to the scope of the function, but the
    // `arguments` object hides it, so it isn't one of the function's declarations.
    let mut is_declared_by_function = false;
    let mut keeps_arguments_object = true;
    for declaration in std::iter::once(function_scope)
        .chain(body_scope)
        .flat_map(|scope| scope.bindings())
        .filter(|declaration| {
            declaration
                .tree()
                .name_token()
                .is_ok_and(|name| name.text_trimmed() == "arguments")
                && declaration.syntax().parent().as_ref() != Some(&function)
        })
    {
        is_declared_by_function |= declaration == binding;
        keeps_arguments_object &= is_var_without_value(&declaration);
    }
    if is_declared_by_function {
        return keeps_arguments_object;
    }

    // The variable is declared either outside the function, where the `arguments` object hides
    // it, or in a nested block or function, where it hides the `arguments` object. The name of
    // the function is in the range of the function, but the `arguments` object hides it.
    let declaration = binding.syntax();
    !function_range.contains_range(declaration.text_trimmed_range())
        || declaration.parent().as_ref() == Some(&function)
}

/// Returns `true` if `binding` is declared by `var` without a value, like `var arguments;`.
/// Declarations in the head of a `for...in` or `for...of` loop receive a value on each iteration,
/// so they don't count.
fn is_var_without_value(binding: &Binding) -> bool {
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
        binding.tree().declaration()
    else {
        return false;
    };
    declarator.initializer().is_none()
        && matches!(
            declarator.declaration(),
            Some(AnyJsVariableDeclaration::JsVariableDeclaration(declaration)) if declaration.is_var()
        )
}

/// Functions that have their own `arguments` object.
const ARGUMENTS_OWNER_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionDeclaration::KIND_SET
    .union(JsFunctionExpression::KIND_SET)
    .union(JsFunctionExportDefaultDeclaration::KIND_SET)
    .union(JsMethodClassMember::KIND_SET)
    .union(JsConstructorClassMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// Methods of `Array.prototype` that the `arguments` object doesn't have, sorted for binary search.
///
/// `toString()` and `toLocaleString()` are left out, because every object has them.
const ARRAY_METHODS: &[&str] = &[
    "at",
    "concat",
    "copyWithin",
    "entries",
    "every",
    "fill",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "flat",
    "flatMap",
    "forEach",
    "includes",
    "indexOf",
    "join",
    "keys",
    "lastIndexOf",
    "map",
    "pop",
    "push",
    "reduce",
    "reduceRight",
    "reverse",
    "shift",
    "slice",
    "some",
    "sort",
    "splice",
    "toReversed",
    "toSorted",
    "toSpliced",
    "unshift",
    "values",
    "with",
];

#[cfg(test)]
mod tests {
    use super::ARRAY_METHODS;

    #[test]
    fn array_methods_are_sorted() {
        for pair in ARRAY_METHODS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "ARRAY_METHODS is not sorted: {:?} must come before {:?}",
                pair[1],
                pair[0]
            );
        }
    }
}
