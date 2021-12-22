#![feature(derive_default_enum)]

pub mod os;
pub mod rhi;

pub struct Version {
    major: u8,
    minor: u8,
    patch: u16,
}

pub fn version() -> Version {
    let v: Vec<f32> = Vec::with_capacity(0);
    // let x = NonNull::<f32>::dangling();

    Version {
        major: 0,
        minor: 0,
        patch: 0,
    }
}
