use std::collections::HashMap;

use crate::bitsize::{BitQ2, BitQ4, BitQ5, BitQ6, BitQDyn, BitQuantity, BitSize};

#[derive(Debug, Clone)]
pub enum OperationTypes {
    DRAW {
        palette_color_index: BitSize<BitQDyn>,
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
                palette_color_index,
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

#[derive(Debug)]
pub struct FileStructure {
    pub metadata: String,
    pub width: u16,
    pub height: u16,
    pub has_alpha_channel: bool,
    pub subdivision: ChunkSubdivision,
    pub pallete_depth: BitQDyn,
    pub palette: Vec<Color>,
    pub frames: Vec<Frame>,
}

/// Chunk Subdivisions are a way to declare how much divisions are made and in which axis they are divided
///
/// ```
///x:0 y:1 x:1 y:0  x:0 y:1
///┌─────┐ ┌──┬──┐ ┌───────┐
///│     │ │  │  │ │       │
///│     │ │  │  │ │       │
///│     │ │  │  │ ├───────┤
///│     │ │  │  │ │       │
///│     │ │  │  │ │       │
///└─────┘ └──┴──┘ └───────┘
///x:1 y:1 x:0 y:2  x:3 y:0         
///┌──┬──┐ ┌─────┐ ┌┬┬┬┬┬┬┬┐
///│  │  │ │_____│ │││││││││
///│  │  │ │     │ │││││││││
///├──┼──┤ ├─────┤ │││││││││
///│  │  │ │_____│ │││││││││
///│  │  │ │     │ │││││││││
///└──┴──┘ └─────┘ └┴┴┴┴┴┴┴┘
/// ```
#[derive(Debug)]
pub struct ChunkSubdivision {
    pub x: BitSize<BitQ2>,
    pub y: BitSize<BitQ2>,
}

impl ChunkSubdivision {
    /// This returns how many chunks will be present in this chunk subdivision setup
    pub fn get_subdivision_quantity(self) -> u8 {
        let chunkx = self.x.to_byte() + 1;
        let chunky = self.y.to_byte() + 1;
        // the need to convert u8 to u32 is pretty weird...
        // TO-DO: Code so this conversion is not needed
        let result = u8::pow(2, chunkx.into()) * u8::pow(2, chunky.into());
        return result;
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
    pub chunks: HashMap<BitSize<BitQDyn>, Vec<OperationTypes>>,
}


impl Frame {
    /// Gets ordered chunks in a list
    pub fn get_chunks_ordered(&self) -> Vec<(u8, &Vec<OperationTypes>)>{
        let mut list: Vec<(u8, &Vec<OperationTypes>)> = self.get_chunk_u8().into_iter().collect();
        
        list.sort_by(|a, b|{
            a.0.cmp(&b.0)
        });
        list
    }

    pub fn get_chunk_u8(&self) -> HashMap<u8, &Vec<OperationTypes>>{
        let mut map = HashMap::<u8, &Vec<OperationTypes>>::new();

        // Turn BitSize into u8 and then creates a new map from it
        for (key, value) in self.chunks.iter().map(|(bit_size, operations)|{
            //We know that a frame have a limit of 64 subdivisions, so a byte is just fine for that
            (bit_size.to_byte(), operations)
        }) {
            map.insert(key, value);
        } 
        map
    }
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
