use std::str::FromStr;

/// Attributes for struct and enum.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ContainerAttrs {
    pub with_validator: bool,
    /// Deserialize the given `from` type, then convert to the annotated type
    ///
    /// See also: <https://serde.rs/container-attrs.html#from>
    pub from: Option<syn::Path>,
    /// Deserialize the given `try_from` type, then try converting to the annotated type
    ///
    /// See also: <https://serde.rs/container-attrs.html#try_from>
    pub try_from: Option<syn::Path>,
    /// Ignore unknown fields in a struct upon deserialization.
    pub unknown_fields: Option<UnknownFields>,
}

/// Attributes for struct that control how unkinown fields are handled.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum UnknownFields {
    #[default]
    Warn,
    Deny,
    Allow,
}
impl FromStr for UnknownFields {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "warn" => Ok(Self::Warn),
            "deny" => Ok(Self::Deny),
            "allow" => Ok(Self::Allow),
            _ => Err("unknown_fields takes a value amomg `deny`, `warn`, and `allow`."),
        }
    }
}

impl TryFrom<&Vec<syn::Attribute>> for ContainerAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path().is_ident("deserializable") {
                attr.parse_nested_meta(|meta| {
                    let name = meta.path.require_ident()?;
                    if name == "with_validator" {
                        opts.with_validator = true
                    } else if name == "from" {
                        opts.from = Some(meta.value()?.parse::<syn::LitStr>()?.parse()?);
                    } else if name == "try_from" {
                        opts.try_from = Some(meta.value()?.parse::<syn::LitStr>()?.parse()?);
                    } else if name == "unknown_fields" {
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        match UnknownFields::from_str(&lit.value()) {
                            Ok(value) => opts.unknown_fields = Some(value),
                            Err(error) => return Err(meta.error(error)),
                        }
                    } else {
                        return Err(meta.error("Unknown attribute"));
                    }
                    Ok(())
                })?;
            } else if attr.path().is_ident("serde") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("from") {
                        opts.from = Some(meta.value()?.parse::<syn::LitStr>()?.parse()?);
                    } else if meta.path.is_ident("try_from") {
                        opts.try_from = Some(meta.value()?.parse::<syn::LitStr>()?.parse()?);
                    } else if meta.path.is_ident("deny_unknown_fields") {
                        if opts.unknown_fields.is_none() {
                            opts.unknown_fields = Some(UnknownFields::Deny);
                        }
                    } else {
                        // Don't fail on unrecognized Serde attrs
                    }
                    Ok(())
                })
                .ok(); // Don't fail for serde attrs parsing
            }
        }
        Ok(opts)
    }
}
