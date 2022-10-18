use std::fmt::{Debug, UpperHex};

use num_traits::Unsigned;

pub trait BitWidth
where
    Self: Debug + Default + Copy + PartialOrd + Unsigned + From<u8> + Into<usize> + UpperHex,
{
}

impl<T: Unsigned> BitWidth for T where
    Self: Debug + Default + Copy + PartialOrd + Unsigned + From<u8> + Into<usize> + UpperHex
{
}
