use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression,
    AnyJsMemberExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectBindingPatternMember,
    JsAssignmentExpression, JsConditionalExpression, JsFormalParameter, JsLogicalExpression, JsIdentifierExpression, JsInitializerClause,
    JsVariableDeclarator, T,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_astro_deprecated_canonical_url::NoAstroDeprecatedCanonicalUrlOptions;

declare_lint_rule! {
    /// Disallow the deprecated `Astro.canonicalURL` property.
    ///
    /// Astro 1.0 deprecated `Astro.canonicalURL`, and Astro 2.0 removed it, so code that reads it causes an error.
    /// Use `Astro.url` to build the canonical URL of the current page instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// ---
    /// const canonicalURL = Astro.canonicalURL;
    /// ---
    /// <link rel="canonical" href={canonicalURL} />
    /// ```
    ///
    /// ```astro,expect_diagnostic
    /// ---
    /// const { canonicalURL } = Astro;
    /// ---
    /// <link rel="canonical" href={canonicalURL} />
    /// ```
    ///
    /// ```astro,expect_diagnostic
    /// <link rel="canonical" href={Astro.canonicalURL} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// const canonicalURL = new URL(Astro.url.pathname, Astro.site);
    /// ---
    /// <link rel="canonical" href={canonicalURL} />
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro 2.0 upgrade guide: `Astro.canonicalURL` removed](https://docs.astro.build/en/guides/upgrade-to/v2/#removed-astrocanonicalurl)
    pub NoAstroDeprecatedCanonicalUrl {
        version: "next",
        name: "noAstroDeprecatedCanonicalUrl",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("no-deprecated-astro-canonicalurl").same()],
    }
}

impl Rule for NoAstroDeprecatedCanonicalUrl {
    type Query = Semantic<JsIdentifierExpression>;
    /// The range of the member access or destructured property that reads `canonicalURL`.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoAstroDeprecatedCanonicalUrlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.as_embedding_kind().is_astro() {
            return Box::default();
        }

        let Ok(reference) = ctx.query().name() else {
            return Box::default();
        };
        let Ok(token) = reference.value_token() else {
            return Box::default();
        };
        if token.text_trimmed() != "Astro" || !ctx.model().is_unresolved_reference(&reference) {
            return Box::default();
        }

        // Template expressions are analyzed separately from the frontmatter,
        // so a frontmatter variable named `Astro` shadows the global without
        // being visible to the template's semantic model.
        if !source_type.is_embedded_source()
            && ctx
                .get_service::<EmbeddedService>()
                .is_some_and(|embedded| embedded.contains_binding(token.token_text_trimmed()))
        {
            return Box::default();
        }

        let mut tracker = ReadTracker {
            model: ctx.model(),
            visited_bindings: Vec::new(),
            reads: Vec::new(),
        };
        tracker.track_expression(&AnyJsExpression::from(ctx.query().clone()));
        tracker.reads.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>"Astro.canonicalURL"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                "Astro 2.0 removed "<Emphasis>"Astro.canonicalURL"</Emphasis>", so reading it causes an error."
            })
            .note(markup! {
                "Build the URL from "<Emphasis>"Astro.url"</Emphasis>" instead, for example "<Emphasis>"new URL(Astro.url.pathname, Astro.site)"</Emphasis>"."
            }),
        )
    }
}

const CANONICAL_URL: &str = "canonicalURL";

/// Collects the reads of `canonicalURL` from an expression that evaluates to
/// the `Astro` global, following the variables the global is stored in.
///
/// Variables are only followed through their initializer, such as
/// `const page = Astro`, and not through later assignments, such as
/// `page = Astro`. A variable has a single initializer, so at most one `Astro`
/// reference reaches it, and each read is reported once.
struct ReadTracker<'a> {
    model: &'a SemanticModel,
    /// The ranges of the variables already followed, so that a variable whose
    /// initializer reads the variable itself isn't followed forever.
    visited_bindings: Vec<TextRange>,
    reads: Vec<TextRange>,
}

impl ReadTracker<'_> {
    /// Records how the value of `expression` is used: as the object of a
    /// member access, or as the value assigned to a pattern or a variable.
    fn track_expression(&mut self, expression: &AnyJsExpression) {
        let Some(mut expression) = expression.outer_expression() else {
            return;
        };
        let parent = loop {
            let Some(parent) = expression.syntax().parent() else {
                return;
            };
            // The value of `cond ? Astro : other` and `Astro || other` can be
            // `Astro` itself.
            let is_pass_through = match JsConditionalExpression::cast_ref(&parent) {
                Some(conditional) => conditional
                    .test()
                    .is_ok_and(|test| test.syntax() != expression.syntax()),
                None => JsLogicalExpression::can_cast(parent.kind()),
            };
            if !is_pass_through {
                break parent;
            }
            let Some(outer) = AnyJsExpression::cast(parent).and_then(|outer| outer.outer_expression())
            else {
                return;
            };
            expression = outer;
        };

        if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
            if member
                .object()
                .is_ok_and(|object| object.syntax() == expression.syntax())
                && member
                    .member_name()
                    .is_some_and(|name| name.text() == CANONICAL_URL)
            {
                self.reads.push(member.range());
            }
        } else if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
            if assignment
                .operator_token()
                .is_ok_and(|operator| operator.kind() == T![=])
                && assignment
                    .right()
                    .is_ok_and(|right| right.syntax() == expression.syntax())
                && let Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(pattern)) =
                    assignment.left()
            {
                self.reads.extend(
                    pattern
                        .properties()
                        .iter()
                        .flatten()
                        .filter(is_canonical_url_assignment_property)
                        .map(|property| property.range()),
                );
            }
        } else if let Some(owner) = JsInitializerClause::cast(parent).and_then(|initializer| {
            initializer
                .syntax()
                .parent()
                .and_then(AnyInitializerOwner::cast)
        }) {
            let pattern = match owner {
                AnyInitializerOwner::JsVariableDeclarator(declarator) => declarator.id(),
                AnyInitializerOwner::JsFormalParameter(parameter) => parameter.binding(),
            };
            if let Ok(pattern) = pattern {
                self.track_binding_pattern(pattern);
            }
        }
    }

    fn track_binding_pattern(&mut self, pattern: AnyJsBindingPattern) {
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                self.track_binding(&self.model.as_binding(&binding));
            }
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                self.reads.extend(
                    pattern
                        .properties()
                        .iter()
                        .flatten()
                        .filter(is_canonical_url_binding_property)
                        .map(|property| property.range()),
                );
            }
            _ => {}
        }
    }

    /// Follows every read of a variable that holds the `Astro` global.
    fn track_binding(&mut self, binding: &Binding) {
        let range = binding.syntax().text_trimmed_range();
        if self.visited_bindings.contains(&range) {
            return;
        }
        self.visited_bindings.push(range);

        for reference in binding.all_reads() {
            if let Some(expression) = reference
                .syntax()
                .parent()
                .and_then(JsIdentifierExpression::cast)
            {
                self.track_expression(&expression.into());
            }
        }
    }
}

declare_node_union! {
    /// The nodes whose initializer assigns a value to a binding pattern.
    pub AnyInitializerOwner = JsVariableDeclarator | JsFormalParameter
}

fn is_canonical_url_binding_property(property: &AnyJsObjectBindingPatternMember) -> bool {
    match property {
        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => property
            .member()
            .ok()
            .and_then(|member| member.name())
            .is_some_and(|name| name.text() == CANONICAL_URL),
        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
            property
                .identifier()
                .ok()
                .and_then(|identifier| identifier.as_js_identifier_binding()?.name_token().ok())
                .is_some_and(|name| name.text_trimmed() == CANONICAL_URL)
        }
        _ => false,
    }
}

fn is_canonical_url_assignment_property(property: &AnyJsObjectAssignmentPatternMember) -> bool {
    match property {
        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
            property
                .member()
                .ok()
                .and_then(|member| member.name())
                .is_some_and(|name| name.text() == CANONICAL_URL)
        }
        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
            property,
        ) => property
            .identifier()
            .ok()
            .and_then(|identifier| identifier.name_token().ok())
            .is_some_and(|name| name.text_trimmed() == CANONICAL_URL),
        _ => false,
    }
}
