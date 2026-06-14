crate::macros::unsafe_impl_trait_for_types! {
    unsafe impl crate::cmp::TrustedPartialEq [for] [
        {T: crate::cmp::TrustedPartialEq} ::alloc::collections::vec_deque::VecDeque<T>,

        {T: crate::cmp::TrustedPartialEq} ::alloc::vec::Vec<T>,
    ]
}
