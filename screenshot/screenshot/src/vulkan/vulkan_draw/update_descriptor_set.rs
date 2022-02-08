use ash::vk;

use crate::vulkan::VulkanData;
use vulkan_base::VulkanBase;

pub fn update_descriptor_set(
    vulkan_data: &VulkanData,
    vulkan_base: &VulkanBase,
    set: vk::DescriptorSet,
) {
    let positions_buffer_info = vk::DescriptorBufferInfo {
        buffer: vulkan_data.positions_mem_buffer.buffer,
        offset: 0,
        range: vk::WHOLE_SIZE,
    };

    let colors_buffer_info = vk::DescriptorBufferInfo {
        buffer: vulkan_data.colors_mem_buffer.buffer,
        offset: 0,
        range: vk::WHOLE_SIZE,
    };

    let infos_1 = [positions_buffer_info];
    let write_descriptor_set_1 = vk::WriteDescriptorSet::builder()
        .dst_set(set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&infos_1)
        .build();

    let infos_2 = [colors_buffer_info];
    let write_descriptor_set_2 = vk::WriteDescriptorSet::builder()
        .dst_set(set)
        .dst_binding(1)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&infos_2)
        .build();

    unsafe {
        vulkan_base
            .device
            .update_descriptor_sets(&[write_descriptor_set_1, write_descriptor_set_2], &[]);
    }
}
