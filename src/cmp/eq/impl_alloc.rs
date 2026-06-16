use super::TrustedEq;

use ::alloc::collections::LinkedList;
use ::alloc::collections::VecDeque;
use ::alloc::vec::Vec;

unsafe impl<T: TrustedEq> TrustedEq for Vec<T> {}
unsafe impl<T: TrustedEq> TrustedEq for VecDeque<T> {}
unsafe impl<T: TrustedEq> TrustedEq for LinkedList<T> {}
