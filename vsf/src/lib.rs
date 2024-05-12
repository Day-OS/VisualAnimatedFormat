mod file_structure;
mod bitsize;
mod buffer;

use bitvec::{array::BitArray, order::Msb0};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn copy_chunk(mut src: &mut BitArray<u16, Msb0>, cpy: BitArray<u8, Msb0>, mut index: usize){
    for bit in cpy {
        if src.len() < index {
            return;
        }
        src.set(index, bit);
        index += 1;
    }
}


#[cfg(test)]
mod tests {
    use crate::bitsize::*;

    #[test]
    fn bit_size() {
        let value = BitSize::new(0xFF, BitQ8).to_byte();
        assert_eq!(value, 0b11111111);

        let value = BitSize::new(0xFF, BitQ7).to_byte();
        assert_eq!(value, 0b1111111);

        let value = BitSize::new(0xFF, BitQ6).to_byte();
        assert_eq!(value, 0b111111);

        let value = BitSize::new(0xFF, BitQ5).to_byte();
        assert_eq!(value, 0b11111);

        let value = BitSize::new(0xFF, BitQ4).to_byte();
        assert_eq!(value, 0b1111);

        let value = BitSize::new(0xFF, BitQ3).to_byte();
        assert_eq!(value, 0b111);

        let value = BitSize::new(0xFF, BitQ2).to_byte();
        assert_eq!(value, 0b11);

        let value = BitSize::new(0xFF, BitQ1).to_byte();
        assert_eq!(value, 0b1);
    }


    #[test]
    fn bit_size_primitives() {
        let value: u8 = 0;
        assert_eq!(value.get_bit_quantity(), 8);
        let value: u16 = 0;
        assert_eq!(value.get_bit_quantity(), 16);
        let value: u32 = 0;
        assert_eq!(value.get_bit_quantity(), 32);
        let value: u64 = 0;
        assert_eq!(value.get_bit_quantity(), 64);
        let value: u128 = 0;
        assert_eq!(value.get_bit_quantity(), 128);
    }
}
