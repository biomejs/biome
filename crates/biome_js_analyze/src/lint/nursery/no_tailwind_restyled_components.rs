use crate::tailwind::{AnyTailwindClassString, host_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsxElementName, AnyJsxObjectName, JsxAttribute};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_tailwind_restyled_components::NoTailwindRestyledComponentsOptions;
use biome_tailwind_logic::no_tailwind_restyled_components::{
    matches_component_name, restyled_component_ranges,
};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use smallvec::{SmallVec, smallvec};

declare_lint_rule! {
    /// Disallow Tailwind utilities that override the appearance of components.
    ///
    /// A design system should own its components' appearance. Use component props
    /// for supported visual variants instead of overriding them with utility classes.
    ///
    /// The rule reports utilities in these categories:
    ///
    /// - `color`, such as `bg-red-500` and `text-white`
    /// - `typography`, such as `text-sm` and `font-bold`
    /// - `spacing`, such as `p-4` and `gap-2`
    /// - `shape`, such as `rounded-none` and `border-2`
    /// - `effects`, such as `shadow` and `opacity-50`
    /// - `motion`, such as `transition` and `animate-spin`
    ///
    /// Variants, important modifiers, and arbitrary values don't change the category, so
    /// `hover:bg-red-500`, `rounded-none!`, and `p-[3px]` are reported too. Arbitrary
    /// properties that set these styles, such as `[font-size:14px]`, are also reported.
    /// Other utilities, such as sizing, positioning, and margins, are ignored.
    ///
    /// Components are elements with capitalized names such as `Button`, member names
    /// such as `UI.Button`, and custom elements with hyphenated names such as `my-button`.
    /// Native elements are not checked. The rule checks `class` and `className`,
    /// including literal branches of conditionals and common class helper calls.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Button className="rounded-none" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Button variant="danger" className="mt-4 w-full" />;
    /// <button className="rounded-none" />;
    /// ```
    ///
    /// ## Options
    ///
    /// ### allow
    ///
    /// Default: `[]`.
    ///
    /// Allows categories or classes on selected components. Each entry contains:
    ///
    /// - `components`: a component name, an array of names, or `"*"` for all components.
    /// - `categories`: any of `color`, `typography`, `spacing`, `shape`, `effects`, or `motion`. Default: `[]`.
    /// - `classes`: exact classes, including variants and modifiers. Default: `[]`.
    ///
    /// A name matches any segment of a member name, so both `UI` and `Button` match
    /// `UI.Button`. Dotted names such as `UI.Button` match consecutive segments.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allow": [
    ///       { "components": "Button", "categories": ["shape"], "classes": ["hover:shadow-lg"] }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// <Button className="rounded-none hover:shadow-lg" />;
    /// ```
    ///
    pub NoTailwindRestyledComponents {
        version: "next",
        name: "noTailwindRestyledComponents",
        language: "jsx",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        issue_number: Some("11342"),
        sources: &[RuleSource::EslintShadcn("no-restyle").inspired()],
    }
}

impl Rule for NoTailwindRestyledComponents {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoTailwindRestyledComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return vec![];
        }
        let attribute = ctx
            .query()
            .node()
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsxAttribute::cast);
        let element = attribute.and_then(|attribute| {
            if !matches!(
                attribute.name_value_token().ok()?.text_trimmed(),
                "class" | "className"
            ) {
                return None;
            }
            attribute
                .syntax()
                .parent()?
                .parent()
                .and_then(AnyJsxElement::cast)
        });
        let Some(element) = element else {
            return vec![];
        };
        if !element.is_custom_component() && !element.is_custom_element() {
            return vec![];
        }
        let segments = element.name().ok().and_then(|name| name_segments(&name));
        let allowances: Vec<_> = ctx
            .options()
            .allow()
            .iter()
            .filter(|allow| {
                allow.components.matches(|name| match &segments {
                    Some(segments) => matches_component_name(segments, name),
                    // Namespaced names such as `svg:rect` are not split into segments.
                    None => element.matches_name(name).unwrap_or(false),
                })
            })
            .collect();
        restyled_component_ranges(&ctx.query().tailwind_root().candidates(), &allowances)
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), *range)?,
                markup! { "This Tailwind utility restyles a component." },
            )
            .note(markup! { "The design system should manage the component's appearance." })
            .note(markup! { "Use a supported component variant or move this style into the component's definition." }),
        )
    }
}

/// Returns the segments of `name`, such as `UI` and `Button` for `UI.Button`.
/// Returns `None` for namespaced names.
fn name_segments(name: &AnyJsxElementName) -> Option<SmallVec<[TokenText; 2]>> {
    let mut object = match name {
        AnyJsxElementName::JsxName(name) => {
            return Some(smallvec![name.value_token().ok()?.token_text_trimmed()]);
        }
        AnyJsxElementName::JsxReferenceIdentifier(name) => {
            AnyJsxObjectName::JsxReferenceIdentifier(name.clone())
        }
        AnyJsxElementName::JsxMemberName(name) => AnyJsxObjectName::JsxMemberName(name.clone()),
        AnyJsxElementName::JsxNamespaceName(_) | AnyJsxElementName::JsMetavariable(_) => {
            return None;
        }
    };
    let mut segments = SmallVec::new();
    loop {
        match object {
            AnyJsxObjectName::JsxMemberName(member) => {
                segments.push(member.member().ok()?.value_token().ok()?.token_text_trimmed());
                object = member.object().ok()?;
            }
            AnyJsxObjectName::JsxReferenceIdentifier(name) => {
                segments.push(name.value_token().ok()?.token_text_trimmed());
                segments.reverse();
                return Some(segments);
            }
            AnyJsxObjectName::JsxNamespaceName(_) => return None,
        }
    }
}
