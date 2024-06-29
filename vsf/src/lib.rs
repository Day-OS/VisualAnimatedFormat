mod bitsize;
mod buffer;
mod errors;
mod file_structure;
mod writer;

use bitvec::{array::BitArray, order::Msb0};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn copy_chunk(mut src: &mut BitArray<u16, Msb0>, cpy: BitArray<u8, Msb0>, mut index: usize) {
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
    use bitvec::{array::BitArray, BitArr};

    use crate::{
        bitsize::*,
        file_structure::{Chunk, Color, FileStructure, Frame, OperationTypes},
        writer,
    };

    #[test]
    fn bit_size() {
        let value = BitSize::new(0xFF, BitQ8).to_byte();
        assert_eq!(value, 0b11111111);

        let value = BitSize::new(0xFF, BitQ7).to_byte();
        assert_eq!(value, 0b11111110);

        let value = BitSize::new(0xFF, BitQ6).to_byte();
        assert_eq!(value, 0b11111100);

        let value = BitSize::new(0xFF, BitQ5).to_byte();
        assert_eq!(value, 0b11111000);

        let value = BitSize::new(0xFF, BitQ4).to_byte();
        assert_eq!(value, 0b11110000);

        let value = BitSize::new(0xFF, BitQ3).to_byte();
        assert_eq!(value, 0b11100000);

        let value = BitSize::new(0xFF, BitQ2).to_byte();
        assert_eq!(value, 0b11000000);

        let value = BitSize::new(0xFF, BitQ1).to_byte();
        assert_eq!(value, 0b10000000);
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

    #[test]
    fn write_file() {
        let file: FileStructure = FileStructure {
            width: 21,
            height: 1,
            has_alpha_channel: false,
            chunks_x: BitSize(BitArray::new(0), BitQ4),
            chunks_y: BitSize(BitArray::new(0), BitQ4),
            colors_quantity: BitSize(BitArray::new(4), BitQ5),
            pallete: vec![
                Color {
                    r: 0xFF,
                    g: 0x00,
                    b: 0x95,
                    a: Some(0xFF),
                },
                Color {
                    r: 0x00,
                    g: 0xB9,
                    b: 0xF2,
                    a: Some(0xFF),
                },
                Color {
                    r: 0xFA,
                    g: 0xD5,
                    b: 0x00,
                    a: Some(0xFF),
                },
                Color {
                    r: 0x00,
                    g: 0x00,
                    b: 0x00,
                    a: Some(0x00),
                },
            ],
            frames: vec![Frame {
                chunks: vec![Chunk {
                    index: Box::new(BitSize(BitArray::new(0), BitQ1)),
                    commands: vec![
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(BitArray::new(3), BitQ2)),
                        },
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(BitArray::new(0), BitQ2)),
                        },
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(BitArray::new(1), BitQ2)),
                        },
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(BitArray::new(2), BitQ2)),
                        },
                    ],
                }],
            }],
        };
        let result = writer::write(file).unwrap().to_byte_vec();
        println!("{result:x?}");
    }
}
