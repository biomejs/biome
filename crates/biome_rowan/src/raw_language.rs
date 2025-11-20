//! Provides a sample language implementation that is useful in API explanation or tests
use crate::raw_language::RawLanguageKind::{COMMA_TOKEN, LITERAL_EXPRESSION, ROOT};
use crate::{
    AstNode, AstSeparatedList, Language, ParsedChildren, RawNodeSlots, RawSyntaxKind,
    RawSyntaxNode, SyntaxFactory, SyntaxKind, SyntaxKindSet, SyntaxList, SyntaxNode, TreeBuilder,
};

#[doc(hidden)]
#[derive(Debug, Default, Hash, Copy, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct RawLanguage;

impl Language for RawLanguage {
    type Kind = RawLanguageKind;
    type Root = RawLanguageRoot;
}

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
#[expect(bad_style)]
pub enum RawLanguageKind {
    ROOT = 0,
    EXPRESSION_LIST = 1,
    SEPARATED_EXPRESSION_LIST = 2,
    COMMA_TOKEN = 3,
    STRING_TOKEN = 4,
    NUMBER_TOKEN = 5,
    LITERAL_EXPRESSION = 6,
    BOGUS = 7,
    FOR_KW = 8,
    L_PAREN_TOKEN = 9,
    SEMICOLON_TOKEN = 10,
    R_PAREN_TOKEN = 11,
    EQUAL_TOKEN = 12,
    LET_TOKEN = 13,
    CONDITION = 14,
    PLUS_TOKEN = 15,
    WHITESPACE = 16,
    TOMBSTONE = 17,
    EOF = 18,
    __LAST,
}

impl SyntaxKind for RawLanguageKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        self == &Self::BOGUS
    }

    fn to_bogus(&self) -> Self {
        Self::BOGUS
    }

    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[expect(unsafe_code)]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        assert!(raw.0 < Self::__LAST as u16);

        unsafe { std::mem::transmute::<u16, Self>(raw.0) }
    }

    fn is_root(&self) -> bool {
        self == &Self::ROOT
    }

    fn is_list(&self) -> bool {
        matches!(
            self,
            Self::EXPRESSION_LIST | Self::SEPARATED_EXPRESSION_LIST
        )
    }

    fn is_trivia(self) -> bool {
        self == Self::WHITESPACE
    }

    fn to_string(&self) -> Option<&'static str> {
        let str = match self {
            COMMA_TOKEN => ",",
            Self::FOR_KW => "for",
            Self::L_PAREN_TOKEN => "(",
            Self::SEMICOLON_TOKEN => ";",
            Self::R_PAREN_TOKEN => ")",
            Self::EQUAL_TOKEN => "=",
            Self::LET_TOKEN => "let",
            Self::PLUS_TOKEN => "+",
            _ => return None,
        };
        Some(str)
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawLanguageRoot {
    node: SyntaxNode<RawLanguage>,
}

impl AstNode for RawLanguageRoot {
    type Language = RawLanguage;

    const KIND_SET: SyntaxKindSet<RawLanguage> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ROOT as u16));

    fn can_cast(kind: RawLanguageKind) -> bool {
        kind == ROOT
    }

    fn cast(syntax: SyntaxNode<RawLanguage>) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == ROOT {
            Some(Self { node: syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode<RawLanguage> {
        &self.node
    }

    fn into_syntax(self) -> SyntaxNode<RawLanguage> {
        self.node
    }
}

#[doc(hidden)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LiteralExpression {
    node: SyntaxNode<RawLanguage>,
}

impl AstNode for LiteralExpression {
    type Language = RawLanguage;

    const KIND_SET: SyntaxKindSet<RawLanguage> =
        SyntaxKindSet::from_raw(RawSyntaxKind(LITERAL_EXPRESSION as u16));

    fn can_cast(kind: RawLanguageKind) -> bool {
        kind == LITERAL_EXPRESSION
    }

    fn cast(syntax: SyntaxNode<RawLanguage>) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LITERAL_EXPRESSION {
            Some(Self { node: syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode<RawLanguage> {
        &self.node
    }

    fn into_syntax(self) -> SyntaxNode<RawLanguage> {
        self.node
    }
}

#[doc(hidden)]
pub struct SeparatedExpressionList {
    syntax_list: SyntaxList<RawLanguage>,
}

impl SeparatedExpressionList {
    pub fn new(list: SyntaxList<RawLanguage>) -> Self {
        Self { syntax_list: list }
    }
}

impl AstSeparatedList for SeparatedExpressionList {
    type Language = RawLanguage;
    type Node = LiteralExpression;

    fn syntax_list(&self) -> &SyntaxList<RawLanguage> {
        &self.syntax_list
    }

    fn into_syntax_list(self) -> SyntaxList<RawLanguage> {
        self.syntax_list
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct RawLanguageSyntaxFactory;

impl SyntaxFactory for RawLanguageSyntaxFactory {
    type Kind = RawLanguageKind;

    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            RawLanguageKind::BOGUS | RawLanguageKind::ROOT => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            RawLanguageKind::EXPRESSION_LIST => {
                Self::make_node_list_syntax(kind, children, |kind| kind == LITERAL_EXPRESSION)
            }
            RawLanguageKind::SEPARATED_EXPRESSION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                |kind| kind == LITERAL_EXPRESSION,
                COMMA_TOKEN,
                true,
            ),
            RawLanguageKind::LITERAL_EXPRESSION => {
                let actual_len = children.len();

                if actual_len > 1 {
                    return RawSyntaxNode::new(kind.to_bogus(), children.into_iter().map(Some));
                }

                let mut elements = children.into_iter();
                let current_element = elements.next();

                if let Some(element) = &current_element {
                    if !matches!(
                        element.kind(),
                        RawLanguageKind::STRING_TOKEN | RawLanguageKind::NUMBER_TOKEN
                    ) {
                        return RawSyntaxNode::new(
                            kind.to_bogus(),
                            std::iter::once(current_element),
                        );
                    }
                } else {
                    return RawSyntaxNode::new(kind, std::iter::once(None));
                }

                RawSyntaxNode::new(kind, std::iter::once(current_element))
            }

            RawLanguageKind::CONDITION => {
                let mut elements = (&children).into_iter();
                let mut current_element = elements.next();
                let mut slots: RawNodeSlots<3> = Default::default();

                if let Some(element) = &current_element
                    && element.kind() == RawLanguageKind::L_PAREN_TOKEN
                {
                    slots.mark_present();
                    current_element = elements.next();
                }

                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == RawLanguageKind::LITERAL_EXPRESSION
                {
                    slots.mark_present();
                    current_element = elements.next();
                }

                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == RawLanguageKind::R_PAREN_TOKEN
                {
                    slots.mark_present();
                    current_element = elements.next();
                }

                slots.next_slot();

                if current_element.is_some() {
                    return RawSyntaxNode::new(kind.to_bogus(), children.into_iter().map(Some));
                }

                slots.into_node(kind, children)
            }
            _ => unreachable!("{:?} is not a node kind", kind),
        }
    }
}

#[doc(hidden)]
pub type RawSyntaxTreeBuilder<'a> = TreeBuilder<'a, RawLanguage, RawLanguageSyntaxFactory>;
