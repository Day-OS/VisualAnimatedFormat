use std::collections::HashMap;

use crate::bitsize::{BitQ2, BitQ4, BitQ5, BitQ6, BitQuantity};

#[derive(Debug, Clone)]
pub enum OperationTypes {
    DRAW {
        palette_color_index: Box<dyn BitQuantity>,
    },
    RUN {
        run_length: BitQ6,
    },
    DIFF {
        r: BitQ2,
        g: BitQ2,
        b: BitQ2,
    },
    BIGDIFF {
        g:   BitQ6,
        r_g: BitQ4,
        b_g: BitQ4,
    },
}

impl OperationTypes {
    pub(crate) fn to_number(&self) -> u8 {
        match self {
            OperationTypes::DRAW {
                palette_color_index: _,
            } => 0,
            OperationTypes::RUN { run_length: _ } => 1,
            OperationTypes::DIFF { r: _, g: _, b: _ } => 2,
            OperationTypes::BIGDIFF { g: _, r_g: _, b_g: _ } => 3,
        }
    }
}
enum DrawOperationTypes {
    RGB,
    RGBA,
}

pub(crate) const HEADER: &str = "VSF";

#[derive(Debug)]
pub struct FileStructure {
    pub metadata: String,
    pub width: u16,
    pub height: u16,
    pub has_alpha_channel: bool,
    pub subdivision: ChunkSubdivision,
    pub pallete_depth: Box<dyn BitQuantity>,
    pub palette: Vec<Color>,
    pub frames: Vec<Frame>,
}

/// Chunk Subdivisions are a way to declare how much divisions they are divided
///
/// ```
//                           QUANT:3
//QUANT:0 QUANT:1  QUANT:2  ┌┬┬┬┬┬┬┬┐
//┌─────┐ ┌──┬──┐ ┌─┬─┬─┬─┐ ├┼┼┼┼┼┼┼┤
//│     │ │  │  │ ├─┼─┼─┼─┤ ├┼┼┼┼┼┼┼┤
//│     │ │  │  │ │ │ │ │ │ ├┼┼┼┼┼┼┼┤
//│     │ ├──┼──┤ ├─┼─┼─┼─┤ ├┼┼┼┼┼┼┼┤
//│     │ │  │  │ │ │ │ │ │ ├┼┼┼┼┼┼┼┤
//│     │ │  │  │ ├─┼─┼─┼─┤ ├┼┼┼┼┼┼┼┤
//└─────┘ └──┴──┘ └─┴─┴─┴─┘ └┴┴┴┴┴┴┴┘
/// ```
#[derive(Debug)]
pub struct ChunkSubdivision (pub BitQ2);

impl ChunkSubdivision {
    /// This returns how many chunks will be present in this chunk subdivision
    pub fn get_subdivision_quantity(&mut self) -> u8 {
        let quantity = self.0.to_byte() + 1;
        // the need to convert u8 to u32 is pretty weird...
        // TO-DO: Code so this conversion is not needed
        let mut  result = u8::pow(2, quantity.into());
        result = result * result;
        return result;
    }
    pub fn get_subdivision_size(&mut self, height: u16, width: u16) -> u16{
        let quantity = self.get_subdivision_quantity();
        (height*width)/(quantity as u16)
    }
    
}

#[derive(PartialEq, Eq, Debug)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

#[derive(Debug)]
pub struct Frame {
    pub chunks: Vec<Option<Vec<OperationTypes>>>,
}

impl FileStructure {
    pub fn test_eq(&self, second: Self) {
        macro_rules! assert_a {
            ($parameter: ident) => {
                assert!(
                    self.$parameter == second.$parameter,
                    "$parameter must the same."
                );
            };
        }
        assert_a!(width);
        assert_a!(height);
        assert_a!(has_alpha_channel);
        //assert_a!(chunks_x);
        //assert_a!(chunks_y);
        //assert_a!(colors_quantity);
        assert_a!(palette);
        //assert_a!(frames);
        //assert_a!(colors_quantity);
    }
}
