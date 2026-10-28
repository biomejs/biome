use crate::{CssRuleAction, utils::get_longhand_sub_properties};
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_factory::make::{
    css_declaration, css_declaration_block, css_declaration_list,
    css_declaration_or_at_rule_block, css_declaration_or_at_rule_list,
    css_declaration_or_rule_block, css_declaration_or_rule_list, css_declaration_with_semicolon,
    css_generic_component_value_list, css_generic_property, css_identifier, scss_expression,
    scss_expression_item_list,
};
use biome_css_syntax::{
    AnyCssBogusNode, AnyCssDeclaration, AnyCssDeclarationOrAtRule, AnyCssDeclarationOrRule,
    AnyCssFunction, AnyCssGenericComponentValue, AnyCssGenericPropertyValueOrExpression, AnyCssValue,
    AnyScssExpressionItem, CssDeclarationBlock, CssDeclarationOrAtRuleBlock,
    CssDeclarationOrRuleBlock, CssDeclarationWithSemicolon, CssFunction,
    CssGenericComponentValueList, CssGenericProperty, CssSyntaxKind, CssSyntaxToken,
    ScssExpressionItemList, decode_css_identifier,
};
use biome_diagnostics::Severity;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Direction, SyntaxNode, TextRange,
    TriviaPiece, TriviaPieceKind, declare_node_union,
};
use biome_rule_options::use_logical_properties::{
    UseLogicalPropertiesDirection, UseLogicalPropertiesOptions,
};
use biome_string_case::StrLikeExtension;

declare_node_union! {
    AnyCssDeclarationList = CssDeclarationBlock | CssDeclarationOrAtRuleBlock | CssDeclarationOrRuleBlock
}

declare_lint_rule! {
    /// Prefer logical CSS properties over physical properties.
    ///
    /// Physical properties such as `left`, `margin-left`, and `width` describe fixed directions or
    /// dimensions. Logical properties such as `inset-inline-start`, `margin-inline-start`, and
    /// `inline-size` adapt when text runs right to left or uses a vertical writing mode.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   margin: 1px 2px;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   padding: var(--spacing) 2px;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   margin-block-start: 1px;
    ///   margin-block-end: 2px;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   width: 100%;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   top: 0;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   margin-left: 1rem;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   border-left: 1px solid;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   float: left;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   text-align: right;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   justify-content: left;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   inline-size: anchor-size(width);
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   inset-block-start: anchor(bottom);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   inline-size: 100%;
    ///   inset-block-start: 0;
    ///   margin-inline-start: 1rem;
    ///   border-inline-start: 1px solid;
    ///   float: inline-start;
    ///   text-align: end;
    ///   justify-content: start;
    ///   inline-size: anchor-size(self-inline);
    ///   block-size: anchor-size(self-block);
    ///   inset-block-start: anchor(end);
    /// }
    /// ```
    ///
    /// A multi-value physical shorthand always applies values to physical sides. Use separate
    /// logical properties when values differ by axis. A four-sided shorthand is only equivalent
    /// when all four values are identical, so the rule does not combine differing axis values
    /// into one. Shorthands containing `var()`, `env()`, `attr()`, or a SCSS variable are reported
    /// without an automatic fix because these values can expand to multiple values. Check the
    /// substitution before splitting the shorthand manually.
    ///
    /// ```css
    /// p {
    ///   margin: 1px;
    ///   margin: var(--spacing);
    ///   margin-block: 1px 2px;
    ///   color: red;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `direction`
    ///
    /// Sets the text direction used to replace left and right properties and to order the inline
    /// values when splitting a four-value physical shorthand. Use `"ltr"` for left-to-right text
    /// or `"rtl"` for right-to-left text. Defaults to `"ltr"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "direction": "rtl"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// p {
    ///   margin-left: 1rem;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// p {
    ///   margin: 1px 2px 3px 4px;
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// p {
    ///   margin-inline-end: 1rem;
    /// }
    /// ```
    ///
    pub UseLogicalProperties {
        version: "2.5.15",
        name: "useLogicalProperties",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseLogicalProperties {
    type Query = Ast<CssGenericProperty>;
    type State = UseLogicalPropertiesState;
    type Signals = Box<[Self::State]>;
    type Options = UseLogicalPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let property = ctx.query();
        let Ok(name) = property.name() else {
            return Box::default();
        };
        let Ok(name_token) = name.declaration() else {
            return Box::default();
        };
        let normalized_name = name_token
            .text_trimmed()
            .to_ascii_lowercase_cow();
        let direction = ctx.options().direction();
        let mut states = Vec::new();

        if let Some(logical_property) =
            physical_to_logical_property(normalized_name.as_ref(), direction)
        {
            states.push(UseLogicalPropertiesState {
                span: name.range(),
                token: name_token.clone(),
                violation: LogicalPropertiesViolation::PropertyName {
                    replacement: logical_property,
                },
            });
        }

        if let Some(state) =
            split_shorthand_violation(property, normalized_name.as_ref(), direction)
        {
            states.push(state);
        }

        if let Some(state) =
            merge_longhands_violation(property, normalized_name.as_ref(), direction)
        {
            states.push(state);
        }

        collect_value_violations(property, normalized_name.as_ref(), direction, &mut states);

        states.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(rule_category!(), state.span, state.message()).note(
            markup! {
                "Logical properties adapt better to different writing modes and layout directions."
            },
        );

        if matches!(
            &state.violation,
            LogicalPropertiesViolation::LegacyAlignmentValue
        ) {
            return Some(diagnostic.note(markup! {
                "The legacy keyword only allows left, right, or center."
            }));
        }

        Some(match &state.violation {
            LogicalPropertiesViolation::SplitShorthand { .. } => diagnostic.note(markup! {
                "Multi-value shorthands always map to physical sides."
            }),
            LogicalPropertiesViolation::DynamicSplitShorthand => diagnostic
                .note(markup! {
                    "The variable or function may supply multiple values, so check its substitution and split the shorthand manually."
                }),
            LogicalPropertiesViolation::MergeLonghands { .. } => diagnostic,
            _ => {
                if let Some(replacement) = state.replacement() {
                    let decoded = decode_css_identifier(state.token.text_trimmed());
                    let physical = decoded.to_ascii_lowercase_cow();
                    diagnostic.note(markup! {
                        "Replace "<Emphasis>{physical.as_ref()}</Emphasis>" with "<Emphasis>{replacement}</Emphasis>"."
                    })
                } else {
                    diagnostic.note(markup! {
                        "Choose a logical alignment value based on the layout axis."
                    })
                }
            }
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let mut mutation = ctx.root().begin();
        let action_message = match &state.violation {
            LogicalPropertiesViolation::DynamicSplitShorthand => return None,
            LogicalPropertiesViolation::SplitShorthand {
                value_parts,
                declaration,
                list,
                block_property,
                inline_property,
                block_indices,
                inline_indices,
            } => {
                let block_indices = collapse_split_indices(value_parts, block_indices)?;
                let inline_indices = collapse_split_indices(value_parts, inline_indices)?;
                let block = make_split_declaration(
                    value_parts,
                    declaration,
                    block_property,
                    &block_indices,
                    &block_indices,
                    &inline_indices,
                    true,
                )?;
                let inline = make_split_declaration(
                    value_parts,
                    declaration,
                    inline_property,
                    &inline_indices,
                    &block_indices,
                    &inline_indices,
                    false,
                )?;
                replace_declaration_with_two(&mut mutation, list, declaration, block, inline)?;
                markup! {
                    "Replace with "<Emphasis>{block_property}</Emphasis>" and "<Emphasis>{inline_property}</Emphasis>"."
                }
                .to_owned()
            }
            LogicalPropertiesViolation::MergeLonghands {
                start,
                end,
                target,
            } => {
                merge_declarations(&mut mutation, start, end, target)?;
                markup! { "Combine these properties as "<Emphasis>{target}</Emphasis>"." }.to_owned()
            }
            _ => {
                let replacement = state.replacement()?;
                let new_token =
                    CssSyntaxToken::new_detached(state.token.kind(), replacement, [], []);
                mutation.replace_token_transfer_trivia(state.token.clone(), new_token);
                markup! { "Use the logical CSS replacement." }.to_owned()
            }
        };

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            action_message,
            mutation,
        ))
    }
}

pub struct UseLogicalPropertiesState {
    /// Range of the physical property name or value highlighted by the diagnostic.
    span: TextRange,
    /// Syntax token containing the flagged text; a fix replaces this token when available.
    token: CssSyntaxToken,
    violation: LogicalPropertiesViolation,
}

enum LogicalPropertiesViolation {
    PropertyName {
        replacement: &'static str,
    },
    PropertyValue {
        replacement: &'static str,
    },
    JustifyContentValue,
    LegacyAlignmentValue,
    AnchorSizeValue {
        replacement: &'static str,
    },
    AnchorValue {
        replacement: &'static str,
    },
    SplitShorthand {
        value_parts: PropertyValueParts,
        declaration: CssDeclarationWithSemicolon,
        list: AnyCssDeclarationList,
        block_property: &'static str,
        inline_property: &'static str,
        block_indices: Vec<usize>,
        inline_indices: Vec<usize>,
    },
    DynamicSplitShorthand,
    MergeLonghands {
        start: CssDeclarationWithSemicolon,
        end: CssDeclarationWithSemicolon,
        target: &'static str,
    },
}

#[derive(Clone)]
enum PropertyValueParts {
    Css(CssGenericComponentValueList),
    Scss(ScssExpressionItemList),
}

#[derive(Clone)]
enum PropertyValuePart {
    Css(AnyCssGenericComponentValue),
    Scss(AnyScssExpressionItem),
}

impl PropertyValueParts {
    fn from_property(property: &CssGenericProperty) -> Option<Self> {
        match property.value().ok()? {
            AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(values) => {
                Some(Self::Css(values))
            }
            AnyCssGenericPropertyValueOrExpression::ScssExpression(expression) => {
                Some(Self::Scss(expression.items()))
            }
            _ => None,
        }
    }

    fn parts(&self) -> Vec<PropertyValuePart> {
        match self {
            Self::Css(values) => values.iter().map(PropertyValuePart::Css).collect(),
            Self::Scss(values) => values.iter().map(PropertyValuePart::Scss).collect(),
        }
    }

    fn build_value(
        &self,
        parts: Vec<PropertyValuePart>,
    ) -> Option<AnyCssGenericPropertyValueOrExpression> {
        Some(match self {
            Self::Css(_) => {
                let values = parts
                    .into_iter()
                    .map(|part| match part {
                        PropertyValuePart::Css(value) => Some(value),
                        PropertyValuePart::Scss(_) => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(
                    css_generic_component_value_list(values),
                )
            }
            Self::Scss(_) => {
                let values = parts
                    .into_iter()
                    .map(|part| match part {
                        PropertyValuePart::Scss(value) => Some(value),
                        PropertyValuePart::Css(_) => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                AnyCssGenericPropertyValueOrExpression::ScssExpression(scss_expression(
                    scss_expression_item_list(values),
                ))
            }
        })
    }
}

impl PropertyValuePart {
    fn syntax(&self) -> &SyntaxNode<biome_css_syntax::CssLanguage> {
        match self {
            Self::Css(value) => value.syntax(),
            Self::Scss(value) => value.syntax(),
        }
    }

    fn any_css_value(&self) -> Option<&AnyCssValue> {
        match self {
            Self::Css(value) => value.as_any_css_value(),
            Self::Scss(value) => value.as_any_css_value(),
        }
    }

    fn with_trailing_space(self) -> Option<Self> {
        match self {
            Self::Css(value) => Some(Self::Css(node_with_trailing_space(value)?)),
            Self::Scss(value) => Some(Self::Scss(node_with_trailing_space(value)?)),
        }
    }

    fn without_trailing_space(self) -> Option<Self> {
        match self {
            Self::Css(value) => Some(Self::Css(node_without_trailing_space(value)?)),
            Self::Scss(value) => Some(Self::Scss(node_without_trailing_space(value)?)),
        }
    }

    fn without_trailing_trivia(self) -> Option<Self> {
        match self {
            Self::Css(value) => Some(Self::Css(node_without_trailing_trivia(value)?)),
            Self::Scss(value) => Some(Self::Scss(node_without_trailing_trivia(value)?)),
        }
    }

    fn append_comments(self, source: &Self) -> Option<Self> {
        match (self, source) {
            (Self::Css(value), Self::Css(source)) => {
                Some(Self::Css(node_with_appended_comments(value, source)?))
            }
            (Self::Scss(value), Self::Scss(source)) => {
                Some(Self::Scss(node_with_appended_comments(value, source)?))
            }
            _ => None,
        }
    }
}

impl UseLogicalPropertiesState {
    fn message(&self) -> &'static str {
        match self.violation {
            LogicalPropertiesViolation::PropertyName { .. } => {
                "Use logical CSS properties over physical ones."
            }
            LogicalPropertiesViolation::PropertyValue { .. } => {
                "Use logical CSS values over physical ones."
            }
            LogicalPropertiesViolation::JustifyContentValue => {
                "Use logical CSS values over physical ones."
            }
            LogicalPropertiesViolation::LegacyAlignmentValue => {
                "Use logical CSS values over physical ones."
            }
            LogicalPropertiesViolation::AnchorSizeValue { .. } => {
                "Use a logical size in anchor-size()."
            }
            LogicalPropertiesViolation::AnchorValue { .. } => "Use a logical side in anchor().",
            LogicalPropertiesViolation::SplitShorthand { .. } => {
                "Use logical CSS longhands instead of a multi-value physical shorthand."
            }
            LogicalPropertiesViolation::DynamicSplitShorthand => {
                "Use logical CSS longhands instead of a multi-value physical shorthand."
            }
            LogicalPropertiesViolation::MergeLonghands { .. } => {
                "These logical longhands can be combined into a shorthand."
            }
        }
    }

    fn replacement(&self) -> Option<&'static str> {
        match &self.violation {
            LogicalPropertiesViolation::PropertyName { replacement }
            | LogicalPropertiesViolation::PropertyValue { replacement }
            | LogicalPropertiesViolation::AnchorSizeValue { replacement }
            | LogicalPropertiesViolation::AnchorValue { replacement } => Some(replacement),
            LogicalPropertiesViolation::JustifyContentValue
            | LogicalPropertiesViolation::LegacyAlignmentValue
            | LogicalPropertiesViolation::SplitShorthand { .. }
            | LogicalPropertiesViolation::DynamicSplitShorthand
            | LogicalPropertiesViolation::MergeLonghands { .. } => None,
        }
    }
}

/// Maps a physical property name to its logical `(ltr, rtl)` replacement names.
static LOGICAL_PROPERTIES: phf::Map<&'static str, (&'static str, &'static str)> = phf::phf_map! {
    // Sizing properties
    "width" => ("inline-size", "inline-size"),
    "min-width" => ("min-inline-size", "min-inline-size"),
    "max-width" => ("max-inline-size", "max-inline-size"),
    "height" => ("block-size", "block-size"),
    "min-height" => ("min-block-size", "min-block-size"),
    "max-height" => ("max-block-size", "max-block-size"),
    // Positioning properties
    "top" => ("inset-block-start", "inset-block-start"),
    "right" => ("inset-inline-end", "inset-inline-start"),
    "bottom" => ("inset-block-end", "inset-block-end"),
    "left" => ("inset-inline-start", "inset-inline-end"),
    // Margin properties
    "margin-top" => ("margin-block-start", "margin-block-start"),
    "margin-right" => ("margin-inline-end", "margin-inline-start"),
    "margin-bottom" => ("margin-block-end", "margin-block-end"),
    "margin-left" => ("margin-inline-start", "margin-inline-end"),
    // Padding properties
    "padding-top" => ("padding-block-start", "padding-block-start"),
    "padding-right" => ("padding-inline-end", "padding-inline-start"),
    "padding-bottom" => ("padding-block-end", "padding-block-end"),
    "padding-left" => ("padding-inline-start", "padding-inline-end"),
    // Border top properties
    "border-top" => ("border-block-start", "border-block-start"),
    "border-top-color" => ("border-block-start-color", "border-block-start-color"),
    "border-top-style" => ("border-block-start-style", "border-block-start-style"),
    "border-top-width" => ("border-block-start-width", "border-block-start-width"),
    // Border bottom properties
    "border-bottom" => ("border-block-end", "border-block-end"),
    "border-bottom-color" => ("border-block-end-color", "border-block-end-color"),
    "border-bottom-style" => ("border-block-end-style", "border-block-end-style"),
    "border-bottom-width" => ("border-block-end-width", "border-block-end-width"),
    // Border left properties
    "border-left" => ("border-inline-start", "border-inline-end"),
    "border-left-color" => ("border-inline-start-color", "border-inline-end-color"),
    "border-left-style" => ("border-inline-start-style", "border-inline-end-style"),
    "border-left-width" => ("border-inline-start-width", "border-inline-end-width"),
    // Border right properties
    "border-right" => ("border-inline-end", "border-inline-start"),
    "border-right-color" => ("border-inline-end-color", "border-inline-start-color"),
    "border-right-style" => ("border-inline-end-style", "border-inline-start-style"),
    "border-right-width" => ("border-inline-end-width", "border-inline-start-width"),
    // Border radius properties
    "border-top-left-radius" => ("border-start-start-radius", "border-start-end-radius"),
    "border-top-right-radius" => ("border-start-end-radius", "border-start-start-radius"),
    "border-bottom-left-radius" => ("border-end-start-radius", "border-end-end-radius"),
    "border-bottom-right-radius" => ("border-end-end-radius", "border-end-start-radius"),
};

fn physical_to_logical_property(
    property: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    let (ltr, rtl) = LOGICAL_PROPERTIES.get(property)?;
    Some(match direction {
        UseLogicalPropertiesDirection::Ltr => ltr,
        UseLogicalPropertiesDirection::Rtl => rtl,
    })
}

fn split_shorthand_violation(
    property: &CssGenericProperty,
    property_name: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<UseLogicalPropertiesState> {
    let logical_names = logical_shorthand_names(property_name)?;
    let value_parts = PropertyValueParts::from_property(property)?;
    let components = value_parts.parts();
    if !(2..=4).contains(&components.len())
        || components.iter().any(|component| {
            is_unsupported_split_part(component)
                || component_contains_css_wide_keyword(component)
        })
    {
        return None;
    }

    let property_token = property.name().ok()?.declaration().ok()?;
    if components.iter().any(is_dynamic_function) {
        return Some(UseLogicalPropertiesState {
            span: property_token.text_trimmed_range(),
            token: property_token,
            violation: LogicalPropertiesViolation::DynamicSplitShorthand,
        });
    }

    let (block_property, inline_property) = logical_names;
    let (block_indices, inline_indices) = match components.len() {
        2 => (vec![0], vec![1]),
        3 => (vec![0, 2], vec![1]),
        4 => (
            vec![0, 2],
            match direction {
                UseLogicalPropertiesDirection::Ltr => vec![3, 1],
                UseLogicalPropertiesDirection::Rtl => vec![1, 3],
            },
        ),
        _ => return None,
    };

    let declaration = property.syntax().ancestors().find_map(|node| {
        CssDeclarationWithSemicolon::cast(node)
    })?;
    let list = property
        .syntax()
        .ancestors()
        .find_map(AnyCssDeclarationList::cast)?;

    Some(UseLogicalPropertiesState {
        span: property_token.text_trimmed_range(),
        token: property_token,
        violation: LogicalPropertiesViolation::SplitShorthand {
            value_parts,
            declaration,
            list,
            block_property,
            inline_property,
            block_indices,
            inline_indices,
        },
    })
}

fn logical_shorthand_names(property_name: &str) -> Option<(&'static str, &'static str)> {
    Some(match property_name {
        "margin" => ("margin-block", "margin-inline"),
        "padding" => ("padding-block", "padding-inline"),
        "inset" => ("inset-block", "inset-inline"),
        "scroll-margin" => ("scroll-margin-block", "scroll-margin-inline"),
        "scroll-padding" => ("scroll-padding-block", "scroll-padding-inline"),
        "border-width" => ("border-block-width", "border-inline-width"),
        "border-style" => ("border-block-style", "border-inline-style"),
        "border-color" => ("border-block-color", "border-inline-color"),
        _ => return None,
    })
}

fn collapse_split_indices(
    value_parts: &PropertyValueParts,
    indices: &[usize],
) -> Option<Vec<usize>> {
    let components = value_parts.parts();
    if let [first, second] = indices
        && property_value_parts_equal(&components[*first], &components[*second])
    {
        return Some(vec![*first]);
    }
    Some(indices.to_vec())
}

fn property_value_parts_equal(first: &PropertyValuePart, second: &PropertyValuePart) -> bool {
    let mut first_tokens = first
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token());
    let mut second_tokens = second
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token());

    loop {
        match (first_tokens.next(), second_tokens.next()) {
            (Some(first), Some(second))
                if first.kind() == second.kind()
                    && first.text_trimmed() == second.text_trimmed() => {}
            (None, None) => return true,
            _ => return false,
        }
    }
}

fn property_value_lists_equal(first: &[PropertyValuePart], second: &[PropertyValuePart]) -> bool {
    first.len() == second.len()
        && first
            .iter()
            .zip(second)
            .all(|(first, second)| property_value_parts_equal(first, second))
}

fn is_dynamic_function(component: &PropertyValuePart) -> bool {
    component
        .any_css_value()
        .is_some_and(is_dynamic_css_value)
        || component
            .syntax()
            .descendants()
            .any(|node| AnyCssValue::cast(node).is_some_and(|value| is_dynamic_css_value(&value)))
}

fn is_dynamic_css_value(value: &AnyCssValue) -> bool {
    let AnyCssValue::AnyCssFunction(function) = value else {
        return matches!(value, AnyCssValue::ScssVariable(_));
    };

    match function {
        AnyCssFunction::CssAttrFunction(_) => true,
        AnyCssFunction::CssFunction(function) => function_name(function).is_some_and(|token| {
            let name = decode_css_identifier(token.text_trimmed());
            matches!(
                name.to_ascii_lowercase_cow().as_ref(),
                "var" | "env"
            )
        }),
        AnyCssFunction::CssIfFunction(_)
        | AnyCssFunction::CssUrlFunction(_)
        | AnyCssFunction::ScssLegacyIfFunction(_) => false,
    }
}

fn is_unsupported_split_part(component: &PropertyValuePart) -> bool {
    component
        .syntax()
        .descendants()
        .any(|node| {
            AnyCssBogusNode::can_cast(node.kind())
                || matches!(
                    node.kind(),
                    CssSyntaxKind::SCSS_INTERPOLATION | CssSyntaxKind::CSS_METAVARIABLE
                )
        })
}

fn component_contains_css_wide_keyword(component: &PropertyValuePart) -> bool {
    component.any_css_value().is_some_and(|value| {
        value_identifier_token(value).is_some_and(|token| {
            let keyword = decode_css_identifier(token.text_trimmed());
            token.kind().is_css_wide_keyword()
                || matches!(
                    keyword.to_ascii_lowercase_cow().as_ref(),
                    "initial" | "inherit" | "unset" | "revert" | "revert-layer"
                )
        })
    })
}

fn property_value_has_interpolation_or_bogus(values: &[PropertyValuePart]) -> bool {
    values.iter().any(|component| {
        component.syntax().descendants().any(|node| {
            AnyCssBogusNode::can_cast(node.kind())
                || node.kind() == CssSyntaxKind::SCSS_INTERPOLATION
        })
    })
}

fn declaration_list_items(
    list: &AnyCssDeclarationList,
) -> Vec<Option<CssDeclarationWithSemicolon>> {
    match list {
        AnyCssDeclarationList::CssDeclarationBlock(block) => block
            .declarations()
            .iter()
            .map(|item| item.as_css_declaration_with_semicolon().cloned())
            .collect(),
        AnyCssDeclarationList::CssDeclarationOrAtRuleBlock(block) => block
            .items()
            .iter()
            .map(|item| item.as_css_declaration_with_semicolon().cloned())
            .collect(),
        AnyCssDeclarationList::CssDeclarationOrRuleBlock(block) => block
            .items()
            .iter()
            .map(|item| item.as_css_declaration_with_semicolon().cloned())
            .collect(),
    }
}

fn generic_property_from_wrapper(
    wrapper: &CssDeclarationWithSemicolon,
) -> Option<CssGenericProperty> {
    wrapper
        .declaration()
        .ok()?
        .property()
        .ok()?
        .as_css_generic_property()
        .cloned()
}

fn property_name_token(property: &CssGenericProperty) -> Option<CssSyntaxToken> {
    property.name().ok()?.declaration().ok()
}

fn merge_longhands_violation(
    property: &CssGenericProperty,
    current_name: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<UseLogicalPropertiesState> {
    let (start_name, end_name, target) = logical_merge_group(current_name)?;
    let list = property
        .syntax()
        .ancestors()
        .find_map(AnyCssDeclarationList::cast)?;
    let items = declaration_list_items(&list);
    let mut start_member = None;
    let mut end_member = None;
    let mut start_count = 0;
    let mut end_count = 0;
    let mut current_index = None;
    let current_wrapper = property
        .syntax()
        .ancestors()
        .find_map(CssDeclarationWithSemicolon::cast)?;

    for (index, wrapper) in items.iter().enumerate() {
        let Some(wrapper) = wrapper else {
            continue;
        };
        let Some(candidate) = generic_property_from_wrapper(wrapper) else {
            continue;
        };
        let Some(candidate_name_token) = property_name_token(&candidate) else {
            continue;
        };
        let candidate_name = candidate_name_token
            .text_trimmed()
            .to_ascii_lowercase_cow();
        if candidate_name == start_name {
            start_count += 1;
            start_member = Some((index, wrapper.clone(), candidate));
        } else if candidate_name == end_name {
            end_count += 1;
            end_member = Some((index, wrapper.clone(), candidate));
        }
        if wrapper.syntax() == current_wrapper.syntax() {
            current_index = Some(index);
        }
    }

    if start_count != 1 || end_count != 1 {
        return None;
    }

    let (start_index, start, start_property) = start_member?;
    let (end_index, end, end_property) = end_member?;
    let first_index = start_index.min(end_index);
    let second_index = start_index.max(end_index);
    if current_index? != first_index
        || items[first_index..=second_index]
            .iter()
            .any(|item| item.is_none())
    {
        return None;
    }

    let start_declaration = start.declaration().ok()?;
    let end_declaration = end.declaration().ok()?;
    if start_declaration.important().is_some() != end_declaration.important().is_some() {
        return None;
    }

    let start_values = PropertyValueParts::from_property(&start_property)?;
    let end_values = PropertyValueParts::from_property(&end_property)?;
    let start_parts = start_values.parts();
    let end_parts = end_values.parts();
    if start_parts.is_empty()
        || end_parts.is_empty()
        || property_value_has_interpolation_or_bogus(&start_parts)
        || property_value_has_interpolation_or_bogus(&end_parts)
        || start_parts
            .iter()
            .chain(&end_parts)
            .any(|part| is_dynamic_function(part) || component_contains_css_wide_keyword(part))
        || (matches!(target, "border-block" | "border-inline")
            && !property_value_lists_equal(&start_parts, &end_parts))
        || declaration_has_comments(&start)
        || declaration_has_comments(&end)
        || has_comments_between(&start, &end, &items[first_index + 1..second_index])
        || has_conflicting_declaration(
            &items[first_index + 1..second_index],
            start_name,
            end_name,
            direction,
        )
    {
        return None;
    }

    let name_token = property.name().ok()?.declaration().ok()?;
    Some(UseLogicalPropertiesState {
        span: name_token.text_trimmed_range(),
        token: name_token,
        violation: LogicalPropertiesViolation::MergeLonghands {
            start,
            end,
            target,
        },
    })
}

fn logical_merge_group(property_name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    LOGICAL_MERGE_GROUPS
        .iter()
        .copied()
        .find(|(start, end, _)| property_name == *start || property_name == *end)
}

const LOGICAL_MERGE_GROUPS: &[(&str, &str, &str)] = &[
    ("margin-block-start", "margin-block-end", "margin-block"),
    ("margin-inline-start", "margin-inline-end", "margin-inline"),
    ("padding-block-start", "padding-block-end", "padding-block"),
    ("padding-inline-start", "padding-inline-end", "padding-inline"),
    ("inset-block-start", "inset-block-end", "inset-block"),
    ("inset-inline-start", "inset-inline-end", "inset-inline"),
    (
        "scroll-margin-block-start",
        "scroll-margin-block-end",
        "scroll-margin-block",
    ),
    (
        "scroll-margin-inline-start",
        "scroll-margin-inline-end",
        "scroll-margin-inline",
    ),
    (
        "scroll-padding-block-start",
        "scroll-padding-block-end",
        "scroll-padding-block",
    ),
    (
        "scroll-padding-inline-start",
        "scroll-padding-inline-end",
        "scroll-padding-inline",
    ),
    ("border-block-start", "border-block-end", "border-block"),
    ("border-inline-start", "border-inline-end", "border-inline"),
    (
        "border-block-start-width",
        "border-block-end-width",
        "border-block-width",
    ),
    (
        "border-inline-start-width",
        "border-inline-end-width",
        "border-inline-width",
    ),
    (
        "border-block-start-style",
        "border-block-end-style",
        "border-block-style",
    ),
    (
        "border-inline-start-style",
        "border-inline-end-style",
        "border-inline-style",
    ),
    (
        "border-block-start-color",
        "border-block-end-color",
        "border-block-color",
    ),
    (
        "border-inline-start-color",
        "border-inline-end-color",
        "border-inline-color",
    ),
];

fn has_comments_between(
    first: &CssDeclarationWithSemicolon,
    last: &CssDeclarationWithSemicolon,
    between: &[Option<CssDeclarationWithSemicolon>],
) -> bool {
    first
        .syntax()
        .last_token()
        .is_some_and(|token| token.has_trailing_comments())
        || last
            .syntax()
            .first_token()
            .is_some_and(|token| token.has_leading_comments())
        || between.iter().flatten().any(|wrapper| {
            wrapper
                .syntax()
                .descendants_with_tokens(Direction::Next)
                .filter_map(|element| element.into_token())
                .any(|token| token.has_leading_comments() || token.has_trailing_comments())
        })
}

fn declaration_has_comments(declaration: &CssDeclarationWithSemicolon) -> bool {
    declaration
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token())
        .any(|token| token.has_leading_comments() || token.has_trailing_comments())
}

fn has_conflicting_declaration(
    between: &[Option<CssDeclarationWithSemicolon>],
    start_name: &str,
    end_name: &str,
    direction: UseLogicalPropertiesDirection,
) -> bool {
    between.iter().flatten().any(|wrapper| {
        let Some(property) = generic_property_from_wrapper(wrapper) else {
            return false;
        };
        let Some(name_token) = property_name_token(&property) else {
            return false;
        };
        let name = name_token.text_trimmed().to_ascii_lowercase_cow();
        property_writes_logical_side(&name, start_name, end_name, direction)
    })
}

fn property_writes_logical_side(
    property: &str,
    start_name: &str,
    end_name: &str,
    direction: UseLogicalPropertiesDirection,
) -> bool {
    property_writes_property(property, start_name, direction)
        || property_writes_property(property, end_name, direction)
}

fn property_writes_property(
    property: &str,
    target: &str,
    direction: UseLogicalPropertiesDirection,
) -> bool {
    if property == "all" {
        return true;
    }

    let property = physical_to_logical_property(property, direction).unwrap_or(property);
    let target = physical_to_logical_property(target, direction).unwrap_or(target);

    if property == target {
        return true;
    }

    if get_longhand_sub_properties(property)
        .iter()
        .any(|longhand| property_writes_property(longhand, target, direction))
        || border_side_subproperties(property)
            .iter()
            .any(|longhand| property_writes_property(longhand, target, direction))
        || LOGICAL_MERGE_GROUPS
            .iter()
            .filter(|(_, _, shorthand)| *shorthand == property)
            .any(|(start, end, _)| {
                property_writes_property(start, target, direction)
                    || property_writes_property(end, target, direction)
            })
    {
        return true;
    }

    get_longhand_sub_properties(target)
        .iter()
        .chain(border_side_subproperties(target))
        .any(|longhand| property_writes_property(property, longhand, direction))
        || LOGICAL_MERGE_GROUPS
            .iter()
            .filter(|(_, _, shorthand)| *shorthand == target)
            .any(|(start, end, _)| {
                property_writes_property(property, start, direction)
                    || property_writes_property(property, end, direction)
            })
}

fn border_side_subproperties(property: &str) -> &'static [&'static str] {
    match property {
        "border-block-start" => &[
            "border-block-start-color",
            "border-block-start-style",
            "border-block-start-width",
        ],
        "border-block-end" => &[
            "border-block-end-color",
            "border-block-end-style",
            "border-block-end-width",
        ],
        "border-inline-start" => &[
            "border-inline-start-color",
            "border-inline-start-style",
            "border-inline-start-width",
        ],
        "border-inline-end" => &[
            "border-inline-end-color",
            "border-inline-end-style",
            "border-inline-end-width",
        ],
        _ => &[],
    }
}

fn make_split_declaration(
    source_values: &PropertyValueParts,
    original: &CssDeclarationWithSemicolon,
    name: &'static str,
    indices: &[usize],
    block_indices: &[usize],
    inline_indices: &[usize],
    is_block: bool,
) -> Option<CssDeclarationWithSemicolon> {
    let property = generic_property_from_wrapper(original)?;
    let components = source_values.parts();
    let mut values = indices
        .iter()
        .map(|index| components[*index].clone())
        .collect::<Vec<_>>();

    for omitted_index in 0..components.len() {
        if block_indices.contains(&omitted_index) || inline_indices.contains(&omitted_index) {
            continue;
        }
        let retained_index = (block_indices.iter().chain(inline_indices))
            .copied()
            .find(|index| {
                property_value_parts_equal(&components[*index], &components[omitted_index])
            })?;
        if indices.contains(&retained_index) {
            let position = indices.iter().position(|index| *index == retained_index)?;
            values[position] = values[position]
                .clone()
                .append_comments(&components[omitted_index])?;
        }
    }

    let source_important = original.declaration().ok()?.important();
    let move_trailing_trivia = original.semicolon_token().is_none() && source_important.is_none();
    if move_trailing_trivia {
        let last_source_index = components.len().checked_sub(1)?;
        for (position, index) in indices.iter().enumerate() {
            if *index == last_source_index {
                values[position] = values[position].clone().without_trailing_trivia()?;
            }
        }
    }

    values = components_with_separators(values)?;
    if source_important.is_some()
        && let Some(last) = values.pop()
    {
        values.push(last.with_trailing_space()?);
    }

    let value = source_values.build_value(values)?;
    let new_property = make_logical_property(&property, name, value, is_block)?;
    let mut declaration_builder = css_declaration(new_property.into());
    if let Some(important) = source_important {
        declaration_builder = declaration_builder.with_important(important.clone());
    }
    let declaration = declaration_builder.build();
    let semicolon = if is_block {
        Some(CssSyntaxToken::new_detached(
            CssSyntaxKind::SEMICOLON,
            "; ",
            [],
            [TriviaPiece::whitespace(1)],
        ))
    } else {
        original.semicolon_token()
    };
    let mut wrapper_builder = css_declaration_with_semicolon(declaration);
    if let Some(semicolon) = semicolon {
        wrapper_builder = wrapper_builder.with_semicolon_token(semicolon);
    }
    let mut wrapper = wrapper_builder.build();

    if !is_block && move_trailing_trivia {
        wrapper = append_trailing_trivia(wrapper, original)?;
    }

    Some(wrapper)
}

fn make_logical_property(
    original: &CssGenericProperty,
    name: &'static str,
    value: AnyCssGenericPropertyValueOrExpression,
    keep_name_trivia: bool,
) -> Option<biome_css_syntax::CssGenericProperty> {
    let name_token = original.name().ok()?.declaration().ok()?;
    let name_token = if keep_name_trivia {
        token_with_text(&name_token, name)
    } else {
        CssSyntaxToken::new_detached(CssSyntaxKind::IDENT, name, [], [])
    };
    let colon = if keep_name_trivia {
        original.colon_token().ok()?
    } else {
        CssSyntaxToken::new_detached(
            CssSyntaxKind::COLON,
            ": ",
            [],
            [TriviaPiece::whitespace(1)],
        )
    };
    Some(css_generic_property(
        biome_css_syntax::AnyCssDeclarationName::CssIdentifier(css_identifier(name_token)),
        colon,
        value,
    ))
}

fn token_with_text(token: &CssSyntaxToken, replacement: &str) -> CssSyntaxToken {
    let leading = token.leading_trivia();
    let trailing = token.trailing_trivia();
    let mut text = String::with_capacity(leading.text().len() + replacement.len() + trailing.text().len());
    text.push_str(leading.text());
    text.push_str(replacement);
    text.push_str(trailing.text());
    let leading_pieces = leading
        .pieces()
        .map(|piece| TriviaPiece::new(piece.kind(), piece.text_len()))
        .collect::<Vec<_>>();
    let trailing_pieces = trailing
        .pieces()
        .map(|piece| TriviaPiece::new(piece.kind(), piece.text_len()))
        .collect::<Vec<_>>();
    CssSyntaxToken::new_detached(token.kind(), &text, leading_pieces, trailing_pieces)
}

fn components_with_separators(
    components: Vec<PropertyValuePart>,
) -> Option<Vec<PropertyValuePart>> {
    let last_index = components.len().checked_sub(1)?;
    components
        .into_iter()
        .enumerate()
        .map(|(index, component)| {
            if index == last_index {
                component.without_trailing_space()
            } else {
                component.with_trailing_space()
            }
        })
        .collect()
}

fn node_with_trailing_space<T: AstNode>(component: T) -> Option<T> {
    let syntax = component.into_syntax();
    let token = syntax.last_token()?;
    let mut trailing = token
        .trailing_trivia()
        .pieces()
        .filter(|piece| !piece.kind().is_whitespace())
        .map(|piece| (piece.kind(), piece.text().to_string()))
        .collect::<Vec<_>>();
    if !trailing.last().is_some_and(|(kind, _)| {
        kind.is_newline() || kind.is_single_line_comment()
    }) {
        trailing.push((TriviaPieceKind::Whitespace, " ".to_string()));
    }
    let new_token = token.with_trailing_trivia(
        trailing
            .iter()
            .map(|(kind, text)| (*kind, text.as_str()))
            .collect::<Vec<_>>(),
    );
    T::cast(syntax.replace_child(token.into(), new_token.into())?)
}

fn node_without_trailing_space<T: AstNode>(component: T) -> Option<T> {
    let syntax = component.into_syntax();
    let token = syntax.last_token()?;
    let trailing = token
        .trailing_trivia()
        .pieces()
        .filter(|piece| !piece.kind().is_whitespace())
        .map(|piece| (piece.kind(), piece.text().to_string()))
        .collect::<Vec<_>>();
    let new_token = token.with_trailing_trivia(
        trailing
            .iter()
            .map(|(kind, text)| (*kind, text.as_str()))
            .collect::<Vec<_>>(),
    );
    T::cast(syntax.replace_child(token.into(), new_token.into())?)
}

fn node_without_trailing_trivia<T: AstNode>(component: T) -> Option<T> {
    let syntax = component.into_syntax();
    let token = syntax.last_token()?;
    let cleared = token.with_trailing_trivia([]);
    T::cast(syntax.replace_child(token.into(), cleared.into())?)
}

fn node_with_appended_comments<T: AstNode>(component: T, source: &T) -> Option<T> {
    let syntax = component.into_syntax();
    let token = syntax.last_token()?;
    let mut trailing = token
        .trailing_trivia()
        .pieces()
        .map(|piece| (piece.kind(), piece.text().to_string()))
        .collect::<Vec<_>>();
    for element in source
        .syntax()
        .descendants_with_tokens(Direction::Next)
    {
        let Some(token) = element.into_token() else {
            continue;
        };
        trailing.extend(
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
                .filter(|piece| piece.kind().is_comment())
                .map(|piece| (piece.kind(), piece.text().to_string())),
        );
    }
    let new_token = token.with_trailing_trivia(
        trailing
            .iter()
            .map(|(kind, text)| (*kind, text.as_str()))
            .collect::<Vec<_>>(),
    );
    T::cast(syntax.replace_child(token.into(), new_token.into())?)
}

fn append_trailing_trivia(
    wrapper: CssDeclarationWithSemicolon,
    source: &CssDeclarationWithSemicolon,
) -> Option<CssDeclarationWithSemicolon> {
    let syntax = wrapper.into_syntax();
    let token = syntax.last_token()?;
    let mut pieces = token
        .trailing_trivia()
        .pieces()
        .map(|piece| (piece.kind(), piece.text().to_string()))
        .collect::<Vec<_>>();
    pieces.extend(source.syntax().last_trailing_trivia()?.pieces().map(|piece| {
        (piece.kind(), piece.text().to_string())
    }));
    let new_token = token.with_trailing_trivia(
        pieces
            .iter()
            .map(|(kind, text)| (*kind, text.as_str()))
            .collect::<Vec<_>>(),
    );
    CssDeclarationWithSemicolon::cast(syntax.replace_child(token.into(), new_token.into())?)
}

fn replace_declaration_with_two(
    mutation: &mut biome_rowan::BatchMutation<biome_css_syntax::CssLanguage>,
    list: &AnyCssDeclarationList,
    original: &CssDeclarationWithSemicolon,
    first: CssDeclarationWithSemicolon,
    second: CssDeclarationWithSemicolon,
) -> Option<()> {
    match list {
        AnyCssDeclarationList::CssDeclarationBlock(block) => {
            let items = block
                .declarations()
                .iter()
                .flat_map(|item| {
                    if item.syntax() == original.syntax() {
                        vec![
                            AnyCssDeclaration::CssDeclarationWithSemicolon(first.clone()),
                            AnyCssDeclaration::CssDeclarationWithSemicolon(second.clone()),
                        ]
                    } else {
                        vec![item]
                    }
                })
                .collect::<Vec<_>>();
            let declarations = css_declaration_list(items);
            let new_block = css_declaration_block(
                block.l_curly_token().ok()?,
                declarations,
                block.r_curly_token().ok()?,
            );
            mutation.replace_node_discard_trivia(block.clone(), new_block);
        }
        AnyCssDeclarationList::CssDeclarationOrAtRuleBlock(block) => {
            let items = block
                .items()
                .iter()
                .flat_map(|item| {
                    if item.syntax() == original.syntax() {
                        vec![
                            AnyCssDeclarationOrAtRule::CssDeclarationWithSemicolon(first.clone()),
                            AnyCssDeclarationOrAtRule::CssDeclarationWithSemicolon(second.clone()),
                        ]
                    } else {
                        vec![item]
                    }
                })
                .collect::<Vec<_>>();
            let items = css_declaration_or_at_rule_list(items);
            let new_block = css_declaration_or_at_rule_block(
                block.l_curly_token().ok()?,
                items,
                block.r_curly_token().ok()?,
            );
            mutation.replace_node_discard_trivia(block.clone(), new_block);
        }
        AnyCssDeclarationList::CssDeclarationOrRuleBlock(block) => {
            let items = block
                .items()
                .iter()
                .flat_map(|item| {
                    if item.syntax() == original.syntax() {
                        vec![
                            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(first.clone()),
                            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(second.clone()),
                        ]
                    } else {
                        vec![item]
                    }
                })
                .collect::<Vec<_>>();
            let items = css_declaration_or_rule_list(items);
            let new_block = css_declaration_or_rule_block(
                block.l_curly_token().ok()?,
                items,
                block.r_curly_token().ok()?,
            );
            mutation.replace_node_discard_trivia(block.clone(), new_block);
        }
    }
    Some(())
}

fn merge_declarations(
    mutation: &mut biome_rowan::BatchMutation<biome_css_syntax::CssLanguage>,
    start: &CssDeclarationWithSemicolon,
    end: &CssDeclarationWithSemicolon,
    target: &'static str,
) -> Option<()> {
    let start_property = generic_property_from_wrapper(start)?;
    let end_property = generic_property_from_wrapper(end)?;
    let (first_wrapper, second_wrapper, first_property) =
        if start
            .syntax()
            .first_token()?
            .text_range()
            .start()
            < end
                .syntax()
                .first_token()?
                .text_range()
                .start()
        {
            (start, end, start_property.clone())
        } else {
            (end, start, end_property.clone())
        };

    let start_values = PropertyValueParts::from_property(&start_property)?;
    let end_values = PropertyValueParts::from_property(&end_property)?;
    let start_parts = start_values.parts();
    let end_parts = end_values.parts();
    let mut components = if property_value_lists_equal(&start_parts, &end_parts) {
        let mut components = start_parts;
        if let Some(mut last) = components.pop() {
            for source in &end_parts {
                last = last.append_comments(source)?;
            }
            components.push(last);
        }
        components
    } else {
        start_parts.into_iter().chain(end_parts).collect()
    };
    components = components_with_separators(components)?;
    if first_wrapper.declaration().ok()?.important().is_some()
        && let Some(last) = components.pop()
    {
        components.push(last.with_trailing_space()?);
    }

    let value = start_values.build_value(components)?;
    let merged = make_logical_property(&first_property, target, value, true)?;
    mutation.replace_node_discard_trivia(first_property, merged);
    mutation.remove_node(second_wrapper.clone());
    Some(())
}

fn collect_value_violations(
    property: &CssGenericProperty,
    property_name: &str,
    direction: UseLogicalPropertiesDirection,
    states: &mut Vec<UseLogicalPropertiesState>,
) {
    let Ok(AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(values)) =
        property.value()
    else {
        return;
    };

    let has_legacy = property_name == "justify-items"
        && values
            .iter()
            .filter_map(|component| component.as_any_css_value().cloned())
            .filter_map(|value| value_identifier_token(&value))
            .any(|token| {
                decode_css_identifier(token.text_trimmed()).eq_ignore_ascii_case("legacy")
            });

    for value in values
        .iter()
        .filter_map(|component| component.as_any_css_value().cloned())
    {
        if let Some(token) = value_identifier_token(&value) {
            let decoded = decode_css_identifier(token.text_trimmed());
            let physical = decoded.to_ascii_lowercase_cow();
            let violation = if property_name == "justify-content"
                && matches!(physical.as_ref(), "left" | "right")
            {
                Some(LogicalPropertiesViolation::JustifyContentValue)
            } else if has_legacy && matches!(physical.as_ref(), "left" | "right") {
                Some(LogicalPropertiesViolation::LegacyAlignmentValue)
            } else {
                physical_to_logical_value(property_name, physical.as_ref(), direction).map(
                    |replacement| LogicalPropertiesViolation::PropertyValue { replacement },
                )
            };
            if let Some(violation) = violation {
                states.push(UseLogicalPropertiesState {
                    span: token.text_trimmed_range(),
                    token,
                    violation,
                });
            }
        }
    }

    for component in values.iter() {
        let Some(AnyCssValue::AnyCssFunction(AnyCssFunction::CssFunction(function))) =
            component.as_any_css_value()
        else {
            continue;
        };
        collect_function_violations(function, direction, states);
    }
}

fn collect_function_violations(
    function: &CssFunction,
    direction: UseLogicalPropertiesDirection,
    states: &mut Vec<UseLogicalPropertiesState>,
) {
    let Some(name_token) = function_name(function) else {
        return;
    };
    let decoded = decode_css_identifier(name_token.text_trimmed());
    let function_name = decoded.to_ascii_lowercase_cow();

    let function_kind = match function_name.as_ref() {
        "anchor-size" => Some(true),
        "anchor" => Some(false),
        _ => None,
    };

    for expression in function.items().iter().flatten() {
        collect_function_expression_violations(
            expression.syntax(),
            function_kind,
            direction,
            states,
        );
    }
}

fn collect_function_expression_violations(
    expression: &biome_rowan::SyntaxNode<biome_css_syntax::CssLanguage>,
    function_kind: Option<bool>,
    direction: UseLogicalPropertiesDirection,
    states: &mut Vec<UseLogicalPropertiesState>,
) {
    for child in expression.children() {
        let Some(value) = AnyCssValue::cast(child.clone()) else {
            collect_function_expression_violations(&child, function_kind, direction, states);
            continue;
        };

        if let Some(token) = value_identifier_token(&value)
            && let Some(is_anchor_size) = function_kind
        {
            let decoded = decode_css_identifier(token.text_trimmed());
            let physical = decoded.to_ascii_lowercase_cow();
            let replacement = if is_anchor_size {
                physical_to_logical_anchor_size(physical.as_ref())
            } else {
                physical_to_logical_anchor_side(physical.as_ref(), direction)
            };

            if let Some(replacement) = replacement {
                states.push(UseLogicalPropertiesState {
                    span: token.text_trimmed_range(),
                    token,
                    violation: if is_anchor_size {
                        LogicalPropertiesViolation::AnchorSizeValue {
                            replacement,
                        }
                    } else {
                        LogicalPropertiesViolation::AnchorValue {
                            replacement,
                        }
                    },
                });
            }
        }

        if let AnyCssValue::AnyCssFunction(AnyCssFunction::CssFunction(function)) = value {
            collect_function_violations(&function, direction, states);
        }
    }
}

fn function_name(function: &CssFunction) -> Option<CssSyntaxToken> {
    function
        .name()
        .ok()?
        .as_css_identifier()?
        .value_token()
        .ok()
}

fn value_identifier_token(value: &AnyCssValue) -> Option<CssSyntaxToken> {
    Some(match value {
        AnyCssValue::CssIdentifier(identifier) => identifier.value_token().ok()?,
        AnyCssValue::CssCustomIdentifier(identifier) => identifier.value_token().ok()?,
        AnyCssValue::AnyCssDashedIdentifier(identifier) => {
            identifier.as_css_dashed_identifier()?.value_token().ok()?
        }
        _ => return None,
    })
}

fn physical_to_logical_value(
    property: &str,
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match property {
        "frame-sizing" => match value {
            "content-width" => Some("content-inline-size"),
            "content-height" => Some("content-block-size"),
            _ => None,
        },
        "float" | "clear" => physical_to_logical_inline_value(value, direction),
        "text-align" | "justify-items" | "justify-self" => {
            physical_to_logical_start_end_value(value, direction)
        }
        _ => None,
    }
}

fn physical_to_logical_anchor_size(value: &str) -> Option<&'static str> {
    match value {
        "width" => Some("self-inline"),
        "height" => Some("self-block"),
        _ => None,
    }
}

fn physical_to_logical_anchor_side(
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match value {
        "top" => Some("start"),
        "bottom" => Some("end"),
        _ => physical_to_logical_start_end_value(value, direction),
    }
}

fn physical_to_logical_inline_value(
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match (value, direction) {
        ("left", UseLogicalPropertiesDirection::Ltr)
        | ("right", UseLogicalPropertiesDirection::Rtl) => Some("inline-start"),
        ("right", UseLogicalPropertiesDirection::Ltr)
        | ("left", UseLogicalPropertiesDirection::Rtl) => Some("inline-end"),
        _ => None,
    }
}

fn physical_to_logical_start_end_value(
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match (value, direction) {
        ("left", UseLogicalPropertiesDirection::Ltr)
        | ("right", UseLogicalPropertiesDirection::Rtl) => Some("start"),
        ("right", UseLogicalPropertiesDirection::Ltr)
        | ("left", UseLogicalPropertiesDirection::Rtl) => Some("end"),
        _ => None,
    }
}
