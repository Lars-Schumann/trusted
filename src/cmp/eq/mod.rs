use ::core::cmp::Eq;

use crate::cmp::TrustedPartialEq;

mod impl_core;

#[cfg(feature = "alloc")]
mod impl_alloc;

/// The primary difference to [`TrustedPartialEq`] is the additional requirement for reflexivity. A type
/// that implements [`TrustedPartialEq`] guarantees that for all `a`, `b` and `c`:
///
/// - symmetric: `a == b` implies `b == a` and `a != b` implies `!(a == b)`
/// - transitive: `a == b` and `b == c` implies `a == c`
///
/// `TrustedEq`, which builds on top of [`TrustedPartialEq`] also implies:
///
/// - reflexive: `a == a`
///
/// # Safety
///
/// Violating this property is a safety error.
/// This means that `unsafe` code may rely on the correctness of these methods.
pub unsafe trait TrustedEq: TrustedPartialEq<Self> + Eq {}
