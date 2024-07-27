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
            | JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => true,
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
            let ret_type = AnyTsFunctionType::unwrap_cast(node.clone()).return_type();
            // Tests if `node` includes inferred return types with extends constraints
            if let Ok(AnyTsReturnType::AnyTsType(AnyTsType::TsInferType(infer_type))) = ret_type {
                infer_type.constraint().is_some()
            } else {
                false
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

// #[cfg(test)]
// mod tests {
//     use crate::*;

//     #[test]
//     fn ts_conditional_type() {
//         assert_needs_parentheses!("type s = (A extends B ? C : D)[]", TsConditionalType);

//         assert_needs_parentheses!("type s = unique (A extends B ? C : D);", TsConditionalType);

//         assert_needs_parentheses!(
//             "type s = [number, ...(A extends B ? C : D)]",
//             TsConditionalType
//         );
//         assert_needs_parentheses!("type s = [(A extends B ? C : D)?]", TsConditionalType);

//         assert_needs_parentheses!("type s = (A extends B ? C : D)[a]", TsConditionalType);
//         assert_not_needs_parentheses!("type s = a[A extends B ? C : D]", TsConditionalType);

//         assert_needs_parentheses!("type s = (A extends B ? C : D) & b", TsConditionalType);
//         assert_needs_parentheses!("type s = a & (A extends B ? C : D)", TsConditionalType);

//         // This does require parentheses but the formatter will strip the leading `&`, leaving only the inner type
//         // thus, no parentheses are required
//         assert_not_needs_parentheses!("type s = &(A extends B ? C : D)", TsConditionalType);

//         assert_needs_parentheses!("type s = (A extends B ? C : D) | b", TsConditionalType);
//         assert_needs_parentheses!("type s = a | (A extends B ? C : D)", TsConditionalType);
//         assert_not_needs_parentheses!("type s = |(A extends B ? C : D)", TsConditionalType);

//         assert_needs_parentheses!(
//             "type s = (A extends B ? C : D) extends E ? F : G",
//             TsConditionalType[1]
//         );
//         assert_not_needs_parentheses!(
//             "type s = (A extends B ? C : D) extends E ? F : G",
//             TsConditionalType[0]
//         );

//         assert_needs_parentheses!(
//             "type s = A extends (B extends C ? D : E) ? F : G",
//             TsConditionalType[1]
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends (B extends C ? D : E) ? F : G",
//             TsConditionalType[0]
//         );

//         assert_not_needs_parentheses!(
//             "type s = A extends B ? (C extends D ? E : F) : G",
//             TsConditionalType[0]
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends B ? (C extends D ? E : F) : G",
//             TsConditionalType[1]
//         );

//         assert_not_needs_parentheses!(
//             "type s = A extends B ? C : (D extends E ? F : G)",
//             TsConditionalType[0]
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends B ? C : (D extends E ? F : G)",
//             TsConditionalType[1]
//         );
//     }

//     #[test]
//     fn ts_constructor_type() {
//         assert_needs_parentheses!("type s = (new () => string)[]", TsConstructorType);

//         assert_needs_parentheses!("type s = unique (new () => string);", TsConstructorType);

//         assert_needs_parentheses!(
//             "type s = [number, ...(new () => string)]",
//             TsConstructorType
//         );
//         assert_needs_parentheses!("type s = [(new () => string)?]", TsConstructorType);

//         assert_needs_parentheses!("type s = (new () => string)[a]", TsConstructorType);
//         assert_not_needs_parentheses!("type s = a[new () => string]", TsConstructorType);

//         assert_needs_parentheses!("type s = (new () => string) & b", TsConstructorType);
//         assert_needs_parentheses!("type s = a & (new () => string)", TsConstructorType);

//         // This does require parentheses but the formatter will strip the leading `&`, leaving only the inner type
//         // thus, no parentheses are required
//         assert_not_needs_parentheses!("type s = &(new () => string)", TsConstructorType);

//         assert_needs_parentheses!("type s = (new () => string) | b", TsConstructorType);
//         assert_needs_parentheses!("type s = a | (new () => string)", TsConstructorType);
//         assert_not_needs_parentheses!("type s = |(new () => string)", TsConstructorType);

//         assert_needs_parentheses!(
//             "type s = (new () => string) extends string ? string : number",
//             TsConstructorType
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends string ? (new () => string) : number",
//             TsConstructorType
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends string ? string : (new () => string)",
//             TsConstructorType
//         )
//     }

//     #[test]
//     fn ts_function_type() {
//         assert_needs_parentheses!("type s = (() => string)[]", TsFunctionType);

//         assert_needs_parentheses!("type s = unique (() => string);", TsFunctionType);

//         assert_needs_parentheses!("type s = [number, ...(() => string)]", TsFunctionType);
//         assert_needs_parentheses!("type s = [(() => string)?]", TsFunctionType);

//         assert_needs_parentheses!("type s = (() => string)[a]", TsFunctionType);
//         assert_not_needs_parentheses!("type s = a[() => string]", TsFunctionType);

//         assert_needs_parentheses!("type s = (() => string) & b", TsFunctionType);
//         assert_needs_parentheses!("type s = a & (() => string)", TsFunctionType);

//         // This does require parentheses but the formatter will strip the leading `&`, leaving only the inner type
//         // thus, no parentheses are required
//         assert_not_needs_parentheses!("type s = &(() => string)", TsFunctionType);

//         assert_needs_parentheses!("type s = (() => string) | b", TsFunctionType);
//         assert_needs_parentheses!("type s = a | (() => string)", TsFunctionType);
//         assert_not_needs_parentheses!("type s = |(() => string)", TsFunctionType);

//         assert_needs_parentheses!(
//             "type s = (() => string) extends string ? string : number",
//             TsFunctionType
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends string ? (() => string) : number",
//             TsFunctionType
//         );
//         assert_not_needs_parentheses!(
//             "type s = A extends string ? string : (() => string)",
//             TsFunctionType
//         )
//     }

//     #[test]
//     fn ts_infer_type() {
//         assert_needs_parentheses!(
//             "type A = T extends (infer string)[] ? string : never",
//             TsInferType
//         );
//         assert_needs_parentheses!(
//             "type A = T extends unique (infer string) ? string : never",
//             TsInferType
//         );

//         assert_not_needs_parentheses!(
//             "type A = T extends [number, ...infer string] ? string : never",
//             TsInferType
//         );
//         assert_needs_parentheses!(
//             "type A = T extends [(infer string)?] ? string : never",
//             TsInferType
//         );
//         assert_needs_parentheses!(
//             "type A = T extends [(infer string) | undefined] ? string : never",
//             TsInferType
//         );

//         assert_needs_parentheses!(
//             "type A = T extends (infer string)[a] ? string : never",
//             TsInferType
//         );
//         assert_not_needs_parentheses!(
//             "type A = T extends a[(infer string)] ? string : never",
//             TsInferType
//         );
//     }

//     #[test]
//     fn ts_intersection_type() {
//         assert_needs_parentheses!("let s: (string & number)[] = symbol();", TsIntersectionType);

//         assert_needs_parentheses!("let s: unique (string & number);", TsIntersectionType);

//         assert_needs_parentheses!("let s: [number, ...(string & number)]", TsIntersectionType);
//         assert_needs_parentheses!("let s: [(string & number)?]", TsIntersectionType);

//         assert_needs_parentheses!("let s: (string & number)[a]", TsIntersectionType);
//         assert_not_needs_parentheses!("let s: a[(string & number)]", TsIntersectionType);

//         assert_not_needs_parentheses!("let s: (&a) & (&b)", TsIntersectionType[1]);
//         assert_not_needs_parentheses!("let s: (&a) & (&b)", TsIntersectionType[2]);

//         assert_needs_parentheses!("let s: (a & b) & (&c)", TsIntersectionType[1]);
//         assert_not_needs_parentheses!("let s: (a & b) & (&c)", TsIntersectionType[2]);
//     }

//     #[test]
//     fn ts_type_operator_type() {
//         assert_needs_parentheses!("let s: (unique symbol)[] = symbol();", TsTypeOperatorType);

//         assert_needs_parentheses!("let s: unique (unique symbol);", TsTypeOperatorType[1]);
//         assert_not_needs_parentheses!("let s: unique (unique symbol);", TsTypeOperatorType[0]);

//         assert_needs_parentheses!("let s: [number, ...(unique symbol)]", TsTypeOperatorType);
//         assert_needs_parentheses!("let s: [(unique symbol)?]", TsTypeOperatorType);

//         assert_needs_parentheses!("let s: (unique symbol)[a]", TsTypeOperatorType);
//         assert_not_needs_parentheses!("let s: a[(unique symbol)]", TsTypeOperatorType);
//     }
//     #[test]
//     fn ts_typeof_type() {
//         assert_not_needs_parentheses!("let s: typeof obj;", TsTypeofType);

//         assert_needs_parentheses!("let s: typeof obj[number];", TsTypeofType);
//         assert_needs_parentheses!("let s: keyof (typeof obj)[number];", TsTypeofType);

//         // Disambiguates to `keyof ((typeof obj)[number])`, so the outer `keyof` doesn't need
//         // parentheses as it's written here, but the inner `typeof obj` does, to clarify precedence.
//         assert_not_needs_parentheses!("let s: keyof (typeof obj[number]);", TsTypeOperatorType);
//         assert_needs_parentheses!("let s: keyof (typeof obj[number]);", TsTypeofType);

//         // Forced precedence change with added parentheses, the `typeof` is no longer an indexed
//         // access type, so no parentheses are needed on it directly.
//         assert_not_needs_parentheses!("let s: (keyof typeof obj)[number];", TsTypeofType);
//         assert_needs_parentheses!("let s: (keyof typeof obj)[number];", TsTypeOperatorType);

//         assert_not_needs_parentheses!("let s: number[typeof obj];", TsTypeofType);
//     }
// }
