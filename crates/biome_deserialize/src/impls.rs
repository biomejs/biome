use crate::{
    diagnostics::ExpectedType, Deserializable, DeserializableValue, DeserializationDiagnostic,
    DeserializationVisitor,
};
use biome_rowan::{TextRange, TokenText};
use indexmap::IndexSet;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    path::PathBuf,
};

/// Implementation of [Deserializable] for common data structures.

/// A String representation of a number (integer, float).
/// The format should be parsable by Rust numeric types.
#[derive(Eq, PartialEq, Clone)]
pub struct TokenNumber(pub(crate) TokenText);
impl TokenNumber {
    pub fn text(&self) -> &str {
        self.0.text()
    }
}
impl Deserializable for TokenNumber {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = TokenNumber;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::NUMBER;
            fn visit_number(
                self,
                value: TokenNumber,
                _range: TextRange,
                _diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}

impl Deserializable for () {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            ExpectedType::empty(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for bool {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = bool;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::BOOL;
            fn visit_bool(
                self,
                value: bool,
                _range: TextRange,
                _diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}

impl Deserializable for f32 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new("The number should be a float representable on 32 bits")
                .with_range(range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for f64 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new("The number should be a float representable on 64 bits")
                .with_range(range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for i8 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for i16 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for i32 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for isize {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for i64 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for u8 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for u16 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for u32 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for usize {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for u64 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new_out_of_bound_integer(Self::MIN, Self::MAX, range);
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for NonZeroU8 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic = DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            range,
        );
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for NonZeroU16 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic = DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            range,
        );
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for NonZeroU32 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic = DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            range,
        );
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for NonZeroUsize {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic = DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            range,
        );
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for NonZeroU64 {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenNumber::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            return Some(value);
        }
        let diagnostic = DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            range,
        );
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for TokenText {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = TokenText;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::STR;
            fn visit_str(
                self,
                value: TokenText,
                _range: TextRange,
                _diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}

impl Deserializable for String {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        TokenText::deserialize(value, diagnostics).map(|value| value.to_string())
    }
}

impl Deserializable for PathBuf {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        String::deserialize(value, diagnostics).map(PathBuf::from)
    }
}

impl<T: Deserializable> Deserializable for Vec<T> {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = Vec<T>;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = impl DeserializableValue>,
                _range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(value, diagnostics))
                        .collect(),
                )
            }
        }
        value.deserialize(Visitor(PhantomData), diagnostics)
    }
}

impl<T: Deserializable + Eq + Hash, S: BuildHasher + Default> Deserializable for HashSet<T, S> {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T, S>(PhantomData<(T, S)>);
        impl<T: Deserializable + Eq + Hash, S: BuildHasher + Default> DeserializationVisitor
            for Visitor<T, S>
        {
            type Output = HashSet<T, S>;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = impl DeserializableValue>,
                _range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(value, diagnostics))
                        .collect(),
                )
            }
        }
        value.deserialize(Visitor(PhantomData), diagnostics)
    }
}

impl<T: Hash + Eq + Deserializable> Deserializable for IndexSet<T> {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Hash + Eq + Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = IndexSet<T>;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = impl DeserializableValue>,
                _range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(value, diagnostics))
                        .collect(),
                )
            }
        }
        value.deserialize(Visitor(PhantomData), diagnostics)
    }
}

impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher> Deserializable
    for HashMap<K, V, S>
{
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<K, V, S>(PhantomData<(K, V, S)>);
        impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher>
            DeserializationVisitor for Visitor<K, V, S>
        {
            type Output = HashMap<K, V, S>;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                _range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key = Deserializable::deserialize(key, diagnostics);
                    let value = Deserializable::deserialize(value, diagnostics);
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(Visitor(PhantomData), diagnostics)
    }
}

impl<K: Ord + Deserializable, V: Deserializable> Deserializable for BTreeMap<K, V> {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<K, V>(PhantomData<(K, V)>);
        impl<K: Ord + Deserializable, V: Deserializable> DeserializationVisitor for Visitor<K, V> {
            type Output = BTreeMap<K, V>;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                _range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key = Deserializable::deserialize(key, diagnostics);
                    let value = Deserializable::deserialize(value, diagnostics);
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(Visitor(PhantomData), diagnostics)
    }
}
