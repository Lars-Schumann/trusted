use ::core::cmp::Ord;

use crate::cmp::TrustedEq;
use crate::cmp::TrustedPartialOrd;

mod impl_core;

#[cfg(feature = "alloc")]
mod impl_alloc;

/// # Safety
///
/// TODO
pub unsafe trait TrustedOrd: TrustedEq + TrustedPartialOrd<Self> + Ord {}
