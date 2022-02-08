use crate::VulkanData;
use ash::vk;
use vulkan_base::VulkanBase;

pub fn draw(
    vulkan_data: &mut VulkanData,
    vulkan_base: &mut VulkanBase,
    make_screenshot: bool,
) -> Result<(), String> {
    let get_image_index_result = super::get_image_index(vulkan_data, vulkan_base)?;

    let image_index = match get_image_index_result {
        super::GetImageIndexResult::Index(index) => index,
        super::GetImageIndexResult::ShouldRebuildSwapchain => {
            println!("swapchain is suboptimal or out of date");
            vulkan_data.should_resize = true;
            return Ok(());
        }
    };

    super::wait_resource_available(vulkan_data, vulkan_base)?;
    super::reset_command_pool(vulkan_data, vulkan_base)?;
    let command_buffer = super::get_command_buffer(vulkan_data, vulkan_base)?;
    super::begin_command_buffer(vulkan_base, command_buffer)?;

    super::begin_render_pass(
        vulkan_base,
        vulkan_data.render_pass,
        vulkan_data.framebuffers[image_index as usize],
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

    let mut screenshot_buffer = None;
    if make_screenshot {
        let _ = screenshot_buffer.insert(super::do_screenshot(
            vulkan_base,
            vulkan_base.swapchain_images[image_index as usize],
            command_buffer,
        )?);
    }

    unsafe {
        vulkan_base
            .device
            .end_command_buffer(command_buffer)
            .map_err(|_| String::from("failed to end command buffer"))?
    }

    super::submit(vulkan_data, vulkan_base, command_buffer)?;

    if make_screenshot && screenshot_buffer.is_some() {
        super::save_screenshot(screenshot_buffer.unwrap(), vulkan_base)?;
    }

    if !super::present(vulkan_data, vulkan_base, image_index)? {
        println!("swapchain is suboptimal or out of date");
        vulkan_data.should_resize = true;
        return Ok(());
    }

    Ok(())
}
