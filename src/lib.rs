#![no_std]

mod macros;

/// # Safety
///
/// TODO
pub unsafe trait TrustedPartialEq<Rhs = Self>: PartialEq<Rhs> {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedEq: Eq {}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedPartialEq for [
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
                 f32, f64,
    ]
}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedEq for [
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
    ]
}
