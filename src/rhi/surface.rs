use enum_dispatch::enum_dispatch;

use super::vk::VkSurface;

// TODO(Bech): TextureFormat.

#[enum_dispatch]
pub trait SurfaceApi: Send {
    // fn format(&self) -> TextureFormat;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceError {
    OutOfMemory,
    NotSupported,
    Unknown,
}

#[enum_dispatch(SurfaceApi)]
pub enum Surface<'a> {
    Vk(VkSurface<'a>),
}
