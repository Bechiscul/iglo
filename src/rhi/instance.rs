use super::vk;
use super::Adapter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InstanceError;

/// A structure containing all the backends for the current target_os.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Vulkan,
}

/// An enum containing all the different variants of an instance.
#[derive(Debug)]
pub enum Instance {
    Vulkan(vk::VkInstance),
}

impl Instance {
    /// Tries to create a new instance with the preferred backend for the current platform.
    ///
    /// # Remarks
    /// Enables debugging and validation when not compiled in release.
    ///
    /// # Examples
    ///
    /// ```
    /// let instance = Instance::new()?;
    /// ```
    pub fn new() -> Result<Self, InstanceError> {
        Self::with_backend(Backend::Vulkan)
    }

    /// Creates a new instance with the supplied backend.
    ///
    /// # Arguments
    ///
    /// * `backend` - Which backend to use.
    ///
    /// # Examples
    ///
    /// ```
    /// let instance = Instance::with_backend(Backend::Vulkan)?;
    /// ```
    pub fn with_backend(backend: Backend) -> Result<Self, InstanceError> {
        match backend {
            Backend::Vulkan => Ok(Instance::Vulkan(vk::VkInstance::new(true, true)?)),
            _ => Err(InstanceError {}),
        }
    }
}

unsafe impl InstanceApi for Instance {
    fn backend(&self) -> Backend {
        match self {
            Instance::Vulkan(instance) => instance.backend(),
        }
    }

    fn adapters(&self) -> Vec<Adapter> {
        match self {
            Instance::Vulkan(instance) => instance.adapters(),
        }
    }
}

pub unsafe trait InstanceApi {
    /// Returns the currently used backend.
    ///
    /// # Examples
    ///
    /// ```
    /// let instance = Instance::new()?;
    /// println!("Backend: {}", instance.backend());
    /// ```
    fn backend(&self) -> Backend;

    /// Returns all adapters that supports the current backend and it's required extensions and layers.
    ///
    /// # Remarks
    /// It is guaranteed that the the returned array contains atleast one element, because Instance creation fails if there aren't any valid adapters.
    ///
    /// # Examples
    ///
    /// ```
    /// let instance = Instance::new()?;
    /// let adapter = unsafe { instance.adapters().get_unchecked(0) };
    /// println!("{}", adapter.description());
    /// ```
    fn adapters(&self) -> Vec<Adapter>;
}
