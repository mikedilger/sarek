

use std::mem;
use std::str;
use vk_sys::*;
use std::ffi::CStr;
use {Error, Version, InstanceLoader};
use {DeviceSize, SampleCountFlags, Bool32};

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
}

/// See vulkan specification, section 4.1 Physical Devices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGPU = 1,
    DiscreteGPU = 2,
    VirtualGPU = 3,
    CPU = 4
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
    residency_standard_2d_block_shape: Bool32,
    residency_standard_2d_multisample_block_shape: Bool32,
    residency_standard_3d_block_shape: Bool32,
    residency_aligned_mip_size: Bool32,
    residency_non_resident_strict: Bool32,
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

impl PhysicalDevice {
    pub fn get_properties(&self, loader: &mut InstanceLoader) ->
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
}
