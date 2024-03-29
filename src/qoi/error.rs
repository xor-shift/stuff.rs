use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorType {
    InsufficientData,
    BadMagic,
    BadMetadata,
    TooMuchData,
    BadEndMark,

    IOError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s = match self {
            ErrorType::InsufficientData => "insufficient data",
            ErrorType::BadMagic => "bad magic value",
            ErrorType::BadMetadata => "bad metadata",
            ErrorType::TooMuchData => "too much data",
            ErrorType::BadEndMark => "bad end mark",
            ErrorType::IOError => "I/O error",
        };

        f.write_str(s)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QoIError {
    pub err_type: ErrorType,

    desc: String,
}

impl Display for QoIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("QoI error of type \"{}\", additional info: {}", self.err_type, self.desc))
    }
}

impl QoIError {
    pub fn new(err_type: ErrorType) -> Self { Self::new_with_description(err_type, "no description provided".into()) }

    pub fn new_with_description(err_type: ErrorType, desc: String) -> Self { Self { err_type, desc } }
}

impl From<std::io::Error> for QoIError {
    fn from(_value: std::io::Error) -> Self { Self::new(ErrorType::IOError) }
}

impl std::error::Error for QoIError {
    fn source(&self) -> Option<&'static dyn std::error::Error> { None }

    fn description(&self) -> &str { self.desc.as_str() }

    fn cause(&self) -> Option<&dyn std::error::Error> { None }
}

pub type Result<T> = std::result::Result<T, QoIError>;
