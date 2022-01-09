use ash::vk;

use crate::rhi::{AdapterApi, AdapterInfo, Surface, SurfaceError};

use super::{VkInstance, VkInstanceApi, VkSurface, VkSurfaceApi};

pub trait VkAdapterApi {}

#[derive(Clone, Copy)]
pub struct VkAdapter<'a> {
    instance: &'a VkInstance,
    handle: &'a vk::PhysicalDevice,
}

impl<'a> AdapterApi for VkAdapter<'a> {
    fn info(&self) -> AdapterInfo {
        todo!()
    }

    fn is_surface_supported(&self, surface: &Surface) -> Result<bool, SurfaceError> {
        let surface: &VkSurface = surface.try_into()?;

        // SAFETY: This is safe because we don't store the extension.
        let ext = unsafe { surface.extension() };

        // SAFETY: This is safe because we don't store the handle.
        let handle = unsafe { surface.handle() };

        match unsafe { ext.get_physical_device_surface_support(*self.handle, 0, *handle) } {
            Ok(support) => Ok(support),
            _ => Err(SurfaceError::Unknown),
        }
    }
}
