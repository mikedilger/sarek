
use std::mem;
use vks::*;
use Extent3D;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct QueueFlags: u32 {
        const QUEUE_FLAGS_GRAPHICS_BIT = 0x1;
        const QUEUE_FLAGS_COMPUTE_BIT = 0x2;
        const QUEUE_FLAGS_TRANSFER_BIT = 0x4;
        const QUEUE_FLAGS_SPARSE_BINDING_BIT = 0x8;
    }
}

impl From<VkQueueFlags> for QueueFlags {
    fn from(vk: VkQueueFlags) -> QueueFlags {
        QueueFlags::from_bits(vk.bits()).unwrap()
    }
}

impl Into<VkQueueFlags> for QueueFlags {
    fn into(self) -> VkQueueFlags {
        VkQueueFlags::from_bits(self.bits()).unwrap()
    }
}


#[derive(Debug, Clone)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

impl From<VkQueueFamilyProperties> for QueueFamilyProperties {
    fn from(vk: VkQueueFamilyProperties) -> QueueFamilyProperties {
        assert_eq!(mem::size_of::<VkQueueFamilyProperties>(),
                   mem::size_of::<QueueFamilyProperties>());
        unsafe {
            mem::transmute(vk)
        }
    }
}
