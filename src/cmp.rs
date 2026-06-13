use ::core::cmp::Eq;
use ::core::cmp::Ord;
use ::core::cmp::PartialEq;
use ::core::cmp::PartialOrd;
use ::core::marker::Sized;

/// Implementations must ensure that `<Self as PartialEq<Rhs>>::eq` and `<Self as PartialEq<Rhs>>::ne` are consistent with each other:
///
/// - `a != b` if and only if `!(a == b)`.
///
/// If [`TrustedPartialOrd`] or [`TrustedOrd`] are also implemented for `Self` and `Rhs`, their methods must also
/// be consistent with `PartialEq` (see the documentation of those traits for the exact requirements).
///
/// The equality relation `==` must satisfy the following conditions
/// (for all `a`, `b`, `c` of type `A`, `B`, `C`):
///
/// - **Symmetry**: if `A: PartialEq<B>` and `B: PartialEq<A>`, then **`a == b` implies `b == a`**; and
///
/// - **Transitivity**: if `A: PartialEq<B>` and `B: PartialEq<C>` and `A:
///   PartialEq<C>`, then **`a == b` and `b == c` implies `a == c`**.
///   This must also work for longer chains, such as when `A: PartialEq<B>`, `B: PartialEq<C>`,
///   `C: PartialEq<D>`, and `A: PartialEq<D>` all exist.
///
/// Note that the `B: PartialEq<A>` (symmetric) and `A: PartialEq<C>`
/// (transitive) impls are not forced to exist, but these requirements apply
/// whenever they do exist.
///
/// ## Cross-crate considerations
///
/// You are never allowed to implement this trait for a foreign type.
/// In other words, you may do `unsafe impl TrustedPartialEq<ForeignType> for LocalType`, but
/// should *never* do `unsafe impl TrustedPartialEq<LocalType> for ForeignType`.
///
/// # Safety
///
/// Violating these requirements is a safety error.
/// This means that `unsafe` code may rely on the correctness of these methods.
pub unsafe trait TrustedPartialEq<Rhs: ?Sized = Self>: PartialEq<Rhs> {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedEq: TrustedPartialEq<Self> + Eq {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedPartialOrd<Rhs: ?Sized = Self>: TrustedPartialEq<Rhs> + PartialOrd<Rhs> {}

/// # Safety
///
/// TODO
pub unsafe trait TrustedOrd: TrustedPartialOrd<Self> + Ord {}

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedPartialEq for [

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

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedEq for [

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

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedPartialOrd for [

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

// SAFETY: we trust std
crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl TrustedOrd for [

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

unsafe impl<T: TrustedPartialEq, const N: usize> TrustedPartialEq for [T; N] {}

#[cfg(feature = "alloc")]
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for ::alloc::vec::Vec<T> {}
