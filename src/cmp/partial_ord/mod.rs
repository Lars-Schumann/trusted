use ::core::cmp::PartialOrd;
use ::core::marker::Sized;

use crate::cmp::TrustedPartialEq;

mod impl_core;

#[cfg(feature = "alloc")]
mod impl_alloc;

/// # Safety
///
/// TODO
pub unsafe trait TrustedPartialOrd<Rhs: ?Sized = Self>: TrustedPartialEq<Rhs> + PartialOrd<Rhs> {}
