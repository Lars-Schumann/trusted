use ::core::cmp::Eq;

use crate::cmp::TrustedPartialEq;

mod impl_core;

#[cfg(feature = "alloc")]
mod impl_alloc;

/// # Safety
///
/// TODO
pub unsafe trait TrustedEq: TrustedPartialEq<Self> + Eq {}
