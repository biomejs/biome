use std::any::TypeId;

use rome_rowan::{Language, SyntaxKindSet, TextRange};

use crate::{registry::Phase, services::FromServices, Phases, ServiceBag, Visitor};

/// Trait implemented for types that lint rules can query in order to emit diagnostics or code actions.
pub trait Queryable: Sized {
    type Input: QueryMatch;
    type Output;

    type Language: Language;
    type Services: FromServices + Phase;

    /// Registers one or more [Visitor] that will emit `Self::Input` query
    /// matches during the analyzer run
    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        root: &<Self::Language as Language>::Root,
    );

    /// Returns the type of query matches this [Queryable] expects as inputs
    ///
    /// Unless your custom queryable needs to match on a specific
    /// [SyntaxKindSet], you should not override the default implementation of
    /// this method
    fn key() -> QueryKey<Self::Language> {
        QueryKey::TypeId(TypeId::of::<Self::Input>())
    }

    /// Unwrap an instance of `Self` from a [QueryMatch].
    ///
    /// ## Panics
    ///
    /// If the [QueryMatch] variant of `query` doesn't match `Self::KEY`
    fn unwrap_match(services: &ServiceBag, query: &Self::Input) -> Self::Output;
}

/// This trait is implemented on all types that supports the registration of [Visitor]
pub trait AddVisitor<L: Language> {
    /// Registers a [Visitor] for a given `phase`
    fn add_visitor<F, V>(&mut self, phase: Phases, visitor: F)
    where
        F: FnOnce() -> V,
        V: Visitor<Language = L> + 'static;
}

/// Marker trait implemented for all the types analyzer visitors may emit
pub trait QueryMatch: 'static {
    fn text_range(&self) -> TextRange;
}

/// Represents which type a given [Queryable] type can match, either a specific
/// subset of syntax node kinds or any generic type
pub enum QueryKey<L: Language> {
    Syntax(SyntaxKindSet<L>),
    TypeId(TypeId),
}
