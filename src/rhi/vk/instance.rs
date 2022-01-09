use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use ash::{extensions::*, vk};

use crate::{
    os::Window,
    rhi::{Adapter, Backend, InstanceApi, InstanceError, InstanceInfo, Surface, SurfaceError},
};

use super::VkSurface;

pub trait VkInstanceApi {
    /// Returns the entry that holds the global vulkan functions.
    fn entry(&self) -> &ash::Entry;

    /// Returns a handle to the vulkan instance.
    ///
    /// # Safety
    ///
    /// The handles lifetime is tied to the instance object
    /// and must not be used after the object has been dropped.
    unsafe fn handle(&self) -> &ash::Instance;

    /// Returns a handle to a vulkan debug utils messenger.
    ///
    /// # Safety
    ///
    /// The messengers lifetime is tied to the lifetime of the instance object
    /// and must not be used after this object has been dropped.
    unsafe fn debug_utils_messenger(&self) -> Option<&vk::DebugUtilsMessengerEXT>;

    /// Returns a handle to the loaded vulkan VkDebugUtils extension.
    ///
    /// # Safety
    ///
    /// The extensions lifetime is tied to the lifetime of the instance object
    /// and must not be used after this object has been dropped.
    unsafe fn debug_utils_extension(&self) -> Option<&ext::DebugUtils>;
}

pub struct VkInstanceInner {
    pub entry: ash::Entry,
    pub handle: ash::Instance,
    pub physical_devices: Vec<vk::PhysicalDevice>,
    pub debug_utils: Option<(vk::DebugUtilsMessengerEXT, ext::DebugUtils)>,
}

impl Drop for VkInstanceInner {
    fn drop(&mut self) {
        if let Some(debug_utils) = &self.debug_utils {
            let ext = &debug_utils.1;
            unsafe {
                ext.destroy_debug_utils_messenger(debug_utils.0, None);
            }
        }

        // SAFETY: Since we don't allow cloning or copying self, we know that we are the only owner of the instance,
        // unless someone used the internal API in the wrong way.
        // Therefore we are allowed to destroy the handles.
        unsafe { self.handle.destroy_instance(None) };
    }
}

pub struct VkInstance {
    inner: Arc<VkInstanceInner>,
}

impl VkInstance {
    // Warning(Bech): The layer- and extension names must all be null-terminated. This is done to make conversion to &CStr trivial.
    const ENABLED_LAYER_NAMES: [&'static str; 0] = [];
    const ENABLED_EXTENSION_NAMES: [&'static str; 2] =
        ["VK_KHR_surface\0", "VK_KHR_win32_surface\0"];

    pub fn new(info: &InstanceInfo) -> Result<Self, InstanceError> {
        // SAFETY: Since we are loading vulkan dynamically, we assume that it is implemented correctly.
        // There is really nothing we can do if it isn't.
        let entry = match unsafe { ash::Entry::load() } {
            Ok(entry) => entry,
            _ => return Err(InstanceError::NotSupported),
        };

        let application_name;
        let application_info;

        let create_info = match info.app_info {
            Some(app_info) => {
                application_name = CString::new(app_info.name.to_owned()).unwrap();

                let version = vk::make_api_version(
                    0,
                    app_info.version.major.into(),
                    app_info.version.minor.into(),
                    app_info.version.patch.into(),
                );

                application_info = vk::ApplicationInfo::builder()
                    .application_version(version)
                    .application_name(application_name.as_c_str());

                vk::InstanceCreateInfo::builder().application_info(&application_info)
            }
            _ => vk::InstanceCreateInfo::builder(),
        };

        // SAFETY: This is safe because all strings in Self::ENABLED_LAYER_NAMES are null-terminated.
        let mut enabled_layer_names: Vec<*const i8> = unsafe {
            Self::ENABLED_LAYER_NAMES
                .iter()
                .map(|s| CStr::from_bytes_with_nul_unchecked(s.as_bytes()).as_ptr())
                .collect()
        };

        const VALIDATION_LAYER_NAME: &str = "VK_LAYER_KHRONOS_validation\0";
        if info.validation && Self::has_layer(VALIDATION_LAYER_NAME, &entry)? {
            // SAFETY: This is safe because VALIDATION_LAYER_NAME is null-terminated and doesn't contain any interior null bytes.
            enabled_layer_names.push(unsafe {
                CStr::from_bytes_with_nul_unchecked(VALIDATION_LAYER_NAME.as_bytes()).as_ptr()
            });
        }

        // SAFETY: This is safe because all strings in Self::ENABLED_EXTENSION_NAMES are null-terminated.
        let mut enabled_extension_names: Vec<*const i8> = unsafe {
            Self::ENABLED_EXTENSION_NAMES
                .iter()
                .map(|s| CStr::from_bytes_with_nul_unchecked(s.as_bytes()).as_ptr())
                .collect()
        };

        const DEBUG_EXTENSION_NAME: &str = "VK_EXT_debug_utils\0";
        if info.debug && Self::has_extension(DEBUG_EXTENSION_NAME, &entry)? {
            // SAFETY: This is safe because DEBUG_EXTENSION_NAME is null-terminated.
            enabled_extension_names.push(unsafe {
                CStr::from_bytes_with_nul_unchecked(DEBUG_EXTENSION_NAME.as_bytes()).as_ptr()
            });
        }

        let create_info = create_info
            .enabled_layer_names(&enabled_layer_names)
            .enabled_extension_names(&enabled_extension_names);

        // SAFETY: We assume the vulkan implementation is implemented correctly.
        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => Ok(Self {
                inner: Arc::new(VkInstanceInner {
                    entry,
                    handle: instance,
                    physical_devices: vec![],
                    debug_utils: None,
                }),
            }),
            _ => Err(InstanceError::Unknown),
        }
    }

    fn has_layer(name: &str, entry: &ash::Entry) -> Result<bool, InstanceError> {
        let name = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        let layer_properties = entry.enumerate_instance_layer_properties();
        match layer_properties {
            Ok(layer_properties) => Ok(layer_properties
                .iter()
                .find(|ep| {
                    // SAFETY: This is safe because the vulkan specification states that VkLayerProperties::layerName is a null-terminated UTF-8 string.
                    let layer_name = unsafe { CStr::from_ptr(ep.layer_name.as_ptr()) };
                    layer_name == name
                })
                .is_some()),
            // TODO(Bech): Proper error handling.
            _ => Err(InstanceError::Unknown),
        }
    }

    fn has_extension(name: &str, entry: &ash::Entry) -> Result<bool, InstanceError> {
        let name = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        let extension_properties = entry.enumerate_instance_extension_properties();
        match extension_properties {
            Ok(extension_properties) => Ok(extension_properties
                .iter()
                .find(|ep| {
                    // SAFETY: This is safe because the vulkan specification states that VkExtensionProperties::extensionName is a null-terminated UTF-8 string.
                    let extension_name = unsafe { CStr::from_ptr(ep.extension_name.as_ptr()) };
                    extension_name == name
                })
                .is_some()),
            // TODO(Bech): Proper error handling.
            _ => Err(InstanceError::Unknown),
        }
    }
}

impl VkInstanceApi for VkInstance {
    fn entry(&self) -> &ash::Entry {
        &self.inner.entry
    }

    unsafe fn handle(&self) -> &ash::Instance {
        &self.inner.handle
    }

    unsafe fn debug_utils_messenger(&self) -> Option<&vk::DebugUtilsMessengerEXT> {
        match &self.inner.debug_utils {
            Some(debug_utils) => Some(&debug_utils.0),
            _ => None,
        }
    }

    unsafe fn debug_utils_extension(&self) -> Option<&ext::DebugUtils> {
        match &self.inner.debug_utils {
            Some(debug_utils) => Some(&debug_utils.1),
            _ => None,
        }
    }
}

impl InstanceApi for VkInstance {
    fn backend(&self) -> Backend {
        Backend::Vulkan
    }

    fn new_surface<'a>(&self, window: &'a Window) -> Result<Surface<'a>, SurfaceError> {
        Ok(Surface::Vk(VkSurface::new(
            Arc::clone(&self.inner),
            window,
        )?))
    }

    fn enumerate_adapters<T: ExactSizeIterator<Item = Adapter>>(&self) -> T {
        todo!()
    }
}
