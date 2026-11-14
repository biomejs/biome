use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsStatement, AnyTsName, AnyTsType, AnyTsTypeMember, JsLanguage, JsSyntaxKind, T, TextRange,
    TsIndexSignatureTypeMember, TsInterfaceDeclaration, TsMappedType, TsNumberType, TsObjectType,
    TsReferenceType, TsReturnTypeAnnotation, TsStringType, TsSymbolType, TsTypeAliasDeclaration,
    TsTypeAnnotation, TsTypeMemberList,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxKindSet, SyntaxNodeOptionExt,
    TriviaPieceKind, declare_node_union,
};
use biome_rule_options::use_consistent_indexed_object_style::{
    IndexedObjectStyle, UseConsistentIndexedObjectStyleOptions,
};

declare_lint_rule! {
    /// Require a consistent style for object types with arbitrary keys.
    ///
    /// TypeScript offers several ways to write the type of an object whose keys are not known in advance:
    /// an index signature (`{ [key: string]: unknown }`), a mapped type (`{ [key in string]: unknown }`),
    /// or the built-in `Record` type (`Record<string, unknown>`).
    /// All three describe the same type.
    ///
    /// Using one style throughout the code makes these types easier to read.
    /// By default, this rule requires the `Record` type.
    ///
    /// The rule doesn't report object types that refer to themselves,
    /// such as `interface Tree { [key: string]: Tree }`, because TypeScript doesn't allow a `Record` type to refer to itself.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Scores {
    ///     [name: string]: number;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Scores = {
    ///     [name: string]: number;
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Scores = {
    ///     [name in string]: number;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// type Scores = Record<string, number>;
    ///
    /// interface Player {
    ///     name: string;
    ///     [key: string]: unknown;
    /// }
    ///
    /// interface Tree {
    ///     [key: string]: Tree;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Default: `"record"`
    ///
    /// The style to require:
    /// - `"record"`: object types with arbitrary keys must use the `Record` type.
    /// - `"indexSignature"`: object types with arbitrary keys must use an index signature.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "indexSignature"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,expect_diagnostic,use_options
    /// type Scores = Record<string, number>;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```ts,use_options
    /// type Scores = {
    ///     [name: string]: number;
    /// };
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to require `interface` or `type` for object types, see [`useConsistentTypeDefinitions`](https://biomejs.dev/linter/rules/use-consistent-type-definitions/).
    pub UseConsistentIndexedObjectStyle {
        version: "next",
        name: "useConsistentIndexedObjectStyle",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("consistent-indexed-object-style").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyIndexedObjectType =
        TsInterfaceDeclaration
        | TsObjectType
        | TsMappedType
        | TsReferenceType
}

impl Rule for UseConsistentIndexedObjectStyle {
    type Query = Semantic<AnyIndexedObjectType>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentIndexedObjectStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match (node, ctx.options().style()) {
            (
                AnyIndexedObjectType::TsInterfaceDeclaration(interface),
                IndexedObjectStyle::Record,
            ) => {
                let member = single_index_signature(&interface.members())?;
                if !is_convertible_index_signature(&member) {
                    return None;
                }
                let target = interface.id().ok()?.range();
                let mut visited = Vec::new();
                (!members_reference_type(&interface.members(), target, model, &mut visited))
                    .then_some(())
            }
            (AnyIndexedObjectType::TsObjectType(object), IndexedObjectStyle::Record) => {
                let member = single_index_signature(&object.members())?;
                if !is_convertible_index_signature(&member) {
                    return None;
                }
                if let Some(target) = enclosing_type_alias(node) {
                    let mut visited = Vec::new();
                    if members_reference_type(&object.members(), target, model, &mut visited) {
                        return None;
                    }
                }
                Some(())
            }
            (AnyIndexedObjectType::TsMappedType(mapped), IndexedObjectStyle::Record) => {
                // `as` clauses rename keys, which `Record` can't express.
                if mapped.as_clause().is_some() {
                    return None;
                }
                // When the value type depends on the key, there is no matching `Record`.
                let key = mapped.property_name().ok()?;
                if model.as_binding(&key).all_references().next().is_some() {
                    return None;
                }
                // `[K in keyof T]` copies the `readonly` and `?` modifiers from `T`, and
                // `Record` doesn't. Parentheses (`[K in (keyof T)]`) turn this behavior off.
                if let AnyTsType::TsTypeOperatorType(operator) = mapped.keys_type().ok()?
                    && operator.operator_token().ok()?.kind() == T![keyof]
                {
                    return None;
                }
                if let Some(target) = enclosing_type_alias(node)
                    && let Some(value) = mapped.mapped_type().and_then(|value| value.ty().ok())
                {
                    let mut visited = Vec::new();
                    if references_type(value, target, model, &mut visited) {
                        return None;
                    }
                }
                Some(())
            }
            (
                AnyIndexedObjectType::TsReferenceType(reference),
                IndexedObjectStyle::IndexSignature,
            ) => {
                let AnyTsName::JsReferenceIdentifier(name) = reference.name().ok()? else {
                    return None;
                };
                if name.value_token().ok()?.text_trimmed() != "Record"
                    || model.binding(&name).is_some()
                {
                    return None;
                }
                (reference.type_arguments()?.ts_type_argument_list().len() == 2).then_some(())
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let message = match node {
            AnyIndexedObjectType::TsInterfaceDeclaration(_)
            | AnyIndexedObjectType::TsObjectType(_) => markup! {
                "Use a "<Emphasis>"Record"</Emphasis>" type instead of an index signature."
            },
            AnyIndexedObjectType::TsMappedType(_) => markup! {
                "Use a "<Emphasis>"Record"</Emphasis>" type instead of a mapped type."
            },
            AnyIndexedObjectType::TsReferenceType(_) => markup! {
                "Use an index signature instead of the "<Emphasis>"Record"</Emphasis>" type."
            },
        };
        Some(
            RuleDiagnostic::new(rule_category!(), node.range(), message).note(markup! {
                "Using one style for object types with arbitrary keys makes the code easier to read."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let message = match node {
            AnyIndexedObjectType::TsReferenceType(reference) => {
                let object = record_to_index_signature(node)?;
                mutation.replace_node(AnyTsType::from(reference.clone()), object);
                markup! { "Use an index signature." }
            }
            // The `Record` fix refers to built-in types by name, so a local type with the same
            // name would change its meaning.
            _ if shadows_built_in_type(ctx.model(), node) => return None,
            AnyIndexedObjectType::TsInterfaceDeclaration(interface) => {
                // `export default type` is not valid syntax, and a type alias can't extend.
                if interface.extends_clause().is_some()
                    || interface.syntax().parent().kind()
                        == Some(JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)
                {
                    return None;
                }
                // Type aliases of a `Record` don't accept `in` and `out` modifiers.
                if interface.type_parameters().is_some_and(|parameters| {
                    parameters
                        .items()
                        .iter()
                        .flatten()
                        .any(|parameter| !parameter.modifiers().is_empty())
                }) {
                    return None;
                }
                let record = index_signature_to_record(node)?;
                let mut alias = make::ts_type_alias_declaration(
                    make::token(T![type])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    interface.id().ok()?.trim_trivia()?,
                    make::token_decorated_with_space(T![=]),
                    record,
                )
                .with_semicolon_token(make::token(T![;]));
                if let Some(type_parameters) = interface.type_parameters() {
                    alias = alias.with_type_parameters(type_parameters.trim_trivia()?);
                }
                mutation.replace_element(
                    interface.syntax().clone().into(),
                    alias.build().into_syntax().into(),
                );
                markup! { "Use a "<Emphasis>"Record"</Emphasis>" type." }
            }
            AnyIndexedObjectType::TsObjectType(object) => {
                let record = index_signature_to_record(node)?;
                mutation.replace_node(AnyTsType::from(object.clone()), record);
                markup! { "Use a "<Emphasis>"Record"</Emphasis>" type." }
            }
            AnyIndexedObjectType::TsMappedType(mapped) => {
                let record = mapped_type_to_record(node)?;
                mutation.replace_node(AnyTsType::from(mapped.clone()), record);
                markup! { "Use a "<Emphasis>"Record"</Emphasis>" type." }
            }
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

/// Returns the index signature of `members` when it is their only member.
fn single_index_signature(members: &TsTypeMemberList) -> Option<TsIndexSignatureTypeMember> {
    let mut iter = members.iter();
    match (iter.next(), iter.next()) {
        (Some(AnyTsTypeMember::TsIndexSignatureTypeMember(member)), None) => Some(member),
        _ => None,
    }
}

/// Whether the key and value types of `member` are both present.
fn is_convertible_index_signature(member: &TsIndexSignatureTypeMember) -> bool {
    member
        .parameter()
        .and_then(|parameter| parameter.type_annotation())
        .and_then(|annotation| annotation.ty())
        .is_ok()
        && member
            .type_annotation()
            .and_then(|annotation| annotation.ty())
            .is_ok()
}

/// Returns the range of the type alias name when `node` is part of the aliased type.
///
/// The search stops at type annotations, such as property or parameter types,
/// because a type alias may refer to itself through them.
fn enclosing_type_alias(node: &AnyIndexedObjectType) -> Option<TextRange> {
    for ancestor in node.syntax().ancestors().skip(1) {
        if let Some(alias) = TsTypeAliasDeclaration::cast_ref(&ancestor) {
            return Some(alias.binding_identifier().ok()?.range());
        }
        if TYPE_ALIAS_SEARCH_BOUNDARY.matches(ancestor.kind()) {
            return None;
        }
    }
    None
}

/// Nodes that end the search for an enclosing type alias: type annotations, and statements other
/// than the type alias itself.
const TYPE_ALIAS_SEARCH_BOUNDARY: SyntaxKindSet<JsLanguage> = TsTypeAnnotation::KIND_SET
    .union(TsReturnTypeAnnotation::KIND_SET)
    .union(AnyJsStatement::KIND_SET);

/// Whether an index signature in `members` refers to the type declared at `target`
/// in a way that a `Record` type can't express.
fn members_reference_type(
    members: &TsTypeMemberList,
    target: TextRange,
    model: &SemanticModel,
    visited: &mut Vec<TextRange>,
) -> bool {
    members.iter().any(|member| {
        let AnyTsTypeMember::TsIndexSignatureTypeMember(member) = member else {
            return false;
        };
        member
            .type_annotation()
            .and_then(|annotation| annotation.ty())
            .is_ok_and(|ty| references_type(ty, target, model, visited))
    })
}

/// Whether `ty` refers to the type declared at `target` in a way that a `Record` type can't
/// express.
///
/// TypeScript reports an error when a `Record` type refers to itself directly, for example
/// `type Tree = Record<string, Tree>`. References nested inside arrays, functions, tuples, or
/// object properties are fine, so the search doesn't enter those types.
///
/// `visited` holds the declarations that have already been searched.
fn references_type(
    ty: AnyTsType,
    target: TextRange,
    model: &SemanticModel,
    visited: &mut Vec<TextRange>,
) -> bool {
    match ty.omit_parentheses() {
        AnyTsType::TsObjectType(object) => {
            members_reference_type(&object.members(), target, model, visited)
        }
        AnyTsType::TsMappedType(mapped) => mapped
            .mapped_type()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| references_type(ty, target, model, visited)),
        AnyTsType::TsIndexedAccessType(access) => [access.object_type(), access.index_type()]
            .into_iter()
            .flatten()
            .any(|ty| references_type(ty, target, model, visited)),
        AnyTsType::TsConditionalType(conditional) => [
            conditional.check_type(),
            conditional.extends_type(),
            conditional.true_type(),
            conditional.false_type(),
        ]
        .into_iter()
        .flatten()
        .any(|ty| references_type(ty, target, model, visited)),
        AnyTsType::TsUnionType(union) => union
            .types()
            .iter()
            .flatten()
            .any(|ty| references_type(ty, target, model, visited)),
        AnyTsType::TsIntersectionType(intersection) => intersection
            .types()
            .iter()
            .flatten()
            .any(|ty| references_type(ty, target, model, visited)),
        AnyTsType::TsReferenceType(reference) => {
            if let Ok(AnyTsName::JsReferenceIdentifier(name)) = reference.name()
                && let Some(binding) = model.binding(&name)
            {
                if binding.range() == target {
                    return true;
                }
                if !visited.contains(&binding.range()) {
                    visited.push(binding.range());
                    let is_referencing = match binding.tree().declaration() {
                        Some(AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias)) => alias
                            .ty()
                            .is_ok_and(|ty| references_type(ty, target, model, visited)),
                        Some(AnyJsBindingDeclaration::TsInterfaceDeclaration(interface)) => {
                            members_reference_type(&interface.members(), target, model, visited)
                        }
                        _ => false,
                    };
                    if is_referencing {
                        return true;
                    }
                }
            }
            reference.type_arguments().is_some_and(|arguments| {
                arguments
                    .ts_type_argument_list()
                    .iter()
                    .flatten()
                    .any(|ty| references_type(ty, target, model, visited))
            })
        }
        _ => false,
    }
}

/// Builds the `Record` type that replaces `node`, an interface or object type whose only member
/// is an index signature.
///
/// Returns `None` when the replacement would drop a comment.
fn index_signature_to_record(node: &AnyIndexedObjectType) -> Option<AnyTsType> {
    let members = match node {
        AnyIndexedObjectType::TsInterfaceDeclaration(interface) => interface.members(),
        AnyIndexedObjectType::TsObjectType(object) => object.members(),
        _ => return None,
    };
    let member = single_index_signature(&members)?;
    let key = member.parameter().ok()?.type_annotation().ok()?.ty().ok()?;
    let value = member.type_annotation().ok()?.ty().ok()?;
    if has_dropped_comments(node, &[key.range(), value.range()]) {
        return None;
    }
    let record = generic_type("Record", [key.trim_trivia()?, value.trim_trivia()?]);
    Some(if member.readonly_token().is_some() {
        generic_type("Readonly", [record])
    } else {
        record
    })
}

/// Builds the `Record` type that replaces `node`, a mapped type.
///
/// Returns `None` when the replacement would drop a comment, or when the mapped type removes
/// `readonly`, which no built-in type can express.
fn mapped_type_to_record(node: &AnyIndexedObjectType) -> Option<AnyTsType> {
    let AnyIndexedObjectType::TsMappedType(mapped) = node else {
        return None;
    };
    let readonly = mapped.readonly_modifier();
    if readonly
        .as_ref()
        .and_then(|modifier| modifier.operator_token())
        .is_some_and(|operator| operator.kind() == T![-])
    {
        return None;
    }
    let key = mapped.keys_type().ok()?;
    let value = mapped
        .mapped_type()
        .map(|annotation| annotation.ty())
        .transpose()
        .ok()?;
    let mut kept = vec![key.range()];
    kept.extend(value.as_ref().map(AstNode::range));
    if has_dropped_comments(node, &kept) {
        return None;
    }
    let value = match value {
        Some(value) => value.trim_trivia()?,
        None => make::ts_any_type(make::token(T![any])).into(),
    };
    let mut record = generic_type("Record", [key.trim_trivia()?, value]);
    if let Some(optional) = mapped.optional_modifier() {
        let wrapper = match optional.operator_token() {
            Some(operator) if operator.kind() == T![-] => "Required",
            _ => "Partial",
        };
        record = generic_type(wrapper, [record]);
    }
    if readonly.is_some() {
        record = generic_type("Readonly", [record]);
    }
    Some(record)
}

/// Builds the index signature object type that replaces `node`, a `Record` type.
///
/// Returns `None` when the replacement would drop a comment or produce invalid code.
fn record_to_index_signature(node: &AnyIndexedObjectType) -> Option<AnyTsType> {
    let AnyIndexedObjectType::TsReferenceType(reference) = node else {
        return None;
    };
    // `interface A extends { ... }` and `class A implements { ... }` are not valid syntax.
    if reference.syntax().parent().kind() == Some(JsSyntaxKind::TS_TYPE_LIST) {
        return None;
    }
    let arguments = reference.type_arguments()?.ts_type_argument_list();
    let mut arguments = arguments.iter();
    let key = arguments.next()?.ok()?;
    let value = arguments.next()?.ok()?;
    if !INDEX_SIGNATURE_KEY_KINDS.matches(key.syntax().kind())
        || has_dropped_comments(node, &[key.range(), value.range()])
    {
        return None;
    }
    let member = make::ts_index_signature_type_member(
        make::token(T!['[']),
        make::ts_index_signature_parameter(
            make::js_identifier_binding(make::ident("key")),
            type_annotation(key.trim_trivia()?),
        ),
        make::token(T![']']),
        type_annotation(value.trim_trivia()?),
    )
    .build();
    Some(
        make::ts_object_type(
            make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::ts_type_member_list([member.into()]),
            make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )
        .into(),
    )
}

/// Key types that an index signature accepts. Index signatures also accept template literal types
/// and unions of these, which the fix leaves alone.
const INDEX_SIGNATURE_KEY_KINDS: SyntaxKindSet<JsLanguage> = TsStringType::KIND_SET
    .union(TsNumberType::KIND_SET)
    .union(TsSymbolType::KIND_SET);

/// Whether replacing `node` with a new node built from the nodes at `kept` would drop a comment.
///
/// Comments before and after `node` are carried over to the replacement.
fn has_dropped_comments(node: &AnyIndexedObjectType, kept: &[TextRange]) -> bool {
    let node = node.syntax();
    if !node.has_inner_comments() {
        return false;
    }
    let node_range = node.text_trimmed_range();
    node.descendants_tokens(biome_rowan::Direction::Next)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
        })
        .filter(|piece| piece.is_comments())
        .any(|comment| {
            let range = comment.text_range();
            node_range.contains_range(range) && !kept.iter().any(|kept| kept.contains_range(range))
        })
}

/// Whether a declaration visible from `node` shadows one of the built-in types that the
/// `Record` fix refers to.
fn shadows_built_in_type(model: &SemanticModel, node: &AnyIndexedObjectType) -> bool {
    model.scope(node.syntax()).ancestors().any(|scope| {
        RECORD_FIX_TYPE_NAMES
            .iter()
            .any(|name| scope.get_binding(name).is_some())
    })
}

/// The built-in types that the `Record` fix can produce.
const RECORD_FIX_TYPE_NAMES: &[&str] = &["Partial", "Readonly", "Record", "Required"];

/// Builds the type `name<arguments>`.
fn generic_type<const N: usize>(name: &str, arguments: [AnyTsType; N]) -> AnyTsType {
    let separators = (1..N)
        .map(|_| make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
    make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
        make::js_reference_identifier(make::ident(name)),
    ))
    .with_type_arguments(make::ts_type_arguments(
        make::token(T![<]),
        make::ts_type_argument_list(arguments, separators),
        make::token(T![>]),
    ))
    .build()
    .into()
}

/// Builds the type annotation `: ty`.
fn type_annotation(ty: AnyTsType) -> TsTypeAnnotation {
    make::ts_type_annotation(
        make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        ty,
    )
}
