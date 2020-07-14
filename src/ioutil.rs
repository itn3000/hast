use super::error::ApplicationError;
use std::io::{Write, Read};

pub fn get_file_or_stdin(filepath: &str) -> Result<Box<dyn Read>, ApplicationError> {
    if filepath != "-" {
        match std::fs::File::open(filepath) {
            Ok(v) => Ok(Box::new(v)),
            Err(e) => Err(ApplicationError::from_io(&e, format!("failed to open file for read({})", filepath).as_str()))
        }
    } else {
        Ok(Box::new(std::io::stdin()))
    }
}

pub fn create_file_for_write(path: &str) -> Result<Box<dyn Write>, ApplicationError> {
    if path != "-" {
        match std::fs::File::create(path) {
            Ok(v) => Ok(Box::new(v)),
            Err(e) => Err(ApplicationError::from_io(&e, format!("failed to create file for write({})", path).as_str()))
        }
    } else {
        Ok(Box::new(std::io::stdout()))
    }
}

