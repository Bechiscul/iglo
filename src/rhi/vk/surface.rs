use std::{borrow::Borrow, marker::PhantomData, sync::Arc};

use ash::{extensions::khr, vk};

use crate::{
    os::Window,
    rhi::{Surface, SurfaceApi, SurfaceError},
};

use super::{VkInstance, VkInstanceApi, VkInstanceInner};

// TODO(Bech): Documentation.
pub trait VkSurfaceApi {
    fn instance(&self) -> &Arc<VkInstanceInner>;
    unsafe fn handle(&self) -> &vk::SurfaceKHR;
    unsafe fn extension(&self) -> &khr::Surface;
}

pub struct VkSurface<'a> {
    instance: Arc<VkInstanceInner>,
    handle: vk::SurfaceKHR,
    extension: khr::Surface,
    _marker: PhantomData<&'a Window>,
}

impl<'a> VkSurface<'a> {
    pub fn new(instance: Arc<VkInstanceInner>, window: &'a Window) -> Result<Self, SurfaceError> {
        let extension = khr::Surface::new(&instance.entry, &instance.handle);

        #[cfg(target_os = "windows")]
        let handle = Self::new_win32_surface(&instance, window)?;

        Ok(Self {
            instance,
            handle,
            extension,
            _marker: PhantomData,
        })
    }

    #[cfg(target_os = "windows")]
    fn new_win32_surface(
        instance: &VkInstanceInner,
        win: &Window,
    ) -> Result<vk::SurfaceKHR, SurfaceError> {
        use std::ffi::c_void;
        let extension = khr::Win32Surface::new(&instance.entry, &instance.handle);

        let create_info = vk::Win32SurfaceCreateInfoKHR {
            hinstance: *win.platform_impl().hinstance() as *const c_void,
            hwnd: *win.platform_impl().hwnd() as *const c_void,
            ..Default::default()
        };

        // SAFETY: We are calling vulkan over FFI which we assume is safe.
        match unsafe { extension.create_win32_surface(&create_info, None) } {
            Ok(surface) => Ok(surface),
            _ => Err(SurfaceError::Unknown),
        }
    }
}

impl<'a> Drop for VkSurface<'a> {
    fn drop(&mut self) {
        // SAFETY:
        // This is safe because we are the only owner of the surface handle and it is not destroyed unless someone used the internal API in the wrong way.
        unsafe { self.extension.destroy_surface(self.handle, None) }
    }
}

impl<'a> SurfaceApi for VkSurface<'a> {}

impl<'a> VkSurfaceApi for VkSurface<'a> {
    fn instance(&self) -> &Arc<VkInstanceInner> {
        &self.instance
    }

    unsafe fn handle(&self) -> &vk::SurfaceKHR {
        &self.handle
    }

    unsafe fn extension(&self) -> &khr::Surface {
        &self.extension
    }
}

// TODO(Bech): Error propagation.
impl<'a> TryFrom<&'a Surface<'a>> for &'a VkSurface<'a> {
    type Error = SurfaceError;
    fn try_from(value: &'a Surface<'a>) -> Result<Self, Self::Error> {
        match value {
            Surface::Vk(value) => Ok(value),
            _ => Err(SurfaceError::Unknown),
        }
    }
}
