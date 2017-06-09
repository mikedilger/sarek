
use std::mem;
use std::ptr;
use vks::*;
use {Error, InstanceLoader, Instance};
use instance::PhysicalDevice;

pub struct Device {
    device: VkDevice,
    loader: InstanceLoader,
}

impl Device {
    pub fn inner(&self) -> VkDevice
    {
        self.device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            (self.loader.0.core.vkDestroyDevice)(
                self.device,
                ptr::null());
        }
    }
}

impl Instance {
    pub fn create_device(&self, loader: InstanceLoader, physical_device: PhysicalDevice,
                         enabled_physical_device_features: &VkPhysicalDeviceFeatures,
                         queue_family_index: u32)
                         -> Result<Device, Error>
    {
        let device_extension_names = [ VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr() as *const i8 ];

        let priorities = [ 1.0 ];

        let queue_info = VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            queueFamilyIndex: queue_family_index,
            queueCount: priorities.len() as u32,
            pQueuePriorities: priorities.as_ptr()
        };

        let create_info = VkDeviceCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: Default::default(),
            queueCreateInfoCount: 1,
            pQueueCreateInfos: &queue_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: device_extension_names.len() as u32,
            ppEnabledExtensionNames: device_extension_names.as_ptr(),
            pEnabledFeatures: enabled_physical_device_features
        };

        let vkdevice = unsafe {
            let mut vkdevice: VkDevice = mem::uninitialized();
            vk_try!((loader.0.core.vkCreateDevice)(
                physical_device.inner(),
                &create_info,
                ptr::null(),
                &mut vkdevice));
            vkdevice
        };

        Ok(Device {
            device: vkdevice,
            loader: loader
        })
    }
}
