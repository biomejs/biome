use biome_rowan::SyntaxKind;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet<K: SyntaxKind>([u128; 3], PhantomData<K>);

impl<K: SyntaxKind> TokenSet<K> {
    pub const EMPTY: Self = Self([0; 3], PhantomData);

    pub fn singleton(kind: K) -> Self {
        unsafe { Self::from_raw(kind.to_raw().0) }
    }

    pub const fn union(self, other: Self) -> Self {
        Self(
            [
                self.0[0] | other.0[0],
                self.0[1] | other.0[1],
                self.0[2] | other.0[2],
            ],
            PhantomData,
        )
    }

    pub fn contains(&self, kind: K) -> bool {
        let kind = kind.to_raw().0;
        let num = kind as usize;
        match num {
            0..=127 => self.0[0] & mask(kind)[0] != 0,
            128..=255 => self.0[1] & mask(kind)[1] != 0,
            256..=383 => self.0[2] & mask(kind)[2] != 0,
            _ => false,
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
        Self(mask(kind), PhantomData)
    }
}

const fn mask(kind: u16) -> [u128; 3] {
    let num = kind as usize;
    match num {
        0..=127 => [1u128 << num, 0, 0],
        128..=255 => [0, 1u128 << (num - 128), 0],
        256..=383 => [0, 0, 1u128 << (num - 256)],
        _ => panic!("TokenKind limit exceeded"),
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

#[cfg(test)]
mod tests {
    use biome_rowan::RawSyntaxKind;

    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum TestKind {
        Kind0 = 0,
        Kind127 = 127,
        Kind128 = 128,
        Kind255 = 255,
        Kind256 = 256,
        Kind383 = 383,
    }

    impl SyntaxKind for TestKind {
        const TOMBSTONE: Self = Self::Kind0;
        const EOF: Self = Self::Kind383;

        fn to_raw(&self) -> RawSyntaxKind {
            RawSyntaxKind(*self as u16)
        }

        #[expect(unsafe_code)]
        fn from_raw(raw: RawSyntaxKind) -> Self {
            unsafe { std::mem::transmute::<u16, Self>(raw.0) }
        }

        fn is_root(&self) -> bool {
            false
        }

        fn is_list(&self) -> bool {
            false
        }

        fn is_bogus(&self) -> bool {
            false
        }

        fn is_trivia(self) -> bool {
            false
        }

        fn to_string(&self) -> Option<&'static str> {
            None
        }

        fn to_bogus(&self) -> Self {
            Self::Kind0
        }

        fn is_allowed_before_suppressions(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_mask_first_128() {
        // lower limit test
        let [a, b, c] = mask(0);

        assert!(a.count_ones() == 1);
        assert!(a.trailing_zeros() == 0);
        assert_eq!(b, 0);
        assert_eq!(c, 0);

        // upper limit test
        let [a, b, c] = mask(127);

        assert!(a.count_ones() == 1);
        assert!(a.trailing_zeros() == u128::BITS - 1);
        assert_eq!(b, 0);
        assert_eq!(c, 0);
    }

    #[test]
    fn test_mask_second_128() {
        // lower limit test
        let [a, b, c] = mask(128);

        assert_eq!(a, 0);
        assert!(b.count_ones() == 1);
        assert!(b.trailing_zeros() == 0);
        assert_eq!(c, 0);

        // upper limit test
        let [a, b, c] = mask(255);

        assert_eq!(a, 0);
        assert!(b.count_ones() == 1);
        assert!(b.trailing_zeros() == u128::BITS - 1);
        assert_eq!(c, 0);
    }

    #[test]
    fn test_mask_third_128() {
        // lower limit test
        let [a, b, c] = mask(256);

        assert_eq!(a, 0);
        assert_eq!(b, 0);
        assert!(c.count_ones() == 1);
        assert!(c.trailing_zeros() == 0);

        // upper limit test
        let [a, b, c] = mask(383);

        assert_eq!(a, 0);
        assert_eq!(b, 0);
        assert!(c.count_ones() == 1);
        assert!(c.trailing_zeros() == u128::BITS - 1);
    }

    #[test]
    #[should_panic(expected = "TokenKind limit exceeded")]
    fn test_mask_out_of_range() {
        mask(384);
    }

    #[test]
    fn test_contains() {
        let set: TokenSet<TestKind> = TokenSet::EMPTY;
        assert!(!set.contains(TestKind::Kind0));
        assert!(!set.contains(TestKind::Kind127));

        let set = TokenSet::singleton(TestKind::Kind0)
            .union(TokenSet::singleton(TestKind::Kind128))
            .union(TokenSet::singleton(TestKind::Kind383));

        assert!(set.contains(TestKind::Kind0));
        assert!(set.contains(TestKind::Kind128));
        assert!(set.contains(TestKind::Kind383));

        assert!(!set.contains(TestKind::Kind127));
        assert!(!set.contains(TestKind::Kind255));
        assert!(!set.contains(TestKind::Kind256));

        let set = TokenSet::singleton(TestKind::Kind0)
            .union(TokenSet::singleton(TestKind::Kind127))
            .union(TokenSet::singleton(TestKind::Kind128))
            .union(TokenSet::singleton(TestKind::Kind255))
            .union(TokenSet::singleton(TestKind::Kind256))
            .union(TokenSet::singleton(TestKind::Kind383));

        assert!(set.contains(TestKind::Kind0));
        assert!(set.contains(TestKind::Kind127));
        assert!(set.contains(TestKind::Kind128));
        assert!(set.contains(TestKind::Kind255));
        assert!(set.contains(TestKind::Kind256));
        assert!(set.contains(TestKind::Kind383));
    }
}
