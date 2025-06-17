use biome_string_case::Case;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub trait LanguageSrc {
    fn syntax_kind(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxKind"
        );
        quote! { #ident }
    }
    fn syntax_factory(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxFactory"
        );
        quote! { #ident }
    }
    fn syntax_node(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxNode"
        );
        quote! { #ident }
    }
    fn syntax_element(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxElement"
        );
        quote! { #ident }
    }
    fn syntax_token(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxToken"
        );
        quote! { #ident }
    }
    fn syntax_element_children(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxElementChildren"
        );
        quote! { #ident }
    }
    fn syntax_list(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "SyntaxList"
        );
        quote! { #ident }
    }

    fn language(&self) -> TokenStream {
        let ident = format_ident!(
            "{}{}",
            Case::Pascal.convert(self.language_prefix()),
            "Language"
        );
        quote! { #ident }
    }

    fn string_literal(&self) -> Ident {
        format_ident!(
            "{}{}",
            Case::Upper.convert(self.language_prefix()),
            "_STRING_LITERAL"
        )
    }

    /// Special characters of the language. Usually these are parenthesis, dots, commas, etc.
    fn punct(&self) -> &[(&str, &str)];
    /// Reserved keywords of the language
    fn keywords(&self) -> &[&str];
    /// Literals are special nodes that holds some **values** inside the language, for example: strings, numbers, etc.
    fn literals(&self) -> &[&str];
    /// Whitespaces, comments, identifiers, etc.
    fn tokens(&self) -> &[&str];

    /// Nodes of the CST. Usually you want to map these names from the `.ungram` file. For example:
    ///
    /// HtmlAttribute -> HTML_ATTRIBUTE
    /// HtmlBogus -> HTML_BOGUS
    fn nodes(&self) -> &[&str];

    fn prefixes(&self) -> &[&str];

    /// The name of the language that will be used to generate types.
    ///
    /// For example, returning "foo", will generate types like `FooSyntaxFactory`
    fn language_prefix(&self) -> &str;

    /// Maps a token or multiple tokens defined in the grammar to a "name" that can be used as field of a struct.
    ///
    /// For example, you could map the tokens `"||"` to `"or"`.
    ///
    /// Another example, you might want to map the start of an HTML comment to specific token:
    /// - `"<!--"` -> `"comment_start"`
    /// - `"-->"` -> `"comment_end"`
    ///
    /// For tokens that might conflict with Rust language like parenthesis, you must use single quotes:
    ///
    /// ```ignore
    /// fn to_token_name(&self, token_name: &str) -> &str {
    ///     match token_name {
    ///         "'('" => "l_braket",
    ///         "')'" => "r_braket",
    ///         "<!--" => "comment_start",
    ///         "-->" => "comment_end",
    ///         _ => token_name
    ///     }
    /// }
    /// ```
    fn to_method_name<'a>(&self, token_name: &'a str) -> &'a str;
}
