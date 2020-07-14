use std::io::Error as IoError;

#[derive(Debug)]
pub struct InvalidParameter {
    name: String,
    message: String
}

#[derive(Debug)]
pub struct CheckError {
    message: String,
    filename1: String,
    filename2: String,
    hash1: String,
    hash2: String
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
    parameter: String,
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(file1 = ({}, {}), file2 = ({}, {})", self.message, self.filename1, self.hash1, self.filename2, self.hash2)
    }
}

#[derive(Debug)]
pub struct CsvError {
    message: String,
    kind: csv::ErrorKind
}
#[derive(Debug)]
pub enum ApplicationError {
    Io(std::io::Error),
    Parameter(InvalidParameter),
    Clap(clap::Error),
    Check(CheckError),
    Csv(CsvError),
    Parse(ParseError),
}

impl ApplicationError {
    pub fn from_io(e: &IoError, msg: &str) -> ApplicationError {
        ApplicationError::Io(IoError::new(e.kind(), format!("{}: {}", msg, e.to_string())))
    }
    pub fn from_parameter(name: &str, msg: &str) -> ApplicationError {
        ApplicationError::Parameter(InvalidParameter {
            name: name.to_string(),
            message: msg.to_string()
        })
    }
    pub fn from_check(message: &str, f1: &str, f2: &str, h1: &str, h2: &str) -> ApplicationError {
        ApplicationError::Check(CheckError {
            message: message.to_owned(),
            filename1: f1.to_owned(),
            filename2: f2.to_owned(),
            hash1: h1.to_owned(),
            hash2: h2.to_owned()
        })
    }
    pub fn from_csv(e: csv::Error, msg: &str) -> ApplicationError {
        ApplicationError::Csv(CsvError {
            message: msg.to_owned(),
            kind: e.into_kind()
        })
    }
    pub fn from_parse_error(parameter: &str, msg: &str) -> ApplicationError {
        ApplicationError::Parse(ParseError {
            parameter: parameter.to_owned(),
            message: msg.to_owned()
        })
    }
}

