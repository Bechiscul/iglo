use enum_dispatch::enum_dispatch;

use crate::{os::Window, Version};

use super::{vk::VkInstance, Adapter, Surface, SurfaceError};

#[enum_dispatch]
pub trait InstanceApi: Send + Sync {
    /// Returns the currently used backend.
    fn backend(&self) -> Backend;

    /// Creates a new surface.
    fn new_surface<'a>(&self, window: &'a Window) -> Result<Surface<'a>, SurfaceError>;

    /// Returns all the adapters that supports the minimum required limits.
    fn enumerate_adapters<T: ExactSizeIterator<Item = Adapter>>(&self) -> T;
}

// /// An object created from an instance.
// #[enum_dispatch]
// pub trait InstanceChild {
//     /// Returns the instance used to create the object.
//     fn instance(&self) -> InstanceRef;
// }

// /// Opaque reference to an instance.
// #[enum_dispatch(InstanceApi)]
// pub enum InstanceRef<'a> {
//     Vk(&'a VkInstance),
// }

/// Opaque owned object to an instance.
#[enum_dispatch(InstanceApi)]
pub enum Instance {
    Vk(VkInstance),
}

impl Instance {
    pub fn new(info: &InstanceInfo) -> Result<Instance, InstanceError> {
        Ok(Self::Vk(VkInstance::new(info)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Vulkan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceError {
    LayerNotPresent,
    ExtensionNotPresent,
    OutOfMemory,
    NotSupported,
    Unknown,
}

/// Information passed to the driver about the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationInfo<'a> {
    /// The name of the application.
    pub name: &'a str,

    /// The version of the application.
    pub version: Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanceInfo<'a> {
    /// Optional information about the application.
    pub app_info: Option<ApplicationInfo<'a>>,

    /// Whether to enable validation. This is strongly recommend for debug builds.
    pub validation: bool,

    /// Whether to enable debugging capabilites. This is recommend for debug builds.
    pub debug: bool,
}
