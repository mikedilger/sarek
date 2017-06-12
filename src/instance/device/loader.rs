
use std::sync::Arc;
use vks::*;
use error::Error;

#[derive(Clone)]
pub struct DeviceLoader(pub Arc<DeviceProcAddrLoader>);

impl DeviceLoader {
    pub fn new() -> DeviceLoader {
        // Instantiate a loader
        let loader = DeviceProcAddrLoader::from_get_device_proc_addr(
            vkGetDeviceProcAddr);

        DeviceLoader(Arc::new(loader))
    }

    pub fn load(&mut self, device: VkDevice) -> Result<(), Error>
    {
        let loader = match Arc::get_mut(&mut self.0) {
            Some(l) => l,
            None => return Err(Error::General(
                "Loader was cloned prior to running load()".to_owned()))
        };

        unsafe { loader.load_core(device); }

        #[cfg(feature = "khr_display_swapchain")]
        unsafe { loader.load_khr_display_swapchain(device); }
        #[cfg(feature = "ext_debug_marker")]
        unsafe { loader.load_ext_debug_marker(device); }
        #[cfg(feature = "amd_draw_indirect_count")]
        unsafe { loader.load_amd_draw_indirect_count(device); }
        #[cfg(feature = "nvx_device_generated_commands")]
        unsafe { loader.load_nvx_device_generated_commands(device); }
        #[cfg(feature = "khr_maintenance1")]
        unsafe { loader.load_khr_maintenance1(device); }
        #[cfg(feature = "ext_display_control")]
        unsafe { loader.load_ext_display_control(device); }
        #[cfg(feature = "khr_push_descriptor")]
        unsafe { loader.load_khr_push_descriptor(device); }
        #[cfg(feature = "khr_descriptor_update_template")]
        unsafe { loader.load_khr_descriptor_update_template(device); }
        #[cfg(feature = "khx_device_group")]
        unsafe { loader.load_khx_device_group(device); }
        #[cfg(feature = "khx_external_memory_win32")]
        unsafe { loader.load_khx_external_memory_win32(device); }
        #[cfg(feature = "khx_external_memory_fd")]
        unsafe { loader.load_khx_external_memory_fd(device); }
        #[cfg(feature = "khx_external_semaphore_win32")]
        unsafe { loader.load_khx_external_semaphore_win32(device); }
        #[cfg(feature = "khx_external_semaphore_fd")]
        unsafe { loader.load_khx_external_semaphore_fd(device); }
        #[cfg(feature = "nv_clip_space_w_scaling")]
        unsafe { loader.load_nv_clip_space_w_scaling(device); }
        #[cfg(feature = "ext_discard_rectangles")]
        unsafe { loader.load_ext_discard_rectangles(device); }
        #[cfg(feature = "google_display_timing")]
        unsafe { loader.load_google_display_timing(device); }
        #[cfg(feature = "ext_hdr_metadata")]
        unsafe { loader.load_ext_hdr_metadata(device); }
        #[cfg(feature = "khr_swapchain")]
        unsafe { loader.load_khr_swapchain(device); }
        #[cfg(feature = "nv_external_memory_win32")]
        unsafe { loader.load_nv_external_memory_win32(device); }
        #[cfg(feature = "khr_shared_presentable_image")]
        unsafe { loader.load_khr_shared_presentable_image(device); }

        Ok(())
    }
}
