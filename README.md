# Iglo

Iglo is a highly performant game engine framework written in pure [Rust](https://rustlang.org).

- [Getting Started](https://iglo.dev/blog/getting-started)

- [Documentation](https://iglo.dev/docs)

## Renderer

<!-- image -->

### Renderer Hardware Interface

Low-level API-agnostic interface, which goal is to provide as close to zero-cost abstractions over [DirectX 12]() and [Vulkan]() as possible.

#### Backends

| Platform | D3D12 | Vulkan | Metal |
| -------- | ----- | ------ | ----- |
| Windows  | :x:   | WIP    | -     |
| Linux    | -     | :x:    | -     |
| MacOs    | -     | -      | :x:   |

## References

- [Halcyon Architecture](https://media.contentapi.ea.com/content/dam/ea/seed/presentations/wihlidal-halcyonarchitecture-notes.pdf)

- [Efficient Rendering Algorithms](http://www.aortiz.me/2018/12/21/CG.html)

- [GPU-Driven Engines](https://vkguide.dev/docs/gpudriven/gpu_driven_engines/)

- [Advanced Scenegraph Rendering Pipeline](https://on-demand.gputechconf.com/gtc/2013/presentations/S3032-Advanced-Scenegraph-Rendering-Pipeline.pdf)

- [Practical DirectX 12](https://developer.nvidia.com/sites/default/files/akamai/gameworks/blog/GDC16/GDC16_gthomas_adunn_Practical_DX12.pdf)

- [Vulkan Multi-Threading](https://developer.nvidia.com/sites/default/files/akamai/gameworks/blog/munich/mschott_vulkan_multi_threading.pdf)