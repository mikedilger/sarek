
extern crate vk_sys;
extern crate winit;
extern crate image;
extern crate libc;
#[cfg(windows)] extern crate user32;
#[cfg(windows)] extern crate winapi;

// Include our macros early
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/macros.rs"));

pub mod error;
pub use error::Error;

pub mod version;
pub use version::Version;

pub mod instance;
pub use instance::{InstanceLoader, Instance};


use std::ptr;
use std::mem;
use std::str;
use std::ffi::{CString, CStr};
use vk_sys::*;

/// See vulkan specification, section 30.2 Extensions
pub struct ExtensionProperties {
    pub extension_name: String,
    pub spec_version: u32
}

impl ExtensionProperties {
    fn from_vk(vk: VkExtensionProperties) -> Result<ExtensionProperties, Error>
    {
        Ok(ExtensionProperties {
            extension_name: unsafe {
                str::from_utf8(
                    CStr::from_ptr(
                        vk.extensionName.as_ptr())
                        .to_bytes())?
                    .to_owned()
            },
            spec_version: vk.specVersion,
        })
    }
}

/// See vulkan specification, section 30.2 Extensions.
/// Despite what the name implies, this returns a Vec not an Iterator.
pub fn enumerate_instance_extension_properties(layer_name: Option<&str>)
    -> Result<Vec<ExtensionProperties>, Error>
{
    let layer_name_cstring: Option<CString> = match layer_name {
        Some(s) => Some(CString::new(s)?),
        None => None
    };
    let p_layer_name = match layer_name_cstring {
        Some(ref s) => s.as_ptr(),
        None => ptr::null(),
    };

    // Call once to get the property count
    let mut property_count: u32 = 0;
    vk_try!(unsafe { vkEnumerateInstanceExtensionProperties(
        p_layer_name,
        &mut property_count,
        ptr::null_mut()
    )});

    // Prepare room for the output
    let capacity: usize = property_count as usize;
    let mut properties: Vec<VkExtensionProperties> = Vec::with_capacity(capacity);

    // Call again to get the data
    vk_try!(unsafe { vkEnumerateInstanceExtensionProperties(
        p_layer_name,
        &mut property_count,
        properties.as_mut_ptr()
    )});

    // Trust the data now in the properties vector
    let properties = unsafe {
        let p = properties.as_mut_ptr();
        mem::forget(properties);
        Vec::from_raw_parts(p, property_count as usize, capacity)
    };

    // Translate for output
    let mut output: Vec<ExtensionProperties> = Vec::with_capacity(property_count as usize);
    for property in properties {
        output.push(ExtensionProperties::from_vk(property)?);
    }
    Ok(output)
}

/// See vulkan specification, section 30.1 Layers
pub struct LayerProperties {
    pub layer_name: String,
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: String,
}

impl LayerProperties {
    fn from_vk(vk: VkLayerProperties) -> Result<LayerProperties, Error>
    {
        Ok(LayerProperties {
            layer_name: unsafe {
                str::from_utf8(
                    CStr::from_ptr(
                        vk.layerName.as_ptr())
                        .to_bytes())?
                    .to_owned()
            },
            spec_version: vk.specVersion,
            implementation_version: vk.implementationVersion,
            description: unsafe {
                str::from_utf8(
                    CStr::from_ptr(
                        vk.description.as_ptr())
                        .to_bytes())?
                    .to_owned()
            },
        })
    }
}

/// See vulkan specification, section 30.1 Layers.
/// Despite what the name implies, this returns a Vec not an Iterator.
pub fn enumerate_instance_layer_properties() -> Result<Vec<LayerProperties>, Error>
{
    // Call once to get the property count
    let mut property_count: u32 = 0;
    vk_try!(unsafe { vkEnumerateInstanceLayerProperties(
        &mut property_count,
        ptr::null_mut()
    )});

    // Prepare room for the output
    let capacity: usize = property_count as usize;
    let mut properties: Vec<VkLayerProperties> = Vec::with_capacity(capacity);

    // Call again to get the data
    vk_try!(unsafe { vkEnumerateInstanceLayerProperties(
        &mut property_count,
        properties.as_mut_ptr()
    )});

    // Trust the data now in the properties vector
    let properties = unsafe {
        let p = properties.as_mut_ptr();
        mem::forget(properties);
        Vec::from_raw_parts(p, property_count as usize, capacity)
    };

    // Translate for output
    let mut output: Vec<LayerProperties> = Vec::with_capacity(property_count as usize);
    for property in properties {
        output.push(LayerProperties::from_vk(property)?);
    }
    Ok(output)
}
