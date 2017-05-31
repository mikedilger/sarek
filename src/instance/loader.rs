
use vks::*;

pub struct InstanceLoader(pub InstanceProcAddrLoader);

impl InstanceLoader {
    pub fn new() -> InstanceLoader {
        // Instantiate a loader
        let mut loader = InstanceProcAddrLoader::from_get_instance_proc_addr(
            vkGetInstanceProcAddr);

        // Load function pointers with global scope
        unsafe { loader.load_core_null_instance(); }

        InstanceLoader(loader)
    }

    pub fn load(&mut self, instance: VkInstance) {

        unsafe { self.0.load_core(instance); }

        #[cfg(feature = "khr_surface")]
        unsafe { self.0.load_khr_surface(instance); }
        #[cfg(feature = "khr_display")]
        unsafe { self.0.load_khr_display(instance); }
        #[cfg(feature = "khr_display_swapchain")]
        unsafe { self.0.load_khr_display_swapchain(instance); }
        #[cfg(feature = "khr_xlib_surface")]
        unsafe { self.0.load_khr_xlib_surface(instance); }
        #[cfg(feature = "khr_xcb_surface")]
        unsafe { self.0.load_khr_xcb_surface(instance); }
        #[cfg(feature = "khr_wayland_surface")]
        unsafe { self.0.load_khr_wayland_surface(instance); }
        #[cfg(feature = "khr_mir_surface")]
        unsafe { self.0.load_khr_mir_surface(instance); }
        #[cfg(feature = "khr_android_surface")]
        unsafe { self.0.load_khr_android_surface(instance); }
        #[cfg(feature = "khr_win32_surface")]
        unsafe { self.0.load_khr_win32_surface(instance); }
        #[cfg(feature = "ext_debug_report")]
        unsafe { self.0.load_ext_debug_report(instance); }
        #[cfg(feature = "ext_debug_marker")]
        unsafe { self.0.load_ext_debug_marker(instance); }
        #[cfg(feature = "amd_draw_indirect_count")]
        unsafe { self.0.load_amd_draw_indirect_count(instance); }
        #[cfg(feature = "nv_external_memory_capabilities")]
        unsafe { self.0.load_nv_external_memory_capabilities(instance); }
        #[cfg(feature = "nv_external_memory_win32")]
        unsafe { self.0.load_nv_external_memory_win32(instance); }
        #[cfg(feature = "nvx_device_generated_commands")]
        unsafe { self.0.load_nvx_device_generated_commands(instance); }
        #[cfg(feature = "khr_get_physical_device_properties2")]
        unsafe { self.0.load_khr_get_physical_device_properties2(instance); }
        #[cfg(feature = "khr_maintenance1")]
        unsafe { self.0.load_khr_maintenance1(instance); }
        #[cfg(feature = "nn_vi_surface")]
        unsafe { self.0.load_nn_vi_surface(instance); }
        #[cfg(feature = "ext_direct_mode_display")]
        unsafe { self.0.load_ext_direct_mode_display(instance); }
        #[cfg(feature = "ext_acquire_xlib_display")]
        unsafe { self.0.load_ext_acquire_xlib_display(instance); }
        #[cfg(feature = "ext_display_surface_counter")]
        unsafe { self.0.load_ext_display_surface_counter(instance); }
        #[cfg(feature = "ext_display_control")]
        unsafe { self.0.load_ext_display_control(instance); }
        #[cfg(feature = "khr_push_descriptor")]
        unsafe { self.0.load_khr_push_descriptor(instance); }
        #[cfg(feature = "khr_descriptor_update_template")]
        unsafe { self.0.load_khr_descriptor_update_template(instance); }
        #[cfg(feature = "khx_device_group")]
        unsafe { self.0.load_khx_device_group(instance); }
        #[cfg(feature = "khx_device_group_creation")]
        unsafe { self.0.load_khx_device_group_creation(instance); }
        #[cfg(feature = "khx_external_memory_capabilities")]
        unsafe { self.0.load_khx_external_memory_capabilities(instance); }
        #[cfg(feature = "khx_external_memory_win32")]
        unsafe { self.0.load_khx_external_memory_win32(instance); }
        #[cfg(feature = "khx_external_memory_fd")]
        unsafe { self.0.load_khx_external_memory_fd(instance); }
        #[cfg(feature = "khx_external_semaphore_capabilities")]
        unsafe { self.0.load_khx_external_semaphore_capabilities(instance); }
        #[cfg(feature = "khx_external_semaphore_win32")]
        unsafe { self.0.load_khx_external_semaphore_win32(instance); }
        #[cfg(feature = "khx_external_semaphore_fd")]
        unsafe { self.0.load_khx_external_semaphore_fd(instance); }
        #[cfg(feature = "nv_clip_space_w_scaling")]
        unsafe { self.0.load_nv_clip_space_w_scaling(instance); }
        #[cfg(feature = "ext_discard_rectangles")]
        unsafe { self.0.load_ext_discard_rectangles(instance); }
        #[cfg(feature = "mvk_ios_surface")]
        unsafe { self.0.load_mvk_ios_surface(instance); }
        #[cfg(feature = "mvk_macos_surface")]
        unsafe { self.0.load_mvk_macos_surface(instance); }
        #[cfg(feature = "google_display_timing")]
        unsafe { self.0.load_google_display_timing(instance); }
        #[cfg(feature = "ext_hdr_metadata")]
        unsafe { self.0.load_ext_hdr_metadata(instance); }
        #[cfg(feature = "khr_swapchain")]
        unsafe { self.0.load_khr_swapchain(instance); }
    }
}
