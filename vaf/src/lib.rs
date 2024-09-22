mod bitsize;
mod buffer;
mod errors;
mod file_structure;
mod reader;
mod writer;

const HEADER: &str = "VAF";

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Instant};

    use bitvec::array::BitArray;

    use crate::{
        bitsize::*,
        file_structure::{ChunkSubdivision, Color, FileStructure, Frame, OperationTypes},
        reader,
        writer, //writer,
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
    fn write_and_verify_file() {
        let start = Instant::now();

        let palette = vec![
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
        ];
        let palette_draw_index_size = f32::log2(palette.len() as f32).ceil() as usize;
        let pallete_depth = BitQDyn::get_from_quantity(palette_draw_index_size).unwrap();
        let file: FileStructure = FileStructure {
            metadata: "Hello World!".to_string(),
            width: 21,
            height: 1,
            has_alpha_channel: true,
            subdivision: ChunkSubdivision {
                x: BitSize(vec![BitArray::new(0)], BitQ2),
                y: BitSize(vec![BitArray::new(0)], BitQ2),
            },
            pallete_depth,
            palette,
            frames: vec![Frame {
                chunks: HashMap::from([
                    (
                        BitSize(vec![BitArray::new(0)], BitQDyn::BitQ1),
                        vec![
                            OperationTypes::DRAW {
                                palette_color_index: BitSize(
                                    vec![BitArray::new(3)],
                                    pallete_depth.clone(),
                                ),
                            },
                            OperationTypes::DRAW {
                                palette_color_index: BitSize(
                                    vec![BitArray::new(0)],
                                    pallete_depth.clone(),
                                ),
                            },
                            OperationTypes::DRAW {
                                palette_color_index: BitSize(
                                    vec![BitArray::new(1)],
                                    pallete_depth.clone(),
                                ),
                            },
                            OperationTypes::DRAW {
                                palette_color_index: BitSize(
                                    vec![BitArray::new(2)],
                                    pallete_depth,
                                ),
                            }
                        ]
                    )
                ])
            }],
        };
        let result = writer::write(file).unwrap();
        let write_elapsed = start.elapsed();
        let read_start = Instant::now();

        println!(
            "byte head: {:?} | bit head: {:?}",
            result.byte_head, result.bit_head
        );

        println!("{:x?}", result.to_byte_vec());

        let read_file = reader::read(result.to_byte_vec());

        let read_elapsed = read_start.elapsed();
        let total_elapsed = start.elapsed();
        println!("{read_file:?}");

        println!(
            "TOTAL ELAPSED TIME: {}secs | Write elapsed time: {}secs | Read elapsed time: {}secs",
            total_elapsed.as_secs_f64(),
            write_elapsed.as_secs_f64(),
            read_elapsed.as_secs_f64()
        )
    }
}
