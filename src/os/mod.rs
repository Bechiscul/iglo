#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod imp;

#[cfg(not(target_os = "windows"))]
std::compile_error!("Unsupported target platform");

pub struct Window(imp::Window);

impl Window {
    pub fn new() -> Self {
        Self(imp::Window::new().unwrap())
    }

    pub fn show(&self) {
        self.0.show()
    }

    pub fn poll_events(&self) {
        self.0.poll_events()
    }

    /// Returns access to the underlying platform specific window.
    pub fn platform_impl(&self) -> &imp::Window {
        &self.0
    }
}

pub trait WindowApi {
    fn show(&self);
    fn poll_events(&self);
}
