macro_rules! unsafe_impl_trait_for_types {
    (unsafe impl $trait:path [for] [ $( $({$($generics:tt)*})? $ty:ty, )*]) => {$(
        unsafe impl<$($($generics)*)?> $trait for $ty {}
    )*};
}
pub(crate) use unsafe_impl_trait_for_types;
