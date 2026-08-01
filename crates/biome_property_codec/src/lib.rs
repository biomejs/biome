mod data;
mod decoder;
mod encoder;

pub use data::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxDiagnostic,
    PropertySyntaxErrorKind, PropertySyntaxMultiplier, PropertySyntaxParseDiagnostic,
    PropertySyntaxResult, PropertySyntaxType,
};
pub use decoder::decode;
pub use encoder::encode;
