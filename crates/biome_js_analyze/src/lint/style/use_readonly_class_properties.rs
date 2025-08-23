use crate::JsRuleAction;
use crate::class_member_references::{
    AnyPropertyMember, ClassMemberReference, ClassMemberReferences, class_member_references,
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsPropertyModifier,
    AnyTsPropertyParameterModifier, JsClassDeclaration, JsClassMemberList, JsFileSource,
    JsSyntaxKind, JsSyntaxToken, TextRange, TsAccessibilityModifier, TsPropertyParameter,
    TsReadonlyModifier,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TriviaPiece,
};
use biome_rule_options::use_readonly_class_properties::UseReadonlyClassPropertiesOptions;
use std::iter::once;

declare_lint_rule! {
    /// Enforce marking members as `readonly` if they are never modified outside the constructor.
    ///
    /// This rule ensures that class properties, especially private ones, are marked as `readonly` if their values
    /// remain constant after being initialized. This helps improve code readability, maintainability, and ensures
    /// immutability where applicable.
    ///
    /// It can be configured to check only private members or all class properties.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     private onlyModifiedInConstructor = 1;
    ///     constructor(
    ///         member1: number,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     constructor(
    ///        private constructorParameter: number,
    ///     ) {
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     private neverModifiedMember = true;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     #neverModifiedPrivateField = 3;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// class Container {
    ///     private readonly neverModifiedMember = true;
    ///     private readonly onlyModifiedInConstructor: number;
    ///     readonly #neverModifiedPrivateField = 3;
    ///
    ///     public constructor(
    ///         onlyModifiedInConstructor: number,
    ///         private readonly neverModifiedParameter: string,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkAllProperties`
    ///
    /// Checks whether all class properties (including public and protected) should be analyzed.
    /// By default, `checkAllProperties` is set to `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "checkAllProperties": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// class Example {
    ///     public constantValue = 42;
    ///
    ///     constructor(value: number) {
    ///         this.constantValue = value;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// class Example {
    ///     constructor(protected constructorParameter: string) {
    ///     }
    /// }
    /// ```
    ///
    pub UseReadonlyClassProperties {
        version: "2.1.0",
        name: "useReadonlyClassProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-readonly").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseReadonlyClassProperties {
    type Query = Ast<JsClassDeclaration>;
    type State = AnyPropertyMember;
    type Signals = Box<[Self::State]>;
    type Options = UseReadonlyClassPropertiesOptions;

    /// Produces the lint candidate signals: non-readonly class properties or constructor parameters
    /// that are never reassigned within the class (TypeScript files only).
    ///
    /// This inspects the class declaration from the rule `Query`, collects constructor-parameter
    /// properties and non-readonly class properties (optionally restricted to private members),
    /// excludes members that are written to anywhere in the class body, and returns the remaining
    /// candidates as a boxed slice of `AnyPropertyMember` to be used as the rule's signals.
    ///
    /// Returns an empty slice for non-TypeScript sources.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Given a `RuleContext` `ctx` for a TypeScript class, running the rule produces
    /// // candidates for adding `readonly`.
    /// let signals = UseReadonlyClassProperties.run(&ctx);
    /// // `signals` contains `AnyPropertyMember` entries for members that are never reassigned.
    /// ```
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        if !source_type.is_typescript() {
            return Box::default();
        }

        let root = ctx.query();
        let members = root.members();
        let private_only = !ctx.options().check_all_properties;
        let ClassMemberReferences { writes, .. } = class_member_references(&members);

        let constructor_params: Vec<_> =
            collect_non_readonly_constructor_parameters(root, private_only);
        let non_readonly_class_property_members =
            collect_non_readonly_class_member_properties(&members, private_only);

        constructor_params
            .clone()
            .into_iter()
            .chain(
                non_readonly_class_property_members.filter(|class_property_member| {
                    !constructor_params.clone().into_iter().any(|node| {
                        node.to_trimmed_text() == class_property_member.to_trimmed_text()
                    })
                }),
            )
            .filter_map(|prop_or_param| {
                if writes
                    .clone()
                    .into_iter()
                    .any(|ClassMemberReference { name, .. }| {
                        if let Some(TextAndRange { text, .. }) =
                            extract_property_or_param_range_and_text(&prop_or_param.clone())
                        {
                            return name.eq(&text);
                        }

                        false
                    })
                {
                    None
                } else {
                    Some(prop_or_param.clone())
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Creates a diagnostic for a class member that is never reassigned.
    ///
    /// Returns `Some(RuleDiagnostic)` pointing at the member's name if the given
    /// property-or-parameter (`node`) has a resolvable name and is never written
    /// after construction; otherwise returns `None`.
    ///
    /// The diagnostic message is "Member '<name>' is never reassigned." and it
    /// includes a note recommending the `readonly` modifier to improve safety.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Given a rule context `ctx` and a candidate `member: AnyPropertyMember`,
    /// // call `diagnostic(&ctx, &member)` to produce a diagnostic suggesting
    /// // that the member be marked `readonly` when it is never reassigned.
    /// let diag = diagnostic(&ctx, &member);
    /// ```
    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let TextAndRange { text, range } = extract_property_or_param_range_and_text(&node.clone())?;

        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Member '"{text.text()}"' is never reassigned."
            },
        ).note(markup! {
                "Using "<Emphasis>"readonly"</Emphasis>" improves code safety, clarity, and helps prevent unintended mutations."
            }),
        )
    }

    /// Create a fix action that inserts a `readonly` modifier for the given class member.
    ///
    /// This constructs a mutation that adds a `readonly` token to either:
    /// - a `JsPropertyClassMember`'s modifier list, preserving annotation, semicolon, and value; or
    /// - a `TsPropertyParameter`'s modifier list (constructor parameter declared as a property),
    ///   preserving decorators and the parameter's formal representation.
    ///
    /// The returned `JsRuleAction` contains the prepared mutation and an explanatory message
    /// ("Add `readonly` decorator."). The function never performs the mutation itself; it only
    /// builds and returns the action for the linter to apply.
    fn action(ctx: &RuleContext<Self>, node: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let original_node = node.syntax();
        let readonly_token = make::ts_readonly_modifier(JsSyntaxToken::new_detached(
            JsSyntaxKind::TS_READONLY_MODIFIER,
            "readonly ",
            [],
            [TriviaPiece::whitespace(1)],
        ));

        if let Some(AnyPropertyMember::JsPropertyClassMember(member)) =
            AnyPropertyMember::cast(original_node.clone())
        {
            if let Ok(member_name) = member.name() {
                let replace_modifiers = make::js_property_modifier_list(
                    member
                        .modifiers()
                        .iter()
                        .chain(once(AnyJsPropertyModifier::TsReadonlyModifier(
                            readonly_token,
                        )))
                        .collect::<Vec<_>>(),
                );

                if let Some(modified_member) =
                    extract_property_member_name_trimmed_whitespace(member_name.clone())
                {
                    let mut builder =
                        make::js_property_class_member(replace_modifiers, modified_member);

                    if let Some(property_annotation) = member.property_annotation() {
                        builder = builder.with_property_annotation(property_annotation);
                    }

                    if let Some(semicolon_token) = member.semicolon_token() {
                        builder = builder.with_semicolon_token(semicolon_token);
                    }

                    if let Some(value) = member.value() {
                        builder = builder.with_value(value);
                    }

                    mutation.replace_node(member.clone(), builder.build());
                }
            }
        } else if let Some(AnyPropertyMember::TsPropertyParameter(parameter)) =
            AnyPropertyMember::cast(original_node.clone())
        {
            let replace_modifiers = make::ts_property_parameter_modifier_list(
                parameter
                    .modifiers()
                    .iter()
                    .chain(once(AnyTsPropertyParameterModifier::TsReadonlyModifier(
                        readonly_token,
                    )))
                    .collect::<Vec<_>>(),
            );

            if let Ok(formal_parameter) = parameter.formal_parameter() {
                let replace_parameter = make::ts_property_parameter(
                    parameter.decorators(),
                    replace_modifiers,
                    formal_parameter,
                );

                mutation.replace_node_discard_trivia(parameter.clone(), replace_parameter);
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"readonly"</Emphasis>" decorator." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug)]
struct TextAndRange {
    text: Text,
    range: TextRange,
}

/// Collects class properties that are not marked `readonly` and are eligible to be suggested as `readonly`.
///
/// This returns an iterator over `AnyPropertyMember::JsPropertyClassMember` items for class members that:
/// - are not `readonly`, `static`, or `accessor`,
/// - do not use computed names, and
/// - if `private_only` is true, are private (either via a `#` private name or a `private` accessibility modifier).
///
/// # Parameters
///
/// - `members`: the class member list to inspect.
/// - `private_only`: when true, restricts results to private properties only.
///
/// # Returns
///
/// An iterator yielding `AnyPropertyMember` entries representing non-readonly class properties that match the filters above.
///
/// # Examples
///
/// ```rust,no_run
/// # use crate::class_member_references::AnyPropertyMember;
/// # use crate::collect_non_readonly_class_member_properties;
/// # use biome_js_syntax::JsClassMemberList;
/// let members: JsClassMemberList = /* obtain from parsed class declaration */ unimplemented!();
/// let candidates = collect_non_readonly_class_member_properties(&members, true);
/// for candidate in candidates {
///     match candidate {
///         AnyPropertyMember::JsPropertyClassMember(prop) => {
///             // `prop` is a non-readonly, private class property candidate
///         }
///         _ => {}
///     }
/// }
/// ```
fn collect_non_readonly_class_member_properties(
    members: &JsClassMemberList,
    private_only: bool,
) -> impl Iterator<Item = AnyPropertyMember> {
    members.iter().filter_map(move |member| {
        let property_class_member = member.as_js_property_class_member()?;

        let is_js_computed_name = property_class_member
            .name()
            .iter()
            .any(|name| name.as_js_computed_member_name().is_some());

        if property_class_member.modifiers().iter().any(|modifier| {
            modifier.as_ts_readonly_modifier().is_some()
                || modifier.as_js_static_modifier().is_some()
                || modifier.as_js_accessor_modifier().is_some()
        }) || is_js_computed_name
        {
            return None;
        }

        let some_property = Some(AnyPropertyMember::JsPropertyClassMember(
            property_class_member.clone(),
        ));

        if !private_only {
            return some_property;
        }

        let is_private = matches!(
            member.name().ok()??,
            AnyJsClassMemberName::JsPrivateClassMemberName(_)
        ) || property_class_member.modifiers().iter().any(|x| {
            TsAccessibilityModifier::cast(x.into_syntax())
                .is_some_and(|modifier| modifier.is_private())
        });

        if is_private {
            return some_property;
        }
        None
    })
}

/// Collects mutable (non-`readonly`) constructor parameters that are declared as class properties.
///
/// Scans the class declaration's constructor (if present) and returns a `Vec<AnyPropertyMember>`
/// containing each `TsPropertyParameter` that is not marked `readonly`. When `private_only` is
/// `true`, only parameters with private visibility are returned; otherwise all non-readonly
/// property-parameters are included.
///
/// The returned members represent constructor parameters that become class fields (e.g.,
/// `constructor(private a: string, public b: number) {}` yields members for `a` and `b`).
///
/// # Parameters
///
/// - `class_declaration`: The class AST node to inspect for a constructor and its parameters.
/// - `private_only`: If `true`, restrict results to parameters declared with private visibility.
///
/// # Returns
///
/// A vector of `AnyPropertyMember::TsPropertyParameter` entries for each mutable constructor
/// parameter that should be considered as a candidate to be marked `readonly`.
///
/// # Examples
///
/// ```
/// // Given a class AST node `class_decl`, find non-readonly constructor-backed properties:
/// let candidates = collect_non_readonly_constructor_parameters(&class_decl, /*private_only=*/ false);
/// ```
fn collect_non_readonly_constructor_parameters(
    class_declaration: &JsClassDeclaration,
    private_only: bool,
) -> Vec<AnyPropertyMember> {
    class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        })
        .into_iter()
        .filter_map(|constructor| constructor.parameters().ok())
        .flat_map(|constructor_params| constructor_params.parameters().iter())
        .filter_map(move |param| match param.ok()? {
            AnyJsConstructorParameter::TsPropertyParameter(ts_property)
                if is_non_readonly_and_optionally_private(&ts_property, private_only) =>
            {
                Some(AnyPropertyMember::TsPropertyParameter(ts_property))
            }
            _ => None,
        })
        .collect()
}

/// Removes leading whitespace from `#privateProperty` names. Without this, the name might include
/// unwanted whitespace (e.g., "\n #privateProperty"). This ensures that when adding modifiers like
/// `readonly`, they are appended correctly without being affected by the whitespace.
fn extract_property_member_name_trimmed_whitespace(
    member_name: AnyJsClassMemberName,
) -> Option<AnyJsClassMemberName> {
    match member_name {
        AnyJsClassMemberName::JsPrivateClassMemberName(name) => {
            let hash_token = name.hash_token().ok()?;
            let new_hash_token = hash_token.with_leading_trivia([]);
            let trimmed = name.replace_token_discard_trivia(hash_token, new_hash_token)?;

            Some(AnyJsClassMemberName::JsPrivateClassMemberName(trimmed))
        }
        _ => Some(member_name),
    }
}

/// Returns true if the given TypeScript constructor property parameter is mutable (not marked `readonly`).

///

/// When `private_only` is true, the function additionally requires the parameter to be `private`.

/// Returns false if the parameter is readonly or, when `private_only` is set, not private.

///

/// # Parameters

/// - `param`: the constructor property parameter to inspect.

/// - `private_only`: if true, only consider parameters with `private` accessibility.

///

/// # Returns

/// `true` when the parameter is non-readonly and (if requested) private; otherwise `false`.
fn is_non_readonly_and_optionally_private(param: &TsPropertyParameter, private_only: bool) -> bool {
    let is_mutable = param
        .modifiers()
        .into_iter()
        .all(|any_modifier| TsReadonlyModifier::cast(any_modifier.into_syntax()).is_none());

    let is_private = param.modifiers().iter().any(|modifier| {
        modifier
            .as_ts_accessibility_modifier()
            .is_some_and(|modifier| modifier.is_private())
    });

    is_mutable && (!private_only || is_private)
}

/// Returns the identifier text and source range for a class property member or a constructor parameter.
///
/// Given an `AnyPropertyMember` that is either a `JsPropertyClassMember` or a `TsPropertyParameter`,
/// this function extracts the member/parameter name's trimmed text and its `TextRange`.
/// Returns `None` when the input is neither supported or the name cannot be resolved.
///
/// # Examples
///
/// ```
/// // Assume `member` is an `AnyPropertyMember` obtained from analysis of a class.
/// // let member: AnyPropertyMember = ...;
/// // let result = extract_property_or_param_range_and_text(&member);
/// // assert!(result.is_some());
/// ```
fn extract_property_or_param_range_and_text(
    property_or_param: &AnyPropertyMember,
) -> Option<TextAndRange> {
    if let Some(AnyPropertyMember::JsPropertyClassMember(member)) =
        AnyPropertyMember::cast(property_or_param.clone().into())
    {
        if let Ok(member_name) = member.name() {
            return Some(TextAndRange {
                text: member_name.to_trimmed_text(),
                range: member_name.range(),
            });
        }
        return None;
    }

    if let Some(AnyPropertyMember::TsPropertyParameter(parameter)) =
        AnyPropertyMember::cast(property_or_param.clone().into())
    {
        let name = parameter
            .formal_parameter()
            .ok()?
            .as_js_formal_parameter()?
            .binding()
            .ok()?;

        return Some(TextAndRange {
            text: name.to_trimmed_text(),
            range: name.range(),
        });
    }

    None
}
