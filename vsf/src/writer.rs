use std::io::Read;
use std::str::Bytes;

use bitvec::array::BitArray;
use bitvec::order::Msb0;

use crate::bitsize::{BitQ1, BitSize};
use crate::buffer;
use crate::buffer::Buffer;
use crate::errors::WriteError;
use crate::file_structure;

fn push_bytes(buffer: &mut Buffer, bytes: Bytes){
    for byte in bytes {
        buffer.0.push(BitArray::new(byte));
    }
}

pub fn write(file: file_structure::FileStructure) -> Result<Buffer, WriteError> {
    let mut buffer = Buffer(vec![], 0);
    let header = "VSF".to_string();
    let bit_head:u8 = 0;

    //Adds Header
    buffer.append_string(header);
    
    //Adds Width
    for byte in file.width.to_be_bytes() {
        buffer.0.push(BitArray::new(byte));
    }

    //Adds Height
    for byte in file.height.to_be_bytes() {
        buffer.0.push(BitArray::new(byte));
    }

    let mut is_animated = 0;

    if file.frames.len() <= 0{
        return Err(WriteError::NoFramesFound)
    }
    else if file.frames.len() > 1 {
        is_animated = 1;
    }

    buffer.append_bitsize(BitSize::new(1, BitQ1));




    Ok(buffer)
}
