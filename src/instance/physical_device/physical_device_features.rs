
use std::mem;
use vks::VkPhysicalDeviceFeatures;

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
