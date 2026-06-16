use crate::cmp::TrustedEq;
use crate::cmp::TrustedOrd;
use crate::cmp::TrustedPartialEq;
use crate::cmp::TrustedPartialOrd;

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl crate::cmp::TrustedPartialEq [for] [
        (),
        ::core::primitive::bool,
        ::core::primitive::char,
        ::core::primitive::f32,
        ::core::primitive::f64,
        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,
        ::core::primitive::str,
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::num::NonZeroU8,
        ::core::num::NonZeroU16,
        ::core::num::NonZeroU32,
        ::core::num::NonZeroU64,
        ::core::num::NonZeroU128,
        ::core::num::NonZeroUsize,
        ::core::num::NonZeroI8,
        ::core::num::NonZeroI16,
        ::core::num::NonZeroI32,
        ::core::num::NonZeroI64,
        ::core::num::NonZeroI128,
        ::core::num::NonZeroIsize,
    ]
}

unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T,) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for (T, T, T, T, T, T, T, T, T, T, T, T) {}

unsafe impl<T: TrustedPartialEq, const N: usize> TrustedPartialEq for [T; N] {}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedEq [for] [
        (),
        ::core::primitive::bool,
        ::core::primitive::char,
        // ::core::primitive::f32,
        // ::core::primitive::f64,
        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,
        ::core::primitive::str,
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::num::NonZeroU8,
        ::core::num::NonZeroU16,
        ::core::num::NonZeroU32,
        ::core::num::NonZeroU64,
        ::core::num::NonZeroU128,
        ::core::num::NonZeroUsize,
        ::core::num::NonZeroI8,
        ::core::num::NonZeroI16,
        ::core::num::NonZeroI32,
        ::core::num::NonZeroI64,
        ::core::num::NonZeroI128,
        ::core::num::NonZeroIsize,
    ]
}

unsafe impl<T: TrustedEq> TrustedEq for (T,) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedEq> TrustedEq for (T, T, T, T, T, T, T, T, T, T, T, T) {}

unsafe impl<T: TrustedEq, const N: usize> TrustedEq for [T; N] {}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl crate::cmp::TrustedPartialOrd [for] [
        (),
        ::core::primitive::bool,
        ::core::primitive::char,
        ::core::primitive::f32,
        ::core::primitive::f64,
        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,
        ::core::primitive::str,
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::num::NonZeroU8,
        ::core::num::NonZeroU16,
        ::core::num::NonZeroU32,
        ::core::num::NonZeroU64,
        ::core::num::NonZeroU128,
        ::core::num::NonZeroUsize,
        ::core::num::NonZeroI8,
        ::core::num::NonZeroI16,
        ::core::num::NonZeroI32,
        ::core::num::NonZeroI64,
        ::core::num::NonZeroI128,
        ::core::num::NonZeroIsize,
    ]
}

unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T,) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedPartialOrd> TrustedPartialOrd for (T, T, T, T, T, T, T, T, T, T, T, T) {}

unsafe impl<T: TrustedPartialOrd, const N: usize> TrustedPartialOrd for [T; N] {}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl crate::cmp::TrustedOrd [for] [
        (),
        ::core::primitive::bool,
        ::core::primitive::char,
        // ::core::primitive::f32,
        // ::core::primitive::f64,
        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,
        ::core::primitive::str,
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::num::NonZeroU8,
        ::core::num::NonZeroU16,
        ::core::num::NonZeroU32,
        ::core::num::NonZeroU64,
        ::core::num::NonZeroU128,
        ::core::num::NonZeroUsize,
        ::core::num::NonZeroI8,
        ::core::num::NonZeroI16,
        ::core::num::NonZeroI32,
        ::core::num::NonZeroI64,
        ::core::num::NonZeroI128,
        ::core::num::NonZeroIsize,
    ]
}

unsafe impl<T: TrustedOrd> TrustedOrd for (T,) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T, T, T, T, T, T) {}
unsafe impl<T: TrustedOrd> TrustedOrd for (T, T, T, T, T, T, T, T, T, T, T, T) {}

unsafe impl<T: TrustedOrd, const N: usize> TrustedOrd for [T; N] {}
