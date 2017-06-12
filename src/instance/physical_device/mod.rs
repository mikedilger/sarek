
mod physical_device_type;
pub use self::physical_device_type::PhysicalDeviceType;

use std::mem;
use std::str;
use std::ptr;
use std::ffi::CStr;
use vks::*;
use {Error, Version, InstanceLoader};
use {DeviceSize, SampleCountFlags, Bool32, Extent3D};
#[cfg(feature = "khr_surface")]
use instance::{Surface};

#[cfg(feature = "khr_surface")]
pub type SurfaceFormat = VkSurfaceFormatKHR;
#[cfg(feature = "khr_surface")]
pub type SurfaceCapabilities = VkSurfaceCapabilitiesKHR;
#[cfg(feature = "khr_surface")]
pub type PresentMode = VkPresentModeKHR;

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

/// See vulkan specification, section 4.1 Physical Devices
#[repr(C)] // laid out exactly like VkPhysicalDeviceLimits so we can just transmute
#[derive(Debug, Clone)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bios: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}

impl From<VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
    fn from(vk: VkPhysicalDeviceLimits) -> PhysicalDeviceLimits {
        assert_eq!(mem::size_of::<VkPhysicalDeviceLimits>(),
                   mem::size_of::<PhysicalDeviceLimits>());
        unsafe {
            mem::transmute(vk)
        }
    }
}

/// See vulkan specification, section 4.1 Physical Devices
// consider using a bit vector in a u8
#[derive(Debug, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: Bool32,
    pub residency_standard_2d_multisample_block_shape: Bool32,
    pub residency_standard_3d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

impl From<VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
    fn from(vk: VkPhysicalDeviceSparseProperties) -> PhysicalDeviceSparseProperties {
        assert_eq!(mem::size_of::<VkPhysicalDeviceSparseProperties>(),
                   mem::size_of::<PhysicalDeviceSparseProperties>());
        unsafe {
            mem::transmute(vk)
        }
    }
}

pub type QueueFlags = VkQueueFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum QueueFlagBits {
    GraphicsBit = 1,
    ComputeBit = 2,
    TransferBit = 4,
    SparseBindingBit = 8,
}

#[derive(Debug, Clone)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}
impl From<VkQueueFamilyProperties> for QueueFamilyProperties {
    fn from(vk: VkQueueFamilyProperties) -> QueueFamilyProperties {
        assert_eq!(mem::size_of::<VkQueueFamilyProperties>(),
                   mem::size_of::<QueueFamilyProperties>());
        unsafe {
            mem::transmute(vk)
        }
    }
}

/// See vulkan specification, section 4.1 Physical Devices
#[derive(Debug, Clone)]
pub struct PhysicalDeviceProperties {
    pub api_version: Version,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; VK_UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

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

/// See vulkan specification, section 31.1 Features
#[repr(C)] // laid out exactly like VkPhysicalDeviceFeatures so we can just transmute
#[derive(Debug, Clone)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: u32,
    pub full_draw_index_uint32: u32,
    pub image_cube_array: u32,
    pub independent_blend: u32,
    pub geometry_shader: u32,
    pub tessellation_shader: u32,
    pub sample_rate_shading: u32,
    pub dual_src_blend: u32,
    pub logic_op: u32,
    pub multi_draw_indirect: u32,
    pub draw_indirect_first_instance: u32,
    pub depth_clamp: u32,
    pub depth_bias_clamp: u32,
    pub fill_mode_non_solid: u32,
    pub depth_bounds: u32,
    pub wide_lines: u32,
    pub large_points: u32,
    pub alpha_to_one: u32,
    pub multi_viewport: u32,
    pub sampler_anisotropy: u32,
    pub texture_compression_etc2: u32,
    pub texture_compression_astc_ldr: u32,
    pub texture_compression_bc: u32,
    pub occlusion_query_precise: u32,
    pub pipeline_statistics_query: u32,
    pub vertex_pipeline_stores_and_atomics: u32,
    pub fragment_stores_and_atomics: u32,
    pub shader_tessellation_and_geometry_point_size: u32,
    pub shader_image_gather_extended: u32,
    pub shader_storage_image_extended_formats: u32,
    pub shader_storage_image_multisample: u32,
    pub shader_storage_image_read_without_format: u32,
    pub shader_storage_image_write_without_format: u32,
    pub shader_uniform_buffer_array_dynamic_indexing: u32,
    pub shader_sampled_image_array_dynamic_indexing: u32,
    pub shader_storage_buffer_array_dynamic_indexing: u32,
    pub shader_storage_image_array_dynamic_indexing: u32,
    pub shader_clip_distance: u32,
    pub shader_cull_distance: u32,
    pub shader_float64: u32,
    pub shader_int64: u32,
    pub shader_int16: u32,
    pub shader_resource_residency: u32,
    pub shader_resource_min_lod: u32,
    pub sparse_binding: u32,
    pub sparse_residency_buffer: u32,
    pub sparse_residency_image2d: u32,
    pub sparse_residency_image3d: u32,
    pub sparse_residency2_samples: u32,
    pub sparse_residency4_samples: u32,
    pub sparse_residency8_samples: u32,
    pub sparse_residency16_samples: u32,
    pub sparse_residency_aliased: u32,
    pub variable_multisample_rate: u32,
    pub inherited_queries: u32,
}

impl From<VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn from(vk: VkPhysicalDeviceFeatures) -> PhysicalDeviceFeatures {
        assert_eq!(mem::size_of::<VkPhysicalDeviceFeatures>(),
                   mem::size_of::<PhysicalDeviceFeatures>());
        unsafe {
            mem::transmute(vk)
        }
    }
}

impl PhysicalDeviceFeatures {
    pub fn into_vk(self) -> VkPhysicalDeviceFeatures {
        assert_eq!(mem::size_of::<VkPhysicalDeviceFeatures>(),
                   mem::size_of::<PhysicalDeviceFeatures>());
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Default for PhysicalDeviceFeatures {
    fn default() -> PhysicalDeviceFeatures {
        let vk: VkPhysicalDeviceFeatures = Default::default();
        From::from(vk)
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

        Ok(PhysicalDeviceProperties {
            api_version: Version::from_vk(properties.apiVersion),
            driver_version: properties.driverVersion,
            vendor_id: properties.vendorID,
            device_id: properties.deviceID,
            device_type: unsafe { mem::transmute(properties.deviceType) },
            device_name:  unsafe {
                str::from_utf8(
                    CStr::from_ptr(
                        properties.deviceName.as_ptr())
                        .to_bytes())?
                .to_owned()
            },
            pipeline_cache_uuid: properties.pipelineCacheUUID.clone(),
            limits: From::from(properties.limits),
            sparse_properties: From::from(properties.sparseProperties),
        })
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
        let surface_formats = unsafe {
            let ptr = surface_formats.as_mut_ptr();
            mem::forget(surface_formats);
            Vec::from_raw_parts(ptr, count as usize, capacity as usize)
        };

        // FIXME: Translate for output

        Ok(surface_formats)
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
        Ok(capabilities)
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
        let present_modes = unsafe {
            let ptr = present_modes.as_mut_ptr();
            mem::forget(present_modes);
            Vec::from_raw_parts(ptr, count as usize, count as usize)
        };

        // FIXME: Translate for output
        Ok(present_modes)
    }
}

#[cfg(feature = "khr_get_physical_device_properties2")]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/instance/physical_device/khr_get_physical_device_properties2_1.rs"));
