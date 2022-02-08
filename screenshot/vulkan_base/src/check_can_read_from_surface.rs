use ash::vk;

pub fn check_can_read_from_surface(
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
) -> Result<(), String> {
    log::info!("checking if can read from surface");

    if !surface_capabilities
        .supported_usage_flags
        .contains(vk::ImageUsageFlags::TRANSFER_SRC)
    {
        return Err(String::from(
            "ImageUsageFlags::TRANSFER_SRC is not supported by the surface",
        ));
    }

    Ok(())
}
