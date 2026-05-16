use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleDomain,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyTsName, AnyTsType, AnyTsTypeMember, JsSyntaxKind, T, TsIndexSignatureTypeMember,
    TsReferenceType, TriviaPieceKind,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_consistent_indexed_object_style::{
    IndexedObjectStyle, UseConsistentIndexedObjectStyleOptions,
};

declare_lint_rule! {
    /// Enforce a consistent style for indexed objects in TypeScript.
    ///
    /// This rule allows you to choose between index signatures and the `Record` utility type for your indexed objects.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// const obj: { [key: string]: number } = {};
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const obj: Record<string, number> = {};
    /// ```
    ///
    pub UseConsistentIndexedObjectStyle {
        version: "next",
        name: "useConsistentIndexedObjectStyle",
        language: "ts",
        recommended: false,
        domains: &[RuleDomain::Types],
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyConsistentIndexedObjectStyle = TsIndexSignatureTypeMember | TsReferenceType
}

impl Rule for UseConsistentIndexedObjectStyle {
    type Query = Ast<AnyConsistentIndexedObjectStyle>;
    type State = IndexedObjectStyle;
    type Signals = Option<Self::State>;
    type Options = UseConsistentIndexedObjectStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let node = ctx.query();

        match node {
            AnyConsistentIndexedObjectStyle::TsIndexSignatureTypeMember(_) => {
                if options.mode == IndexedObjectStyle::Record {
                    return Some(IndexedObjectStyle::Record);
                }
            }
            AnyConsistentIndexedObjectStyle::TsReferenceType(record) => {
                if options.mode == IndexedObjectStyle::IndexSignature {
                    let is_rec = is_record(record);
                    if is_rec {
                        return Some(IndexedObjectStyle::IndexSignature);
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = node.range();

        let (message, note) = match state {
            IndexedObjectStyle::Record => (
                markup! { "Prefer the "<Emphasis>"Record"</Emphasis>" utility type over index signatures." },
                "The Record utility type is more concise and consistent with other utility types."
            ),
            IndexedObjectStyle::IndexSignature => (
                markup! { "Prefer "<Emphasis>"index signatures"</Emphasis>" over the Record utility type." },
                "Index signatures are the standard TypeScript way to define indexed objects."
            ),
        };

        Some(
            RuleDiagnostic::new(rule_category!(), range, message)
                .note(note),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match (node, state) {
            (AnyConsistentIndexedObjectStyle::TsIndexSignatureTypeMember(index_signature), IndexedObjectStyle::Record) => {
                let key_type = index_signature.parameter().ok()?.type_annotation().ok()?.ty().ok()?;
                let value_type = index_signature.type_annotation().ok()?.ty().ok()?;

                let record_type = make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                    make::js_reference_identifier(make::ident("Record")),
                ))
                .with_type_arguments(make::ts_type_arguments(
                    make::token(T![<]),
                    make::ts_type_argument_list(
                        [key_type, value_type],
                        [make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])]
                    ),
                    make::token(T![>]),
                ))
                .build();

                let parent = index_signature.syntax().parent()?;
                if parent.kind() == JsSyntaxKind::TS_TYPE_MEMBER_LIST {
                    let member_list = biome_js_syntax::TsTypeMemberList::cast(parent)?;
                    if member_list.iter().count() == 1 {
                        if let Some(object_type) = member_list.syntax().parent().and_then(AnyTsType::cast) {
                            mutation.replace_node(object_type, AnyTsType::TsReferenceType(record_type));
                        }
                    }
                }
            }
            (AnyConsistentIndexedObjectStyle::TsReferenceType(record), IndexedObjectStyle::IndexSignature) => {
                let type_args = record.type_arguments()?;
                let mut args = type_args.ts_type_argument_list().iter();
                let key_type = args.next()?.ok()?;
                let value_type = args.next()?.ok()?;

                let index_signature = make::ts_index_signature_type_member(
                    make::token(T!['[']),
                    make::ts_index_signature_parameter(
                        make::js_identifier_binding(make::ident("key")),
                        make::ts_type_annotation(make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]), key_type),
                    ),
                    make::token(T![']']),
                    make::ts_type_annotation(make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]), value_type),
                ).build();

                let object_type = make::ts_object_type(
                    make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    make::ts_type_member_list([AnyTsTypeMember::TsIndexSignatureTypeMember(index_signature)]),
                    make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
                );

                mutation.replace_node(AnyTsType::TsReferenceType(record.clone()), AnyTsType::TsObjectType(object_type));
            }
            _ => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state {
                IndexedObjectStyle::Record => markup! { "Use "<Emphasis>"Record<K, V>"</Emphasis>" syntax." }.to_owned(),
                IndexedObjectStyle::IndexSignature => markup! { "Use "<Emphasis>"{ [key: K]: V }"</Emphasis>" syntax." }.to_owned(),
            },
            mutation,
        ))
    }
}

fn is_record(record: &TsReferenceType) -> bool {
    let Ok(name) = record.name() else { return false };
    match name {
        AnyTsName::JsReferenceIdentifier(ident) => {
            ident.value_token().map_or(false, |token| {
                let text = token.text_trimmed();
                text == "Record"
            })
        }
        _ => false,
    }
}
