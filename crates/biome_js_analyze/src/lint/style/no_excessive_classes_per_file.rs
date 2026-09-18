use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsSyntaxKind};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_classes_per_file::NoExcessiveClassesPerFileOptions;

declare_lint_rule! {
    /// Enforce a maximum number of classes per file.
    ///
    /// Files containing multiple classes can often result in a less navigable and poorly structured codebase.
    /// Best practice is to keep each file limited to a single responsibility.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Foo {}
    /// class Bar {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class Foo {}
    /// ```
    ///
    /// ## Options
    ///
    /// The following options are available:
    ///
    /// ### `maxClasses`
    ///
    /// This option sets the maximum number of classes allowed in a file.
    /// If the file exceeds this limit, a diagnostic will be reported.
    ///
    /// Default: `1`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "maxClasses": 2
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// class Foo {}
    /// class Bar {}
    /// class Baz {}
    /// ```
    ///
    pub NoExcessiveClassesPerFile {
        version: "2.3.12",
        name: "noExcessiveClassesPerFile",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("max-classes-per-file").same()],
    }
}

impl Rule for NoExcessiveClassesPerFile {
    type Query = Ast<AnyJsRoot>;
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveClassesPerFileOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let max_classes = ctx.options().max_classes();

        let count = node
            .syntax()
            .descendants()
            .filter_map(|n| {
                if n.kind() == JsSyntaxKind::JS_CLASS_DECLARATION
                    || n.kind() == JsSyntaxKind::JS_CLASS_EXPRESSION
                {
                    Some(())
                } else {
                    None
                }
            })
            .count();

        if count > max_classes.get().into() {
            return Some(count);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let max_classes = ctx.options().max_classes();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "File exceeds the maximum of "{{max_classes.to_string()}}" class"{{if max_classes.get() > 1 { "es" } else { "" }}}", found "{{state}}" classes."
                },
            )
            .note(markup! {
                "Files containing multiple classes can often result in a less navigable and poorly structured codebase. Extract the excessive classes into a separate file."
            }),
        )
    }
}
