use vulkan_base::VulkanBase;

pub fn save_screenshot(
    screenshot_mem_buffer: vulkan_utils::MemBuffer,
    vulkan_base: &mut VulkanBase,
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

    for x in 0..extent.width {
        for y in 0..extent.height {
            let b = data[(extent.width * y * 4 + x * 4 + 0) as usize];
            let g = data[(extent.width * y * 4 + x * 4 + 1) as usize];
            let r = data[(extent.width * y * 4 + x * 4 + 2) as usize];
            let a = data[(extent.width * y * 4 + x * 4 + 3) as usize];

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgba([r, g, b, a]);
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
