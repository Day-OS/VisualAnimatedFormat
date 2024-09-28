use dyn_clone::{clone_trait_object, DynClone};
use paste::paste;
use std::{fmt::Debug, u8};

use bitvec::{array::BitArray, order::Msb0};

//fn get_byte_size(number: u32) -> u8{
//    match number {
//        0..256 => 1,
//        0..65536 => 2,
//        0..16777216 => 3,
//        0_u32..=u32::MAX => 4,
//    }
//}

const fn get_byte_size(number: u8) -> u8{
    match number {
        1..=8 => 1,
        9..=16 => 2,
        17..=24 => 3,
        25..=32 => 4,
        33_u8..=u8::MAX | 0 => 0
    }
}



pub trait BitQuantity: Debug + DynClone {
    
    fn new(value: u32) -> Self where Self: Sized;

     /// DO NOT USE
     fn _new(value: u32, bit_quantity: usize, full_bytes_quantity: u8) -> Vec<BitArray<u8, Msb0>> where Self: Sized {
        let mut bytes = vec![];
        let value = value.to_le_bytes();
        let quantity: usize = bit_quantity;

        if full_bytes_quantity == 2 {
            println!("aslçihdiashidahsuiod")
        }

        //Adds all full bytes 
        for index in 0..full_bytes_quantity {
            bytes.push(BitArray::<u8, Msb0>::new(value[index as usize]))
        }

        //In case the value does not fit into 8*n bits (Most cases)
        if quantity % 8 != 0 {
            let mut value_bitarr = BitArray::<u8, Msb0>::new(0);
            value_bitarr.shift_left(8 - (quantity % 8));
            bytes.push(value_bitarr)
        }

        bytes
    }
    
    fn get(&mut self, index: usize) -> Option<&BitArray<u8, Msb0>>;
    fn get_mut(&mut self, index: usize) -> Option<&mut BitArray<u8, Msb0>>;

    /// Gets the capacity of how much bits does this type uses
    fn get_bit_quantity(&self) -> usize;
    
    /// Gets the ammount of bytes needed to be wasted to store this type
    fn get_byte_quantity(&self) -> usize {
        self.get_bit_quantity() / 8
    }

    fn to_bytes(&mut self) -> Vec<u8>;
    
    /// DO NOT USE
    fn _to_bytes(&mut self, buffer: Vec<BitArray<u8, Msb0>>) -> Vec<u8> {
        let mut bytes = vec![];
        let quantity: usize = self.get_bit_quantity();
        let full_bytes_quantity = self.get_byte_quantity();

        for index in 0..full_bytes_quantity {
            bytes.push(buffer[index].data);
        }

        if quantity % 8 != 0 {
            let mut value = buffer[full_bytes_quantity];

            value.shift_right(8 - (quantity % 8));
            bytes.push(value.data)
        }
        return bytes;
    }
    //gets the first byte from a vector of bytes
    fn to_byte(&mut self) -> u8 {
        let bytes = self.to_bytes();
        bytes[0]
    }

    fn to_u32(&mut self) -> u32 {
        let mut result: [u8; 4] = [0, 0, 0, 0];
        let bytes: Vec<u8> = self.to_bytes();

        for index in 0..4 {
            if let Some(value) = bytes.get(index) {
                result[index] = value.clone();
            }
        }
        u32::from_le_bytes(result)
    }
}

clone_trait_object!(BitQuantity);

macro_rules! impl_bitQuantityList {
    ($($size:expr), +) => {
        paste!{
            $(
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub struct [<BitQ $size>](pub [BitArray<u8, Msb0>; get_byte_size($size) as usize]);

                impl BitQuantity for [<BitQ $size>]{
                    fn new(value: u32) -> Self{
                        const BYTE_SIZE: u8 =  get_byte_size($size);
                        let mut byte_vec: Vec<BitArray<u8, Msb0>> = [<BitQ $size>]::_new(value, $size, BYTE_SIZE);
                        //println!("AMOSTRADINHORSRSRS: {value:?} \n {byte_vec:#?}");

                        for _ in 0..(byte_vec.len() - BYTE_SIZE as usize){
                            byte_vec.pop();
                        }
                        let bytes: [BitArray<u8, Msb0>; BYTE_SIZE as usize] = byte_vec.try_into().unwrap_or_else(
                            |v: Vec<BitArray<u8, Msb0>>| panic!("Expected a Vec of length {} but it was {}", BYTE_SIZE, v.len()));
                        Self(bytes )
                    }

                    fn get_bit_quantity(&self) -> usize{
                        $size
                    }
                    fn to_bytes(&mut self) -> Vec<u8> { 
                        self._to_bytes(self.0.try_into().unwrap())
                    }

                    fn get(&mut self, index:usize)-> Option<&BitArray<u8, Msb0>>{
                        self.0.get(index)
                    }
                    fn get_mut(&mut self, index:usize)-> Option<&mut BitArray<u8, Msb0>>{
                        self.0.get_mut(index)
                    }
                }
            )*
        }

    };
}

impl_bitQuantityList!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
);
