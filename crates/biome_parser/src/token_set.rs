use rome_rowan::SyntaxKind;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet<K: SyntaxKind>([u128; 2], PhantomData<K>);

impl<K: SyntaxKind> TokenSet<K> {
    pub const EMPTY: TokenSet<K> = TokenSet([0; 2], PhantomData);

    pub fn singleton(kind: K) -> Self {
        unsafe { TokenSet::from_raw(kind.to_raw().0) }
    }

    pub const fn union(self, other: TokenSet<K>) -> Self {
        TokenSet(
            [self.0[0] | other.0[0], self.0[1] | other.0[1]],
            PhantomData,
        )
    }

    pub fn contains(&self, kind: K) -> bool {
        let kind = kind.to_raw().0;
        let num = kind as usize;
        match num {
            0..=127 => self.0[0] & mask(kind)[0] != 0,
            _ => self.0[1] & mask(kind)[1] != 0,
        }
    }

    /// Constructs a token set for a single kind from a kind's raw `u16` representation.
    ///
    /// # Safety
    ///
    /// This method is marked unsafe to discourage its usage over using `TokenSet::singleton`.
    /// It exists to support the `token_set` macro in a `const` context.
    #[doc(hidden)]
    pub const unsafe fn from_raw(kind: u16) -> Self {
        TokenSet(mask(kind), PhantomData)
    }
}

const fn mask(kind: u16) -> [u128; 2] {
    let num = kind as usize;
    match num {
        0..=127 => [1u128 << num, 0],
        _ => [0, 1u128 << (num - 127)],
    }
}

/// Utility macro for making a new token set
#[macro_export]
macro_rules! token_set {
    ($($t:expr),*) => {{
            use $crate::TokenSet;
            TokenSet::EMPTY$(.union(unsafe { TokenSet::from_raw($t as u16) }))*
        }};
    ($($t:expr),* ,) => { token_set!($($t),*) };
}
