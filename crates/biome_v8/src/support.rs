//! Utility traits copied from [v8] as they are not exposed to the public API

use std::{marker::PhantomData, mem::size_of};

pub trait UnitType
where
    Self: Copy + Sized,
{
    #[inline(always)]
    fn get() -> Self {
        UnitValue::<Self>::get()
    }
}

impl<T> UnitType for T where T: Copy + Sized {}

#[derive(Copy, Clone, Debug)]
struct UnitValue<T>(PhantomData<T>)
where
    Self: Sized;

impl<T> UnitValue<T>
where
    Self: Copy + Sized,
{
    const SELF: Self = Self::new_checked();

    const fn new_checked() -> Self {
        // Statically assert that T is indeed a unit type.
        let size_must_be_0 = size_of::<T>();
        let s = Self(PhantomData::<T>);
        [s][size_must_be_0]
    }

    #[inline(always)]
    fn get_checked(self) -> T {
        // This run-time check serves just as a backup for the compile-time
        // check when Self::SELF is initialized.
        assert_eq!(size_of::<T>(), 0);
        unsafe { std::mem::MaybeUninit::<T>::zeroed().assume_init() }
    }

    #[inline(always)]
    pub fn get() -> T {
        // Accessing the Self::SELF is necessary to make the compile-time type check
        // work.
        Self::SELF.get_checked()
    }
}
