# Notes

- Vulkan har 2 forskellige resource typer. VkImage (ID3D11Texture) og VkBuffer. D3D12 har kun 1, ID3D12Resource, men når den allokeres sættes dens dimension, hvor den enten er en buffer eller en af de forskellige textures

- Et VkRenderPass beskriver hvilke ressourcer en VkCommandBuffer bruger.
- 1 CommandAllocator og CommandList per thread.
- 1 Graphics Queue og flere async compute.

Devices and CommandQueues are free-threaded meaning they can be used from multiple threads simoultanesly. 

CommandList cannot outlive their CommandAllocator.
Only 1 CommandList can be in a recording state with the same CommandAllocator. This means two CommandList allocated with a CommandAllocator cannot both be recording at the same time.

# Abstraktion

- Instance
- Device
- CommandQueue <- Mapper til grafikkortets funktionalitet. Executor commandlists
- CommandAllocator <- Hukommelse der bruges til at allokere Commands.
- CommandList <- En liste af Commands, der kan sendes til Grafikkortet.
- Swapchain