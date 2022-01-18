// TODO(Bech): Only enable if vulkan.
pub mod vk;

use crate::Version;

use bitflags::bitflags;

pub enum Backend {
    Vulkan,
}

pub trait IntoError: Sized {
    fn into_error(self) -> Error;
}

pub enum Error {
    /// Failed to allocate host memory.
    OutOfHostMemory,

    /// Failed to allocate device memory.
    OutOfDeviceMemory,

    /// Device lost.
    DeviceLost,

    /// A layer was not present.
    LayerNotPresent,

    /// An extension was not present.
    ExtensionNotPresent,

    /// A feature was not present.
    FeatureNotPresent,

    BackendMismatch,

    /// The requested backend is not supported.
    NotSupported,

    /// The call failed due to invalid arguments or implementation specific reasons.
    Unknown,
}

pub trait IntoResult<T>: Sized {
    fn into_result(self) -> Result<T>;
}

pub type Result<T> = std::result::Result<T, Error>;

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

pub trait InstanceApi: Send + Sync + Sized {
    /// Returns an iterator over all the adapters that support the minimum requirements.
    ///
    /// The iterators length is always > 0, because a successfull instantation requires at least
    /// 1 supported adapter. If none was found creation fails with [`Error::NotSupported`].
    fn enumerate_adapters<T: Iterator<Item = impl AdapterApi>>(&self) -> T;

    /// Creates a new surface.
    fn new_surface(&self) -> Result<Surface>;
}

pub enum Instance {}

impl Instance {
    /// Creates a new instance with the preferred backend.
    ///
    /// If the preferred backend fails to be instantiated,
    /// the function retries with the next backend in the following priority list:
    ///
    /// - **Windows**: D3D12, Vulkan, D3D11, OpenGL
    /// - **Linux**: Vulkan, OpenGL
    /// - **Mac**: Metal
    ///
    /// To only check a single backend use [`Instance::with_backend()`]
    ///
    /// **Note**: The above assumes all backends are enabled.
    ///
    /// # Arguments
    ///
    /// `info` - Info about the instance that is passed to the implementation.
    pub fn new(info: &InstanceInfo) -> Result<Self> {
        todo!()
    }

    /// Creates a new instance with the supplied backend.
    ///
    /// Unlike [`Instance::new()`] this function only tries to instantiate with the passed in backend.
    ///
    /// # Arguments
    ///
    /// - `backend` - The backend to use for instantiation.
    /// - `info` - Info about the instance that is passed to the implementation.
    pub fn with_backend(backend: Backend, info: &InstanceInfo) -> Result<Self> {
        todo!()
    }
}

pub trait AdapterApi: Sized {}

pub enum Surface {}
