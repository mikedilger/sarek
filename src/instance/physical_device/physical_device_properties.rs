
use std::mem;
use std::str;
use std::ffi::CStr;
use vks::*;
use super::{PhysicalDeviceType, PhysicalDeviceLimits, PhysicalDeviceSparseProperties};
use {Version};

/// See vulkan specification, section 4.1 Physical Devices
#[derive(Debug, Clone)]
pub struct PhysicalDeviceProperties {
    pub api_version: Version,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; VK_UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

impl From<VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
    fn from(vk: VkPhysicalDeviceProperties) -> PhysicalDeviceProperties {
        PhysicalDeviceProperties {
            api_version: Version::from_vk(vk.apiVersion),
            driver_version: vk.driverVersion,
            vendor_id: vk.vendorID,
            device_id: vk.deviceID,
            device_type: unsafe { mem::transmute(vk.deviceType) },
            device_name:  unsafe {
                str::from_utf8(
                    CStr::from_ptr(
                        vk.deviceName.as_ptr())
                        .to_bytes()).unwrap()
                .to_owned()
            },
            pipeline_cache_uuid: vk.pipelineCacheUUID.clone(),
            limits: From::from(vk.limits),
            sparse_properties: From::from(vk.sparseProperties),
        }
    }
}
