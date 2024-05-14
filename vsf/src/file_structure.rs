use crate::bitsize::{self, BitQ1, BitQ4, BitQ5, BitQuantity, BitSize};

enum OperationTypes<'a>{
    DRAW{
        pallete_color_index: &'a dyn BitQuantity,
    },
    RUN{

    },
    DIFF,
    BIGDIFF
}

impl OperationTypes<'_> {
    pub(crate) fn to_number(&self) -> u8{
        match self {
            OperationTypes::DRAW { pallete_color_id: _ } => 0,
            OperationTypes::RUN{} => 1,
            OperationTypes::DIFF => 2,
            OperationTypes::BIGDIFF => 3,
        }
    }
}

enum DrawOperationTypes{
    RGB,
    RGBA
}

struct FileStructure<'a>{
    header: String,
    width: u16,
    height: u16,
    is_animaated: bool,
    has_alpha_channel: bool,
    chunks_x: BitSize<BitQ4>,
    chunks_y: BitSize<BitQ4>,
    colors_quantity: BitSize<BitQ5>,
    pallete: Vec<Color>,
    is_animated: bool,
    frames: Vec<Frame<'a>>,

}

struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>
}

struct Frame<'a>{
    chunks: Vec<Chunk<'a>>
}

struct Chunk<'a>{
    index: &'a dyn BitQuantity,
    commands: Vec<OperationTypes<'a>>
}