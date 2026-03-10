use crate::embed::types::{EmbedCandidate, GuestLanguage, TemplateTagKind};
use crate::workspace::DocumentFileSource;

/// A single embed detector. Entirely const-constructible.
///
/// Each variant corresponds to an `EmbedCandidate` variant. The
/// `try_match` method only succeeds when both the detector and candidate
/// are the same variant AND the detector's pattern matches.
pub(crate) enum EmbedDetector {
    /// Matches `EmbedCandidate::Element` by tag name.
    Element {
        tag: &'static str,
        target: EmbedTarget,
    },

    /// Matches `EmbedCandidate::Frontmatter`. Always matches (no pattern).
    Frontmatter { target: EmbedTarget },

    /// Matches `EmbedCandidate::TaggedTemplate` where the tag is an identifier.
    /// e.g. `css\`...\``, `gql\`...\``
    TemplateTag {
        tag: &'static str,
        target: EmbedTarget,
    },

    /// Matches `EmbedCandidate::TaggedTemplate` where the tag is a member
    /// expression or call expression with a matching object/callee.
    /// e.g. `styled.div\`...\``, `styled(Comp)\`...\``
    TemplateExpression {
        object: &'static str,
        target: EmbedTarget,
    },

    /// Matches `EmbedCandidate::TextExpression`. Always matches (no pattern).
    /// The guest language depends on the host framework.
    TextExpression { target: EmbedTarget },

    /// Matches `EmbedCandidate::Directive`. Always matches (no pattern).
    /// The guest language depends on the host framework.
    Directive { target: EmbedTarget },
}

impl EmbedDetector {
    /// Attempt to match this detector against a candidate.
    /// Returns `Some(guest)` if both the variant and pattern match.
    /// Returns `None` on variant mismatch or pattern mismatch.
    pub fn try_match(
        &self,
        candidate: &EmbedCandidate,
        file_source: &DocumentFileSource,
    ) -> Option<GuestLanguage> {
        match (self, candidate) {
            // Element detector VS an Element candidate: match by tag name
            (Self::Element { tag, target }, EmbedCandidate::Element { tag_name, .. }) => {
                if tag_name.text().eq_ignore_ascii_case(tag) {
                    target.resolve(candidate, file_source)
                } else {
                    None
                }
            }

            // Frontmatter detector + Frontmatter candidate: always matches
            (Self::Frontmatter { target }, EmbedCandidate::Frontmatter { .. }) => {
                target.resolve(candidate, file_source)
            }

            // TemplateTag detector + TaggedTemplate candidate with Identifier tag
            (
                Self::TemplateTag { tag, target },
                EmbedCandidate::TaggedTemplate {
                    tag: TemplateTagKind::Identifier(name),
                    ..
                },
            ) => {
                if name.text() == *tag {
                    target.resolve(candidate, file_source)
                } else {
                    None
                }
            }

            // TemplateExpression detector + TaggedTemplate candidate with
            // MemberExpression or CallExpression tag
            (
                Self::TemplateExpression { object, target },
                EmbedCandidate::TaggedTemplate { tag, .. },
            ) => match tag {
                TemplateTagKind::MemberExpression { object: obj, .. } => {
                    if obj.text() == *object {
                        target.resolve(candidate, file_source)
                    } else {
                        None
                    }
                }
                TemplateTagKind::CallExpression { callee } => {
                    if callee.text() == *object {
                        target.resolve(candidate, file_source)
                    } else {
                        None
                    }
                }
                _ => None,
            },

            // TextExpression detector + TextExpression candidate: always matches
            (Self::TextExpression { target }, EmbedCandidate::TextExpression { .. }) => {
                target.resolve(candidate, file_source)
            }

            // Directive detector + Directive candidate: always matches
            (Self::Directive { target }, EmbedCandidate::Directive { .. }) => {
                target.resolve(candidate, file_source)
            }

            // Mismatched variant — no match
            _ => None,
        }
    }
}

/// What guest language to embed.
pub(crate) enum EmbedTarget {
    /// Fixed guest language, always known when the embed is detected
    Static(GuestLanguage),

    /// Guest language depends on element attributes / host file source.
    Dynamic {
        /// Function that is used to determine the guest language.
        resolver: fn(&EmbedCandidate, &DocumentFileSource) -> Option<GuestLanguage>,
        /// Possible fallback. Use `None` to tell the matcher to ignore the snippet
        fallback: Option<GuestLanguage>,
    },
}

impl EmbedTarget {
    /// Resolve the guest language for this target.
    fn resolve(
        &self,
        candidate: &EmbedCandidate,
        file_source: &DocumentFileSource,
    ) -> Option<GuestLanguage> {
        match self {
            Self::Static(g) => Some(*g),
            Self::Dynamic { resolver, fallback } => resolver(candidate, file_source).or(*fallback),
        }
    }
}
