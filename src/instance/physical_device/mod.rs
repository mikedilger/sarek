
mod physical_device_type;
pub use self::physical_device_type::PhysicalDeviceType;

mod physical_device_limits;
pub use self::physical_device_limits::{PhysicalDeviceLimits, DeviceSize};

mod physical_device_sparse_properties;
pub use self::physical_device_sparse_properties::PhysicalDeviceSparseProperties;

mod queue_family_properties;
pub use self::queue_family_properties::{QueueFamilyProperties, QueueFlags};
pub use self::queue_family_properties::{QUEUE_FLAGS_GRAPHICS_BIT, QUEUE_FLAGS_COMPUTE_BIT,
                                        QUEUE_FLAGS_TRANSFER_BIT, QUEUE_FLAGS_SPARSE_BINDING_BIT};

mod physical_device_properties;
pub use self::physical_device_properties::PhysicalDeviceProperties;

mod extension_properties;
pub use self::extension_properties::ExtensionProperties;

mod physical_device_features;
pub use self::physical_device_features::PhysicalDeviceFeatures;

use std::mem;
use std::str;
use std::ptr;
use vks::*;
use {Error, InstanceLoader, Format, FormatProperties};
#[cfg(feature = "khr_surface")]
use instance::surface::{Surface, SurfaceFormat, SurfaceCapabilities, PresentMode};

/// See vulkan specification, section 4.1 Physical Devices
pub struct PhysicalDevice {
    #[allow(dead_code)]
    device: VkPhysicalDevice,
}
// No need to destroy VkPhysicalDevice explicitly.  They are implicitly destroyed
// when the instance is destroyed (see Section 2.3)

impl PhysicalDevice {
    pub fn from_vk(vk: VkPhysicalDevice) -> Result<PhysicalDevice, Error>
    {
        Ok(PhysicalDevice {
            device: vk
        })
    }

    pub fn inner(&self) -> VkPhysicalDevice
    {
        self.device
    }
}


impl PhysicalDevice {
    #[cfg(not(feature = "khr_get_physical_device_properties2"))]
    pub fn get_properties(&self, loader: &InstanceLoader) ->
        Result<PhysicalDeviceProperties, Error>
    {
        let mut properties: VkPhysicalDeviceProperties = unsafe { mem::uninitialized() };
        unsafe {
            (loader.0.core.vkGetPhysicalDeviceProperties)(
                self.device,
                &mut properties
            )
        }

        Ok(From::from(properties))
    }

    // fixme: need custom version for khr_get_physical_device_properties2
    pub fn get_queue_family_properties(&self, loader: &InstanceLoader) ->
        Result<Vec<QueueFamilyProperties>, Error>
    {
        // Call once to get the count
        let mut property_count: u32 = unsafe { mem::uninitialized() };
        unsafe {
            (loader.0.core.vkGetPhysicalDeviceQueueFamilyProperties)(
                self.device,
                &mut property_count,
                ptr::null_mut(),
            );
        }

        // Prepare room for the output
        let capacity: usize = property_count as usize;
        let mut properties: Vec<VkQueueFamilyProperties> = Vec::with_capacity(capacity);

        // Call again to get the data
        unsafe {
            (loader.0.core.vkGetPhysicalDeviceQueueFamilyProperties)(
                self.device,
                &mut property_count,
                properties.as_mut_ptr(),
            );
        }
        assert_eq!(property_count as usize, capacity);

        // Trust the data now in the properties vector
        let properties = unsafe {
            let ptr = properties.as_mut_ptr();
            mem::forget(properties);
            Vec::from_raw_parts(ptr, property_count as usize, capacity)
        };

        // Translate for output
        let mut output: Vec<QueueFamilyProperties> = Vec::with_capacity(property_count as usize);
        for property in properties {
            output.push(From::from(property));
        }
        Ok(output)
    }

    pub fn get_extension_properties(&self, loader: &InstanceLoader) ->
        Result<Vec<ExtensionProperties>, Error>
    {
        // Call once to get the count
        let mut property_count: u32 = unsafe { mem::uninitialized() };

        unsafe {
            (loader.0.core.vkEnumerateDeviceExtensionProperties)(
                self.device,
                ptr::null(), // pLayerName: *const c_char
                &mut property_count, // pProprtyCount: *mut u32
                ptr::null_mut()); // pProperties: *mut VkExtensionProperties
        }

        let capacity: usize = property_count as usize;
        let mut properties: Vec<VkExtensionProperties> = Vec::with_capacity(capacity);

        // Call again to get the data
        unsafe {
            (loader.0.core.vkEnumerateDeviceExtensionProperties)(
                self.device,
                ptr::null(), // pLayerName: *const c_char
                &mut property_count, // pProprtyCount: *mut u32
                properties.as_mut_ptr()); // pProperties: *mut VkExtensionProperties
        }
        assert_eq!(property_count as usize, capacity);

        // Trust the data now in the properties vector
        let properties = unsafe {
            let ptr = properties.as_mut_ptr();
            mem::forget(properties);
            Vec::from_raw_parts(ptr, property_count as usize, capacity)
        };

        // Translate for output
        let mut output: Vec<ExtensionProperties> = Vec::with_capacity(property_count as usize);
        for property in properties {
            output.push(From::from(property));
        }
        Ok(output)
    }

    #[cfg(feature = "khr_surface")]
    pub fn get_surface_support(&self, loader: &InstanceLoader, queue_family_index: u32,
                               surface: &Surface)
                               -> Result<bool, Error>
    {
        Ok( unsafe {
            let mut supported: u32 = mem::uninitialized();
            vk_try!((loader.0.khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR)(
                self.device,
                queue_family_index,
                surface.inner(),
                &mut supported
            ));
            supported
        } != 0)
    }

    #[cfg(feature = "khr_surface")]
    pub fn get_surface_formats(&self, loader: &InstanceLoader, surface: &Surface)
                               -> Result<Vec<SurfaceFormat>, Error>
    {
        // Call once to get the count
        let mut count: u32 = 0;
        unsafe {
            vk_try!((loader.0.khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR)(
                self.device,
                surface.inner(),
                &mut count,
                ptr::null_mut()
            ));
        }

        // Prepare room for the surface_formats output
        let capacity: usize = count as usize;
        let mut surface_formats: Vec<VkSurfaceFormatKHR> = Vec::with_capacity(capacity);

        // Call again to get the data
        unsafe {
            vk_try!((loader.0.khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR)(
                self.device,
                surface.inner(),
                &mut count,
                surface_formats.as_mut_ptr()
            ));
        }
        assert_eq!(count as usize, capacity);

        // Trust the data now in the surface_formats vector
        let mut surface_formats = unsafe {
            let ptr = surface_formats.as_mut_ptr();
            mem::forget(surface_formats);
            Vec::from_raw_parts(ptr, count as usize, capacity as usize)
        };

        // Translate for output
        let output = surface_formats.drain(..).map(|sf| From::from(sf)).collect();
        Ok(output)
    }

    #[cfg(feature = "khr_surface")]
    pub fn get_surface_capabilities(&self, loader: &InstanceLoader, surface: &Surface)
                                    -> Result<SurfaceCapabilities, Error>
    {
        let capabilities = unsafe {
            let mut capabilities: VkSurfaceCapabilitiesKHR = mem::uninitialized();
            vk_try!((loader.0.khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(
                self.device,
                surface.inner(),
                &mut capabilities
            ));
            capabilities
        };
        Ok(From::from(capabilities))
    }

    #[cfg(feature = "khr_surface")]
    pub fn get_surface_present_modes(&self, loader: &InstanceLoader, surface: &Surface)
                                     -> Result<Vec<PresentMode>, Error>
    {
        // Call once to get the count
        let mut count: u32 = 0;
        unsafe {
            vk_try!((loader.0.khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR)(
                self.device,
                surface.inner(),
                &mut count,
                ptr::null_mut()
            ));
        }

        // Prepare room for the present_modes output
        let mut present_modes: Vec<VkPresentModeKHR> = Vec::with_capacity(count as usize);

        // Call again to get the data
        unsafe {
            vk_try!((loader.0.khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR)(
                self.device,
                surface.inner(),
                &mut count,
                present_modes.as_mut_ptr(),
            ));
        }

        // Trust the data now in the present_modes vector
        let mut present_modes = unsafe {
            let ptr = present_modes.as_mut_ptr();
            mem::forget(present_modes);
            Vec::from_raw_parts(ptr, count as usize, count as usize)
        };

        // Translate for output
        let output: Vec<PresentMode> = present_modes.drain(..).map(|pm| From::from(pm))
            .collect();
        Ok(output)
    }

    pub fn get_format_properties(&self, loader: &InstanceLoader, format: Format)
                                 -> Result<FormatProperties, Error>
    {
        let mut format_properties: VkFormatProperties = unsafe {
            mem::uninitialized()
        };
        unsafe {
            (loader.0.core.vkGetPhysicalDeviceFormatProperties)(
                self.device,
                format.into(),
                &mut format_properties);
        }
        Ok(From::from(format_properties))
    }
}

#[cfg(feature = "khr_get_physical_device_properties2")]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/instance/physical_device/khr_get_physical_device_properties2_1.rs"));
