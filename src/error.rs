
use std::convert::From;
use std::ffi::NulError;
use std::str::Utf8Error;
use vk_sys::VkResult;

#[derive(Debug)]
pub enum Error {
    General(String),
    Nul(NulError),
    Vulkan(VkResult),
    StrUtf8(Utf8Error),
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
