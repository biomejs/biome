use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationOrRule, AnyCssProperty, CssDeclaration, CssDeclarationOrRuleList,
    CssKeyframesAtRule, CssSyntaxKind, ScssNestingDeclaration,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, SyntaxKind, TextRange, TokenText};
use biome_rule_options::no_duplicate_properties::NoDuplicatePropertiesOptions;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow duplicate properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate properties. It ignores custom properties and declarations inside `@keyframes` blocks.
    ///
    /// ## SCSS limitations
    ///
    /// Declarations are compared within each authored declaration block. The rule does not expand
    /// mixins or includes, so duplicates introduced only after SCSS expansion are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   color: pink;
    ///   color: orange;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: pink;
    ///   background: orange;
    /// }
    /// ```
    ///
    pub NoDuplicateProperties {
        version: "1.9.4",
        name: "noDuplicateProperties",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties").same()],
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = (TextRange, (TextRange, TokenText));
    type Signals = Option<Self::State>;
    type Options = NoDuplicatePropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.syntax().parent().is_some_and(|block| {
            block
                .parent()
                .is_some_and(|parent| ScssNestingDeclaration::can_cast(parent.kind()))
        }) {
            return None;
        }

        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range())?;

        let mut declarations = Vec::new();

        for declaration in rule.declarations().iter().filter(|declaration| {
            declaration
                .declaration()
                .syntax()
                .parent()
                .and_then(|parent| parent.parent())
                .as_ref()
                == Some(node.syntax())
        }) {
            let prop = declaration.property();
            let prop_name = prop.value().ok()?;
            let prop_range = prop.range();

            let is_custom_property = prop_name.text().starts_with("--");

            if is_custom_property {
                continue;
            }

            // We skip this declaration if it's inside a keyframes block.
            if prop
                .syntax()
                .ancestors()
                .skip(1)
                .any(|node| CssKeyframesAtRule::can_cast(node.kind()))
            {
                continue;
            }

            declarations.push((prop_range, prop_range, prop_name));
        }

        for item in node.iter() {
            if let AnyCssDeclarationOrRule::ScssNestingDeclaration(nesting) = item {
                collect_scss_nesting_declarations(&nesting, None, &mut declarations);
            }
        }

        declarations.sort_by_key(|(position, _, _)| position.start());

        let mut seen: FxHashMap<TokenText, TextRange> = FxHashMap::default();

        for (_, prop_range, prop_name) in declarations {
            match seen.entry(prop_name.clone()) {
                Entry::Occupied(entry) => {
                    return Some((*entry.get(), (prop_range, prop_name)));
                }
                Entry::Vacant(entry) => {
                    entry.insert(prop_range);
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_occurrence_range, (duplicate_range, duplicate_property_name)) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                duplicate_range,
                markup! {
                    "Duplicate properties can lead to unexpected behavior and may override previous declarations unintentionally."
                },
            )
            .detail(first_occurrence_range, markup! {
                <Emphasis>{duplicate_property_name.text()}</Emphasis> " is already defined here."
            })
            .note(markup! {
                "Remove or rename the duplicate property to ensure consistent styling."
            }),
        )
    }
}

fn collect_scss_nesting_declarations(
    declaration: &ScssNestingDeclaration,
    parent_name: Option<&str>,
    declarations: &mut Vec<(TextRange, TextRange, TokenText)>,
) {
    let Ok(name_node) = declaration.name() else {
        return;
    };
    let Some(name) = name_node.identifier_text() else {
        return;
    };

    let name_range = name_node.syntax().text_trimmed_range();

    if name_node
        .syntax()
        .ancestors()
        .skip(1)
        .any(|node| CssKeyframesAtRule::can_cast(node.kind()))
    {
        return;
    }

    let expanded_name = match parent_name {
        Some(parent_name) => format!("{parent_name}-{}", name.text()),
        None => name.text().to_string(),
    };

    if declaration
        .value()
        .is_ok_and(|value| !value.syntax().text_trimmed().is_empty())
    {
        declarations.push((
            name_range,
            name_range,
            TokenText::new_raw(CssSyntaxKind::IDENT.to_raw(), &expanded_name),
        ));
    }

    let Ok(block) = declaration.block() else {
        return;
    };
    let Some(block) = block.as_css_declaration_or_rule_block() else {
        return;
    };

    for item in block.items().iter() {
        match item {
            AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(declaration) => {
                let Ok(declaration) = declaration.declaration() else {
                    continue;
                };
                let Some((name, range)) = declaration_property(&declaration) else {
                    continue;
                };
                declarations.push((
                    range,
                    range,
                    TokenText::new_raw(
                        CssSyntaxKind::IDENT.to_raw(),
                        &format!("{expanded_name}-{}", name.text()),
                    ),
                ));
            }
            AnyCssDeclarationOrRule::ScssNestingDeclaration(nesting) => {
                collect_scss_nesting_declarations(&nesting, Some(&expanded_name), declarations);
            }
            _ => {}
        }
    }
}

fn declaration_property(declaration: &CssDeclaration) -> Option<(TokenText, TextRange)> {
    match declaration.property().ok()? {
        AnyCssProperty::CssComposesProperty(property) => {
            let name = property.name().ok()?;
            let range = name.syntax().text_trimmed_range();
            Some((name.value_token().ok()?.token_text_trimmed(), range))
        }
        AnyCssProperty::CssGenericProperty(property) => {
            let name = property.name().ok()?;
            let range = name.syntax().text_trimmed_range();
            Some((name.identifier_text()?, range))
        }
        AnyCssProperty::CssBogusProperty(_) => None,
    }
}
