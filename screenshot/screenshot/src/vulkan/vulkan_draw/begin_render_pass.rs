use ash::vk;

use vulkan_base::VulkanBase;

pub fn begin_render_pass(
    vulkan_base: &VulkanBase,
    render_pass: vk::RenderPass,
    framebuffer: vk::Framebuffer,
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
        .render_pass(render_pass)
        .framebuffer(framebuffer)
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
