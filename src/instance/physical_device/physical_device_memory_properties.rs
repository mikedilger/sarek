
use std::mem;
use vks::{VkPhysicalDeviceMemoryProperties, VkMemoryType, VkMemoryHeap,
          VkMemoryPropertyFlags, VkMemoryHeapFlags};
use super::DeviceSize;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryPropertyFlags: u32 {
        const MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001;
        const MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002;
        const MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004;
        const MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008;
        const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010;
    }
}

impl From<VkMemoryPropertyFlags> for MemoryPropertyFlags {
    fn from(vk: VkMemoryPropertyFlags) -> MemoryPropertyFlags {
        MemoryPropertyFlags::from_bits(vk.bits()).unwrap()
    }
}

impl Into<VkMemoryPropertyFlags> for MemoryPropertyFlags {
    fn into(self) -> VkMemoryPropertyFlags {
        VkMemoryPropertyFlags::from_bits(self.bits()).unwrap()
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryHeapFlags: u32 {
        const MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001;
        #[cfg(feature = "khx_device_group_creation")]
        const MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX = 0x00000002;
    }
}

impl From<VkMemoryHeapFlags> for MemoryHeapFlags {
    fn from(vk: VkMemoryHeapFlags) -> MemoryHeapFlags {
        MemoryHeapFlags::from_bits(vk.bits()).unwrap()
    }
}

impl Into<VkMemoryHeapFlags> for MemoryHeapFlags {
    fn into(self) -> VkMemoryHeapFlags {
        VkMemoryHeapFlags::from_bits(self.bits()).unwrap()
    }
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

impl From<VkMemoryType> for MemoryType {
    fn from(vk: VkMemoryType) -> MemoryType {
        unsafe {
            mem::transmute(vk)
        }
    }
}

impl Into<VkMemoryType> for MemoryType {
    fn into(self) -> VkMemoryType {
        unsafe {
            mem::transmute(self)
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

impl From<VkMemoryHeap> for MemoryHeap {
    fn from(vk: VkMemoryHeap) -> MemoryHeap {
        unsafe {
            mem::transmute(vk)
        }
    }
}

impl Into<VkMemoryHeap> for MemoryHeap {
    fn into(self) -> VkMemoryHeap {
        unsafe {
            mem::transmute(self)
        }
    }
}


#[derive(Debug, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_types: Vec<MemoryType>,
    pub memory_heaps: Vec<MemoryHeap>
}

impl From<VkPhysicalDeviceMemoryProperties> for PhysicalDeviceMemoryProperties {
    fn from(vk: VkPhysicalDeviceMemoryProperties) -> PhysicalDeviceMemoryProperties {
        assert!(vk.memoryTypeCount <= 32);
        assert!(vk.memoryHeapCount <= 16);
        PhysicalDeviceMemoryProperties {
            memory_types: unsafe {
                mem::transmute(vk.memoryTypes[0..vk.memoryTypeCount as usize].to_vec())
            },
            memory_heaps: unsafe {
                mem::transmute(vk.memoryHeaps[0..vk.memoryHeapCount as usize].to_vec())
            }
        }
    }
}
