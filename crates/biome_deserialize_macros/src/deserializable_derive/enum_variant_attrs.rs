#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EnumVariantAttrs {
    /// Optional name to use in the serialized format.
    ///
    /// See also: <https://serde.rs/variant-attrs.html#rename>
    pub rename: Option<String>,
}

impl TryFrom<&Vec<syn::Attribute>> for EnumVariantAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path().is_ident("deserializable") {
                attr.parse_nested_meta(|meta| {
                    let name = meta.path.require_ident()?;
                    if name == "rename" {
                        opts.rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                    } else {
                        return Err(meta.error("Unknown attribute"));
                    }
                    Ok(())
                })?;
            } else if attr.path().is_ident("serde") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename") {
                        opts.rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
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
