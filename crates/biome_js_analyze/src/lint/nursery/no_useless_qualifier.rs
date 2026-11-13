use std::iter;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, AnyTsName, JsExport, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxToken, JsUnaryExpression, JsUnaryOperator, T,
    TsEnumDeclaration, TsModuleBlock, TsModuleDeclaration, TsQualifiedName,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction, declare_node_union};
use biome_rule_options::no_useless_qualifier::NoUselessQualifierOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow unnecessary namespace and enum qualifiers.
    ///
    /// The members of a TypeScript namespace or enum are usually accessed through
    /// the name of the namespace or enum, for example `Direction.Up`. The
    /// `Direction.` part is called a qualifier.
    ///
    /// Inside the namespace or enum itself, the members can be used by their name
    /// alone, so the qualifier only repeats what is already known.
    ///
    /// This rule doesn't report a qualifier when the member name alone would refer
    /// to something else, for example because a local variable has the same name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// enum Direction {
    ///     Up,
    ///     Down = Direction.Up + 1,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// namespace Shapes {
    ///     export type Point = { x: number; y: number };
    ///     const origin: Shapes.Point = { x: 0, y: 0 };
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// namespace Config {
    ///     export const timeout = 1000;
    ///     export function getTimeout() {
    ///         return Config.timeout;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Direction {
    ///     Up,
    ///     Down = Up + 1,
    /// }
    ///
    /// namespace Shapes {
    ///     export type Point = { x: number; y: number };
    ///     const origin: Point = { x: 0, y: 0 };
    /// }
    ///
    /// namespace Outer {
    ///     export type Id = number;
    ///     namespace Inner {
    ///         type Id = string;
    ///         // `Id` alone would refer to `Inner.Id`.
    ///         const id: Outer.Id = 0;
    ///     }
    /// }
    /// ```
    ///
    pub NoUselessQualifier {
        version: "2.6.7",
        name: "noUselessQualifier",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-qualifier").same()],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    /// A name accessed through a qualifier: `A.B` in a type, or `A.b` in an expression.
    pub AnyJsQualifiedName = TsQualifiedName | JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Rule for NoUselessQualifier {
    type Query = Semantic<AnyJsQualifiedName>;
    /// The access whose qualifier is unnecessary. For `A.B.c`, this is either
    /// `A.B.c` or `A.B`.
    type State = AnyJsQualifiedName;
    type Signals = Option<Self::State>;
    type Options = NoUselessQualifierOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // The outermost access of a chain such as `A.B.c` checks the whole chain.
        if node.parent::<AnyJsQualifiedName>().is_some() {
            return None;
        }
        let model = ctx.model();

        // The accesses of the chain, from the innermost (`A.B`) to the outermost (`A.B.c`).
        let mut accesses: Vec<_> = iter::successors(Some(node.clone()), |access| {
            AnyJsQualifiedName::cast(access.qualifier()?.into_syntax())
        })
        .collect();
        accesses.reverse();
        let root = match accesses.first()?.qualifier()? {
            AnyJsQualifier::AnyTsName(AnyTsName::JsReferenceIdentifier(reference)) => reference,
            AnyJsQualifier::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
                expression,
            )) => expression.name().ok()?,
            _ => return None,
        };
        let declaration = model
            .binding(&root)?
            .syntax()
            .ancestors()
            .skip(1)
            .find(|ancestor| ancestor.kind() != JsSyntaxKind::TS_QUALIFIED_MODULE_NAME)
            .and_then(AnyTsQualifierTarget::cast)?;

        let names = enclosing_names(node, &declaration)?;

        // The number of names of the chain that match the enclosing declarations.
        let matched = iter::once(true)
            .chain(
                accesses
                    .iter()
                    .zip(names.iter().skip(1))
                    .map(|(access, name)| {
                        access.member_token().is_some_and(|member| {
                            member.text_trimmed() == name.token.text_trimmed()
                        })
                    }),
            )
            .take_while(|is_match| *is_match)
            .count();

        accesses
            .iter()
            .zip(&names)
            .take(matched)
            .rev()
            .find(|(access, name)| {
                let Some(target) = &name.body else {
                    return false;
                };
                if is_delete_operand(access) {
                    return false;
                }
                access.member_token().is_some_and(|member| {
                    refers_to_member(model, access, target, member.text_trimmed())
                })
            })
            .map(|(access, _)| access.clone())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, access: &Self::State) -> Option<RuleDiagnostic> {
        let qualifier = access.qualifier()?;
        let member = access.member_token()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                qualifier.range(),
                markup! {
                    "The qualifier before "<Emphasis>{member.text_trimmed()}</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "Inside a namespace or an enum, its members can be used by their name alone, so the qualifier only repeats what is already known."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, access: &Self::State) -> Option<JsRuleAction> {
        // Removing the qualifier would also remove the comments inside it.
        if access.syntax().has_inner_comments() {
            return None;
        }
        let member = make::ident(access.member_token()?.text_trimmed());
        let mut mutation = ctx.root().begin();
        match access {
            AnyJsQualifiedName::TsQualifiedName(name) => mutation.replace_node(
                AnyTsName::from(name.clone()),
                make::js_reference_identifier(member).into(),
            ),
            AnyJsQualifiedName::JsStaticMemberExpression(expression) => mutation.replace_node(
                AnyJsExpression::from(expression.clone()),
                make::js_identifier_expression(make::js_reference_identifier(member)).into(),
            ),
            AnyJsQualifiedName::JsStaticMemberAssignment(assignment) => mutation.replace_node(
                AnyJsAssignment::from(assignment.clone()),
                make::js_identifier_assignment(member).into(),
            ),
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the qualifier." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    /// The part before the dot of an [AnyJsQualifiedName].
    pub AnyJsQualifier = AnyTsName | AnyJsExpression
}

declare_node_union! {
    /// A declaration whose members can be accessed through a qualifier.
    pub AnyTsQualifierTarget = TsModuleDeclaration | TsEnumDeclaration
}

/// One name of an enclosing namespace or enum.
///
/// `namespace A.B {}` has two names, `A` and `B`. Only the last name owns the body.
struct EnclosingName {
    token: JsSyntaxToken,
    /// The declaration whose body belongs to this name.
    body: Option<AnyTsQualifierTarget>,
}

impl AnyJsQualifiedName {
    /// Returns the part before the dot: `A` in `A.B`.
    fn qualifier(&self) -> Option<AnyJsQualifier> {
        Some(match self {
            Self::TsQualifiedName(name) => name.left().ok()?.into(),
            Self::JsStaticMemberExpression(expression) => expression.object().ok()?.into(),
            Self::JsStaticMemberAssignment(assignment) => assignment.object().ok()?.into(),
        })
    }

    /// Returns the name after the dot: `B` in `A.B`.
    fn member_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::TsQualifiedName(name) => name.right().ok()?.value_token().ok(),
            Self::JsStaticMemberExpression(expression) => {
                expression.member().ok()?.as_js_name()?.value_token().ok()
            }
            Self::JsStaticMemberAssignment(assignment) => {
                assignment.member().ok()?.as_js_name()?.value_token().ok()
            }
        }
    }
}

/// Returns the names of `declaration` and of the declarations nested inside it
/// that enclose `node`, from the outermost to the innermost.
///
/// The names stop at the first declaration that can't be reached with a
/// qualifier from the previous one, because it isn't exported.
///
/// Returns `None` if `declaration` doesn't enclose `node`.
fn enclosing_names(
    node: &AnyJsQualifiedName,
    declaration: &AnyTsQualifierTarget,
) -> Option<Vec<EnclosingName>> {
    let mut declarations: Vec<_> = node
        .syntax()
        .ancestors()
        .skip(1)
        .filter_map(AnyTsQualifierTarget::cast)
        .collect();
    let index = declarations
        .iter()
        .position(|ancestor| ancestor == declaration)?;
    declarations.truncate(index + 1);
    declarations.reverse();

    let mut names = Vec::new();
    for (index, declaration) in declarations.iter().enumerate() {
        if index > 0 && !is_exported_member(&declarations[index - 1], declaration) {
            break;
        }
        let tokens: Vec<_> = match declaration {
            AnyTsQualifierTarget::TsModuleDeclaration(module) => module
                .name()
                .ok()?
                .syntax()
                .descendants_tokens(Direction::Next)
                .filter(|token| token.kind() != T![.])
                .collect(),
            AnyTsQualifierTarget::TsEnumDeclaration(enumeration) => {
                vec![
                    enumeration
                        .id()
                        .ok()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok()?,
                ]
            }
        };
        let last = tokens.len().checked_sub(1)?;
        names.extend(
            tokens
                .into_iter()
                .enumerate()
                .map(|(index, token)| EnclosingName {
                    token,
                    body: (index == last).then(|| declaration.clone()),
                }),
        );
    }
    Some(names)
}

/// Returns whether `member` is declared with `export` directly in the body of `parent`.
fn is_exported_member(parent: &AnyTsQualifierTarget, member: &AnyTsQualifierTarget) -> bool {
    member
        .parent::<JsExport>()
        .and_then(|export| export.syntax().grand_parent())
        .and_then(TsModuleBlock::cast)
        .and_then(|block| block.syntax().parent())
        .is_some_and(|block_parent| &block_parent == parent.syntax())
}

/// Returns whether `name`, written without a qualifier at the position of
/// `access`, refers to a member of `target`.
fn refers_to_member(
    model: &SemanticModel,
    access: &AnyJsQualifiedName,
    target: &AnyTsQualifierTarget,
    name: &str,
) -> bool {
    // Keywords such as `class` can be enum member names, but not variable names.
    // `await` is only a contextual keyword, but it's reserved in modules.
    if JsSyntaxKind::from_keyword(name)
        .is_some_and(|kind| kind.is_non_contextual_keyword() || kind == T![await])
    {
        return false;
    }
    let target_range = target.syntax().text_trimmed_range();
    // The semantic model doesn't index enum members by name, so the members of
    // the enclosing enums are looked up in the syntax tree.
    let mut enclosing_enums = access
        .syntax()
        .ancestors()
        .skip(1)
        .filter_map(TsEnumDeclaration::cast)
        .peekable();
    for mut scope in model.scope(access.syntax()).ancestors() {
        let is_target = scope.range() == target_range;
        if is_target {
            // When several scopes share a range, such as in a file that only
            // contains a namespace, `SemanticModel::scope` returns the outermost
            // one. The members of `target` are in the innermost one.
            while let Some(child) = scope.children().find(|child| child.range() == target_range) {
                scope = child;
            }
        }
        let declaration = match enclosing_enums
            .next_if(|enumeration| enumeration.syntax().text_trimmed_range() == scope.range())
        {
            // Every enum member can be accessed through the enum name.
            Some(enumeration) => declares_enum_member(&enumeration, name).then_some(true),
            None => scope.get_binding(name).map(|binding| binding.is_exported()),
        };
        if let Some(is_accessible_through_qualifier) = declaration {
            return is_target && is_accessible_through_qualifier;
        }
        if is_target {
            return false;
        }
    }
    false
}

fn declares_enum_member(enumeration: &TsEnumDeclaration, name: &str) -> bool {
    enumeration.members().iter().flatten().any(|member| {
        member
            .name()
            .ok()
            .and_then(|member_name| member_name.name())
            .is_some_and(|member_name| member_name.text() == name)
    })
}

/// Returns whether `access` is the operand of `delete`, which doesn't accept a
/// plain variable name. Parentheses and type assertions around `access` are
/// ignored, as in `delete (A.x as any)`.
fn is_delete_operand(access: &AnyJsQualifiedName) -> bool {
    let AnyJsQualifiedName::JsStaticMemberExpression(expression) = access else {
        return false;
    };
    AnyJsExpression::from(expression.clone())
        .outer_expression()
        .and_then(|outer| outer.parent::<JsUnaryExpression>())
        .and_then(|unary| unary.operator().ok())
        .is_some_and(|operator| operator == JsUnaryOperator::Delete)
}
