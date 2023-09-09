use crate::DeserializationDiagnostic;
use rome_rowan::{Language, SyntaxNode};

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
