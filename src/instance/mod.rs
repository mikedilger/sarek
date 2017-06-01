
mod physical_device;
pub use self::physical_device::{PhysicalDevice, PhysicalDeviceType, PhysicalDeviceLimits,
                                PhysicalDeviceProperties, PhysicalDeviceSparseProperties,
                                QueueFlags, QueueFlagBits, QueueFamilyProperties,
                                ExtensionProperties};

mod loader;
pub use self::loader::InstanceLoader;

mod debug;
pub use self::debug::DebugCallback;

use libc::c_char;
use std::ffi::CString;
use std::ptr;
use std::mem;
use vks::*;

use {Error, Version};

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
}


/// See vulkan specification, section 3.2 Instances
pub struct Instance(VkInstance);

impl Instance {
    #[cfg(not(feature = "ext_validation_flags"))]
    #[allow(unused_variables)]
    pub fn new(mut loader: InstanceLoader, create_info: InstanceCreateInfo)
               -> Result<(Instance, InstanceLoader), Error>
    {
        let extension_names = get_extension_names();

        // Setup strings for passing into vkCreateInstance down below.  These must
        // not go out of scope until after that function is called.
        let (extension_names_owned, extension_names) = {
            let mut extension_names_owned: Vec<CString> = Vec::new();
            for ref name in extension_names {
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

        let create_info = {
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

        let instance = unsafe {
            let mut instance: VkInstance = ptr::null_mut();
            vk_try!((loader.0.core_null_instance.vkCreateInstance)(
                &create_info,
                ptr::null(),
                &mut instance
            ));
            assert!(instance != ptr::null_mut());
            instance
        };

        // Load instance functions
        loader.load(instance)?;

        Ok((Instance(instance), loader))
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

impl Instance {
    pub fn enumerate_physical_devices(&self, loader: &InstanceLoader)
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

    pub fn get_debug_callback(&self, loader: InstanceLoader) -> Result<DebugCallback, Error>
    {
        DebugCallback::new(self.0, loader)
    }
}

#[allow(unused_mut)]
fn get_extension_names() -> Vec<&'static str>
{
    let mut extension_names: Vec<&'static str> = Vec::new();

    #[cfg(feature = "khr_surface")]
    extension_names.push(VK_KHR_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_swapchain")]
    extension_names.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_display")]
    extension_names.push(VK_KHR_DISPLAY_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_display_swapchain")]
    extension_names.push(VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_xlib_surface")]
    extension_names.push(VK_KHR_XLIB_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_xcb_surface")]
    extension_names.push(VK_KHR_XCB_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_wayland_surface")]
    extension_names.push(VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_mir_surface")]
    extension_names.push(VK_KHR_MIR_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_android_surface")]
    extension_names.push(VK_KHR_ANDROID_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_win32_surface")]
    extension_names.push(VK_KHR_WIN32_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_sampler_mirror_clamp_to_edge")]
    extension_names.push(VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_get_physical_device_properties2")]
    extension_names.push(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_shader_draw_parameters")]
    extension_names.push(VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_maintenance1")]
    extension_names.push(VK_KHR_MAINTENANCE1_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_push_descriptor")]
    extension_names.push(VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_incremental_present")]
    extension_names.push(VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME_STR);
    #[cfg(feature = "khr_descriptor_update_template")]
    extension_names.push(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_debug_report")]
    extension_names.push(VK_EXT_DEBUG_REPORT_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_glsl_shader")]
    extension_names.push(VK_NV_GLSL_SHADER_EXTENSION_NAME_STR);
    #[cfg(feature = "img_filter_cubic")]
    extension_names.push(VK_IMG_FILTER_CUBIC_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_rasterization_order")]
    extension_names.push(VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_shader_trinary_minmax")]
    extension_names.push(VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_shader_explicit_vertex_parameter")]
    extension_names.push(VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_debug_marker")]
    extension_names.push(VK_EXT_DEBUG_MARKER_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_gcn_shader")]
    extension_names.push(VK_AMD_GCN_SHADER_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_dedicated_allocation")]
    extension_names.push(VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_draw_indirect_count")]
    extension_names.push(VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_negative_viewport_height")]
    extension_names.push(VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_gpu_shader_half_float")]
    extension_names.push(VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME_STR);
    #[cfg(feature = "amd_shader_ballot")]
    extension_names.push(VK_AMD_SHADER_BALLOT_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_multiview")]
    extension_names.push(VK_KHX_MULTIVIEW_EXTENSION_NAME_STR);
    #[cfg(feature = "img_format_pvrtc")]
    extension_names.push(VK_IMG_FORMAT_PVRTC_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_external_memory_capabilities")]
    extension_names.push(VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_external_memory")]
    extension_names.push(VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_external_memory_win32")]
    extension_names.push(VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_win32_keyed_mutex")]
    extension_names.push(VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_device_group")]
    extension_names.push(VK_KHX_DEVICE_GROUP_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_validation_flags")]
    extension_names.push(VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME_STR);
    #[cfg(feature = "nn_vi_surface")]
    extension_names.push(VK_NN_VI_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_shader_subgroup_ballot")]
    extension_names.push(VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_shader_subgroup_vote")]
    extension_names.push(VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_device_group_creation")]
    extension_names.push(VK_KHX_DEVICE_GROUP_CREATION_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_memory_capabilities")]
    extension_names.push(VK_KHX_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_memory")]
    extension_names.push(VK_KHX_EXTERNAL_MEMORY_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_memory_win32")]
    extension_names.push(VK_KHX_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_memory_fd")]
    extension_names.push(VK_KHX_EXTERNAL_MEMORY_FD_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_win32_keyed_mutex")]
    extension_names.push(VK_KHX_WIN32_KEYED_MUTEX_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_semaphore_capabilities")]
    extension_names.push(VK_KHX_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_semaphore")]
    extension_names.push(VK_KHX_EXTERNAL_SEMAPHORE_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_semaphore_win32")]
    extension_names.push(VK_KHX_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME_STR);
    #[cfg(feature = "khx_external_semaphore_fd")]
    extension_names.push(VK_KHX_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME_STR);
    #[cfg(feature = "nvx_device_generated_commands")]
    extension_names.push(VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_clip_space_w_scaling")]
    extension_names.push(VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_direct_mode_display")]
    extension_names.push(VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_acquire_xlib_display")]
    extension_names.push(VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_display_surface_counter")]
    extension_names.push(VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_display_control")]
    extension_names.push(VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME_STR);
    #[cfg(feature = "google_display_timing")]
    extension_names.push(VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_sample_mask_override_coverage")]
    extension_names.push(VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_geometry_shader_passthrough")]
    extension_names.push(VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_viewport_array2")]
    extension_names.push(VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME_STR);
    #[cfg(feature = "nvx_multiview_per_view_attributes")]
    extension_names.push(VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME_STR);
    #[cfg(feature = "nv_viewport_swizzle")]
    extension_names.push(VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_discard_rectangles")]
    extension_names.push(VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_swapchain_colorspace")]
    extension_names.push(VK_EXT_SWAPCHAIN_COLORSPACE_EXTENSION_NAME_STR);
    #[cfg(feature = "ext_hdr_metadata")]
    extension_names.push(VK_EXT_HDR_METADATA_EXTENSION_NAME_STR);
    #[cfg(feature = "mvk_ios_surface")]
    extension_names.push(VK_MVK_IOS_SURFACE_EXTENSION_NAME_STR);
    #[cfg(feature = "mvk_macos_surface")]
    extension_names.push(VK_MVK_MACOS_SURFACE_EXTENSION_NAME_STR);

    extension_names
}

#[cfg(feature = "ext_validation_flags")]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/instance/ext_validation_flags_1.rs"));
