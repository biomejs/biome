//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    GritLanguage as Language, GritSyntaxElement as SyntaxElement,
    GritSyntaxElementChildren as SyntaxElementChildren,
    GritSyntaxKind::{self as SyntaxKind, *},
    GritSyntaxList as SyntaxList, GritSyntaxNode as SyntaxNode, GritSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
#[allow(dead_code)]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AnyGritPattern {
    pub(crate) syntax: SyntaxNode,
}
impl AnyGritPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AnyGritPatternFields {
        AnyGritPatternFields {
            any_grit_literal: self.any_grit_literal(),
            grit_pattern_not: self.grit_pattern_not(),
            grit_pattern_or: self.grit_pattern_or(),
            grit_pattern_or_else: self.grit_pattern_or_else(),
            grit_pattern_any: self.grit_pattern_any(),
            grit_pattern_and: self.grit_pattern_and(),
            grit_pattern_maybe: self.grit_pattern_maybe(),
            grit_pattern_if_else: self.grit_pattern_if_else(),
            grit_pattern_contains: self.grit_pattern_contains(),
            grit_pattern_includes: self.grit_pattern_includes(),
            grit_pattern_after: self.grit_pattern_after(),
            grit_pattern_before: self.grit_pattern_before(),
            grit_within: self.grit_within(),
            grit_bubble: self.grit_bubble(),
            grit_node_like: self.grit_node_like(),
            grit_map_accessor: self.grit_map_accessor(),
            grit_list_accessor: self.grit_list_accessor(),
            grit_dot: self.grit_dot(),
            grit_some: self.grit_some(),
            grit_every: self.grit_every(),
            grit_underscore: self.grit_underscore(),
            grit_variable: self.grit_variable(),
            grit_regex_pattern: self.grit_regex_pattern(),
            grit_pattern_as: self.grit_pattern_as(),
            grit_pattern_limit: self.grit_pattern_limit(),
            grit_assignment_as_pattern: self.grit_assignment_as_pattern(),
            grit_pattern_accumulate: self.grit_pattern_accumulate(),
            grit_rewrite: self.grit_rewrite(),
            grit_like: self.grit_like(),
            grit_pattern_where: self.grit_pattern_where(),
            grit_mul_operation: self.grit_mul_operation(),
            grit_div_operation: self.grit_div_operation(),
            grit_mod_operation: self.grit_mod_operation(),
            grit_add_operation: self.grit_add_operation(),
            grit_sub_operation: self.grit_sub_operation(),
            grit_sequential: self.grit_sequential(),
            grit_files: self.grit_files(),
            l_paren_token: self.l_paren_token(),
            any_grit_pattern: self.any_grit_pattern(),
            r_paren_token: self.r_paren_token(),
            grit_bogus_pattern: self.grit_bogus_pattern(),
        }
    }
    pub fn any_grit_literal(&self) -> SyntaxResult<AnyGritLiteral> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn grit_pattern_not(&self) -> SyntaxResult<GritPatternNot> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn grit_pattern_or(&self) -> SyntaxResult<GritPatternOr> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn grit_pattern_or_else(&self) -> SyntaxResult<GritPatternOrElse> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn grit_pattern_any(&self) -> SyntaxResult<GritPatternAny> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn grit_pattern_and(&self) -> SyntaxResult<GritPatternAnd> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn grit_pattern_maybe(&self) -> SyntaxResult<GritPatternMaybe> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn grit_pattern_if_else(&self) -> SyntaxResult<GritPatternIfElse> {
        support::required_node(&self.syntax, 7usize)
    }
    pub fn grit_pattern_contains(&self) -> SyntaxResult<GritPatternContains> {
        support::required_node(&self.syntax, 8usize)
    }
    pub fn grit_pattern_includes(&self) -> SyntaxResult<GritPatternIncludes> {
        support::required_node(&self.syntax, 9usize)
    }
    pub fn grit_pattern_after(&self) -> SyntaxResult<GritPatternAfter> {
        support::required_node(&self.syntax, 10usize)
    }
    pub fn grit_pattern_before(&self) -> SyntaxResult<GritPatternBefore> {
        support::required_node(&self.syntax, 11usize)
    }
    pub fn grit_within(&self) -> SyntaxResult<GritWithin> {
        support::required_node(&self.syntax, 12usize)
    }
    pub fn grit_bubble(&self) -> SyntaxResult<GritBubble> {
        support::required_node(&self.syntax, 13usize)
    }
    pub fn grit_node_like(&self) -> SyntaxResult<GritNodeLike> {
        support::required_node(&self.syntax, 14usize)
    }
    pub fn grit_map_accessor(&self) -> SyntaxResult<GritMapAccessor> {
        support::required_node(&self.syntax, 15usize)
    }
    pub fn grit_list_accessor(&self) -> SyntaxResult<GritListAccessor> {
        support::required_node(&self.syntax, 16usize)
    }
    pub fn grit_dot(&self) -> SyntaxResult<GritDot> {
        support::required_node(&self.syntax, 17usize)
    }
    pub fn grit_some(&self) -> SyntaxResult<GritSome> {
        support::required_node(&self.syntax, 18usize)
    }
    pub fn grit_every(&self) -> SyntaxResult<GritEvery> {
        support::required_node(&self.syntax, 19usize)
    }
    pub fn grit_underscore(&self) -> SyntaxResult<GritUnderscore> {
        support::required_node(&self.syntax, 20usize)
    }
    pub fn grit_variable(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 21usize)
    }
    pub fn grit_regex_pattern(&self) -> SyntaxResult<GritRegexPattern> {
        support::required_node(&self.syntax, 22usize)
    }
    pub fn grit_pattern_as(&self) -> SyntaxResult<GritPatternAs> {
        support::required_node(&self.syntax, 23usize)
    }
    pub fn grit_pattern_limit(&self) -> SyntaxResult<GritPatternLimit> {
        support::required_node(&self.syntax, 24usize)
    }
    pub fn grit_assignment_as_pattern(&self) -> SyntaxResult<GritAssignmentAsPattern> {
        support::required_node(&self.syntax, 25usize)
    }
    pub fn grit_pattern_accumulate(&self) -> SyntaxResult<GritPatternAccumulate> {
        support::required_node(&self.syntax, 26usize)
    }
    pub fn grit_rewrite(&self) -> SyntaxResult<GritRewrite> {
        support::required_node(&self.syntax, 27usize)
    }
    pub fn grit_like(&self) -> SyntaxResult<GritLike> {
        support::required_node(&self.syntax, 28usize)
    }
    pub fn grit_pattern_where(&self) -> SyntaxResult<GritPatternWhere> {
        support::required_node(&self.syntax, 29usize)
    }
    pub fn grit_mul_operation(&self) -> SyntaxResult<GritMulOperation> {
        support::required_node(&self.syntax, 30usize)
    }
    pub fn grit_div_operation(&self) -> SyntaxResult<GritDivOperation> {
        support::required_node(&self.syntax, 31usize)
    }
    pub fn grit_mod_operation(&self) -> SyntaxResult<GritModOperation> {
        support::required_node(&self.syntax, 32usize)
    }
    pub fn grit_add_operation(&self) -> SyntaxResult<GritAddOperation> {
        support::required_node(&self.syntax, 33usize)
    }
    pub fn grit_sub_operation(&self) -> SyntaxResult<GritSubOperation> {
        support::required_node(&self.syntax, 34usize)
    }
    pub fn grit_sequential(&self) -> SyntaxResult<GritSequential> {
        support::required_node(&self.syntax, 35usize)
    }
    pub fn grit_files(&self) -> SyntaxResult<GritFiles> {
        support::required_node(&self.syntax, 36usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 37usize)
    }
    pub fn any_grit_pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 38usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 39usize)
    }
    pub fn grit_bogus_pattern(&self) -> SyntaxResult<GritBogusPattern> {
        support::required_node(&self.syntax, 40usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for AnyGritPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnyGritPatternFields {
    pub any_grit_literal: SyntaxResult<AnyGritLiteral>,
    pub grit_pattern_not: SyntaxResult<GritPatternNot>,
    pub grit_pattern_or: SyntaxResult<GritPatternOr>,
    pub grit_pattern_or_else: SyntaxResult<GritPatternOrElse>,
    pub grit_pattern_any: SyntaxResult<GritPatternAny>,
    pub grit_pattern_and: SyntaxResult<GritPatternAnd>,
    pub grit_pattern_maybe: SyntaxResult<GritPatternMaybe>,
    pub grit_pattern_if_else: SyntaxResult<GritPatternIfElse>,
    pub grit_pattern_contains: SyntaxResult<GritPatternContains>,
    pub grit_pattern_includes: SyntaxResult<GritPatternIncludes>,
    pub grit_pattern_after: SyntaxResult<GritPatternAfter>,
    pub grit_pattern_before: SyntaxResult<GritPatternBefore>,
    pub grit_within: SyntaxResult<GritWithin>,
    pub grit_bubble: SyntaxResult<GritBubble>,
    pub grit_node_like: SyntaxResult<GritNodeLike>,
    pub grit_map_accessor: SyntaxResult<GritMapAccessor>,
    pub grit_list_accessor: SyntaxResult<GritListAccessor>,
    pub grit_dot: SyntaxResult<GritDot>,
    pub grit_some: SyntaxResult<GritSome>,
    pub grit_every: SyntaxResult<GritEvery>,
    pub grit_underscore: SyntaxResult<GritUnderscore>,
    pub grit_variable: SyntaxResult<GritVariable>,
    pub grit_regex_pattern: SyntaxResult<GritRegexPattern>,
    pub grit_pattern_as: SyntaxResult<GritPatternAs>,
    pub grit_pattern_limit: SyntaxResult<GritPatternLimit>,
    pub grit_assignment_as_pattern: SyntaxResult<GritAssignmentAsPattern>,
    pub grit_pattern_accumulate: SyntaxResult<GritPatternAccumulate>,
    pub grit_rewrite: SyntaxResult<GritRewrite>,
    pub grit_like: SyntaxResult<GritLike>,
    pub grit_pattern_where: SyntaxResult<GritPatternWhere>,
    pub grit_mul_operation: SyntaxResult<GritMulOperation>,
    pub grit_div_operation: SyntaxResult<GritDivOperation>,
    pub grit_mod_operation: SyntaxResult<GritModOperation>,
    pub grit_add_operation: SyntaxResult<GritAddOperation>,
    pub grit_sub_operation: SyntaxResult<GritSubOperation>,
    pub grit_sequential: SyntaxResult<GritSequential>,
    pub grit_files: SyntaxResult<GritFiles>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub any_grit_pattern: SyntaxResult<AnyGritPattern>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub grit_bogus_pattern: SyntaxResult<GritBogusPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AnyGritPredicate {
    pub(crate) syntax: SyntaxNode,
}
impl AnyGritPredicate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> AnyGritPredicateFields {
        AnyGritPredicateFields {
            grit_predicate_not: self.grit_predicate_not(),
            grit_predicate_maybe: self.grit_predicate_maybe(),
            grit_predicate_and: self.grit_predicate_and(),
            grit_predicate_or: self.grit_predicate_or(),
            grit_predicate_any: self.grit_predicate_any(),
            grit_predicate_if_else: self.grit_predicate_if_else(),
            grit_predicate_assignment: self.grit_predicate_assignment(),
            grit_predicate_accumulate: self.grit_predicate_accumulate(),
            grit_predicate_rewrite: self.grit_predicate_rewrite(),
            grit_predicate_greater: self.grit_predicate_greater(),
            grit_predicate_less: self.grit_predicate_less(),
            grit_predicate_greater_equal: self.grit_predicate_greater_equal(),
            grit_predicate_less_equal: self.grit_predicate_less_equal(),
            grit_predicate_not_equal: self.grit_predicate_not_equal(),
            grit_predicate_equal: self.grit_predicate_equal(),
            grit_predicate_match: self.grit_predicate_match(),
            grit_predicate_call: self.grit_predicate_call(),
            l_paren_token: self.l_paren_token(),
            any_grit_predicate: self.any_grit_predicate(),
            r_paren_token: self.r_paren_token(),
            grit_boolean_value: self.grit_boolean_value(),
            grit_predicate_return: self.grit_predicate_return(),
            grit_bogus_predicate: self.grit_bogus_predicate(),
        }
    }
    pub fn grit_predicate_not(&self) -> SyntaxResult<GritPredicateNot> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn grit_predicate_maybe(&self) -> SyntaxResult<GritPredicateMaybe> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn grit_predicate_and(&self) -> SyntaxResult<GritPredicateAnd> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn grit_predicate_or(&self) -> SyntaxResult<GritPredicateOr> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn grit_predicate_any(&self) -> SyntaxResult<GritPredicateAny> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn grit_predicate_if_else(&self) -> SyntaxResult<GritPredicateIfElse> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn grit_predicate_assignment(&self) -> SyntaxResult<GritPredicateAssignment> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn grit_predicate_accumulate(&self) -> SyntaxResult<GritPredicateAccumulate> {
        support::required_node(&self.syntax, 7usize)
    }
    pub fn grit_predicate_rewrite(&self) -> SyntaxResult<GritPredicateRewrite> {
        support::required_node(&self.syntax, 8usize)
    }
    pub fn grit_predicate_greater(&self) -> SyntaxResult<GritPredicateGreater> {
        support::required_node(&self.syntax, 9usize)
    }
    pub fn grit_predicate_less(&self) -> SyntaxResult<GritPredicateLess> {
        support::required_node(&self.syntax, 10usize)
    }
    pub fn grit_predicate_greater_equal(&self) -> SyntaxResult<GritPredicateGreaterEqual> {
        support::required_node(&self.syntax, 11usize)
    }
    pub fn grit_predicate_less_equal(&self) -> SyntaxResult<GritPredicateLessEqual> {
        support::required_node(&self.syntax, 12usize)
    }
    pub fn grit_predicate_not_equal(&self) -> SyntaxResult<GritPredicateNotEqual> {
        support::required_node(&self.syntax, 13usize)
    }
    pub fn grit_predicate_equal(&self) -> SyntaxResult<GritPredicateEqual> {
        support::required_node(&self.syntax, 14usize)
    }
    pub fn grit_predicate_match(&self) -> SyntaxResult<GritPredicateMatch> {
        support::required_node(&self.syntax, 15usize)
    }
    pub fn grit_predicate_call(&self) -> SyntaxResult<GritPredicateCall> {
        support::required_node(&self.syntax, 16usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 17usize)
    }
    pub fn any_grit_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 18usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 19usize)
    }
    pub fn grit_boolean_value(&self) -> SyntaxResult<GritBooleanValue> {
        support::required_node(&self.syntax, 20usize)
    }
    pub fn grit_predicate_return(&self) -> SyntaxResult<GritPredicateReturn> {
        support::required_node(&self.syntax, 21usize)
    }
    pub fn grit_bogus_predicate(&self) -> SyntaxResult<GritBogusPredicate> {
        support::required_node(&self.syntax, 22usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for AnyGritPredicate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnyGritPredicateFields {
    pub grit_predicate_not: SyntaxResult<GritPredicateNot>,
    pub grit_predicate_maybe: SyntaxResult<GritPredicateMaybe>,
    pub grit_predicate_and: SyntaxResult<GritPredicateAnd>,
    pub grit_predicate_or: SyntaxResult<GritPredicateOr>,
    pub grit_predicate_any: SyntaxResult<GritPredicateAny>,
    pub grit_predicate_if_else: SyntaxResult<GritPredicateIfElse>,
    pub grit_predicate_assignment: SyntaxResult<GritPredicateAssignment>,
    pub grit_predicate_accumulate: SyntaxResult<GritPredicateAccumulate>,
    pub grit_predicate_rewrite: SyntaxResult<GritPredicateRewrite>,
    pub grit_predicate_greater: SyntaxResult<GritPredicateGreater>,
    pub grit_predicate_less: SyntaxResult<GritPredicateLess>,
    pub grit_predicate_greater_equal: SyntaxResult<GritPredicateGreaterEqual>,
    pub grit_predicate_less_equal: SyntaxResult<GritPredicateLessEqual>,
    pub grit_predicate_not_equal: SyntaxResult<GritPredicateNotEqual>,
    pub grit_predicate_equal: SyntaxResult<GritPredicateEqual>,
    pub grit_predicate_match: SyntaxResult<GritPredicateMatch>,
    pub grit_predicate_call: SyntaxResult<GritPredicateCall>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub any_grit_predicate: SyntaxResult<AnyGritPredicate>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub grit_boolean_value: SyntaxResult<GritBooleanValue>,
    pub grit_predicate_return: SyntaxResult<GritPredicateReturn>,
    pub grit_bogus_predicate: SyntaxResult<GritBogusPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CurlyGritPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CurlyGritPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> CurlyGritPatternFields {
        CurlyGritPatternFields {
            l_curly_token: self.l_curly_token(),
            any_grit_pattern: self.any_grit_pattern(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn any_grit_pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CurlyGritPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CurlyGritPatternFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub any_grit_pattern: SyntaxResult<AnyGritPattern>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritAddOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritAddOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritAddOperationFields {
        GritAddOperationFields {
            left: self.left(),
            plus_token: self.plus_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn plus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritAddOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritAddOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub plus_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl GritAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritAnnotationFields {
        GritAnnotationFields {
            grit_annotation_token: self.grit_annotation_token(),
        }
    }
    pub fn grit_annotation_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritAnnotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritAnnotationFields {
    pub grit_annotation_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritAssignmentAsPattern {
    pub(crate) syntax: SyntaxNode,
}
impl GritAssignmentAsPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritAssignmentAsPatternFields {
        GritAssignmentAsPatternFields {
            container: self.container(),
            eq_token: self.eq_token(),
            pattern: self.pattern(),
        }
    }
    pub fn container(&self) -> SyntaxResult<AnyGritContainer> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritAssignmentAsPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritAssignmentAsPatternFields {
    pub container: SyntaxResult<AnyGritContainer>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBacktickSnippet {
    pub(crate) syntax: SyntaxNode,
}
impl GritBacktickSnippet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBacktickSnippetFields {
        GritBacktickSnippetFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritBacktickSnippet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBacktickSnippetFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBooleanValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritBooleanValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBooleanValueFields {
        GritBooleanValueFields {
            true_token: self.true_token(),
            false_token: self.false_token(),
        }
    }
    pub fn true_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn false_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritBooleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBooleanValueFields {
    pub true_token: SyntaxResult<SyntaxToken>,
    pub false_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBubble {
    pub(crate) syntax: SyntaxNode,
}
impl GritBubble {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBubbleFields {
        GritBubbleFields {
            bubble_token: self.bubble_token(),
            variables: self.variables(),
            pattern: self.pattern(),
        }
    }
    pub fn bubble_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn variables(&self) -> Option<GritBubbleScope> {
        support::node(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritBubble {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBubbleFields {
    pub bubble_token: SyntaxResult<SyntaxToken>,
    pub variables: Option<GritBubbleScope>,
    pub pattern: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritBubbleScope {
    pub(crate) syntax: SyntaxNode,
}
impl GritBubbleScope {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritBubbleScopeFields {
        GritBubbleScopeFields {
            l_paren_token: self.l_paren_token(),
            grit_variable_list: self.grit_variable_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn grit_variable_list(&self) -> GritVariableList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritBubbleScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBubbleScopeFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub grit_variable_list: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritCodeSnippet {
    pub(crate) syntax: SyntaxNode,
}
impl GritCodeSnippet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritCodeSnippetFields {
        GritCodeSnippetFields {
            source: self.source(),
        }
    }
    pub fn source(&self) -> SyntaxResult<GritCodeSnippetSource> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritCodeSnippet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritCodeSnippetFields {
    pub source: SyntaxResult<GritCodeSnippetSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritCurlyPredicateList {
    pub(crate) syntax: SyntaxNode,
}
impl GritCurlyPredicateList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritCurlyPredicateListFields {
        GritCurlyPredicateListFields {
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritCurlyPredicateList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritCurlyPredicateListFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDivOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritDivOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDivOperationFields {
        GritDivOperationFields {
            left: self.left(),
            slash_token: self.slash_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn slash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritDivOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritDivOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub slash_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDot {
    pub(crate) syntax: SyntaxNode,
}
impl GritDot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDotFields {
        GritDotFields {
            dot_token: self.dot_token(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritDot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritDotFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDotdotdot {
    pub(crate) syntax: SyntaxNode,
}
impl GritDotdotdot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDotdotdotFields {
        GritDotdotdotFields {
            dollar_dotdotdot_token: self.dollar_dotdotdot_token(),
            maybe_curly_grit_pattern: self.maybe_curly_grit_pattern(),
        }
    }
    pub fn dollar_dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn maybe_curly_grit_pattern(&self) -> Option<MaybeCurlyGritPattern> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritDotdotdot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritDotdotdotFields {
    pub dollar_dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub maybe_curly_grit_pattern: Option<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritDoubleValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritDoubleValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritDoubleValueFields {
        GritDoubleValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritDoubleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritDoubleValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritEvery {
    pub(crate) syntax: SyntaxNode,
}
impl GritEvery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritEveryFields {
        GritEveryFields {
            every_token: self.every_token(),
            pattern: self.pattern(),
        }
    }
    pub fn every_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritEvery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritEveryFields {
    pub every_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritFiles {
    pub(crate) syntax: SyntaxNode,
}
impl GritFiles {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritFilesFields {
        GritFilesFields {
            multifile_token: self.multifile_token(),
            l_curly_token: self.l_curly_token(),
            files: self.files(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn multifile_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn files(&self) -> GritFilesList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritFilesFields {
    pub multifile_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub files: GritFilesList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritFunctionDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritFunctionDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritFunctionDefinitionFields {
        GritFunctionDefinitionFields {
            function_token: self.function_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn args(&self) -> GritVariableList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn body(&self) -> SyntaxResult<GritCurlyPredicateList> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritFunctionDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritFunctionDefinitionFields {
    pub function_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: GritVariableList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<GritCurlyPredicateList>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritIntValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritIntValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritIntValueFields {
        GritIntValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritIntValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritIntValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageDeclarationFields {
        GritLanguageDeclarationFields {
            language_token: self.language_token(),
            name: self.name(),
            flavor: self.flavor(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn language_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GritLanguageName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn flavor(&self) -> Option<GritLanguageFlavor> {
        support::node(&self.syntax, 2usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLanguageDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLanguageDeclarationFields {
    pub language_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritLanguageName>,
    pub flavor: Option<GritLanguageFlavor>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageFlavor {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageFlavor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageFlavorFields {
        GritLanguageFlavorFields {
            l_paren_token: self.l_paren_token(),
            grit_language_flavor_list: self.grit_language_flavor_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn grit_language_flavor_list(&self) -> GritLanguageFlavorList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLanguageFlavor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLanguageFlavorFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub grit_language_flavor_list: GritLanguageFlavorList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageFlavorKind {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageFlavorKind {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageFlavorKindFields {
        GritLanguageFlavorKindFields {
            flavor_kind: self.flavor_kind(),
        }
    }
    pub fn flavor_kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLanguageFlavorKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLanguageFlavorKindFields {
    pub flavor_kind: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageName {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageNameFields {
        GritLanguageNameFields {
            js_token: self.js_token(),
            css_token: self.css_token(),
            json_token: self.json_token(),
            grit_token: self.grit_token(),
            html_token: self.html_token(),
        }
    }
    pub fn js_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn css_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn json_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn grit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn html_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLanguageName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLanguageNameFields {
    pub js_token: SyntaxResult<SyntaxToken>,
    pub css_token: SyntaxResult<SyntaxToken>,
    pub json_token: SyntaxResult<SyntaxToken>,
    pub grit_token: SyntaxResult<SyntaxToken>,
    pub html_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLanguageSpecificSnippet {
    pub(crate) syntax: SyntaxNode,
}
impl GritLanguageSpecificSnippet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLanguageSpecificSnippetFields {
        GritLanguageSpecificSnippetFields {
            language: self.language(),
            snippet_token: self.snippet_token(),
        }
    }
    pub fn language(&self) -> SyntaxResult<GritLanguageName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn snippet_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLanguageSpecificSnippet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLanguageSpecificSnippetFields {
    pub language: SyntaxResult<GritLanguageName>,
    pub snippet_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLike {
    pub(crate) syntax: SyntaxNode,
}
impl GritLike {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLikeFields {
        GritLikeFields {
            like_token: self.like_token(),
            grit_like_threshold: self.grit_like_threshold(),
            l_curly_token: self.l_curly_token(),
            example: self.example(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn like_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn grit_like_threshold(&self) -> Option<GritLikeThreshold> {
        support::node(&self.syntax, 1usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn example(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLikeFields {
    pub like_token: SyntaxResult<SyntaxToken>,
    pub grit_like_threshold: Option<GritLikeThreshold>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub example: SyntaxResult<AnyGritPattern>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritLikeThreshold {
    pub(crate) syntax: SyntaxNode,
}
impl GritLikeThreshold {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritLikeThresholdFields {
        GritLikeThresholdFields {
            l_paren_token: self.l_paren_token(),
            threshold: self.threshold(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn threshold(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLikeThreshold {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritLikeThresholdFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub threshold: SyntaxResult<AnyGritPattern>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritList {
    pub(crate) syntax: SyntaxNode,
}
impl GritList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritListFields {
        GritListFields {
            grit_name: self.grit_name(),
            l_brack_token: self.l_brack_token(),
            patterns: self.patterns(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn grit_name(&self) -> Option<GritName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritListPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritListFields {
    pub grit_name: Option<GritName>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritListPatternList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritListAccessor {
    pub(crate) syntax: SyntaxNode,
}
impl GritListAccessor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritListAccessorFields {
        GritListAccessorFields {
            list: self.list(),
            l_brack_token: self.l_brack_token(),
            index: self.index(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn list(&self) -> SyntaxResult<GritListAccessorSubject> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn index(&self) -> SyntaxResult<GritListIndex> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritListAccessor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritListAccessorFields {
    pub list: SyntaxResult<GritListAccessorSubject>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub index: SyntaxResult<GritListIndex>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMap {
    pub(crate) syntax: SyntaxNode,
}
impl GritMap {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMapFields {
        GritMapFields {
            l_curly_token: self.l_curly_token(),
            elements: self.elements(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> GritMapElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritMapFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub elements: GritMapElementList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMapAccessor {
    pub(crate) syntax: SyntaxNode,
}
impl GritMapAccessor {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMapAccessorFields {
        GritMapAccessorFields {
            map: self.map(),
            dot_token: self.dot_token(),
            key: self.key(),
        }
    }
    pub fn map(&self) -> SyntaxResult<GritMapAccessorSubject> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn key(&self) -> SyntaxResult<GritMapKey> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritMapAccessor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritMapAccessorFields {
    pub map: SyntaxResult<GritMapAccessorSubject>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub key: SyntaxResult<GritMapKey>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMapElement {
    pub(crate) syntax: SyntaxNode,
}
impl GritMapElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMapElementFields {
        GritMapElementFields {
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritMapElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritMapElementFields {
    pub key: SyntaxResult<GritName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritModOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritModOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritModOperationFields {
        GritModOperationFields {
            left: self.left(),
            remainder_token: self.remainder_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn remainder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritModOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritModOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub remainder_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritMulOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritMulOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritMulOperationFields {
        GritMulOperationFields {
            left: self.left(),
            star_token: self.star_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritMulOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritMulOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub star_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritName {
    pub(crate) syntax: SyntaxNode,
}
impl GritName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNameFields {
        GritNameFields {
            grit_name_token: self.grit_name_token(),
        }
    }
    pub fn grit_name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritNameFields {
    pub grit_name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNamedArg {
    pub(crate) syntax: SyntaxNode,
}
impl GritNamedArg {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNamedArgFields {
        GritNamedArgFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritNamedArg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritNamedArgFields {
    pub name: SyntaxResult<GritName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNamedArgWithDefault {
    pub(crate) syntax: SyntaxNode,
}
impl GritNamedArgWithDefault {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNamedArgWithDefaultFields {
        GritNamedArgWithDefaultFields {
            name: self.name(),
            eq_token: self.eq_token(),
            pattern: self.pattern(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritNamedArgWithDefault {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritNamedArgWithDefaultFields {
    pub name: SyntaxResult<GritName>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNegativeIntValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritNegativeIntValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNegativeIntValueFields {
        GritNegativeIntValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritNegativeIntValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritNegativeIntValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNodeLike {
    pub(crate) syntax: SyntaxNode,
}
impl GritNodeLike {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNodeLikeFields {
        GritNodeLikeFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            named_args: self.named_args(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn named_args(&self) -> GritNamedArgList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritNodeLike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritNodeLikeFields {
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub named_args: GritNamedArgList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritNot {
    pub(crate) syntax: SyntaxNode,
}
impl GritNot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritNotFields {
        GritNotFields {
            not_token: self.not_token(),
            excl_token: self.excl_token(),
        }
    }
    pub fn not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritNot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritNotFields {
    pub not_token: SyntaxResult<SyntaxToken>,
    pub excl_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAccumulate {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAccumulate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAccumulateFields {
        GritPatternAccumulateFields {
            left: self.left(),
            add_assign_token: self.add_assign_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn add_assign_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternAccumulate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternAccumulateFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub add_assign_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAfter {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAfter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAfterFields {
        GritPatternAfterFields {
            after_token: self.after_token(),
            pattern: self.pattern(),
        }
    }
    pub fn after_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternAfter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternAfterFields {
    pub after_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAnd {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAnd {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAndFields {
        GritPatternAndFields {
            and_token: self.and_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternAnd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternAndFields {
    pub and_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAny {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAny {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAnyFields {
        GritPatternAnyFields {
            any_token: self.any_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternAnyFields {
    pub any_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternArgList {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternArgList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternArgListFields {
        GritPatternArgListFields {
            grit_variable_list: self.grit_variable_list(),
        }
    }
    pub fn grit_variable_list(&self) -> GritVariableList {
        support::list(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternArgList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternArgListFields {
    pub grit_variable_list: GritVariableList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternAs {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternAs {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternAsFields {
        GritPatternAsFields {
            pattern: self.pattern(),
            as_token: self.as_token(),
            variable: self.variable(),
        }
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn variable(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternAs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternAsFields {
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub variable: SyntaxResult<GritVariable>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternBefore {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternBefore {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternBeforeFields {
        GritPatternBeforeFields {
            before_token: self.before_token(),
            pattern: self.pattern(),
        }
    }
    pub fn before_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternBefore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternBeforeFields {
    pub before_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternContains {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternContains {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternContainsFields {
        GritPatternContainsFields {
            contains_token: self.contains_token(),
            contains: self.contains(),
            grit_pattern_contains_until_clause: self.grit_pattern_contains_until_clause(),
        }
    }
    pub fn contains_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn contains(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn grit_pattern_contains_until_clause(&self) -> Option<GritPatternContainsUntilClause> {
        support::node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternContains {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternContainsFields {
    pub contains_token: SyntaxResult<SyntaxToken>,
    pub contains: SyntaxResult<MaybeCurlyGritPattern>,
    pub grit_pattern_contains_until_clause: Option<GritPatternContainsUntilClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternContainsUntilClause {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternContainsUntilClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternContainsUntilClauseFields {
        GritPatternContainsUntilClauseFields {
            until_token: self.until_token(),
            until: self.until(),
        }
    }
    pub fn until_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn until(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternContainsUntilClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternContainsUntilClauseFields {
    pub until_token: SyntaxResult<SyntaxToken>,
    pub until: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternDefinitionFields {
        GritPatternDefinitionFields {
            visibility_token: self.visibility_token(),
            pattern_token: self.pattern_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            language: self.language(),
            body: self.body(),
        }
    }
    pub fn visibility_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn pattern_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn args(&self) -> Option<GritPatternArgList> {
        support::node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn language(&self) -> Option<GritLanguageDeclaration> {
        support::node(&self.syntax, 6usize)
    }
    pub fn body(&self) -> SyntaxResult<GritPatternDefinitionBody> {
        support::required_node(&self.syntax, 7usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternDefinitionFields {
    pub visibility_token: Option<SyntaxToken>,
    pub pattern_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: Option<GritPatternArgList>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub language: Option<GritLanguageDeclaration>,
    pub body: SyntaxResult<GritPatternDefinitionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternDefinitionBody {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternDefinitionBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternDefinitionBodyFields {
        GritPatternDefinitionBodyFields {
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternDefinitionBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternDefinitionBodyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternElseClauseFields {
        GritPatternElseClauseFields {
            else_token: self.else_token(),
            else_pattern: self.else_pattern(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub else_pattern: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternIfElse {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternIfElse {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternIfElseFields {
        GritPatternIfElseFields {
            if_token: self.if_token(),
            l_paren_token: self.l_paren_token(),
            if_predicate: self.if_predicate(),
            r_paren_token: self.r_paren_token(),
            then_pattern: self.then_pattern(),
            grit_pattern_else_clause: self.grit_pattern_else_clause(),
        }
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn then_pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn grit_pattern_else_clause(&self) -> Option<GritPatternElseClause> {
        support::node(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternIfElse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternIfElseFields {
    pub if_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub if_predicate: SyntaxResult<AnyGritPredicate>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub then_pattern: SyntaxResult<MaybeCurlyGritPattern>,
    pub grit_pattern_else_clause: Option<GritPatternElseClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternIncludes {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternIncludes {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternIncludesFields {
        GritPatternIncludesFields {
            includes_token: self.includes_token(),
            includes: self.includes(),
        }
    }
    pub fn includes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn includes(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternIncludes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternIncludesFields {
    pub includes_token: SyntaxResult<SyntaxToken>,
    pub includes: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternLimit {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternLimit {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternLimitFields {
        GritPatternLimitFields {
            pattern: self.pattern(),
            limit_token: self.limit_token(),
            limit: self.limit(),
        }
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn limit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn limit(&self) -> SyntaxResult<GritIntValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternLimitFields {
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub limit_token: SyntaxResult<SyntaxToken>,
    pub limit: SyntaxResult<GritIntValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternMaybe {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternMaybe {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternMaybeFields {
        GritPatternMaybeFields {
            maybe_token: self.maybe_token(),
            pattern: self.pattern(),
        }
    }
    pub fn maybe_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternMaybe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternMaybeFields {
    pub maybe_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternNot {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternNot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternNotFields {
        GritPatternNotFields {
            grit_not: self.grit_not(),
            pattern: self.pattern(),
        }
    }
    pub fn grit_not(&self) -> SyntaxResult<GritNot> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternNot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternNotFields {
    pub grit_not: SyntaxResult<GritNot>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternOr {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternOr {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternOrFields {
        GritPatternOrFields {
            or_token: self.or_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternOr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternOrFields {
    pub or_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternOrElse {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternOrElse {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternOrElseFields {
        GritPatternOrElseFields {
            orelse_token: self.orelse_token(),
            l_curly_token: self.l_curly_token(),
            patterns: self.patterns(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn orelse_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn patterns(&self) -> GritPatternList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternOrElse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternOrElseFields {
    pub orelse_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub patterns: GritPatternList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPatternWhere {
    pub(crate) syntax: SyntaxNode,
}
impl GritPatternWhere {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPatternWhereFields {
        GritPatternWhereFields {
            pattern: self.pattern(),
            where_token: self.where_token(),
            side_condition: self.side_condition(),
        }
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn where_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn side_condition(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternWhere {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPatternWhereFields {
    pub pattern: SyntaxResult<AnyGritPattern>,
    pub where_token: SyntaxResult<SyntaxToken>,
    pub side_condition: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAccumulate {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAccumulate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAccumulateFields {
        GritPredicateAccumulateFields {
            left: self.left(),
            add_assign_token: self.add_assign_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn add_assign_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateAccumulate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateAccumulateFields {
    pub left: SyntaxResult<GritVariable>,
    pub add_assign_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAnd {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAnd {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAndFields {
        GritPredicateAndFields {
            and_token: self.and_token(),
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn and_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateAnd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateAndFields {
    pub and_token: Option<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAny {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAny {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAnyFields {
        GritPredicateAnyFields {
            any_token: self.any_token(),
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateAnyFields {
    pub any_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateAssignmentFields {
        GritPredicateAssignmentFields {
            container: self.container(),
            eq_token: self.eq_token(),
            pattern: self.pattern(),
        }
    }
    pub fn container(&self) -> SyntaxResult<AnyGritContainer> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateAssignmentFields {
    pub container: SyntaxResult<AnyGritContainer>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateCall {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateCall {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateCallFields {
        GritPredicateCallFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            named_args: self.named_args(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn named_args(&self) -> GritNamedArgList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateCallFields {
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub named_args: GritNamedArgList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateDefinitionFields {
        GritPredicateDefinitionFields {
            predicate_token: self.predicate_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn predicate_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<GritName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn args(&self) -> Option<GritPatternArgList> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn body(&self) -> SyntaxResult<GritCurlyPredicateList> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateDefinitionFields {
    pub predicate_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<GritName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: Option<GritPatternArgList>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<GritCurlyPredicateList>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateElseClauseFields {
        GritPredicateElseClauseFields {
            else_token: self.else_token(),
            else_predicate: self.else_predicate(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn else_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub else_predicate: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateEqualFields {
        GritPredicateEqualFields {
            left: self.left(),
            equality_token: self.equality_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn equality_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub equality_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateGreater {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateGreater {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateGreaterFields {
        GritPredicateGreaterFields {
            left: self.left(),
            r_angle_token: self.r_angle_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateGreater {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateGreaterFields {
    pub left: SyntaxResult<GritVariable>,
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateGreaterEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateGreaterEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateGreaterEqualFields {
        GritPredicateGreaterEqualFields {
            left: self.left(),
            greater_than_equal_token: self.greater_than_equal_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn greater_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateGreaterEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateGreaterEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub greater_than_equal_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateIfElse {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateIfElse {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateIfElseFields {
        GritPredicateIfElseFields {
            if_token: self.if_token(),
            l_paren_token: self.l_paren_token(),
            if_predicate: self.if_predicate(),
            r_paren_token: self.r_paren_token(),
            then_predicate: self.then_predicate(),
            grit_predicate_else_clause: self.grit_predicate_else_clause(),
        }
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn then_predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn grit_predicate_else_clause(&self) -> Option<GritPredicateElseClause> {
        support::node(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateIfElse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateIfElseFields {
    pub if_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub if_predicate: SyntaxResult<AnyGritPredicate>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub then_predicate: SyntaxResult<AnyGritPredicate>,
    pub grit_predicate_else_clause: Option<GritPredicateElseClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateLess {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateLess {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateLessFields {
        GritPredicateLessFields {
            left: self.left(),
            l_angle_token: self.l_angle_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateLess {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateLessFields {
    pub left: SyntaxResult<GritVariable>,
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateLessEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateLessEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateLessEqualFields {
        GritPredicateLessEqualFields {
            left: self.left(),
            less_than_equal_token: self.less_than_equal_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn less_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateLessEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateLessEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub less_than_equal_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateMatch {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateMatch {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateMatchFields {
        GritPredicateMatchFields {
            left: self.left(),
            match_token: self.match_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritPredicateMatchSubject> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn match_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateMatchFields {
    pub left: SyntaxResult<GritPredicateMatchSubject>,
    pub match_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateMaybe {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateMaybe {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateMaybeFields {
        GritPredicateMaybeFields {
            maybe_token: self.maybe_token(),
            predicate: self.predicate(),
        }
    }
    pub fn maybe_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateMaybe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateMaybeFields {
    pub maybe_token: SyntaxResult<SyntaxToken>,
    pub predicate: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateNot {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateNot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateNotFields {
        GritPredicateNotFields {
            grit_not: self.grit_not(),
            predicate: self.predicate(),
        }
    }
    pub fn grit_not(&self) -> SyntaxResult<GritNot> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn predicate(&self) -> SyntaxResult<AnyGritPredicate> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateNot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateNotFields {
    pub grit_not: SyntaxResult<GritNot>,
    pub predicate: SyntaxResult<AnyGritPredicate>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateNotEqual {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateNotEqual {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateNotEqualFields {
        GritPredicateNotEqualFields {
            left: self.left(),
            inequality_token: self.inequality_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn inequality_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateNotEqual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateNotEqualFields {
    pub left: SyntaxResult<GritVariable>,
    pub inequality_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateOr {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateOr {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateOrFields {
        GritPredicateOrFields {
            or_token: self.or_token(),
            l_curly_token: self.l_curly_token(),
            predicates: self.predicates(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn predicates(&self) -> GritPredicateList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateOr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateOrFields {
    pub or_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub predicates: GritPredicateList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateReturn {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateReturn {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateReturnFields {
        GritPredicateReturnFields {
            return_token: self.return_token(),
            pattern: self.pattern(),
        }
    }
    pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateReturnFields {
    pub return_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritPredicateRewrite {
    pub(crate) syntax: SyntaxNode,
}
impl GritPredicateRewrite {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritPredicateRewriteFields {
        GritPredicateRewriteFields {
            left: self.left(),
            annotation: self.annotation(),
            fat_arrow_token: self.fat_arrow_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<GritVariable> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn annotation(&self) -> Option<GritAnnotation> {
        support::node(&self.syntax, 1usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateRewrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritPredicateRewriteFields {
    pub left: SyntaxResult<GritVariable>,
    pub annotation: Option<GritAnnotation>,
    pub fat_arrow_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRawBacktickSnippet {
    pub(crate) syntax: SyntaxNode,
}
impl GritRawBacktickSnippet {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRawBacktickSnippetFields {
        GritRawBacktickSnippetFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritRawBacktickSnippet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritRawBacktickSnippetFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRegexPattern {
    pub(crate) syntax: SyntaxNode,
}
impl GritRegexPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRegexPatternFields {
        GritRegexPatternFields {
            regex: self.regex(),
            variables: self.variables(),
        }
    }
    pub fn regex(&self) -> SyntaxResult<GritRegex> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn variables(&self) -> Option<GritRegexPatternVariables> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritRegexPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritRegexPatternFields {
    pub regex: SyntaxResult<GritRegex>,
    pub variables: Option<GritRegexPatternVariables>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRegexPatternVariables {
    pub(crate) syntax: SyntaxNode,
}
impl GritRegexPatternVariables {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRegexPatternVariablesFields {
        GritRegexPatternVariablesFields {
            l_paren_token: self.l_paren_token(),
            grit_pattern_arg_list: self.grit_pattern_arg_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn grit_pattern_arg_list(&self) -> Option<GritPatternArgList> {
        support::node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritRegexPatternVariables {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritRegexPatternVariablesFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub grit_pattern_arg_list: Option<GritPatternArgList>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRegexValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritRegexValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRegexValueFields {
        GritRegexValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritRegexValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritRegexValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRewrite {
    pub(crate) syntax: SyntaxNode,
}
impl GritRewrite {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRewriteFields {
        GritRewriteFields {
            left: self.left(),
            annotation: self.annotation(),
            fat_arrow_token: self.fat_arrow_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn annotation(&self) -> Option<GritAnnotation> {
        support::node(&self.syntax, 1usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritRewrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritRewriteFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub annotation: Option<GritAnnotation>,
    pub fat_arrow_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritRoot {
    pub(crate) syntax: SyntaxNode,
}
impl GritRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritRootFields {
        GritRootFields {
            bom_token: self.bom_token(),
            version: self.version(),
            language: self.language(),
            definitions: self.definitions(),
            pattern: self.pattern(),
            definitions_continued: self.definitions_continued(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn version(&self) -> Option<GritVersion> {
        support::node(&self.syntax, 1usize)
    }
    pub fn language(&self) -> Option<GritLanguageDeclaration> {
        support::node(&self.syntax, 2usize)
    }
    pub fn definitions(&self) -> GritDefinitionList {
        support::list(&self.syntax, 3usize)
    }
    pub fn pattern(&self) -> Option<AnyGritPattern> {
        support::node(&self.syntax, 4usize)
    }
    pub fn definitions_continued(&self) -> GritDefinitionList {
        support::list(&self.syntax, 5usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub version: Option<GritVersion>,
    pub language: Option<GritLanguageDeclaration>,
    pub definitions: GritDefinitionList,
    pub pattern: Option<AnyGritPattern>,
    pub definitions_continued: GritDefinitionList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSequential {
    pub(crate) syntax: SyntaxNode,
}
impl GritSequential {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSequentialFields {
        GritSequentialFields {
            sequential_token: self.sequential_token(),
            l_curly_token: self.l_curly_token(),
            sequential: self.sequential(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn sequential_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn sequential(&self) -> GritSequentialList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritSequential {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritSequentialFields {
    pub sequential_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub sequential: GritSequentialList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSnippetRegexValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritSnippetRegexValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSnippetRegexValueFields {
        GritSnippetRegexValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritSnippetRegexValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritSnippetRegexValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSome {
    pub(crate) syntax: SyntaxNode,
}
impl GritSome {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSomeFields {
        GritSomeFields {
            some_token: self.some_token(),
            pattern: self.pattern(),
        }
    }
    pub fn some_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritSome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritSomeFields {
    pub some_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl GritStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritStringValueFields {
        GritStringValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritStringValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritSubOperation {
    pub(crate) syntax: SyntaxNode,
}
impl GritSubOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritSubOperationFields {
        GritSubOperationFields {
            left: self.left(),
            minus_token: self.minus_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn minus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyGritPattern> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritSubOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritSubOperationFields {
    pub left: SyntaxResult<AnyGritPattern>,
    pub minus_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritUndefined {
    pub(crate) syntax: SyntaxNode,
}
impl GritUndefined {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritUndefinedFields {
        GritUndefinedFields {
            undefined_token: self.undefined_token(),
        }
    }
    pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritUndefined {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritUndefinedFields {
    pub undefined_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritUnderscore {
    pub(crate) syntax: SyntaxNode,
}
impl GritUnderscore {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritUnderscoreFields {
        GritUnderscoreFields {
            dollar_underscore_token: self.dollar_underscore_token(),
        }
    }
    pub fn dollar_underscore_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritUnderscore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritUnderscoreFields {
    pub dollar_underscore_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritVariable {
    pub(crate) syntax: SyntaxNode,
}
impl GritVariable {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritVariableFields {
        GritVariableFields {
            grit_variable_token: self.grit_variable_token(),
        }
    }
    pub fn grit_variable_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritVariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritVariableFields {
    pub grit_variable_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritVersion {
    pub(crate) syntax: SyntaxNode,
}
impl GritVersion {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritVersionFields {
        GritVersionFields {
            engine_token: self.engine_token(),
            biome_token: self.biome_token(),
            l_paren_token: self.l_paren_token(),
            grit_double_value: self.grit_double_value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn engine_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn biome_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn grit_double_value(&self) -> SyntaxResult<GritDoubleValue> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritVersionFields {
    pub engine_token: SyntaxResult<SyntaxToken>,
    pub biome_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub grit_double_value: SyntaxResult<GritDoubleValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GritWithin {
    pub(crate) syntax: SyntaxNode,
}
impl GritWithin {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> GritWithinFields {
        GritWithinFields {
            within_token: self.within_token(),
            pattern: self.pattern(),
        }
    }
    pub fn within_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<MaybeCurlyGritPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritWithin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritWithinFields {
    pub within_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<MaybeCurlyGritPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyGritContainer {
    GritListAccessor(GritListAccessor),
    GritMapAccessor(GritMapAccessor),
    GritVariable(GritVariable),
}
impl AnyGritContainer {
    pub fn as_grit_list_accessor(&self) -> Option<&GritListAccessor> {
        match &self {
            AnyGritContainer::GritListAccessor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map_accessor(&self) -> Option<&GritMapAccessor> {
        match &self {
            AnyGritContainer::GritMapAccessor(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_variable(&self) -> Option<&GritVariable> {
        match &self {
            AnyGritContainer::GritVariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyGritDefinition {
    GritBogusDefinition(GritBogusDefinition),
    GritFunctionDefinition(GritFunctionDefinition),
    GritPatternDefinition(GritPatternDefinition),
    GritPredicateDefinition(GritPredicateDefinition),
}
impl AnyGritDefinition {
    pub fn as_grit_bogus_definition(&self) -> Option<&GritBogusDefinition> {
        match &self {
            AnyGritDefinition::GritBogusDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_function_definition(&self) -> Option<&GritFunctionDefinition> {
        match &self {
            AnyGritDefinition::GritFunctionDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_pattern_definition(&self) -> Option<&GritPatternDefinition> {
        match &self {
            AnyGritDefinition::GritPatternDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_predicate_definition(&self) -> Option<&GritPredicateDefinition> {
        match &self {
            AnyGritDefinition::GritPredicateDefinition(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyGritListPattern {
    AnyGritPattern(AnyGritPattern),
    GritDotdotdot(GritDotdotdot),
}
impl AnyGritListPattern {
    pub fn as_any_grit_pattern(&self) -> Option<&AnyGritPattern> {
        match &self {
            AnyGritListPattern::AnyGritPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_dotdotdot(&self) -> Option<&GritDotdotdot> {
        match &self {
            AnyGritListPattern::GritDotdotdot(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyGritLiteral {
    GritBogusLiteral(GritBogusLiteral),
    GritBooleanValue(GritBooleanValue),
    GritCodeSnippet(GritCodeSnippet),
    GritDoubleValue(GritDoubleValue),
    GritIntValue(GritIntValue),
    GritList(GritList),
    GritMap(GritMap),
    GritStringValue(GritStringValue),
    GritUndefined(GritUndefined),
}
impl AnyGritLiteral {
    pub fn as_grit_bogus_literal(&self) -> Option<&GritBogusLiteral> {
        match &self {
            AnyGritLiteral::GritBogusLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_boolean_value(&self) -> Option<&GritBooleanValue> {
        match &self {
            AnyGritLiteral::GritBooleanValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_code_snippet(&self) -> Option<&GritCodeSnippet> {
        match &self {
            AnyGritLiteral::GritCodeSnippet(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_double_value(&self) -> Option<&GritDoubleValue> {
        match &self {
            AnyGritLiteral::GritDoubleValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_int_value(&self) -> Option<&GritIntValue> {
        match &self {
            AnyGritLiteral::GritIntValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_list(&self) -> Option<&GritList> {
        match &self {
            AnyGritLiteral::GritList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map(&self) -> Option<&GritMap> {
        match &self {
            AnyGritLiteral::GritMap(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_string_value(&self) -> Option<&GritStringValue> {
        match &self {
            AnyGritLiteral::GritStringValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_undefined(&self) -> Option<&GritUndefined> {
        match &self {
            AnyGritLiteral::GritUndefined(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyGritNamedArg {
    GritBogusNamedArg(GritBogusNamedArg),
    GritNamedArg(GritNamedArg),
    GritNamedArgWithDefault(GritNamedArgWithDefault),
}
impl AnyGritNamedArg {
    pub fn as_grit_bogus_named_arg(&self) -> Option<&GritBogusNamedArg> {
        match &self {
            AnyGritNamedArg::GritBogusNamedArg(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_named_arg(&self) -> Option<&GritNamedArg> {
        match &self {
            AnyGritNamedArg::GritNamedArg(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_named_arg_with_default(&self) -> Option<&GritNamedArgWithDefault> {
        match &self {
            AnyGritNamedArg::GritNamedArgWithDefault(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritCodeSnippetSource {
    GritBacktickSnippet(GritBacktickSnippet),
    GritLanguageSpecificSnippet(GritLanguageSpecificSnippet),
    GritRawBacktickSnippet(GritRawBacktickSnippet),
}
impl GritCodeSnippetSource {
    pub fn as_grit_backtick_snippet(&self) -> Option<&GritBacktickSnippet> {
        match &self {
            GritCodeSnippetSource::GritBacktickSnippet(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_language_specific_snippet(&self) -> Option<&GritLanguageSpecificSnippet> {
        match &self {
            GritCodeSnippetSource::GritLanguageSpecificSnippet(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_raw_backtick_snippet(&self) -> Option<&GritRawBacktickSnippet> {
        match &self {
            GritCodeSnippetSource::GritRawBacktickSnippet(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritListAccessorSubject {
    AnyGritContainer(AnyGritContainer),
    GritList(GritList),
}
impl GritListAccessorSubject {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            GritListAccessorSubject::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_list(&self) -> Option<&GritList> {
        match &self {
            GritListAccessorSubject::GritList(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritListIndex {
    AnyGritContainer(AnyGritContainer),
    GritIntValue(GritIntValue),
    GritNegativeIntValue(GritNegativeIntValue),
}
impl GritListIndex {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            GritListIndex::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_int_value(&self) -> Option<&GritIntValue> {
        match &self {
            GritListIndex::GritIntValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_negative_int_value(&self) -> Option<&GritNegativeIntValue> {
        match &self {
            GritListIndex::GritNegativeIntValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritMapAccessorSubject {
    AnyGritContainer(AnyGritContainer),
    GritMap(GritMap),
}
impl GritMapAccessorSubject {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            GritMapAccessorSubject::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_map(&self) -> Option<&GritMap> {
        match &self {
            GritMapAccessorSubject::GritMap(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritMapKey {
    GritName(GritName),
    GritVariable(GritVariable),
}
impl GritMapKey {
    pub fn as_grit_name(&self) -> Option<&GritName> {
        match &self {
            GritMapKey::GritName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_variable(&self) -> Option<&GritVariable> {
        match &self {
            GritMapKey::GritVariable(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritPredicateMatchSubject {
    AnyGritContainer(AnyGritContainer),
    AnyGritLiteral(AnyGritLiteral),
}
impl GritPredicateMatchSubject {
    pub fn as_any_grit_container(&self) -> Option<&AnyGritContainer> {
        match &self {
            GritPredicateMatchSubject::AnyGritContainer(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_any_grit_literal(&self) -> Option<&AnyGritLiteral> {
        match &self {
            GritPredicateMatchSubject::AnyGritLiteral(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GritRegex {
    GritRegexValue(GritRegexValue),
    GritSnippetRegexValue(GritSnippetRegexValue),
}
impl GritRegex {
    pub fn as_grit_regex_value(&self) -> Option<&GritRegexValue> {
        match &self {
            GritRegex::GritRegexValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_grit_snippet_regex_value(&self) -> Option<&GritSnippetRegexValue> {
        match &self {
            GritRegex::GritSnippetRegexValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MaybeCurlyGritPattern {
    AnyGritPattern(AnyGritPattern),
    CurlyGritPattern(CurlyGritPattern),
}
impl MaybeCurlyGritPattern {
    pub fn as_any_grit_pattern(&self) -> Option<&AnyGritPattern> {
        match &self {
            MaybeCurlyGritPattern::AnyGritPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_curly_grit_pattern(&self) -> Option<&CurlyGritPattern> {
        match &self {
            MaybeCurlyGritPattern::CurlyGritPattern(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for AnyGritPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ANY_GRIT_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ANY_GRIT_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AnyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyGritPattern")
            .field(
                "any_grit_literal",
                &support::DebugSyntaxResult(self.any_grit_literal()),
            )
            .field(
                "grit_pattern_not",
                &support::DebugSyntaxResult(self.grit_pattern_not()),
            )
            .field(
                "grit_pattern_or",
                &support::DebugSyntaxResult(self.grit_pattern_or()),
            )
            .field(
                "grit_pattern_or_else",
                &support::DebugSyntaxResult(self.grit_pattern_or_else()),
            )
            .field(
                "grit_pattern_any",
                &support::DebugSyntaxResult(self.grit_pattern_any()),
            )
            .field(
                "grit_pattern_and",
                &support::DebugSyntaxResult(self.grit_pattern_and()),
            )
            .field(
                "grit_pattern_maybe",
                &support::DebugSyntaxResult(self.grit_pattern_maybe()),
            )
            .field(
                "grit_pattern_if_else",
                &support::DebugSyntaxResult(self.grit_pattern_if_else()),
            )
            .field(
                "grit_pattern_contains",
                &support::DebugSyntaxResult(self.grit_pattern_contains()),
            )
            .field(
                "grit_pattern_includes",
                &support::DebugSyntaxResult(self.grit_pattern_includes()),
            )
            .field(
                "grit_pattern_after",
                &support::DebugSyntaxResult(self.grit_pattern_after()),
            )
            .field(
                "grit_pattern_before",
                &support::DebugSyntaxResult(self.grit_pattern_before()),
            )
            .field(
                "grit_within",
                &support::DebugSyntaxResult(self.grit_within()),
            )
            .field(
                "grit_bubble",
                &support::DebugSyntaxResult(self.grit_bubble()),
            )
            .field(
                "grit_node_like",
                &support::DebugSyntaxResult(self.grit_node_like()),
            )
            .field(
                "grit_map_accessor",
                &support::DebugSyntaxResult(self.grit_map_accessor()),
            )
            .field(
                "grit_list_accessor",
                &support::DebugSyntaxResult(self.grit_list_accessor()),
            )
            .field("grit_dot", &support::DebugSyntaxResult(self.grit_dot()))
            .field("grit_some", &support::DebugSyntaxResult(self.grit_some()))
            .field("grit_every", &support::DebugSyntaxResult(self.grit_every()))
            .field(
                "grit_underscore",
                &support::DebugSyntaxResult(self.grit_underscore()),
            )
            .field(
                "grit_variable",
                &support::DebugSyntaxResult(self.grit_variable()),
            )
            .field(
                "grit_regex_pattern",
                &support::DebugSyntaxResult(self.grit_regex_pattern()),
            )
            .field(
                "grit_pattern_as",
                &support::DebugSyntaxResult(self.grit_pattern_as()),
            )
            .field(
                "grit_pattern_limit",
                &support::DebugSyntaxResult(self.grit_pattern_limit()),
            )
            .field(
                "grit_assignment_as_pattern",
                &support::DebugSyntaxResult(self.grit_assignment_as_pattern()),
            )
            .field(
                "grit_pattern_accumulate",
                &support::DebugSyntaxResult(self.grit_pattern_accumulate()),
            )
            .field(
                "grit_rewrite",
                &support::DebugSyntaxResult(self.grit_rewrite()),
            )
            .field("grit_like", &support::DebugSyntaxResult(self.grit_like()))
            .field(
                "grit_pattern_where",
                &support::DebugSyntaxResult(self.grit_pattern_where()),
            )
            .field(
                "grit_mul_operation",
                &support::DebugSyntaxResult(self.grit_mul_operation()),
            )
            .field(
                "grit_div_operation",
                &support::DebugSyntaxResult(self.grit_div_operation()),
            )
            .field(
                "grit_mod_operation",
                &support::DebugSyntaxResult(self.grit_mod_operation()),
            )
            .field(
                "grit_add_operation",
                &support::DebugSyntaxResult(self.grit_add_operation()),
            )
            .field(
                "grit_sub_operation",
                &support::DebugSyntaxResult(self.grit_sub_operation()),
            )
            .field(
                "grit_sequential",
                &support::DebugSyntaxResult(self.grit_sequential()),
            )
            .field("grit_files", &support::DebugSyntaxResult(self.grit_files()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "any_grit_pattern",
                &support::DebugSyntaxResult(self.any_grit_pattern()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "grit_bogus_pattern",
                &support::DebugSyntaxResult(self.grit_bogus_pattern()),
            )
            .finish()
    }
}
impl From<AnyGritPattern> for SyntaxNode {
    fn from(n: AnyGritPattern) -> SyntaxNode {
        n.syntax
    }
}
impl From<AnyGritPattern> for SyntaxElement {
    fn from(n: AnyGritPattern) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for AnyGritPredicate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ANY_GRIT_PREDICATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ANY_GRIT_PREDICATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for AnyGritPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyGritPredicate")
            .field(
                "grit_predicate_not",
                &support::DebugSyntaxResult(self.grit_predicate_not()),
            )
            .field(
                "grit_predicate_maybe",
                &support::DebugSyntaxResult(self.grit_predicate_maybe()),
            )
            .field(
                "grit_predicate_and",
                &support::DebugSyntaxResult(self.grit_predicate_and()),
            )
            .field(
                "grit_predicate_or",
                &support::DebugSyntaxResult(self.grit_predicate_or()),
            )
            .field(
                "grit_predicate_any",
                &support::DebugSyntaxResult(self.grit_predicate_any()),
            )
            .field(
                "grit_predicate_if_else",
                &support::DebugSyntaxResult(self.grit_predicate_if_else()),
            )
            .field(
                "grit_predicate_assignment",
                &support::DebugSyntaxResult(self.grit_predicate_assignment()),
            )
            .field(
                "grit_predicate_accumulate",
                &support::DebugSyntaxResult(self.grit_predicate_accumulate()),
            )
            .field(
                "grit_predicate_rewrite",
                &support::DebugSyntaxResult(self.grit_predicate_rewrite()),
            )
            .field(
                "grit_predicate_greater",
                &support::DebugSyntaxResult(self.grit_predicate_greater()),
            )
            .field(
                "grit_predicate_less",
                &support::DebugSyntaxResult(self.grit_predicate_less()),
            )
            .field(
                "grit_predicate_greater_equal",
                &support::DebugSyntaxResult(self.grit_predicate_greater_equal()),
            )
            .field(
                "grit_predicate_less_equal",
                &support::DebugSyntaxResult(self.grit_predicate_less_equal()),
            )
            .field(
                "grit_predicate_not_equal",
                &support::DebugSyntaxResult(self.grit_predicate_not_equal()),
            )
            .field(
                "grit_predicate_equal",
                &support::DebugSyntaxResult(self.grit_predicate_equal()),
            )
            .field(
                "grit_predicate_match",
                &support::DebugSyntaxResult(self.grit_predicate_match()),
            )
            .field(
                "grit_predicate_call",
                &support::DebugSyntaxResult(self.grit_predicate_call()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "any_grit_predicate",
                &support::DebugSyntaxResult(self.any_grit_predicate()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "grit_boolean_value",
                &support::DebugSyntaxResult(self.grit_boolean_value()),
            )
            .field(
                "grit_predicate_return",
                &support::DebugSyntaxResult(self.grit_predicate_return()),
            )
            .field(
                "grit_bogus_predicate",
                &support::DebugSyntaxResult(self.grit_bogus_predicate()),
            )
            .finish()
    }
}
impl From<AnyGritPredicate> for SyntaxNode {
    fn from(n: AnyGritPredicate) -> SyntaxNode {
        n.syntax
    }
}
impl From<AnyGritPredicate> for SyntaxElement {
    fn from(n: AnyGritPredicate) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for CurlyGritPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CURLY_GRIT_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CURLY_GRIT_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for CurlyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CurlyGritPattern")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field(
                "any_grit_pattern",
                &support::DebugSyntaxResult(self.any_grit_pattern()),
            )
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CurlyGritPattern> for SyntaxNode {
    fn from(n: CurlyGritPattern) -> SyntaxNode {
        n.syntax
    }
}
impl From<CurlyGritPattern> for SyntaxElement {
    fn from(n: CurlyGritPattern) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritAddOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ADD_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ADD_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritAddOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritAddOperation")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("plus_token", &support::DebugSyntaxResult(self.plus_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritAddOperation> for SyntaxNode {
    fn from(n: GritAddOperation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritAddOperation> for SyntaxElement {
    fn from(n: GritAddOperation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritAnnotation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ANNOTATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ANNOTATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritAnnotation")
            .field(
                "grit_annotation_token",
                &support::DebugSyntaxResult(self.grit_annotation_token()),
            )
            .finish()
    }
}
impl From<GritAnnotation> for SyntaxNode {
    fn from(n: GritAnnotation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritAnnotation> for SyntaxElement {
    fn from(n: GritAnnotation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritAssignmentAsPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ASSIGNMENT_AS_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ASSIGNMENT_AS_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritAssignmentAsPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritAssignmentAsPattern")
            .field("container", &support::DebugSyntaxResult(self.container()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritAssignmentAsPattern> for SyntaxNode {
    fn from(n: GritAssignmentAsPattern) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritAssignmentAsPattern> for SyntaxElement {
    fn from(n: GritAssignmentAsPattern) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritBacktickSnippet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BACKTICK_SNIPPET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BACKTICK_SNIPPET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBacktickSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBacktickSnippet")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritBacktickSnippet> for SyntaxNode {
    fn from(n: GritBacktickSnippet) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBacktickSnippet> for SyntaxElement {
    fn from(n: GritBacktickSnippet) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritBooleanValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOOLEAN_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOOLEAN_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBooleanValue")
            .field("true_token", &support::DebugSyntaxResult(self.true_token()))
            .field(
                "false_token",
                &support::DebugSyntaxResult(self.false_token()),
            )
            .finish()
    }
}
impl From<GritBooleanValue> for SyntaxNode {
    fn from(n: GritBooleanValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBooleanValue> for SyntaxElement {
    fn from(n: GritBooleanValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritBubble {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BUBBLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BUBBLE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBubble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBubble")
            .field(
                "bubble_token",
                &support::DebugSyntaxResult(self.bubble_token()),
            )
            .field(
                "variables",
                &support::DebugOptionalElement(self.variables()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritBubble> for SyntaxNode {
    fn from(n: GritBubble) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBubble> for SyntaxElement {
    fn from(n: GritBubble) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritBubbleScope {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BUBBLE_SCOPE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BUBBLE_SCOPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBubbleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBubbleScope")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("grit_variable_list", &self.grit_variable_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritBubbleScope> for SyntaxNode {
    fn from(n: GritBubbleScope) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBubbleScope> for SyntaxElement {
    fn from(n: GritBubbleScope) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritCodeSnippet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_CODE_SNIPPET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_CODE_SNIPPET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritCodeSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritCodeSnippet")
            .field("source", &support::DebugSyntaxResult(self.source()))
            .finish()
    }
}
impl From<GritCodeSnippet> for SyntaxNode {
    fn from(n: GritCodeSnippet) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritCodeSnippet> for SyntaxElement {
    fn from(n: GritCodeSnippet) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritCurlyPredicateList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_CURLY_PREDICATE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_CURLY_PREDICATE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritCurlyPredicateList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritCurlyPredicateList")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("predicates", &self.predicates())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritCurlyPredicateList> for SyntaxNode {
    fn from(n: GritCurlyPredicateList) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritCurlyPredicateList> for SyntaxElement {
    fn from(n: GritCurlyPredicateList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritDivOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DIV_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DIV_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDivOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritDivOperation")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "slash_token",
                &support::DebugSyntaxResult(self.slash_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritDivOperation> for SyntaxNode {
    fn from(n: GritDivOperation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritDivOperation> for SyntaxElement {
    fn from(n: GritDivOperation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritDot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritDot")
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .finish()
    }
}
impl From<GritDot> for SyntaxNode {
    fn from(n: GritDot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritDot> for SyntaxElement {
    fn from(n: GritDot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritDotdotdot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DOTDOTDOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DOTDOTDOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDotdotdot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritDotdotdot")
            .field(
                "dollar_dotdotdot_token",
                &support::DebugSyntaxResult(self.dollar_dotdotdot_token()),
            )
            .field(
                "maybe_curly_grit_pattern",
                &support::DebugOptionalElement(self.maybe_curly_grit_pattern()),
            )
            .finish()
    }
}
impl From<GritDotdotdot> for SyntaxNode {
    fn from(n: GritDotdotdot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritDotdotdot> for SyntaxElement {
    fn from(n: GritDotdotdot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritDoubleValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DOUBLE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DOUBLE_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritDoubleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritDoubleValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritDoubleValue> for SyntaxNode {
    fn from(n: GritDoubleValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritDoubleValue> for SyntaxElement {
    fn from(n: GritDoubleValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritEvery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_EVERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_EVERY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritEvery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritEvery")
            .field(
                "every_token",
                &support::DebugSyntaxResult(self.every_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritEvery> for SyntaxNode {
    fn from(n: GritEvery) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritEvery> for SyntaxElement {
    fn from(n: GritEvery) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritFiles {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_FILES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_FILES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritFiles")
            .field(
                "multifile_token",
                &support::DebugSyntaxResult(self.multifile_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("files", &self.files())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritFiles> for SyntaxNode {
    fn from(n: GritFiles) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritFiles> for SyntaxElement {
    fn from(n: GritFiles) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritFunctionDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_FUNCTION_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_FUNCTION_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritFunctionDefinition")
            .field(
                "function_token",
                &support::DebugSyntaxResult(self.function_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("args", &self.args())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<GritFunctionDefinition> for SyntaxNode {
    fn from(n: GritFunctionDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritFunctionDefinition> for SyntaxElement {
    fn from(n: GritFunctionDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritIntValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_INT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_INT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritIntValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritIntValue> for SyntaxNode {
    fn from(n: GritIntValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritIntValue> for SyntaxElement {
    fn from(n: GritIntValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLanguageDeclaration")
            .field(
                "language_token",
                &support::DebugSyntaxResult(self.language_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("flavor", &support::DebugOptionalElement(self.flavor()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<GritLanguageDeclaration> for SyntaxNode {
    fn from(n: GritLanguageDeclaration) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLanguageDeclaration> for SyntaxElement {
    fn from(n: GritLanguageDeclaration) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageFlavor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_FLAVOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_FLAVOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLanguageFlavor")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "grit_language_flavor_list",
                &self.grit_language_flavor_list(),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritLanguageFlavor> for SyntaxNode {
    fn from(n: GritLanguageFlavor) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLanguageFlavor> for SyntaxElement {
    fn from(n: GritLanguageFlavor) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageFlavorKind {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_FLAVOR_KIND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_FLAVOR_KIND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLanguageFlavorKind")
            .field(
                "flavor_kind",
                &support::DebugSyntaxResult(self.flavor_kind()),
            )
            .finish()
    }
}
impl From<GritLanguageFlavorKind> for SyntaxNode {
    fn from(n: GritLanguageFlavorKind) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLanguageFlavorKind> for SyntaxElement {
    fn from(n: GritLanguageFlavorKind) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLanguageName")
            .field("js_token", &support::DebugSyntaxResult(self.js_token()))
            .field("css_token", &support::DebugSyntaxResult(self.css_token()))
            .field("json_token", &support::DebugSyntaxResult(self.json_token()))
            .field("grit_token", &support::DebugSyntaxResult(self.grit_token()))
            .field("html_token", &support::DebugSyntaxResult(self.html_token()))
            .finish()
    }
}
impl From<GritLanguageName> for SyntaxNode {
    fn from(n: GritLanguageName) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLanguageName> for SyntaxElement {
    fn from(n: GritLanguageName) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLanguageSpecificSnippet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_SPECIFIC_SNIPPET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_SPECIFIC_SNIPPET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLanguageSpecificSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLanguageSpecificSnippet")
            .field("language", &support::DebugSyntaxResult(self.language()))
            .field(
                "snippet_token",
                &support::DebugSyntaxResult(self.snippet_token()),
            )
            .finish()
    }
}
impl From<GritLanguageSpecificSnippet> for SyntaxNode {
    fn from(n: GritLanguageSpecificSnippet) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLanguageSpecificSnippet> for SyntaxElement {
    fn from(n: GritLanguageSpecificSnippet) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLike {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIKE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIKE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLike")
            .field("like_token", &support::DebugSyntaxResult(self.like_token()))
            .field(
                "grit_like_threshold",
                &support::DebugOptionalElement(self.grit_like_threshold()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("example", &support::DebugSyntaxResult(self.example()))
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritLike> for SyntaxNode {
    fn from(n: GritLike) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLike> for SyntaxElement {
    fn from(n: GritLike) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritLikeThreshold {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIKE_THRESHOLD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIKE_THRESHOLD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritLikeThreshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritLikeThreshold")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("threshold", &support::DebugSyntaxResult(self.threshold()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritLikeThreshold> for SyntaxNode {
    fn from(n: GritLikeThreshold) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritLikeThreshold> for SyntaxElement {
    fn from(n: GritLikeThreshold) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritList")
            .field(
                "grit_name",
                &support::DebugOptionalElement(self.grit_name()),
            )
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("patterns", &self.patterns())
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<GritList> for SyntaxNode {
    fn from(n: GritList) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritList> for SyntaxElement {
    fn from(n: GritList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritListAccessor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIST_ACCESSOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIST_ACCESSOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritListAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritListAccessor")
            .field("list", &support::DebugSyntaxResult(self.list()))
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("index", &support::DebugSyntaxResult(self.index()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<GritListAccessor> for SyntaxNode {
    fn from(n: GritListAccessor) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritListAccessor> for SyntaxElement {
    fn from(n: GritListAccessor) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritMap {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritMap")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("elements", &self.elements())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritMap> for SyntaxNode {
    fn from(n: GritMap) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritMap> for SyntaxElement {
    fn from(n: GritMap) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritMapAccessor {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP_ACCESSOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP_ACCESSOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMapAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritMapAccessor")
            .field("map", &support::DebugSyntaxResult(self.map()))
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("key", &support::DebugSyntaxResult(self.key()))
            .finish()
    }
}
impl From<GritMapAccessor> for SyntaxNode {
    fn from(n: GritMapAccessor) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritMapAccessor> for SyntaxElement {
    fn from(n: GritMapAccessor) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritMapElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritMapElement")
            .field("key", &support::DebugSyntaxResult(self.key()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<GritMapElement> for SyntaxNode {
    fn from(n: GritMapElement) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritMapElement> for SyntaxElement {
    fn from(n: GritMapElement) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritModOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MOD_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MOD_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritModOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritModOperation")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "remainder_token",
                &support::DebugSyntaxResult(self.remainder_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritModOperation> for SyntaxNode {
    fn from(n: GritModOperation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritModOperation> for SyntaxElement {
    fn from(n: GritModOperation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritMulOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MUL_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MUL_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritMulOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritMulOperation")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritMulOperation> for SyntaxNode {
    fn from(n: GritMulOperation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritMulOperation> for SyntaxElement {
    fn from(n: GritMulOperation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritName")
            .field(
                "grit_name_token",
                &support::DebugSyntaxResult(self.grit_name_token()),
            )
            .finish()
    }
}
impl From<GritName> for SyntaxNode {
    fn from(n: GritName) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritName> for SyntaxElement {
    fn from(n: GritName) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritNamedArg {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAMED_ARG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAMED_ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritNamedArg")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<GritNamedArg> for SyntaxNode {
    fn from(n: GritNamedArg) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritNamedArg> for SyntaxElement {
    fn from(n: GritNamedArg) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritNamedArgWithDefault {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAMED_ARG_WITH_DEFAULT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAMED_ARG_WITH_DEFAULT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNamedArgWithDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritNamedArgWithDefault")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritNamedArgWithDefault> for SyntaxNode {
    fn from(n: GritNamedArgWithDefault) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritNamedArgWithDefault> for SyntaxElement {
    fn from(n: GritNamedArgWithDefault) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritNegativeIntValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NEGATIVE_INT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NEGATIVE_INT_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNegativeIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritNegativeIntValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritNegativeIntValue> for SyntaxNode {
    fn from(n: GritNegativeIntValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritNegativeIntValue> for SyntaxElement {
    fn from(n: GritNegativeIntValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritNodeLike {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NODE_LIKE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NODE_LIKE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNodeLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritNodeLike")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("named_args", &self.named_args())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritNodeLike> for SyntaxNode {
    fn from(n: GritNodeLike) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritNodeLike> for SyntaxElement {
    fn from(n: GritNodeLike) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritNot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritNot")
            .field("not_token", &support::DebugSyntaxResult(self.not_token()))
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .finish()
    }
}
impl From<GritNot> for SyntaxNode {
    fn from(n: GritNot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritNot> for SyntaxElement {
    fn from(n: GritNot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAccumulate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ACCUMULATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ACCUMULATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternAccumulate")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "add_assign_token",
                &support::DebugSyntaxResult(self.add_assign_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPatternAccumulate> for SyntaxNode {
    fn from(n: GritPatternAccumulate) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternAccumulate> for SyntaxElement {
    fn from(n: GritPatternAccumulate) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAfter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_AFTER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_AFTER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternAfter")
            .field(
                "after_token",
                &support::DebugSyntaxResult(self.after_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritPatternAfter> for SyntaxNode {
    fn from(n: GritPatternAfter) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternAfter> for SyntaxElement {
    fn from(n: GritPatternAfter) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAnd {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_AND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_AND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternAnd")
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("patterns", &self.patterns())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPatternAnd> for SyntaxNode {
    fn from(n: GritPatternAnd) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternAnd> for SyntaxElement {
    fn from(n: GritPatternAnd) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAny {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ANY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ANY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternAny")
            .field("any_token", &support::DebugSyntaxResult(self.any_token()))
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("patterns", &self.patterns())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPatternAny> for SyntaxNode {
    fn from(n: GritPatternAny) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternAny> for SyntaxElement {
    fn from(n: GritPatternAny) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternArgList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ARG_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ARG_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternArgList")
            .field("grit_variable_list", &self.grit_variable_list())
            .finish()
    }
}
impl From<GritPatternArgList> for SyntaxNode {
    fn from(n: GritPatternArgList) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternArgList> for SyntaxElement {
    fn from(n: GritPatternArgList) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternAs {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_AS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_AS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternAs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternAs")
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field("variable", &support::DebugSyntaxResult(self.variable()))
            .finish()
    }
}
impl From<GritPatternAs> for SyntaxNode {
    fn from(n: GritPatternAs) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternAs> for SyntaxElement {
    fn from(n: GritPatternAs) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternBefore {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_BEFORE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_BEFORE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternBefore")
            .field(
                "before_token",
                &support::DebugSyntaxResult(self.before_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritPatternBefore> for SyntaxNode {
    fn from(n: GritPatternBefore) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternBefore> for SyntaxElement {
    fn from(n: GritPatternBefore) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternContains {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_CONTAINS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_CONTAINS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternContains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternContains")
            .field(
                "contains_token",
                &support::DebugSyntaxResult(self.contains_token()),
            )
            .field("contains", &support::DebugSyntaxResult(self.contains()))
            .field(
                "grit_pattern_contains_until_clause",
                &support::DebugOptionalElement(self.grit_pattern_contains_until_clause()),
            )
            .finish()
    }
}
impl From<GritPatternContains> for SyntaxNode {
    fn from(n: GritPatternContains) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternContains> for SyntaxElement {
    fn from(n: GritPatternContains) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternContainsUntilClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_CONTAINS_UNTIL_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_CONTAINS_UNTIL_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternContainsUntilClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternContainsUntilClause")
            .field(
                "until_token",
                &support::DebugSyntaxResult(self.until_token()),
            )
            .field("until", &support::DebugSyntaxResult(self.until()))
            .finish()
    }
}
impl From<GritPatternContainsUntilClause> for SyntaxNode {
    fn from(n: GritPatternContainsUntilClause) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternContainsUntilClause> for SyntaxElement {
    fn from(n: GritPatternContainsUntilClause) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternDefinition")
            .field(
                "visibility_token",
                &support::DebugOptionalElement(self.visibility_token()),
            )
            .field(
                "pattern_token",
                &support::DebugSyntaxResult(self.pattern_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("args", &support::DebugOptionalElement(self.args()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("language", &support::DebugOptionalElement(self.language()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<GritPatternDefinition> for SyntaxNode {
    fn from(n: GritPatternDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternDefinition> for SyntaxElement {
    fn from(n: GritPatternDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternDefinitionBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_DEFINITION_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_DEFINITION_BODY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternDefinitionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternDefinitionBody")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("patterns", &self.patterns())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPatternDefinitionBody> for SyntaxNode {
    fn from(n: GritPatternDefinitionBody) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternDefinitionBody> for SyntaxElement {
    fn from(n: GritPatternDefinitionBody) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_ELSE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternElseClause")
            .field("else_token", &support::DebugSyntaxResult(self.else_token()))
            .field(
                "else_pattern",
                &support::DebugSyntaxResult(self.else_pattern()),
            )
            .finish()
    }
}
impl From<GritPatternElseClause> for SyntaxNode {
    fn from(n: GritPatternElseClause) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternElseClause> for SyntaxElement {
    fn from(n: GritPatternElseClause) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternIfElse {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_IF_ELSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_IF_ELSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternIfElse")
            .field("if_token", &support::DebugSyntaxResult(self.if_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "if_predicate",
                &support::DebugSyntaxResult(self.if_predicate()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "then_pattern",
                &support::DebugSyntaxResult(self.then_pattern()),
            )
            .field(
                "grit_pattern_else_clause",
                &support::DebugOptionalElement(self.grit_pattern_else_clause()),
            )
            .finish()
    }
}
impl From<GritPatternIfElse> for SyntaxNode {
    fn from(n: GritPatternIfElse) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternIfElse> for SyntaxElement {
    fn from(n: GritPatternIfElse) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternIncludes {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_INCLUDES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_INCLUDES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternIncludes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternIncludes")
            .field(
                "includes_token",
                &support::DebugSyntaxResult(self.includes_token()),
            )
            .field("includes", &support::DebugSyntaxResult(self.includes()))
            .finish()
    }
}
impl From<GritPatternIncludes> for SyntaxNode {
    fn from(n: GritPatternIncludes) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternIncludes> for SyntaxElement {
    fn from(n: GritPatternIncludes) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternLimit {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_LIMIT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_LIMIT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternLimit")
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field(
                "limit_token",
                &support::DebugSyntaxResult(self.limit_token()),
            )
            .field("limit", &support::DebugSyntaxResult(self.limit()))
            .finish()
    }
}
impl From<GritPatternLimit> for SyntaxNode {
    fn from(n: GritPatternLimit) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternLimit> for SyntaxElement {
    fn from(n: GritPatternLimit) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternMaybe {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_MAYBE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_MAYBE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternMaybe")
            .field(
                "maybe_token",
                &support::DebugSyntaxResult(self.maybe_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritPatternMaybe> for SyntaxNode {
    fn from(n: GritPatternMaybe) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternMaybe> for SyntaxElement {
    fn from(n: GritPatternMaybe) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternNot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_NOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_NOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternNot")
            .field("grit_not", &support::DebugSyntaxResult(self.grit_not()))
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritPatternNot> for SyntaxNode {
    fn from(n: GritPatternNot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternNot> for SyntaxElement {
    fn from(n: GritPatternNot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternOr {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_OR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_OR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternOr")
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("patterns", &self.patterns())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPatternOr> for SyntaxNode {
    fn from(n: GritPatternOr) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternOr> for SyntaxElement {
    fn from(n: GritPatternOr) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternOrElse {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_OR_ELSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_OR_ELSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternOrElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternOrElse")
            .field(
                "orelse_token",
                &support::DebugSyntaxResult(self.orelse_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("patterns", &self.patterns())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPatternOrElse> for SyntaxNode {
    fn from(n: GritPatternOrElse) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternOrElse> for SyntaxElement {
    fn from(n: GritPatternOrElse) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPatternWhere {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_WHERE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_WHERE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPatternWhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPatternWhere")
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field(
                "where_token",
                &support::DebugSyntaxResult(self.where_token()),
            )
            .field(
                "side_condition",
                &support::DebugSyntaxResult(self.side_condition()),
            )
            .finish()
    }
}
impl From<GritPatternWhere> for SyntaxNode {
    fn from(n: GritPatternWhere) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPatternWhere> for SyntaxElement {
    fn from(n: GritPatternWhere) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAccumulate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ACCUMULATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ACCUMULATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateAccumulate")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "add_assign_token",
                &support::DebugSyntaxResult(self.add_assign_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateAccumulate> for SyntaxNode {
    fn from(n: GritPredicateAccumulate) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateAccumulate> for SyntaxElement {
    fn from(n: GritPredicateAccumulate) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAnd {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_AND as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_AND
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateAnd")
            .field(
                "and_token",
                &support::DebugOptionalElement(self.and_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("predicates", &self.predicates())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPredicateAnd> for SyntaxNode {
    fn from(n: GritPredicateAnd) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateAnd> for SyntaxElement {
    fn from(n: GritPredicateAnd) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAny {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ANY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ANY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateAny")
            .field("any_token", &support::DebugSyntaxResult(self.any_token()))
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("predicates", &self.predicates())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPredicateAny> for SyntaxNode {
    fn from(n: GritPredicateAny) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateAny> for SyntaxElement {
    fn from(n: GritPredicateAny) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateAssignment")
            .field("container", &support::DebugSyntaxResult(self.container()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritPredicateAssignment> for SyntaxNode {
    fn from(n: GritPredicateAssignment) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateAssignment> for SyntaxElement {
    fn from(n: GritPredicateAssignment) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateCall {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_CALL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_CALL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateCall")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("named_args", &self.named_args())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritPredicateCall> for SyntaxNode {
    fn from(n: GritPredicateCall) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateCall> for SyntaxElement {
    fn from(n: GritPredicateCall) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateDefinition")
            .field(
                "predicate_token",
                &support::DebugSyntaxResult(self.predicate_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("args", &support::DebugOptionalElement(self.args()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<GritPredicateDefinition> for SyntaxNode {
    fn from(n: GritPredicateDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateDefinition> for SyntaxElement {
    fn from(n: GritPredicateDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_ELSE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateElseClause")
            .field("else_token", &support::DebugSyntaxResult(self.else_token()))
            .field(
                "else_predicate",
                &support::DebugSyntaxResult(self.else_predicate()),
            )
            .finish()
    }
}
impl From<GritPredicateElseClause> for SyntaxNode {
    fn from(n: GritPredicateElseClause) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateElseClause> for SyntaxElement {
    fn from(n: GritPredicateElseClause) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateEqual")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "equality_token",
                &support::DebugSyntaxResult(self.equality_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateEqual> for SyntaxNode {
    fn from(n: GritPredicateEqual) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateEqual> for SyntaxElement {
    fn from(n: GritPredicateEqual) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateGreater {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_GREATER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_GREATER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateGreater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateGreater")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateGreater> for SyntaxNode {
    fn from(n: GritPredicateGreater) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateGreater> for SyntaxElement {
    fn from(n: GritPredicateGreater) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateGreaterEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_GREATER_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_GREATER_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateGreaterEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateGreaterEqual")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "greater_than_equal_token",
                &support::DebugSyntaxResult(self.greater_than_equal_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateGreaterEqual> for SyntaxNode {
    fn from(n: GritPredicateGreaterEqual) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateGreaterEqual> for SyntaxElement {
    fn from(n: GritPredicateGreaterEqual) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateIfElse {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_IF_ELSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_IF_ELSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateIfElse")
            .field("if_token", &support::DebugSyntaxResult(self.if_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "if_predicate",
                &support::DebugSyntaxResult(self.if_predicate()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "then_predicate",
                &support::DebugSyntaxResult(self.then_predicate()),
            )
            .field(
                "grit_predicate_else_clause",
                &support::DebugOptionalElement(self.grit_predicate_else_clause()),
            )
            .finish()
    }
}
impl From<GritPredicateIfElse> for SyntaxNode {
    fn from(n: GritPredicateIfElse) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateIfElse> for SyntaxElement {
    fn from(n: GritPredicateIfElse) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateLess {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_LESS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_LESS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateLess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateLess")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateLess> for SyntaxNode {
    fn from(n: GritPredicateLess) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateLess> for SyntaxElement {
    fn from(n: GritPredicateLess) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateLessEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_LESS_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_LESS_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateLessEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateLessEqual")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "less_than_equal_token",
                &support::DebugSyntaxResult(self.less_than_equal_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateLessEqual> for SyntaxNode {
    fn from(n: GritPredicateLessEqual) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateLessEqual> for SyntaxElement {
    fn from(n: GritPredicateLessEqual) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateMatch {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_MATCH as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_MATCH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateMatch")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "match_token",
                &support::DebugSyntaxResult(self.match_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateMatch> for SyntaxNode {
    fn from(n: GritPredicateMatch) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateMatch> for SyntaxElement {
    fn from(n: GritPredicateMatch) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateMaybe {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_MAYBE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_MAYBE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateMaybe")
            .field(
                "maybe_token",
                &support::DebugSyntaxResult(self.maybe_token()),
            )
            .field("predicate", &support::DebugSyntaxResult(self.predicate()))
            .finish()
    }
}
impl From<GritPredicateMaybe> for SyntaxNode {
    fn from(n: GritPredicateMaybe) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateMaybe> for SyntaxElement {
    fn from(n: GritPredicateMaybe) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateNot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_NOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_NOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateNot")
            .field("grit_not", &support::DebugSyntaxResult(self.grit_not()))
            .field("predicate", &support::DebugSyntaxResult(self.predicate()))
            .finish()
    }
}
impl From<GritPredicateNot> for SyntaxNode {
    fn from(n: GritPredicateNot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateNot> for SyntaxElement {
    fn from(n: GritPredicateNot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateNotEqual {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_NOT_EQUAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_NOT_EQUAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateNotEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateNotEqual")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "inequality_token",
                &support::DebugSyntaxResult(self.inequality_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateNotEqual> for SyntaxNode {
    fn from(n: GritPredicateNotEqual) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateNotEqual> for SyntaxElement {
    fn from(n: GritPredicateNotEqual) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateOr {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_OR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_OR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateOr")
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("predicates", &self.predicates())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritPredicateOr> for SyntaxNode {
    fn from(n: GritPredicateOr) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateOr> for SyntaxElement {
    fn from(n: GritPredicateOr) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateReturn {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_RETURN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_RETURN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateReturn")
            .field(
                "return_token",
                &support::DebugSyntaxResult(self.return_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritPredicateReturn> for SyntaxNode {
    fn from(n: GritPredicateReturn) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateReturn> for SyntaxElement {
    fn from(n: GritPredicateReturn) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritPredicateRewrite {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_REWRITE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_REWRITE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritPredicateRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritPredicateRewrite")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "annotation",
                &support::DebugOptionalElement(self.annotation()),
            )
            .field(
                "fat_arrow_token",
                &support::DebugSyntaxResult(self.fat_arrow_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritPredicateRewrite> for SyntaxNode {
    fn from(n: GritPredicateRewrite) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritPredicateRewrite> for SyntaxElement {
    fn from(n: GritPredicateRewrite) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritRawBacktickSnippet {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_RAW_BACKTICK_SNIPPET as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_RAW_BACKTICK_SNIPPET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRawBacktickSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritRawBacktickSnippet")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritRawBacktickSnippet> for SyntaxNode {
    fn from(n: GritRawBacktickSnippet) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritRawBacktickSnippet> for SyntaxElement {
    fn from(n: GritRawBacktickSnippet) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritRegexPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REGEX_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REGEX_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRegexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritRegexPattern")
            .field("regex", &support::DebugSyntaxResult(self.regex()))
            .field(
                "variables",
                &support::DebugOptionalElement(self.variables()),
            )
            .finish()
    }
}
impl From<GritRegexPattern> for SyntaxNode {
    fn from(n: GritRegexPattern) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritRegexPattern> for SyntaxElement {
    fn from(n: GritRegexPattern) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritRegexPatternVariables {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REGEX_PATTERN_VARIABLES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REGEX_PATTERN_VARIABLES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRegexPatternVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritRegexPatternVariables")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "grit_pattern_arg_list",
                &support::DebugOptionalElement(self.grit_pattern_arg_list()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritRegexPatternVariables> for SyntaxNode {
    fn from(n: GritRegexPatternVariables) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritRegexPatternVariables> for SyntaxElement {
    fn from(n: GritRegexPatternVariables) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritRegexValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REGEX_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REGEX_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRegexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritRegexValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritRegexValue> for SyntaxNode {
    fn from(n: GritRegexValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritRegexValue> for SyntaxElement {
    fn from(n: GritRegexValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritRewrite {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_REWRITE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_REWRITE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritRewrite")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "annotation",
                &support::DebugOptionalElement(self.annotation()),
            )
            .field(
                "fat_arrow_token",
                &support::DebugSyntaxResult(self.fat_arrow_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritRewrite> for SyntaxNode {
    fn from(n: GritRewrite) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritRewrite> for SyntaxElement {
    fn from(n: GritRewrite) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritRoot")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("version", &support::DebugOptionalElement(self.version()))
            .field("language", &support::DebugOptionalElement(self.language()))
            .field("definitions", &self.definitions())
            .field("pattern", &support::DebugOptionalElement(self.pattern()))
            .field("definitions_continued", &self.definitions_continued())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<GritRoot> for SyntaxNode {
    fn from(n: GritRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritRoot> for SyntaxElement {
    fn from(n: GritRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritSequential {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SEQUENTIAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SEQUENTIAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSequential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritSequential")
            .field(
                "sequential_token",
                &support::DebugSyntaxResult(self.sequential_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("sequential", &self.sequential())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<GritSequential> for SyntaxNode {
    fn from(n: GritSequential) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritSequential> for SyntaxElement {
    fn from(n: GritSequential) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritSnippetRegexValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SNIPPET_REGEX_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SNIPPET_REGEX_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSnippetRegexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritSnippetRegexValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritSnippetRegexValue> for SyntaxNode {
    fn from(n: GritSnippetRegexValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritSnippetRegexValue> for SyntaxElement {
    fn from(n: GritSnippetRegexValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritSome {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SOME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SOME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritSome")
            .field("some_token", &support::DebugSyntaxResult(self.some_token()))
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritSome> for SyntaxNode {
    fn from(n: GritSome) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritSome> for SyntaxElement {
    fn from(n: GritSome) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_STRING_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritStringValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<GritStringValue> for SyntaxNode {
    fn from(n: GritStringValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritStringValue> for SyntaxElement {
    fn from(n: GritStringValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritSubOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SUB_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SUB_OPERATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritSubOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritSubOperation")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "minus_token",
                &support::DebugSyntaxResult(self.minus_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<GritSubOperation> for SyntaxNode {
    fn from(n: GritSubOperation) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritSubOperation> for SyntaxElement {
    fn from(n: GritSubOperation) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritUndefined {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_UNDEFINED as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_UNDEFINED
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritUndefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritUndefined")
            .field(
                "undefined_token",
                &support::DebugSyntaxResult(self.undefined_token()),
            )
            .finish()
    }
}
impl From<GritUndefined> for SyntaxNode {
    fn from(n: GritUndefined) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritUndefined> for SyntaxElement {
    fn from(n: GritUndefined) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritUnderscore {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_UNDERSCORE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_UNDERSCORE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritUnderscore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritUnderscore")
            .field(
                "dollar_underscore_token",
                &support::DebugSyntaxResult(self.dollar_underscore_token()),
            )
            .finish()
    }
}
impl From<GritUnderscore> for SyntaxNode {
    fn from(n: GritUnderscore) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritUnderscore> for SyntaxElement {
    fn from(n: GritUnderscore) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritVariable {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_VARIABLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_VARIABLE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritVariable")
            .field(
                "grit_variable_token",
                &support::DebugSyntaxResult(self.grit_variable_token()),
            )
            .finish()
    }
}
impl From<GritVariable> for SyntaxNode {
    fn from(n: GritVariable) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritVariable> for SyntaxElement {
    fn from(n: GritVariable) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritVersion {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_VERSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_VERSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritVersion")
            .field(
                "engine_token",
                &support::DebugSyntaxResult(self.engine_token()),
            )
            .field(
                "biome_token",
                &support::DebugSyntaxResult(self.biome_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "grit_double_value",
                &support::DebugSyntaxResult(self.grit_double_value()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<GritVersion> for SyntaxNode {
    fn from(n: GritVersion) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritVersion> for SyntaxElement {
    fn from(n: GritVersion) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for GritWithin {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_WITHIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_WITHIN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritWithin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritWithin")
            .field(
                "within_token",
                &support::DebugSyntaxResult(self.within_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<GritWithin> for SyntaxNode {
    fn from(n: GritWithin) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritWithin> for SyntaxElement {
    fn from(n: GritWithin) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<GritListAccessor> for AnyGritContainer {
    fn from(node: GritListAccessor) -> AnyGritContainer {
        AnyGritContainer::GritListAccessor(node)
    }
}
impl From<GritMapAccessor> for AnyGritContainer {
    fn from(node: GritMapAccessor) -> AnyGritContainer {
        AnyGritContainer::GritMapAccessor(node)
    }
}
impl From<GritVariable> for AnyGritContainer {
    fn from(node: GritVariable) -> AnyGritContainer {
        AnyGritContainer::GritVariable(node)
    }
}
impl AstNode for AnyGritContainer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritListAccessor::KIND_SET
        .union(GritMapAccessor::KIND_SET)
        .union(GritVariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_LIST_ACCESSOR | GRIT_MAP_ACCESSOR | GRIT_VARIABLE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_LIST_ACCESSOR => AnyGritContainer::GritListAccessor(GritListAccessor { syntax }),
            GRIT_MAP_ACCESSOR => AnyGritContainer::GritMapAccessor(GritMapAccessor { syntax }),
            GRIT_VARIABLE => AnyGritContainer::GritVariable(GritVariable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGritContainer::GritListAccessor(it) => &it.syntax,
            AnyGritContainer::GritMapAccessor(it) => &it.syntax,
            AnyGritContainer::GritVariable(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGritContainer::GritListAccessor(it) => it.syntax,
            AnyGritContainer::GritMapAccessor(it) => it.syntax,
            AnyGritContainer::GritVariable(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGritContainer::GritListAccessor(it) => std::fmt::Debug::fmt(it, f),
            AnyGritContainer::GritMapAccessor(it) => std::fmt::Debug::fmt(it, f),
            AnyGritContainer::GritVariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritContainer> for SyntaxNode {
    fn from(n: AnyGritContainer) -> SyntaxNode {
        match n {
            AnyGritContainer::GritListAccessor(it) => it.into(),
            AnyGritContainer::GritMapAccessor(it) => it.into(),
            AnyGritContainer::GritVariable(it) => it.into(),
        }
    }
}
impl From<AnyGritContainer> for SyntaxElement {
    fn from(n: AnyGritContainer) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusDefinition> for AnyGritDefinition {
    fn from(node: GritBogusDefinition) -> AnyGritDefinition {
        AnyGritDefinition::GritBogusDefinition(node)
    }
}
impl From<GritFunctionDefinition> for AnyGritDefinition {
    fn from(node: GritFunctionDefinition) -> AnyGritDefinition {
        AnyGritDefinition::GritFunctionDefinition(node)
    }
}
impl From<GritPatternDefinition> for AnyGritDefinition {
    fn from(node: GritPatternDefinition) -> AnyGritDefinition {
        AnyGritDefinition::GritPatternDefinition(node)
    }
}
impl From<GritPredicateDefinition> for AnyGritDefinition {
    fn from(node: GritPredicateDefinition) -> AnyGritDefinition {
        AnyGritDefinition::GritPredicateDefinition(node)
    }
}
impl AstNode for AnyGritDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBogusDefinition::KIND_SET
        .union(GritFunctionDefinition::KIND_SET)
        .union(GritPatternDefinition::KIND_SET)
        .union(GritPredicateDefinition::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_DEFINITION
                | GRIT_FUNCTION_DEFINITION
                | GRIT_PATTERN_DEFINITION
                | GRIT_PREDICATE_DEFINITION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_DEFINITION => {
                AnyGritDefinition::GritBogusDefinition(GritBogusDefinition { syntax })
            }
            GRIT_FUNCTION_DEFINITION => {
                AnyGritDefinition::GritFunctionDefinition(GritFunctionDefinition { syntax })
            }
            GRIT_PATTERN_DEFINITION => {
                AnyGritDefinition::GritPatternDefinition(GritPatternDefinition { syntax })
            }
            GRIT_PREDICATE_DEFINITION => {
                AnyGritDefinition::GritPredicateDefinition(GritPredicateDefinition { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGritDefinition::GritBogusDefinition(it) => &it.syntax,
            AnyGritDefinition::GritFunctionDefinition(it) => &it.syntax,
            AnyGritDefinition::GritPatternDefinition(it) => &it.syntax,
            AnyGritDefinition::GritPredicateDefinition(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGritDefinition::GritBogusDefinition(it) => it.syntax,
            AnyGritDefinition::GritFunctionDefinition(it) => it.syntax,
            AnyGritDefinition::GritPatternDefinition(it) => it.syntax,
            AnyGritDefinition::GritPredicateDefinition(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGritDefinition::GritBogusDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGritDefinition::GritFunctionDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGritDefinition::GritPatternDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyGritDefinition::GritPredicateDefinition(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritDefinition> for SyntaxNode {
    fn from(n: AnyGritDefinition) -> SyntaxNode {
        match n {
            AnyGritDefinition::GritBogusDefinition(it) => it.into(),
            AnyGritDefinition::GritFunctionDefinition(it) => it.into(),
            AnyGritDefinition::GritPatternDefinition(it) => it.into(),
            AnyGritDefinition::GritPredicateDefinition(it) => it.into(),
        }
    }
}
impl From<AnyGritDefinition> for SyntaxElement {
    fn from(n: AnyGritDefinition) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<AnyGritPattern> for AnyGritListPattern {
    fn from(node: AnyGritPattern) -> AnyGritListPattern {
        AnyGritListPattern::AnyGritPattern(node)
    }
}
impl From<GritDotdotdot> for AnyGritListPattern {
    fn from(node: GritDotdotdot) -> AnyGritListPattern {
        AnyGritListPattern::GritDotdotdot(node)
    }
}
impl AstNode for AnyGritListPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyGritPattern::KIND_SET.union(GritDotdotdot::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, ANY_GRIT_PATTERN | GRIT_DOTDOTDOT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ANY_GRIT_PATTERN => AnyGritListPattern::AnyGritPattern(AnyGritPattern { syntax }),
            GRIT_DOTDOTDOT => AnyGritListPattern::GritDotdotdot(GritDotdotdot { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGritListPattern::AnyGritPattern(it) => &it.syntax,
            AnyGritListPattern::GritDotdotdot(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGritListPattern::AnyGritPattern(it) => it.syntax,
            AnyGritListPattern::GritDotdotdot(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritListPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGritListPattern::AnyGritPattern(it) => std::fmt::Debug::fmt(it, f),
            AnyGritListPattern::GritDotdotdot(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritListPattern> for SyntaxNode {
    fn from(n: AnyGritListPattern) -> SyntaxNode {
        match n {
            AnyGritListPattern::AnyGritPattern(it) => it.into(),
            AnyGritListPattern::GritDotdotdot(it) => it.into(),
        }
    }
}
impl From<AnyGritListPattern> for SyntaxElement {
    fn from(n: AnyGritListPattern) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusLiteral> for AnyGritLiteral {
    fn from(node: GritBogusLiteral) -> AnyGritLiteral {
        AnyGritLiteral::GritBogusLiteral(node)
    }
}
impl From<GritBooleanValue> for AnyGritLiteral {
    fn from(node: GritBooleanValue) -> AnyGritLiteral {
        AnyGritLiteral::GritBooleanValue(node)
    }
}
impl From<GritCodeSnippet> for AnyGritLiteral {
    fn from(node: GritCodeSnippet) -> AnyGritLiteral {
        AnyGritLiteral::GritCodeSnippet(node)
    }
}
impl From<GritDoubleValue> for AnyGritLiteral {
    fn from(node: GritDoubleValue) -> AnyGritLiteral {
        AnyGritLiteral::GritDoubleValue(node)
    }
}
impl From<GritIntValue> for AnyGritLiteral {
    fn from(node: GritIntValue) -> AnyGritLiteral {
        AnyGritLiteral::GritIntValue(node)
    }
}
impl From<GritList> for AnyGritLiteral {
    fn from(node: GritList) -> AnyGritLiteral {
        AnyGritLiteral::GritList(node)
    }
}
impl From<GritMap> for AnyGritLiteral {
    fn from(node: GritMap) -> AnyGritLiteral {
        AnyGritLiteral::GritMap(node)
    }
}
impl From<GritStringValue> for AnyGritLiteral {
    fn from(node: GritStringValue) -> AnyGritLiteral {
        AnyGritLiteral::GritStringValue(node)
    }
}
impl From<GritUndefined> for AnyGritLiteral {
    fn from(node: GritUndefined) -> AnyGritLiteral {
        AnyGritLiteral::GritUndefined(node)
    }
}
impl AstNode for AnyGritLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBogusLiteral::KIND_SET
        .union(GritBooleanValue::KIND_SET)
        .union(GritCodeSnippet::KIND_SET)
        .union(GritDoubleValue::KIND_SET)
        .union(GritIntValue::KIND_SET)
        .union(GritList::KIND_SET)
        .union(GritMap::KIND_SET)
        .union(GritStringValue::KIND_SET)
        .union(GritUndefined::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_LITERAL
                | GRIT_BOOLEAN_VALUE
                | GRIT_CODE_SNIPPET
                | GRIT_DOUBLE_VALUE
                | GRIT_INT_VALUE
                | GRIT_LIST
                | GRIT_MAP
                | GRIT_STRING_VALUE
                | GRIT_UNDEFINED
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_LITERAL => AnyGritLiteral::GritBogusLiteral(GritBogusLiteral { syntax }),
            GRIT_BOOLEAN_VALUE => AnyGritLiteral::GritBooleanValue(GritBooleanValue { syntax }),
            GRIT_CODE_SNIPPET => AnyGritLiteral::GritCodeSnippet(GritCodeSnippet { syntax }),
            GRIT_DOUBLE_VALUE => AnyGritLiteral::GritDoubleValue(GritDoubleValue { syntax }),
            GRIT_INT_VALUE => AnyGritLiteral::GritIntValue(GritIntValue { syntax }),
            GRIT_LIST => AnyGritLiteral::GritList(GritList { syntax }),
            GRIT_MAP => AnyGritLiteral::GritMap(GritMap { syntax }),
            GRIT_STRING_VALUE => AnyGritLiteral::GritStringValue(GritStringValue { syntax }),
            GRIT_UNDEFINED => AnyGritLiteral::GritUndefined(GritUndefined { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGritLiteral::GritBogusLiteral(it) => &it.syntax,
            AnyGritLiteral::GritBooleanValue(it) => &it.syntax,
            AnyGritLiteral::GritCodeSnippet(it) => &it.syntax,
            AnyGritLiteral::GritDoubleValue(it) => &it.syntax,
            AnyGritLiteral::GritIntValue(it) => &it.syntax,
            AnyGritLiteral::GritList(it) => &it.syntax,
            AnyGritLiteral::GritMap(it) => &it.syntax,
            AnyGritLiteral::GritStringValue(it) => &it.syntax,
            AnyGritLiteral::GritUndefined(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGritLiteral::GritBogusLiteral(it) => it.syntax,
            AnyGritLiteral::GritBooleanValue(it) => it.syntax,
            AnyGritLiteral::GritCodeSnippet(it) => it.syntax,
            AnyGritLiteral::GritDoubleValue(it) => it.syntax,
            AnyGritLiteral::GritIntValue(it) => it.syntax,
            AnyGritLiteral::GritList(it) => it.syntax,
            AnyGritLiteral::GritMap(it) => it.syntax,
            AnyGritLiteral::GritStringValue(it) => it.syntax,
            AnyGritLiteral::GritUndefined(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGritLiteral::GritBogusLiteral(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritBooleanValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritCodeSnippet(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritDoubleValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritIntValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritList(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritMap(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritStringValue(it) => std::fmt::Debug::fmt(it, f),
            AnyGritLiteral::GritUndefined(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritLiteral> for SyntaxNode {
    fn from(n: AnyGritLiteral) -> SyntaxNode {
        match n {
            AnyGritLiteral::GritBogusLiteral(it) => it.into(),
            AnyGritLiteral::GritBooleanValue(it) => it.into(),
            AnyGritLiteral::GritCodeSnippet(it) => it.into(),
            AnyGritLiteral::GritDoubleValue(it) => it.into(),
            AnyGritLiteral::GritIntValue(it) => it.into(),
            AnyGritLiteral::GritList(it) => it.into(),
            AnyGritLiteral::GritMap(it) => it.into(),
            AnyGritLiteral::GritStringValue(it) => it.into(),
            AnyGritLiteral::GritUndefined(it) => it.into(),
        }
    }
}
impl From<AnyGritLiteral> for SyntaxElement {
    fn from(n: AnyGritLiteral) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBogusNamedArg> for AnyGritNamedArg {
    fn from(node: GritBogusNamedArg) -> AnyGritNamedArg {
        AnyGritNamedArg::GritBogusNamedArg(node)
    }
}
impl From<GritNamedArg> for AnyGritNamedArg {
    fn from(node: GritNamedArg) -> AnyGritNamedArg {
        AnyGritNamedArg::GritNamedArg(node)
    }
}
impl From<GritNamedArgWithDefault> for AnyGritNamedArg {
    fn from(node: GritNamedArgWithDefault) -> AnyGritNamedArg {
        AnyGritNamedArg::GritNamedArgWithDefault(node)
    }
}
impl AstNode for AnyGritNamedArg {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBogusNamedArg::KIND_SET
        .union(GritNamedArg::KIND_SET)
        .union(GritNamedArgWithDefault::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BOGUS_NAMED_ARG | GRIT_NAMED_ARG | GRIT_NAMED_ARG_WITH_DEFAULT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BOGUS_NAMED_ARG => {
                AnyGritNamedArg::GritBogusNamedArg(GritBogusNamedArg { syntax })
            }
            GRIT_NAMED_ARG => AnyGritNamedArg::GritNamedArg(GritNamedArg { syntax }),
            GRIT_NAMED_ARG_WITH_DEFAULT => {
                AnyGritNamedArg::GritNamedArgWithDefault(GritNamedArgWithDefault { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyGritNamedArg::GritBogusNamedArg(it) => &it.syntax,
            AnyGritNamedArg::GritNamedArg(it) => &it.syntax,
            AnyGritNamedArg::GritNamedArgWithDefault(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyGritNamedArg::GritBogusNamedArg(it) => it.syntax,
            AnyGritNamedArg::GritNamedArg(it) => it.syntax,
            AnyGritNamedArg::GritNamedArgWithDefault(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyGritNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyGritNamedArg::GritBogusNamedArg(it) => std::fmt::Debug::fmt(it, f),
            AnyGritNamedArg::GritNamedArg(it) => std::fmt::Debug::fmt(it, f),
            AnyGritNamedArg::GritNamedArgWithDefault(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyGritNamedArg> for SyntaxNode {
    fn from(n: AnyGritNamedArg) -> SyntaxNode {
        match n {
            AnyGritNamedArg::GritBogusNamedArg(it) => it.into(),
            AnyGritNamedArg::GritNamedArg(it) => it.into(),
            AnyGritNamedArg::GritNamedArgWithDefault(it) => it.into(),
        }
    }
}
impl From<AnyGritNamedArg> for SyntaxElement {
    fn from(n: AnyGritNamedArg) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritBacktickSnippet> for GritCodeSnippetSource {
    fn from(node: GritBacktickSnippet) -> GritCodeSnippetSource {
        GritCodeSnippetSource::GritBacktickSnippet(node)
    }
}
impl From<GritLanguageSpecificSnippet> for GritCodeSnippetSource {
    fn from(node: GritLanguageSpecificSnippet) -> GritCodeSnippetSource {
        GritCodeSnippetSource::GritLanguageSpecificSnippet(node)
    }
}
impl From<GritRawBacktickSnippet> for GritCodeSnippetSource {
    fn from(node: GritRawBacktickSnippet) -> GritCodeSnippetSource {
        GritCodeSnippetSource::GritRawBacktickSnippet(node)
    }
}
impl AstNode for GritCodeSnippetSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritBacktickSnippet::KIND_SET
        .union(GritLanguageSpecificSnippet::KIND_SET)
        .union(GritRawBacktickSnippet::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            GRIT_BACKTICK_SNIPPET | GRIT_LANGUAGE_SPECIFIC_SNIPPET | GRIT_RAW_BACKTICK_SNIPPET
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_BACKTICK_SNIPPET => {
                GritCodeSnippetSource::GritBacktickSnippet(GritBacktickSnippet { syntax })
            }
            GRIT_LANGUAGE_SPECIFIC_SNIPPET => {
                GritCodeSnippetSource::GritLanguageSpecificSnippet(GritLanguageSpecificSnippet {
                    syntax,
                })
            }
            GRIT_RAW_BACKTICK_SNIPPET => {
                GritCodeSnippetSource::GritRawBacktickSnippet(GritRawBacktickSnippet { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritCodeSnippetSource::GritBacktickSnippet(it) => &it.syntax,
            GritCodeSnippetSource::GritLanguageSpecificSnippet(it) => &it.syntax,
            GritCodeSnippetSource::GritRawBacktickSnippet(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritCodeSnippetSource::GritBacktickSnippet(it) => it.syntax,
            GritCodeSnippetSource::GritLanguageSpecificSnippet(it) => it.syntax,
            GritCodeSnippetSource::GritRawBacktickSnippet(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for GritCodeSnippetSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritCodeSnippetSource::GritBacktickSnippet(it) => std::fmt::Debug::fmt(it, f),
            GritCodeSnippetSource::GritLanguageSpecificSnippet(it) => std::fmt::Debug::fmt(it, f),
            GritCodeSnippetSource::GritRawBacktickSnippet(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritCodeSnippetSource> for SyntaxNode {
    fn from(n: GritCodeSnippetSource) -> SyntaxNode {
        match n {
            GritCodeSnippetSource::GritBacktickSnippet(it) => it.into(),
            GritCodeSnippetSource::GritLanguageSpecificSnippet(it) => it.into(),
            GritCodeSnippetSource::GritRawBacktickSnippet(it) => it.into(),
        }
    }
}
impl From<GritCodeSnippetSource> for SyntaxElement {
    fn from(n: GritCodeSnippetSource) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritList> for GritListAccessorSubject {
    fn from(node: GritList) -> GritListAccessorSubject {
        GritListAccessorSubject::GritList(node)
    }
}
impl AstNode for GritListAccessorSubject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritContainer::KIND_SET.union(GritList::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_LIST => true,
            k if AnyGritContainer::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_LIST => GritListAccessorSubject::GritList(GritList { syntax }),
            _ => {
                if let Some(any_grit_container) = AnyGritContainer::cast(syntax) {
                    return Some(GritListAccessorSubject::AnyGritContainer(
                        any_grit_container,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritListAccessorSubject::GritList(it) => &it.syntax,
            GritListAccessorSubject::AnyGritContainer(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritListAccessorSubject::GritList(it) => it.syntax,
            GritListAccessorSubject::AnyGritContainer(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for GritListAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritListAccessorSubject::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            GritListAccessorSubject::GritList(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritListAccessorSubject> for SyntaxNode {
    fn from(n: GritListAccessorSubject) -> SyntaxNode {
        match n {
            GritListAccessorSubject::AnyGritContainer(it) => it.into(),
            GritListAccessorSubject::GritList(it) => it.into(),
        }
    }
}
impl From<GritListAccessorSubject> for SyntaxElement {
    fn from(n: GritListAccessorSubject) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritIntValue> for GritListIndex {
    fn from(node: GritIntValue) -> GritListIndex {
        GritListIndex::GritIntValue(node)
    }
}
impl From<GritNegativeIntValue> for GritListIndex {
    fn from(node: GritNegativeIntValue) -> GritListIndex {
        GritListIndex::GritNegativeIntValue(node)
    }
}
impl AstNode for GritListIndex {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritContainer::KIND_SET
        .union(GritIntValue::KIND_SET)
        .union(GritNegativeIntValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_INT_VALUE | GRIT_NEGATIVE_INT_VALUE => true,
            k if AnyGritContainer::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_INT_VALUE => GritListIndex::GritIntValue(GritIntValue { syntax }),
            GRIT_NEGATIVE_INT_VALUE => {
                GritListIndex::GritNegativeIntValue(GritNegativeIntValue { syntax })
            }
            _ => {
                if let Some(any_grit_container) = AnyGritContainer::cast(syntax) {
                    return Some(GritListIndex::AnyGritContainer(any_grit_container));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritListIndex::GritIntValue(it) => &it.syntax,
            GritListIndex::GritNegativeIntValue(it) => &it.syntax,
            GritListIndex::AnyGritContainer(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritListIndex::GritIntValue(it) => it.syntax,
            GritListIndex::GritNegativeIntValue(it) => it.syntax,
            GritListIndex::AnyGritContainer(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for GritListIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritListIndex::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            GritListIndex::GritIntValue(it) => std::fmt::Debug::fmt(it, f),
            GritListIndex::GritNegativeIntValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritListIndex> for SyntaxNode {
    fn from(n: GritListIndex) -> SyntaxNode {
        match n {
            GritListIndex::AnyGritContainer(it) => it.into(),
            GritListIndex::GritIntValue(it) => it.into(),
            GritListIndex::GritNegativeIntValue(it) => it.into(),
        }
    }
}
impl From<GritListIndex> for SyntaxElement {
    fn from(n: GritListIndex) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritMap> for GritMapAccessorSubject {
    fn from(node: GritMap) -> GritMapAccessorSubject {
        GritMapAccessorSubject::GritMap(node)
    }
}
impl AstNode for GritMapAccessorSubject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyGritContainer::KIND_SET.union(GritMap::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            GRIT_MAP => true,
            k if AnyGritContainer::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_MAP => GritMapAccessorSubject::GritMap(GritMap { syntax }),
            _ => {
                if let Some(any_grit_container) = AnyGritContainer::cast(syntax) {
                    return Some(GritMapAccessorSubject::AnyGritContainer(any_grit_container));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritMapAccessorSubject::GritMap(it) => &it.syntax,
            GritMapAccessorSubject::AnyGritContainer(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritMapAccessorSubject::GritMap(it) => it.syntax,
            GritMapAccessorSubject::AnyGritContainer(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for GritMapAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritMapAccessorSubject::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            GritMapAccessorSubject::GritMap(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritMapAccessorSubject> for SyntaxNode {
    fn from(n: GritMapAccessorSubject) -> SyntaxNode {
        match n {
            GritMapAccessorSubject::AnyGritContainer(it) => it.into(),
            GritMapAccessorSubject::GritMap(it) => it.into(),
        }
    }
}
impl From<GritMapAccessorSubject> for SyntaxElement {
    fn from(n: GritMapAccessorSubject) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritName> for GritMapKey {
    fn from(node: GritName) -> GritMapKey {
        GritMapKey::GritName(node)
    }
}
impl From<GritVariable> for GritMapKey {
    fn from(node: GritVariable) -> GritMapKey {
        GritMapKey::GritVariable(node)
    }
}
impl AstNode for GritMapKey {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = GritName::KIND_SET.union(GritVariable::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_NAME | GRIT_VARIABLE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_NAME => GritMapKey::GritName(GritName { syntax }),
            GRIT_VARIABLE => GritMapKey::GritVariable(GritVariable { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritMapKey::GritName(it) => &it.syntax,
            GritMapKey::GritVariable(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritMapKey::GritName(it) => it.syntax,
            GritMapKey::GritVariable(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for GritMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritMapKey::GritName(it) => std::fmt::Debug::fmt(it, f),
            GritMapKey::GritVariable(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritMapKey> for SyntaxNode {
    fn from(n: GritMapKey) -> SyntaxNode {
        match n {
            GritMapKey::GritName(it) => it.into(),
            GritMapKey::GritVariable(it) => it.into(),
        }
    }
}
impl From<GritMapKey> for SyntaxElement {
    fn from(n: GritMapKey) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl AstNode for GritPredicateMatchSubject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyGritContainer::KIND_SET.union(AnyGritLiteral::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            k if AnyGritContainer::can_cast(k) => true,
            k if AnyGritLiteral::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if let Some(any_grit_container) = AnyGritContainer::cast(syntax.clone()) {
            return Some(GritPredicateMatchSubject::AnyGritContainer(
                any_grit_container,
            ));
        }
        if let Some(any_grit_literal) = AnyGritLiteral::cast(syntax) {
            return Some(GritPredicateMatchSubject::AnyGritLiteral(any_grit_literal));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritPredicateMatchSubject::AnyGritContainer(it) => it.syntax(),
            GritPredicateMatchSubject::AnyGritLiteral(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritPredicateMatchSubject::AnyGritContainer(it) => it.into_syntax(),
            GritPredicateMatchSubject::AnyGritLiteral(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for GritPredicateMatchSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritPredicateMatchSubject::AnyGritContainer(it) => std::fmt::Debug::fmt(it, f),
            GritPredicateMatchSubject::AnyGritLiteral(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritPredicateMatchSubject> for SyntaxNode {
    fn from(n: GritPredicateMatchSubject) -> SyntaxNode {
        match n {
            GritPredicateMatchSubject::AnyGritContainer(it) => it.into(),
            GritPredicateMatchSubject::AnyGritLiteral(it) => it.into(),
        }
    }
}
impl From<GritPredicateMatchSubject> for SyntaxElement {
    fn from(n: GritPredicateMatchSubject) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<GritRegexValue> for GritRegex {
    fn from(node: GritRegexValue) -> GritRegex {
        GritRegex::GritRegexValue(node)
    }
}
impl From<GritSnippetRegexValue> for GritRegex {
    fn from(node: GritSnippetRegexValue) -> GritRegex {
        GritRegex::GritSnippetRegexValue(node)
    }
}
impl AstNode for GritRegex {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        GritRegexValue::KIND_SET.union(GritSnippetRegexValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, GRIT_REGEX_VALUE | GRIT_SNIPPET_REGEX_VALUE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            GRIT_REGEX_VALUE => GritRegex::GritRegexValue(GritRegexValue { syntax }),
            GRIT_SNIPPET_REGEX_VALUE => {
                GritRegex::GritSnippetRegexValue(GritSnippetRegexValue { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GritRegex::GritRegexValue(it) => &it.syntax,
            GritRegex::GritSnippetRegexValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            GritRegex::GritRegexValue(it) => it.syntax,
            GritRegex::GritSnippetRegexValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for GritRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GritRegex::GritRegexValue(it) => std::fmt::Debug::fmt(it, f),
            GritRegex::GritSnippetRegexValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<GritRegex> for SyntaxNode {
    fn from(n: GritRegex) -> SyntaxNode {
        match n {
            GritRegex::GritRegexValue(it) => it.into(),
            GritRegex::GritSnippetRegexValue(it) => it.into(),
        }
    }
}
impl From<GritRegex> for SyntaxElement {
    fn from(n: GritRegex) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<AnyGritPattern> for MaybeCurlyGritPattern {
    fn from(node: AnyGritPattern) -> MaybeCurlyGritPattern {
        MaybeCurlyGritPattern::AnyGritPattern(node)
    }
}
impl From<CurlyGritPattern> for MaybeCurlyGritPattern {
    fn from(node: CurlyGritPattern) -> MaybeCurlyGritPattern {
        MaybeCurlyGritPattern::CurlyGritPattern(node)
    }
}
impl AstNode for MaybeCurlyGritPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyGritPattern::KIND_SET.union(CurlyGritPattern::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, ANY_GRIT_PATTERN | CURLY_GRIT_PATTERN)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ANY_GRIT_PATTERN => MaybeCurlyGritPattern::AnyGritPattern(AnyGritPattern { syntax }),
            CURLY_GRIT_PATTERN => {
                MaybeCurlyGritPattern::CurlyGritPattern(CurlyGritPattern { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            MaybeCurlyGritPattern::AnyGritPattern(it) => &it.syntax,
            MaybeCurlyGritPattern::CurlyGritPattern(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            MaybeCurlyGritPattern::AnyGritPattern(it) => it.syntax,
            MaybeCurlyGritPattern::CurlyGritPattern(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for MaybeCurlyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeCurlyGritPattern::AnyGritPattern(it) => std::fmt::Debug::fmt(it, f),
            MaybeCurlyGritPattern::CurlyGritPattern(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<MaybeCurlyGritPattern> for SyntaxNode {
    fn from(n: MaybeCurlyGritPattern) -> SyntaxNode {
        match n {
            MaybeCurlyGritPattern::AnyGritPattern(it) => it.into(),
            MaybeCurlyGritPattern::CurlyGritPattern(it) => it.into(),
        }
    }
}
impl From<MaybeCurlyGritPattern> for SyntaxElement {
    fn from(n: MaybeCurlyGritPattern) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyGritContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritListPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritCodeSnippetSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritListAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritListIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMapAccessorSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateMatchSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MaybeCurlyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyGritPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CurlyGritPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritAddOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritAssignmentAsPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBacktickSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBubble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritBubbleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritCodeSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritCurlyPredicateList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDivOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDotdotdot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritDoubleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritEvery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageFlavorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLanguageSpecificSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritLikeThreshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritListAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMapAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritModOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritMulOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNamedArgWithDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNegativeIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNodeLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternAs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternContains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternContainsUntilClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternDefinitionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternIncludes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternOrElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPatternWhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAccumulate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateGreater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateGreaterEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateIfElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateLess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateLessEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateMaybe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateNotEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritPredicateRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRawBacktickSnippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegexPatternVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRegexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSequential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSnippetRegexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritSubOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritUndefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritUnderscore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GritWithin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBogus {
    syntax: SyntaxNode,
}
impl GritBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogus> for SyntaxNode {
    fn from(n: GritBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBogus> for SyntaxElement {
    fn from(n: GritBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBogusDefinition {
    syntax: SyntaxNode,
}
impl GritBogusDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusDefinition")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusDefinition> for SyntaxNode {
    fn from(n: GritBogusDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBogusDefinition> for SyntaxElement {
    fn from(n: GritBogusDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBogusLiteral {
    syntax: SyntaxNode,
}
impl GritBogusLiteral {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusLiteral {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_LITERAL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusLiteral")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusLiteral> for SyntaxNode {
    fn from(n: GritBogusLiteral) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBogusLiteral> for SyntaxElement {
    fn from(n: GritBogusLiteral) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBogusNamedArg {
    syntax: SyntaxNode,
}
impl GritBogusNamedArg {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusNamedArg {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_NAMED_ARG as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_NAMED_ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusNamedArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusNamedArg")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusNamedArg> for SyntaxNode {
    fn from(n: GritBogusNamedArg) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBogusNamedArg> for SyntaxElement {
    fn from(n: GritBogusNamedArg) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBogusPattern {
    syntax: SyntaxNode,
}
impl GritBogusPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusPattern")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusPattern> for SyntaxNode {
    fn from(n: GritBogusPattern) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBogusPattern> for SyntaxElement {
    fn from(n: GritBogusPattern) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct GritBogusPredicate {
    syntax: SyntaxNode,
}
impl GritBogusPredicate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for GritBogusPredicate {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_BOGUS_PREDICATE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_BOGUS_PREDICATE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for GritBogusPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritBogusPredicate")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<GritBogusPredicate> for SyntaxNode {
    fn from(n: GritBogusPredicate) -> SyntaxNode {
        n.syntax
    }
}
impl From<GritBogusPredicate> for SyntaxElement {
    fn from(n: GritBogusPredicate) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritDefinitionList {
    syntax_list: SyntaxList,
}
impl GritDefinitionList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_DEFINITION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritDefinitionList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritDefinitionList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritDefinitionList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritDefinitionList {
    type Language = Language;
    type Node = AnyGritDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritDefinitionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritDefinitionList {
    type Item = SyntaxResult<AnyGritDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritDefinitionList {
    type Item = SyntaxResult<AnyGritDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritFilesList {
    syntax_list: SyntaxList,
}
impl GritFilesList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritFilesList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_FILES_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_FILES_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritFilesList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritFilesList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritFilesList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritFilesList {
    type Language = Language;
    type Node = AnyGritPattern;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritFilesList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritFilesList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritFilesList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritFilesList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritLanguageFlavorList {
    syntax_list: SyntaxList,
}
impl GritLanguageFlavorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritLanguageFlavorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LANGUAGE_FLAVOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LANGUAGE_FLAVOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritLanguageFlavorList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritLanguageFlavorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritLanguageFlavorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritLanguageFlavorList {
    type Language = Language;
    type Node = GritLanguageFlavorKind;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritLanguageFlavorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritLanguageFlavorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritLanguageFlavorList {
    type Item = SyntaxResult<GritLanguageFlavorKind>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritLanguageFlavorKind>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritLanguageFlavorList {
    type Item = SyntaxResult<GritLanguageFlavorKind>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritLanguageFlavorKind>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritListPatternList {
    syntax_list: SyntaxList,
}
impl GritListPatternList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritListPatternList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_LIST_PATTERN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_LIST_PATTERN_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritListPatternList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritListPatternList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritListPatternList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritListPatternList {
    type Language = Language;
    type Node = AnyGritListPattern;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritListPatternList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritListPatternList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritListPatternList {
    type Item = SyntaxResult<AnyGritListPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritListPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritListPatternList {
    type Item = SyntaxResult<AnyGritListPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritListPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritMapElementList {
    syntax_list: SyntaxList,
}
impl GritMapElementList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritMapElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_MAP_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_MAP_ELEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritMapElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritMapElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritMapElementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritMapElementList {
    type Language = Language;
    type Node = GritMapElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritMapElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritMapElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritMapElementList {
    type Item = SyntaxResult<GritMapElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritMapElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritMapElementList {
    type Item = SyntaxResult<GritMapElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritMapElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritNamedArgList {
    syntax_list: SyntaxList,
}
impl GritNamedArgList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritNamedArgList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_NAMED_ARG_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_NAMED_ARG_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritNamedArgList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritNamedArgList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritNamedArgList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritNamedArgList {
    type Language = Language;
    type Node = GritNamedArg;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritNamedArgList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritNamedArgList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritNamedArgList {
    type Item = SyntaxResult<GritNamedArg>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritNamedArg>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritNamedArgList {
    type Item = SyntaxResult<GritNamedArg>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritNamedArg>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritPatternList {
    syntax_list: SyntaxList,
}
impl GritPatternList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritPatternList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PATTERN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PATTERN_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritPatternList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritPatternList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPatternList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritPatternList {
    type Language = Language;
    type Node = AnyGritPattern;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritPatternList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritPatternList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritPatternList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritPatternList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritPredicateList {
    syntax_list: SyntaxList,
}
impl GritPredicateList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritPredicateList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_PREDICATE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_PREDICATE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritPredicateList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritPredicateList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritPredicateList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritPredicateList {
    type Language = Language;
    type Node = AnyGritPredicate;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritPredicateList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritPredicateList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritPredicateList {
    type Item = SyntaxResult<AnyGritPredicate>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPredicate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritPredicateList {
    type Item = SyntaxResult<AnyGritPredicate>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPredicate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritSequentialList {
    syntax_list: SyntaxList,
}
impl GritSequentialList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritSequentialList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_SEQUENTIAL_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_SEQUENTIAL_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritSequentialList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritSequentialList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritSequentialList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritSequentialList {
    type Language = Language;
    type Node = AnyGritPattern;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritSequentialList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritSequentialList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritSequentialList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritSequentialList {
    type Item = SyntaxResult<AnyGritPattern>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyGritPattern>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GritVariableList {
    syntax_list: SyntaxList,
}
impl GritVariableList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for GritVariableList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GRIT_VARIABLE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == GRIT_VARIABLE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<GritVariableList> {
        if Self::can_cast(syntax.kind()) {
            Some(GritVariableList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
#[cfg(feature = "serde")]
impl Serialize for GritVariableList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for GritVariableList {
    type Language = Language;
    type Node = GritVariable;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for GritVariableList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GritVariableList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for GritVariableList {
    type Item = SyntaxResult<GritVariable>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritVariable>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &GritVariableList {
    type Item = SyntaxResult<GritVariable>;
    type IntoIter = AstSeparatedListNodesIterator<Language, GritVariable>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
