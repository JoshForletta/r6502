use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Range},
};

use crate::device::Device;

#[derive(Default)]
pub struct Ram {
    address_range: Range<u16>,
    m: Vec<u8>,
}

impl Debug for Ram {
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

impl Ram {
    pub fn with_address_range(start: u16, end: u16) -> Self {
        Self {
            address_range: Range { start, end },
            m: vec![u8::from(0); (end - start).into()],
        }
    }
}

impl Index<u16> for Ram {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        self.m
            .index(Into::<usize>::into(index - self.address_range.start))
    }
}

impl IndexMut<u16> for Ram {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.m.index_mut(Into::<usize>::into(index))
    }
}

impl Device for Ram {
    fn address_range(&self) -> &std::ops::Range<u16> {
        &self.address_range
    }
}
