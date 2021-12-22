use super::vk;

/// Various implementations of backend specific adapters.
pub enum Adapter<'a> {
    Vulkan(vk::VkAdapter<'a>),
}

unsafe impl<'a> AdapterApi for Adapter<'a> {
    fn description(&self) -> String {
        match self {
            Self::Vulkan(adapter) => adapter.description(),
        }
    }

    fn software(&self) -> bool {
        match self {
            Self::Vulkan(adapter) => adapter.software(),
        }
    }
}

pub unsafe trait AdapterApi {
    /// Returns a vendor specific text describing the adapter.
    fn description(&self) -> String;

    /// Returns true if the adapter is a software based one. Otherwise returns false.
    fn software(&self) -> bool;
}
