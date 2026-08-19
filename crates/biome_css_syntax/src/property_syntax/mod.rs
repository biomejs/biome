mod data;
mod matcher;
mod parser;
mod serializer;

pub use data::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxErrorKind,
    PropertySyntaxMultiplier, PropertySyntaxParseDiagnostic, PropertySyntaxResult,
    PropertySyntaxType,
};
pub use parser::encode;
pub use serializer::decode;
