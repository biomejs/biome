use crate::prelude::*;
use biome_formatter::CstFormatContext;
use biome_js_syntax::AnyTsType;
use biome_rowan::AstSeparatedList;

/// Utility function that checks if the current type is object like type
/// ```ts
///     type A = {};
///     type B = {
///         [key in A]: number;
///     };
/// ```
pub(crate) fn is_object_like_type(ty: &AnyTsType) -> bool {
    matches!(ty, AnyTsType::TsMappedType(_) | AnyTsType::TsObjectType(_))
}

/// Utility function that checks if the current type is can categorized as "simple"
pub(crate) fn is_simple_type(ty: &AnyTsType) -> bool {
    if matches!(
        ty,
        AnyTsType::TsAnyType(_)
            | AnyTsType::TsNullLiteralType(_)
            | AnyTsType::TsThisType(_)
            | AnyTsType::TsVoidType(_)
            | AnyTsType::TsNumberType(_)
            | AnyTsType::TsNumberLiteralType(_)
            | AnyTsType::TsBooleanType(_)
            | AnyTsType::TsBooleanLiteralType(_)
            | AnyTsType::TsBigintType(_)
            | AnyTsType::TsBigintLiteralType(_)
            | AnyTsType::TsStringType(_)
            | AnyTsType::TsStringLiteralType(_)
            | AnyTsType::TsSymbolType(_)
            | AnyTsType::TsTemplateLiteralType(_)
            | AnyTsType::TsNeverType(_)
            | AnyTsType::TsNonPrimitiveType(_)
            | AnyTsType::TsUndefinedType(_)
            | AnyTsType::TsUnknownType(_)
    ) {
        return true;
    }

    if let AnyTsType::TsReferenceType(reference) = ty {
        return reference.type_arguments().is_none();
    }

    false
}

/// Logic ported from [prettier], function `shouldHugType`
///
/// [prettier]: https://github.com/prettier/prettier/blob/main/src/language-js/print/type-annotation.js#L27-L56
pub(crate) fn should_hug_type(ty: &AnyTsType, f: &Formatter<JsFormatContext>) -> bool {
    let comments = f.context().comments();

    if is_simple_type(ty) || is_object_like_type(ty) {
        return true;
    }

    // Checking for unions where all types but one are "void types", so things like `TypeName | null | void`
    if let AnyTsType::TsUnionType(union_type) = ty {
        let mut iter = union_type.types().iter();

        let has_object_type = iter.any(|ty| {
            matches!(
                ty,
                Ok(AnyTsType::TsObjectType(_) | AnyTsType::TsReferenceType(_))
            )
        });

        let successful = union_type
            .types()
            .iter()
            .filter_map(|node: Result<AnyTsType, biome_rowan::SyntaxError>| node.ok());

        let comments = union_type
            .types()
            .iter()
            .filter_map(|node: Result<AnyTsType, biome_rowan::SyntaxError>| node.ok())
            .filter(|node| comments.has_comments(node.syntax()))
            .count();

        let void_count = union_type
            .types()
            .iter()
            .filter(|node| {
                matches!(
                    node,
                    Ok(AnyTsType::TsVoidType(_) | AnyTsType::TsNullLiteralType(_))
                )
            })
            .count();

        union_type.types().len() - 1 == void_count && has_object_type && comments == 0
            || successful.count() == 1
    } else {
        false
    }
}
