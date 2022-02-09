use ash::vk;
use vulkan_base::VulkanBase;

pub fn save_screenshot(
    screenshot_mem_buffer: vulkan_utils::MemBuffer,
    vulkan_base: &mut VulkanBase,
    format: vk::Format,
) -> Result<(), String> {
    log::info!("saving screenshot data");

    unsafe {
        if let Err(_) = vulkan_base.device.device_wait_idle() {
            return Err(String::from("Failed to wait idle device"));
        }
    }

    let data: &[u8] =
        bytemuck::cast_slice(screenshot_mem_buffer.allocation.mapped_slice().unwrap());

    let extent = &vulkan_base.surface_extent;
    let mut imgbuf = image::ImageBuffer::new(extent.width, extent.height);

    let block_size = vulkan_utils::get_format_block_size(format);
    let color_reader = vulkan_utils::get_color_reader(format);

    for x in 0..extent.width {
        for y in 0..extent.height {
            let offset = extent.width * y * (block_size as u32) + x * (block_size as u32);
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = color_reader.read(data, offset as usize)
        }
    }

    imgbuf.save("screenshot.png").unwrap();

    unsafe {
        vulkan_base
            .device
            .destroy_buffer(screenshot_mem_buffer.buffer, None);
        let _ = vulkan_base.allocator.free(screenshot_mem_buffer.allocation);
    }

    log::info!("screenshot data saved");

    Ok(())
}
