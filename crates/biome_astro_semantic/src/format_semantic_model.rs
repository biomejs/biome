//! A plain-text rendering of an [`AstroSemanticModel`] for snapshot tests.

use crate::semantic_model::*;
use biome_languages::JsFileSource;
use biome_rowan::TextRange;
use std::fmt::{self, Display, Formatter};

fn range(range: TextRange) -> String {
    format!("{}..{}", u32::from(range.start()), u32::from(range.end()))
}

fn open_reason_names(reasons: OpenReasons) -> String {
    reasons
        .iter()
        .map(|reason| format!("{reason:?}"))
        .collect::<Vec<_>>()
        .join("|")
}

fn source_name(source: JsFileSource) -> &'static str {
    let embedding = source.as_embedding_kind();
    if embedding.is_astro_frontmatter() {
        "astro-frontmatter"
    } else if embedding.is_astro_template() {
        "astro-template"
    } else if source.is_typescript() && source.is_jsx() {
        "tsx"
    } else if source.is_typescript() {
        "typescript"
    } else if source.is_jsx() {
        "jsx"
    } else {
        "javascript"
    }
}

impl Display for AstroSemanticModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for block in self.blocks() {
            write!(f, "block {:?} {}", block.kind(), range(block.host_range()))?;
            if let Some(content) = block.content_range() {
                write!(f, " content={}", range(content))?;
            }
            if block.is_globally_scoped() {
                write!(f, " global")?;
            }
            if block.has_external_source() {
                write!(f, " external")?;
            }
            if let Some(scope) = block.scope() {
                write!(f, " scope#{}", scope.id().0)?;
            }
            if let Some(snippet) = block.snippet() {
                write!(f, " snippet#{}", snippet.id().0)?;
            }
            let define_vars_open = block.define_vars_open_reasons();
            if !define_vars_open.is_empty() {
                write!(
                    f,
                    " define-vars-open={}",
                    open_reason_names(define_vars_open)
                )?;
            }
            writeln!(f)?;
            for variable in block.define_vars() {
                writeln!(
                    f,
                    "  define:vars {} {}",
                    variable.name(),
                    range(variable.host_range())
                )?;
            }
        }

        for snippet in self.snippets() {
            write!(
                f,
                "snippet#{} {} {:?} offset={} source={}",
                snippet.id().0,
                range(snippet.host_range()),
                snippet.host(),
                u32::from(snippet.content_offset()),
                source_name(snippet.source())
            )?;
            if let Some(scope) = snippet.scope() {
                write!(f, " scope#{}", scope.id().0)?;
            }
            writeln!(f)?;
        }

        for scope in self.scopes() {
            write!(f, "scope#{} {:?}", scope.id().0, scope.kind())?;
            if let Some(parent) = scope.parent() {
                write!(f, " parent=scope#{}", parent.id().0)?;
            }
            for namespace in [Namespace::Value, Namespace::Type, Namespace::Namespace] {
                let reasons = scope.open_reasons(namespace);
                if !reasons.is_empty() {
                    write!(f, " open-{namespace:?}={}", open_reason_names(reasons))?;
                }
            }
            writeln!(f)?;
            for symbol in scope.symbols() {
                write!(
                    f,
                    "  {:?} {} {}",
                    symbol.kind(),
                    symbol.name(),
                    range(symbol.host_range())
                )?;
                if symbol.origin() != Origin::None {
                    write!(f, " origin={:?}", symbol.origin())?;
                }
                if symbol.declares_type() && symbol.declares_value() {
                    write!(f, " type+value")?;
                } else if symbol.is_type_only() {
                    write!(f, " type-only")?;
                }
                if symbol.declares_namespace() {
                    write!(f, " namespace")?;
                }
                writeln!(f)?;
            }
        }

        for reference in self.references() {
            write!(
                f,
                "ref {} {} {:?} {:?} {:?}",
                reference.name(),
                range(reference.host_range()),
                reference.namespace(),
                reference.site(),
                reference.access()
            )?;
            match reference.resolution() {
                Resolution::Symbol(symbol) => {
                    let symbol = self.symbol(symbol);
                    writeln!(f, " -> {:?} {}", symbol.kind(), symbol.name())?;
                }
                resolution => writeln!(f, " -> {resolution:?}")?,
            }
        }

        Ok(())
    }
}
