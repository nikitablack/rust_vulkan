use crate::VulkanData;
use ash::vk;
use vulkan_base::VulkanBase;

pub fn draw_screenshot(
    vulkan_data: &mut VulkanData,
    vulkan_base: &mut VulkanBase,
) -> Result<(), String> {
    super::wait_resource_available(vulkan_data, vulkan_base)?;
    super::reset_command_pool(vulkan_data, vulkan_base)?;
    let command_buffer = super::get_command_buffer(vulkan_data, vulkan_base)?;
    super::begin_command_buffer(vulkan_base, command_buffer)?;

    super::begin_render_pass(
        vulkan_base,
        vulkan_data.render_pass,
        vulkan_data.screenshot_framebuffer,
        command_buffer,
    );

    super::set_viewport(vulkan_base, command_buffer);
    super::set_scissor(vulkan_base, command_buffer);
    super::reset_descriptor_pool(vulkan_data, vulkan_base)?;
    let descriptor_set = super::allocate_descriptor_set(vulkan_data, vulkan_base)?;
    super::update_descriptor_set(vulkan_data, vulkan_base, descriptor_set);

    unsafe {
        vulkan_base.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            vulkan_data.pipeline_layout,
            0,
            &[descriptor_set],
            &[],
        );

        vulkan_base.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            vulkan_data.pipeline,
        );

        vulkan_base.device.cmd_bind_index_buffer(
            command_buffer,
            vulkan_data.indices_mem_buffer.buffer,
            0,
            vk::IndexType::UINT16,
        );

        vulkan_base
            .device
            .cmd_draw_indexed(command_buffer, vulkan_data.indices_count, 1, 0, 0, 0);
    }

    unsafe {
        vulkan_base.device.cmd_end_render_pass(command_buffer);
    }

    let screenshot_buffer = super::do_screenshot(
        vulkan_base,
        vulkan_data.screenshot_mem_image.image,
        command_buffer,
    )?;

    unsafe {
        vulkan_base
            .device
            .end_command_buffer(command_buffer)
            .map_err(|_| String::from("failed to end command buffer"))?
    }

    super::submit_screenshot(vulkan_data, vulkan_base, command_buffer)?;

    super::save_screenshot(screenshot_buffer, vulkan_base)?;

    Ok(())
}
