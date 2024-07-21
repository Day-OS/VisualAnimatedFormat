use std::io::Read;
use std::str::Bytes;

use bitvec::array::BitArray;
use bitvec::order::Msb0;

use crate::bitsize::{BitQ1, BitQ8, BitSize};
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

    //Adds Header
    buffer.append_string(header);
    
    //Adds Width
    buffer.append_bytes(file.width.to_be_bytes().to_vec());

    //Adds Height
    buffer.append_bytes(file.height.to_be_bytes().to_vec());


    // Sets Animated Tag
    let mut is_animated = 0;

    if file.frames.len() <= 0{
        return Err(WriteError::NoFramesFound)
    }
    else if file.frames.len() > 1 {
        is_animated = 1;
    }

    buffer.append_bitsize(BitSize::new(is_animated, BitQ1));

    // Sets Alpha Tag
    let mut has_alpha_channel = 0;
    if file.has_alpha_channel {
        has_alpha_channel = 1;
    }
    buffer.append_bitsize(BitSize::new(has_alpha_channel, BitQ1));

    //FOR THE SAKE OF READABILITY WE IGNORE THE REST OF THE ZEROS UNTIL IT REACHES THE NEXT BYTE
    buffer.1 = 8;

    // How many divisions theres on each axis 1 subdivison = 2 chunks in that axis. 3 divisions are 6 chunks in that axis.
    
    buffer.append_bitsize(file.chunks_x);
    buffer.append_bitsize(file.chunks_y);

    for color in file.palette {
        //Representing there's more colors the next information are colors
        buffer.append_bitsize(BitSize::new(1, BitQ1));
        
        //Actual color:
        buffer.append_bitsize(BitSize::new(color.r.into(), BitQ8));
        buffer.append_bitsize(BitSize::new(color.g.into(), BitQ8));
        buffer.append_bitsize(BitSize::new(color.b.into(), BitQ8));
        if file.has_alpha_channel {
            buffer.append_bitsize(BitSize::new(color.a.unwrap_or(0).into(), BitQ8));
        }
    }
    // Representing there's no color left to be added from the palette
    buffer.append_bitsize(BitSize::new(0, BitQ1));

    // Building the image:

    for frame in file.frames {
        for chunk in frame.chunks {
            
            for operations in chunk.commands {
                
            }
        }
    }



    







    Ok(buffer)
}
