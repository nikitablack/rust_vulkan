use ash::vk;

pub fn create_screenshot_mem_image(
    device: &ash::Device,
    allocator: &mut gpu_allocator::vulkan::Allocator,
    format: vk::Format,
    surface_extent: &vk::Extent2D,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<vulkan_utils::MemImage, String> {
    log::info!("creating screenshot mem image");

    let extent = vk::Extent3D {
        width: surface_extent.width,
        height: surface_extent.height,
        depth: 1,
    };

    let mem_image = vulkan_utils::create_image_2d(
        device,
        allocator,
        format,
        extent,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
        &format!("screenshot image"),
        debug_utils_loader,
    )?;

    log::info!("screenshot mem image created");

    Ok(mem_image)
}
