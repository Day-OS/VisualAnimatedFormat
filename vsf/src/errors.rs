use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("File does not contain any Frames")]
    NoFramesFound,

    #[error("Buffer Error")]
    BufferError(#[from] BufferError)
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("File does not contain any Frames")]
    NoFramesFound,

    #[error("Loaded file is not a VAF")]
    IncorrectFormat,

    #[error("invalid")]
    Invalid,

    #[error("Buffer Error")]
    BufferError(#[from] BufferError)
}


#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Item in defined index does not exist")]
    IndexOutOfBound,
    #[error("Data could not be converted into String")]
    StringConversionFailed,
}
