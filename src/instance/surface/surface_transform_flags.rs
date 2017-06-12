
use vks::VkSurfaceTransformFlagsKHR;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SurfaceTransformFlags: u32 {
        const SURFACE_TRANSFORM_IDENTITY_BIT = 0x1;
        const SURFACE_TRANSFORM_ROTATE_90_BIT = 0x2;
        const SURFACE_TRANSFORM_ROTATE_180_BIT = 0x4;
        const SURFACE_TRANSFORM_ROTATE_270_BIT = 0x8;
        const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT = 0x10;
        const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT = 0x20;
        const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT = 0x40;
        const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT = 0x80;
        const SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x100;
    }
}

impl From<VkSurfaceTransformFlagsKHR> for SurfaceTransformFlags {
    fn from(vk: VkSurfaceTransformFlagsKHR) -> SurfaceTransformFlags {
        SurfaceTransformFlags::from_bits(vk.bits()).unwrap()
    }
}

impl Into<VkSurfaceTransformFlagsKHR> for SurfaceTransformFlags {
    fn into(self) -> VkSurfaceTransformFlagsKHR {
        VkSurfaceTransformFlagsKHR::from_bits(self.bits()).unwrap()
    }
}


