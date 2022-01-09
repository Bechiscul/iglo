use enum_dispatch::enum_dispatch;

use super::{Surface, SurfaceError};

#[enum_dispatch]
pub trait AdapterApi: Send {
    /// Returns info about the adapter itself.
    fn info(&self) -> AdapterInfo;

    /// Returns whether the surface may present on this adapter.
    ///
    /// # Arguments
    ///
    /// - `surface` - The surface, presentation support is checked on.
    ///
    fn is_surface_supported(&self, surface: &Surface) -> Result<bool, SurfaceError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdapterInfo<'a> {
    name: &'a str,
}

pub enum Adapter {}
