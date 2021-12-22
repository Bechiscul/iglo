// use super::{vk, Backend, Instance, InstanceApi};

// pub struct DeviceError {}

// pub enum Device {
//     Vulkan(vk::Device),
// }

// impl Device {
//     pub fn new(instance: &Instance) -> Result<Self, ()> {
//         match instance.backend() {
//             Backend::Vulkan => Ok(Device::Vulkan(vk::Device::new(instance)?)),
//         }
//     }

//     // pub fn with_adapter(adapter: Adapter) -> Result<Self, ()>;
// }

// unsafe impl DeviceApi for Device {}

// pub unsafe trait DeviceApi {
//     fn create_command_queue(&self) -> Result<CommandQueue, DeviceError>;
//     fn create_command_allocator(&self) -> Result<CommandAllocator, DeviceError>;
//     fn create_command_list(&self, alloc: &CommandAllocator) -> Result<CommandList, DeviceError>;
//     fn create_buffer(&self) -> Result<Buffer, DeviceError>;
//     fn create_buffer_view(&self) -> Result<BufferView, DeviceError>;
//     fn create_texture(&self) -> Result<Texture, DeviceError>;
//     fn create_texture_view(&self) -> Result<TextureView, DeviceError>;
//     fn create_graphics_pipeline(&self) -> Result<GraphicsPipeline, DeviceError>;
//     fn create_compute_pipeline(&self) -> Result<ComputePipeline, DeviceError>;
// }
