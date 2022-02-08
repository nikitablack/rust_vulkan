use ash::vk;

use crate::vulkan::VulkanData;
use vulkan_base::VulkanBase;

pub fn submit_screenshot(
    vulkan_data: &VulkanData,
    vulkan_base: &VulkanBase,
    command_buffer: vk::CommandBuffer,
) -> Result<(), String> {
    let fence = vulkan_data.fences[vulkan_data.curr_resource_index as usize];

    let cmd_buffers = [command_buffer];
    let submit_info = vk::SubmitInfo::builder()
        .command_buffers(&cmd_buffers)
        .build();

    unsafe {
        vulkan_base
            .device
            .queue_submit(vulkan_base.queue, &[submit_info], fence)
            .map_err(|_| String::from("failed to submit graphics command buffer"))?
    }

    Ok(())
}
