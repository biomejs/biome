//! The primitive kinds of fresh literal types, which TypeScript widens where
//! a value initializes a mutable location.
//!
//! Collection wraps the value of each mutable location in a
//! [`TypeofWidenedLiteralExpression`]. Resolution in the module graph then
//! collects the kinds of the fresh literal types of the value, following
//! names into other modules, and widens the literal types of those kinds.

use enumflags2::BitFlags;

use crate::interned_types::TypeData;
use crate::type_relation::widened_literal;
use crate::{Literal, TypeDb};

#[cfg(doc)]
use crate::TypeofWidenedLiteralExpression;

/// A primitive type that has literal types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[enumflags2::bitflags]
#[repr(u8)]
pub enum PrimitiveLiteralKind {
    BigInt = 1 << 0,
    Boolean = 1 << 1,
    Number = 1 << 2,
    String = 1 << 3,
}

/// The primitive kinds of which a value has a fresh literal type: the type
/// of a literal expression such as `1n`, `true`, `1`, or `"a"`.
///
/// Widening every literal type of such a kind to its primitive type gives
/// the type that TypeScript gives by widening only the fresh ones, because
/// the primitive type absorbs the other literal types of its kind, as
/// `string` absorbs `"a"` in `"a" | string`. It is wider only for a literal
/// type that is both fresh and declared, which TypeScript keeps. All kinds
/// stand for a value whose fresh literal types aren't known. Widening all of
/// them is never narrower than widening only the fresh ones.
pub type FreshLiteralKinds = BitFlags<PrimitiveLiteralKind>;

impl PrimitiveLiteralKind {
    /// Returns the kind of `literal`, the raw type of a literal expression.
    ///
    /// Returns `None` for an object or regular expression literal, because it
    /// has no primitive type, and for a template literal type, because only a
    /// type annotation has one.
    pub fn from_literal_expression(literal: &Literal) -> Option<Self> {
        match literal {
            Literal::BigInt(_) => Some(Self::BigInt),
            Literal::Boolean(_) => Some(Self::Boolean),
            Literal::Number(_) => Some(Self::Number),
            Literal::String(_) => Some(Self::String),
            Literal::Object(_) | Literal::RegExp(_) | Literal::Template(_) => None,
        }
    }

    /// Returns the kind of the primitive type `ty`, or `None` when `ty` is
    /// another type.
    fn from_primitive(ty: TypeData) -> Option<Self> {
        match ty {
            TypeData::BigInt => Some(Self::BigInt),
            TypeData::Boolean => Some(Self::Boolean),
            TypeData::Number => Some(Self::Number),
            TypeData::String => Some(Self::String),
            _ => None,
        }
    }
}

/// Returns the primitive type of the literal type `ty` when `kinds` contains
/// its kind, or `None` otherwise. A template literal type widens with the
/// `string` kind.
pub fn widen_literal_of_kinds<'db>(
    db: &'db dyn TypeDb,
    kinds: FreshLiteralKinds,
    ty: TypeData<'db>,
) -> Option<TypeData<'db>> {
    let primitive = widened_literal(db, ty)?;
    let kind = PrimitiveLiteralKind::from_primitive(primitive)?;
    kinds.contains(kind).then_some(primitive)
}
