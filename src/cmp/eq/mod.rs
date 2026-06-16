use crate::cmp::TrustedPartialEq;
use ::core::cmp::Eq;

mod impl_core;

#[cfg(feature = "alloc")]
mod impl_alloc;

/// # Safety
///
/// TODO
pub unsafe trait TrustedEq: TrustedPartialEq<Self> + Eq {}
