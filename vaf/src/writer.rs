use std::collections::HashMap;

use crate::bitsize::{BitQ1, BitQ16, BitQ8, BitQDyn, BitQuantity, BitSize};
use crate::buffer::Buffer;
use crate::errors::WriteError;
use crate::file_structure::OperationTypes;
use crate::{file_structure, HEADER};

pub fn write(file: file_structure::FileStructure) -> Result<Buffer, WriteError> {
    let mut buffer = Buffer {
        body: vec![],
        bit_head: 0,
        byte_head: 0,
    };

    //Adds Header
    buffer.append_chars(HEADER.as_bytes().to_vec());

    buffer.append_string(file.metadata);

    //Adds Width
    buffer.append_bitsize(BitSize::new(file.width.into(), BitQ16))?;

    //Adds Height
    buffer.append_bitsize(BitSize::new(file.height.into(), BitQ16))?;

    // Sets Animated Tag
    let mut is_animated = 0;

    if file.frames.len() <= 0 {
        return Err(WriteError::NoFramesFound);
    } else if file.frames.len() > 1 {
        is_animated = 1;
    }

    buffer.append_bitsize(BitSize::new(is_animated, BitQ1))?;

    // Sets Alpha Tag
    let mut has_alpha_channel = 0;
    if file.has_alpha_channel {
        has_alpha_channel = 1;
    }
    buffer.append_bitsize(BitSize::new(has_alpha_channel, BitQ1))?;

    //FOR THE SAKE OF READABILITY WE IGNORE THE REST OF THE ZEROS UNTIL IT REACHES THE NEXT BYTE
    buffer.bit_head = 8;

    // How many divisions theres on each axis 1 subdivison = 2 chunks in that axis. 3 divisions are 6 chunks in that axis.
    buffer.append_bitsize(file.subdivision.x.clone())?;
    buffer.append_bitsize(file.subdivision.y.clone())?;

    for color in file.palette {
        //Representing there's more colors the next information are colors
        buffer.append_bitsize(BitSize::new(1, BitQ1))?;
        //Actual color:
        buffer.append_bitsize(BitSize::new(color.r.into(), BitQ8))?;
        buffer.append_bitsize(BitSize::new(color.g.into(), BitQ8))?;
        buffer.append_bitsize(BitSize::new(color.b.into(), BitQ8))?;
        if file.has_alpha_channel {
            buffer.append_bitsize(BitSize::new(color.a.unwrap_or(0).into(), BitQ8))?;
        }
    }
    // Representing there's no color left to be added from the palette
    buffer.append_bitsize(BitSize::new(0, BitQ1))?;

    // Building the image:

    let chunks_quantity = file.subdivision.get_subdivision_quantity();

    let mut first_time: bool = true;
    for frame in file.frames {
        //Tells that there will be frames ahead
        buffer.append_bitsize(BitSize::new(1, BitQ1))?;

        let chunks = frame.get_chunk_u8();

        // if it is the first frame, fill empty chunks with dummy chunks, this prevent the program from going crazy
        if first_time {
            let mut chunks_temp: HashMap<u8, Vec<OperationTypes>> = HashMap::new();
            
            for i in 0..chunks_quantity-1 {
                if let Some(chunk) = chunks.get(&i) {
                    chunks_temp.insert(i, chunk.to_vec());
                }else{
                    let chunk = get_dummy_chunk(file.height as u32 * file.width as u32, file.pallete_depth);
                    chunks_temp.insert(i, chunk);
                }
            }
            first_time = false;
        }




        for chunk in chunks {
            println!("{chunks_quantity}");
            //for operations in chunk.commands {
            //    
            //}
        }
    }
    //Tells that there are no frames ahead
    buffer.append_bitsize(BitSize::new(0, BitQ1))?;

    Ok(buffer)
}

fn get_dummy_chunk(quantity_of_pixels: u32, bit_quantity: BitQDyn) -> Vec<OperationTypes>{
    let mut vec = Vec::<OperationTypes>::new();
    for i in 0..(quantity_of_pixels-1){
        vec.push(OperationTypes::DRAW { palette_color_index: BitSize::new(i, bit_quantity) });
    }
    vec
}