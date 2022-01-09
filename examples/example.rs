use std::{borrow::Borrow, ops::Deref, rc::Rc, sync::Arc};

use iglo::{os::Window, rhi::*};

struct Renderer<'a> {
    window: &'a Window,
    instance: Instance,
    surface: Surface<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(window: &'a Window) -> Option<Self> {
        let instance_info = InstanceInfo {
            app_info: None,
            debug: true,
            validation: true,
        };

        let instance = Instance::new(&instance_info).unwrap();
        let surface = instance.new_surface(window).unwrap();

        Some(Self {
            window,
            instance,
            surface,
        })
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
