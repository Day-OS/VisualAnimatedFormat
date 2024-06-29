use crate::bitsize::{self, BitQ1, BitQ2, BitQ4, BitQ5, BitQ6, BitQuantity, BitSize};

pub enum OperationTypes {
    DRAW {
        pallete_color_index: Box<dyn BitQuantity>,
    },
    RUN {
        run_length: BitSize<BitQ6>,
    },
    DIFF {
        r: BitSize<BitQ2>,
        g: BitSize<BitQ2>,
        b: BitSize<BitQ2>,
    },
    BIGDIFF {
        g: BitSize<BitQ6>,
        r_g: BitSize<BitQ4>,
        b_g: BitSize<BitQ4>,
    },
}

impl OperationTypes {
    pub(crate) fn to_number(&self) -> u8 {
        match self {
            OperationTypes::DRAW {
                pallete_color_index,
            } => 0,
            OperationTypes::RUN { run_length } => 1,
            OperationTypes::DIFF { r, g, b } => 2,
            OperationTypes::BIGDIFF { g, r_g, b_g } => 3,
        }
    }
}

enum DrawOperationTypes {
    RGB,
    RGBA,
}

pub(crate) const HEADER: &str = "VSF";

pub struct FileStructure {
    //pub header: String,
    pub width: u16,
    pub height: u16,
    pub has_alpha_channel: bool,
    pub chunks_x: BitSize<BitQ4>,
    pub chunks_y: BitSize<BitQ4>,
    pub colors_quantity: BitSize<BitQ5>,
    pub pallete: Vec<Color>,
    pub frames: Vec<Frame>,
}

#[derive(PartialEq, Eq)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

pub struct Frame {
    pub chunks: Vec<Chunk>,
}

pub struct Chunk {
    pub index: Box<dyn BitQuantity>,
    pub commands: Vec<OperationTypes>,
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
        assert_a!(pallete);
        //assert_a!(frames);
        //assert_a!(colors_quantity);
    }
}
