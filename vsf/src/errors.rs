use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("File does not contain any Frames")]
    NoFramesFound
}
