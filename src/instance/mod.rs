
use libc::c_char;
use std::ffi::CString;
use std::ptr;
use std::mem;
use vk_sys::*;

use {Error, Version};

pub struct InstanceLoader(InstanceProcAddrLoader);

impl InstanceLoader {
    pub fn new() -> InstanceLoader {
        // Instantiate a loader
        let mut loader = InstanceProcAddrLoader::from_get_instance_proc_addr(
            vkGetInstanceProcAddr);

        // Load function pointers with global scope
        unsafe { loader.load_core_null_instance(); }

        InstanceLoader(loader)
    }
}

pub struct ApplicationInfo {
    pub application_name: String,
    pub application_version: Version,
    pub engine_name: String,
    pub engine_version: Version,
}

pub struct InstanceCreateInfo {
    pub application_info: ApplicationInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: Vec<String>,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: Vec<String>,
}

/// See vulkan specification, section 3.2 Instances
pub struct Instance(VkInstance);

impl Instance {
    #[allow(unused_variables)]
    pub fn new(loader: &mut InstanceLoader, create_info: InstanceCreateInfo)
               -> Result<Instance, Error>
    {
        // Setup strings for passing into vkCreateInstance down below.  These must
        // not go out of scope until after that function is called.
        let (extension_names_owned, extension_names) = {
            let mut extension_names_owned: Vec<CString> = Vec::new();
            for ref name in create_info.enabled_extension_names {
                extension_names_owned.push( CString::new(name.as_bytes())? );
            }
            let extension_names: Vec<*const c_char> = extension_names_owned.iter()
                .map(|name| name.as_ptr())
                .collect();
            (extension_names_owned, extension_names)
        };

        let app_name = CString::new(create_info.application_info.application_name.as_bytes())?;
        let engine_name = CString::new(create_info.application_info.engine_name.as_bytes())?;

        let (layer_names_owned, layer_names) = {
            let mut layer_names_owned: Vec<CString> = Vec::new();
            for ref name in &create_info.enabled_layer_names {
                layer_names_owned.push( CString::new(name.as_bytes())? );
            }
            let layer_names: Vec<*const c_char> = layer_names_owned.iter()
                .map(|name| name.as_ptr())
                .collect();
            (layer_names_owned, layer_names)
        };

        let instance = {
            let create_info = {
                let app_info = {
                    VkApplicationInfo {
                        sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                        pNext: ptr::null(),
                        pApplicationName: app_name.as_ptr(),
                        applicationVersion: create_info.application_info
                            .application_version.to_vk(),
                        pEngineName: engine_name.as_ptr(),
                        engineVersion: create_info.application_info
                            .engine_version.to_vk(),
                        apiVersion: vk_make_version!(1, 0, 3),
                    }
                };

                VkInstanceCreateInfo {
                    sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                    pNext: ptr::null(),
                    flags: VK_INSTANCE_CREATE_DUMMY,
                    pApplicationInfo: &app_info,
                    enabledLayerCount: layer_names.len() as u32,
                    ppEnabledLayerNames: if layer_names.len() > 0 {
                        layer_names.as_ptr()
                    } else {
                        ptr::null()
                    },
                    enabledExtensionCount: extension_names.len() as u32,
                    ppEnabledExtensionNames: if extension_names.len() > 0 {
                        extension_names.as_ptr()
                    } else {
                        ptr::null()
                    }
                }
            };

            unsafe {
                let mut instance: VkInstance = ptr::null_mut();
                vk_try!((loader.0.core_null_instance.vkCreateInstance)(
                    &create_info,
                    ptr::null(),
                    &mut instance
                ));
                assert!(instance != ptr::null_mut());
                instance
            }
        };

        // Load core instance-level functions
        unsafe { loader.0.load_core(instance); }

        Ok(Instance(instance))
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(
                self.0,
                ptr::null());
        }
    }
}

/// See vulkan specification, section 4.1 Physical Devices
pub struct PhysicalDevice {
    #[allow(dead_code)]
    device: VkPhysicalDevice,
}
// No need to destroy VkPhysicalDevice explicitly.  They are implicitly destroyed
// when the instance is destroyed (see Section 2.3)

impl PhysicalDevice {
    fn from_vk(vk: VkPhysicalDevice) -> Result<PhysicalDevice, Error>
    {
        Ok(PhysicalDevice {
            device: vk
        })
    }
}

impl Instance {
    pub fn enumerate_physical_devices(&self, loader: &mut InstanceLoader)
                                      -> Result<Vec<PhysicalDevice>, Error>
    {
        // Call once to get the count
        let mut physical_device_count: u32 = unsafe { mem::uninitialized() };
        vk_try!(unsafe { (loader.0.core.vkEnumeratePhysicalDevices)(
            self.0,
            &mut physical_device_count,
            ptr::null_mut()
        )});

        // Prepare room for the output
        let capacity: usize = physical_device_count as usize;
        let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(capacity);

        // Call again to get the data
        vk_try!(unsafe { (loader.0.core.vkEnumeratePhysicalDevices)(
            self.0,
            &mut physical_device_count,
            devices.as_mut_ptr()
        )});

        // Trust the data now in the devices vector
        let devices = unsafe {
            let ptr = devices.as_mut_ptr();
            mem::forget(devices);
            Vec::from_raw_parts(ptr, physical_device_count as usize, capacity)
        };

        // Translate for output
        let mut output: Vec<PhysicalDevice> = Vec::with_capacity(physical_device_count as usize);
        for device in devices {
            output.push(PhysicalDevice::from_vk(device)?);
        }
        Ok(output)
    }
}
