use std::fmt::Debug;
use paste::paste;

use bitvec::{array::BitArray, order::Msb0};

pub trait BitQuantity : Debug {
    #[allow(dead_code)]
    fn get_bit_quantity(&self) -> usize;
}

/// Generates N BitQ(N)s
macro_rules! impl_bitQuantity {
    ($size:expr) => {
        paste!{
            #[derive(Debug, Clone, Copy)]
            pub struct [<BitQ $size>];

            impl BitQuantity for [<BitQ $size>] {
                fn get_bit_quantity(&self) -> usize {
                    let value: usize = $size;
                    return value;
                }
            }
        }
    };
}

macro_rules! impl_bitQuantityList {
    ($($size:expr), +) => {
        $(
            impl_bitQuantity!($size);
        )*
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

impl_bitQuantity_primitives!(for u8, u16, u32);
impl_bitQuantityList!(1,2,3,4,5,6,7,8);
impl_bitQuantityList!(9,10,11,12,13,14,15,16);
impl_bitQuantityList!(17,18,19,20,21,22,23,24);

#[allow(dead_code)]
#[derive(Debug)]
pub struct BitSize<Q>(pub Vec<BitArray<u8, Msb0>>, pub Q)
where
    Q: BitQuantity;

impl<BitQuantity> BitSize<BitQuantity>
where
    BitQuantity: self::BitQuantity,
{
    #[allow(dead_code)]
    pub fn new(value: u32, bit_quantity: BitQuantity) -> Self {
        let mut bytes = vec![];
        let value = value.to_le_bytes();
        let quantity: usize = bit_quantity.get_bit_quantity();
        let full_bytes_quantity = quantity/8;
        
        //Adds all full bytes
        for index in 0..full_bytes_quantity {
            bytes.push(BitArray::<u8, Msb0>::new(value[index]))
        }

        //In case the value does not fit into 8*n bits (Most cases)
        if quantity % 8 != 0 {
            let mut value = BitArray::<u8, Msb0>::new(value[full_bytes_quantity]);
            value.shift_left(8 - (quantity%8));
            bytes.push(value)
        }




        Self(bytes, bit_quantity)
    }
    #[allow(dead_code)]
    pub fn to_byte(&mut self) -> Vec<u8> {
        let bitarr = &self.0;
        let mut bytes = vec![];
        let quantity: usize = self.1.get_bit_quantity();
        let full_bytes_quantity = quantity/8;

        for index in 0..full_bytes_quantity {
            bytes.push(bitarr[index].data);
        }

        if quantity % 8 != 0 {
            let mut value = bitarr[full_bytes_quantity];

            value.shift_right(8 - (quantity%8));
            bytes.push(value.data)
        }

        return bytes;
    }

    pub fn to_u32(&mut self) -> u32{
        let mut result: [u8; 4] = [0, 0, 0, 0];
        let mut bytes: Vec<u8> = self.to_byte();

        for index in 0..4 {
            if let Some(value ) = bytes.get(index){
                result[index] = value.clone();
                
            }
        }
        u32::from_le_bytes(result)
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
