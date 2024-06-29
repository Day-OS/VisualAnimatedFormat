use bitvec::{array::BitArray, order::Msb0};

use crate::bitsize::{BitQ8, BitQuantity, BitSize};

#[derive(Clone)]
pub struct Buffer(pub Vec<BitArray<u8, Msb0>>, pub u8);

impl Buffer {
    pub fn to_byte_vec(&self) -> Vec<u8> {
        let _vec = self.0.clone();
        _vec.iter().map(|bitarr| bitarr.data).collect()
    }

    pub fn append_string(&mut self, mut string: String){
        let mut bytes = string.as_bytes();

        for byte in bytes {
            let bit_size = BitSize::new(*byte, BitQ8);
            self.append_bitsize(bit_size);
        }

    }
    
    pub fn append_bitsize<Q>(&mut self, bit_size: BitSize<Q>) where  Q: BitQuantity{
        let quantity = bit_size.1.get_bit_quantity();
        let mut vec = &mut self.0;
        let head = &mut self.1;
        if quantity > 8 {
           return; 
        }
        
        if vec.len() <= 0 {
            vec.push(BitArray::new(0));
        }

        let mut last_byte_index = vec.len() - 1;
        for i in 0..quantity {
            // if head skips 8 bits
            if *head >= 8 {
                *head = 0;
                last_byte_index += 1;
                vec.push(BitArray::new(0));
            }
            let bit = *bit_size.0.get(i).unwrap(); //This unwrapping is temporary

            vec[last_byte_index].set((*head).into(), bit);
            *head += 1;
        }
    }


}
