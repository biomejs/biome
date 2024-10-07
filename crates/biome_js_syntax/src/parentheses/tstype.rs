use biome_rowan::{AstNode, AstSeparatedList};

use crate::{
    parentheses::NeedsParentheses, type_ext::AnyTsFunctionType, AnyTsReturnType, AnyTsType,
    JsSyntaxKind, JsSyntaxNode, TsConditionalType, TsConstructorType, TsFunctionType,
    TsIndexedAccessType, TsInferType, TsIntersectionType, TsIntersectionTypeElementList,
    TsTypeOperatorType, TsTypeofType, TsUnionType, TsUnionTypeVariantList,
};

impl NeedsParentheses for TsConditionalType {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                let conditional = TsConditionalType::unwrap_cast(parent.clone());
                let is_extends_type = conditional
                    .extends_type()
                    .is_ok_and(|extends_type| extends_type.syntax() == self.syntax());
                let is_check_type = conditional
                    .check_type()
                    .is_ok_and(|check_type| check_type.syntax() == self.syntax());
                is_check_type || is_extends_type
            }
            JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                TsUnionTypeVariantList::unwrap_cast(parent).len() > 1
            }
            JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
                TsIntersectionTypeElementList::unwrap_cast(parent).len() > 1
            }
            _ => operator_type_or_higher_needs_parens(self.syntax(), parent),
        }
    }
}

impl NeedsParentheses for TsConstructorType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        function_like_type_needs_parentheses(self.syntax())
    }
}

impl NeedsParentheses for TsFunctionType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        function_like_type_needs_parentheses(self.syntax())
    }
}

impl NeedsParentheses for TsInferType {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => false,
            JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST
            | JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => self.constraint().is_some(),
            _ => operator_type_or_higher_needs_parens(self.syntax(), parent),
        }
    }
}

impl NeedsParentheses for TsIntersectionType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                self.types().len() > 1 && TsUnionTypeVariantList::unwrap_cast(parent).len() > 1
            }
            JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
                self.types().len() > 1
                    && TsIntersectionTypeElementList::unwrap_cast(parent).len() > 1
            }
            _ => operator_type_or_higher_needs_parens(self.syntax(), parent),
        }
    }
}

impl NeedsParentheses for TsTypeOperatorType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        operator_type_or_higher_needs_parens(self.syntax(), parent)
    }
}

impl NeedsParentheses for TsTypeofType {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::TS_ARRAY_TYPE => true,
            // Typeof operators are parenthesized when used as an object type in an indexed access
            // to avoid ambiguity of precedence, as it's higher than the JS equivalent:
            // ```typescript
            // const array = [1, 2, 3]
            // type T = typeof array[0]; // => number
            // type T2 = (typeof array)[0]; // => number
            // const J1 = typeof array[0]; // => 'number'
            // const J2 = (typeof array)[0]; // => 'o', because `typeof array` is 'object'
            // ```
            JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                let indexed = TsIndexedAccessType::unwrap_cast(parent);
                // The typeof operator only needs parens if it's the object of the indexed access.
                // If it's the index_type, then the braces already act as the visual precedence.
                indexed.object_type().map(AstNode::into_syntax).as_ref() == Ok(self.syntax())
            }
            _ => false,
        }
    }
}

impl NeedsParentheses for TsUnionType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                self.types().len() > 1 && TsUnionTypeVariantList::unwrap_cast(parent).len() > 1
            }
            JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
                self.types().len() > 1
                    && TsIntersectionTypeElementList::unwrap_cast(parent).len() > 1
            }
            _ => operator_type_or_higher_needs_parens(self.syntax(), parent),
        }
    }
}

fn function_like_type_needs_parentheses(node: &JsSyntaxNode) -> bool {
    debug_assert!(AnyTsFunctionType::can_cast(node.kind()));
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => parent.parent().is_some_and(|grand_parent| {
            grand_parent.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        }),
        JsSyntaxKind::TS_CONDITIONAL_TYPE => {
            let conditional = TsConditionalType::unwrap_cast(parent.clone());
            let is_check_type = conditional
                .check_type()
                .is_ok_and(|check_type| check_type.syntax() == node);
            if is_check_type {
                return true;
            }
            let is_not_extends_type = conditional
                .extends_type()
                .is_ok_and(|extends_type| extends_type.syntax() != node);
            if is_not_extends_type {
                return false;
            }
            match AnyTsFunctionType::unwrap_cast(node.clone()).return_type() {
                Ok(AnyTsReturnType::AnyTsType(AnyTsType::TsInferType(infer_type))) => {
                    infer_type.constraint().is_some()
                }
                Ok(AnyTsReturnType::TsAssertsReturnType(asserts_type)) => {
                    asserts_type.predicate().is_some()
                }
                _ => false,
            }
        }
        JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
            TsUnionTypeVariantList::unwrap_cast(parent).len() > 1
        }
        JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
            TsIntersectionTypeElementList::unwrap_cast(parent).len() > 1
        }
        _ => operator_type_or_higher_needs_parens(node, parent),
    }
}

/// Returns `true` if a TS primary type needs parentheses
fn operator_type_or_higher_needs_parens(node: &JsSyntaxNode, parent: JsSyntaxNode) -> bool {
    match parent.kind() {
        JsSyntaxKind::TS_ARRAY_TYPE
        | JsSyntaxKind::TS_TYPE_OPERATOR_TYPE
        | JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT
        | JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => true,
        _ => TsIndexedAccessType::cast(parent)
            .and_then(|cast| cast.object_type().ok())
            .is_some_and(|object_type| object_type.syntax() == node),
    }
}
