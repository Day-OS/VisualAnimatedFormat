use std::fmt::Debug;

use bitvec::{array::BitArray, order::Msb0};

pub trait BitQuantity : Debug {
    #[allow(dead_code)]
    fn get_bit_quantity(&self) -> usize;
}

macro_rules! impl_bitQuantity {
    ($struct:ident, $size:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $struct;

        impl BitQuantity for $struct {
            fn get_bit_quantity(&self) -> usize {
                let value: usize = $size;
                return value;
            }
        }
    };
}

macro_rules! impl_bitQuantity_primitives {
    (for $($t:ty),+) => {
        $(impl BitQuantity for $t {
            fn get_bit_quantity(&self) -> usize {
                std::mem::size_of::<$t>() * 8
            }
        })*
    }
}

impl_bitQuantity_primitives!(for u8, u16, u32, u64, u128);
impl_bitQuantity!(BitQ1, 1);
impl_bitQuantity!(BitQ2, 2);
impl_bitQuantity!(BitQ3, 3);
impl_bitQuantity!(BitQ4, 4);
impl_bitQuantity!(BitQ5, 5);
impl_bitQuantity!(BitQ6, 6);
impl_bitQuantity!(BitQ7, 7);
impl_bitQuantity!(BitQ8, 8);

#[allow(dead_code)]
#[derive(Debug)]
pub struct BitSize<Q>(pub BitArray<u8, Msb0>, pub Q)
where
    Q: BitQuantity;

impl<BitQuantity> BitSize<BitQuantity>
where
    BitQuantity: self::BitQuantity,
{
    #[allow(dead_code)]
    pub fn new(value: u8, bit_quantity: BitQuantity) -> Self {
        let mut value = BitArray::<u8, Msb0>::new(value);
        let quantity: usize = bit_quantity.get_bit_quantity();
        value.shift_left(8-quantity);

        Self(value, bit_quantity)
    }
    #[allow(dead_code)]
    pub fn to_byte(&mut self) -> u8 {
        let bitarr = self.0;
        let quantity: usize = self.1.get_bit_quantity();
        self.0.shift_right(8-quantity);
        bitarr.data
    }
}
impl<Bq> BitQuantity for BitSize<Bq>
where
    Bq: self::BitQuantity,
{
    fn get_bit_quantity(&self) -> usize {
        self.1.get_bit_quantity()
    }
}
