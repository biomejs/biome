use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use crate::services::semantic_class::{
    AccessKind, AnyNamedClassMember, ClassMemberReference, ClassMemberReferences,
    SemanticClassModel,
};
use crate::utils::rename::RenameSymbolExtensions;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{ReferencesExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsComputedMember, AnyJsExpression,
    AnyJsFormalParameter, AnyJsName, AnyTsType, JsClassDeclaration, JsSyntaxNode,
    TsAccessibilityModifier, TsPropertyParameter, TsReferenceType, TsStringLiteralType,
    TsTypeAliasDeclaration, TsUnionType,
};
use biome_js_syntax::{JsFormalParameter, JsIdentifierExpression};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TextRange, declare_node_union,
};
use biome_rule_options::no_unused_private_class_members::NoUnusedPrivateClassMembersOptions;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow unused private class members
    ///
    /// Private class members that are declared and not used anywhere in the code are most likely an error due to incomplete refactoring.
    /// Such class members take up space in the code and can lead to confusion by readers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class OnlyWrite {
    ///   #usedOnlyInWrite = 5;
    ///
    ///   method() {
    ///        this.#usedOnlyInWrite = 212;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    ///  class TsBioo {
    ///    private unusedProperty = 5;
    ///  }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    ///  class TsBioo {
    ///    private unusedMethod() {}
    ///  }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class UsedMember {
    ///   #usedMember = 42;
    ///
    ///   method() {
    ///        return this.#usedMember;
    ///   }
    /// }
    /// ```
    ///
    /// ## Caveats
    ///
    /// The rule currently considers that all TypeScript private members are used if it encounters a computed access.
    /// In the following example `member` is not reported. It is considered as used.
    ///
    /// ```ts
    ///  class TsBioo {
    ///    private member: number;
    ///    private anotherMember: number;
    ///
    ///    set_with_name(name: 'member' | 'anotherMember', value: number) {
    ///      this[name] = value;
    ///    }
    ///  }
    /// ```
    ///
    pub NoUnusedPrivateClassMembers {
        version: "1.3.3",
        name: "noUnusedPrivateClassMembers",
        language: "js",
        sources: &[RuleSource::Eslint("no-unused-private-class-members").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyMember = AnyJsClassMember | TsPropertyParameter
}

#[derive(Debug, Clone)]
pub enum UnusedMemberAction {
    RemoveMember {
        member: AnyMember,
        semantic_class: Arc<SemanticClassModel>,
    },
    RemovePrivateModifier {
        member: AnyMember,
        rename_with_underscore: bool,
        semantic_class: Arc<SemanticClassModel>,
    },
}

impl UnusedMemberAction {
    fn property_range(&self) -> Option<TextRange> {
        match self {
            Self::RemoveMember {
                member,
                semantic_class,
            } => member.member_range(semantic_class),
            Self::RemovePrivateModifier {
                member,
                semantic_class,
                ..
            } => member.member_range(semantic_class),
        }
    }
}

impl Rule for NoUnusedPrivateClassMembers {
    type Query = Semantic<JsClassDeclaration>;
    type State = UnusedMemberAction;
    type Signals = Box<[Self::State]>;
    type Options = NoUnusedPrivateClassMembersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let private_members: Vec<AnyMember> = get_all_declared_private_members(node).collect();
        if private_members.is_empty() {
            Box::default()
        } else {
            let mut results = Vec::new();
            let semantic_class = Arc::new(SemanticClassModel::default());
            let class_member_references = semantic_class
                .clone()
                .class_member_references(&node.members());
            let used_members = traverse_computed_members_usage(
                ctx.model(),
                &semantic_class,
                node.syntax(),
                private_members.as_slice(),
            );

            // Filter out members that are already used
            let mut unused_members: Vec<_> = private_members
                .into_iter()
                .filter(|member| !used_members.contains(member))
                .collect();

            unused_members = traverse_meaningful_read_members_usage(
                &semantic_class,
                unused_members,
                &class_member_references,
            );

            for member in unused_members {
                match &member {
                    AnyMember::AnyJsClassMember(_) => {
                        results.push(UnusedMemberAction::RemoveMember {
                            member,
                            semantic_class: semantic_class.clone(),
                        });
                    }
                    AnyMember::TsPropertyParameter(ts_property_param) => {
                        // Check if the parameter is also unused in constructor body using semantic analysis
                        let should_rename =
                            check_ts_property_parameter_usage(ctx, ts_property_param);
                        results.push(UnusedMemberAction::RemovePrivateModifier {
                            member,
                            semantic_class: semantic_class.clone(),
                            rename_with_underscore: should_rename,
                        });
                    }
                }
            }
            results.into_boxed_slice()
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UnusedMemberAction::RemoveMember { .. } => Some(RuleDiagnostic::new(
                rule_category!(),
                state.property_range(),
                markup! {
                    "This private class member is defined but never used."
                },
            )),
            UnusedMemberAction::RemovePrivateModifier {
                rename_with_underscore,
                ..
            } => {
                if *rename_with_underscore {
                    Some(RuleDiagnostic::new(
                        rule_category!(),
                        state.property_range(),
                        markup! {
                            "This private class member is defined but never used."
                        },
                    ))
                } else {
                    Some(RuleDiagnostic::new(
                        rule_category!(),
                        state.property_range(),
                        markup! {
                            "This parameter is never used outside of the constructor."
                        },
                    ))
                }
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match state {
            UnusedMemberAction::RemoveMember { member, .. } => {
                mutation.remove_node(member.clone());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove unused declaration." }.to_owned(),
                    mutation,
                ))
            }
            UnusedMemberAction::RemovePrivateModifier {
                member,
                rename_with_underscore,
                ..
            } => {
                if let AnyMember::TsPropertyParameter(ts_property_param) = member {
                    // Remove the private modifier
                    let modifiers = ts_property_param.modifiers();
                    for modifier in modifiers.iter() {
                        if let Some(accessibility_modifier) =
                            TsAccessibilityModifier::cast(modifier.into_syntax())
                            && accessibility_modifier.is_private()
                        {
                            mutation.remove_node(accessibility_modifier);
                            break;
                        }
                    }
                    // If needed, rename with underscore prefix
                    if *rename_with_underscore
                        && let Ok(AnyJsFormalParameter::JsFormalParameter(param)) =
                            ts_property_param.formal_parameter()
                    {
                        let binding = param.binding().ok()?;
                        let identifier_binding =
                            binding.as_any_js_binding()?.as_js_identifier_binding()?;
                        let name_token = identifier_binding.name_token().ok()?;
                        let name_trimmed = name_token.text_trimmed();
                        let new_name = format!("_{name_trimmed}");
                        if !mutation.rename_node_declaration(
                            ctx.model(),
                            identifier_binding,
                            &new_name,
                        ) {
                            return None;
                        }
                    }
                }
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove private modifier" }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

/// Filters out private class members that are read meaningfully in the class.
///
/// Returns only private members **not read meaningfully**.
fn traverse_meaningful_read_members_usage(
    semantic_class: &SemanticClassModel,
    private_members: Vec<AnyMember>,
    class_member_references: &ClassMemberReferences,
) -> Vec<AnyMember> {
    let ClassMemberReferences { reads, .. } = class_member_references;

    private_members
        .into_iter()
        .filter_map(|private_member| {
            if !reads
                .iter()
                .filter(|read| read.access_kind == AccessKind::MeaningfulRead)
                .any(|reference| {
                    let ClassMemberReference { name, .. } = reference;
                    private_member.matches_name(semantic_class, name)
                })
            {
                Some(private_member)
            } else {
                None
            }
        })
        .collect()
}

/// Filters out private members that are used via computed property access (`this[something]`).
///
/// # Example
/// ```ts
/// class Example {
///     private tsMember: number;
///     #sharpMember: number;
///
///     method() {
///         this["tsMember"]; // counts as usage for tsMember
///     }
/// }
/// ```
/// After calling this function, `#sharpMember` remains; `tsMember` is removed.
///
/// Returns only used members.
fn traverse_computed_members_usage(
    model: &SemanticModel,
    semantic_class_model: &SemanticClassModel,
    syntax: &JsSyntaxNode,
    private_members: &[AnyMember],
) -> Vec<AnyMember> {
    let mut used_members = Vec::new();

    for node in syntax.descendants() {
        if !AnyJsName::can_cast(node.kind())
            && let Some(computed_member) = AnyJsComputedMember::cast(node)
            && matches!(
                computed_member.object(),
                Ok(AnyJsExpression::JsThisExpression(_))
            )
            && let Ok(member_expr) = computed_member.member()
            && let Some(id_expr) = JsIdentifierExpression::cast_ref(member_expr.syntax())
            && let Some(ty) = resolve_formal_param_type(model, &id_expr)
            && let Some(ts_union_type) = TsUnionType::cast(ty.syntax().clone())
                .or_else(|| resolve_reference_to_union(model, &ty))
        {
            let items: Vec<_> = extract_literal_types(&ts_union_type);

            // Collect only members that match the computed member names
            used_members.extend(
                private_members
                    .iter()
                    .filter(|member| {
                        items
                            .iter()
                            .any(|name| member.matches_name(semantic_class_model, name))
                    })
                    .cloned(),
            );
        }
    }

    used_members
}

/// Check if a TsPropertyParameter is also unused as a function parameter
fn check_ts_property_parameter_usage(
    ctx: &RuleContext<NoUnusedPrivateClassMembers>,
    ts_property_param: &TsPropertyParameter,
) -> bool {
    if let Ok(AnyJsFormalParameter::JsFormalParameter(param)) = ts_property_param.formal_parameter()
        && let Ok(binding) = param.binding()
        && let Some(identifier_binding) = binding
            .as_any_js_binding()
            .and_then(|b| b.as_js_identifier_binding())
    {
        let name_token = match identifier_binding.name_token() {
            Ok(token) => token,
            Err(_) => return false,
        };

        let name = name_token.text_trimmed();

        if name.starts_with('_') {
            return false;
        }

        if identifier_binding
            .all_references(ctx.model())
            .next()
            .is_some()
        {
            return false;
        }

        return true;
    }

    false
}

fn get_all_declared_private_members(
    class_declaration: &JsClassDeclaration,
) -> impl Iterator<Item = AnyMember> {
    class_declaration
        .members()
        .iter()
        .map(AnyMember::AnyJsClassMember)
        .chain(get_constructor_params(class_declaration))
        .filter(|member| member.is_private() == Some(true))
}

fn get_constructor_params(
    class_declaration: &JsClassDeclaration,
) -> impl Iterator<Item = AnyMember> {
    class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        })
        .and_then(|constructor_member| constructor_member.parameters().ok())
        .into_iter()
        .flat_map(|constructor_params| {
            constructor_params
                .parameters()
                .iter()
                .filter_map(|param| match param.ok()? {
                    biome_js_syntax::AnyJsConstructorParameter::TsPropertyParameter(
                        ts_property,
                    ) => Some(ts_property.into()),
                    _ => None,
                })
        })
}

impl AnyMember {
    fn is_private(&self) -> Option<bool> {
        match self {
            Self::AnyJsClassMember(member) => {
                let is_es_private = matches!(
                    member.name().ok()??,
                    AnyJsClassMemberName::JsPrivateClassMemberName(_)
                );
                let is_ts_private = match member {
                    AnyJsClassMember::JsGetterClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsMethodClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsPropertyClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsSetterClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    _ => false,
                };

                Some(is_es_private || is_ts_private)
            }
            Self::TsPropertyParameter(param) => Some(
                param
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                    .any(|accessibility| accessibility.is_private()),
            ),
        }
    }

    fn member_range(&self, semantic_class: &SemanticClassModel) -> Option<TextRange> {
        if let Some(any_named_class_member) = AnyNamedClassMember::cast(self.syntax().clone())
            && let Some(prop_name) = semantic_class.extract_named_member(&any_named_class_member)
        {
            return Some(prop_name.range);
        }

        None
    }

    fn matches_name(&self, semantic_class: &SemanticClassModel, name: &Text) -> bool {
        if let Some(any_named_class_member) = AnyNamedClassMember::cast(self.syntax().clone())
            && let Some(prop_name) = semantic_class.extract_named_member(&any_named_class_member)
        {
            return prop_name.name.eq(name);
        }

        false
    }
}

/// Extracts the immediate string literal types only from a union like `A | B | C`.
/// Filters out any non string literal type.
/// Does not recurse into nested unions.
pub fn extract_literal_types(union: &TsUnionType) -> Vec<Text> {
    extract_shallow_union_members(union)
        .iter()
        .filter_map(|item| {
            if let Some(literal_type) = TsStringLiteralType::cast(item.syntax().clone()) {
                return Some(Text::new_owned(Box::from(
                    literal_type
                        .to_trimmed_text()
                        .trim_matches(&['"', '\''][..]),
                )));
            }

            None
        })
        .collect()
}

/// Extracts the immediate types from a union like `A | B | C`.
/// Does not recurse into nested unions.
fn extract_shallow_union_members(union: &TsUnionType) -> Vec<AnyTsType> {
    let mut members = Vec::new();

    for ty in union.types().into_iter().flatten() {
        members.push(ty);
    }

    members
}

/// Attempts to resolve the type annotation of a formal parameter for the given identifier expression.
/// - Looks up the binding for the identifier expression in the semantic model.
/// - Checks if the binding corresponds to a `JsFormalParameter`.
/// - If so, extracts and returns its type annotation.
fn resolve_formal_param_type(
    model: &SemanticModel,
    id_expr: &JsIdentifierExpression,
) -> Option<AnyTsType> {
    // Get the reference identifier inside `id_expr`
    let ref_ident = id_expr.name().ok()?;

    // Find the binding
    let binding = model.binding(&ref_ident)?;

    // Look at the parent node of the binding
    let parent = binding.syntax().parent()?;

    // If the parent is a formal parameter, extract its type annotation
    let js_param = JsFormalParameter::cast(parent)?;
    let type_annotation = js_param.type_annotation()?;
    type_annotation.ty().ok()
}

/// Resolves a type reference to its aliased union type, if the reference points to a union.
fn resolve_reference_to_union(model: &SemanticModel, ty: &AnyTsType) -> Option<TsUnionType> {
    let ts_reference_type = TsReferenceType::cast(ty.syntax().clone())?;
    // Get the reference identifier
    let ref_name = ts_reference_type.name().ok()?;
    let ref_ident = ref_name.as_js_reference_identifier()?;

    // Resolve the binding
    let binding = model.binding(ref_ident)?;
    let parent = binding.syntax().parent()?;

    // If it's a type alias, get its type, anything else is ignored, since it can not lead to type union of strings
    // names of class members are strings
    let type_alias = TsTypeAliasDeclaration::cast(parent)?;
    let ty = type_alias.ty().ok()?;

    // See if that type is a union
    TsUnionType::cast(ty.into_syntax())
}
