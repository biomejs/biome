use super::super::{
    Sealed, TypeInferenceCodeReference, TypeInferenceRequest, TypeInferenceRequestContext,
    TypeInferenceRequestMetadata, TypeInferenceRequestOrigin,
};
use biome_js_type_info::interned_types::ConditionalType;
use biome_rowan::{Text, TextRange};

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum CaseLiteral {
    String(Text),
    Number(u64),
    Boolean(bool),
    Null,
}

/// Attempts to classify an expression without resolving complete local type
/// tables. A missing result falls back to regular expression inference.
pub struct ConditionalTypeRequest {
    module: crate::ModuleInfo,
    expression: TextRange,
}

impl ConditionalTypeRequest {
    /// Uses `expression` as both the query input and profile origin.
    pub const fn new(module: crate::ModuleInfo, expression: TextRange) -> Self {
        Self { module, expression }
    }
}

impl Sealed for ConditionalTypeRequest {}

impl TypeInferenceRequestMetadata for ConditionalTypeRequest {
    const ID: &'static str = "request.conditional-type";
    const LABEL: &'static str = "Conditional expression type";
}

impl<'db> TypeInferenceRequest<'db> for ConditionalTypeRequest {
    type Output = Option<ConditionalType>;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "ConditionalTypeRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.conditional_type(self.module, self.expression)
    }
}

/// Attempts to determine whether an expression can equal a literal used by a
/// switch case without resolving complete local type tables.
pub struct CaseLiteralRequest {
    module: crate::ModuleInfo,
    expression: TextRange,
    literal: CaseLiteral,
}

impl CaseLiteralRequest {
    /// Uses `expression` as both the query input and profile origin.
    pub const fn new(
        module: crate::ModuleInfo,
        expression: TextRange,
        literal: CaseLiteral,
    ) -> Self {
        Self {
            module,
            expression,
            literal,
        }
    }
}

impl Sealed for CaseLiteralRequest {}

impl TypeInferenceRequestMetadata for CaseLiteralRequest {
    const ID: &'static str = "request.case-literal";
    const LABEL: &'static str = "Case literal compatibility";
}

impl<'db> TypeInferenceRequest<'db> for CaseLiteralRequest {
    type Output = Option<bool>;

    const IMPLEMENTATION: TypeInferenceCodeReference =
        TypeInferenceCodeReference::new(file!(), line!(), "CaseLiteralRequest::execute");

    fn origin(&self) -> TypeInferenceRequestOrigin {
        TypeInferenceRequestOrigin::new(self.module, self.expression)
    }

    fn execute(self, context: &TypeInferenceRequestContext<'db>) -> Self::Output {
        context.case_literal(self.module, self.expression, self.literal)
    }
}
