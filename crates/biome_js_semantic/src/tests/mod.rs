mod assertions;
pub mod declarations;
mod format;
mod functions;
mod infer;
mod references;
mod scopes;

#[macro_export]
macro_rules! assert_semantics {
    ($(#[$attr:meta])* $($name:ident, $code:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::tests::assertions::assert($code, stringify!($name));
            }
        )*
    };
}
