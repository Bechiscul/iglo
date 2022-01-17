// TODO(Bech): Only enable if vulkan.
// pub mod vk;

// pub use adapter::*;
// pub use instance::*;
// pub use surface::*;

// mod adapter;
// mod instance;
// mod surface;

use crate::Version;

use bitflags::bitflags;

pub enum Backend {
    Vulkan,
}

pub enum Error {
    /// A layer wnas not present.
    LayerNotPresent,

    /// An extension was not present.
    ExtensionNotPresent,

    /// A feature was not present.
    FeatureNotPresent,

    /// Failed to allocate host memory.
    OutOfHostMemory,

    /// Failed to allocate device memory.
    OutOfDeviceMemory,

    BackendMismatch,

    /// The requested backend is not supported.
    NotSupported,

    /// The call failed due to invalid arguments or implementation specific reasons.
    Unknown,
}

pub struct ApplicationInfo<'a> {
    pub name: &'a str,
    pub version: Version,
}

bitflags! {
    pub struct InstanceFlags: u8 {
        const VALIDATION = 0b00000001;
        const DEBUG = 0b00000010;
    }
}
pub struct InstanceInfo<'a> {
    pub app_info: Option<ApplicationInfo<'a>>,
    pub flags: InstanceFlags,
}

impl<'a> Default for InstanceInfo<'a> {
    fn default() -> Self {
        Self {
            app_info: None,
            flags: InstanceFlags::VALIDATION | InstanceFlags::DEBUG,
        }
    }
}

pub trait InstanceApi: Sized {
    /// Creates a new instance with the preferred backend.
    ///
    /// If the preferred backend fails to be instantiated,
    /// the function retries with the next backend in the following priority list:
    ///
    /// - **Windows**: D3D12, Vulkan, D3D11, OpenGL
    /// - **Linux**: Vulkan, OpenGL
    /// - **Mac**: Metal
    ///
    /// To only check a single backend use [`InstanceApi::with_backend()`]
    ///
    /// **Note**: The above assumes all backends are enabled.
    ///
    /// # Arguments
    ///
    /// `info` - Info about the instance that is passed to the implementation.
    fn new(info: &InstanceInfo) -> Result<Self, Error> {
        Self::with_backend(Backend::Vulkan, info)
    }

    /// Creates a new instance with the supplied backend.
    ///
    /// Unlike [`InstanceApi::new()`] this function only tries to instantiate with the passed in backend.
    ///
    /// # Arguments
    ///
    /// - `backend` - The backend to use for instantiation.
    /// - `info` - Info about the instance that is passed to the implementation.
    fn with_backend(backend: Backend, info: &InstanceInfo) -> Result<Self, Error>;

    /// Returns an iterator over all the adapters that support the minimum requirements.
    ///
    /// The iterators length is always > 0, because a successfull instantation requires at least
    /// 1 supported adapter. If none was found creation fails with [`Error::NotSupported`].
    fn enumerate_adapters<T: Iterator<Item = impl AdapterApi>>(&self) -> T;

    /// Creates a new surface.
    fn new_surface(&self) -> Result<Surface, Error>;
}

pub trait AdapterApi: Sized {}

pub enum Surface {}
