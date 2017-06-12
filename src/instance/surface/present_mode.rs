
use std::mem;
use vks::VkPresentModeKHR;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PresentMode {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    #[cfg(feature = "khr_shared_presentable_image")]
    SharedDemandRefresh = 1000111000,
    #[cfg(feature = "khr_shared_presentable_image")]
    SharedContinuousRefresh = 1000111001,
}

impl From<VkPresentModeKHR> for PresentMode {
    fn from(vk: VkPresentModeKHR) -> PresentMode {
        unsafe {
            mem::transmute(vk.as_raw())
        }
    }
}

impl Into<VkPresentModeKHR> for PresentMode {
    fn into(self) -> VkPresentModeKHR {
        VkPresentModeKHR::from_raw(unsafe {
            mem::transmute(self)
        })
    }
}

impl Default for PresentMode {
    fn default() -> PresentMode {
        PresentMode::Immediate
    }
}
