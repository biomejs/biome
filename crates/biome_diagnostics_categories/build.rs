use quote::{format_ident, quote};
use std::{env, fs, io, path::PathBuf};

macro_rules! define_categories {
    ( $( $name_link:literal : $link:literal, )* ; $( $name:literal , )* ) => {
        const CATEGORIES: &[(&str, Option<&str>)] = &[
            $( ($name_link, Some($link)), )*
            $( ($name, None), )*
        ];
    };
}

include!("src/categories.rs");

pub fn main() -> io::Result<()> {
    let mut metadata = Vec::with_capacity(CATEGORIES.len());
    let mut macro_arms = Vec::with_capacity(CATEGORIES.len());
    let mut parse_arms = Vec::with_capacity(CATEGORIES.len());
    let mut enum_variants = Vec::with_capacity(CATEGORIES.len());
    let mut concat_macro_arms = Vec::with_capacity(CATEGORIES.len());

    for (name, link) in CATEGORIES {
        let meta_name = name.replace('/', "_").to_uppercase();
        let meta_ident = format_ident!("{meta_name}");

        let link = if let Some(link) = link {
            quote! { Some(#link) }
        } else {
            quote! { None }
        };

        metadata.push(quote! {
            pub static #meta_ident: crate::Category = crate::Category {
                name: #name,
                link: #link,
            };
        });

        macro_arms.push(quote! {
            (#name) => { &$crate::registry::#meta_ident };
        });

        parse_arms.push(quote! {
            #name => Ok(&crate::registry::#meta_ident),
        });

        enum_variants.push(*name);

        let parts = name.split('/');
        concat_macro_arms.push(quote! {
            ( #( #parts ),* ) => { &$crate::registry::#meta_ident };
        });
    }

    let tokens = quote! {
        impl FromStr for &'static Category {
            type Err = ();

            fn from_str(name: &str) -> Result<Self, ()> {
                match name {
                    #( #parse_arms )*
                    _ => Err(()),
                }
            }
        }

        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for &'static Category {
            fn schema_name() -> String {
                String::from("Category")
            }

            fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::String.into()),
                    enum_values: Some(vec![#( #enum_variants.into() ),*]),
                    ..Default::default()
                })
            }
        }

        /// The `category!` macro can be used to statically lookup a category
        /// by name from the registry
        ///
        /// # Example
        ///
        /// ```
        /// # use biome_diagnostics_categories::{Category, category};
        /// let category: &'static Category = category!("internalError/io");
        /// assert_eq!(category.name(), "internalError/io");
        /// assert_eq!(category.link(), None);
        /// ```
        #[macro_export]
        macro_rules! category {
            #( #macro_arms )*

            ( $name:literal ) => {
                compile_error!(concat!("Unregistered diagnostic category \"", $name, "\", please add it to \"crates/biome_diagnostics_categories/src/categories.rs\""))
            };
            ( $( $parts:tt )* ) => {
                compile_error!(concat!("Invalid diagnostic category `", stringify!($( $parts )*), "`, expected a single string literal"))
            };
        }

        /// The `category_concat!` macro is a variant of `category!` using a
        /// slightly different syntax, for use in the `declare_group` and
        /// `declare_rule` macros in the analyzer
        #[macro_export]
        macro_rules! category_concat {
            #( #concat_macro_arms )*

            ( @compile_error $( $parts:tt )* ) => {
                compile_error!(concat!("Unregistered diagnostic category \"", $( $parts, )* "\", please add it to \"crates/biome_diagnostics_categories/src/categories.rs\""))
            };
            ( $( $parts:tt ),* ) => {
                $crate::category_concat!( @compile_error $( $parts )"/"* )
            };
            ( $( $parts:tt )* ) => {
                compile_error!(concat!("Invalid diagnostic category `", stringify!($( $parts )*), "`, expected a comma-separated list of string literals"))
            };
        }

        pub mod registry {
            #( #metadata )*
        }
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(
        PathBuf::from(out_dir).join("categories.rs"),
        tokens.to_string(),
    )?;

    Ok(())
}
