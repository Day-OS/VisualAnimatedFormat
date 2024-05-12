use crate::bitsize::{self, BitQ1, BitQ5, BitSize};

struct FileStructure{
    header: String,
    width: u16,
    height: u16,
    is_animaated: bool,
    has_alpha_channel: bool,
    palletes_quantity: BitSize<BitQ5>,
    palletes: Vec<Pallete>,
    is_animated: bool,
    frames: Vec<Frame>
}

struct Pallete{
    bit_depth: BitSize<BitQ5>,
    colors: Vec<Color>,
}

struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>
}

struct Frame{
    chunk_subdivision: u8
}