#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub(crate) enum TextPrintMode {
    /// Keep the original formatting. Don't attempt to optimize it. This is usually achieved
    /// by formatting the code as verbatim.
    #[default]
    Pristine,
    /// Usually used inside code blocks. It keeps the original formatting of the content,
    /// but it removes possible spaces if there's empty hard line.
    ///
    /// In the following example, the first line will keep only the newline, as there are only spaces
    /// ``````md
    /// ```
    ///
    /// ```
    /// ``````
    /// However, in the following example, spaces are keep as is because there's text (code):
    ///
    /// ``````md
    /// ```js
    ///    function f() {}
    /// ```
    /// ``````
    Clean,
    /// It removes the token/node
    _Remove,
    /// Replace the token/node
    _Replace,
    /// It cleans the code by using a trimming strategy
    Trim(TrimMode),
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub(crate) enum TrimMode {
    /// Trim the start of the list
    Start,
    /// Trim start and end of the list
    All,
    /// If the first and last [MdTextual] are `<` and `>` respectively, they are trimmed.
    /// If no link has been detected, if falls back to [Self::All]
    AutoLinkLike,
    /// This mode works similarly to [TrimMode::All], however, text that contains
    /// words and have more than trailing/leading spaces are normalized to one
    NormalizeWords,
    /// Don't trim anything
    #[default]
    None,
}

impl TextPrintMode {
    pub(crate) const fn is_start(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::Start))
    }

    pub(crate) const fn is_all(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::All))
    }

    pub(crate) const fn is_normalize_words(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::NormalizeWords))
    }

    pub(crate) const fn is_auto_link_like(&self) -> bool {
        matches!(self, Self::Trim(TrimMode::AutoLinkLike))
    }

    pub(crate) const fn is_pristine(&self) -> bool {
        matches!(self, Self::Pristine)
    }

    pub(crate) const fn is_clean(&self) -> bool {
        matches!(self, Self::Clean)
    }
}
