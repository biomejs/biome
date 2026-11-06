use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsParameter, JsAssignmentExpression,
    JsAssignmentOperator, JsCallArgumentList, JsCallArguments, JsCallExpression, JsFormalParameter,
    JsInitializerClause, JsParameters, JsPropertyClassMember, JsSyntaxKind, JsVariableDeclarator,
    TsAsExpression, TsDeclareFunctionDeclaration, TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};
use biome_rule_options::use_vue_typed_ref::UseVueTypedRefOptions;

use crate::frameworks::vue::vue_call::vue_api_name;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require a type for refs created with `ref()` or `shallowRef()` in TypeScript.
    ///
    /// When `ref()` or `shallowRef()` is called without a type argument and without an initial
    /// value, TypeScript gives the ref the `any` type. You can then put any value in the ref, and
    /// TypeScript won't report a mistake, even when
    /// [`noImplicitAny`](https://www.typescriptlang.org/tsconfig#noImplicitAny) is enabled.
    /// The same applies when the initial value is `null` or `undefined`: TypeScript can't tell
    /// which other values the ref is meant to hold.
    ///
    /// This rule reports these calls, unless the type of the ref is already written next to the
    /// call:
    ///
    /// - in a variable declaration, such as `const count: Ref<number | undefined> = ref()`;
    /// - in a class property declaration;
    /// - in an assignment to a variable or parameter that has a type;
    /// - with `as`, `satisfies` or `<Type>`;
    /// - as the default value of a parameter that has a type;
    /// - as an argument to a function declared in the same file, when the matching parameter has
    ///   a type.
    ///
    /// The rule doesn't know the types of functions imported from other files, so a call passed
    /// to one of them is still reported.
    ///
    /// Only TypeScript files and Vue files with `<script lang="ts">` are checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// import { ref } from "vue";
    ///
    /// const count = ref();
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// import { shallowRef } from "vue";
    ///
    /// const user = shallowRef(null);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// import { ref, type Ref } from "vue";
    ///
    /// const a = ref<number>();
    /// const b = ref(0);
    /// const c: Ref<number | undefined> = ref();
    /// const d = ref() as Ref<number | undefined>;
    ///
    /// function useCount(count: Ref<number | undefined>) {}
    /// useCount(ref());
    /// ```
    ///
    pub UseVueTypedRef {
        version: "2.6.3",
        name: "useVueTypedRef",
        language: "ts",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-typed-ref").same()],
    }
}

impl Rule for UseVueTypedRef {
    type Query = Semantic<JsCallExpression>;
    type State = RefApi;
    type Signals = Option<Self::State>;
    type Options = UseVueTypedRefOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<JsFileSource>().language().is_typescript() {
            return None;
        }
        let call = ctx.query();
        if call.type_arguments().is_some() || has_initial_value(call) {
            return None;
        }
        let model = ctx.model();
        let api = match vue_api_name(&call.callee().ok()?, model)?.text() {
            "ref" => RefApi::Ref,
            "shallowRef" => RefApi::ShallowRef,
            _ => return None,
        };
        if is_in_typed_position(call, model) {
            return None;
        }
        Some(api)
    }

    fn diagnostic(ctx: &RuleContext<Self>, api: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>{api}"()"</Emphasis>" call creates a ref without a type."
                },
            )
            .note(markup! {
                "Without a type argument, TypeScript can't tell which values the ref is meant to hold, so it can't catch mistakes when the ref is used."
            })
            .note(markup! {
                "Add a type argument, for example "<Emphasis>{api}"<number>()"</Emphasis>"."
            }),
        )
    }
}

/// The Vue function that creates the ref.
#[derive(Clone, Copy)]
pub enum RefApi {
    Ref,
    ShallowRef,
}

impl biome_console::fmt::Display for RefApi {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Ref => "ref",
            Self::ShallowRef => "shallowRef",
        })
    }
}

/// Returns `true` if the first argument of `call` gives the ref a type, which is any argument
/// other than `null` or `undefined`.
fn has_initial_value(call: &JsCallExpression) -> bool {
    let Some(Ok(first)) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().first())
    else {
        return false;
    };
    match first {
        AnyJsCallArgument::AnyJsExpression(expression) => !expression
            .omit_parentheses()
            .as_static_value()
            .is_some_and(|value| value.is_null_or_undefined()),
        AnyJsCallArgument::JsSpread(_) => true,
    }
}

/// Returns `true` if a type annotation written next to `call` already gives the created ref its
/// type.
fn is_in_typed_position(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let mut expression = call.syntax().clone();
    let Some(parent) = call.syntax().ancestors().skip(1).find(|node| {
        if node.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            expression = node.clone();
            false
        } else {
            true
        }
    }) else {
        return false;
    };

    // `ref() as Ref<T>`, `ref() satisfies Ref<T>`, `<Ref<T>>ref()`
    if TsAsExpression::can_cast(parent.kind())
        || TsSatisfiesExpression::can_cast(parent.kind())
        || TsTypeAssertionExpression::can_cast(parent.kind())
    {
        return true;
    }

    // `const value: Ref<T> = ref()`, `function useValue(value: Ref<T> = ref()) {}`,
    // `class Store { value: Ref<T> = ref() }`
    if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
        return initializer
            .syntax()
            .parent()
            .and_then(AnyTypedDeclaration::cast)
            .is_some_and(|declaration| declaration.has_type_annotation());
    }

    // `value = ref()`, where `value` is a variable or a parameter that has a type
    if let Some(assignment) = JsAssignmentExpression::cast(parent.clone()) {
        return assignment
            .operator()
            .is_ok_and(|operator| operator == JsAssignmentOperator::Assign)
            && assignment
                .left()
                .ok()
                .and_then(|left| {
                    left.as_any_js_assignment()?
                        .as_js_identifier_assignment()
                        .cloned()
                })
                .and_then(|identifier| model.binding(&identifier))
                .and_then(|binding| binding.syntax().parent())
                .and_then(AnyTypedDeclaration::cast)
                .is_some_and(|declaration| declaration.has_type_annotation());
    }

    // `useValue(ref())`, where the matching parameter of `useValue` has a type
    if let Some(arguments) = JsCallArgumentList::cast(parent) {
        let Some(index) = arguments
            .iter()
            .position(|argument| argument.is_ok_and(|argument| *argument.syntax() == expression))
        else {
            return false;
        };
        return arguments
            .parent::<JsCallArguments>()
            .and_then(|arguments| arguments.parent::<JsCallExpression>())
            .and_then(|outer_call| local_function_parameters(outer_call.callee().ok()?, model))
            .is_some_and(|parameters| has_typed_parameter(&parameters, index));
    }

    false
}

declare_node_union! {
    /// A declaration whose type annotation, when present, gives a type to the value assigned
    /// to it.
    pub AnyTypedDeclaration = JsVariableDeclarator | JsFormalParameter | JsPropertyClassMember
}

impl AnyTypedDeclaration {
    fn has_type_annotation(&self) -> bool {
        match self {
            Self::JsVariableDeclarator(declarator) => declarator.variable_annotation().is_some(),
            Self::JsFormalParameter(parameter) => parameter.type_annotation().is_some(),
            Self::JsPropertyClassMember(member) => {
                member.property_annotation().is_some_and(|annotation| {
                    annotation.type_annotation().is_ok_and(|ty| ty.is_some())
                })
            }
        }
    }
}

/// Returns the parameters of the function that `callee` refers to, if it is declared in the
/// same file as a function declaration, a `declare function`, or a variable initialized with a
/// function or an arrow function.
fn local_function_parameters(
    callee: AnyJsExpression,
    model: &SemanticModel,
) -> Option<JsParameters> {
    let reference = callee.omit_parentheses().as_js_reference_identifier()?;
    let declaration = model.binding(&reference)?.syntax().parent()?;
    if let Some(declarator) = JsVariableDeclarator::cast_ref(&declaration) {
        let initializer = declarator.initializer()?.expression().ok()?;
        return AnyJsFunction::cast(initializer.omit_parentheses().into_syntax())?
            .parenthesized_parameters();
    }
    if let Some(declaration) = TsDeclareFunctionDeclaration::cast_ref(&declaration) {
        return declaration.parameters().ok();
    }
    AnyJsFunction::cast(declaration)?.parenthesized_parameters()
}

/// Returns `true` if the parameter in `parameters` that receives the argument at `index` has a
/// type, including a rest parameter that receives the remaining arguments.
fn has_typed_parameter(parameters: &JsParameters, index: usize) -> bool {
    let mut last = None;
    for (position, parameter) in parameters
        .items()
        .iter()
        .flatten()
        .filter(|parameter| !matches!(parameter, AnyJsParameter::TsThisParameter(_)))
        .enumerate()
    {
        if position == index {
            return parameter.type_annotation().is_some();
        }
        last = Some(parameter);
    }
    last.is_some_and(|parameter| {
        matches!(parameter, AnyJsParameter::JsRestParameter(_))
            && parameter.type_annotation().is_some()
    })
}
