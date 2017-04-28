
use std::ptr;

impl PhysicalDevice {
    pub fn get_properties(&self, loader: &mut InstanceLoader) ->
        Result<PhysicalDeviceProperties, Error>
    {
        let mut properties = VkPhysicalDeviceProperties2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
            pNext: ptr::null_mut(),
            properties: Default::default(),
        };
        unsafe {
            (loader.0.khr_get_physical_device_properties2.vkGetPhysicalDeviceProperties2KHR)(
                self.device,
                &mut properties
            );
        }

        Ok(PhysicalDeviceProperties {
            api_version: Version::from_vk(properties.properties.apiVersion),
            driver_version: properties.properties.driverVersion,
            vendor_id: properties.properties.vendorID,
            device_id: properties.properties.deviceID,
            device_type: unsafe { mem::transmute(properties.properties.deviceType) },
            device_name:  unsafe {
                str::from_utf8(
                    CStr::from_ptr(
                        properties.properties.deviceName.as_ptr())
                        .to_bytes())?
                .to_owned()
            },
            pipeline_cache_uuid: properties.properties.pipelineCacheUUID.clone(),
            limits: From::from(properties.properties.limits),
            sparse_properties: From::from(properties.properties.sparseProperties),
        })
    }
}
