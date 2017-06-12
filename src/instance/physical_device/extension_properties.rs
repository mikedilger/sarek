
use std::ffi::CStr;
use vks::VkExtensionProperties;

#[derive(Debug, Clone)]
pub struct ExtensionProperties {
    pub extension_name: String,
    pub spec_version: u32,
}
impl From<VkExtensionProperties> for ExtensionProperties {
    fn from(vk: VkExtensionProperties) -> ExtensionProperties {
        ExtensionProperties {
            extension_name: unsafe {
                String::from_utf8_lossy(
                    CStr::from_ptr(vk.extensionName.as_ptr()).to_bytes()
                ).into_owned()
            },
            spec_version: vk.specVersion
        }
    }
}
