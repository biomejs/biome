use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssPseudoClass, AnyCssPseudoElement, AnyCssSelector, AnyCssSelectorIdentifier,
    AnyCssSimpleSelector, AnyCssSubSelector, CssAttributeSelector, CssComplexSelector,
    CssCompoundSelector, CssLanguage, CssPseudoClassSelector, CssPseudoElementSelector,
    CssRelativeSelector, CssSyntaxKind, CssSyntaxToken, ScssPartialCombinatorSelector, T,
    decode_css_identifier,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxKindSet, TextRange};
use biome_rule_options::no_excessive_selector_complexity::{
    AttributeMatcherOperator, NoExcessiveSelectorComplexityOptions, SelectorCombinator,
};

declare_lint_rule! {
    /// Limit how complex selectors can be, and disallow chosen parts of selectors.
    ///
    /// Long selectors that mix many IDs, element names, attributes, and pseudo-classes are hard
    /// to read and hard to override. They also tie styles closely to the structure of the page,
    /// so small changes to the HTML can break them.
    ///
    /// This rule lets you set a maximum for each kind of selector part, and list combinators,
    /// pseudo-classes, pseudo-elements, attributes, and attribute operators that aren't
    /// allowed. The rule does nothing until at least one option is set.
    ///
    /// Each selector in a comma-separated list is checked on its own. For example, `#a, #b`
    /// contains two selectors with one ID each.
    ///
    /// Selectors inside a pseudo-class such as `:not()`, `:is()`, or `:has()` are checked as
    /// separate selectors, and don't count toward the selector that contains them. For example,
    /// `#a:not(#b)` contains two selectors with one ID each.
    ///
    /// Nested selectors are checked as written, without combining them with the selectors of
    /// their parent rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxIds": 1,
    ///         "disallowPseudoClasses": ["hover"]
    ///     }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// #main #sidebar {}
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// button:hover {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css,use_options
    /// #main .sidebar,
    /// #footer {}
    ///
    /// #main:not(#sidebar) {}
    ///
    /// button:focus {}
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxIds`
    ///
    /// Default: no limit
    ///
    /// The maximum number of ID selectors, such as `#main`, allowed in a single selector.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxIds": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// #main > #sidebar {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// #main .sidebar {}
    /// ```
    ///
    /// ### `maxTypes`
    ///
    /// Default: no limit
    ///
    /// The maximum number of type selectors allowed in a single selector. A type selector
    /// matches elements by their tag name, such as `div` or `a`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxTypes": 2
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// ul li a {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// li.item a {}
    /// ```
    ///
    /// ### `maxAttributes`
    ///
    /// Default: no limit
    ///
    /// The maximum number of attribute selectors, such as `[type="text"]`, allowed in a single
    /// selector.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxAttributes": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// [name="email"][type="text"] {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// input[type="text"] {}
    /// ```
    ///
    /// ### `maxPseudoClasses`
    ///
    /// Default: no limit
    ///
    /// The maximum number of pseudo-classes, such as `:hover` or `:first-child`, allowed in a
    /// single selector. Pseudo-elements such as `::before` don't count, including the older
    /// single-colon forms `:before`, `:after`, `:first-line`, and `:first-letter`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxPseudoClasses": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// li:first-child:hover {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a:hover::before {}
    /// ```
    ///
    /// ### `maxUniversals`
    ///
    /// Default: no limit
    ///
    /// The maximum number of universal selectors (`*`) allowed in a single selector.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxUniversals": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// * > * {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// .list > * {}
    /// ```
    ///
    /// ### `maxCompounds`
    ///
    /// Default: no limit
    ///
    /// The maximum number of compound selectors allowed in a single selector. A compound
    /// selector is a group of selectors written together without spaces or combinators. For
    /// example, `nav a.link:hover` has two compound selectors: `nav` and `a.link:hover`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxCompounds": 2
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// nav ul a {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// nav a.link:hover {}
    /// ```
    ///
    /// ### `maxCombinators`
    ///
    /// Default: no limit
    ///
    /// The maximum number of combinators allowed in a single selector. A combinator is the
    /// symbol between two compound selectors: a space, `>`, `+`, `~`, or `||`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxCombinators": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// .menu > .item > a {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// .menu > .item {}
    /// ```
    ///
    /// ### `disallowCombinators`
    ///
    /// Default: `[]`
    ///
    /// Combinators that aren't allowed. Accepted values are `" "` (a space, which selects
    /// elements inside another element), `">"`, `"+"`, `"~"`, and `"||"`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowCombinators": [">"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// .menu > .item {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// .menu .item {}
    /// ```
    ///
    /// ### `disallowPseudoClasses`
    ///
    /// Default: `[]`
    ///
    /// Pseudo-classes that aren't allowed, written without the leading colon. Names are compared
    /// without regard to letter case.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowPseudoClasses": ["hover"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// a:not(.active):hover {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a:not(.active) {}
    /// ```
    ///
    /// ### `disallowPseudoElements`
    ///
    /// Default: `[]`
    ///
    /// Pseudo-elements that aren't allowed, written without the leading colons. Names are
    /// compared without regard to letter case. The older single-colon forms `:before`,
    /// `:after`, `:first-line`, and `:first-letter` are checked as well.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowPseudoElements": ["before"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// a:before {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// input::placeholder {}
    /// ```
    ///
    /// ### `disallowAttributes`
    ///
    /// Default: `[]`
    ///
    /// Attribute names that aren't allowed in attribute selectors. Names must match exactly,
    /// including letter case.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowAttributes": ["class"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// [class*="button"] {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// input[type="text"] {}
    /// ```
    ///
    /// ### `disallowAttributeMatchers`
    ///
    /// Default: `[]`
    ///
    /// Operators that aren't allowed in attribute selectors. Accepted values are `"="`, `"~="`,
    /// `"|="`, `"^="`, `"$="`, and `"*="`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowAttributeMatchers": ["*="]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// [class*="button"] {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// [class="button"] {}
    /// ```
    ///
    /// ## See Also
    ///
    /// - To limit the number of classes in a selector, see [`noExcessiveSelectorClasses`](https://biomejs.dev/linter/rules/no-excessive-selector-classes/).
    ///
    pub NoExcessiveSelectorComplexity {
        version: "2.6.7",
        name: "noExcessiveSelectorComplexity",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintCss("selector-complexity").same()],
    }
}

#[derive(Debug)]
pub enum SelectorComplexityViolation {
    /// The selector contains `count` parts of one kind, which is more than the configured limit.
    /// The diagnostic covers the whole selector.
    TooMany { part: LimitedPart, count: usize },
    /// A part of the selector appears in one of the configured disallow lists. `name` is the
    /// token shown in the diagnostic, and `range` is the part of the selector to highlight.
    Disallowed {
        part: DisallowedPart,
        range: TextRange,
        name: CssSyntaxToken,
    },
}

impl Rule for NoExcessiveSelectorComplexity {
    type Query = Ast<AnyCssSelector>;
    type State = SelectorComplexityViolation;
    type Signals = Box<[Self::State]>;
    type Options = NoExcessiveSelectorComplexityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let selector = ctx.query();

        // A selector joined to others by a combinator is only a piece of the complete
        // selector, which is checked as a whole when the query visits it.
        if selector
            .syntax()
            .parent()
            .is_some_and(|parent| SELECTOR_PIECE_PARENT_KINDS.matches(parent.kind()))
        {
            return Box::default();
        }

        let mut walker = SelectorWalker {
            options: ctx.options(),
            parts: SelectorParts::default(),
            violations: Vec::new(),
        };

        // In `:has(> img)` and in nested rules such as `> a {}`, the leading combinator
        // belongs to the parent node rather than to the selector.
        if let Some(combinator) = selector
            .parent::<CssRelativeSelector>()
            .and_then(|relative| relative.combinator())
        {
            let range = combinator.text_trimmed_range();
            walker.visit_combinator(combinator, range);
        }

        walker.visit_selector(selector);
        walker.check_limits();
        walker.violations.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            SelectorComplexityViolation::TooMany { part, count } => {
                let max = part.limit(ctx.options())?;
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        ctx.query().range(),
                        markup! {
                            "This selector has too many "{part}": it has "{count}", but the limit is "{max}"."
                        },
                    )
                    .note(markup! {
                        "Complex selectors are hard to read and override, and break easily when the page structure changes."
                    })
                    .note(markup! {
                        "Simplify the selector, for example by giving the element a class and selecting it directly."
                    }),
                )
            }
            SelectorComplexityViolation::Disallowed { part, range, name } => {
                let diagnostic = match part {
                    DisallowedPart::Combinator
                        if name.kind() == CssSyntaxKind::CSS_SPACE_LITERAL =>
                    {
                        RuleDiagnostic::new(
                            rule_category!(),
                            range,
                            markup! {
                                "The descendant combinator (a space between selectors) isn't allowed."
                            },
                        )
                    }
                    DisallowedPart::PseudoClass => RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "The pseudo-class "<Emphasis>":"{name.text_trimmed()}</Emphasis>" isn't allowed."
                        },
                    ),
                    DisallowedPart::PseudoElement => RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "The pseudo-element "<Emphasis>"::"{name.text_trimmed()}</Emphasis>" isn't allowed."
                        },
                    ),
                    DisallowedPart::LegacyPseudoElement => RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "The pseudo-element "<Emphasis>":"{name.text_trimmed()}</Emphasis>" isn't allowed."
                        },
                    ),
                    DisallowedPart::Combinator
                    | DisallowedPart::Attribute
                    | DisallowedPart::AttributeMatcher => RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "The "{part}" "<Emphasis>{name.text_trimmed()}</Emphasis>" isn't allowed."
                        },
                    ),
                };

                Some(
                    diagnostic
                        .note(markup! {
                            "The rule's configuration disallows this "{part}" in selectors."
                        })
                        .note(markup! {
                            "Rewrite the selector without it, for example by giving the element a class and selecting it directly."
                        }),
                )
            }
        }
    }
}

/// Parent kinds of a selector that is only one side of a combinator.
const SELECTOR_PIECE_PARENT_KINDS: SyntaxKindSet<CssLanguage> =
    CssComplexSelector::KIND_SET.union(ScssPartialCombinatorSelector::KIND_SET);

/// Pseudo-elements that may also be written with a single colon, as in `a:before`.
const LEGACY_PSEUDO_ELEMENTS: &[&str] = &["after", "before", "first-letter", "first-line"];

#[derive(Clone, Copy, Debug)]
pub enum LimitedPart {
    Ids,
    Types,
    Attributes,
    PseudoClasses,
    Universals,
    Compounds,
    Combinators,
}

impl LimitedPart {
    fn limit(self, options: &NoExcessiveSelectorComplexityOptions) -> Option<u16> {
        match self {
            Self::Ids => options.max_ids(),
            Self::Types => options.max_types(),
            Self::Attributes => options.max_attributes(),
            Self::PseudoClasses => options.max_pseudo_classes(),
            Self::Universals => options.max_universals(),
            Self::Compounds => options.max_compounds(),
            Self::Combinators => options.max_combinators(),
        }
    }
}

impl biome_console::fmt::Display for LimitedPart {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Ids => "ID selectors",
            Self::Types => "type selectors",
            Self::Attributes => "attribute selectors",
            Self::PseudoClasses => "pseudo-classes",
            Self::Universals => "universal selectors",
            Self::Compounds => "compound selectors",
            Self::Combinators => "combinators",
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DisallowedPart {
    Combinator,
    PseudoClass,
    PseudoElement,
    /// A pseudo-element written with a single colon, as in `a:before`.
    LegacyPseudoElement,
    Attribute,
    AttributeMatcher,
}

impl biome_console::fmt::Display for DisallowedPart {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Combinator => "combinator",
            Self::PseudoClass => "pseudo-class",
            Self::PseudoElement | Self::LegacyPseudoElement => "pseudo-element",
            Self::Attribute => "attribute",
            Self::AttributeMatcher => "attribute operator",
        })
    }
}

/// How many parts of each limited kind one selector contains.
#[derive(Default)]
struct SelectorParts {
    ids: usize,
    types: usize,
    attributes: usize,
    pseudo_classes: usize,
    universals: usize,
    compounds: usize,
    combinators: usize,
}

/// Counts the parts of one complete selector and collects its violations.
///
/// The walk doesn't enter the arguments of pseudo-classes or pseudo-elements, because the
/// selectors inside them are checked separately.
struct SelectorWalker<'a> {
    options: &'a NoExcessiveSelectorComplexityOptions,
    parts: SelectorParts,
    violations: Vec<SelectorComplexityViolation>,
}

impl SelectorWalker<'_> {
    /// Visits the compound selectors and combinators of `root` from left to right.
    fn visit_selector(&mut self, root: &AnyCssSelector) {
        // Complex selectors nest to the left, so `a > b c` is `((a > b) c)`. Descending to the
        // leftmost piece and then climbing back up visits every piece in source order. Only the
        // right side of each complex selector is visited recursively, and that side is a single
        // compound selector.
        let mut current = root.clone();
        loop {
            let left = match &current {
                AnyCssSelector::CssComplexSelector(selector) => selector.left().ok(),
                AnyCssSelector::ScssPartialCombinatorSelector(selector) => selector.left(),
                _ => None,
            };
            match left {
                Some(left) => current = left,
                None => break,
            }
        }

        loop {
            match &current {
                AnyCssSelector::CssCompoundSelector(selector) => self.visit_compound(selector),
                AnyCssSelector::CssComplexSelector(selector) => {
                    let right = selector.right().ok();
                    if let Ok(combinator) = selector.combinator() {
                        let range = descendant_combinator_range(selector, right.as_ref())
                            .unwrap_or_else(|| combinator.text_trimmed_range());
                        self.visit_combinator(combinator, range);
                    }
                    if let Some(right) = right {
                        self.visit_selector(&right);
                    }
                }
                AnyCssSelector::ScssPartialCombinatorSelector(selector) => {
                    if let Ok(combinator) = selector.combinator() {
                        let range = combinator.text_trimmed_range();
                        self.visit_combinator(combinator, range);
                    }
                }
                AnyCssSelector::CssBogusSelector(_) | AnyCssSelector::CssMetavariable(_) => {}
            }

            if current.syntax() == root.syntax() {
                break;
            }
            match current.syntax().parent().and_then(AnyCssSelector::cast) {
                Some(parent) => current = parent,
                None => break,
            }
        }
    }

    fn visit_combinator(&mut self, token: CssSyntaxToken, range: TextRange) {
        self.parts.combinators += 1;

        let combinator = match token.kind() {
            CssSyntaxKind::CSS_SPACE_LITERAL => SelectorCombinator::Descendant,
            T![>] => SelectorCombinator::Child,
            T![+] => SelectorCombinator::NextSibling,
            T![~] => SelectorCombinator::SubsequentSibling,
            T![||] => SelectorCombinator::Column,
            _ => return,
        };
        if self.options.disallow_combinators().contains(&combinator) {
            self.violations
                .push(SelectorComplexityViolation::Disallowed {
                    part: DisallowedPart::Combinator,
                    range,
                    name: token,
                });
        }
    }

    fn visit_compound(&mut self, compound: &CssCompoundSelector) {
        self.parts.compounds += 1;

        match compound.simple_selector() {
            Some(AnyCssSimpleSelector::CssUniversalSelector(_)) => self.parts.universals += 1,
            Some(AnyCssSimpleSelector::CssTypeSelector(_)) => self.parts.types += 1,
            _ => {}
        }

        for sub_selector in compound.sub_selectors() {
            match sub_selector {
                AnyCssSubSelector::CssIdSelector(_) => self.parts.ids += 1,
                AnyCssSubSelector::CssAttributeSelector(attribute) => {
                    self.visit_attribute(&attribute);
                }
                AnyCssSubSelector::CssPseudoClassSelector(pseudo_class) => {
                    self.visit_pseudo_class(&pseudo_class);
                }
                AnyCssSubSelector::CssPseudoElementSelector(pseudo_element) => {
                    self.visit_pseudo_element(&pseudo_element);
                }
                _ => {}
            }
        }
    }

    fn visit_attribute(&mut self, attribute: &CssAttributeSelector) {
        self.parts.attributes += 1;

        if !self.options.disallow_attributes().is_empty()
            && let Ok(attribute_name) = attribute.name()
            && let Some(name) = attribute_name
                .name()
                .ok()
                .and_then(|name| name.as_css_identifier()?.value_token().ok())
            && self
                .options
                .disallow_attributes()
                .iter()
                .any(|entry| **entry == *decode_css_identifier(name.text_trimmed()))
        {
            self.violations
                .push(SelectorComplexityViolation::Disallowed {
                    part: DisallowedPart::Attribute,
                    range: attribute_name.range(),
                    name,
                });
        }

        let Some(operator) = attribute
            .matcher()
            .and_then(|matcher| matcher.operator().ok())
        else {
            return;
        };
        let matcher = match operator.kind() {
            T![=] => AttributeMatcherOperator::Exact,
            T![~=] => AttributeMatcherOperator::Includes,
            T![|=] => AttributeMatcherOperator::DashMatch,
            T![^=] => AttributeMatcherOperator::Prefix,
            CssSyntaxKind::DOLLAR_EQ => AttributeMatcherOperator::Suffix,
            T![*=] => AttributeMatcherOperator::Substring,
            _ => return,
        };
        if self
            .options
            .disallow_attribute_matchers()
            .contains(&matcher)
        {
            self.violations
                .push(SelectorComplexityViolation::Disallowed {
                    part: DisallowedPart::AttributeMatcher,
                    range: operator.text_trimmed_range(),
                    name: operator,
                });
        }
    }

    fn visit_pseudo_class(&mut self, pseudo_class: &CssPseudoClassSelector) {
        let name = pseudo_class
            .class()
            .ok()
            .and_then(|class| pseudo_class_name(&class));

        if let Some(name) = &name
            && is_legacy_pseudo_element(name)
        {
            if is_listed(self.options.disallow_pseudo_elements(), name) {
                self.violations
                    .push(SelectorComplexityViolation::Disallowed {
                        part: DisallowedPart::LegacyPseudoElement,
                        range: pseudo_class.range(),
                        name: name.clone(),
                    });
            }
            return;
        }

        self.parts.pseudo_classes += 1;

        if let Some(name) = name
            && is_listed(self.options.disallow_pseudo_classes(), &name)
        {
            self.violations
                .push(SelectorComplexityViolation::Disallowed {
                    part: DisallowedPart::PseudoClass,
                    range: pseudo_class.range(),
                    name,
                });
        }
    }

    fn visit_pseudo_element(&mut self, pseudo_element: &CssPseudoElementSelector) {
        if let Some(name) = pseudo_element
            .element()
            .ok()
            .and_then(|element| pseudo_element_name(&element))
            && is_listed(self.options.disallow_pseudo_elements(), &name)
        {
            self.violations
                .push(SelectorComplexityViolation::Disallowed {
                    part: DisallowedPart::PseudoElement,
                    range: pseudo_element.range(),
                    name,
                });
        }
    }

    fn check_limits(&mut self) {
        let SelectorParts {
            ids,
            types,
            attributes,
            pseudo_classes,
            universals,
            compounds,
            combinators,
        } = self.parts;

        for (part, count) in [
            (LimitedPart::Ids, ids),
            (LimitedPart::Types, types),
            (LimitedPart::Attributes, attributes),
            (LimitedPart::PseudoClasses, pseudo_classes),
            (LimitedPart::Universals, universals),
            (LimitedPart::Compounds, compounds),
            (LimitedPart::Combinators, combinators),
        ] {
            if part
                .limit(self.options)
                .is_some_and(|max| count > usize::from(max))
            {
                self.violations
                    .push(SelectorComplexityViolation::TooMany { part, count });
            }
        }
    }
}

/// Returns the range of the two compound selectors joined by a descendant combinator, such as
/// `nav a`, or `None` for other combinators.
///
/// The combinator token holds a single whitespace character, which is a line break when the
/// selector spans several lines. A range covering only whitespace isn't visible in a
/// diagnostic, and a range covering only a line break can't be printed at all.
fn descendant_combinator_range(
    selector: &CssComplexSelector,
    right: Option<&AnyCssSelector>,
) -> Option<TextRange> {
    if selector.combinator().ok()?.kind() != CssSyntaxKind::CSS_SPACE_LITERAL {
        return None;
    }
    let left = selector.left().ok()?;
    let left_compound = match &left {
        AnyCssSelector::CssComplexSelector(left) => left.right().ok()?,
        _ => left,
    };
    Some(TextRange::new(
        left_compound.range().start(),
        right?.range().end(),
    ))
}

/// Returns `true` if `name`, with CSS escapes decoded, case-insensitively equals an entry of
/// `list`.
fn is_listed(list: &[Box<str>], name: &CssSyntaxToken) -> bool {
    if list.is_empty() {
        return false;
    }
    let name = decode_css_identifier(name.text_trimmed());
    list.iter().any(|entry| entry.eq_ignore_ascii_case(&name))
}

fn is_legacy_pseudo_element(name: &CssSyntaxToken) -> bool {
    let name = decode_css_identifier(name.text_trimmed());
    LEGACY_PSEUDO_ELEMENTS
        .iter()
        .any(|legacy| legacy.eq_ignore_ascii_case(&name))
}

/// Returns the name token of a pseudo-class, or `None` when the name contains SCSS
/// interpolation or doesn't parse.
fn pseudo_class_name(pseudo_class: &AnyCssPseudoClass) -> Option<CssSyntaxToken> {
    let name = match pseudo_class {
        AnyCssPseudoClass::CssPseudoClassIdentifier(pseudo_class) => {
            return selector_identifier_token(pseudo_class.name().ok()?);
        }
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(pseudo_class) => {
            pseudo_class.name()
        }
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(pseudo_class) => {
            pseudo_class.name()
        }
        AnyCssPseudoClass::CssPseudoClassFunctionCustomIdentifier(pseudo_class) => {
            pseudo_class.name()
        }
        AnyCssPseudoClass::CssPseudoClassFunctionCustomIdentifierList(pseudo_class) => {
            pseudo_class.name()
        }
        AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(pseudo_class) => pseudo_class.name(),
        AnyCssPseudoClass::CssPseudoClassFunctionNth(pseudo_class) => pseudo_class.name(),
        AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(pseudo_class) => {
            pseudo_class.name()
        }
        AnyCssPseudoClass::CssPseudoClassFunctionSelector(pseudo_class) => pseudo_class.name(),
        AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(pseudo_class) => pseudo_class.name(),
        AnyCssPseudoClass::CssPseudoClassFunctionValueList(pseudo_class) => pseudo_class.name(),
        AnyCssPseudoClass::ScssInterpolatedPseudoClassFunction(_)
        | AnyCssPseudoClass::CssBogusPseudoClass(_) => return None,
    };
    name.ok()?.value_token().ok()
}

/// Returns the name token of a pseudo-element, or `None` when the name contains SCSS
/// interpolation or doesn't parse.
fn pseudo_element_name(pseudo_element: &AnyCssPseudoElement) -> Option<CssSyntaxToken> {
    let name = match pseudo_element {
        AnyCssPseudoElement::CssPseudoElementIdentifier(pseudo_element) => {
            return selector_identifier_token(pseudo_element.name().ok()?);
        }
        AnyCssPseudoElement::CssPseudoElementFunction(pseudo_element) => pseudo_element.name(),
        AnyCssPseudoElement::CssPseudoElementFunctionCustomIdentifier(pseudo_element) => {
            pseudo_element.name()
        }
        AnyCssPseudoElement::CssPseudoElementFunctionSelector(pseudo_element) => {
            pseudo_element.name()
        }
        AnyCssPseudoElement::ScssInterpolatedPseudoElementFunction(_)
        | AnyCssPseudoElement::CssBogusPseudoElement(_) => return None,
    };
    name.ok()?.value_token().ok()
}

fn selector_identifier_token(name: AnyCssSelectorIdentifier) -> Option<CssSyntaxToken> {
    name.as_css_identifier()?.value_token().ok()
}
