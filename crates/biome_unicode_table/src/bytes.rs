use Dispatch::*;

/// Every handler a byte coming in could be mapped to
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Dispatch {
    /// Error token
    ERR,

    /// Whitespace
    WHS,

    /// Exclamation
    EXL,

    /// Single `'` or Double quote `"`
    QOT,

    /// ASCII letter or `_`
    IDT,

    /// Dollar sign `$`
    DOL,

    /// Hash `#`
    HAS,

    /// Percentage `%`
    PRC,

    /// Ampersand `&`
    AMP,

    /// Left paren `(`
    PNO,

    /// Right paren `)`
    PNC,

    /// Multiply `*`
    MUL,

    /// Plus `+`
    PLS,

    /// Comma `,`
    COM,

    /// Minus `-`
    MIN,

    /// Dot `.`
    PRD,

    /// Slash `/`
    SLH,

    /// Zero 0
    ZER,

    /// Digit (1-9)
    DIG,

    /// Colon `:`
    COL,

    /// Semicolon `;`
    SEM,

    ///`Less than `<`
    LSS,

    /// Equal `=`
    EQL,

    /// More than `>`
    MOR,
    /// Question `?`
    QST,
    /// At `@`
    AT_,

    /// Left bracket `[`
    BTO,

    /// Backslash `\`
    BSL,

    /// Right bracket `]`
    BTC,

    /// `^`
    CRT,

    /// Tick `
    TPL,

    /// Left curly bracket `{`
    BEO,

    /// Pipe `|`
    PIP,

    /// Right curly bracket `}`
    BEC,

    /// Tilde `~`
    TLD,

    /// Unicode range (non ASCII)
    UNI,
}

// A lookup table mapping any incoming byte to a handler function
// This is taken from the ratel project lexer and modified
// FIXME: Should we ignore the first ascii control chars which are nearly never seen instead of returning Err?
pub(crate) static DISPATCHER: [Dispatch; 256] = [
    //0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F   //
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, WHS, WHS, WHS, WHS, WHS, ERR, ERR, // 0
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, // 1
    WHS, EXL, QOT, HAS, DOL, PRC, AMP, QOT, PNO, PNC, MUL, PLS, COM, MIN, PRD, SLH, // 2
    ZER, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, COL, SEM, LSS, EQL, MOR, QST, // 3
    AT_, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BTO, BSL, BTC, CRT, IDT, // 5
    TPL, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 6
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BEO, PIP, BEC, TLD, ERR, // 7
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 8
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 9
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // A
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // B
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // F
];
