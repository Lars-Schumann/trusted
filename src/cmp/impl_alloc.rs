use crate::cmp::TrustedEq;
use crate::cmp::TrustedPartialEq;

use ::alloc::collections::LinkedList;
use ::alloc::collections::VecDeque;
use ::alloc::vec::Vec;

unsafe impl<T1: TrustedPartialEq<T2>, T2> TrustedPartialEq<Vec<T2>> for Vec<T1> {}
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for VecDeque<T> {} // we cannot do what Vec does, since VecDQ is missing the corresponding PartialEq impl
unsafe impl<T: TrustedPartialEq> TrustedPartialEq for LinkedList<T> {}

unsafe impl<T: TrustedEq> TrustedEq for Vec<T> {}
unsafe impl<T: TrustedEq> TrustedEq for VecDeque<T> {}
unsafe impl<T: TrustedEq> TrustedEq for LinkedList<T> {}
