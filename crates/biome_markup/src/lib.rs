#![deny(clippy::use_self)]

use proc_macro_error2::*;
use proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree};
use quote::{ToTokens, quote};

struct StackEntry {
    name: Ident,
    attributes: Vec<(Ident, TokenTree)>,
}

impl ToTokens for StackEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        tokens.extend(quote! {
            biome_console::MarkupElement::#name
        });

        if !self.attributes.is_empty() {
            let attributes: Vec<_> = self
                .attributes
                .iter()
                .map(|(key, value)| quote! { #key: (#value).into() })
                .collect();

            tokens.extend(quote! { { #( #attributes ),* } })
        }
    }
}

/// Creates a markup structure for formatted console output.
///
/// The `markup!` macro provides an XML-like syntax for creating styled and structured
/// console output. It generates a `biome_console::Markup` instance containing markup
/// nodes that define the styling and content hierarchy.
///
/// # Syntax
///
/// The macro accepts a mix of elements, text literals, and interpolated expressions:
///
/// - **Elements**: `<ElementName>` ... `</ElementName>`
/// - **Self-closing elements**: `<ElementName />`
/// - **Attributes**: `<ElementName attr="value">` or `<ElementName attr={expression}>`
/// - **Text content**: String literals between elements
/// - **Dynamic content**: `{expression}` for interpolating Rust expressions
///
/// ## Elements
///
/// Elements correspond to `biome_console::MarkupElement` variants and are used to
/// apply styling or structure to content. Common elements include:
///
/// - `<Emphasis>` - Emphasized/bold text
/// - `<Dim>` - Dimmed/faint text
/// - `<Italic>` - Italic text
/// - `<Underline>` - Underlined text
/// - `<Error>` - Error styling (typically red)
/// - `<Warn>` - Warning styling (typically yellow)
/// - `<Info>` - Info styling (typically blue)
/// - `<Success>` - Success styling (typically green)
/// - `<Debug>` - Debug styling (typically blue/cyan)
/// - `<Trace>` - Trace styling (typically magenta)
/// - `<Inverse>` - Inverse video styling
/// - `<Hyperlink>` - Hyperlink with href attribute
///
/// Elements can be nested to combine styles.
///
/// ## Attributes
///
/// Some elements accept attributes to customize their behavior:
///
/// - Literal values: `attr="value"`
/// - Expressions: `attr={rust_expression}`
///
/// Attribute values are converted using `.into()`, so they must implement the
/// appropriate trait for the expected type.
///
/// ## Content
///
/// Content can be:
///
/// - **String literals**: Directly embedded as text nodes
/// - **Interpolated expressions**: Wrapped in `{}` and must implement `Display`
///
/// # Examples
///
/// ## Basic styling
///
/// ```
/// # use biome_markup::markup;
/// let msg = markup! {
///     <Emphasis>"Hello, world!"</Emphasis>
/// };
/// ```
///
/// ## Nested elements
///
/// ```
/// # use biome_markup::markup;
/// let msg = markup! {
///     <Error><Emphasis>"Critical error"</Emphasis></Error>
/// };
/// ```
///
/// ## Mixed content
///
/// ```
/// # use biome_markup::markup;
/// let msg = markup! {
///     "Found " <Emphasis>"3"</Emphasis> " errors"
/// };
/// ```
///
/// ## Dynamic content with interpolation
///
/// ```
/// # use biome_markup::markup;
/// let count = 5;
/// let msg = markup! {
///     "Found " <Emphasis>{count}</Emphasis> " errors"
/// };
/// ```
///
/// ## Elements with attributes
///
/// ```
/// # use biome_markup::markup;
/// let path = "src/main.rs";
/// let msg = markup! {
///     <Hyperlink href={path}>"Click here"</Hyperlink>
/// };
/// ```
///
/// ## Complex example with multiple elements and interpolation
///
/// ```
/// # use biome_markup::markup;
/// let filename = "config.json";
/// let line = 42;
/// let error_msg = "Invalid syntax";
///
/// let msg = markup! {
///     <Error>"Error"</Error> " in " <Emphasis>{filename}</Emphasis> " at line " {line} ": " <Dim>{error_msg}</Dim>
/// };
/// ```
///
/// # Errors
///
/// The macro will produce compile-time errors for:
///
/// - Mismatched opening and closing tags
/// - Unclosed elements
/// - Unexpected closing elements
/// - Invalid syntax (missing `=` in attributes, invalid punctuation, etc.)
///
/// # Notes
///
/// - All elements must be properly closed unless self-closing
/// - Element names correspond to `MarkupElement` enum variants
/// - The macro expands to a `biome_console::Markup` instance
/// - Interpolated expressions are cast to `&dyn Display`
#[proc_macro]
#[proc_macro_error]
pub fn markup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = TokenStream::from(input).into_iter().peekable();
    let mut stack = Vec::new();
    let mut output = Vec::new();

    while let Some(token) = input.next() {
        match token {
            TokenTree::Punct(punct) => match punct.as_char() {
                '<' => {
                    let is_closing_element = match input.peek() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '/' => {
                            // SAFETY: Guarded by above call to peek
                            input.next().unwrap();
                            true
                        }
                        _ => false,
                    };

                    let name = match input.next() {
                        Some(TokenTree::Ident(ident)) => ident,
                        Some(token) => abort!(token.span(), "unexpected token"),
                        None => abort_call_site!("unexpected end of input"),
                    };

                    let mut attributes = Vec::new();
                    while let Some(TokenTree::Ident(_)) = input.peek() {
                        // SAFETY: these panics are checked by the above call to peek
                        let attr = match input.next().unwrap() {
                            TokenTree::Ident(attr) => attr,
                            _ => unreachable!(),
                        };

                        match input.next() {
                            Some(TokenTree::Punct(punct)) => {
                                if punct.as_char() != '=' {
                                    abort!(punct.span(), "unexpected token");
                                }
                            }
                            Some(token) => abort!(token.span(), "unexpected token"),
                            None => abort_call_site!("unexpected end of input"),
                        }

                        let value = match input.next() {
                            Some(TokenTree::Literal(value)) => TokenTree::Literal(value),
                            Some(TokenTree::Group(group)) => {
                                TokenTree::Group(Group::new(Delimiter::None, group.stream()))
                            }
                            Some(token) => abort!(token.span(), "unexpected token"),
                            None => abort_call_site!("unexpected end of input"),
                        };

                        attributes.push((attr, value));
                    }

                    let is_self_closing = match input.next() {
                        Some(TokenTree::Punct(punct)) => match punct.as_char() {
                            '>' => false,
                            '/' if !is_closing_element => {
                                match input.next() {
                                    Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {}
                                    Some(token) => abort!(token.span(), "unexpected token"),
                                    None => abort_call_site!("unexpected end of input"),
                                }
                                true
                            }
                            _ => abort!(punct.span(), "unexpected token"),
                        },
                        Some(token) => abort!(token.span(), "unexpected token"),
                        None => abort_call_site!("unexpected end of input"),
                    };

                    if !is_closing_element {
                        stack.push(StackEntry {
                            name: name.clone(),
                            attributes: attributes.clone(),
                        });
                    } else if let Some(top) = stack.last() {
                        // Only verify the coherence of the top element on the
                        // stack with a closing element, skip over the check if
                        // the stack is empty as that error will be handled
                        // when the top element gets popped off the stack later
                        let name_str = name.to_string();
                        let top_str = top.name.to_string();
                        if name_str != top_str {
                            abort!(
                                name.span(), "closing element mismatch";
                                close = "found closing element {}", name_str;
                                open = top.name.span() => "expected {}", top_str
                            );
                        }
                    }

                    if (is_closing_element || is_self_closing) && stack.pop().is_none() {
                        abort!(name.span(), "unexpected closing element");
                    }
                }
                _ => {
                    abort!(punct.span(), "unexpected token");
                }
            },
            TokenTree::Literal(literal) => {
                let elements: Vec<_> = stack
                    .iter()
                    .map(|entry| {
                        quote! { #entry }
                    })
                    .collect();

                output.push(quote! {
                    biome_console::MarkupNode {
                        elements: &[ #( #elements ),* ],
                        content: &(#literal),
                    }
                });
            }
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Brace => {
                    let elements: Vec<_> = stack.iter().map(|entry| quote! { #entry }).collect();

                    let body = group.stream();
                    output.push(quote! {
                        biome_console::MarkupNode {
                            elements: &[ #( #elements ),* ],
                            content: &(#body) as &dyn biome_console::fmt::Display,
                        }
                    });
                }
                _ => abort!(group.span(), "unexpected token"),
            },
            TokenTree::Ident(_) => abort!(token.span(), "unexpected token"),
        }
    }

    if let Some(top) = stack.pop() {
        abort!(top.name.span(), "unclosed element");
    }

    quote! { biome_console::Markup(&[ #( #output ),* ]) }.into()
}
