use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    inner_string_text, AnyJsArrayAssignmentPatternElement, AnyJsArrayElement, AnyJsAssignment,
    AnyJsAssignmentPattern, AnyJsExpression, AnyJsLiteralExpression, AnyJsName,
    AnyJsObjectAssignmentPatternMember, AnyJsObjectMember, JsAssignmentExpression,
    JsAssignmentOperator, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsIdentifierAssignment, JsLanguage, JsName, JsPrivateName, JsReferenceIdentifier,
    JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxToken,
};
use biome_rowan::{
    declare_node_union, AstNode, AstSeparatedList, AstSeparatedListNodesIterator, SyntaxError,
    SyntaxResult, TextRange,
};
use std::collections::VecDeque;
use std::iter::FusedIterator;

declare_lint_rule! {
    /// Disallow assignments where both sides are exactly the same.
    ///
    /// Self assignments have no effect, so probably those are an error due to incomplete refactoring.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// a = a;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// [a] = [a];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ({a: b} = {a: b});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a.b = a.b;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a[b] = a[b];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a[b].foo = a[b].foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a['b'].foo = a['b'].foo;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// a &= a;
    /// var a = a;
    /// let a = a;
    /// const a = a;
    /// [a, b] = [b, a];
    /// ```
    ///
    pub NoSelfAssign {
        version: "1.0.0",
        name: "noSelfAssign",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-self-assign"),
            RuleSource::Clippy("self_assignment"),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoSelfAssign {
    type Query = Ast<JsAssignmentExpression>;
    type State = IdentifiersLike;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok();
        let right = node.right().ok();
        let operator = node.operator().ok();
        let mut result = vec![];
        if let Some(operator) = operator {
            if matches!(
                operator,
                JsAssignmentOperator::Assign
                    | JsAssignmentOperator::LogicalAndAssign
                    | JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign
            ) {
                if let (Some(left), Some(right)) = (left, right) {
                    if let Ok(pair) = AnyAssignmentLike::try_from((left, right)) {
                        compare_assignment_like(pair, &mut result);
                    }
                }
            }
        }
        result.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, identifier_like: &Self::State) -> Option<RuleDiagnostic> {
        let name = identifier_like.name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                identifier_like.right_range(),
                markup! {
                    {{name.text_trimmed()}}" is assigned to itself."
                },
            )
            .detail(
                identifier_like.left_range(),
                markup! {
                    "This is where is assigned."
                },
            )
            .note(markup! {
             "Self assignments have no effect and can be removed."
            }),
        )
    }
}

/// It traverses an [AnyAssignmentLike] and tracks the identifiers that have the same name
fn compare_assignment_like(
    any_assignment_like: AnyAssignmentLike,
    incorrect_identifiers: &mut Vec<IdentifiersLike>,
) {
    let same_identifiers = SameIdentifiers {
        current_assignment_like: any_assignment_like,
        assignment_queue: VecDeque::new(),
    };

    for identifier_like in same_identifiers {
        if with_same_identifiers(&identifier_like).is_some() {
            incorrect_identifiers.push(identifier_like);
        }
    }
}

/// Convenient type to iterate through all the identifiers that can be found
/// inside an assignment expression.
struct SameIdentifiers {
    /// The current assignment-like that is being inspected
    current_assignment_like: AnyAssignmentLike,
    /// A queue of assignments-like that are inspected during the traversal.
    ///
    /// The queue is used to "save" the current traversal when it's needed to start a new one.
    ///
    /// These kind of cases happen, for example, when we have a code like
    ///
    /// ```js
    /// [ a, [b, c], d ]
    /// ```
    ///
    /// After `a`, we find a new assignment-like pattern that requires a new traversal, so we save the
    /// current traversal in the queue and we start a new one. When the inner traversal is finished,
    /// we resume the previous one.
    assignment_queue: VecDeque<AnyAssignmentLike>,
}

impl SameIdentifiers {
    /// Any assignment-like has a left arm and a right arm. Both arms needs to be "similar"
    /// in order to be compared. If during the traversal part of each arm differ, they are then ignored
    ///
    /// The iterator logic makes sure to return the next eligible assignment-like.
    fn next_assignment_like(&mut self) -> Option<AnyAssignmentLike> {
        let current_assignment_like = &mut self.current_assignment_like;
        match current_assignment_like {
            AnyAssignmentLike::Arrays { left, right } => {
                let new_assignment_like = Self::next_array_assignment(left, right);
                // In case we have nested array/object structures, we save the current
                // pair and we restore it once this iterator is consumed
                if let Some(new_assignment_like) = new_assignment_like.as_ref() {
                    if new_assignment_like.has_sub_structures() {
                        self.assignment_queue
                            .push_back(self.current_assignment_like.clone());
                    }
                }
                new_assignment_like
            }
            AnyAssignmentLike::Object { left, right } => {
                let new_assignment_like = Self::next_object_assignment(left, right);
                // In case we have nested array/object structures, we save the current
                // pair and we restore it once this iterator is consumed
                if let Some(new_assignment_like) = new_assignment_like.as_ref() {
                    if new_assignment_like.has_sub_structures() {
                        self.assignment_queue
                            .push_back(self.current_assignment_like.clone());
                    }
                }
                new_assignment_like
            }
            AnyAssignmentLike::StaticExpression { left, right } => {
                Self::next_static_expression(left, right)
            }
            AnyAssignmentLike::None | AnyAssignmentLike::Identifiers { .. } => {
                let new_assignment = self.current_assignment_like.clone();
                self.current_assignment_like = AnyAssignmentLike::None;
                Some(new_assignment)
            }
        }
    }

    /// Handles cases where the assignment is something like
    /// ```js
    /// [a] = [a]
    /// ```
    fn next_array_assignment(
        left: &mut AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayAssignmentPatternElement>,
        right: &mut AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayElement>,
    ) -> Option<AnyAssignmentLike> {
        if let (Some(left_element), Some(right_element)) = (left.next(), right.next()) {
            let left_element = left_element.ok()?;
            let right_element = right_element.ok()?;

            if let (
                AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(left),
                AnyJsArrayElement::AnyJsExpression(right),
            ) = (left_element, right_element)
            {
                if left.init().is_some() {
                    // Allow self assign when the pattern has a default value.
                    return Some(AnyAssignmentLike::None);
                }
                let new_assignment_like =
                    AnyAssignmentLike::try_from((left.pattern().ok()?, right)).ok()?;

                return Some(new_assignment_like);
            }
        }
        Some(AnyAssignmentLike::None)
    }

    /// Computes the next assignment like.
    ///
    /// It handles code like:
    ///
    /// ```js
    /// {a} = {b}
    /// ```
    fn next_object_assignment(
        left: &mut AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectAssignmentPatternMember>,
        right: &mut AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectMember>,
    ) -> Option<AnyAssignmentLike> {
        let result = if let (Some(left_element), Some(right_element)) = (left.next(), right.next())
        {
            let left_element = left_element.ok()?;
            let right_element = right_element.ok()?;

            match (left_element, right_element) {
                // matches {a} = {a}
                (
                    AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                        left,
                    ),
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(right),
                ) => AnyAssignmentLike::Identifiers(IdentifiersLike::IdentifierAndReference(
                    left.identifier().ok()?,
                    right.name().ok()?,
                )),

                (
                    AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(left),
                    AnyJsObjectMember::JsPropertyObjectMember(right),
                ) => {
                    let left = left.pattern().ok()?;
                    let right = right.value().ok()?;
                    match (left, right) {
                        // matches {a: b} = {a: b}
                        (
                            AnyJsAssignmentPattern::AnyJsAssignment(
                                AnyJsAssignment::JsIdentifierAssignment(left),
                            ),
                            AnyJsExpression::JsIdentifierExpression(right),
                        ) => AnyAssignmentLike::Identifiers(
                            IdentifiersLike::IdentifierAndReference(left, right.name().ok()?),
                        ),
                        // matches {a: [b]} = {a: [b]}
                        (
                            AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                            AnyJsExpression::JsArrayExpression(right),
                        ) => AnyAssignmentLike::Arrays {
                            left: left.elements().iter(),
                            right: right.elements().iter(),
                        },
                        // matches {a: {b}} = {a: {b}}
                        (
                            AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
                            AnyJsExpression::JsObjectExpression(right),
                        ) => AnyAssignmentLike::Object {
                            left: left.properties().iter(),
                            right: right.members().iter(),
                        },
                        _ => AnyAssignmentLike::None,
                    }
                }
                _ => AnyAssignmentLike::None,
            }
        } else {
            AnyAssignmentLike::None
        };

        Some(result)
    }

    /// Computes the next static expression.
    ///
    /// It handles codes like:
    ///
    /// ```js
    /// a.b = a.b;
    /// a[b] = a[b];
    /// ```
    fn next_static_expression(
        left: &mut AnyJsAssignmentExpressionLikeIterator,
        right: &mut AnyJsAssignmentExpressionLikeIterator,
    ) -> Option<AnyAssignmentLike> {
        if let (Some(left_item), Some(right_item)) = (left.next(), right.next()) {
            let (left_name, left_reference) = left_item;
            let (right_name, right_reference) = right_item;

            if let Ok(identifier_like) = IdentifiersLike::try_from((left_name, right_name)) {
                if with_same_identifiers(&identifier_like).is_some() {
                    if let (Some(left_reference), Some(right_reference)) =
                        (left_reference, right_reference)
                    {
                        if with_same_identifiers(&IdentifiersLike::References(
                            left_reference,
                            right_reference,
                        ))
                        .is_some()
                        {
                            let source_identifier = IdentifiersLike::try_from((
                                left.source_member.clone(),
                                right.source_member.clone(),
                            ))
                            .ok()?;
                            return Some(AnyAssignmentLike::Identifiers(source_identifier));
                        }
                    } else {
                        return Self::next_static_expression(left, right);
                    }
                }
            }
        }
        Some(AnyAssignmentLike::None)
    }
}

impl Iterator for SameIdentifiers {
    type Item = IdentifiersLike;

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self.current_assignment_like, AnyAssignmentLike::None) {
            return None;
        }

        loop {
            let new_assignment_like = self.next_assignment_like()?;

            match new_assignment_like {
                // if we are here, it's plausible that we consumed the current iterator and we have to
                // resume the previous one
                AnyAssignmentLike::None => {
                    // we still have assignments-like to complete, so we continue the loop
                    if let Some(pair) = self.assignment_queue.pop_front() {
                        self.current_assignment_like = pair;
                        continue;
                    }
                    // the queue is empty
                    else {
                        return None;
                    }
                }
                AnyAssignmentLike::Identifiers(identifier_like) => {
                    return Some(identifier_like);
                }

                // we have a sub structure, which means we queue the current assignment,
                // and inspect the sub structure
                AnyAssignmentLike::StaticExpression { .. }
                | AnyAssignmentLike::Object { .. }
                | AnyAssignmentLike::Arrays { .. } => {
                    self.assignment_queue
                        .push_back(self.current_assignment_like.clone());
                    self.current_assignment_like = new_assignment_like;
                    continue;
                }
            }
        }
    }
}

impl FusedIterator for SameIdentifiers {}

/// A convenient iterator that continues to return the nested [JsStaticMemberExpression]
#[derive(Debug, Clone)]
struct AnyJsAssignmentExpressionLikeIterator {
    source_member: AnyNameLike,
    source_object: AnyJsExpression,
    current_member_expression: Option<AnyAssignmentExpressionLike>,
    drained: bool,
}

impl AnyJsAssignmentExpressionLikeIterator {
    fn from_static_member_expression(source: &JsStaticMemberExpression) -> SyntaxResult<Self> {
        Ok(Self {
            source_member: source.member().map(AnyNameLike::from)?,
            source_object: source.object()?,
            current_member_expression: None,
            drained: false,
        })
    }

    fn from_static_member_assignment(source: &JsStaticMemberAssignment) -> SyntaxResult<Self> {
        Ok(Self {
            source_member: source.member().map(AnyNameLike::from)?,
            source_object: source.object()?,
            current_member_expression: None,
            drained: false,
        })
    }

    fn from_computed_member_assignment(source: &JsComputedMemberAssignment) -> SyntaxResult<Self> {
        Ok(Self {
            source_member: source.member().and_then(|expression| match expression {
                AnyJsExpression::JsIdentifierExpression(node) => {
                    Ok(AnyNameLike::from(node.name()?))
                }
                AnyJsExpression::AnyJsLiteralExpression(node) => Ok(AnyNameLike::from(node)),
                _ => Err(SyntaxError::MissingRequiredChild),
            })?,
            source_object: source.object()?,
            current_member_expression: None,
            drained: false,
        })
    }

    fn from_computed_member_expression(source: &JsComputedMemberExpression) -> SyntaxResult<Self> {
        Ok(Self {
            source_member: source.member().and_then(|expression| match expression {
                AnyJsExpression::JsIdentifierExpression(node) => {
                    Ok(AnyNameLike::from(node.name()?))
                }
                AnyJsExpression::AnyJsLiteralExpression(node) => Ok(AnyNameLike::from(node)),

                _ => Err(SyntaxError::MissingRequiredChild),
            })?,
            source_object: source.object()?,
            current_member_expression: None,
            drained: false,
        })
    }
}

impl Iterator for AnyJsAssignmentExpressionLikeIterator {
    type Item = (AnyNameLike, Option<JsReferenceIdentifier>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.drained {
            return None;
        }

        let (name, object) =
            if let Some(current_member_expression) = self.current_member_expression.as_ref() {
                (
                    current_member_expression.member()?,
                    current_member_expression.object()?,
                )
            } else {
                (self.source_member.clone(), self.source_object.clone())
            };

        let reference = match object {
            AnyJsExpression::JsStaticMemberExpression(expression) => {
                self.current_member_expression =
                    Some(AnyAssignmentExpressionLike::from(expression));
                None
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                // the left side of the static member expression is an identifier, which means that we can't
                // go any further and we should mark the iterator and drained
                self.drained = true;
                Some(identifier.name().ok()?)
            }

            AnyJsExpression::JsComputedMemberExpression(computed_expression) => {
                self.current_member_expression = Some(
                    AnyAssignmentExpressionLike::JsComputedMemberExpression(computed_expression),
                );
                None
            }
            _ => return None,
        };

        Some((name, reference))
    }
}

impl FusedIterator for AnyJsAssignmentExpressionLikeIterator {}

/// Convenient type to map assignments that have similar arms
#[derive(Debug, Clone)]
enum AnyAssignmentLike {
    /// No assignments. This variant is used to signal that there aren't any more assignments
    /// to inspect
    None,
    /// To track identifiers that will be compared and check if they are the same.
    Identifiers(IdentifiersLike),
    /// To track array assignment-likes
    /// ```js
    /// [a] = [a]
    /// ```
    ///
    /// It stores a left iterator and a right iterator. Using iterators is useful to signal when
    /// there aren't any more elements to inspect.
    Arrays {
        left: AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayAssignmentPatternElement>,
        right: AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayElement>,
    },
    /// To track assignments like
    /// ```js
    /// {a} = {a}
    /// ```
    ///
    /// It stores a left iterator and a right iterator. Using iterators is useful to signal when
    /// there aren't any more elements to inspect.
    Object {
        left: AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectAssignmentPatternMember>,
        right: AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectMember>,
    },
    /// To track static expressions
    /// ```js
    /// a.b = a.b;
    /// a[b] = a[b];
    /// ```
    ///
    /// It stores a left iterator and a right iterator. Using iterators is useful to signal when
    /// there aren't any more elements to inspect.
    StaticExpression {
        left: AnyJsAssignmentExpressionLikeIterator,
        right: AnyJsAssignmentExpressionLikeIterator,
    },
}

declare_node_union! {
    pub AnyNameLike = AnyJsName | JsReferenceIdentifier | AnyJsLiteralExpression
}

declare_node_union! {
    pub AnyAssignmentExpressionLike = JsStaticMemberExpression | JsComputedMemberExpression
}

impl AnyAssignmentExpressionLike {
    fn member(&self) -> Option<AnyNameLike> {
        match self {
            AnyAssignmentExpressionLike::JsStaticMemberExpression(node) => {
                node.member().ok().map(AnyNameLike::from)
            }
            AnyAssignmentExpressionLike::JsComputedMemberExpression(node) => {
                node.member().ok().and_then(|node| {
                    Some(match node {
                        AnyJsExpression::JsIdentifierExpression(node) => node.name().ok()?.into(),
                        AnyJsExpression::AnyJsLiteralExpression(node) => node.into(),
                        _ => return None,
                    })
                })
            }
        }
    }

    fn object(&self) -> Option<AnyJsExpression> {
        match self {
            AnyAssignmentExpressionLike::JsStaticMemberExpression(node) => node.object().ok(),
            AnyAssignmentExpressionLike::JsComputedMemberExpression(node) => node.object().ok(),
        }
    }
}

impl AnyAssignmentLike {
    const fn has_sub_structures(&self) -> bool {
        matches!(
            self,
            AnyAssignmentLike::Arrays { .. } | AnyAssignmentLike::Object { .. }
        )
    }
}

impl TryFrom<(AnyJsAssignmentPattern, AnyJsExpression)> for AnyAssignmentLike {
    type Error = SyntaxError;

    fn try_from(
        (left, right): (AnyJsAssignmentPattern, AnyJsExpression),
    ) -> Result<Self, Self::Error> {
        Ok(match (left, right) {
            (
                AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                AnyJsExpression::JsArrayExpression(right),
            ) => AnyAssignmentLike::Arrays {
                left: left.elements().iter(),
                right: right.elements().iter(),
            },

            (
                AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
                AnyJsExpression::JsObjectExpression(right),
            ) => AnyAssignmentLike::Object {
                left: left.properties().iter(),
                right: right.members().iter(),
            },

            (
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                    left,
                )),
                AnyJsExpression::JsIdentifierExpression(right),
            ) => AnyAssignmentLike::Identifiers(IdentifiersLike::IdentifierAndReference(
                left,
                right.name()?,
            )),
            (
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsStaticMemberAssignment(
                    left,
                )),
                AnyJsExpression::JsStaticMemberExpression(right),
            ) => AnyAssignmentLike::StaticExpression {
                left: AnyJsAssignmentExpressionLikeIterator::from_static_member_assignment(&left)?,
                right: AnyJsAssignmentExpressionLikeIterator::from_static_member_expression(
                    &right,
                )?,
            },

            (
                AnyJsAssignmentPattern::AnyJsAssignment(
                    AnyJsAssignment::JsComputedMemberAssignment(left),
                ),
                AnyJsExpression::JsComputedMemberExpression(right),
            ) => AnyAssignmentLike::StaticExpression {
                left: AnyJsAssignmentExpressionLikeIterator::from_computed_member_assignment(
                    &left,
                )?,
                right: AnyJsAssignmentExpressionLikeIterator::from_computed_member_expression(
                    &right,
                )?,
            },
            _ => AnyAssignmentLike::None,
        })
    }
}

/// Convenient type that pair possible combination of "identifiers" like that we can find.
///
/// Each variant has two types:
/// - the first one is the identifier found in the left arm of the assignment;
/// - the second one is the identifier found in the right arm of the assignment;
#[derive(Debug, Clone)]
pub enum IdentifiersLike {
    /// To store identifiers found in code like:
    ///
    /// ```js
    /// a = a;
    /// [a] = [a];
    /// {a} = {a};
    /// ```
    IdentifierAndReference(JsIdentifierAssignment, JsReferenceIdentifier),
    /// To store identifiers found in code like:
    ///
    /// ```js
    /// a[b] = a[b];
    /// ```
    References(JsReferenceIdentifier, JsReferenceIdentifier),
    /// To store identifiers found in code like:
    ///
    /// ```js
    /// a.b = a.b;
    /// ```
    Name(JsName, JsName),
    /// To store identifiers found in code like:
    ///
    /// ```js
    /// a.#b = a.#b;
    /// ```
    PrivateName(JsPrivateName, JsPrivateName),
    /// To store identifiers found in code like:
    ///
    /// ```js
    /// a['b'].d = a['b'].d
    /// a[3].d = a[4].d
    /// ```
    Literal(AnyJsLiteralExpression, AnyJsLiteralExpression),
}

impl TryFrom<(AnyNameLike, AnyNameLike)> for IdentifiersLike {
    type Error = ();

    fn try_from((left, right): (AnyNameLike, AnyNameLike)) -> Result<Self, Self::Error> {
        match (left, right) {
            (
                AnyNameLike::AnyJsName(AnyJsName::JsName(left)),
                AnyNameLike::AnyJsName(AnyJsName::JsName(right)),
            ) => Ok(Self::Name(left, right)),
            (
                AnyNameLike::AnyJsName(AnyJsName::JsPrivateName(left)),
                AnyNameLike::AnyJsName(AnyJsName::JsPrivateName(right)),
            ) => Ok(Self::PrivateName(left, right)),

            (
                AnyNameLike::JsReferenceIdentifier(left),
                AnyNameLike::JsReferenceIdentifier(right),
            ) => Ok(Self::References(left, right)),

            (
                AnyNameLike::AnyJsLiteralExpression(left),
                AnyNameLike::AnyJsLiteralExpression(right),
            ) => Ok(Self::Literal(left, right)),

            _ => Err(()),
        }
    }
}

impl IdentifiersLike {
    fn left_range(&self) -> TextRange {
        match self {
            IdentifiersLike::IdentifierAndReference(left, _) => left.range(),
            IdentifiersLike::Name(left, _) => left.range(),
            IdentifiersLike::PrivateName(left, _) => left.range(),
            IdentifiersLike::References(left, _) => left.range(),
            IdentifiersLike::Literal(left, _) => left.range(),
        }
    }

    fn right_range(&self) -> TextRange {
        match self {
            IdentifiersLike::IdentifierAndReference(_, right) => right.range(),
            IdentifiersLike::Name(_, right) => right.range(),
            IdentifiersLike::PrivateName(_, right) => right.range(),
            IdentifiersLike::References(_, right) => right.range(),
            IdentifiersLike::Literal(_, right) => right.range(),
        }
    }

    fn name(&self) -> Option<JsSyntaxToken> {
        match self {
            IdentifiersLike::IdentifierAndReference(_, right) => right.value_token().ok(),
            IdentifiersLike::Name(_, right) => right.value_token().ok(),
            IdentifiersLike::PrivateName(_, right) => right.value_token().ok(),
            IdentifiersLike::References(_, right) => right.value_token().ok(),
            IdentifiersLike::Literal(_, right) => right.value_token().ok(),
        }
    }
}

/// Checks if the left identifier and the right reference have the same name
fn with_same_identifiers(identifiers_like: &IdentifiersLike) -> Option<()> {
    let (left_value, right_value) = match &identifiers_like {
        IdentifiersLike::IdentifierAndReference(left, right) => {
            let left_value = left.name_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::Name(left, right) => {
            let left_value = left.value_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::PrivateName(left, right) => {
            let left_value = left.value_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::References(left, right) => {
            let left_value = left.value_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::Literal(left, right) => match (left, right) {
            (
                AnyJsLiteralExpression::JsStringLiteralExpression(left),
                AnyJsLiteralExpression::JsStringLiteralExpression(right),
            ) => {
                let left_value = left.value_token().ok()?;
                let right_value = right.value_token().ok()?;
                (left_value, right_value)
            }

            (
                AnyJsLiteralExpression::JsNumberLiteralExpression(left),
                AnyJsLiteralExpression::JsNumberLiteralExpression(right),
            ) => {
                let left_value = left.value_token().ok()?;
                let right_value = right.value_token().ok()?;
                (left_value, right_value)
            }

            _ => return None,
        },
    };

    if inner_string_text(&left_value) == inner_string_text(&right_value) {
        Some(())
    } else {
        None
    }
}
