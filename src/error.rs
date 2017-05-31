
use std::error::Error as StdError;
use std::convert::From;
use std::fmt;

use std::ffi::NulError;
use std::str::Utf8Error;
use vks::VkResult;

#[derive(Debug)]
pub enum Error {
    General(String),
    Nul(NulError),
    Vulkan(VkResult),
    StrUtf8(Utf8Error),
}

impl StdError for Error {
    fn description(&self) -> &str
    {
        match *self {
            Error::General(_) => "",
            Error::Nul(_) => "Nul Error",
            Error::Vulkan(_) => "Vulkan Error",
            Error::StrUtf8(_) => "UTF-8 Error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Nul(ref e) => Some(e),
            Error::StrUtf8(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::General(ref s) => write!(f, "{}", s),
            Error::Nul(ref e) => write!(f, "{}: {}", self.description(), e),
            Error::Vulkan(ref e) => write!(f, "{}: {:?}", self.description(), e),
            Error::StrUtf8(ref e) => write!(f, "{}: {}", self.description(), e),
        }
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::General(e)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(e: &'a str) -> Error {
        Error::General(e.to_owned())
    }
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Error {
        Error::Nul(e)
    }
}

impl From<VkResult> for Error {
    fn from(vk: VkResult) -> Error {
        Error::Vulkan(vk)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::StrUtf8(e)
    }
}
