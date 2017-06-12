
use vks::VkPresentModeKHR;

pub struct PresentMode(VkPresentModeKHR); // u32

impl From<VkPresentModeKHR> for PresentMode {
    fn from(vk: VkPresentModeKHR) -> PresentMode {
        PresentMode(vk)
    }
}

impl Into<VkPresentModeKHR> for PresentMode {
    fn into(self) -> VkPresentModeKHR {
        self.0
    }
}
