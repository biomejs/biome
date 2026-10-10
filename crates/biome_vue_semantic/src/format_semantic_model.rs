//! A plain-text rendering of a [`SemanticModel`], used by snapshot tests.

use crate::semantic_model::*;
use biome_rowan::TextRange;
use std::fmt::{self, Display, Formatter, Write};

fn range(range: TextRange) -> String {
    format!("{}..{}", u32::from(range.start()), u32::from(range.end()))
}

fn open_reasons(reasons: OpenReasons) -> String {
    let mut names = Vec::new();
    for (flag, name) in [
        (OpenReasons::SPREAD, "spread"),
        (OpenReasons::EXTENDS, "extends"),
        (OpenReasons::UNRESOLVED_TYPE, "unresolved-type"),
        (OpenReasons::NON_LITERAL, "non-literal"),
        (OpenReasons::EXTERNAL_SRC, "external-src"),
    ] {
        if reasons.contains(flag) {
            names.push(name);
        }
    }
    names.join(",")
}

fn type_set(types: TypeSet) -> String {
    let mut names = Vec::new();
    for (ty, name) in [
        (PropType::Boolean, "Boolean"),
        (PropType::String, "String"),
        (PropType::Number, "Number"),
        (PropType::Array, "Array"),
        (PropType::Object, "Object"),
        (PropType::Function, "Function"),
        (PropType::Symbol, "Symbol"),
        (PropType::Date, "Date"),
        (PropType::BigInt, "BigInt"),
        (PropType::Unknown, "Unknown"),
    ] {
        if types.contains(ty) {
            names.push(name);
        }
    }
    names.join("|")
}

const NAMESPACES: [Namespace; Namespace::COUNT] = [
    Namespace::Binding,
    Namespace::Emit,
    Namespace::Slot,
    Namespace::Component,
    Namespace::Directive,
    Namespace::TemplateRef,
    Namespace::Option,
];

fn write_symbol(f: &mut Formatter<'_>, indent: &str, symbol: &Symbol) -> fmt::Result {
    write!(
        f,
        "{indent}{:?} {} {}",
        symbol.kind(),
        symbol.name(),
        range(symbol.host_range())
    )?;
    if symbol.origin() != Origin::None {
        write!(f, " origin={:?}", symbol.origin())?;
    }
    if symbol.is_type_only() {
        f.write_str(" type-only")?;
    }
    if let Some(parent) = symbol.parent() {
        write!(f, " in={}", parent.name())?;
    }
    match symbol.kind() {
        SymbolKind::Prop => {
            let prop = Prop(symbol.clone());
            if let Some(form) = prop.form() {
                write!(f, " form={form:?}")?;
            }
            if !prop.types().is_empty() {
                write!(f, " types={}", type_set(prop.types()))?;
            }
            write!(f, " required={:?}", prop.required())?;
            for default in prop.defaults() {
                write!(
                    f,
                    " default={:?}@{}",
                    default.source(),
                    range(default.host_range())
                )?;
            }
            if prop.validator_range().is_some() {
                f.write_str(" validator")?;
            }
        }
        SymbolKind::Emit => {
            let emit = Emit(symbol.clone());
            if let Some(form) = emit.form() {
                write!(f, " form={form:?}")?;
            }
            if let Some(params) = emit.payload_params() {
                write!(f, " payload={params}")?;
            }
            if emit.validator_range().is_some() {
                f.write_str(" validator")?;
            }
        }
        SymbolKind::Method => {
            let method = Method(symbol.clone());
            if let Some(params) = method.params() {
                write!(f, " params={params}")?;
            }
            if method.has_rest_param() {
                f.write_str(" rest")?;
            }
        }
        _ => {}
    }
    let computed = Computed(symbol.clone());
    if computed.getter_range().is_some() {
        f.write_str(" getter")?;
    }
    if computed.has_setter() {
        f.write_str(" setter")?;
    }
    let uses = symbol.uses().count();
    if uses > 0 {
        write!(f, " uses={uses}")?;
    }
    if !symbol.has_complete_uses() {
        f.write_str(" uses-open")?;
    }
    f.write_char('\n')
}

impl Display for SemanticModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for block in self.blocks() {
            write!(
                f,
                "block {:?} <{}> {}",
                block.kind(),
                block.element().tag(),
                range(block.element().range())
            )?;
            if let Some(lang) = block.lang() {
                write!(f, " lang={lang}")?;
            }
            for (flag, name) in [
                (block.is_setup(), "setup"),
                (block.is_scoped(), "scoped"),
                (block.is_module(), "module"),
                (block.has_src(), "src"),
                (block.is_empty(), "empty"),
            ] {
                if flag {
                    write!(f, " {name}")?;
                }
            }
            f.write_char('\n')?;
        }

        for component in self.components() {
            write!(
                f,
                "component {:?} {}",
                component.kind(),
                range(component.host_range())
            )?;
            if component.has_template() {
                f.write_str(" template")?;
            }
            if let Some(name) = component.name() {
                write!(f, " name={name}")?;
            }
            if component.inherit_attrs() != Tri::Unknown {
                write!(f, " inheritAttrs={:?}", component.inherit_attrs())?;
            }
            if component.has_macro_conflict() {
                f.write_str(" macro-conflict")?;
            }
            f.write_char('\n')?;
            for namespace in NAMESPACES {
                let reasons = component.open_reasons(namespace);
                if !reasons.is_empty() {
                    writeln!(f, "  open {namespace:?}: {}", open_reasons(reasons))?;
                }
            }
            for symbol in component.declarations() {
                write_symbol(f, "  ", &symbol)?;
            }
        }

        for element in self.elements() {
            let depth = std::iter::successors(element.parent(), Element::parent).count();
            let indent = "  ".repeat(depth);
            write!(f, "{indent}<{}> {}", element.tag(), range(element.range()))?;
            if element.is_component() {
                f.write_str(" component")?;
            }
            if let Some((chain, index)) = element.chain() {
                write!(f, " chain={}#{index}", chain.id().0)?;
            }
            f.write_char('\n')?;
            for attr in element.attrs() {
                write!(f, "{indent}  @")?;
                match attr.directive_name() {
                    Some(name) => {
                        f.write_str(name)?;
                        match attr.directive_arg() {
                            Some(DirectiveArg::Static(arg)) => write!(f, ":{}", arg.text())?,
                            Some(DirectiveArg::Dynamic(arg)) => write!(f, ":[{}]", arg.text())?,
                            _ => {}
                        }
                        for modifier in attr.modifiers() {
                            write!(f, ".{modifier}")?;
                        }
                        if attr.is_shorthand() {
                            f.write_str(" shorthand")?;
                        }
                    }
                    None => f.write_str(attr.plain_name().unwrap_or_default())?,
                }
                match attr.value() {
                    AttrValue::Absent => {}
                    AttrValue::Static(text) => write!(f, " = {text:?}")?,
                    AttrValue::Expr(snippet) => write!(f, " = snippet#{}", snippet.id().0)?,
                    AttrValue::Unparsed => f.write_str(" = <unparsed>")?,
                }
                f.write_char('\n')?;
            }
            for class in element.classes() {
                writeln!(
                    f,
                    "{indent}  .{} {:?}{}",
                    class.name(),
                    class.certainty(),
                    if class.is_from_expression() {
                        " expr"
                    } else {
                        ""
                    }
                )?;
            }
            if element.has_open_classes() {
                writeln!(f, "{indent}  .<open>")?;
            }
            for symbol in element.scope().symbols() {
                write_symbol(f, &format!("{indent}  let "), &symbol)?;
            }
        }

        for snippet in self.snippets() {
            write!(
                f,
                "snippet#{} {} {:?} root={:?}",
                snippet.id().0,
                range(snippet.host_range()),
                snippet.host(),
                snippet.root()
            )?;
            if !snippet.operands().is_empty() {
                write!(f, " operands={}", snippet.operands().len())?;
            }
            f.write_char('\n')?;
        }

        for scope in (0..self.data.scopes.len()).map(|index| Scope {
            data: self.data.clone(),
            id: ScopeId::new(index),
        }) {
            if !matches!(scope.kind(), ScopeKind::Module(_) | ScopeKind::Builtins) {
                continue;
            }
            writeln!(f, "scope {:?}", scope.kind())?;
            for symbol in scope.symbols() {
                write_symbol(f, "  ", &symbol)?;
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
                reference.access(),
            )?;
            match reference.base() {
                RefBase::Bare => {}
                RefBase::This => f.write_str(" via=this")?,
                RefBase::Builtin => f.write_str(" via=builtin")?,
                RefBase::Via(symbol) => write!(f, " via={}", self.symbol(symbol).name())?,
            }
            let path: Vec<_> = reference.path().collect();
            if !path.is_empty() {
                write!(f, " path={}", path.join("."))?;
            }
            if reference.has_open_path() {
                f.write_str(" path-open")?;
            }
            if let Some(args) = reference.call_args() {
                write!(f, " args={args}")?;
            }
            match reference.resolution() {
                Resolution::Symbol(symbol) => {
                    let symbol = self.symbol(symbol);
                    write!(f, " -> {:?} {}", symbol.kind(), symbol.name())?;
                }
                other => write!(f, " -> {other:?}")?,
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
