use ash::vk;

use crate::vulkan::VulkanData;
use vulkan_base::VulkanBase;

pub fn screenshot_begin_render_pass(
    vulkan_data: &VulkanData,
    vulkan_base: &VulkanBase,
    command_buffer: vk::CommandBuffer,
) {
    let clear_color = vk::ClearColorValue {
        float32: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
    };

    let clear_depth = vk::ClearDepthStencilValue {
        depth: 1.0,
        stencil: 0,
    };

    let clear_values = vec![
        vk::ClearValue { color: clear_color },
        vk::ClearValue {
            depth_stencil: clear_depth,
        },
    ];

    let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
        .render_pass(vulkan_data.screenshot_render_pass)
        .framebuffer(vulkan_data.screenshot_framebuffer)
        .render_area(vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vulkan_base.surface_extent,
        })
        .clear_values(&clear_values)
        .build();

    unsafe {
        vulkan_base.device.cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        );
    }
}
