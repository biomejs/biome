//! Recognizes reads of enum members, whose TypeScript type is the enum.

use biome_rowan::TextRange;

use crate::ModuleInfo;

use super::super::{
    Sealed, TypeInferenceClassification, TypeInferenceCodeReference, TypeInferenceRequest,
    TypeInferenceRequestContext, TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};

/// Classifies whether an expression reads a member of an enum through an
/// identifier that names the enum, as `E.A` and `E["A"]` do, also inside
/// parentheses.
///
/// Biome gives an enum member the literal type of its value, where
/// TypeScript gives it the type of the enum. An analyzer that needs to tell
/// the two apart uses this request. It reads syntax, semantic models, and
/// module exports, without inferring any type.
///
/// [`TypeInferenceClassification::Match`] means that the identifier binds to
/// an enum declaration of the module, or to an import that resolves to one,
/// and that the enum declares the member with an identifier or a string
/// literal as its name. [`TypeInferenceClassification::NoMatch`] covers
/// every other expression, including `E?.A` and a member read through a
/// variable that holds the enum. An import that doesn't resolve to a single
/// declaration, disabled inference, and a range that no expression has
/// produce [`TypeInferenceClassification::Indeterminate`].
///
/// `Color.Red` matches, and `Palette.Red` doesn't:
///
/// ```ts
/// enum Color { Red = "red" }
/// const Palette = { Red: "red" };
/// Color.Red;
/// Palette.Red;
/// ```
pub struct EnumMemberClassificationRequest {
    module: ModuleInfo,
    expression: TextRange,
}

impl EnumMemberClassificationRequest {
    /// Uses `expression` as both the classification input and profile origin.
    pub const fn new(module: ModuleInfo, expression: TextRange) -> Self {
        Self { module, expression }
    }
}

impl Sealed for EnumMemberClassificationRequest {}

impl TypeInferenceRequestMetadata for EnumMemberClassificationRequest {
    const ID: &'static str = "request.enum-member-classification";
    const LABEL: &'static str = "Enum member classification";
}

impl<'db> TypeInferenceRequest<'db> for EnumMemberClassificationRequest {
    type Output = TypeInferenceClassification;

    const IMPLEMENTATION: TypeInferenceCodeReference = TypeInferenceCodeReference::new(
        file!(),
        line!(),
        "EnumMemberClassificationRequest::execute",
    );

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.classify_expression_as_enum_member(self.module, self.expression)
    }
}
