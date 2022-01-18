use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use ash::prelude::*;
use ash::{self, extensions::*, vk};

use crate::rhi::{
    ApplicationInfo, Error, InstanceApi, InstanceFlags, InstanceInfo, IntoError, IntoResult, Result,
};
use crate::Version;

impl Into<Error> for vk::Result {
    fn into(self) -> Error {
        match self {
            vk::Result::ERROR_OUT_OF_HOST_MEMORY => Error::OutOfHostMemory,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => Error::OutOfDeviceMemory,
            vk::Result::ERROR_INITIALIZATION_FAILED => Error::Unknown,
            vk::Result::ERROR_DEVICE_LOST => Error::DeviceLost,
            vk::Result::ERROR_LAYER_NOT_PRESENT => Error::LayerNotPresent,
            vk::Result::ERROR_EXTENSION_NOT_PRESENT => Error::ExtensionNotPresent,
            vk::Result::ERROR_FEATURE_NOT_PRESENT => Error::FeatureNotPresent,
            vk::Result::ERROR_INCOMPATIBLE_DRIVER => Error::NotSupported,
            vk::Result::ERROR_UNKNOWN => Error::Unknown,
            _ => Error::Unknown,
        }
    }
}

impl Into<u32> for Version {
    fn into(self) -> u32 {
        vk::make_api_version(0, self.major.into(), self.minor.into(), self.patch.into())
    }
}

struct InstanceShared {
    entry: ash::Entry,
    instance: ash::Instance,
    physical_devices: Vec<vk::PhysicalDevice>,
    debug_utils: Option<(ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

impl Drop for InstanceShared {
    fn drop(&mut self) {
        if let Some((extension, messenger)) = self.debug_utils.take() {
            unsafe { extension.destroy_debug_utils_messenger(messenger, None) };
        }

        unsafe { self.instance.destroy_instance(None) };
    }
}

pub struct Instance {
    inner: Arc<InstanceShared>,
}

impl Instance {
    pub fn new(info: &InstanceInfo) -> Result<Self> {
        let entry = match unsafe { ash::Entry::load() } {
            Ok(entry) => entry,
            _ => return Err(Error::NotSupported),
        };

        let application_info: vk::ApplicationInfo;
        let application_name: CString;

        let mut create_info = match &info.app_info {
            Some(app) => {
                application_name = CString::new(app.name.to_owned()).unwrap();
                application_info = vk::ApplicationInfo {
                    p_application_name: application_name.as_ptr(),
                    application_version: app.version.into(),
                    ..Default::default()
                };

                vk::InstanceCreateInfo::builder().application_info(&application_info)
            }
            _ => vk::InstanceCreateInfo::builder(),
        };

        let layers = Vec::from([]);
        let extensions: Vec<*const i8> = [khr::Surface::name(), khr::Win32Surface::name()]
            .into_iter()
            .map(|ext| ext.as_ptr())
            .collect();

        create_info = create_info
            .enabled_layer_names(layers.as_slice())
            .enabled_extension_names(extensions.as_slice());

        let instance = unsafe { entry.create_instance(&create_info, None) }.map_err(Into::into)?;
        let physical_devices: Vec<vk::PhysicalDevice> =
            Self::enumerate_physical_devices(&instance)?
                .into_iter()
                .filter(|pd| Self::is_physical_device_supported(pd))
                .collect();

        Err(Error::Unknown)
    }

    fn enumerate_physical_devices(instance: &ash::Instance) -> Result<Vec<vk::PhysicalDevice>> {
        Ok(unsafe { instance.enumerate_physical_devices() }.map_err(Into::into)?)
    }

    fn is_physical_device_supported(physical_device: &vk::PhysicalDevice) -> bool {
        true
    }
}
