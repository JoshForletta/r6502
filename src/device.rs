use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Range},
};

use crate::core::BitWidth;

pub mod ram;
pub use ram::Ram;

pub trait Device<Address, Data>
where
    Self: Debug + Index<Address, Output = Data> + IndexMut<Address, Output = Data>,
    Address: BitWidth,
    Data: BitWidth,
{
    fn address_range(&self) -> &Range<Address>;

    fn contains(&self, a: Address) -> bool {
        self.address_range().contains(&a)
    }
}
