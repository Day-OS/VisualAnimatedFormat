use bitvec::{array::BitArray, order::Msb0};
use thiserror::Error;

use crate::bitsize::BitQuantity;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("File does not contain any Frames")]
    NoFramesFound,

    #[error("Buffer Error")]
    BufferError(#[from] BufferError),
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("File does not contain any Frames")]
    NoFramesFound,

    #[error("Loaded file is not a VAF")]
    IncorrectFormat,

    #[error("invalid")]
    Invalid,

    #[error(transparent)]
    BufferError(#[from] BufferError),
}

#[derive(Error, Debug)]
pub enum BufferError {
    #[error(transparent)]
    BitSizeError(#[from] BitSizeError),
    #[error("Byte at {index} goes out of buffer's bounderies")]
    BufferIndexOutOfBound { index: usize },
    #[error("Data could not be converted into String")]
    StringConversionFailed,
}

#[derive(Error, Debug)]
pub enum BitSizeError {
    #[error("Data: {data:x?} | Byte at {index} index does not exist")]
    ByteIndexOutOfBound { data: Vec<u8>, index: usize },
    #[error("Data: {data:x?} | Bit at {index} index does not exist")]
    BitIndexOutOfBound { data: u8, index: u8 },
}

impl BitSizeError {
    pub fn throw_byte_index_out_of_bound(
        data: Vec<u8>,
        index: usize,
    ) -> BitSizeError {
        BitSizeError::ByteIndexOutOfBound {
            data: data,
            index: index,
        }
    }
    pub fn throw_bit_index_out_of_bound(data: BitArray<u8, Msb0>, index: usize) -> BitSizeError {
        BitSizeError::BitIndexOutOfBound {
            data: data.data,
            index: index as u8,
        }
    }
}
