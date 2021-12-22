use iglo::{os::Window, rhi};

fn main() {
    let window = Window::new();
    window.show();

    let instance = rhi::Instance::new().unwrap();

    loop {
        window.poll_events()
    }
}
