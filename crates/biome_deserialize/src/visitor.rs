use crate::{diagnostics::DeserializationDiagnostics, DeserializationDiagnostic};
use biome_rowan::{Language, SyntaxNode, TokenText};

/// Generic trait to implement when resolving the configuration from a generic language
pub trait VisitNode<L: Language>: Sized {
    /// Called when visiting the key of a member
    fn visit_member_name(
        &mut self,
        _node: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_member_name")
    }
    /// Called when visiting the value of a member
    fn visit_member_value(
        &mut self,
        _node: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_member_value")
    }

    /// Called when visiting a list of key-value.
    ///
    /// The implementor should loop through the list and call this function by passing two nodes,
    /// the key/name as first argument, and the value as second argument.
    fn visit_map(
        &mut self,
        _key: &SyntaxNode<L>,
        _value: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_map")
    }

    /// Called when visiting a list of elements.
    ///
    /// The implementor should loop through the list and call this function by passing the encountered nodes.
    fn visit_array_member(
        &mut self,
        _element: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        unimplemented!("you should implement visit_array_member")
    }
}

impl<L: Language> VisitNode<L> for () {
    fn visit_map(
        &mut self,
        _key: &SyntaxNode<L>,
        _value: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        Some(())
    }

    fn visit_member_name(
        &mut self,
        _node: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        Some(())
    }

    fn visit_member_value(
        &mut self,
        _node: &SyntaxNode<L>,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        Some(())
    }
}

pub trait DeserializationVisitor<L: DeserializableLanguage>: Sized {
    fn visit_null(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Self::fallback(diagnostics)
    }

    fn visit_bool(_value: bool, diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Self::fallback(diagnostics)
    }

    fn visit_i64(_value: i64, diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Self::fallback(diagnostics)
    }

    fn visit_u64(value: u64, diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        // SAFE because i64::MAX is representable with u64
        if value <= u64::try_from(i64::MAX).unwrap() {
            Self::visit_i64(value as i64, diagnostics)
        } else {
            Self::fallback(diagnostics)
        }
    }

    fn visit_token_text(
        value: TokenText,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<Self> {
        Self::visit_str(value.text(), diagnostics)
    }

    fn visit_str(_value: &str, diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Self::fallback(diagnostics)
    }

    fn visit_map(
        _members: impl Iterator<Item = (SyntaxNode<L>, SyntaxNode<L>)>,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<Self> {
        Self::fallback(diagnostics)
    }

    fn visit_array(
        _items: impl Iterator<Item = SyntaxNode<L>>,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<Self> {
        Self::fallback(diagnostics)
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        // Default impl assume that we implement a map (struct / object).
        diagnostics.report_incorrect_type("map");
        None
    }
}

pub trait Deserializable<L: DeserializableLanguage> {
    fn deserialize<V: DeserializationVisitor<L>>(
        self,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<V>;
}

impl<L: DeserializableLanguage> Deserializable<L> for SyntaxNode<L> {
    fn deserialize<V: DeserializationVisitor<L>>(
        self,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<V> {
        L::deserialize_value(self, diagnostics)
    }
}

pub trait DeserializableLanguage: Language {
    fn deserialize_value<V: DeserializationVisitor<Self>>(
        value: SyntaxNode<Self>,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<V>;
}

impl<L: DeserializableLanguage> DeserializationVisitor<L> for bool {
    fn visit_bool(value: bool, _diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Some(value)
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_incorrect_type("boolean");
        None
    }
}

impl<L: DeserializableLanguage> DeserializationVisitor<L> for i64 {
    fn visit_i64(value: i64, _diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Some(value)
    }

    fn visit_u64(_value: u64, diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_number_out_of_bounds(Self::MIN, Self::MAX as u64);
        None
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_incorrect_type("number");
        None
    }
}

impl<L: DeserializableLanguage> DeserializationVisitor<L> for u64 {
    fn visit_i64(_value: i64, diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_number_out_of_bounds(Self::MIN as i64, Self::MAX);
        None
    }

    fn visit_u64(value: u64, _diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Some(value)
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_incorrect_type("number");
        None
    }
}

impl<L: DeserializableLanguage> DeserializationVisitor<L> for TokenText {
    fn visit_token_text(
        value: TokenText,
        _diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<Self> {
        Some(value)
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_incorrect_type("string");
        None
    }
}

impl<L: DeserializableLanguage> DeserializationVisitor<L> for String {
    fn visit_str(value: &str, _diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Some(value.to_string())
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_incorrect_type("string");
        None
    }
}

impl<L: DeserializableLanguage, T: DeserializationVisitor<L>> DeserializationVisitor<L> for Vec<T> {
    fn visit_array(
        values: impl Iterator<Item = SyntaxNode<L>>,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<Self> {
        let (min_size, max_size) = values.size_hint();
        let mut result = Vec::with_capacity(max_size.unwrap_or(min_size));
        for value in values {
            if let Some(item) = L::deserialize_value(value, diagnostics) {
                result.push(item);
            }
        }
        Some(result)
    }

    fn fallback(diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        diagnostics.report_incorrect_type("array");
        None
    }
}

impl<L: DeserializableLanguage, T: DeserializationVisitor<L>> DeserializationVisitor<L>
    for Option<T>
{
    fn visit_null(_diagnostics: &mut DeserializationDiagnostics) -> Option<Self> {
        Some(None)
    }

    fn visit_map(
        members: impl Iterator<Item = (SyntaxNode<L>, SyntaxNode<L>)>,
        diagnostics: &mut DeserializationDiagnostics,
    ) -> Option<Self> {
        Some(T::visit_map(members, diagnostics))
    }
}
