pub trait IsEven {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

macro_rules! impl_is_even {
    ($($t:ty),*) => {
        $(
            impl IsEven for $t {
                fn is_even(&self) -> bool {
                    (self & 1) == 0
                }

                fn is_odd(&self) -> bool {
                    (self & 1) == 1
                }
            }
        )*
    }
}

impl_is_even!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);