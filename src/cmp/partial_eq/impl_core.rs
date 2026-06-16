use super::TrustedPartialEq;

crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedPartialEq [for] [
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

// TODO: make these refer to ::core::range::legacy once that is stable
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::ops::Range<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::ops::RangeFrom<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::ops::RangeInclusive<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::ops::RangeToInclusive<T> {}

unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::range::Range<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::range::RangeFrom<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::range::RangeInclusive<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::core::range::RangeToInclusive<T> {}
