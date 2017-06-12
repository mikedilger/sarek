
use std::mem;
use vks::*;
use Bool32;

/// See vulkan specification, section 4.1 Physical Devices
// consider using a bit vector in a u8
#[derive(Debug, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: Bool32,
    pub residency_standard_2d_multisample_block_shape: Bool32,
    pub residency_standard_3d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

impl From<VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
    fn from(vk: VkPhysicalDeviceSparseProperties) -> PhysicalDeviceSparseProperties {
        assert_eq!(mem::size_of::<VkPhysicalDeviceSparseProperties>(),
                   mem::size_of::<PhysicalDeviceSparseProperties>());
        unsafe {
            mem::transmute(vk)
        }
    }
}
