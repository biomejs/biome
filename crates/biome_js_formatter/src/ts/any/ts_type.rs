//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsType;
impl FormatRule<AnyTsType> for FormatAnyTsType {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsType, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsType::JsMetavariable(node) => node.format().fmt(f),
            AnyTsType::TsAnyType(node) => node.format().fmt(f),
            AnyTsType::TsArrayType(node) => node.format().fmt(f),
            AnyTsType::TsBigintLiteralType(node) => node.format().fmt(f),
            AnyTsType::TsBigintType(node) => node.format().fmt(f),
            AnyTsType::TsBogusType(node) => node.format().fmt(f),
            AnyTsType::TsBooleanLiteralType(node) => node.format().fmt(f),
            AnyTsType::TsBooleanType(node) => node.format().fmt(f),
            AnyTsType::TsConditionalType(node) => node.format().fmt(f),
            AnyTsType::TsConstructorType(node) => node.format().fmt(f),
            AnyTsType::TsFunctionType(node) => node.format().fmt(f),
            AnyTsType::TsImportType(node) => node.format().fmt(f),
            AnyTsType::TsIndexedAccessType(node) => node.format().fmt(f),
            AnyTsType::TsInferType(node) => node.format().fmt(f),
            AnyTsType::TsIntersectionType(node) => node.format().fmt(f),
            AnyTsType::TsMappedType(node) => node.format().fmt(f),
            AnyTsType::TsNeverType(node) => node.format().fmt(f),
            AnyTsType::TsNonPrimitiveType(node) => node.format().fmt(f),
            AnyTsType::TsNullLiteralType(node) => node.format().fmt(f),
            AnyTsType::TsNumberLiteralType(node) => node.format().fmt(f),
            AnyTsType::TsNumberType(node) => node.format().fmt(f),
            AnyTsType::TsObjectType(node) => node.format().fmt(f),
            AnyTsType::TsParenthesizedType(node) => node.format().fmt(f),
            AnyTsType::TsReferenceType(node) => node.format().fmt(f),
            AnyTsType::TsStringLiteralType(node) => node.format().fmt(f),
            AnyTsType::TsStringType(node) => node.format().fmt(f),
            AnyTsType::TsSymbolType(node) => node.format().fmt(f),
            AnyTsType::TsTemplateLiteralType(node) => node.format().fmt(f),
            AnyTsType::TsThisType(node) => node.format().fmt(f),
            AnyTsType::TsTupleType(node) => node.format().fmt(f),
            AnyTsType::TsTypeOperatorType(node) => node.format().fmt(f),
            AnyTsType::TsTypeofType(node) => node.format().fmt(f),
            AnyTsType::TsUndefinedType(node) => node.format().fmt(f),
            AnyTsType::TsUnionType(node) => node.format().fmt(f),
            AnyTsType::TsUnknownType(node) => node.format().fmt(f),
            AnyTsType::TsVoidType(node) => node.format().fmt(f),
        }
    }
}
