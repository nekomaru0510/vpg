pub mod toml;

use std::fmt;
use crate::param::{SystemConfig, ContainerConfig};

pub struct Parser {}

impl Parser {
    pub fn parse<T: TraitFormat>(text: String) -> Result<SystemConfig, FormatError> {
        T::parse(text)
    }
}

pub trait TraitFormat {
    fn parse(text: String) -> Result<SystemConfig, FormatError>;
}

#[derive(Debug)]
pub enum FormatError {
    ParseError(String),
    KeyNotFoundError(String),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FormatError::KeyNotFoundError(key) => write!(f, "Key not found: {}", key),
        }
    }
}

impl std::error::Error for FormatError {}

