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
    use bitvec::{array::BitArray, vec, BitArr};

    use crate::{
        bitsize::*,
        file_structure::{Chunk, Color, FileStructure, Frame, OperationTypes}, writer,
        //writer,
    };

    #[test]
    fn bit_size() {
        println!("testing 1 byte!");

        let value = BitSize::new(0xFF, BitQ8).to_u32();
        assert_eq!(value, 0b11111111);

        let value = BitSize::new(0xFF, BitQ7).to_u32();
        assert_eq!(value, 0b1111111);

        let value = BitSize::new(0xFF, BitQ6).to_u32();
        assert_eq!(value, 0b111111);

        let value = BitSize::new(0xFF, BitQ5).to_u32();
        assert_eq!(value, 0b11111);

        let value = BitSize::new(0xFF, BitQ4).to_u32();
        assert_eq!(value, 0b1111);

        let value = BitSize::new(0xFF, BitQ3).to_u32();
        assert_eq!(value, 0b111);

        let value = BitSize::new(0xFF, BitQ2).to_u32();
        assert_eq!(value, 0b11);

        let value = BitSize::new(0xFF, BitQ1).to_u32();
        assert_eq!(value, 0b1);

        println!("testing 1 and a half bytes");
        let value = BitSize::new(0xFFFF, BitQ12).to_u32();
        assert_eq!(value, 0b111111111111);

        println!("testing 2 bytes");
        let value = BitSize::new(0xFFFF, BitQ16).to_u32();
        assert_eq!(value, 0b1111111111111111);

        println!("testing 2 and a half bytes");
        let value = BitSize::new(0xFFFFFF, BitQ20).to_u32();
        assert_eq!(value, 0b11111111111111111111);

        println!("testing 3 bytes");
        let value = BitSize::new(0xFFFFFF, BitQ24).to_u32();
        assert_eq!(value, 0b111111111111111111111111);

    }

    #[test]
    fn bit_size_primitives() {
        let value: u8 = 0;
        assert_eq!(value.get_bit_quantity(), 8);
        let value: u16 = 0;
        assert_eq!(value.get_bit_quantity(), 16);
        let value: u32 = 0;
        assert_eq!(value.get_bit_quantity(), 32);
    }

    #[test]
    fn write_file() {
        let file: FileStructure = FileStructure {
            width: 21,
            height: 1,
            has_alpha_channel: true,
            chunks_x: BitSize(vec![BitArray::new(0)], BitQ2),
            chunks_y: BitSize(vec![BitArray::new(0)], BitQ2),
            colors_quantity: BitSize(vec![BitArray::new(4)], BitQ5),
            palette: vec![
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
                    index: Box::new(BitSize(vec![BitArray::new(0)], BitQ1)),
                    commands: vec![
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(vec![BitArray::new(3)], BitQ2)),
                        },
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(vec![BitArray::new(0)], BitQ2)),
                        },
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(vec![BitArray::new(1)], BitQ2)),
                        },
                        OperationTypes::DRAW {
                            pallete_color_index: Box::new(BitSize(vec![BitArray::new(2)], BitQ2)),
                        },
                    ],
                }],
            }],
        };
        let result = writer::write(file).unwrap().to_byte_vec();
        println!("{result:x?}");
    }
}
