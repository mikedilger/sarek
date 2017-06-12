
use std::mem;
use vks::{VkFormat, VkFormatFeatureFlags, VkFormatProperties};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    Undefined = 0,
    R4g4UnormPack8 = 1,
    R4g4b4a4UnormPack16 = 2,
    B4g4r4a4UnormPack16 = 3,
    R5g6b5UnormPack16 = 4,
    B5g6r5UnormPack16 = 5,
    R5g5b5a1UnormPack16 = 6,
    B5g5r5a1UnormPack16 = 7,
    A1r5g5b5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8Uscaled = 11,
    R8Sscaled = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8g8Unorm = 16,
    R8g8Snorm = 17,
    R8g8Uscaled = 18,
    R8g8Sscaled = 19,
    R8g8Uint = 20,
    R8g8Sint = 21,
    R8g8Srgb = 22,
    R8g8b8Unorm = 23,
    R8g8b8Snorm = 24,
    R8g8b8Uscaled = 25,
    R8g8b8Sscaled = 26,
    R8g8b8Uint = 27,
    R8g8b8Sint = 28,
    R8g8b8Srgb = 29,
    B8g8r8Unorm = 30,
    B8g8r8Snorm = 31,
    B8g8r8Uscaled = 32,
    B8g8r8Sscaled = 33,
    B8g8r8Uint = 34,
    B8g8r8Sint = 35,
    B8g8r8Srgb = 36,
    R8g8b8a8Unorm = 37,
    R8g8b8a8Snorm = 38,
    R8g8b8a8Uscaled = 39,
    R8g8b8a8Sscaled = 40,
    R8g8b8a8Uint = 41,
    R8g8b8a8Sint = 42,
    R8g8b8a8Srgb = 43,
    B8g8r8a8Unorm = 44,
    B8g8r8a8Snorm = 45,
    B8g8r8a8Uscaled = 46,
    B8g8r8a8Sscaled = 47,
    B8g8r8a8Uint = 48,
    B8g8r8a8Sint = 49,
    B8g8r8a8Srgb = 50,
    A8b8g8r8UnormPack32 = 51,
    A8b8g8r8SnormPack32 = 52,
    A8b8g8r8UscaledPack32 = 53,
    A8b8g8r8SscaledPack32 = 54,
    A8b8g8r8UintPack32 = 55,
    A8b8g8r8SintPack32 = 56,
    A8b8g8r8SrgbPack32 = 57,
    A2r10g10b10UnormPack32 = 58,
    A2r10g10b10SnormPack32 = 59,
    A2r10g10b10UscaledPack32 = 60,
    A2r10g10b10SscaledPack32 = 61,
    A2r10g10b10UintPack32 = 62,
    A2r10g10b10SintPack32 = 63,
    A2b10g10r10UnormPack32 = 64,
    A2b10g10r10SnormPack32 = 65,
    A2b10g10r10UscaledPack32 = 66,
    A2b10g10r10SscaledPack32 = 67,
    A2b10g10r10UintPack32 = 68,
    A2b10g10r10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16g16Unorm = 77,
    R16g16Snorm = 78,
    R16g16Uscaled = 79,
    R16g16Sscaled = 80,
    R16g16Uint = 81,
    R16g16Sint = 82,
    R16g16Sfloat = 83,
    R16g16b16Unorm = 84,
    R16g16b16Snorm = 85,
    R16g16b16Uscaled = 86,
    R16g16b16Sscaled = 87,
    R16g16b16Uint = 88,
    R16g16b16Sint = 89,
    R16g16b16Sfloat = 90,
    R16g16b16a16Unorm = 91,
    R16g16b16a16Snorm = 92,
    R16g16b16a16Uscaled = 93,
    R16g16b16a16Sscaled = 94,
    R16g16b16a16Uint = 95,
    R16g16b16a16Sint = 96,
    R16g16b16a16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32g32Uint = 101,
    R32g32Sint = 102,
    R32g32Sfloat = 103,
    R32g32b32Uint = 104,
    R32g32b32Sint = 105,
    R32g32b32Sfloat = 106,
    R32g32b32a32Uint = 107,
    R32g32b32a32Sint = 108,
    R32g32b32a32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64g64Uint = 113,
    R64g64Sint = 114,
    R64g64Sfloat = 115,
    R64g64b64Uint = 116,
    R64g64b64Sint = 117,
    R64g64b64Sfloat = 118,
    R64g64b64a64Uint = 119,
    R64g64b64a64Sint = 120,
    R64g64b64a64Sfloat = 121,
    B10g11r11UfloatPack32 = 122,
    E5b9g9r9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    Bc1RgbUnormBlock = 131,
    Bc1RgbSrgbBlock = 132,
    Bc1RgbaUnormBlock = 133,
    Bc1RgbaSrgbBlock = 134,
    Bc2UnormBlock = 135,
    Bc2SrgbBlock = 136,
    Bc3UnormBlock = 137,
    Bc3SrgbBlock = 138,
    Bc4UnormBlock = 139,
    Bc4SnormBlock = 140,
    Bc5UnormBlock = 141,
    Bc5SnormBlock = 142,
    Bc6hUfloatBlock = 143,
    Bc6hSfloatBlock = 144,
    Bc7UnormBlock = 145,
    Bc7SrgbBlock = 146,
    Etc2R8g8b8UnormBlock = 147,
    Etc2R8g8b8SrgbBlock = 148,
    Etc2R8g8b8a1UnormBlock = 149,
    Etc2R8g8b8a1SrgbBlock = 150,
    Etc2R8g8b8a8UnormBlock = 151,
    Etc2R8g8b8a8SrgbBlock = 152,
    EacR11UnormBlock = 153,
    EacR11SnormBlock = 154,
    EacR11g11UnormBlock = 155,
    EacR11g11SnormBlock = 156,
    Astc4x4UnormBlock = 157,
    Astc4x4SrgbBlock = 158,
    Astc5x4UnormBlock = 159,
    Astc5x4SrgbBlock = 160,
    Astc5x5UnormBlock = 161,
    Astc5x5SrgbBlock = 162,
    Astc6x5UnormBlock = 163,
    Astc6x5SrgbBlock = 164,
    Astc6x6UnormBlock = 165,
    Astc6x6SrgbBlock = 166,
    Astc8x5UnormBlock = 167,
    Astc8x5SrgbBlock = 168,
    Astc8x6UnormBlock = 169,
    Astc8x6SrgbBlock = 170,
    Astc8x8UnormBlock = 171,
    Astc8x8SrgbBlock = 172,
    Astc10x5UnormBlock = 173,
    Astc10x5SrgbBlock = 174,
    Astc10x6UnormBlock = 175,
    Astc10x6SrgbBlock = 176,
    Astc10x8UnormBlock = 177,
    Astc10x8SrgbBlock = 178,
    Astc10x10UnormBlock = 179,
    Astc10x10SrgbBlock = 180,
    Astc12x10UnormBlock = 181,
    Astc12x10SrgbBlock = 182,
    Astc12x12UnormBlock = 183,
    Astc12x12SrgbBlock = 184,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc12bppUnormBlockImg = 1000054000,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc14bppUnormBlockImg = 1000054001,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc22bppUnormBlockImg = 1000054002,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc24bppUnormBlockImg = 1000054003,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc12bppSrgbBlockImg = 1000054004,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc14bppSrgbBlockImg = 1000054005,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc22bppSrgbBlockImg = 1000054006,
    #[cfg(feature = "img_format_pvrtc")]
    Pvrtc24bppSrgbBlockImg = 1000054007,
}

impl From<VkFormat> for Format {
    fn from(vk: VkFormat) -> Format {
        unsafe {
            mem::transmute(vk.as_raw())
        }
    }
}

impl Into<VkFormat> for Format {
    fn into(self) -> VkFormat {
        VkFormat::from_raw(unsafe {
            mem::transmute(self)
        })
    }
}

impl Default for Format {
    fn default() -> Format {
        Format::Undefined
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct FormatFeatureFlags: u32 {
        const FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001;
        const FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002;
        const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004;
        const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008;
        const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010;
        const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020;
        const FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040;
        const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080;
        const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100;
        const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200;
        const FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400;
        const FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800;
        const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000;
        #[cfg(feature = "img_filter_cubic")]
        const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000;
        #[cfg(feature = "khr_maintenance1")]
        const FORMAT_FEATURE_TRANSFER_SRC_BIT = 0x00004000;
        #[cfg(feature = "khr_maintenance1")]
        const FORMAT_FEATURE_TRANSFER_DST_BIT = 0x00008000;
    }
}

impl From<VkFormatFeatureFlags> for FormatFeatureFlags {
    fn from(vk: VkFormatFeatureFlags) -> FormatFeatureFlags {
        FormatFeatureFlags::from_bits(vk.bits()).unwrap()
    }
}

impl Into<VkFormatFeatureFlags> for FormatFeatureFlags {
    fn into(self) -> VkFormatFeatureFlags {
        VkFormatFeatureFlags::from_bits(self.bits()).unwrap()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}

impl From<VkFormatProperties> for FormatProperties {
    fn from(vk: VkFormatProperties) -> FormatProperties {
        unsafe {
            mem::transmute(vk)
        }
    }
}

impl Into<VkFormatProperties> for FormatProperties {
    fn into(self) -> VkFormatProperties {
        unsafe {
            mem::transmute(self)
        }
    }
}
