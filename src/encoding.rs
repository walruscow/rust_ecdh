use std::io;

#[derive(Debug)]
pub enum EncodingError {
    IoError(io::Error),
}

#[derive(Debug)]
pub enum DecodingError {
    IoError(io::Error),
}

pub type EncodingResult = Result<(), EncodingError>;
pub type DecodingResult<T> = Result<T, DecodingError>;
