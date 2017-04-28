
#[derive(Debug, Clone)]
pub struct Version(pub u32, pub u32, pub u32);

impl Version {
    #[inline]
    pub fn to_vk(&self) -> u32
    {
        (self.0 << 22) | (self.0 << 12) | self.2
    }

    #[inline]
    pub fn from_vk(vk: u32) -> Version
    {
        Version(
            (vk & 0xffc00000) >> 22,
            (vk & 0x003ff000) >> 12,
            (vk & 0x00000fff) )
    }
}

use std::fmt;

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}
