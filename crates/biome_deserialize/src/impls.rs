//! Implementations of [Deserializable] for common data structures.
//!
//! Tests of these implementations are available in [biome_deserialize::json::tests] module.
use crate::{
    diagnostics::VisitableType, Deserializable, DeserializableValue, DeserializationDiagnostic,
    DeserializationVisitor,
};
use biome_rowan::{TextRange, TokenText};
use indexmap::{IndexMap, IndexSet};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    ops::Deref,
    path::PathBuf,
};

/// Type that allows deserializing a string without heap-allocation.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Text(pub(crate) TokenText);
impl Text {
    pub fn text(&self) -> &str {
        self.0.text()
    }
}
impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.text()
    }
}
impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}
impl Deserializable for Text {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Text;
            const EXPECTED_TYPE: VisitableType = VisitableType::STR;
            fn visit_str(
                self,
                value: Text,
                _range: TextRange,
                _name: &str,
                _diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(Visitor, name, diagnostics)
    }
}

/// A string representation of an integer or a float.
/// The format should be parsable by Rust numeric types.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TextNumber(pub(crate) TokenText);
impl TextNumber {
    pub fn text(&self) -> &str {
        self.0.text()
    }
}
impl Deref for TextNumber {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.text()
    }
}
impl std::fmt::Display for TextNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}
impl Deserializable for TextNumber {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = TextNumber;
            const EXPECTED_TYPE: VisitableType = VisitableType::NUMBER;
            fn visit_number(
                self,
                value: TextNumber,
                _range: TextRange,
                _name: &str,
                _diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(Visitor, name, diagnostics)
    }
}

impl Deserializable for () {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = ();
            const EXPECTED_TYPE: VisitableType = VisitableType::empty();
        }
        value.deserialize(Visitor, name, diagnostics)
    }
}

impl Deserializable for bool {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = bool;
            const EXPECTED_TYPE: VisitableType = VisitableType::BOOL;
            fn visit_bool(
                self,
                value: bool,
                _range: TextRange,
                _name: &str,
                _diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(Visitor, name, diagnostics)
    }
}

impl Deserializable for f32 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new("The number should be a float representable on 32 bits")
                .with_range(value.range());
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for f64 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new("The number should be a float representable on 64 bits")
                .with_range(value.range());
        diagnostics.push(diagnostic);
        None
    }
}

impl Deserializable for i8 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for i16 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for i32 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for isize {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for i64 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u8 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u16 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u32 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for usize {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u64 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU8 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU16 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU32 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroUsize {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU64 {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        diagnostics.push(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for String {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        Text::deserialize(value, name, diagnostics).map(|value| value.text().to_string())
    }
}

impl Deserializable for PathBuf {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        String::deserialize(value, name, diagnostics).map(PathBuf::from)
    }
}

impl<T: Deserializable> Deserializable for Vec<T> {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = Vec<T>;
            const EXPECTED_TYPE: VisitableType = VisitableType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(&value?, "", diagnostics))
                        .collect(),
                )
            }
        }
        value.deserialize(Visitor(PhantomData), name, diagnostics)
    }
}

impl<T: Deserializable + Eq + Hash, S: BuildHasher + Default> Deserializable for HashSet<T, S> {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T, S>(PhantomData<(T, S)>);
        impl<T: Deserializable + Eq + Hash, S: BuildHasher + Default> DeserializationVisitor
            for Visitor<T, S>
        {
            type Output = HashSet<T, S>;
            const EXPECTED_TYPE: VisitableType = VisitableType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(&value?, "", diagnostics))
                        .collect(),
                )
            }
        }
        value.deserialize(Visitor(PhantomData), name, diagnostics)
    }
}

impl<T: Hash + Eq + Deserializable> Deserializable for IndexSet<T> {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Hash + Eq + Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = IndexSet<T>;
            const EXPECTED_TYPE: VisitableType = VisitableType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(&value?, "", diagnostics))
                        .collect(),
                )
            }
        }
        value.deserialize(Visitor(PhantomData), name, diagnostics)
    }
}

impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher> Deserializable
    for HashMap<K, V, S>
{
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<K, V, S>(PhantomData<(K, V, S)>);
        impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher>
            DeserializationVisitor for Visitor<K, V, S>
        {
            type Output = HashMap<K, V, S>;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let key = Deserializable::deserialize(&key, "", diagnostics);
                    let value = Deserializable::deserialize(&value, "", diagnostics);
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(Visitor(PhantomData), name, diagnostics)
    }
}

impl<K: Ord + Deserializable, V: Deserializable> Deserializable for BTreeMap<K, V> {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<K, V>(PhantomData<(K, V)>);
        impl<K: Ord + Deserializable, V: Deserializable> DeserializationVisitor for Visitor<K, V> {
            type Output = BTreeMap<K, V>;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let key = Deserializable::deserialize(&key, "", diagnostics);
                    let value = Deserializable::deserialize(&value, "", diagnostics);
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(Visitor(PhantomData), name, diagnostics)
    }
}

impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher> Deserializable
    for IndexMap<K, V, S>
{
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<K, V, S>(PhantomData<(K, V, S)>);
        impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher>
            DeserializationVisitor for Visitor<K, V, S>
        {
            type Output = IndexMap<K, V, S>;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let key = Deserializable::deserialize(&key, "", diagnostics);
                    let value = Deserializable::deserialize(&value, "", diagnostics);
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(Visitor(PhantomData), name, diagnostics)
    }
}
