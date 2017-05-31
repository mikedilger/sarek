
use std::ptr;
use std::ffi::CStr;
use std::default::Default;
use std::mem;
use vks::*;

use error::Error;
use super::loader::InstanceLoader;

pub struct DebugCallback {
    loader: InstanceLoader,
    instance: VkInstance, // reference only, we do not drop this.
    callback: VkDebugReportCallbackEXT
}

impl DebugCallback {
    pub fn new(instance: VkInstance, loader: InstanceLoader) -> Result<DebugCallback, Error>
    {
        let mut create_info: VkDebugReportCallbackCreateInfoEXT =
            Default::default();
        // FIXME: add config settings for these flags.
        create_info.flags = VK_DEBUG_REPORT_ERROR_BIT_EXT
            | VK_DEBUG_REPORT_WARNING_BIT_EXT
            | VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT;
        create_info.pfnCallback = vulkan_debug_callback;

        let mut callback: VkDebugReportCallbackEXT = unsafe { mem::uninitialized() };

        unsafe {
            vk_try!((loader.0.ext_debug_report.vkCreateDebugReportCallbackEXT)(
                instance, &create_info,
                ptr::null(), // pAllocator: *const VkAllocationCallbacks
                &mut callback));
        }

        Ok(DebugCallback {
            loader: loader,
            instance: instance,
            callback: callback
        })
    }
}

impl Drop for DebugCallback {
    fn drop(&mut self) {
        unsafe {
            (self.loader.0.ext_debug_report.vkDestroyDebugReportCallbackEXT)(
                self.instance, self.callback, ptr::null());
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(flags: VkDebugReportFlagsEXT,
                                                _: VkDebugReportObjectTypeEXT,
                                                _: u64,
                                                _: usize,
                                                _: i32,
                                                _: *const i8,
                                                p_message: *const i8,
                                                _: *mut ::libc::c_void)
                                                -> u32
{
    println!("{:?}", CStr::from_ptr(p_message));

    // We should return true here ONLY IF this was a validation ERROR (not warning
    // or info).
    if flags.intersects(VK_DEBUG_REPORT_ERROR_BIT_EXT) {
        1
    } else {
        0
    }
}
