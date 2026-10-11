//! Re-exports most types from [`vulkano`] as a flat namespace with a "Vulkan" prefix.
//!
//! Intended to be imported as `use sleepy_vulkano::prelude::*`.

pub use vulkano::acceleration_structure::{
    AccelerationStructure as VulkanAccelerationStructure,
    AccelerationStructureBuildGeometryInfo as VulkanAccelerationStructureBuildGeometryInfo,
    AccelerationStructureBuildRangeInfo as VulkanAccelerationStructureBuildRangeInfo,
    AccelerationStructureBuildType as VulkanAccelerationStructureBuildType,
    AccelerationStructureCreateFlags as VulkanAccelerationStructureCreateFlags,
    AccelerationStructureCreateInfo as VulkanAccelerationStructureCreateInfo,
    AccelerationStructureGeometry as VulkanAccelerationStructureGeometry,
    AccelerationStructureGeometryAabbsData as VulkanAccelerationStructureGeometryAabbsData,
    AccelerationStructureGeometryInstancesData as VulkanAccelerationStructureGeometryInstancesData,
    AccelerationStructureGeometryTrianglesData as VulkanAccelerationStructureGeometryTrianglesData,
    AccelerationStructureInstance as VulkanAccelerationStructureInstance,
    AccelerationStructureType as VulkanAccelerationStructureType,
    BuildAccelerationStructureFlags as VulkanBuildAccelerationStructureFlags,
    BuildAccelerationStructureMode as VulkanBuildAccelerationStructureMode,
    GeometryFlags as VulkanGeometryFlags, GeometryInstanceFlags as VulkanGeometryInstanceFlags,
};
pub use vulkano::buffer::view::{
    BufferView as VulkanBufferView, BufferViewCreateInfo as VulkanBufferViewCreateInfo,
};
pub use vulkano::buffer::{
    Buffer as VulkanBuffer, BufferCreateFlags as VulkanBufferCreateFlags,
    BufferCreateInfo as VulkanBufferCreateInfo, BufferUsage as VulkanBufferUsage,
    IndexBuffer as VulkanIndexBuffer, RawBuffer as VulkanRawBuffer, Subbuffer as VulkanSubbuffer,
};
pub use vulkano::command_buffer::allocator::{
    CommandBufferAllocator as VulkanCommandBufferAllocator,
    StandardCommandBufferAllocator as VulkanStandardCommandBufferAllocator,
};
pub use vulkano::command_buffer::auto::{
    AutoCommandBufferBuilder as VulkanAutoCommandBufferBuilder,
    PrimaryAutoCommandBuffer as VulkanPrimaryAutoCommandBuffer,
};
pub use vulkano::command_buffer::{
    BufferCopy as VulkanBufferCopy, BufferImageCopy as VulkanBufferImageCopy,
    CommandBufferBeginInfo as VulkanCommandBufferBeginInfo,
    CommandBufferLevel as VulkanCommandBufferLevel,
    CommandBufferSubmitInfo as VulkanCommandBufferSubmitInfo,
    CommandBufferUsage as VulkanCommandBufferUsage, CopyBufferInfo as VulkanCopyBufferInfo,
    CopyBufferInfoTyped as VulkanCopyBufferInfoTyped,
    CopyBufferToImageInfo as VulkanCopyBufferToImageInfo,
    DrawIndexedIndirectCommand as VulkanDrawIndexedIndirectCommand,
    DrawIndirectCommand as VulkanDrawIndirectCommand,
    PrimaryCommandBufferAbstract as VulkanPrimaryCommandBufferAbstract,
    RecordingCommandBuffer as VulkanRecordingCommandBuffer,
    RenderPassBeginInfo as VulkanRenderPassBeginInfo,
    SemaphoreSubmitInfo as VulkanSemaphoreSubmitInfo, SubmitInfo as VulkanSubmitInfo,
    SubpassBeginInfo as VulkanSubpassBeginInfo, SubpassContents as VulkanSubpassContents,
    SubpassEndInfo as VulkanSubpassEndInfo,
};
pub use vulkano::descriptor_set::allocator::{
    DescriptorSetAllocator as VulkanDescriptorSetAllocator,
    StandardDescriptorSetAllocator as VulkanStandardDescriptorSetAllocator,
    StandardDescriptorSetAllocatorCreateInfo as VulkanStandardDescriptorSetAllocatorCreateInfo,
};
pub use vulkano::descriptor_set::layout::{
    DescriptorBindingFlags as VulkanDescriptorBindingFlags,
    DescriptorSetLayout as VulkanDescriptorSetLayout,
    DescriptorSetLayoutBinding as VulkanDescriptorSetLayoutBinding,
    DescriptorSetLayoutCreateFlags as VulkanDescriptorSetLayoutCreateFlags,
    DescriptorSetLayoutCreateInfo as VulkanDescriptorSetLayoutCreateInfo,
    DescriptorType as VulkanDescriptorType,
};
pub use vulkano::descriptor_set::{
    CopyDescriptorSet as VulkanCopyDescriptorSet,
    DescriptorBufferInfo as VulkanDescriptorBufferInfo,
    DescriptorImageInfo as VulkanDescriptorImageInfo, DescriptorSet as VulkanDescriptorSet,
    WriteDescriptorSet as VulkanWriteDescriptorSet,
};
pub use vulkano::device::physical::PhysicalDeviceType as VulkanPhysicalDeviceType;
pub use vulkano::device::{
    Device as VulkanDevice, DeviceCreateInfo as VulkanDeviceCreateInfo,
    DeviceExtensions as VulkanDeviceExtensions, DeviceFeatures as VulkanDeviceFeatures,
    Queue as VulkanQueue, QueueCreateInfo as VulkanQueueCreateInfo, QueueFlags as VulkanQueueFlags,
};
pub use vulkano::format::Format as VulkanFormat;
pub use vulkano::image::sampler::{
    Filter as VulkanFilter, Sampler as VulkanSampler,
    SamplerAddressMode as VulkanSamplerAddressMode, SamplerCreateInfo as VulkanSamplerCreateInfo,
    SamplerMipmapMode as VulkanSamplerMipmapMode,
};
pub use vulkano::image::view::ImageView as VulkanImageView;
pub use vulkano::image::{
    Image as VulkanImage, ImageAspects as VulkanImageAspects,
    ImageCreateInfo as VulkanImageCreateInfo, ImageLayout as VulkanImageLayout,
    ImageSubresourceLayers as VulkanImageSubresourceLayers, ImageType as VulkanImageType,
    ImageUsage as VulkanImageUsage,
};
pub use vulkano::instance::debug::{
    DebugUtilsMessageSeverity as VulkanDebugUtilsMessageSeverity,
    DebugUtilsMessageType as VulkanDebugUtilsMessageType,
    DebugUtilsMessenger as VulkanDebugUtilsMessenger,
    DebugUtilsMessengerCallback as VulkanDebugUtilsMessengerCallback,
    DebugUtilsMessengerCreateInfo as VulkanDebugUtilsMessengerCreateInfo,
    ValidationFeatureEnable as VulkanValidationFeatureEnable,
};
pub use vulkano::instance::{
    Instance as VulkanInstance, InstanceCreateInfo as VulkanInstanceCreateInfo,
    InstanceExtensions as VulkanInstanceExtensions,
};
pub use vulkano::memory::allocator::{
    AllocationCreateInfo as VulkanAllocationCreateInfo, DeviceLayout as VulkanDeviceLayout,
    MemoryAllocatePreference as VulkanMemoryAllocatePreference,
    MemoryAllocator as VulkanMemoryAllocator, MemoryTypeFilter as VulkanMemoryTypeFilter,
    StandardMemoryAllocator as VulkanStandardMemoryAllocator,
};
pub use vulkano::memory::{
    DedicatedAllocation as VulkanDedicatedAllocation, DeviceAlignment as VulkanDeviceAlignment,
    DeviceMemory as VulkanDeviceMemory, MemoryAllocateInfo as VulkanMemoryAllocateInfo,
    ResourceMemory as VulkanResourceMemory,
};
pub use vulkano::pipeline::compute::ComputePipelineCreateInfo as VulkanComputePipelineCreateInfo;
pub use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo as VulkanGraphicsPipelineCreateInfo;
pub use vulkano::pipeline::graphics::color_blend::{
    AttachmentBlend as VulkanAttachmentBlend, BlendFactor as VulkanBlendFactor,
    BlendOp as VulkanBlendOp, ColorBlendAttachmentState as VulkanColorBlendAttachmentState,
    ColorBlendState as VulkanColorBlendState, ColorBlendStateFlags as VulkanColorBlendStateFlags,
    ColorComponents as VulkanColorComponents, LogicOp as VulkanLogicOp,
};
pub use vulkano::pipeline::graphics::depth_stencil::{
    CompareOp as VulkanCompareOp, DepthState as VulkanDepthState,
    DepthStencilState as VulkanDepthStencilState,
    DepthStencilStateFlags as VulkanDepthStencilStateFlags,
};
pub use vulkano::pipeline::graphics::input_assembly::{
    InputAssemblyState as VulkanInputAssemblyState, PrimitiveTopology as VulkanPrimitiveTopology,
};
pub use vulkano::pipeline::graphics::multisample::MultisampleState as VulkanMultisampleState;
pub use vulkano::pipeline::graphics::rasterization::{
    CullMode as VulkanCullMode, RasterizationState as VulkanRasterizationState,
};
pub use vulkano::pipeline::graphics::subpass::{
    PipelineRenderingCreateInfo as VulkanPipelineRenderingCreateInfo,
    PipelineSubpassType as VulkanPipelineSubpassType,
};
pub use vulkano::pipeline::graphics::vertex_input::{
    VertexInputAttributeDescription as VulkanVertexInputAttributeDescription,
    VertexInputBindingDescription as VulkanVertexInputBindingDescription,
    VertexInputRate as VulkanVertexInputRate, VertexInputState as VulkanVertexInputState,
};
pub use vulkano::pipeline::graphics::viewport::{
    Scissor as VulkanScissor, Viewport as VulkanViewport, ViewportState as VulkanViewportState,
};
pub use vulkano::pipeline::layout::{
    PipelineLayoutCreateFlags as VulkanPipelineLayoutCreateFlags,
    PipelineLayoutCreateInfo as VulkanPipelineLayoutCreateInfo,
    PushConstantRange as VulkanPushConstantRange,
};
pub use vulkano::pipeline::{
    ComputePipeline as VulkanComputePipeline, DynamicState as VulkanDynamicState,
    GraphicsPipeline as VulkanGraphicsPipeline, PipelineBindPoint as VulkanPipelineBindPoint,
    PipelineCreateFlags as VulkanPipelineCreateFlags, PipelineLayout as VulkanPipelineLayout,
    PipelineShaderStageCreateFlags as VulkanPipelineShaderStageCreateFlags,
    PipelineShaderStageCreateInfo as VulkanPipelineShaderStageCreateInfo,
};
pub use vulkano::render_pass::{
    Framebuffer as VulkanFramebuffer, FramebufferCreateFlags as VulkanFramebufferCreateFlags,
    FramebufferCreateInfo as VulkanFramebufferCreateInfo, RenderPass as VulkanRenderPass,
    Subpass as VulkanSubpass,
};
pub use vulkano::shader::{
    EntryPoint as VulkanEntryPoint, ShaderModule as VulkanShaderModule,
    ShaderModuleCreateInfo as VulkanShaderModuleCreateInfo, ShaderStage as VulkanShaderStage,
    ShaderStages as VulkanShaderStages,
};
pub use vulkano::swapchain::{
    AcquireNextImageInfo as VulkanAcquireNextImageInfo, AcquiredImage as VulkanAcquiredImage,
    PresentInfo as VulkanPresentInfo, PresentMode as VulkanPresentMode,
    SemaphorePresentInfo as VulkanSemaphorePresentInfo, Surface as VulkanSurface,
    Swapchain as VulkanSwapchain, SwapchainCreateInfo as VulkanSwapchainCreateInfo,
    SwapchainPresentInfo as VulkanSwapchainPresentInfo,
    acquire_next_image as vulkan_swapchain_acquire_next_image,
};
pub use vulkano::sync::fence::Fence as VulkanFence;
pub use vulkano::sync::semaphore::Semaphore as VulkanSemaphore;
pub use vulkano::sync::{GpuFuture, Sharing as VulkanSharing};
pub use vulkano::{
    DeviceSize as VulkanDeviceSize, Packed24_8 as VulkanPacked24_8, Validated as VulkanValidated,
    VulkanError, VulkanLibrary, VulkanObject,
    single_pass_renderpass as vulkan_single_pass_renderpass,
};
