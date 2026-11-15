//! A plain-text rendering of a [`SvelteSemanticModel`], used by snapshot
//! tests.

use crate::semantic_model::*;
use biome_rowan::TextRange;
use std::fmt::{self, Display, Formatter, Write};

fn range(range: TextRange) -> String {
    format!("{}..{}", u32::from(range.start()), u32::from(range.end()))
}

fn facts(facts: &ValueFacts) -> String {
    let mut text = String::new();
    if let Some(value) = facts.value() {
        let _ = write!(text, " value={value:?}");
    }
    if let Some(strings) = facts.possible_strings() {
        let _ = write!(text, " strings={strings:?}");
    }
    if let Some(prefix) = facts.prefix() {
        let _ = write!(text, " prefix={prefix:?}");
    }
    if let Some(suffix) = facts.suffix() {
        let _ = write!(text, " suffix={suffix:?}");
    }
    text
}

impl Display for SvelteSemanticModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file {:?} runes={}",
            self.file_kind(),
            self.is_runes_mode()
        )?;
        if let Some(option) = self.runes_option() {
            write!(f, " runes-option={option}")?;
        }
        if let Some(route) = self.route_file() {
            write!(f, " route={route:?}")?;
        }
        writeln!(f)?;

        for script in self.scripts() {
            write!(
                f,
                "script {:?} {}",
                script.kind(),
                range(script.content_range())
            )?;
            if let Some(lang) = script.lang() {
                write!(f, " lang={lang}")?;
            }
            if script.has_generics() {
                f.write_str(" generics")?;
            }
            writeln!(f)?;
        }
        for style in self.styles() {
            write!(f, "style {}", range(style.content_range()))?;
            if let Some(lang) = style.lang() {
                write!(f, " lang={lang}")?;
            }
            writeln!(f)?;
        }

        for element in self.elements() {
            write!(
                f,
                "element {} {} {:?} {:?} scope={}",
                element.tag(),
                range(element.range()),
                element.kind(),
                element.occurrence(),
                element.scope().id().0
            )?;
            if let Some(reference) = element.tag_reference() {
                write!(f, " tag-ref={}", reference.id().0)?;
            }
            writeln!(f)?;
            for attr in element.attrs() {
                f.write_str("  attr")?;
                if let Some(kind) = attr.directive_kind() {
                    write!(
                        f,
                        " {}:{}",
                        kind.prefix(),
                        attr.directive_name().unwrap_or("")
                    )?;
                    for modifier in attr.modifiers() {
                        write!(f, "|{modifier}")?;
                    }
                } else if attr.is_spread() {
                    f.write_str(" spread")?;
                } else if attr.is_attachment() {
                    f.write_str(" attach")?;
                } else if attr.is_shorthand() {
                    write!(f, " {{{}}}", attr.name().unwrap_or(""))?;
                } else {
                    write!(f, " {}", attr.name().unwrap_or(""))?;
                }
                match attr.value() {
                    AttrValue::Absent => {}
                    AttrValue::Static(text) => write!(f, " = {text:?}")?,
                    AttrValue::Expr(snippet) => write!(
                        f,
                        " = snippet {}",
                        snippet.map_or("?".to_string(), |snippet| snippet.id().0.to_string())
                    )?,
                    AttrValue::Interpolated(parts) => {
                        f.write_str(" =")?;
                        for part in parts {
                            match part {
                                ValuePart::Text(text) => write!(f, " {text:?}")?,
                                ValuePart::Expr(snippet) => write!(
                                    f,
                                    " snippet {}",
                                    snippet.map_or("?".to_string(), |s| s.id().0.to_string())
                                )?,
                            }
                        }
                    }
                    AttrValue::FunctionBinding { get, set } => write!(
                        f,
                        " = get {} set {}",
                        get.map_or("?".to_string(), |s| s.id().0.to_string()),
                        set.map_or("?".to_string(), |s| s.id().0.to_string())
                    )?,
                }
                if attr.is_event_handler() {
                    f.write_str(" event-handler")?;
                }
                if let Some(reference) = attr.directive_reference() {
                    write!(f, " name-ref={}", reference.id().0)?;
                }
                if let Some(reference) = attr.implicit_reference() {
                    write!(f, " implicit-ref={}", reference.id().0)?;
                }
                writeln!(f)?;
            }
        }

        for block in self.blocks() {
            write!(f, "block {:?} {}", block.kind(), range(block.range()))?;
            if let Some(snippet) = block.expression() {
                write!(f, " expression={}", snippet.id().0)?;
            }
            if let Some(snippet) = block.key() {
                write!(f, " key={}", snippet.id().0)?;
            }
            writeln!(f)?;
            for branch in block.branches() {
                write!(
                    f,
                    "  branch {:?} {} scope={}",
                    branch.kind(),
                    range(branch.content_range()),
                    branch.scope().id().0
                )?;
                if let Some(condition) = branch.condition() {
                    write!(f, " condition={}", condition.id().0)?;
                }
                writeln!(f)?;
            }
        }

        for snippet in self.snippets() {
            write!(
                f,
                "snippet {} {:?} {} {:?}",
                snippet.id().0,
                snippet.host(),
                range(snippet.host_range()),
                snippet.root()
            )?;
            if let Some(scope) = snippet.scope() {
                write!(f, " scope={}", scope.id().0)?;
            }
            if snippet.is_in_snippet_block() {
                f.write_str(" in-snippet-block")?;
            }
            f.write_str(&facts(snippet.facts()))?;
            writeln!(f)?;
        }

        let scopes: Vec<_> = self
            .symbols()
            .map(|symbol| symbol.scope())
            .chain(self.module_scope())
            .chain(self.instance_scope())
            .chain(self.template_scope())
            .collect();
        let mut seen = Vec::new();
        let mut sorted = scopes;
        sorted.sort_by_key(|scope| scope.id());
        for scope in sorted {
            if seen.contains(&scope.id()) {
                continue;
            }
            seen.push(scope.id());
            write!(f, "scope {} {:?}", scope.id().0, scope.kind())?;
            if let Some(parent) = scope.parent() {
                write!(f, " parent={}", parent.id().0)?;
            }
            writeln!(f)?;
            for symbol in scope.symbols() {
                write!(
                    f,
                    "  symbol {} {:?} {} {} {:?}",
                    symbol.id().0,
                    symbol.kind(),
                    symbol.name(),
                    range(symbol.host_range()),
                    symbol.declaration_kind()
                )?;
                if let (Some(source), Some(imported)) = (symbol.import_source(), symbol.imported())
                {
                    write!(f, " import={source}:{imported:?}")?;
                }
                if let Some(rune) = symbol.rune() {
                    write!(f, " rune={}", rune.name())?;
                }
                if let Some(store) = symbol.store_creator() {
                    write!(f, " store={store:?}")?;
                }
                if let Some(source) = symbol.each_source() {
                    write!(f, " each-source={}", source.name())?;
                }
                if symbol.is_exported() {
                    f.write_str(" exported")?;
                }
                if symbol.is_type_only() {
                    f.write_str(" type-only")?;
                }
                if symbol.is_reassigned() {
                    f.write_str(" reassigned")?;
                }
                f.write_str(&facts(symbol.facts()))?;
                writeln!(f)?;
            }
        }

        for call in self.rune_calls() {
            write!(
                f,
                "rune {} {} target={:?} args={}",
                call.rune().name(),
                range(call.host_range()),
                call.target(),
                call.args()
            )?;
            let symbols: Vec<_> = call
                .symbols()
                .map(|symbol| symbol.name().to_string())
                .collect();
            if !symbols.is_empty() {
                write!(f, " symbols={}", symbols.join(","))?;
            }
            writeln!(f)?;
        }

        if let Some(props) = self.props_declaration() {
            write!(f, "props")?;
            if let Some(object) = props.object() {
                write!(f, " object={}", object.name())?;
            }
            if let Some(rest) = props.rest() {
                write!(f, " rest={}", rest.name())?;
            }
            if props.has_type() {
                f.write_str(" typed")?;
            }
            writeln!(f, " complete={}", props.is_complete())?;
        }
        for prop in self.props() {
            write!(f, "prop {} {}", prop.name(), range(prop.host_range()))?;
            if let Some(local) = prop.local() {
                write!(f, " local={}", local.name())?;
            }
            if prop.default_range().is_some() {
                f.write_str(" default")?;
            }
            if prop.is_bindable() {
                f.write_str(" bindable")?;
            }
            if prop.is_in_type() {
                f.write_str(" in-type")?;
            }
            writeln!(f, " uses={}", prop.uses().count())?;
        }

        for export in self.exports() {
            write!(f, "export {} {}", export.name(), range(export.host_range()))?;
            if let Some(local) = export.local() {
                write!(f, " local={}", local.name())?;
            }
            if export.is_reexport() {
                f.write_str(" reexport")?;
            }
            if export.is_type_only() {
                f.write_str(" type-only")?;
            }
            writeln!(f)?;
        }

        for reference in self.references() {
            write!(
                f,
                "ref {} {} {} {:?} {:?}",
                reference.id().0,
                reference.name(),
                range(reference.host_range()),
                reference.site(),
                reference.access()
            )?;
            let path: Vec<_> = reference.path().collect();
            if !path.is_empty() {
                write!(f, " path={}", path.join("."))?;
            }
            if reference.has_open_path() {
                f.write_str(" path-open")?;
            }
            match reference.resolution() {
                Resolution::Symbol(_) => {
                    let symbol = reference.symbol().expect("a symbol");
                    write!(f, " -> {} {}", symbol.id().0, symbol.name())?;
                }
                resolution => write!(f, " -> {resolution:?}")?,
            }
            if let Some(via) = reference.via() {
                write!(f, " via={}", via.name())?;
            }
            if reference.is_store_subscription() {
                f.write_str(" store")?;
            }
            if reference.is_in_function() {
                f.write_str(" in-function")?;
            }
            if reference.is_in_snippet_block() {
                f.write_str(" in-snippet-block")?;
            }
            if reference.is_type_only() {
                f.write_str(" type")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
