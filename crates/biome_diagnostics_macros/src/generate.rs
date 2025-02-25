use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error2::*;
use quote::quote;

use crate::parse::{
    DeriveEnumInput, DeriveInput, DeriveStructInput, StaticOrDynamic, StringOrMarkup,
};

pub(crate) fn generate_diagnostic(input: DeriveInput) -> TokenStream {
    match input {
        DeriveInput::DeriveStructInput(input) => generate_struct_diagnostic(input),
        DeriveInput::DeriveEnumInput(input) => generate_enum_diagnostic(input),
    }
}

fn generate_struct_diagnostic(input: DeriveStructInput) -> TokenStream {
    let category = generate_category(&input);
    let severity = generate_severity(&input);
    let description = generate_description(&input);
    let message = generate_message(&input);
    let advices = generate_advices(&input);
    let verbose_advices = generate_verbose_advices(&input);
    let location = generate_location(&input);
    let tags = generate_tags(&input);
    let source = generate_source(&input);

    let generic_params = if !input.generics.params.is_empty() {
        let lt_token = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt_token = &input.generics.gt_token;
        quote! { #lt_token #params #gt_token }
    } else {
        quote!()
    };

    let ident = input.ident;
    let generics = input.generics;

    quote! {
        impl #generic_params biome_diagnostics::Diagnostic for #ident #generics {
            #category
            #severity
            #description
            #message
            #advices
            #verbose_advices
            #location
            #tags
            #source
        }
    }
}

fn generate_category(input: &DeriveStructInput) -> TokenStream {
    let category = match &input.category {
        Some(StaticOrDynamic::Static(value)) => quote! {
            biome_diagnostics::category!(#value)
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            self.#value
        },
        None => return quote!(),
    };

    quote! {
        fn category(&self) -> Option<&'static biome_diagnostics::Category> {
            Some(#category)
        }
    }
}

fn generate_severity(input: &DeriveStructInput) -> TokenStream {
    let severity = match &input.severity {
        Some(StaticOrDynamic::Static(value)) => quote! {
            biome_diagnostics::Severity::#value
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            self.#value
        },
        None => return quote!(),
    };

    quote! {
        fn severity(&self) -> biome_diagnostics::Severity {
            #severity
        }
    }
}

fn generate_description(input: &DeriveStructInput) -> TokenStream {
    let description = match &input.description {
        Some(StaticOrDynamic::Static(StringOrMarkup::String(value))) => {
            let mut format_string = String::new();
            let mut format_params = Vec::new();

            let input = value.value();
            let mut input = input.as_str();

            while let Some(idx) = input.find('{') {
                let (before, after) = input.split_at(idx);
                format_string.push_str(before);

                let after = &after[1..];
                format_string.push('{');

                if let Some(after) = after.strip_prefix('{') {
                    input = after;
                    continue;
                }

                let end = match after.find([':', '}']) {
                    Some(end) => end,
                    None => abort!(value.span(), "failed to parse format string"),
                };

                let (ident, after) = after.split_at(end);
                let ident = Ident::new(ident, Span::call_site());
                format_params.push(quote! { self.#ident });

                input = after;
            }

            if !input.is_empty() {
                format_string.push_str(input);
            }

            if format_params.is_empty() {
                quote! {
                    fmt.write_str(#format_string)
                }
            } else {
                quote! {
                    fmt.write_fmt(::std::format_args!(#format_string, #( #format_params ),*))
                }
            }
        }
        Some(StaticOrDynamic::Static(StringOrMarkup::Markup(markup))) => quote! {
            let mut buffer = Vec::new();

            let write = biome_diagnostics::termcolor::NoColor::new(&mut buffer);
            let mut write = biome_diagnostics::console::fmt::Termcolor(write);
            let mut write = biome_diagnostics::console::fmt::Formatter::new(&mut write);

            use biome_diagnostics::console as biome_console;
            write.write_markup(&biome_diagnostics::console::markup!{ #markup })
                .map_err(|_| ::std::fmt::Error)?;

            fmt.write_str(::std::str::from_utf8(&buffer).map_err(|_| ::std::fmt::Error)?)
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            fmt.write_fmt(::std::format_args!("{}", self.#value))
        },
        None => return quote!(),
    };

    quote! {
        fn description(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            #description
        }
    }
}

fn generate_message(input: &DeriveStructInput) -> TokenStream {
    let message = match &input.message {
        Some(StaticOrDynamic::Static(StringOrMarkup::String(value))) => quote! {
            fmt.write_str(#value)
        },
        Some(StaticOrDynamic::Static(StringOrMarkup::Markup(markup))) => quote! {
            use biome_diagnostics::console as biome_console;
            fmt.write_markup(biome_diagnostics::console::markup!{ #markup })
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            biome_diagnostics::console::fmt::Display::fmt(&self.#value, fmt)
        },
        None => return quote!(),
    };

    quote! {
        fn message(&self, fmt: &mut biome_diagnostics::console::fmt::Formatter<'_>) -> ::std::io::Result<()> {
            #message
        }
    }
}

fn generate_advices(input: &DeriveStructInput) -> TokenStream {
    if input.advices.is_empty() {
        return quote!();
    }

    let advices = input.advices.iter();

    quote! {
        fn advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> ::std::io::Result<()> {
            #( biome_diagnostics::Advices::record(&self.#advices, visitor)?; )*
            Ok(())
        }
    }
}

fn generate_verbose_advices(input: &DeriveStructInput) -> TokenStream {
    if input.verbose_advices.is_empty() {
        return quote!();
    }

    let verbose_advices = input.verbose_advices.iter();

    quote! {
        fn verbose_advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> ::std::io::Result<()> {
            #( biome_diagnostics::Advices::record(&self.#verbose_advices, visitor)?; )*
            Ok(())
        }
    }
}

fn generate_location(input: &DeriveStructInput) -> TokenStream {
    if input.location.is_empty() {
        return quote!();
    }

    let field = input.location.iter().map(|(field, _)| field);
    let method = input.location.iter().map(|(_, method)| method);

    quote! {
        fn location(&self) -> biome_diagnostics::Location<'_> {
            biome_diagnostics::Location::builder()
                #( .#method(&self.#field) )*
                .build()
        }
    }
}

fn generate_tags(input: &DeriveStructInput) -> TokenStream {
    let tags = match &input.tags {
        Some(StaticOrDynamic::Static(value)) => {
            let values = value.iter();
            quote! {
                #( biome_diagnostics::DiagnosticTags::#values )|*
            }
        }
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            self.#value
        },
        None => return quote!(),
    };

    quote! {
        fn tags(&self) -> biome_diagnostics::DiagnosticTags {
            #tags
        }
    }
}

fn generate_source(input: &DeriveStructInput) -> TokenStream {
    match &input.source {
        Some(value) => quote! {
            fn source(&self) -> Option<&dyn biome_diagnostics::Diagnostic> {
                self.#value.as_deref()
            }
        },
        None => quote!(),
    }
}

fn generate_enum_diagnostic(input: DeriveEnumInput) -> TokenStream {
    let generic_params = if !input.generics.params.is_empty() {
        let lt_token = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt_token = &input.generics.gt_token;
        quote! { #lt_token #params #gt_token }
    } else {
        quote!()
    };

    let ident = input.ident;
    let generics = input.generics;
    let variants: Vec<_> = input
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();

    quote! {
        impl #generic_params biome_diagnostics::Diagnostic for #ident #generics {
            fn category(&self) -> Option<&'static biome_diagnostics::Category> {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::category(error),)*
                }
            }

            fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::description(error, fmt),)*
                }
            }

            fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::message(error, fmt),)*
                }
            }

            fn severity(&self) -> biome_diagnostics::Severity {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::severity(error),)*
                }
            }

            fn tags(&self) -> biome_diagnostics::DiagnosticTags {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::tags(error),)*
                }
            }

            fn location(&self) -> biome_diagnostics::Location<'_> {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::location(error),)*
                }
            }

            fn source(&self) -> Option<&dyn biome_diagnostics::Diagnostic> {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::source(error),)*
                }
            }

            fn advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> std::io::Result<()> {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::advices(error, visitor),)*
                }
            }

            fn verbose_advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> std::io::Result<()> {
                match self {
                    #(Self::#variants(error) => biome_diagnostics::Diagnostic::verbose_advices(error, visitor),)*
                }
            }
        }
    }
}
