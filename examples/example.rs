use iglo::{os::Window, rhi::*};

struct Renderer<'a> {
    window: &'a Window,
    instance: Instance,
}

impl<'a> Renderer<'a> {
    pub fn new(window: &'a Window) -> Option<Self> {
        let instance_info = InstanceInfo {
            app_info: None,
            debug: true,
            validation: true,
        };

        let instance = Instance::new(&instance_info).ok()?;
        Some(Self { window, instance })
    }
}

fn main() {
    let window = Window::new();
    window.show();
    let _renderer = Renderer::new(&window).expect("Failed to create renderer!");

    loop {
        window.poll_events()
    }
}
