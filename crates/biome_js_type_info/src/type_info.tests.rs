use super::*;

#[test]
#[cfg(target_pointer_width = "64")]
fn verify_type_sizes() {
    assert_eq!(
        std::mem::size_of::<Type>(),
        8,
        "`Type` should not be bigger than 8 bytes"
    );
    assert_eq!(
        std::mem::size_of::<TypeInner>(),
        16,
        "`TypeInner` should not be bigger than 16 bytes"
    );
}
