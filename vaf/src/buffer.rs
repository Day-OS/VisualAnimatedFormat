use bitvec::{array::BitArray, order::Msb0};

use crate::{
    bitsize::{BitQ1, BitQ16, BitQ8, BitQuantity, BitSize},
    errors::{self, BufferError},
};

///
#[derive(Clone)]
pub struct Buffer {
    pub body: Vec<BitArray<u8, Msb0>>,
    pub bit_head: u8,
    pub byte_head: u32,
}

impl Buffer {
    pub fn to_byte_vec(&self) -> Vec<u8> {
        let _vec = self.body.clone();
        _vec.iter().map(|bitarr| bitarr.data).collect()
    }

    pub fn append_string(&mut self, string: String) {
        let mut chars = string.as_bytes().to_vec();
        chars.push(0);
        self.append_chars(chars)
    }

    pub fn append_chars(&mut self, chars: Vec<u8>) {
        for byte in chars {
            let bit_size = BitSize::new(byte.into(), BitQ8);
            self.append_bitsize(bit_size);
        }
    }

    pub fn append_bitsize<Q>(&mut self, bit_size: BitSize<Q>) -> Result<(), BufferError>
    where
        Q: BitQuantity,
    {
        let buffer = &mut self.body;

        //Create a new byte if buffer is empty
        if buffer.len() <= 0 {
            buffer.push(BitArray::new(0));
        }

        // How many bits the data that will be appended have
        let byte_quantity = bit_size.get_byte_quantity();
        let bit_head = &mut self.bit_head;
        let byte_head = &mut self.byte_head;

        // Read every bit from bit_size and transfers it into our buffer
        for i in 0..byte_quantity {
            // if bit head reached the byte limit (8 bits), start to iterating in the next byte
            if *bit_head >= 8 {
                *bit_head = 0;
                *byte_head += 1;
                buffer.push(BitArray::new(0));
            }

            let byte = bit_size.0.get(i / 8).ok_or(
                errors::BitSizeError::throw_byte_index_out_of_bound(bit_size.clone(), i / 8),
            )?;
            let bit = byte
                .get(i % 8)
                .ok_or(errors::BitSizeError::throw_bit_index_out_of_bound(
                    byte.clone(),
                    i % 8,
                ))?;

            buffer[byte_head.clone() as usize].set((*bit_head).into(), *bit);
            *bit_head += 1;
        }

        Ok(())
    }

    pub fn read_bits<Q>(&mut self, size: Q) -> Result<BitSize<Q>, BufferError>
    where
        Q: BitQuantity,
    {
        let quantity = size.get_bit_quantity();
        let buffer = &mut self.body;
        let bit_head = &mut self.bit_head;
        let byte_head = &mut self.byte_head;

        let mut bit_size = BitSize::new(0, size);

        let mut last_byte =
            buffer
                .get(*byte_head as usize)
                .ok_or(BufferError::BufferIndexOutOfBound {
                    index: *byte_head as usize,
                })?;
        for i in 0..quantity {
            // if bit head reached the byte limit (8 bits), start to iterating in the next byte
            if *bit_head >= 8 {
                *bit_head = 0;
                *byte_head += 1;
                last_byte =
                    buffer
                        .get(*byte_head as usize)
                        .ok_or(BufferError::BufferIndexOutOfBound {
                            index: *byte_head as usize,
                        })?;
            }

            let bit = last_byte.get(*bit_head as usize).ok_or(
                errors::BitSizeError::throw_bit_index_out_of_bound(
                    last_byte.clone(),
                    *bit_head as usize,
                ),
            )?;

            let _byte_err = bit_size.clone();
            let byte = bit_size.0.get_mut(i / 8).ok_or(
                errors::BitSizeError::throw_byte_index_out_of_bound(_byte_err, i / 8),
            )?;
            byte.set(i % 8, *bit);
            *bit_head += 1;
        }
        Ok(bit_size)
    }

    pub fn read_string(&mut self) -> Result<String, BufferError> {
        let mut string = self.read_chars(None)?;
        string = string[0..string.len() - 1].to_string();
        Ok(string)
    }

    /// Size defines if the reading is going to happen between a fixed ammount of time or when it reaches a \0
    pub fn read_chars(&mut self, size: Option<usize>) -> Result<String, BufferError> {
        let mut vec = vec![];

        for _ in 0..size.unwrap_or(((self.body.len() as u32) - self.byte_head) as usize) {
            let byte = self.read_bits(BitQ8)?;
            let byte = byte
                .0
                .get(0)
                .ok_or(errors::BitSizeError::throw_byte_index_out_of_bound(
                    byte.clone(),
                    0,
                ))?;
            vec.push(byte.data);
            if byte.data == 0 {
                break;
            }
        }
        Ok(String::from_utf8(vec).map_err(|_| BufferError::StringConversionFailed)?)
    }

    pub fn read_u16(&mut self) -> Result<u16, BufferError> {
        let bytes = self.read_bits(BitQ16)?;
        let vec: Vec<u8> = bytes.0.iter().map(|byte| byte.data).collect();
        let result = u16::from_le_bytes([
            *vec.get(0)
                .ok_or(errors::BitSizeError::throw_byte_index_out_of_bound(
                    bytes.clone(),
                    0,
                ))?,
            *vec.get(1)
                .ok_or(errors::BitSizeError::throw_byte_index_out_of_bound(
                    bytes.clone(),
                    1,
                ))?,
        ]);
        Ok(result)
    }

    pub fn read_bool(&mut self) -> Result<bool, BufferError> {
        let bytes = self.read_bits(BitQ1)?;
        let byte = bytes
            .0
            .get(0)
            .ok_or(errors::BitSizeError::throw_byte_index_out_of_bound(
                bytes.clone(),
                0,
            ))?;
        let result = *byte
            .get(0)
            .ok_or(errors::BitSizeError::throw_bit_index_out_of_bound(
                byte.clone(),
                0,
            ))?;
        Ok(result)
    }

    pub fn read_u8(&mut self) -> Result<u8, BufferError> {
        let bytes = self.read_bits(BitQ8)?;
        let byte = bytes
            .0
            .get(0)
            .ok_or(errors::BitSizeError::throw_byte_index_out_of_bound(
                bytes.clone(),
                0,
            ))?;
        let result = byte.data;
        Ok(result)
    }
}
