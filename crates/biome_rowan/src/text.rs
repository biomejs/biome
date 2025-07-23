use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::{slice, str};

use crate::TokenText;

/// Type used for storing text from a syntax tree that avoids heap-allocations
/// when possible.
///
/// This is somewhat similar to [std::borrow::Cow], except it's optimised for
/// token strings.
pub struct Text(TextRepr);

// SAFETY: `Text` is immutable, and is safe to `Send`.
#[expect(unsafe_code)]
unsafe impl Send for Text {}

// SAFETY: `Text` is immutable, and is `Sync` safe.
#[expect(unsafe_code)]
unsafe impl Sync for Text {}

union TextRepr {
    token: ManuallyDrop<TokenText>,
    string: ManuallyDrop<SmallStr>,
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self.text()
    }
}

impl Clone for Text {
    #[expect(unsafe_code)]
    fn clone(&self) -> Self {
        if self.is_string() {
            // SAFETY: We checked it's a string.
            unsafe {
                Self(TextRepr {
                    string: self.0.string.clone(),
                })
            }
        } else {
            // SAFETY: If it's not a string, it must be a token.
            unsafe {
                Self(TextRepr {
                    token: self.0.token.clone(),
                })
            }
        }
    }
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.text())
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new_static("")
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl Drop for Text {
    #[expect(unsafe_code)]
    fn drop(&mut self) {
        if self.is_string() {
            // SAFETY: We checked it's a string.
            unsafe {
                ManuallyDrop::drop(&mut self.0.string);
            }
        } else {
            // SAFETY: If it's not a string, it must be a token.
            unsafe {
                ManuallyDrop::drop(&mut self.0.token);
            }
        }
    }
}

impl Eq for Text {}

impl From<TokenText> for Text {
    fn from(text: TokenText) -> Self {
        Self::new_token(text)
    }
}

impl From<String> for Text {
    fn from(string: String) -> Self {
        Self::new_owned(string.into())
    }
}

impl From<&'static str> for Text {
    fn from(string: &'static str) -> Self {
        Self::new_static(string)
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text().cmp(other.text())
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl PartialEq<&'_ str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self.text() == *other
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Text> for String {
    #[inline]
    #[expect(unsafe_code)]
    fn from(mut value: Text) -> Self {
        if value.is_string() {
            // SAFETY: We checked it's a string.
            let string = unsafe { ManuallyDrop::take(&mut value.0.string) };
            mem::forget(value);
            string.into_boxed_str().into()
        } else {
            // SAFETY: If it's not a string, it must be a token.
            unsafe { value.0.token.to_string() }
        }
    }
}

impl From<Text> for Box<str> {
    #[inline]
    fn from(value: Text) -> Self {
        Self::from(value.text())
    }
}

impl Text {
    #[inline]
    pub fn new_owned(string: Box<str>) -> Self {
        Self(TextRepr {
            string: ManuallyDrop::new(SmallStr::new_owned(string)),
        })
    }

    #[inline]
    pub fn new_static(string: &'static str) -> Self {
        Self(TextRepr {
            string: ManuallyDrop::new(SmallStr::new_static(string)),
        })
    }

    #[inline]
    pub fn new_token(token: TokenText) -> Self {
        Self(TextRepr {
            token: ManuallyDrop::new(token),
        })
    }

    #[inline]
    #[expect(unsafe_code)]
    pub fn is_string(&self) -> bool {
        // SAFETY: See [`SmallStr::tag`] for details.
        unsafe { self.0.string.tag == SmallStr::TAG }
    }

    #[inline]
    #[expect(unsafe_code)]
    pub fn text(&self) -> &str {
        if self.is_string() {
            // SAFETY: We checked it's a string.
            unsafe { self.0.string.as_str() }
        } else {
            // SAFETY: If it's not a string, it must be a token.
            unsafe { self.0.token.text() }
        }
    }
}

/// Small string whose maximum length cannot exceed 2GB.
///
/// Functions the same as `Cow<str>` but is designed to still have an niche
/// optimisation bit so it can fit inside `Text` while keeping `Text` at 16
/// bytes only.
#[derive(Debug)]
#[repr(C)]
struct SmallStr {
    /// Pointer to the string data.
    ///
    /// We use `NonNull` to expose a niche optimisation bit available.
    ptr: ptr::NonNull<u8>,

    /// The length of the string, with the first bit used for marking ownership.
    ///
    /// If `flag_and_len & OWNED_FLAG != 0`, the string data pointed to by `ptr`
    /// is owned and must be freed on drop. Otherwise it's a `&'static str`.
    flag_and_len: u32,

    /// Tag that must have the value [Self::TAG] for the `SmallStr` to be valid.
    ///
    /// If the tag contains any other value, we know `TextRepr` union does not
    /// contain a `SmallStr`, but a `TokenText` instead. This works because in
    /// the same position as the tag, `TokenText` stores the `end` field of its
    /// `TextRange`. And because the upper bit of `TextSize` can never be set,
    /// we can use it to represent this tag.
    tag: u32,
}

impl SmallStr {
    const LEN_MASK: u32 = 0x7fff_ffff; // 31 bits for length.
    const OWNED_FLAG: u32 = 0x8000_0000; // 1 bit for ownership.
    const TAG: u32 = 0x8000_0000; // 1 bit for ownership.

    #[inline]
    fn new_owned(string: Box<str>) -> Self {
        let len = string.len() as u32;
        debug_assert!(len <= Self::LEN_MASK, "string too long (>2GB)");
        let ptr = NonNull::from(Box::leak(string)).cast();
        Self {
            ptr,
            flag_and_len: len | Self::OWNED_FLAG,
            tag: Self::TAG,
        }
    }

    #[inline]
    fn new_static(string: &'static str) -> Self {
        let len = string.len() as u32;
        debug_assert!(len <= Self::LEN_MASK, "string too long (>2GB)");
        let ptr = NonNull::from(string).cast();
        Self {
            ptr,
            flag_and_len: len, // OWNED_FLAG = 0
            tag: Self::TAG,
        }
    }

    #[inline]
    #[expect(unsafe_code)]
    fn as_str(&self) -> &str {
        // SAFETY: `SmallStr` only gets initialised from valid strings, so we
        //         know this is UTF-8. For the same reason, we know the `ptr` is
        //         valid too.
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.ptr.as_ptr(), self.len())) }
    }

    /// Converts the small string into a `Box<str>`.
    ///
    /// Allocates if the string was static.
    #[inline]
    #[expect(unsafe_code)]
    fn into_boxed_str(self) -> Box<str> {
        if self.is_owned() {
            let slice = ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len());
            mem::forget(self);
            // SAFETY: We checked this is an owned instance, so we can safely
            //         transfer the ownership into a `Box`. We also know the
            //         data is valid UTF-8, because `SmallStr` only gets
            //         initialised from valid strings. We also made sure to
            //         "forget" `self` to avoid a double-free.
            unsafe { str::from_boxed_utf8_unchecked(Box::from_raw(slice)) }
        } else {
            let slice = self.as_str();
            slice.into()
        }
    }

    #[inline]
    fn is_owned(&self) -> bool {
        self.flag_and_len & Self::OWNED_FLAG == Self::OWNED_FLAG
    }

    #[inline]
    fn len(&self) -> usize {
        (self.flag_and_len & Self::LEN_MASK) as usize
    }
}

impl Clone for SmallStr {
    fn clone(&self) -> Self {
        if self.is_owned() {
            let slice = self.as_str();
            Self::new_owned(slice.into())
        } else {
            Self {
                ptr: self.ptr,
                flag_and_len: self.flag_and_len,
                tag: Self::TAG,
            }
        }
    }
}

impl Drop for SmallStr {
    #[inline]
    #[expect(unsafe_code)]
    fn drop(&mut self) {
        if self.is_owned() {
            // SAFETY: `SmallStr` only gets initialised from valid strings, so
            //         we know the `ptr` and its length are valid.
            drop(unsafe {
                Box::from_raw(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len()))
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_type_size() {
        assert_eq!(
            std::mem::size_of::<Text>(),
            16,
            "`Text` should not be bigger than 16 bytes"
        );
    }
}
