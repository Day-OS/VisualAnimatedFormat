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

        let value = BitQ8::new(0xFF).to_u32();
        assert_eq!(value, 0b11111111);

        let value = BitQ7::new(0xFF).to_u32();
        assert_eq!(value, 0b1111111);

        let value = BitQ6::new(0xFF).to_u32();
        assert_eq!(value, 0b111111);

        let value = BitQ5::new(0xFF).to_u32();
        assert_eq!(value, 0b11111);

        let value = BitQ4::new(0xFF).to_u32();
        assert_eq!(value, 0b1111);

        let value = BitQ3::new(0xFF).to_u32();
        assert_eq!(value, 0b111);

        let value = BitQ2::new(0xFF).to_u32();
        assert_eq!(value, 0b11);

        let value = BitQ1::new(0xFF).to_u32();
        assert_eq!(value, 0b1);
        println!("testing 1 and a half bytes");
        let value  = BitQ12::new(0xFFFF).to_u32();
        assert_eq!(value, 0b111111111111);

        println!("testing 2 bytes");
        let value = BitQ16::new(0xFFFF).to_u32();
        assert_eq!(value, 0b1111111111111111);

        println!("testing 2 and a half bytes");
        assert_eq!(value, 0b11111111111111111111);

        println!("testing 3 bytes");
        let value = BitQ24::new(0xFFFFFF).to_u32();
        assert_eq!(value, 0b111111111111111111111111);
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
        //TODO: Fix this later
        let pallete_depth: Box<BitQ24> = Box::new(BitQ24::new(palette_draw_index_size.try_into().unwrap())); //BitQDyn::get_from_quantity(palette_draw_index_size).unwrap();
        let file: FileStructure = FileStructure {
            metadata: "Hello World!".to_string(),
            width: 21,
            height: 1,
            has_alpha_channel: true,
            subdivision: ChunkSubdivision(BitQ2::new(0)),
            pallete_depth,
            palette,
            frames: vec![Frame {
                chunks: vec![Some(
                    vec![
                        OperationTypes::DRAW {
                            palette_color_index: Box::new(BitQ2::new(3))
                        },
                        OperationTypes::DRAW {
                            palette_color_index: Box::new(BitQ2::new(0))
                        },
                        OperationTypes::DRAW {
                            palette_color_index: Box::new(BitQ2::new(1))
                        },
                        OperationTypes::DRAW {
                            palette_color_index: Box::new(BitQ2::new(2))
                        }
                    ]
                )]
                
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
