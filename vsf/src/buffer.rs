use bitvec::{array::BitArray, order::Msb0};

use crate::{bitsize::{BitQ1, BitQ16, BitQ8, BitQuantity, BitSize}, errors::BufferError};


///
#[derive(Clone)]
pub struct Buffer{
    pub body: Vec<BitArray<u8, Msb0>>, 
    pub bit_head: u8,
    pub byte_head: u8
}

impl Buffer {
    pub fn to_byte_vec(&self) -> Vec<u8> {
        let _vec = self.body.clone();
        _vec.iter().map(|bitarr| bitarr.data).collect()
    }

    pub fn append_chars(&mut self, chars: Vec<u8>){

        for byte in chars {
            let bit_size = BitSize::new(byte.into(), BitQ8);
            self.append_bitsize(bit_size);
        }
    }
    
    pub fn append_bitsize<Q>(&mut self, bit_size: BitSize<Q>) -> Result<(), BufferError> where  Q: BitQuantity{
        let quantity = bit_size.1.get_bit_quantity();
        let vec = &mut self.body;
        let bit_head = &mut self.bit_head;
        let byte_head = &mut self.byte_head;
 
        if vec.len() <= 0 {
            vec.push(BitArray::new(0));
        }

        for i in 0..quantity {
            // if head skips 8 bits
            if *bit_head >= 8 {
                *bit_head = 0;
                *byte_head += 1;
                vec.push(BitArray::new(0));
            }
            
            let byte = bit_size.0.get(i/8).ok_or(BufferError::IndexOutOfBound)?;

            let bit = byte.get(i%8).ok_or(BufferError::IndexOutOfBound)?;

            vec[byte_head.clone() as usize].set((*bit_head).into(), *bit);
            *bit_head += 1;
        }
        Ok(())
    }

    pub fn read_bits<Q>(&mut self, size: Q) -> Result<BitSize<Q>, BufferError> where  Q: BitQuantity{
        let quantity = size.get_bit_quantity();
        let vec = &mut self.body;
        let bit_head = &mut self.bit_head;
        let byte_head = &mut self.byte_head;

        let mut bit_size = BitSize::new(0, size);

        let mut last_byte = vec.get(*byte_head as usize).ok_or(BufferError::IndexOutOfBound)?;
        for i in 0..quantity {
            // if head skips 8 bits
            if *bit_head >= 8 {
                *bit_head = 0;
                *byte_head += 1;
                last_byte = vec.get(*byte_head as usize).ok_or(BufferError::IndexOutOfBound)?;
            }

            let bit = last_byte.get(*bit_head as usize);

            if bit.is_none() {
                return Err(BufferError::IndexOutOfBound);
            }
            
            let byte = bit_size.0.get_mut(i/8).ok_or(BufferError::IndexOutOfBound)?;
            byte.set(i%8, *bit.ok_or(BufferError::IndexOutOfBound)?);
            *bit_head += 1;
        }
        Ok(bit_size)
    }

    pub fn read_chars(&mut self, size: usize) -> Result<String, BufferError>{
        let mut vec = vec![];
        for _ in 0..size {
            let byte = self.read_bits(BitQ8)?;
            let byte = byte.0.get(0).ok_or(BufferError::IndexOutOfBound)?;
            vec.push(byte.data);
        }
        Ok(String::from_utf8(vec).map_err(|_| BufferError::StringConversionFailed)?)
    }

    pub fn read_u16(&mut self) -> Result<u16, BufferError>{
        let bytes = self.read_bits(BitQ16)?;
        let vec: Vec<u8> = bytes.0.iter().map(|byte| {
            byte.data
        }).collect();
        let result = u16::from_le_bytes([*vec.get(0).ok_or(BufferError::IndexOutOfBound)?, *vec.get(1).ok_or(BufferError::IndexOutOfBound)?]);
        Ok(result)
    }

    pub fn read_bool(&mut self) -> Result<bool, BufferError>{
        let bytes = self.read_bits(BitQ1)?;
        let byte = bytes.0.get(0).ok_or(BufferError::IndexOutOfBound)?;
        let result = *byte.get(0).ok_or(BufferError::IndexOutOfBound)?;
        Ok(result)
    }

    pub fn read_u8(&mut self) -> Result<u8, BufferError>{
        let bytes = self.read_bits(BitQ8)?;
        let byte = bytes.0.get(0).ok_or(BufferError::IndexOutOfBound)?;
        let result = byte.data;
        Ok(result)
    }



}
