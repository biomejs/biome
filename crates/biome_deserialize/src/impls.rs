//! Implementations of [Deserializable] for common data structures.
//!
//! Tests of these implementations are available in [biome_deserialize::json::tests] module.
use crate::{
    diagnostics::DeserializableTypes, Deserializable, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor,
};
use biome_rowan::{TextRange, TokenText};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    ops::Deref,
    path::PathBuf,
};

/// Type that allows deserializing a string without heap-allocation when possible.
///
/// This is analogous to [std::borrow::Cow], except for strings.
#[derive(Clone, Debug)]
pub enum Text {
    Borrowed(TokenText),
    Owned(String),
}
impl Text {
    pub fn text(&self) -> &str {
        match self {
            Text::Borrowed(token_text) => token_text.text(),
            Text::Owned(string) => string,
        }
    }
}
impl Default for Text {
    fn default() -> Self {
        Self::Owned(String::new())
    }
}
impl Deserializable for Text {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Text;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::STR;
            fn visit_str(
                self,
                _ctx: &mut impl DeserializationContext,
                value: Text,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(ctx, Visitor, name)
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
impl Eq for Text {}
impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.text().as_bytes());
    }
}
impl Ord for Text {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text().cmp(other.text())
    }
}
impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}
impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<Text> for String {
    fn from(value: Text) -> Self {
        match value {
            Text::Borrowed(token_text) => token_text.to_string(),
            Text::Owned(string) => string,
        }
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
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = TextNumber;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::NUMBER;
            fn visit_number(
                self,
                _ctx: &mut impl DeserializationContext,
                value: TextNumber,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(ctx, Visitor, name)
    }
}

impl Deserializable for () {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = ();
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::empty();
        }
        value.deserialize(ctx, Visitor, name)
    }
}

impl Deserializable for bool {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = bool;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::BOOL;
            fn visit_bool(
                self,
                _ctx: &mut impl DeserializationContext,
                value: bool,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(value)
            }
        }
        value.deserialize(ctx, Visitor, name)
    }
}

impl Deserializable for f32 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new("The number should be a float representable on 32 bits")
                .with_range(value.range());
        ctx.report(diagnostic);
        None
    }
}

impl Deserializable for f64 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        let diagnostic =
            DeserializationDiagnostic::new("The number should be a float representable on 64 bits")
                .with_range(value.range());
        ctx.report(diagnostic);
        None
    }
}

impl Deserializable for i8 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for i16 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for i32 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for isize {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for i64 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u8 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u16 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u32 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for usize {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for u64 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN,
            Self::MAX,
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU8 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU16 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU32 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroUsize {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for NonZeroU64 {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value_text = TextNumber::deserialize(ctx, value, name)?;
        if let Ok(value) = value_text.parse::<Self>() {
            return Some(value);
        }
        ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
            Self::MIN.get(),
            Self::MAX.get(),
            value.range(),
        ));
        None
    }
}

impl Deserializable for String {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Text::deserialize(ctx, value, name).map(|value| value.into())
    }
}

impl Deserializable for Box<str> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        String::deserialize(ctx, value, name).map(|s| s.into_boxed_str())
    }
}

impl Deserializable for PathBuf {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        String::deserialize(ctx, value, name).map(PathBuf::from)
    }
}

impl<T: Deserializable> Deserializable for Box<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        T::deserialize(ctx, value, name).map(Box::new)
    }
}

impl<T: Deserializable> Deserializable for Option<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type() == Some(crate::DeserializableType::Null) {
            None
        } else {
            T::deserialize(ctx, value, name).map(Option::Some)
        }
    }
}

impl<T: Deserializable> Deserializable for Vec<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = Vec<T>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(ctx, &value?, ""))
                        .collect(),
                )
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

impl<T: Deserializable> Deserializable for Box<[T]> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Deserializable::deserialize(ctx, value, name).map(Vec::into_boxed_slice)
    }
}

#[cfg(feature = "smallvec")]
impl<T: Deserializable, const L: usize> Deserializable for smallvec::SmallVec<[T; L]> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<T, const L: usize>(PhantomData<T>);
        impl<T: Deserializable, const L: usize> DeserializationVisitor for Visitor<T, L> {
            type Output = smallvec::SmallVec<[T; L]>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(ctx, &value?, ""))
                        .collect(),
                )
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

impl<T: Deserializable + Eq + Hash, S: BuildHasher + Default> Deserializable for HashSet<T, S> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<T, S>(PhantomData<(T, S)>);
        impl<T: Deserializable + Eq + Hash, S: BuildHasher + Default> DeserializationVisitor
            for Visitor<T, S>
        {
            type Output = HashSet<T, S>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(ctx, &value?, ""))
                        .collect(),
                )
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

impl<T: Ord + Deserializable> Deserializable for BTreeSet<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Ord + Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = BTreeSet<T>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(ctx, &value?, ""))
                        .collect(),
                )
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

#[cfg(feature = "indexmap")]
impl<T: Hash + Eq + Deserializable> Deserializable for indexmap::IndexSet<T> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<T>(PhantomData<T>);
        impl<T: Hash + Eq + Deserializable> DeserializationVisitor for Visitor<T> {
            type Output = indexmap::IndexSet<T>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(
                    values
                        .filter_map(|value| Deserializable::deserialize(ctx, &value?, ""))
                        .collect(),
                )
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher> Deserializable
    for HashMap<K, V, S>
{
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<K, V, S>(PhantomData<(K, V, S)>);
        impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher>
            DeserializationVisitor for Visitor<K, V, S>
        {
            type Output = HashMap<K, V, S>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let key = Deserializable::deserialize(ctx, &key, "");
                    let value = Deserializable::deserialize(ctx, &value, "");
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

impl<K: Ord + Deserializable, V: Deserializable> Deserializable for BTreeMap<K, V> {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<K, V>(PhantomData<(K, V)>);
        impl<K: Ord + Deserializable, V: Deserializable> DeserializationVisitor for Visitor<K, V> {
            type Output = BTreeMap<K, V>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let key = Deserializable::deserialize(ctx, &key, "");
                    let value = Deserializable::deserialize(ctx, &value, "");
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

#[cfg(feature = "indexmap")]
impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher> Deserializable
    for indexmap::IndexMap<K, V, S>
{
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<K, V, S>(PhantomData<(K, V, S)>);
        impl<K: Hash + Eq + Deserializable, V: Deserializable, S: Default + BuildHasher>
            DeserializationVisitor for Visitor<K, V, S>
        {
            type Output = indexmap::IndexMap<K, V, S>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let key = Deserializable::deserialize(ctx, &key, "");
                    let value = Deserializable::deserialize(ctx, &value, "");
                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
                Some(result)
            }
        }
        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

#[cfg(feature = "camino")]
impl Deserializable for camino::Utf8PathBuf {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        String::deserialize(ctx, value, name).map(Self::from)
    }
}
