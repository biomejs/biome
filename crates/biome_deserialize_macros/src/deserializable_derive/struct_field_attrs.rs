#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructFieldAttrs {
    /// If `true`, bails on deserializing the entire struct if validation for
    /// this field fails.
    ///
    /// Note the struct may still be deserialized if the field is not present in
    /// the serialized representation at all. In that case `Default::default()`
    /// will be filled in.
    pub bail_on_error: bool,

    /// If set, this provides information about the deprecated of the field.
    pub deprecated: Option<DeprecatedField>,

    /// If set, the name passed to the deserializer (which was passed by the
    /// deserializer of the parent object) is also passed through to the
    /// deserializer of the field value.
    pub passthrough_name: bool,

    /// Optional name to use in the serialized format.
    ///
    /// See also: <https://serde.rs/field-attrs.html#rename>
    pub rename: Option<String>,

    /// If `true`, presence of this field is required for successful
    /// deserialization of the struct.
    ///
    /// Implies `bail_on_error`.
    pub required: bool,

    /// Optional validation function to be called on the field value.
    pub validate: Option<syn::Path>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeprecatedField {
    /// A generic message that explains what to do or why the field is deprecated.
    Message(String),

    /// Provides the path for a new field to use instead.
    UseInstead(String),
}

impl TryFrom<&Vec<syn::Attribute>> for StructFieldAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path().is_ident("deserializable") {
                attr.parse_nested_meta(|meta| {
                    let Ok(name) = meta.path.require_ident() else {
                        panic!("HERE {:?}", meta.path);
                    };
                    if name == "required" {
                        opts.required = true;
                    } else if name == "passthrough_name" {
                        opts.passthrough_name = true;
                    } else if name == "bail_on_error" {
                        opts.bail_on_error = true;
                    } else if name == "rename" {
                        opts.rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                    } else if name == "validate" {
                        opts.validate = Some(meta.value()?.parse::<syn::LitStr>()?.parse()?);
                    } else if name == "deprecated" {
                        meta.parse_nested_meta(|meta| {
                            let name = meta.path.require_ident()?;
                            let value = meta.value()?.parse::<syn::LitStr>()?.value();
                            if name == "use_instead" {
                                opts.deprecated = Some(DeprecatedField::UseInstead(value));
                            } else if name == "message" {
                                opts.deprecated = Some(DeprecatedField::Message(value));
                            } else {
                                return Err(meta.error("Unknown attribute"));
                            }
                            Ok(())
                        })?;
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
