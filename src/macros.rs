macro_rules! unsafe_impl_trait_for_types {
    (unsafe impl $trait:path [for] [$($ty:ty,)*]) => {$(
        unsafe impl $trait for $ty {}
    )*};
}
pub(crate) use unsafe_impl_trait_for_types;
