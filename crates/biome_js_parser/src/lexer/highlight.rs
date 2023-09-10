use crate::*;

pub use ansi_term::{self, ANSIGenericString, Color, Style};
use atty::is;

/// A structure for syntax highlighting pieces of JavaScript source code
/// using ANSI.
///
/// The highlighter will auto detect if stderr or stdout are terminals, if
/// they are not then it will return the original uncolored source code.
/// All errors encountered while lexing are ignored.
///
/// The highlighter is iterator based, which allows for coloring a part of code
/// at a time.
/// The highlighter's position can be controlled through various methods which allows
/// for reuse of the highlighter without the need to rescan the source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Highlighter<'s> {
    pub source: &'s str,
    tokens: Vec<Token>,
    /// Current token position
    cur: usize,
    /// Current byte index in source
    cur_idx: usize,
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color::RGB($r, $g, $b)
    };
}

impl<'s> Highlighter<'s> {
    /// Make a new highlighter, this will invoke the lexer to get tokens.
    pub fn new(source: &'s str) -> Highlighter<'s> {
        let tokens = Lexer::from_str(source, 0).map(|t| t.0).collect();

        Self {
            source,
            tokens,
            cur: 0,
            cur_idx: 0,
        }
    }

    fn check_terminal(&self) -> bool {
        is(atty::Stream::Stderr) && is(atty::Stream::Stdout)
    }

    /// Reset the highlighter to the start of the source code
    pub fn reset(&mut self) {}

    /// Consume the rest of the highlighter's tokens and turn them into an ANSI colored string.
    /// This returns an unaltered string if stdout and stderr are not terminals.
    pub fn color(&mut self) -> String {
        if !self.check_terminal() {
            let ret = self.source[self.cur_idx..self.source.len()].to_string();
            self.cur = self.tokens.len();
            self.cur_idx = self.source.len();
            return ret;
        }

        self.map(|x| x.to_string()).collect()
    }

    fn src(&self) -> &'s str {
        let range = TextRange::at(
            TextSize::from(self.cur_idx as u32),
            self.tokens.get(self.cur).unwrap().len(),
        );
        &self.source[range]
    }
}

const PURPLE_IDENT: [&str; 4] = ["let", "class", "await", "yield"];
const BUILTINS: [&str; 27] = [
    "Math",
    "Promise",
    "Number",
    "String",
    "Date",
    "Infinity",
    "NaN",
    "undefined",
    "globalThis",
    "Object",
    "Function",
    "Symbol",
    "Boolean",
    "Error",
    "EvalError",
    "InternalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "Number",
    "BigInt",
    "RegExp",
    "Array",
    "Map",
    "Set",
    "JSON",
];

impl<'s> Iterator for Highlighter<'s> {
    /// An individual colored token, you can see the color used by checking the string's style foreground
    type Item = ANSIGenericString<'s, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.get(self.cur) == None {
            return None;
        }

        let color = match self.tokens.get(self.cur)?.kind {
            T!['{'] | T!['}'] | T!['('] | T![')'] => rgb![255, 215, 0],
            T![import] => rgb![97, 175, 239],
            T![ident] if PURPLE_IDENT.contains(&self.src()) => rgb![198, 120, 221],
            T![ident] if self.src() == "from" => rgb![97, 175, 239],
            T![ident] if BUILTINS.contains(&self.src()) => rgb![229, 192, 123],
            T![ident] => rgb![224, 108, 117],
            T![instanceof] | T![new] | T![?] | T![delete] | T![:] | T![const] => {
                rgb![198, 120, 221]
            }
            t if t.is_punct() => rgb![86, 182, 194],
            t if t.is_keyword() => rgb![198, 120, 221],
            JsSyntaxKind::JS_STRING_LITERAL
            | JsSyntaxKind::BACKTICK
            | JsSyntaxKind::TEMPLATE_CHUNK => {
                rgb![152, 195, 121]
            }
            JsSyntaxKind::JS_NUMBER_LITERAL => rgb![209, 154, 102],
            JsSyntaxKind::DOLLAR_CURLY => rgb![198, 120, 221],
            JsSyntaxKind::ERROR_TOKEN => rgb![244, 71, 71],
            JsSyntaxKind::COMMENT => rgb![127, 132, 142],
            _ => Color::White,
        };

        let string = self.src();
        let token_len: usize = self.tokens.get(self.cur).unwrap().len().into();
        self.cur_idx += token_len;
        self.cur += 1;
        Some(color.paint(string))
    }
}

/// Colors a piece of source code using ANSI.
/// The string returned will be unaltered if stdout and stderr are not terminals.
pub fn color(source: &str) -> String {
    Highlighter::new(source).color()
}
