/// # Safety
///
/// TODO
pub unsafe trait TrustedPartialEq<Rhs = Self>: PartialEq<Rhs> {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedEq: Eq {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedPartialOrd<Rhs = Self>: PartialOrd<Rhs> {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedOrd: Ord {}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedPartialEq for [
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,

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

        ::core::primitive::f32,
        ::core::primitive::f64,
    ]
}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedEq for [
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,

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

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedPartialOrd for [
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,

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

        ::core::primitive::f32,
        ::core::primitive::f64,
    ]
}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedOrd for [
        ::core::primitive::u8,
        ::core::primitive::u16,
        ::core::primitive::u32,
        ::core::primitive::u64,
        ::core::primitive::u128,
        ::core::primitive::usize,

        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
        ::core::primitive::i128,
        ::core::primitive::isize,

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
