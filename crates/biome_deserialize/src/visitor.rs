use crate::DeserializationDiagnostic;
use biome_rowan::{Language, SyntaxNode, TextRange, TokenText};

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

pub trait NodeVisitor<L: DeserializableLanguage> {
    fn visit_unit(&mut self, range: TextRange, diagnostics: &mut Vec<DeserializationDiagnostic>) {
        self.fallback_on_error(range, diagnostics)
    }

    fn visit_bool(
        &mut self,
        _value: bool,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        self.fallback_on_error(range, diagnostics)
    }

    fn visit_str(
        &mut self,
        _value: &str,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        self.fallback_on_error(range, diagnostics)
    }

    fn visit_map(
        &mut self,
        _members: impl Iterator<Item = (SyntaxNode<L>, SyntaxNode<L>)>,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        self.fallback_on_error(range, diagnostics)
    }

    fn visit_array(
        &mut self,
        _items: impl Iterator<Item = SyntaxNode<L>>,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        self.fallback_on_error(range, diagnostics)
    }

    fn fallback_on_error(
        &self,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        diagnostics.push(DeserializationDiagnostic::new("unsupported value").with_range(range))
    }
}

pub trait Acceptable<L: Language> {
    fn deserialize(
        &mut self,
        value: SyntaxNode<L>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    );
}

impl<L: DeserializableLanguage, T: NodeVisitor<L>> Acceptable<L> for T {
    fn deserialize(
        &mut self,
        value: SyntaxNode<L>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        L::deserialize(self, value, diagnostics)
    }
}

pub trait DeserializableLanguage: Language {
    fn deserialize(
        visitor: &mut impl NodeVisitor<Self>,
        value: SyntaxNode<Self>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    );

    fn map_to_str(value: SyntaxNode<Self>) -> Option<TokenText>;
}

impl<L: DeserializableLanguage> NodeVisitor<L> for bool {
    fn visit_bool(
        &mut self,
        value: bool,
        _range: TextRange,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        *self = value;
    }

    fn fallback_on_error(
        &self,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            "boolean", range,
        ))
    }
}

impl<L: DeserializableLanguage> NodeVisitor<L> for String {
    fn visit_str(
        &mut self,
        value: &str,
        _range: TextRange,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        *self = value.to_string();
    }

    fn fallback_on_error(
        &self,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            "string", range,
        ))
    }
}

impl<L: DeserializableLanguage, T: Default + NodeVisitor<L>> NodeVisitor<L> for Vec<T> {
    fn visit_array(
        &mut self,
        values: impl Iterator<Item = SyntaxNode<L>>,
        _range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        for value in values {
            let mut element = T::default();
            element.deserialize(value, diagnostics);
            self.push(element);
        }
    }

    fn fallback_on_error(
        &self,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) {
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            "array", range,
        ))
    }
}
