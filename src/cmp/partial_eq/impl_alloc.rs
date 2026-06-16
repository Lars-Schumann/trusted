use super::TrustedPartialEq;

use ::alloc::collections::LinkedList;
use ::alloc::collections::VecDeque;
use ::alloc::vec::Vec;

unsafe impl<T: TrustedPartialEq<U>, U> TrustedPartialEq<Vec<U>> for Vec<T> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for VecDeque<T> {} // we cannot do what Vec does, since VecDQ is missing the corresponding PartialEq impl
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for LinkedList<T> {}
