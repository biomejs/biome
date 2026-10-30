use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, Direction, TextSize, declare_node_union};
use biome_rule_options::no_empty_source::NoEmptySourceOptions;
use biome_yaml_syntax::{AnyYamlBlockNode, AnyYamlFlowNode, YamlDocument, YamlRoot};

declare_lint_rule! {
    /// Disallow empty sources.
    ///
    /// A YAML file can hold several documents, separated by `---` lines.
    /// This rule reports each document that has no content, and files that contain no documents at all.
    /// A document that only has a tag (such as `!!map`) or an anchor (such as `&config`) also counts as empty,
    /// because it still holds no value.
    ///
    /// Empty documents and files are usually leftovers from editing, and tools that read the file
    /// get an empty value (`null`) for them.
    ///
    /// By default, whitespace and comments do not count as content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,expect_diagnostic
    ///
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// # Only comments
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// name: foo
    /// ---
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// --- !!map
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// name: foo
    /// ---
    /// name: bar
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowComments`
    ///
    /// Default: `false`
    ///
    /// Treats comments as meaningful content when set to `true`, so a file or document that only
    /// contains comments is valid.
    /// An entirely empty file or document remains invalid.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowComments": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```yaml,expect_diagnostic,use_options
    /// name: foo
    /// ---
    /// ```
    ///
    /// #### Valid
    ///
    /// ```yaml,use_options
    /// name: foo
    /// ---
    /// # Only comments
    /// ```
    ///
    pub NoEmptySource {
        version: "2.6.0",
        name: "noEmptySource",
        language: "yaml",
        sources: &[RuleSource::EslintYml("no-empty-document").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyNoEmptySourceQuery = YamlRoot | YamlDocument
}

impl Rule for NoEmptySource {
    type Query = Ast<AnyNoEmptySourceQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptySourceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let allow_comments = ctx.options().allow_comments();
        match ctx.query() {
            AnyNoEmptySourceQuery::YamlRoot(root) => {
                if !root.documents().is_empty() {
                    return None;
                }
                if allow_comments && root.eof_token().ok()?.has_leading_comments() {
                    return None;
                }
            }
            AnyNoEmptySourceQuery::YamlDocument(document) => {
                if !is_empty_document(document) {
                    return None;
                }
                if allow_comments && has_content_comments(document) {
                    return None;
                }
            }
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = match node {
            AnyNoEmptySourceQuery::YamlRoot(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "An empty source is not allowed."
                },
            )
            .note(markup! {
                "Empty sources can clutter the codebase and increase cognitive load; deleting empty sources can help reduce it."
            }),
            AnyNoEmptySourceQuery::YamlDocument(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "An empty document is not allowed."
                },
            )
            .note(markup! {
                "Tools that read this file get an empty value for this document, which is rarely intended."
            })
            .note(markup! {
                "Add content to the document, or remove it."
            }),
        };
        Some(diagnostic)
    }
}

/// Returns `true` when the document holds no value: it has no node at all, or
/// only a tag or anchor without a value after it.
fn is_empty_document(document: &YamlDocument) -> bool {
    match document.node() {
        None => true,
        Some(AnyYamlBlockNode::YamlFlowInBlockNode(node)) => matches!(
            node.flow(),
            Ok(AnyYamlFlowNode::YamlFlowYamlNode(flow)) if flow.content().is_none()
        ),
        Some(_) => false,
    }
}

/// Returns `true` when a comment appears in the body of the document, after
/// its `---` marker.
///
/// Comments are attached to the token that follows them, so a comment at the
/// end of a document without a `...` marker belongs to the first token of the
/// next document, or to the end of the file. That following token is checked
/// as well.
fn has_content_comments(document: &YamlDocument) -> bool {
    // Comments before `---` precede the document rather than belong to its content.
    let content_start = document
        .dashdashdash_token()
        .map_or(TextSize::from(0), |token| token.text_trimmed_range().end());
    let following_token = if document.dotdotdot_token().is_none() {
        document
            .syntax()
            .last_token()
            .and_then(|token| token.next_token())
    } else {
        None
    };
    document
        .syntax()
        .descendants_tokens(Direction::Next)
        .chain(following_token)
        .any(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
                .any(|piece| piece.is_comments() && piece.text_range().start() >= content_start)
        })
}
