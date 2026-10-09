use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern,
    AnyJsClass, AnyJsClassMemberName, AnyJsExpression, AnyJsMemberExpression, JsArrayExpression,
    JsAssignmentExpression, JsCallExpression, JsComputedMemberAssignment, JsExport,
    JsGetterClassMember, JsGetterObjectMember, JsIdentifierExpression, JsInitializerClause,
    JsLanguage, JsNewExpression, JsObjectExpression, JsPropertyClassMember, JsReferenceIdentifier,
    JsSetterClassMember, JsSetterObjectMember, JsStaticInitializationBlockClassMember,
    JsStaticMemberAssignment, JsSyntaxKind, JsVariableDeclarator, static_value::StaticValue,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, TextRange};
use biome_rule_options::use_svelte_reactive_classes::UseSvelteReactiveClassesOptions;

use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require the reactive classes from `svelte/reactivity` instead of built-in objects that are
    /// modified.
    ///
    /// Svelte updates the page when a `$state` variable is reassigned, or when a plain object or
    /// array stored in it changes. It doesn't detect changes made inside objects created from
    /// built-in classes. For example, calling `map.set()` on a `Map`, calling `date.setMonth()`
    /// on a `Date`, or assigning `url.port` on a `URL` changes the object, but the parts of the
    /// page that use it don't update. Wrapping the object in `$state()` doesn't help, because
    /// `$state` only tracks plain objects and arrays.
    ///
    /// The `svelte/reactivity` module provides versions of these classes that Svelte tracks:
    ///
    /// | Built-in class    | Svelte class            |
    /// | ----------------- | ----------------------- |
    /// | `Date`            | `SvelteDate`            |
    /// | `Map`             | `SvelteMap`             |
    /// | `Set`             | `SvelteSet`             |
    /// | `URL`             | `SvelteURL`             |
    /// | `URLSearchParams` | `SvelteURLSearchParams` |
    ///
    /// This rule reports objects created from the built-in classes that are modified after they're
    /// created, including through other variables that hold the same object. Objects that are only
    /// read are allowed. Objects stored only in variables declared inside a function are allowed
    /// too: each call to the function creates a new object, so the object can't hold data that
    /// the page keeps showing.
    ///
    /// In Svelte modules (`.svelte.js` and `.svelte.ts` files), the rule also reports objects that
    /// are exported, such as `export const cache = new Map()`, because other files that import
    /// them can modify them.
    ///
    /// Changes made in the component markup, such as in an `onclick` attribute, aren't detected.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// const tags = new Set();
    /// function addTag(tag) {
    ///     tags.add(tag);
    /// }
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// const date = $state(new Date());
    /// date.setFullYear(2000);
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// const url = new URL("https://svelte.dev/");
    /// url.pathname = "/docs";
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// import { SvelteSet } from "svelte/reactivity";
    /// const tags = new SvelteSet();
    /// function addTag(tag) {
    ///     tags.add(tag);
    /// }
    ///
    /// const map = new Map([[1, "one"]]);
    /// console.log(map.get(1));
    ///
    /// function unique(items) {
    ///     const seen = new Set();
    ///     for (const item of items) {
    ///         seen.add(item);
    ///     }
    ///     return [...seen];
    /// }
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to remove `$state()` around the classes from `svelte/reactivity`, which
    ///   don't need it, see [`noSvelteUnnecessaryStateWrap`](https://biomejs.dev/linter/rules/no-svelte-unnecessary-state-wrap/).
    ///
    /// ## References
    ///
    /// - [`svelte/reactivity`](https://svelte.dev/docs/svelte/svelte-reactivity)
    pub UseSvelteReactiveClasses {
        version: "next",
        name: "useSvelteReactiveClasses",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("prefer-svelte-reactivity").inspired()],
        recommended: true,
        severity: Severity::Warning,
    }
}

/// Why the object created by the queried `new` expression is reported.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RuleState {
    /// The object is modified at the given range.
    Mutated(TextRange),
    /// The object is exported from a Svelte module.
    Exported,
}

impl Rule for UseSvelteReactiveClasses {
    type Query = Semantic<JsNewExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseSvelteReactiveClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        let embedding_kind = source_type.as_embedding_kind();
        if !embedding_kind.is_svelte() {
            return None;
        }

        let new_expression = ctx.query();
        let model = ctx.model();

        let reference = callee_reference(new_expression)?;
        let name = reference.value_token().ok()?;
        let class = BuiltinClass::from_name(name.text_trimmed())?;

        // A local or imported binding, for example `import { SvelteMap as Map }`, shadows the
        // built-in class. Bindings declared in another script block of the same component are
        // only visible through the embedded service.
        if model.binding(&reference).is_some() {
            return None;
        }
        if ctx
            .get_service::<EmbeddedService>()
            .is_some_and(|embedded| embedded.contains_binding(name.token_text_trimmed()))
        {
            return None;
        }

        let mut visited = Vec::new();
        let expression = AnyJsExpression::JsNewExpression(new_expression.clone());
        if let Some(range) = find_mutation(model, class, expression.clone(), false, &mut visited) {
            return Some(RuleState::Mutated(range));
        }

        if embedding_kind.is_svelte_source_module() && is_exported_value(model, expression) {
            return Some(RuleState::Exported);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let class =
            BuiltinClass::from_name(callee_reference(node)?.value_token().ok()?.text_trimmed())?;
        let diagnostic = match state {
            RuleState::Mutated(range) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>{class}</Emphasis>" is modified, but Svelte doesn't detect changes to it."
                },
            )
            .detail(range, markup! { "It's modified here." })
            .note(markup! {
                "Svelte doesn't detect changes made inside a built-in "<Emphasis>{class}</Emphasis>", so the parts of the page that use it don't update."
            }),
            RuleState::Exported => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>{class}</Emphasis>" is exported, but Svelte doesn't detect changes to it."
                },
            )
            .note(markup! {
                "Other files that import it can modify it, and Svelte doesn't detect changes made inside a built-in "<Emphasis>{class}</Emphasis>"."
            }),
        };
        Some(diagnostic.note(markup! {
            "Use "<Emphasis>"Svelte"{class}</Emphasis>" from "<Emphasis>"svelte/reactivity"</Emphasis>" instead."
        }))
    }
}

/// A built-in class that has a reactive counterpart in `svelte/reactivity`, named after the
/// built-in class with a `Svelte` prefix.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BuiltinClass {
    Date,
    Map,
    Set,
    Url,
    UrlSearchParams,
}

impl BuiltinClass {
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "Date" => Self::Date,
            "Map" => Self::Map,
            "Set" => Self::Set,
            "URL" => Self::Url,
            "URLSearchParams" => Self::UrlSearchParams,
            _ => return None,
        })
    }

    /// Returns whether calling the method `name` on an instance mutates it.
    fn is_mutating_method(self, name: &str) -> bool {
        match self {
            Self::Date => DATE_MUTATING_METHODS.binary_search(&name).is_ok(),
            Self::Map => matches!(name, "clear" | "delete" | "set"),
            Self::Set => matches!(name, "add" | "clear" | "delete"),
            Self::Url => false,
            Self::UrlSearchParams => matches!(name, "append" | "delete" | "set" | "sort"),
        }
    }

    /// Returns whether assigning the property `name` of an instance mutates it.
    fn is_mutable_property(self, name: &str) -> bool {
        match self {
            Self::Url => URL_MUTABLE_PROPERTIES.binary_search(&name).is_ok(),
            Self::Date | Self::Map | Self::Set | Self::UrlSearchParams => false,
        }
    }
}

impl biome_console::fmt::Display for BuiltinClass {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Date => "Date",
            Self::Map => "Map",
            Self::Set => "Set",
            Self::Url => "URL",
            Self::UrlSearchParams => "URLSearchParams",
        })
    }
}

/// Methods that modify a `Date`, sorted for binary search.
const DATE_MUTATING_METHODS: &[&str] = &[
    "setDate",
    "setFullYear",
    "setHours",
    "setMilliseconds",
    "setMinutes",
    "setMonth",
    "setSeconds",
    "setTime",
    "setUTCDate",
    "setUTCFullYear",
    "setUTCHours",
    "setUTCMilliseconds",
    "setUTCMinutes",
    "setUTCMonth",
    "setUTCSeconds",
    "setYear",
];

/// Properties of a `URL` that modify it when assigned, sorted for binary search.
const URL_MUTABLE_PROPERTIES: &[&str] = &[
    "hash", "host", "hostname", "href", "password", "pathname", "port", "protocol", "search",
    "username",
];

/// Nodes with a body that runs on each call, so that the variables declared in it are created
/// again each time.
const FUNCTION_BOUNDARY_KINDS: SyntaxKindSet<JsLanguage> = AnyFunctionLike::KIND_SET
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(JsStaticInitializationBlockClassMember::KIND_SET);

/// Returns the class name referenced by `new_expression`, such as `Map` in `new Map()`.
fn callee_reference(new_expression: &JsNewExpression) -> Option<JsReferenceIdentifier> {
    new_expression
        .callee()
        .ok()?
        .omit_parentheses()
        .as_js_identifier_expression()?
        .name()
        .ok()
}

/// Returns the range of an operation that mutates the value of `expression`, an instance of
/// `class`, through a variable that outlives a function call.
///
/// The value is followed through wrappers that pass it through unchanged, through `$state()` and
/// `$state.raw()`, and through the variables it's assigned to. `persistent` is whether one of
/// these variables is declared outside of all functions; mutations of temporary instances, held
/// only by function-local variables or by no variable at all, aren't reported. `visited` holds
/// the variables already followed, so that aliases that refer to each other are followed only
/// once.
fn find_mutation(
    model: &SemanticModel,
    class: BuiltinClass,
    expression: AnyJsExpression,
    persistent: bool,
    visited: &mut Vec<Binding>,
) -> Option<TextRange> {
    let expression = value_position(expression);
    let parent = expression.syntax().parent()?;
    match parent.kind() {
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let member = AnyJsMemberExpression::unwrap_cast(parent);
            if member.object().ok()?.syntax() != expression.syntax() {
                return None;
            }
            let name = member.member_name()?;
            if !class.is_mutating_method(name.text()) {
                return None;
            }
            let call = member.parent::<JsCallExpression>()?;
            (persistent && call.callee().ok()?.syntax() == member.syntax()).then(|| call.range())
        }
        JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
            let assignment = JsStaticMemberAssignment::unwrap_cast(parent);
            if assignment.object().ok()?.syntax() != expression.syntax() {
                return None;
            }
            let name = assignment.member().ok()?.as_js_name()?.value_token().ok()?;
            (persistent && class.is_mutable_property(name.text_trimmed()))
                .then(|| assignment.range())
        }
        JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
            let assignment = JsComputedMemberAssignment::unwrap_cast(parent);
            if assignment.object().ok()?.syntax() != expression.syntax() {
                return None;
            }
            let name = assignment
                .member()
                .ok()?
                .omit_parentheses()
                .as_static_value()?;
            if !matches!(name, StaticValue::String(_)) {
                return None;
            }
            (persistent && class.is_mutable_property(name.text())).then(|| assignment.range())
        }
        JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
            let declarator =
                JsInitializerClause::unwrap_cast(parent).parent::<JsVariableDeclarator>()?;
            let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) =
                declarator.id().ok()?
            else {
                return None;
            };
            find_binding_mutation(
                model,
                class,
                model.as_binding(&binding),
                persistent,
                visited,
            )
        }
        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
            let assignment = JsAssignmentExpression::unwrap_cast(parent);
            if assignment.right().ok()?.syntax() != expression.syntax() {
                return None;
            }
            if let AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                identifier,
            )) = assignment.left().ok()?
                && let Some(binding) = model.binding(&identifier)
                && let Some(range) =
                    find_binding_mutation(model, class, binding, persistent, visited)
            {
                return Some(range);
            }
            find_mutation(model, class, assignment.into(), persistent, visited)
        }
        JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
            let call = parent.parent()?.parent().and_then(JsCallExpression::cast)?;
            if !is_state_rune(&call) || call.arguments().ok()?.args().len() != 1 {
                return None;
            }
            find_mutation(model, class, call.into(), persistent, visited)
        }
        _ => None,
    }
}

/// Returns the range of an operation that mutates the value of `binding` through one of its
/// read references.
fn find_binding_mutation(
    model: &SemanticModel,
    class: BuiltinClass,
    binding: Binding,
    persistent: bool,
    visited: &mut Vec<Binding>,
) -> Option<TextRange> {
    if visited.contains(&binding) {
        return None;
    }
    let persistent = persistent
        || !binding
            .syntax()
            .ancestors()
            .any(|ancestor| FUNCTION_BOUNDARY_KINDS.matches(ancestor.kind()));
    let references = binding.all_reads();
    visited.push(binding);
    references
        .filter_map(|reference| {
            reference
                .syntax()
                .parent()
                .and_then(JsIdentifierExpression::cast)
        })
        .find_map(|expression| find_mutation(model, class, expression.into(), persistent, visited))
}

/// Returns the outermost expression that has the same value as `expression`.
///
/// Besides parentheses and TypeScript wrappers, this includes the last expression of a comma
/// sequence, the branches of conditional expressions, and the operands of logical expressions,
/// which may evaluate to `expression`.
fn value_position(expression: AnyJsExpression) -> AnyJsExpression {
    let node = expression.syntax().clone();
    let mut value = expression;
    for ancestor in node.ancestors().skip(1) {
        let Some(parent) = AnyJsExpression::cast(ancestor) else {
            break;
        };
        let passes_value = match &parent {
            AnyJsExpression::JsParenthesizedExpression(_)
            | AnyJsExpression::JsLogicalExpression(_)
            | AnyJsExpression::TsAsExpression(_)
            | AnyJsExpression::TsInstantiationExpression(_)
            | AnyJsExpression::TsNonNullAssertionExpression(_)
            | AnyJsExpression::TsSatisfiesExpression(_)
            | AnyJsExpression::TsTypeAssertionExpression(_) => true,
            AnyJsExpression::JsSequenceExpression(sequence) => sequence
                .right()
                .is_ok_and(|right| right.syntax() == value.syntax()),
            AnyJsExpression::JsConditionalExpression(conditional) => conditional
                .test()
                .is_ok_and(|test| test.syntax() != value.syntax()),
            _ => false,
        };
        if !passes_value {
            break;
        }
        value = parent;
    }
    value
}

/// Returns whether `call` is a `$state()` or `$state.raw()` rune call.
fn is_state_rune(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    let object = match callee {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if !member
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
                .is_some_and(|name| name.text_trimmed() == "raw")
            {
                return false;
            }
            let Ok(object) = member.object() else {
                return false;
            };
            object
        }
        callee => callee,
    };
    object
        .as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok()?.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "$state")
}

/// Returns whether the value of `expression` is exported, directly or as part of an exported
/// object, array, or class.
///
/// Only containers that hold the value itself are followed, so the instance in
/// `export const time = new Date().getTime()` or `export const now = () => new Date()` isn't
/// exported.
fn is_exported_value(model: &SemanticModel, expression: AnyJsExpression) -> bool {
    let mut expression = expression;
    loop {
        let outer = value_position(expression);
        let Some(parent) = outer.syntax().parent() else {
            return false;
        };
        expression = match parent.kind() {
            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => return true,
            JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                let Some(owner) = parent.parent() else {
                    return false;
                };
                if let Some(declarator) = JsVariableDeclarator::cast_ref(&owner) {
                    return matches!(
                        declarator.id(),
                        Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                            binding
                        ))) if model.is_exported(&binding)
                    );
                }
                let Some(member) = JsPropertyClassMember::cast(owner) else {
                    return false;
                };
                // Private fields can't be reached from the modules that import the class.
                let is_private = matches!(
                    member.name(),
                    Ok(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                ) || member.modifiers().iter().any(|modifier| {
                    modifier
                        .as_ts_accessibility_modifier()
                        .is_some_and(|modifier| modifier.is_private())
                });
                let Some(class) = member.syntax().grand_parent().and_then(AnyJsClass::cast) else {
                    return false;
                };
                if is_private {
                    return false;
                }
                match class {
                    AnyJsClass::JsClassDeclaration(declaration) => {
                        return declaration.parent::<JsExport>().is_some()
                            || matches!(
                                declaration.id(),
                                Ok(AnyJsBinding::JsIdentifierBinding(binding))
                                    if model.is_exported(&binding)
                            );
                    }
                    AnyJsClass::JsClassExportDefaultDeclaration(_) => return true,
                    AnyJsClass::JsClassExpression(class) => class.into(),
                }
            }
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                let Some(object) = parent.grand_parent().and_then(JsObjectExpression::cast) else {
                    return false;
                };
                object.into()
            }
            JsSyntaxKind::JS_ARRAY_ELEMENT_LIST => {
                let Some(array) = parent.parent().and_then(JsArrayExpression::cast) else {
                    return false;
                };
                array.into()
            }
            JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                let Some(call) = parent.grand_parent().and_then(JsCallExpression::cast) else {
                    return false;
                };
                if !is_state_rune(&call) {
                    return false;
                }
                call.into()
            }
            _ => return false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{DATE_MUTATING_METHODS, URL_MUTABLE_PROPERTIES};

    #[test]
    fn date_mutating_methods_are_sorted() {
        for pair in DATE_MUTATING_METHODS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come before {}",
                pair[1],
                pair[0]
            );
        }
    }

    #[test]
    fn url_mutable_properties_are_sorted() {
        for pair in URL_MUTABLE_PROPERTIES.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come before {}",
                pair[1],
                pair[0]
            );
        }
    }
}
