use crate::services::module_graph::CssModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::selector_ext::AnyCssPseudoClassFunctionSelector;
use biome_css_syntax::{
    AnyCssNestedSelector, AnyCssRelativeSelector, AnyCssSelector, CssClassSelector,
    CssNestedQualifiedRule, CssQualifiedRule, CssSyntaxKind, CssSyntaxNode, ScssAtRootAtRule,
    ScssParentSelector,
};
use biome_module_graph::{SymbolFromModuleInfo, is_class_referenced_by_importers};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, Text, TokenText, declare_node_union};
use biome_rule_options::no_unused_classes::NoUnusedClassesOptions;

declare_lint_rule! {
    /// Reports CSS class selectors that are never referenced in any JSX or HTML file.
    ///
    /// This rule checks all CSS class selectors (`.foo`) in a CSS file and verifies
    /// that each class name is referenced somewhere in a `class=` or `className=`
    /// attribute in an HTML or JSX file that imports (directly or transitively) the
    /// CSS file.
    ///
    /// Classes inside `:global(.foo)` are excluded from this check, as they are
    /// intended to be used by external consumers without explicit imports.
    ///
    /// ## SCSS limitations
    ///
    /// This rule does not evaluate SCSS. A parent selector suffix, such as `&__element` in
    /// `.block { &__element {} }`, is checked under the class name Sass builds (`block__element`)
    /// for each parent selector that ends in a class. Interpolated class names are ignored, and so
    /// are parent selector suffixes whose parent a mixin decides, as in a mixin body or an
    /// `@include` content block. `@extend` references
    /// are not treated as class references. Statically named classes inside mixin bodies are
    /// checked as authored even when the mixin is not included.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,file=styles.css
    /// .unused { color: red; }  /* Class "unused" is never referenced */
    /// .used { color: blue; }
    /// ```
    ///
    /// ```jsx,file=App.jsx
    /// import "./styles.css";
    /// export default () => <div className="used" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css,file=styles.css
    /// .button { color: blue; }
    /// .container { padding: 1rem; }
    /// ```
    ///
    /// ```jsx,file=App.jsx
    /// import "./styles.css";
    /// export default () => (
    ///     <div className="container">
    ///         <button className="button">Click</button>
    ///     </div>
    /// );
    /// ```
    ///
    pub NoUnusedClasses {
        version: "2.5.0",
        name: "noUnusedClasses",
        language: "css",
        recommended: false,
        issue_number: Some("9156"),
        domains: &[RuleDomain::Project],
    }
}

declare_node_union! {
    pub AnyClassNameSelector = CssClassSelector | ScssParentSelector
}

impl Rule for NoUnusedClasses {
    type Query = CssModuleGraph<AnyClassNameSelector>;
    type State = Text;
    type Signals = Vec<Self::State>;
    type Options = NoUnusedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Skip classes inside :global(.foo) — they are globally scoped
        if is_inside_global_pseudo(node.syntax()) {
            return Vec::new();
        }

        let db = ctx.db();
        let Some(module) = db.module_for_path(ctx.file_path()) else {
            return Vec::new();
        };
        let is_unused = |class_name: &str| {
            !is_class_referenced_by_importers(db, SymbolFromModuleInfo::new(db, class_name, module))
        };

        match node {
            AnyClassNameSelector::CssClassSelector(selector) => class_selector_name(selector)
                .filter(|class_name| is_unused(class_name.text()))
                .map(Text::from)
                .into_iter()
                .collect(),
            AnyClassNameSelector::ScssParentSelector(selector) => {
                parent_suffix_class_names(selector)
                    .into_iter()
                    .flatten()
                    .filter(|class_name| is_unused(class_name))
                    .map(Text::from)
                    .collect()
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, class_name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The CSS class "<Emphasis>{class_name.text()}</Emphasis>" is not referenced in any HTML or JSX file."
                },
            )
            .note(markup! {
                "Unused CSS classes add dead weight to your stylesheet, increase bundle size, and make the codebase harder to maintain."
            })
            .note(markup! {
                "Either use this class somewhere or remove the selector."
            }),
        )
    }
}

fn class_selector_name(selector: &CssClassSelector) -> Option<TokenText> {
    Some(
        selector
            .name()
            .ok()?
            .as_css_custom_identifier()?
            .value_token()
            .ok()?
            .token_text_trimmed(),
    )
}

/// Returns the class names Sass builds for a parent selector with a suffix, such as `block__element`
/// for `&__element` in `.block { &__element {} }`, one for each parent selector that ends in a
/// class.
///
/// Returns `None` if the suffix is interpolated or the parent selectors are unknown.
fn parent_suffix_class_names(selector: &ScssParentSelector) -> Option<Vec<String>> {
    let suffix = selector.suffix().ok()?;
    if suffix
        .items()
        .iter()
        .any(|part| part.as_scss_interpolation().is_some())
    {
        return None;
    }
    let suffix = suffix.syntax().text_trimmed().to_string();
    let mut class_names = parent_class_names(selector.syntax())?;
    class_names.sort_unstable();
    class_names.dedup();
    for class_name in &mut class_names {
        class_name.push_str(&suffix);
    }
    Some(class_names)
}

/// Returns the classes that end the selectors `&` refers to in the selector containing `node`.
/// Parent selectors that don't end in a class are left out.
///
/// Returns `None` if the parent selectors are unknown: there is no style rule around the selector,
/// or a mixin decides them, as in a mixin body or an `@include` content block.
fn parent_class_names(node: &CssSyntaxNode) -> Option<Vec<String>> {
    let mut ancestors = node.ancestors().skip_while(|ancestor| {
        !matches!(
            ancestor.kind(),
            CssSyntaxKind::CSS_QUALIFIED_RULE
                | CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE
                | CssSyntaxKind::SCSS_AT_ROOT_AT_RULE
        )
    });
    // `&` refers to the rule around the one whose selector contains `node`. In an at-rule prelude,
    // such as `@supports selector(&-x)`, Sass doesn't resolve `&`.
    let rule = ancestors.next()?;
    let selector_range = if let Some(rule) = CssQualifiedRule::cast_ref(&rule) {
        rule.prelude().range()
    } else if let Some(rule) = CssNestedQualifiedRule::cast_ref(&rule) {
        rule.prelude().range()
    } else {
        ScssAtRootAtRule::cast_ref(&rule)?.selector()?.range()
    };
    if !selector_range.contains_range(node.text_trimmed_range()) {
        return None;
    }

    let selectors: Vec<_> = loop {
        let ancestor = ancestors.next()?;
        if let Some(rule) = CssQualifiedRule::cast_ref(&ancestor) {
            break rule.prelude().iter().collect::<Result<_, _>>().ok()?;
        }
        if let Some(rule) = CssNestedQualifiedRule::cast_ref(&ancestor) {
            break rule
                .prelude()
                .iter()
                .map(|selector| match selector.ok()? {
                    AnyCssRelativeSelector::CssRelativeSelector(selector) => {
                        selector.selector().ok()
                    }
                    _ => None,
                })
                .collect::<Option<_>>()?;
        }
        if let Some(rule) = ScssAtRootAtRule::cast_ref(&ancestor)
            && let Some(selector) = rule.selector()
        {
            break selector
                .selector()
                .ok()?
                .as_css_selector_list()?
                .iter()
                .collect::<Result<_, _>>()
                .ok()?;
        }
        if matches!(
            ancestor.kind(),
            CssSyntaxKind::SCSS_MIXIN_AT_RULE | CssSyntaxKind::SCSS_INCLUDE_AT_RULE
        ) {
            return None;
        }
    };

    Some(
        selectors
            .into_iter()
            .filter_map(trailing_class_names)
            .flatten()
            .collect(),
    )
}

/// Returns the classes that end `selector`, resolving a trailing `&` or `&suffix` against the
/// rule around it.
fn trailing_class_names(mut selector: AnyCssSelector) -> Option<Vec<String>> {
    let compound = loop {
        match selector {
            AnyCssSelector::CssCompoundSelector(compound) => break compound,
            AnyCssSelector::CssComplexSelector(complex) => selector = complex.right().ok()?,
            _ => return None,
        }
    };

    if let Some(sub_selector) = compound.sub_selectors().last() {
        let class_name = class_selector_name(sub_selector.as_css_class_selector()?)?;
        return Some(vec![class_name.text().to_string()]);
    }
    if compound.simple_selector().is_some() {
        return None;
    }
    match compound.nesting_selectors().last()? {
        AnyCssNestedSelector::CssNestedSelector(_) => parent_class_names(compound.syntax()),
        AnyCssNestedSelector::ScssParentSelector(selector) => parent_suffix_class_names(&selector),
    }
}

/// Returns `true` if `node` is a descendant of a `:global(...)` pseudo-class function selector.
fn is_inside_global_pseudo(node: &CssSyntaxNode) -> bool {
    node.ancestors().any(|ancestor| {
        AnyCssPseudoClassFunctionSelector::cast(ancestor)
            .is_some_and(|selector| selector.is_global_pseudo())
    })
}
