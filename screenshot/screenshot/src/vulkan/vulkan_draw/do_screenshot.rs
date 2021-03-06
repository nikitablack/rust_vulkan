use ash::vk;
use vulkan_base::VulkanBase;

pub fn do_screenshot(
    vulkan_base: &mut VulkanBase,
    image: vk::Image,
    format: vk::Format,
    command_buffer: vk::CommandBuffer,
) -> Result<vulkan_utils::MemBuffer, String> {
    log::info!("do screenshot");

    let barrier = vk::ImageMemoryBarrier::builder()
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .old_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .src_access_mask(vk::AccessFlags::NONE_KHR)
        .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
        .image(image)
        .build();

    unsafe {
        vulkan_base.device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier],
        );
    }

    let extent = &vulkan_base.surface_extent;

    let region = vk::BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(
            vk::ImageSubresourceLayers::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .build(),
        )
        .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
        .image_extent(vk::Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        })
        .build();

    let block_size = vulkan_utils::get_format_block_size(format);

    let buffer = vulkan_utils::create_buffer(
        &vulkan_base.device,
        &mut vulkan_base.allocator,
        &vulkan_base.debug_utils_loader,
        (extent.width * extent.height * block_size as u32) as u64,
        vk::BufferUsageFlags::TRANSFER_DST,
        gpu_allocator::MemoryLocation::CpuToGpu,
        "screenshot buffer",
    )?;

    unsafe {
        vulkan_base.device.cmd_copy_image_to_buffer(
            command_buffer,
            image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            buffer.buffer,
            &[region],
        );
    }

    let barrier = vk::ImageMemoryBarrier::builder()
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .src_access_mask(vk::AccessFlags::TRANSFER_READ)
        .dst_access_mask(vk::AccessFlags::NONE_KHR)
        .image(image)
        .build();

    unsafe {
        vulkan_base.device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier],
        );
    }

    log::info!("screenshot done");

    Ok(buffer)
}
