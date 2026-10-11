use crate::prelude::*;

pub trait VulkanSubbufferExt {
    fn as_descriptor_buffer_info(&self) -> VulkanDescriptorBufferInfo<'_>;
}

impl<T: ?Sized> VulkanSubbufferExt for VulkanSubbuffer<T> {
    fn as_descriptor_buffer_info(&self) -> VulkanDescriptorBufferInfo<'_> {
        VulkanDescriptorBufferInfo {
            buffer: Some(self.buffer()),
            offset: self.offset(),
            range: Some(self.size()),
        }
    }
}

#[inline]
pub fn vertex_stride<T: Copy>() -> u32 {
    core::mem::size_of::<T>()
        .next_multiple_of(core::mem::align_of::<T>())
        .try_into()
        .unwrap()
}

#[macro_export]
macro_rules! vertex_default_input_rate {
    (Vertex) => {
        $crate::prelude::VulkanVertexInputRate::Vertex
    };
    (Instance) => {
        $crate::prelude::VulkanVertexInputRate::Instance { divisor: 1 }
    };
}

#[macro_export]
macro_rules! define_vertex_bindings {
    ( $( $binding:literal => ($vertex_type:ty, $input_rate:ident) $(,)? )* ) => {
        ::core::iter::FromIterator::from_iter([
            $(
                (
                    $binding,
                    VulkanVertexInputBindingDescription {
                        stride: $crate::utils::vertex_stride::<$vertex_type>(),
                        input_rate: $crate::vertex_default_input_rate!($input_rate),
                        ..Default::default()
                    }
                )
            ),*
        ])
    };
}

#[macro_export]
macro_rules! define_vertex_attributes {
    ( $num_bindings:literal, [ $( [ $attribute:literal <- $binding:literal ] => $format:ident $(,)? )* ] ) => {{
        let mut __current_attribute_offsets: [u32; $num_bindings] = [0; $num_bindings];
        ::core::iter::FromIterator::from_iter([
            $(
                (
                    $attribute,
                    {
                        let __format = $crate::vulkano::format::Format::$format;
                        let __current_offset = __current_attribute_offsets[$binding];
                        let __block_size: u32 = __format.block_size().try_into().unwrap();
                        __current_attribute_offsets[$binding] += __block_size;
                        VulkanVertexInputAttributeDescription {
                            binding: $binding,
                            format: __format,
                            offset: __current_offset,
                            ..Default::default()
                        }
                    }
                )
            ),*
        ])
    }};
}

/// Creates a new buffer, and writes all elements of `iter` into it using a temporary CPU staging
/// buffer.
///
/// This is not particularly efficient, so is best used for long-lived GPU buffers.
///
/// [`VulkanBufferUsage::TRANSFER_DST`] will be automatically added to `create_info.usage`.
pub fn buffer_from_iter_staged<T, I>(
    queue: std::sync::Arc<VulkanQueue>,
    command_buffer_allocator: std::sync::Arc<dyn VulkanCommandBufferAllocator>,
    memory_allocator: &std::sync::Arc<dyn VulkanMemoryAllocator>,
    create_info: &VulkanBufferCreateInfo,
    allocation_info: &VulkanAllocationCreateInfo,
    iter: I,
) -> anyhow::Result<VulkanSubbuffer<[T]>>
where
    T: vulkano::buffer::BufferContents,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    use anyhow::Context;
    // Create temporary staging buffer with our data.
    let staging_buffer: VulkanSubbuffer<[T]> = VulkanBuffer::from_iter(
        memory_allocator,
        &VulkanBufferCreateInfo {
            usage: VulkanBufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        &VulkanAllocationCreateInfo {
            memory_type_filter: VulkanMemoryTypeFilter::PREFER_HOST
                | VulkanMemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        iter,
    )
    .context("Error while creating temporary staging buffer")?;
    // Create final buffer to be returned.
    let buffer: VulkanSubbuffer<[T]> = VulkanBuffer::new_slice(
        memory_allocator,
        &VulkanBufferCreateInfo {
            usage: create_info.usage | VulkanBufferUsage::TRANSFER_DST,
            ..*create_info
        },
        allocation_info,
        staging_buffer.len(),
    )
    .context("Error while creating return buffer")?;
    // Queue a copy from the staging buffer to the device buffer.
    let mut command_buffer = VulkanAutoCommandBufferBuilder::primary(
        command_buffer_allocator,
        queue.queue_family_index(),
        VulkanCommandBufferUsage::OneTimeSubmit,
    )
    .context("Error while creating command buffer builder")?;
    command_buffer
        .copy_buffer(VulkanCopyBufferInfoTyped::new(
            staging_buffer,
            buffer.clone(),
        ))
        .context("Error while recording buffer copy command")?;
    command_buffer
        .build()
        .context("Error while building copy command buffer")?
        .execute(queue)
        .context("Error while executing copy command buffer on queue")?
        .flush()
        .context("Error while flushing copy command buffer future")?;
    Ok(buffer)
}

/// Creates a new uninitialized buffer for a slice, with a specified alignment for the device
/// address.
/// Contains enough space to store `len` items of `T`.
pub fn new_buffer_slice_aligned<T: vulkano::buffer::BufferContents>(
    allocator: &std::sync::Arc<dyn VulkanMemoryAllocator>,
    create_info: &VulkanBufferCreateInfo,
    allocation_info: &VulkanAllocationCreateInfo,
    num_items: VulkanDeviceSize,
    alignment: VulkanDeviceAlignment,
) -> Result<VulkanSubbuffer<[T]>, VulkanValidated<vulkano::buffer::AllocateBufferError>> {
    let t_device_size: VulkanDeviceSize = core::mem::size_of::<T>().try_into().unwrap();
    let unaligned_byte_buffer = VulkanBuffer::new(
        allocator,
        create_info,
        allocation_info,
        VulkanDeviceLayout::from_size_alignment(
            t_device_size * num_items,
            VulkanDeviceSize::max(
                T::LAYOUT.alignment().as_devicesize(),
                alignment.as_devicesize(),
            ),
        )
        .unwrap(),
    )?;
    Ok(VulkanSubbuffer::new(unaligned_byte_buffer).cast_aligned())
}

/// Creates a buffer with a specified device address alignment, and writes all elements of `iter`
/// in it.
pub fn buffer_from_iter_aligned<T, I>(
    allocator: &std::sync::Arc<dyn VulkanMemoryAllocator>,
    create_info: &VulkanBufferCreateInfo,
    allocation_info: &VulkanAllocationCreateInfo,
    iter: I,
    alignment: VulkanDeviceAlignment,
) -> Result<VulkanSubbuffer<[T]>, VulkanValidated<vulkano::buffer::AllocateBufferError>>
where
    T: vulkano::buffer::BufferContents,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    let iter = iter.into_iter();
    let t_device_size: VulkanDeviceSize = core::mem::size_of::<T>().try_into().unwrap();
    let iter_len: VulkanDeviceSize = iter.len().try_into().unwrap();
    let unaligned_byte_buffer = VulkanBuffer::new(
        allocator,
        create_info,
        allocation_info,
        VulkanDeviceLayout::from_size_alignment(
            t_device_size * iter_len,
            VulkanDeviceSize::max(
                T::LAYOUT.alignment().as_devicesize(),
                alignment.as_devicesize(),
            ),
        )
        .unwrap(),
    )?;
    let aligned_buffer: VulkanSubbuffer<[T]> =
        VulkanSubbuffer::new(unaligned_byte_buffer).cast_aligned();
    // Write elements
    {
        let mut write_guard = aligned_buffer.write().unwrap();
        for (out_item, iter_item) in write_guard.iter_mut().zip(iter) {
            *out_item = iter_item;
        }
    }
    Ok(aligned_buffer)
}

/// Creates a new uninitialized buffer for a slice.
/// Contains enough space to store `len` items of `T`.
///
/// Always creates a dedicated device allocation, and so is designed for long-lived large
/// allocations (>=256MiB).
pub fn new_buffer_slice_large<T: vulkano::buffer::BufferContents>(
    device: &std::sync::Arc<VulkanDevice>,
    usage: VulkanBufferUsage,
    sharing: VulkanSharing,
    num_items: usize,
) -> Result<VulkanSubbuffer<[T]>, VulkanValidated<VulkanError>> {
    let num_items_device_size: VulkanDeviceSize = num_items.try_into().unwrap();
    let t_device_size: VulkanDeviceSize = core::mem::size_of::<T>().try_into().unwrap();
    let byte_size = num_items_device_size * t_device_size;
    let create_info = VulkanBufferCreateInfo {
        usage,
        size: byte_size,
        sharing,
        ..Default::default()
    };
    let raw_buffer = VulkanRawBuffer::new(device, &create_info)?;
    let memory_requirements = device.try_buffer_memory_requirements(&create_info)?;
    let allowed_memory_type_bits = memory_requirements.memory_type_bits;
    assert!(
        allowed_memory_type_bits != 0,
        "No suitable memory found for large buffer"
    );
    let memory_type_index = allowed_memory_type_bits.trailing_zeros();
    let memory_allocation = VulkanDeviceMemory::allocate(
        device,
        &VulkanMemoryAllocateInfo {
            allocation_size: byte_size,
            memory_type_index,
            dedicated_allocation: Some(VulkanDedicatedAllocation::Buffer(&raw_buffer)),
            ..Default::default()
        },
    )?;
    let resource_memory = VulkanResourceMemory::new_dedicated(memory_allocation);
    let buffer = raw_buffer
        .bind_memory(resource_memory)
        .map_err(|(err, _raw_buf, _resource_mem)| err)?;
    let subbuffer = VulkanSubbuffer::new(std::sync::Arc::new(buffer));
    Ok(subbuffer.reinterpret())
}
