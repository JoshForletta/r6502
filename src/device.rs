use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Range},
};

pub mod ram;
pub use ram::Ram;

pub trait Device
where
    Self: Debug + Index<u16, Output = u8> + IndexMut<u16, Output = u8>,
{
    fn address_range(&self) -> &Range<u16>;

    #[inline]
    fn contains(&self, a: u16) -> bool {
        self.address_range().contains(&a)
    }
}
