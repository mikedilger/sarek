
use std::mem;
use vks::*;
use {Extent2D, ImageUsageFlags};
use super::{SurfaceTransformFlags, CompositeAlphaFlags};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SurfaceCapabilities {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlags,
    pub current_transform: SurfaceTransformFlags,
    pub supported_composite_alpha: CompositeAlphaFlags,
    pub supported_usage_flags: ImageUsageFlags,
}

impl From<VkSurfaceCapabilitiesKHR> for SurfaceCapabilities {
    fn from(vk: VkSurfaceCapabilitiesKHR) -> SurfaceCapabilities {
        assert_eq!(mem::size_of::<VkSurfaceCapabilitiesKHR>(),
                   mem::size_of::<SurfaceCapabilities>());
        unsafe {
            mem::transmute(vk)
        }
    }
}
