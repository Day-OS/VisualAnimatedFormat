use bitvec::array::BitArray;

use crate::bitsize::{BitQ1, BitQ16, BitQ2, BitQ8, BitSize};
use crate::buffer::Buffer;
use crate::errors::{ReadError, WriteError};
use crate::file_structure::{self, Color, FileStructure};
use crate::HEADER;


pub fn read(file: Vec<u8>) -> Result<FileStructure, ReadError> {
    
    let data = file.iter().map(|byte|{
        BitArray::new(byte.clone())
    }).collect();
    let mut buffer = Buffer{
        body: data,
        bit_head: 0,
        byte_head: 0
    };

    let header = buffer.read_chars(Some(HEADER.len()))?;
    if header != HEADER {
        return Err(ReadError::IncorrectFormat)
    }

    let metadata = buffer.read_string()?;

    println!("{metadata:?}");



    let width = buffer.read_u16()?;
    let heigth = buffer.read_u16()?;
    
    let mut is_animated = buffer.read_bool()?;
    let mut has_alpha_channel = buffer.read_bool()?;

    //FOR THE SAKE OF READABILITY WE IGNORE THE REST OF THE ZEROS UNTIL IT REACHES THE NEXT BYTE
    buffer.bit_head = 8;

    let chunks_x = buffer.read_bits(BitQ2)?;
    let chunks_y = buffer.read_bits(BitQ2)?;

    let mut palette: Vec<Color> = vec![];

    //While there are colors to be added
    while buffer.read_bool()? {
        let r = buffer.read_u8()?;
        let g = buffer.read_u8()?;
        let b = buffer.read_u8()?;
        let a = if has_alpha_channel {
            Some(buffer.read_u8()?)
        }else{
            None
        };
        palette.push(Color{r,g,b,a});
    }
    println!("{palette:?}");


    Err(ReadError::Invalid) // remove LATER!!!!
}
