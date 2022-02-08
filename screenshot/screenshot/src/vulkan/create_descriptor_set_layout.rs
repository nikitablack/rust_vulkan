use ash::vk;

pub fn create_descriptor_set_layout(
    device: &ash::Device,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<vk::DescriptorSetLayout, String> {
    log::info!("creating descriptor set layout");

    let positions_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .build();

    let colors_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(1)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .build();

    let bindings = [positions_binding, colors_binding];
    let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(&bindings)
        .build();

    let descriptor_set_layout = unsafe {
        device
            .create_descriptor_set_layout(&create_info, None)
            .map_err(|_| String::from("failed to create descriptor set layout"))?
    };

    vulkan_utils::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        descriptor_set_layout,
        "descriptor set layout",
    );

    log::info!("descriptor set layout created");

    Ok(descriptor_set_layout)
}
