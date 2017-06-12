
use vks::VkImageUsageFlags;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ImageUsageFlags: u32 {
        const IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001;
        const IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002;
        const IMAGE_USAGE_SAMPLED_BIT = 0x00000004;
        const IMAGE_USAGE_STORAGE_BIT = 0x00000008;
        const IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010;
        const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020;
        const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040;
        const IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080;
    }
}

impl From<VkImageUsageFlags> for ImageUsageFlags {
    fn from(vk: VkImageUsageFlags) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(vk.bits()).unwrap()
    }
}

impl Into<VkImageUsageFlags> for ImageUsageFlags {
    fn into(self) -> VkImageUsageFlags {
        VkImageUsageFlags::from_bits(self.bits()).unwrap()
    }
}
