enum TextureDimension {
    Texture1D,
    Texture2D,
    Texture3D,
}

enum Texture {
    Vulkan(vk::Texture),
}

/// A view into a vertex buffer
enum VertexBufferView {
    Vulkan(vk::VertexBufferView),
}

/// An owner of an contiguous array of GPU memory.
enum Buffer {
    Vulkan(vk::Buffer),
}

impl Buffer {
    pub fn new(resource: Resource) -> Option<Self> {
        match resource.dimension() {
            ResourceDimension::Buffer => unsafe { Self::new_unchecked(resource) },
            _ => None,
        }
    }

    pub unsafe fn new_unchecked(resource: Resource) -> Self {
        Self(resource)
    }
}

impl BufferApi for Buffer {}

pub struct VertexBuffer<'a> {
    ptr: &'a Buffer,
    view: &'a VertexBufferView,
}

impl<'a> VertexBuffer<'a> {
    pub unsafe fn from_view(buffer: &'a Buffer, view: VertexBuffer) -> Self {
        Self { ptr: buffer, view }
    }

    pub fn as_buffer(&self) -> &'a Buffer {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn stride(&self) -> usize {
        self.stride
    }
}

pub trait BufferApi {}
