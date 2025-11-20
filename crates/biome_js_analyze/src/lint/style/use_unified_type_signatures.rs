use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyParameter};
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsBinding, AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsFormalParameter, AnyJsObjectMemberName, AnyJsParameter,
    AnyTsMethodSignatureModifier, AnyTsReturnType, AnyTsType, JsBogusBinding, JsComputedMemberName,
    JsExport, JsIdentifierBinding, JsLanguage, JsLiteralMemberName, JsMetavariable,
    JsPrivateClassMemberName, JsSyntaxKind, JsSyntaxNode, T, TsCallSignatureTypeMember,
    TsConstructSignatureTypeMember, TsConstructorSignatureClassMember,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration, TsDeclareStatement,
    TsMethodSignatureClassMember, TsMethodSignatureTypeMember, TsTypeParameters,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, SyntaxResult, TextRange,
    TriviaPieceKind, chain_trivia_pieces, declare_node_union,
};
use biome_rule_options::use_unified_type_signatures::UseUnifiedTypeSignaturesOptions;

declare_lint_rule! {
    /// Disallow overload signatures that can be unified into a single signature.
    ///
    /// Overload signatures that can be merged into a single signature are redundant and should be avoided.
    /// This rule helps simplify function signatures by combining overloads by making parameters optional
    /// and/or using type unions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// function f(a: number): void;
    /// function f(a: string): void;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface I {
    ///     a(): void;
    ///     a(x: number): void;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function f(a: number | string): void {}
    /// ```
    ///
    /// ```ts
    /// interface I {
    ///     a(x?: number): void;
    /// }
    /// ```
    ///
    /// Different return types cannot be merged:
    /// ```ts
    /// interface I {
    ///     f(): void;
    ///     f(x: number): number;
    /// }
    /// ```
    ///
    /// Different type parameters cannot be merged:
    /// ```ts
    /// function f<T extends number>(x: T): void;
    /// function f<T extends string>(x: T): void;
    /// function f(x: unknown): void {}
    /// ```
    ///
    pub UseUnifiedTypeSignatures {
        version: "2.1.0",
        name: "useUnifiedTypeSignatures",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("unified-signatures").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseUnifiedTypeSignatures {
    type Query = Ast<AnyPotentialTsOverloadSignature>;
    type State = MergeOverloadSignaturesInfo;
    type Signals = Option<Self::State>;
    type Options = UseUnifiedTypeSignaturesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let overload_info = OverloadInfo::from_overload_signature(node)?;

        // Yes, this is a O(n^2) algorithm, but we need to compare all overload signatures
        // with each other. This shouldn't be a problem in practice,
        // since the number of overloads is usually relatively small.
        for (i, overload1) in overload_info.overload_signatures.iter().enumerate() {
            for overload2 in &overload_info.overload_signatures[i + 1..] {
                if !overload1
                    .type_parameters()
                    .is_type_equal(&overload2.type_parameters())
                {
                    // We can only combine signatures if their type parameters are equal.
                    continue;
                }

                if !overload1
                    .return_type_annotation()
                    .is_type_equal(&overload2.return_type_annotation())
                {
                    // We can only combine signatures if their return types are equal.
                    continue;
                }

                let (Some(parameters1), Some(parameters2)) =
                    (overload1.parameters(), overload2.parameters())
                else {
                    continue;
                };

                let Some((signature_to_remove, signature_to_keep, instructions)) = parameters2
                    .try_merge(&parameters1)
                    .map(|instr| (overload1.clone(), overload2.clone(), instr))
                    .or_else(|| {
                        parameters1
                            .try_merge(&parameters2)
                            .map(|instr| (overload2.clone(), overload1.clone(), instr))
                    })
                else {
                    continue;
                };

                return Some(MergeOverloadSignaturesInfo {
                    signature_to_remove,
                    signature_to_extend: signature_to_keep,
                    parameters_to_merge: instructions.into_boxed_slice(),
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.signature_to_remove.overload_range(),
            markup! {
                "Overload signatures are hard to read and maintain."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleAction<JsLanguage>> {
        let mut mutation = ctx.root().begin();

        // When removing the signature, we need to get the wrapper node.
        // That can be either the signature itself or an export or a declare statement.
        let signature_to_remove_wrapper = state.signature_to_remove.wrapper()?;
        let signature_to_extend_wrapper = state.signature_to_extend.wrapper()?;

        // Check if the signature to remove has comments, which we would like to preserve.
        if signature_to_remove_wrapper
            .syntax()
            .first_leading_trivia()
            .filter(|trivia| trivia.pieces().any(|piece| piece.is_comments()))
            .is_some()
        {
            // Transfer comments and whitespace before removing the signature.
            mutation.remove_node_and_transfer_trivia(
                signature_to_remove_wrapper,
                signature_to_extend_wrapper,
            );
        } else {
            // Nothing important to transfer, just remove the signature.
            mutation.remove_node(signature_to_remove_wrapper);
        }

        // As we go through the operations list, we collect what changes we need to make
        // in order to provide the user with better description of the changes.
        for parameter_info in &state.parameters_to_merge {
            match &parameter_info.operation {
                MergeParameterOperation::CombineTypeWithParameter(combine_with_param) => {
                    let parameter_type = parameter_info
                        .parameter
                        .type_annotation()
                        .and_then(|type_annotation| type_annotation.ty().ok())?;
                    let combine_with_type = combine_with_param
                        .type_annotation()
                        .and_then(|type_annotation| type_annotation.ty().ok())?;
                    mutation.replace_node(
                        parameter_type.clone(),
                        AnyTsType::TsUnionType(
                            make::ts_union_type(make::ts_union_type_variant_list(
                                [parameter_type, combine_with_type],
                                [make::token(T![|])
                                    .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])],
                            ))
                            .build(),
                        ),
                    );
                }
                MergeParameterOperation::MakeOptional => {
                    if let Some(optional_parameter) = parameter_info.parameter.as_optional() {
                        mutation.replace_node(parameter_info.parameter.clone(), optional_parameter)
                    }
                }
            }
        }

        let first_parameter_operation = state
            .parameters_to_merge
            .first()
            .map(|info| &info.operation);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                {
                    if matches!(
                        first_parameter_operation,
                        Some(MergeParameterOperation::CombineTypeWithParameter(_))
                    ) {
                        "Combine overloads using a type union."
                    } else if matches!(
                        first_parameter_operation,
                        Some(MergeParameterOperation::MakeOptional)
                    ) {
                        "Combine overloads by making parameters optional."
                    } else {
                        "Remove the unnecessary overload signature."
                    }
                }
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// Represents the information needed to merge overload signatures.
#[derive(Debug)]
pub struct MergeOverloadSignaturesInfo {
    /// List of parameters that need to be merged.
    pub parameters_to_merge: Box<[MergeParameterInfo]>,
    /// The overload signature that will be removed.
    pub signature_to_remove: AnyPotentialTsOverloadSignature,
    /// The overload signature that will be kept
    /// and potentially extended with the merged parameters.
    pub signature_to_extend: AnyPotentialTsOverloadSignature,
}

/// Represents the information about a single parameter that needs to be merged.
#[derive(Debug)]
pub struct MergeParameterInfo {
    /// The parameter that needs to be merged.
    pub parameter: AnyParameter,
    /// The operation to be performed on a merge parameter.
    pub operation: MergeParameterOperation,
}

/// Represents the operation to be performed on a merge parameter.
#[derive(Debug)]
pub enum MergeParameterOperation {
    /// The parameter type should be combined with another parameter type using a union type.
    CombineTypeWithParameter(AnyParameter),
    /// The parameter should be made optional.
    MakeOptional,
}

trait MutationExt {
    /// A utility method to remove a node and transfer its leading trivia to another node.
    fn remove_node_and_transfer_trivia<T>(&mut self, node_to_remove: T, trivia_target_node: T)
    where
        T: AstNode<Language = JsLanguage>;
}

impl MutationExt for BatchMutation<JsLanguage> {
    fn remove_node_and_transfer_trivia<T>(&mut self, node_to_remove: T, trivia_target_node: T)
    where
        T: AstNode<Language = JsLanguage>,
    {
        if let (Some(first_token_to_remove), Some(target_token)) = (
            node_to_remove.syntax().first_token(),
            trivia_target_node.syntax().first_token(),
        ) {
            // If the node to remove has a first token, we transfer its leading trivia to the target node.
            let leading_trivia = first_token_to_remove.leading_trivia();
            if !leading_trivia.is_empty() {
                let mut target_pieces = target_token.leading_trivia().pieces().peekable();
                target_pieces.next_if(|piece| piece.is_newline());
                let updated_target_token = target_token.with_leading_trivia_pieces(
                    chain_trivia_pieces(leading_trivia.pieces(), target_pieces),
                );
                self.replace_token_discard_trivia(target_token, updated_target_token);
            }
        }
        self.remove_node(node_to_remove);
    }
}

trait AnyJsParameterListExt {
    /// Checks if the parameters in this list are assignable to the parameters in the other list.
    /// Returns a list of parameters that needs to be made optional.
    fn try_merge(&self, other: &Self) -> Option<Vec<MergeParameterInfo>>;
}

impl AnyJsParameterListExt for AnyJsParameterList {
    /// Checks if the parameters in this list are assignable to the parameters in the other list.
    /// Returns a list of instructions that need to be applied to make the signatures compatible.
    fn try_merge(&self, other: &Self) -> Option<Vec<MergeParameterInfo>> {
        if other.len() > self.len() {
            // Other list has more parameters, can't be assignable.
            return None;
        }
        let required_self_len = self
            .iter()
            .position(|param| {
                param.is_ok_and(|param| param.is_optional() || param.is_rest_parameter())
            })
            .unwrap_or(self.len());

        if required_self_len > other.len() + 1 {
            // If the current signature has more than one required parameter than the other,
            // we can't merge the signatures. Example:
            // function test(): void;
            // function test(a: number, b: number): void;
            // We can't merge these signatures into `test(a?: number, b?: number): void`,
            // because the original definition says "either no parameters or two parameter",
            // and making parameters optional would allow for one parameter.
            return None;
        }

        let has_equal_parameters_len = other.len() == self.len();

        let mut result = Vec::new();
        let mut found_non_equal_param = false;
        for (self_param, other_param) in self.iter().zip(other.iter()) {
            let (Ok(self_param), Ok(other_param)) = (self_param, other_param) else {
                // If we can't get the parameters, we can't compare them.
                return None;
            };
            match self_param.compare(&other_param) {
                ParameterCompareResult::Equal => {
                    if !self_param.is_optional() && other_param.is_optional() {
                        // If the parameter is not optional in the first list,
                        // but is optional in the second list, we need to make it optional.
                        result.push(MergeParameterInfo {
                            parameter: self_param.clone(),
                            operation: MergeParameterOperation::MakeOptional,
                        });
                    } else if self_param.is_optional() != other_param.is_optional() {
                        return None;
                    }
                }
                ParameterCompareResult::NotEqual => {
                    // We found a pair of parameters that are not equal.
                    // If that's the only pair, we can merge the signatures.
                    // If more parameters are not equal, we can't merge the signatures.
                    if !found_non_equal_param && has_equal_parameters_len {
                        // If parameter types are not equal and optionality is different as well,
                        // we can not merge the signatures.
                        if self_param.is_optional() != other_param.is_optional() {
                            return None;
                        }
                        found_non_equal_param = true;
                        result.push(MergeParameterInfo {
                            parameter: self_param.clone(),
                            operation: MergeParameterOperation::CombineTypeWithParameter(
                                other_param.clone(),
                            ),
                        });
                    } else {
                        return None;
                    }
                }
                ParameterCompareResult::Incompatible => {
                    return None;
                }
            }
        }

        for self_param in self.iter().skip(other.len()) {
            let Ok(self_param) = self_param else {
                // If we can't get the parameter, we can't process it.
                return None;
            };
            if self_param.is_optional() || self_param.is_rest_parameter() {
                // If the parameter is already optional or a rest parameter, we can skip it.
                continue;
            }
            if !self_param.can_be_optional() {
                // If the parameter cannot be made optional, we can't merge the signatures.
                return None;
            }
            result.push(MergeParameterInfo {
                parameter: self_param.clone(),
                operation: MergeParameterOperation::MakeOptional,
            });
        }

        Some(result)
    }
}

declare_node_union! {
    /// Represents any function or method name in JavaScript/TypeScript.
    pub AnyFunctionOrMethodName =
        JsPrivateClassMemberName
        | JsLiteralMemberName
        | JsIdentifierBinding
        | JsMetavariable
        | JsComputedMemberName
        | JsBogusBinding
}

impl From<AnyJsClassMemberName> for AnyFunctionOrMethodName {
    fn from(name: AnyJsClassMemberName) -> Self {
        match name {
            AnyJsClassMemberName::JsPrivateClassMemberName(name) => {
                Self::JsPrivateClassMemberName(name)
            }
            AnyJsClassMemberName::JsLiteralMemberName(name) => Self::JsLiteralMemberName(name),
            AnyJsClassMemberName::JsComputedMemberName(name) => Self::JsComputedMemberName(name),
            AnyJsClassMemberName::JsMetavariable(name) => Self::JsMetavariable(name),
        }
    }
}

impl From<AnyJsObjectMemberName> for AnyFunctionOrMethodName {
    fn from(name: AnyJsObjectMemberName) -> Self {
        match name {
            AnyJsObjectMemberName::JsLiteralMemberName(name) => Self::JsLiteralMemberName(name),
            AnyJsObjectMemberName::JsComputedMemberName(name) => Self::JsComputedMemberName(name),
            AnyJsObjectMemberName::JsMetavariable(name) => Self::JsMetavariable(name),
        }
    }
}

impl From<AnyJsBinding> for AnyFunctionOrMethodName {
    fn from(name: AnyJsBinding) -> Self {
        match name {
            AnyJsBinding::JsIdentifierBinding(name) => Self::JsIdentifierBinding(name),
            AnyJsBinding::JsMetavariable(name) => Self::JsMetavariable(name),
            AnyJsBinding::JsBogusBinding(name) => Self::JsBogusBinding(name),
        }
    }
}

impl AnyFunctionOrMethodName {
    fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            Self::JsComputedMemberName(computed) => computed
                .expression()
                .ok()
                .and_then(|expr| expr.as_static_value()),
            Self::JsLiteralMemberName(literal) => literal.value().ok().map(StaticValue::String),
            Self::JsIdentifierBinding(ident) => ident.name_token().ok().map(StaticValue::String),
            _ => None,
        }
    }
}

trait NameEquals {
    /// Checks if the name of the current node is equal to the name of another node.
    fn is_name_equal(&self, other: &Self) -> bool;
}

impl NameEquals for AnyFunctionOrMethodName {
    fn is_name_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::JsMetavariable(meta1), Self::JsMetavariable(meta2)) => {
                if let (Ok(name1), Ok(name2)) = (meta1.value_token(), meta2.value_token()) {
                    name1.text_trimmed() == name2.text_trimmed()
                } else {
                    false
                }
            }
            (Self::JsPrivateClassMemberName(name1), Self::JsPrivateClassMemberName(name2)) => {
                if let (Ok(id1), Ok(id2)) = (name1.id_token(), name2.id_token()) {
                    id1.text_trimmed() == id2.text_trimmed()
                } else {
                    false
                }
            }
            _ => {
                if let (Some(static1), Some(static2)) =
                    (self.as_static_value(), other.as_static_value())
                {
                    static1.text() == static2.text()
                } else {
                    false
                }
            }
        }
    }
}

impl<T: NameEquals> NameEquals for Option<T> {
    fn is_name_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(self_name), Some(other_name)) => self_name.is_name_equal(other_name),
            (None, None) => true,
            _ => false,
        }
    }
}

declare_node_union! {
    /// Represents any potential overload signature in TypeScript.
    pub AnyPotentialTsOverloadSignature =
        TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration
        | TsMethodSignatureTypeMember
        | TsMethodSignatureClassMember
        | TsConstructorSignatureClassMember
        | TsCallSignatureTypeMember
        | TsConstructSignatureTypeMember
}

impl AnyPotentialTsOverloadSignature {
    /// Returns the name of the overload signature, if it exists.
    fn name(&self) -> Option<AnyFunctionOrMethodName> {
        match self {
            Self::TsDeclareFunctionDeclaration(decl) => decl.id().ok().map(|name| name.into()),
            Self::TsDeclareFunctionExportDefaultDeclaration(decl) => {
                decl.id().map(|name| name.into())
            }
            Self::TsMethodSignatureTypeMember(member) => member.name().ok().map(|name| name.into()),
            Self::TsMethodSignatureClassMember(member) => {
                member.name().ok().map(|name| name.into())
            }
            Self::TsConstructorSignatureClassMember(_)
            | Self::TsConstructSignatureTypeMember(_)
            | Self::TsCallSignatureTypeMember(_) => None,
        }
    }

    /// Checks if the overload signature is a constructor signature.
    /// This can be either a class constructor or a construct signature of interface/type.
    fn is_constructor(&self) -> bool {
        matches!(
            self,
            Self::TsConstructorSignatureClassMember(_) | Self::TsConstructSignatureTypeMember(_)
        )
    }

    /// Checks if the overload signature is static class member.
    fn is_static(&self) -> bool {
        match self {
            Self::TsMethodSignatureClassMember(member) => {
                member.modifiers().iter().any(|modifier| {
                    matches!(modifier, AnyTsMethodSignatureModifier::JsStaticModifier(_))
                })
            }
            _ => false,
        }
    }

    /// Returns the type parameters of the generic overload signature, if it exists.
    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            Self::TsDeclareFunctionDeclaration(decl) => decl.type_parameters(),
            Self::TsDeclareFunctionExportDefaultDeclaration(decl) => decl.type_parameters(),
            Self::TsMethodSignatureTypeMember(member) => member.type_parameters(),
            Self::TsMethodSignatureClassMember(member) => member.type_parameters(),
            Self::TsCallSignatureTypeMember(member) => member.type_parameters(),
            Self::TsConstructSignatureTypeMember(member) => member.type_parameters(),
            Self::TsConstructorSignatureClassMember(_) => None,
        }
    }

    /// Returns the return type of the overload signature, if it exists.
    fn return_type_annotation(&self) -> Option<AnyTsReturnType> {
        // This is the only case where a method has not a return type annotation,
        // but a regular type annotation.
        if let Self::TsConstructSignatureTypeMember(member) = self {
            return member
                .type_annotation()
                .and_then(|annotation| annotation.ty().ok())
                .map(AnyTsReturnType::AnyTsType);
        }

        (match self {
            Self::TsDeclareFunctionDeclaration(decl) => decl.return_type_annotation(),
            Self::TsDeclareFunctionExportDefaultDeclaration(decl) => decl.return_type_annotation(),
            Self::TsMethodSignatureTypeMember(member) => member.return_type_annotation(),
            Self::TsMethodSignatureClassMember(member) => member.return_type_annotation(),
            Self::TsCallSignatureTypeMember(member) => member.return_type_annotation(),
            Self::TsConstructSignatureTypeMember(_)
            | Self::TsConstructorSignatureClassMember(_) => None,
        })
        .and_then(|annotation| annotation.ty().ok())
    }

    /// Returns the parameters of the overload signature, if it exists.
    fn parameters(&self) -> Option<AnyJsParameterList> {
        Some(match self {
            Self::TsDeclareFunctionDeclaration(func) => func.parameters().ok()?.items().into(),
            Self::TsDeclareFunctionExportDefaultDeclaration(func) => {
                func.parameters().ok()?.items().into()
            }
            Self::TsMethodSignatureTypeMember(method) => method.parameters().ok()?.items().into(),
            Self::TsMethodSignatureClassMember(method) => method.parameters().ok()?.items().into(),
            Self::TsCallSignatureTypeMember(call) => call.parameters().ok()?.items().into(),
            Self::TsConstructSignatureTypeMember(member) => {
                member.parameters().ok()?.items().into()
            }
            Self::TsConstructorSignatureClassMember(constr) => {
                constr.parameters().ok()?.parameters().into()
            }
        })
    }

    /// Returns the top syntax node that wraps the overload signature.
    /// This can be the signature itself, an export statement, or a declare statement.
    fn wrapper_syntax(&self) -> JsSyntaxNode {
        match self {
            Self::TsDeclareFunctionDeclaration(function_decl) => {
                if let Some(parent) = function_decl.syntax().parent()
                    && matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_EXPORT | JsSyntaxKind::TS_DECLARE_STATEMENT
                    )
                {
                    return parent;
                }
            }
            Self::TsDeclareFunctionExportDefaultDeclaration(function_decl) => {
                if let Some(parent) = function_decl.syntax().parent()
                    && matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE
                    )
                    && let Some(export) = parent.parent()
                {
                    return export;
                }
            }
            _ => {}
        }
        self.syntax().clone()
    }

    /// Returns the node which can potentially be used to create a new PotentialTsOverloadSignature.
    fn get_sibling_syntax(syntax: JsSyntaxNode) -> Option<JsSyntaxNode> {
        if let Some(export) = JsExport::cast_ref(&syntax) {
            match export.export_clause().ok()? {
                AnyJsExportClause::JsExportDefaultDeclarationClause(default_clause) => {
                    return Some(default_clause.declaration().ok()?.syntax().clone());
                }
                AnyJsExportClause::AnyJsDeclarationClause(
                    AnyJsDeclarationClause::TsDeclareFunctionDeclaration(function),
                ) => {
                    return Some(function.syntax().clone());
                }
                _ => {}
            }
        }
        if let Some(declare_statement) = TsDeclareStatement::cast_ref(&syntax) {
            return Some(declare_statement.declaration().ok()?.syntax().clone());
        }
        Some(syntax)
    }

    /// Returns the previous sibling of the overload signature, if it exists.
    fn prev_sibling(&self) -> Option<JsSyntaxNode> {
        Self::get_sibling_syntax(self.wrapper_syntax().prev_sibling()?)
    }

    /// Returns the next sibling of the overload signature, if it exists.
    fn next_sibling(&self) -> Option<JsSyntaxNode> {
        Self::get_sibling_syntax(self.wrapper_syntax().next_sibling()?)
    }

    /// Returns the wrapper node of the overload signature, if it exists.
    /// This can be the signature itself, an export statement, or a declare statement.
    fn wrapper(&self) -> Option<AnyPotentialTsOverloadSignatureWrapper> {
        AnyPotentialTsOverloadSignatureWrapper::cast_ref(&self.wrapper_syntax())
    }

    /// Returns the range of the overload signature, excluding any comments or whitespace.
    fn overload_range(&self) -> TextRange {
        self.wrapper_syntax().text_trimmed_range()
    }
}

declare_node_union! {
    /// The wrapper node of the overload signature.
    /// This can be the signature itself, an export statement, or a declare statement.
    AnyPotentialTsOverloadSignatureWrapper =
        AnyPotentialTsOverloadSignature
        | JsExport
        | TsDeclareStatement
}

/// Represents the information about overload signatures.
#[derive(Debug)]
struct OverloadInfo {
    /// List of signatures that are part of the overload.
    overload_signatures: Box<[AnyPotentialTsOverloadSignature]>,
}

impl OverloadInfo {
    fn from_overload_signature(signature: &AnyPotentialTsOverloadSignature) -> Option<Self> {
        let name = signature.name();
        let prev_sibling = signature.prev_sibling();
        if let Some(prev_sibling) = prev_sibling
            && let Some(prev_signature) =
                AnyPotentialTsOverloadSignature::cast(prev_sibling.clone())
        {
            // If the previous signature has the same name,
            // return None as it is not the first overload.
            if name.is_name_equal(&prev_signature.name()) {
                return None;
            }
        }
        let mut overload_signatures = vec![signature.clone()];
        let mut current_next_sibling = signature.next_sibling();
        while let Some(next_sibling) = &current_next_sibling {
            if let Some(next_signature) = AnyPotentialTsOverloadSignature::cast_ref(next_sibling) {
                if signature.is_constructor() != next_signature.is_constructor()
                    || signature.is_static() != next_signature.is_static()
                {
                    // If signatures are not both constructors or not both non-constructors,
                    // we stop collecting overload signatures.
                    break;
                }
                if name.is_name_equal(&next_signature.name()) {
                    // If the next signature has the same name,
                    // we add it to the overload signatures and continue collecting.
                    overload_signatures.push(next_signature.clone());
                    current_next_sibling = next_signature.next_sibling();
                    // We only continue collecting overload signatures if we've found a match.
                    continue;
                }
            }
            // We stop collecting in all other cases.
            break;
        }

        // If we have only one signature.
        if overload_signatures.len() == 1 {
            return None;
        }

        Some(Self {
            overload_signatures: overload_signatures.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParameterCompareResult {
    Equal,
    NotEqual,
    Incompatible,
}

trait ParameterExt {
    /// Checks if the parameter is optional.
    fn is_optional(&self) -> bool;
    /// Checks if the parameter is a rest parameter.
    fn is_rest_parameter(&self) -> bool;
    /// Some of the parameters cannot be compared.
    fn can_compare(&self) -> bool;
    /// Compares two parameters and returns the result of the comparison.
    fn compare(&self, other: &Self) -> ParameterCompareResult;
    /// Checks if the parameter can be optional.
    fn can_be_optional(&self) -> bool;
}

impl ParameterExt for AnyParameter {
    fn is_optional(&self) -> bool {
        match self {
            Self::AnyJsParameter(AnyJsParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsFormalParameter(param),
            ))
            | Self::AnyJsConstructorParameter(AnyJsConstructorParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsFormalParameter(param),
            )) => param.question_mark_token().is_some(),
            _ => false,
        }
    }

    fn is_rest_parameter(&self) -> bool {
        matches!(
            self,
            Self::AnyJsParameter(AnyJsParameter::JsRestParameter(_))
                | Self::AnyJsConstructorParameter(AnyJsConstructorParameter::JsRestParameter(_))
        )
    }

    fn can_compare(&self) -> bool {
        !matches!(
            self,
            Self::AnyJsParameter(AnyJsParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsMetavariable(_) | AnyJsFormalParameter::JsBogusParameter(_)
            )) | Self::AnyJsConstructorParameter(AnyJsConstructorParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsMetavariable(_) | AnyJsFormalParameter::JsBogusParameter(_)
            ))
        )
    }

    fn compare(&self, other: &Self) -> ParameterCompareResult {
        if !self.can_compare() || !other.can_compare() {
            return ParameterCompareResult::Incompatible;
        }

        if self.is_rest_parameter() != other.is_rest_parameter() {
            // If one is a rest parameter and the other is not, they are incompatible.
            return ParameterCompareResult::Incompatible;
        }

        if self
            .type_annotation()
            .is_type_equal(&other.type_annotation())
        {
            ParameterCompareResult::Equal
        } else if self.is_rest_parameter() {
            // Cannot use type union for rest parameters.
            // I.e. ...args: string[] | number[] is not valid.
            ParameterCompareResult::Incompatible
        } else {
            ParameterCompareResult::NotEqual
        }
    }

    fn can_be_optional(&self) -> bool {
        matches!(
            self,
            Self::AnyJsParameter(AnyJsParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsFormalParameter(_),
            )) | Self::AnyJsConstructorParameter(AnyJsConstructorParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsFormalParameter(_),
            ))
        )
    }
}

trait AsOptionalParameter {
    /// Attempts to merge the parameter with another and returns the new parameter.
    fn as_optional(&self) -> Option<Self>
    where
        Self: Sized;
}

impl AsOptionalParameter for AnyParameter {
    fn as_optional(&self) -> Option<Self> {
        match self {
            Self::AnyJsParameter(param) => param.as_optional().map(Self::AnyJsParameter),
            Self::AnyJsConstructorParameter(param) => {
                param.as_optional().map(Self::AnyJsConstructorParameter)
            }
        }
    }
}

impl AsOptionalParameter for AnyJsParameter {
    fn as_optional(&self) -> Option<Self> {
        match self {
            Self::AnyJsFormalParameter(param) => {
                param.as_optional().map(Self::AnyJsFormalParameter)
            }
            _ => None, // Other parameter types cannot be merged.
        }
    }
}

impl AsOptionalParameter for AnyJsConstructorParameter {
    fn as_optional(&self) -> Option<Self> {
        match self {
            Self::AnyJsFormalParameter(param) => {
                param.as_optional().map(Self::AnyJsFormalParameter)
            }
            _ => None, // Other parameter types cannot be merged.
                       // TsPropertyParameter cannot be part of overload signatures.
        }
    }
}

impl AsOptionalParameter for AnyJsFormalParameter {
    fn as_optional(&self) -> Option<Self> {
        match self {
            Self::JsFormalParameter(param) => Some(Self::JsFormalParameter(
                param
                    .clone()
                    .with_question_mark_token(Some(make::token(T![?]))),
            )),
            _ => None,
        }
    }
}

/// Trait for comparing types to determine if they are equal.
/// This can potentially be instead generated in the same process as the AST node generation.
/// Basically it compares only AST ignoring node metadata like comments, whitespace, etc.
trait TypeEquals {
    fn is_type_equal(&self, other: &Self) -> bool;
}

/// This macro implements the `TypeEquals` trait for enum types that have multiple variants.
/// Each variant is compared using the `is_type_equal` method of the contained type.
macro_rules! enum_type_equals {
    ( $($enum_type:ident : [ $($element:ident),+ $(,)?] ),+ $(,)? ) => {
        $(
            impl TypeEquals for biome_js_syntax::$enum_type {
                fn is_type_equal(&self, other: &Self) -> bool {
                    match (&self, &other) {
                        $((Self::$element(a), Self::$element(b)) => a.is_type_equal(b),)+
                        _ => false,
                    }
                }
            }
        )+
    };
}

enum_type_equals! {
    AnyJsArrayBindingPatternElement: [
        JsArrayBindingPatternElement,
        JsArrayBindingPatternRestElement,
        JsArrayHole
    ],
    AnyJsBinding: [JsBogusBinding, JsIdentifierBinding, JsMetavariable],
    AnyJsBindingPattern: [AnyJsBinding, JsArrayBindingPattern, JsObjectBindingPattern],
    AnyJsFormalParameter: [JsFormalParameter, JsBogusParameter, JsMetavariable],
    AnyJsImportAssertionEntry: [JsImportAssertionEntry, JsBogusImportAssertionEntry],
    AnyJsObjectBindingPatternMember: [
        JsBogusBinding,
        JsMetavariable,
        JsObjectBindingPatternProperty,
        JsObjectBindingPatternRest,
        JsObjectBindingPatternShorthandProperty
    ],
    AnyJsObjectMemberName: [JsComputedMemberName, JsLiteralMemberName, JsMetavariable],
    AnyJsParameter: [AnyJsFormalParameter, JsRestParameter, TsThisParameter],
    AnyTsName: [JsReferenceIdentifier, TsQualifiedName],
    AnyTsReturnType: [AnyTsType, TsAssertsReturnType, TsPredicateReturnType],
    AnyTsTemplateElement: [TsTemplateChunkElement, TsTemplateElement],
    AnyTsTupleTypeElement: [
        AnyTsType,
        TsNamedTupleTypeElement,
        TsOptionalTupleTypeElement,
        TsRestTupleTypeElement,
    ],
    AnyTsType: [
        JsMetavariable,
        TsAnyType,
        TsArrayType,
        TsBigintLiteralType,
        TsBigintType,
        TsBogusType,
        TsBooleanLiteralType,
        TsBooleanType,
        TsConditionalType,
        TsConstructorType,
        TsFunctionType,
        TsImportType,
        TsIndexedAccessType,
        TsInferType,
        TsIntersectionType,
        TsMappedType,
        TsNeverType,
        TsNonPrimitiveType,
        TsNullLiteralType,
        TsNumberLiteralType,
        TsNumberType,
        TsObjectType,
        TsParenthesizedType,
        TsReferenceType,
        TsStringLiteralType,
        TsStringType,
        TsSymbolType,
        TsTemplateLiteralType,
        TsThisType,
        TsTupleType,
        TsTypeOperatorType,
        TsTypeofType,
        TsUndefinedType,
        TsUnionType,
        TsUnknownType,
        TsVoidType,
    ],
    AnyTsTypeMember: [
        JsBogusMember,
        TsCallSignatureTypeMember,
        TsConstructSignatureTypeMember,
        TsGetterSignatureTypeMember,
        TsIndexSignatureTypeMember,
        TsMethodSignatureTypeMember,
        TsPropertySignatureTypeMember,
        TsSetterSignatureTypeMember,
    ],
    AnyTsTypeParameterModifier: [TsConstModifier, TsInModifier, TsOutModifier],
    AnyTsTypePredicateParameterName: [JsReferenceIdentifier, TsThisType],
}

// This macro implements the `TypeEquals` trait for types that are always equal to each other.
// No matter the content, they are considered equal.
macro_rules! always_type_equals [
    ( $($type:ident),+ $(,)? ) => {
        $(
            impl TypeEquals for biome_js_syntax::$type {
                fn is_type_equal(&self, _other: &Self) -> bool {
                    true // These types are always equal to each other.
                }
            }
        )+
    };
];

always_type_equals![
    TsAnyType,
    TsBigintType,
    TsBooleanType,
    TsNeverType,
    TsNonPrimitiveType,
    TsNullLiteralType,
    TsNumberType,
    TsStringType,
    TsSymbolType,
    TsUndefinedType,
    TsUnknownType,
    TsVoidType,
    TsThisType,
    JsArrayHole,
    TsConstModifier,
    TsInModifier,
    TsOutModifier,
];

/// This macro implements the `TypeEquals` trait for types that have lists of types.
macro_rules! list_type_equals [
    ( $($type:ident),+ $(,)? ) => {
        use biome_rowan::AstNodeList;
        $(
            impl TypeEquals for biome_js_syntax::$type {
                fn is_type_equal(&self, other: &Self) -> bool {
                    if self.len() != other.len() {
                        return false; // Different number of types
                    }
                    self
                        .iter()
                        .zip(other.iter())
                        .all(|(ty1, ty2)| ty1.is_type_equal(&ty2))
                }
            }
        )+
    };
];

list_type_equals![
    TsUnionTypeVariantList,
    TsIntersectionTypeElementList,
    TsTypeArgumentList,
    TsTupleTypeElementList,
    TsTemplateElementList,
    JsParameterList,
    JsImportAssertionEntryList,
    TsTypeMemberList,
    JsArrayBindingPatternElementList,
    JsObjectBindingPatternPropertyList,
    TsTypeParameterList,
    TsTypeParameterModifierList,
];

/// This macro implements the `TypeEquals` trait for types that have multiple properties.
/// Compares each property of the type with the corresponding property of the other type.
macro_rules! compare_type_equals {
    ( $( $node_type:ident : [ $($prop:ident),+ $(,)? ] ),+ $(,)? ) => {
        $(
            impl TypeEquals for biome_js_syntax::$node_type {
                fn is_type_equal(&self, other: &Self) -> bool {
                    $(
                        self.$prop().is_type_equal(&other.$prop())
                    )&&+
                }
            }
        )+
    }
}

compare_type_equals! {
    JsArrayBindingPattern: [ elements ],
    JsArrayBindingPatternElement: [ pattern ],
    JsArrayBindingPatternRestElement: [ pattern ],
    JsComputedMemberName: [ expression ],
    JsFormalParameter: [ binding, question_mark_token, type_annotation ],
    JsIdentifierBinding: [ name_token ],
    JsImportAssertionEntry: [ key, value_token ],
    JsLiteralMemberName: [ value ],
    JsMetavariable: [ value_token ],
    JsName: [ value_token ],
    JsObjectBindingPattern: [ properties ],
    JsObjectBindingPatternProperty: [ member, pattern ],
    JsObjectBindingPatternRest: [ binding ],
    JsObjectBindingPatternShorthandProperty: [ identifier ],
    JsParameters: [ items ],
    JsReferenceIdentifier: [ value_token ],
    JsRestParameter: [ type_annotation ],
    TsArrayType: [ element_type ],
    TsAssertsReturnType: [ parameter_name ],
    TsBigintLiteralType: [ literal_token ],
    TsBooleanLiteralType: [ literal ],
    TsCallSignatureTypeMember: [ type_parameters, parameters, return_type_annotation ],
    TsConditionalType: [ check_type, extends_type, true_type, false_type ],
    TsConstructorType: [ type_parameters, parameters, return_type ],
    TsConstructSignatureTypeMember: [ type_parameters, parameters, type_annotation ],
    TsDefaultTypeClause: [ ty ],
    TsFunctionType: [ type_parameters, parameters, return_type ],
    TsGetterSignatureTypeMember: [ name, type_annotation ],
    TsImportType: [ arguments, type_arguments, qualifier_clause ],
    TsImportTypeArguments: [ argument, ts_import_type_assertion_block ],
    TsImportTypeAssertion: [ assertions ],
    TsImportTypeAssertionBlock: [ type_assertion ],
    TsImportTypeQualifier: [ right ],
    TsIndexedAccessType: [ object_type, index_type ],
    TsIndexSignatureParameter: [ type_annotation ],
    TsIndexSignatureTypeMember: [ readonly_token, parameter, type_annotation ],
    TsInferType: [ name, constraint ],
    TsIntersectionType: [ types ],
    TsMappedType: [ keys_type, as_clause, mapped_type, readonly_modifier, optional_modifier ],
    TsMappedTypeAsClause: [ ty ],
    TsMappedTypeOptionalModifierClause: [ operator_token ],
    TsMappedTypeReadonlyModifierClause: [ operator_token ],
    TsMethodSignatureTypeMember: [
        name, optional_token, type_parameters, parameters, return_type_annotation
    ],
    TsNamedTupleTypeElement: [ ty ],
    TsNumberLiteralType: [ literal_token ],
    TsObjectType: [ members ],
    TsOptionalTupleTypeElement: [ ty ],
    TsParenthesizedType: [ ty ],
    TsPredicateReturnType: [ parameter_name, ty ],
    TsPropertySignatureTypeMember: [ readonly_token, name, optional_token, type_annotation ],
    TsQualifiedName: [ left, right ],
    TsReferenceType: [ name, type_arguments ],
    TsRestTupleTypeElement: [ ty ],
    TsReturnTypeAnnotation: [ ty ],
    TsSetterSignatureTypeMember: [ name, parameter ],
    TsStringLiteralType: [ literal_token ],
    TsTemplateChunkElement: [ template_chunk_token ],
    TsTemplateElement: [ ty ],
    TsTemplateLiteralType: [ elements ],
    TsThisParameter: [ type_annotation ],
    TsTupleType: [ elements ],
    TsTypeAnnotation: [ ty ],
    TsTypeArguments: [ ts_type_argument_list ],
    TsTypeConstraintClause: [ ty ],
    TsTypeofType: [ expression_name ],
    TsTypeOperatorType: [ operator_token, ty ],
    TsTypeParameter: [ modifiers, name, constraint, default ],
    TsTypeParameterName: [ ident_token ],
    TsTypeParameters: [ items ],
    TsUnionType: [ types ],
}

/// This macro implements the `TypeEquals` trait for bogus types
/// by comparing their string representations.
macro_rules! bogus_types [
    ( $($bogus:ident),+ $(,)? ) => {
        $(
            impl TypeEquals for biome_js_syntax::$bogus {
                fn is_type_equal(&self, other: &Self) -> bool {
                    self.to_trimmed_string() == other.to_trimmed_string()
                }
            }
        )+
    };
];

bogus_types![
    JsBogusBinding,
    JsBogusImportAssertionEntry,
    JsBogusMember,
    JsBogusParameter,
    TsBogusType,
];

impl TypeEquals for biome_js_syntax::AnyJsExpression {
    fn is_type_equal(&self, other: &Self) -> bool {
        // We only compare literal expressions, the rest would be too complex.
        if let (Self::AnyJsLiteralExpression(a), Self::AnyJsLiteralExpression(b)) = (self, other)
            && let (Ok(a), Ok(b)) = (a.value_token(), b.value_token())
        {
            return a.text_trimmed() == b.text_trimmed();
        }
        false
    }
}

impl TypeEquals for biome_js_syntax::JsSyntaxToken {
    fn is_type_equal(&self, other: &Self) -> bool {
        self.text_trimmed() == other.text_trimmed()
    }
}

impl<T: TypeEquals> TypeEquals for Option<T> {
    fn is_type_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.is_type_equal(b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: TypeEquals> TypeEquals for SyntaxResult<T> {
    fn is_type_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => a.is_type_equal(b),
            (Err(_), Err(_)) => true, // Both are errors, we consider them equal.
            _ => false,               // One is Ok and the other is Err.
        }
    }
}
