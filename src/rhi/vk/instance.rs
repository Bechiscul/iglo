use crate::rhi;
use ash::{
    self,
    vk::{self, ExtensionProperties, InstanceCreateInfo, LayerProperties},
    Entry,
};
use std::{ffi::CString, fmt::Debug};

pub struct VkInstance {
    entry: ash::Entry,
    instance: ash::Instance,
    debug: bool,
    validation: bool,
}

impl Debug for VkInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "VkInstance {{ entry: _, instance: _, debug: {:?}, validation: {:?} }}",
            self.debug, self.validation
        ))
    }
}

impl VkInstance {
    const EXTENSIONS: [&'static str; 2] = ["VK_KHR_surface", "VK_KHR_win32_surface"];
    const LAYERS: [&'static str; 0] = [];

    pub fn new(debug: bool, validation: bool) -> Result<Self, rhi::InstanceError> {
        let entry = match unsafe { Entry::new() } {
            Ok(entry) => entry,
            _ => return Err(rhi::InstanceError {}),
        };

        let all_extensions = entry.enumerate_instance_extension_properties().unwrap();
        let all_layers = entry.enumerate_instance_layer_properties().unwrap();

        let extension_names = Self::EXTENSIONS.as_slice();
        if !Self::has_extensions(extension_names, &all_extensions) {
            return Err(rhi::InstanceError {});
        }

        let layer_names = Self::LAYERS.as_slice();
        if !Self::has_layers(layer_names, &all_layers) {
            return Err(rhi::InstanceError {});
        }

        let mut extensions =
            unsafe { Self::find_extensions(extension_names, &all_extensions).unwrap_unchecked() };
        if debug && Self::has_extension("VK_KHR_debug_utils", &all_extensions) {
            let extension = Self::find_extension("VK_KHR_debug_utils", &all_extensions);
            unsafe { extensions.push(extension.unwrap_unchecked()) };
        }

        let mut layers = unsafe { Self::find_layers(layer_names, &all_layers).unwrap() };
        if validation && Self::has_layer("VK_LAYER_KHRONOS_validation", &all_layers) {
            let layer = Self::find_layer("VK_LAYER_KHRONOS_validation", &all_layers);
            unsafe { layers.push(layer.unwrap_unchecked()) };
        }

        let extensions: Vec<*const i8> = extensions
            .iter()
            .map(|extension| extension.extension_name.as_ptr())
            .collect();

        let layers: Vec<*const i8> = layers
            .iter()
            .map(|layer| layer.layer_name.as_ptr())
            .collect();

        let create_info = InstanceCreateInfo {
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_layer_count: layers.len() as u32,
            pp_enabled_layer_names: layers.as_ptr(),
            ..Default::default()
        };

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => Ok(Self {
                entry,
                instance,
                debug,
                validation,
            }),
            _ => Err(rhi::InstanceError {}),
        }
    }

    fn find_extension<'a>(
        name: &str,
        extensions: &'a Vec<ExtensionProperties>,
    ) -> Option<&'a ExtensionProperties> {
        extensions.iter().find(|v| {
            // Safe since Vulkan guarantees VkExtensionProperties::extensionName is a null-terminated UTF8-string.
            let extension_name = unsafe { std::ffi::CStr::from_ptr(v.extension_name.as_ptr()) };
            extension_name.to_bytes() == name.as_bytes()
        })
    }

    fn has_extension<'a>(name: &str, extensions: &'a Vec<ExtensionProperties>) -> bool {
        Self::find_extension(name, extensions).is_some()
    }

    fn find_extensions<'a>(
        names: &[&str],
        extensions: &'a Vec<ExtensionProperties>,
    ) -> Option<Vec<&'a ExtensionProperties>> {
        let mut found_extensions = Vec::with_capacity(names.len());
        for &name in names {
            match Self::find_extension(name, extensions) {
                Some(extension) => found_extensions.push(extension),
                _ => {
                    return None;
                }
            }
        }

        Some(found_extensions)
    }

    fn has_extensions(names: &[&str], extensions: &Vec<ExtensionProperties>) -> bool {
        Self::find_extensions(names, extensions).is_some()
    }

    fn find_layer<'a>(name: &str, layers: &'a Vec<LayerProperties>) -> Option<&'a LayerProperties> {
        layers.iter().find(|v| {
            // Safe since Vulkan guarantees VkLayerProperties::layerName is a null-terminated UTF8-string.
            let layer_name = unsafe { std::ffi::CStr::from_ptr(v.layer_name.as_ptr()) };
            layer_name.to_bytes() == name.as_bytes()
        })
    }

    fn has_layer(name: &str, layers: &Vec<LayerProperties>) -> bool {
        Self::find_layer(name, layers).is_some()
    }

    fn find_layers<'a>(
        names: &[&str],
        layers: &'a Vec<LayerProperties>,
    ) -> Option<Vec<&'a LayerProperties>> {
        let mut found_layers = Vec::with_capacity(names.len());
        for &name in names {
            match Self::find_layer(name, layers) {
                Some(layer) => found_layers.push(layer),
                _ => {
                    return None;
                }
            }
        }

        Some(found_layers)
    }

    fn has_layers(names: &[&str], layers: &Vec<LayerProperties>) -> bool {
        Self::find_layers(names, layers).is_some()
    }
}

unsafe impl rhi::InstanceApi for VkInstance {
    fn backend(&self) -> rhi::Backend {
        rhi::Backend::Vulkan
    }

    fn adapters(&self) -> Vec<rhi::Adapter> {
        Vec::default()
    }
}
