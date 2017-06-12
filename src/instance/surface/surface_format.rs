
use std::mem;
use vks::*;
use super::ColorSpace;
use Format;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SurfaceFormat {
    pub format: Format,
    pub color_space: ColorSpace,
}

impl From<VkSurfaceFormatKHR> for SurfaceFormat {
    fn from(vk: VkSurfaceFormatKHR) -> SurfaceFormat {
        assert_eq!(mem::size_of::<VkSurfaceFormatKHR>(),
                   mem::size_of::<SurfaceFormat>());
        unsafe {
            mem::transmute(vk)
        }
    }
}
