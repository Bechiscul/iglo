use std::marker::PhantomData;

use ash::vk::PhysicalDevice;

use crate::rhi::{AdapterApi, Instance};

#[derive(Debug)]
pub struct VkAdapter<'a> {
    physical_device: PhysicalDevice,
    _marker: PhantomData<&'a Instance>,
}

unsafe impl<'a> VkAdapterApi for VkAdapter<'a> {
    unsafe fn physical_device(&self) -> PhysicalDevice {
        self.physical_device
    }
}

unsafe impl<'a> AdapterApi for VkAdapter<'a> {
    fn description(&self) -> String {
        "".to_string()
    }

    fn software(&self) -> bool {
        false
    }
}

pub unsafe trait VkAdapterApi {
    unsafe fn physical_device(&self) -> PhysicalDevice;
}
