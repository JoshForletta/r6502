use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Range},
};

use crate::{core::BitWidth, device::Device};

#[derive(Default)]
pub struct Ram<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    address_range: Range<Address>,
    m: Vec<Data>,
}

impl<Address, Data> Debug for Ram<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ram")
            .field(
                "address_range",
                &format_args!(
                    "${:X}..${:X}",
                    self.address_range.start, self.address_range.end
                ),
            )
            .finish()
    }
}

impl<Address, Data> Ram<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    pub fn with_address_range(start: Address, end: Address) -> Self {
        Self {
            address_range: Range { start, end },
            m: vec![Data::from(0); (end - start).into()],
        }
    }
}

impl<Address, Data> Index<Address> for Ram<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    type Output = Data;
    fn index(&self, index: Address) -> &Self::Output {
        self.m.index((index - self.address_range.start).into())
    }
}

impl<Address, Data> IndexMut<Address> for Ram<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        self.m.index_mut(index.into())
    }
}

impl<Address, Data> Device<Address, Data> for Ram<Address, Data>
where
    Address: BitWidth,
    Data: BitWidth,
{
    fn address_range(&self) -> &std::ops::Range<Address> {
        &self.address_range
    }
}
