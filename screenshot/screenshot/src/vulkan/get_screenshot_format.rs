use ash::vk;

pub fn get_screenshot_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<vk::Format, String> {
    log::info!("getting screenshot format");

    // trying to find preferred format
    {
        let format = vk::Format::R8G8B8A8_UNORM;

        let props =
            unsafe { instance.get_physical_device_format_properties(physical_device, format) };

        if props.optimal_tiling_features.contains(
            vk::FormatFeatureFlags::COLOR_ATTACHMENT
                | vk::FormatFeatureFlags::COLOR_ATTACHMENT_BLEND,
        ) {
            log::info!("selected screenshot format: {:?}", format);
            return Ok(format);
        }
    }

    let all_formats = [
        vk::Format::R4G4_UNORM_PACK8,
        vk::Format::R4G4B4A4_UNORM_PACK16,
        vk::Format::B4G4R4A4_UNORM_PACK16,
        vk::Format::R5G6B5_UNORM_PACK16,
        vk::Format::B5G6R5_UNORM_PACK16,
        vk::Format::R5G5B5A1_UNORM_PACK16,
        vk::Format::B5G5R5A1_UNORM_PACK16,
        vk::Format::A1R5G5B5_UNORM_PACK16,
        vk::Format::R8_UNORM,
        vk::Format::R8_SNORM,
        vk::Format::R8_USCALED,
        vk::Format::R8_SSCALED,
        vk::Format::R8_UINT,
        vk::Format::R8_SINT,
        vk::Format::R8_SRGB,
        vk::Format::R8G8_UNORM,
        vk::Format::R8G8_SNORM,
        vk::Format::R8G8_USCALED,
        vk::Format::R8G8_SSCALED,
        vk::Format::R8G8_UINT,
        vk::Format::R8G8_SINT,
        vk::Format::R8G8_SRGB,
        vk::Format::R8G8B8_UNORM,
        vk::Format::R8G8B8_SNORM,
        vk::Format::R8G8B8_USCALED,
        vk::Format::R8G8B8_SSCALED,
        vk::Format::R8G8B8_UINT,
        vk::Format::R8G8B8_SINT,
        vk::Format::R8G8B8_SRGB,
        vk::Format::B8G8R8_UNORM,
        vk::Format::B8G8R8_SNORM,
        vk::Format::B8G8R8_USCALED,
        vk::Format::B8G8R8_SSCALED,
        vk::Format::B8G8R8_UINT,
        vk::Format::B8G8R8_SINT,
        vk::Format::B8G8R8_SRGB,
        vk::Format::R8G8B8A8_UNORM,
        vk::Format::R8G8B8A8_SNORM,
        vk::Format::R8G8B8A8_USCALED,
        vk::Format::R8G8B8A8_SSCALED,
        vk::Format::R8G8B8A8_UINT,
        vk::Format::R8G8B8A8_SINT,
        vk::Format::R8G8B8A8_SRGB,
        vk::Format::B8G8R8A8_UNORM,
        vk::Format::B8G8R8A8_SNORM,
        vk::Format::B8G8R8A8_USCALED,
        vk::Format::B8G8R8A8_SSCALED,
        vk::Format::B8G8R8A8_UINT,
        vk::Format::B8G8R8A8_SINT,
        vk::Format::B8G8R8A8_SRGB,
        vk::Format::A8B8G8R8_UNORM_PACK32,
        vk::Format::A8B8G8R8_SNORM_PACK32,
        vk::Format::A8B8G8R8_USCALED_PACK32,
        vk::Format::A8B8G8R8_SSCALED_PACK32,
        vk::Format::A8B8G8R8_UINT_PACK32,
        vk::Format::A8B8G8R8_SINT_PACK32,
        vk::Format::A8B8G8R8_SRGB_PACK32,
        vk::Format::A2R10G10B10_UNORM_PACK32,
        vk::Format::A2R10G10B10_SNORM_PACK32,
        vk::Format::A2R10G10B10_USCALED_PACK32,
        vk::Format::A2R10G10B10_SSCALED_PACK32,
        vk::Format::A2R10G10B10_UINT_PACK32,
        vk::Format::A2R10G10B10_SINT_PACK32,
        vk::Format::A2B10G10R10_UNORM_PACK32,
        vk::Format::A2B10G10R10_SNORM_PACK32,
        vk::Format::A2B10G10R10_USCALED_PACK32,
        vk::Format::A2B10G10R10_SSCALED_PACK32,
        vk::Format::A2B10G10R10_UINT_PACK32,
        vk::Format::A2B10G10R10_SINT_PACK32,
        vk::Format::R16_UNORM,
        vk::Format::R16_SNORM,
        vk::Format::R16_USCALED,
        vk::Format::R16_SSCALED,
        vk::Format::R16_UINT,
        vk::Format::R16_SINT,
        vk::Format::R16_SFLOAT,
        vk::Format::R16G16_UNORM,
        vk::Format::R16G16_SNORM,
        vk::Format::R16G16_USCALED,
        vk::Format::R16G16_SSCALED,
        vk::Format::R16G16_UINT,
        vk::Format::R16G16_SINT,
        vk::Format::R16G16_SFLOAT,
        vk::Format::R16G16B16_UNORM,
        vk::Format::R16G16B16_SNORM,
        vk::Format::R16G16B16_USCALED,
        vk::Format::R16G16B16_SSCALED,
        vk::Format::R16G16B16_UINT,
        vk::Format::R16G16B16_SINT,
        vk::Format::R16G16B16_SFLOAT,
        vk::Format::R16G16B16A16_UNORM,
        vk::Format::R16G16B16A16_SNORM,
        vk::Format::R16G16B16A16_USCALED,
        vk::Format::R16G16B16A16_SSCALED,
        vk::Format::R16G16B16A16_UINT,
        vk::Format::R16G16B16A16_SINT,
        vk::Format::R16G16B16A16_SFLOAT,
        vk::Format::R32_UINT,
        vk::Format::R32_SINT,
        vk::Format::R32_SFLOAT,
        vk::Format::R32G32_UINT,
        vk::Format::R32G32_SINT,
        vk::Format::R32G32_SFLOAT,
        vk::Format::R32G32B32_UINT,
        vk::Format::R32G32B32_SINT,
        vk::Format::R32G32B32_SFLOAT,
        vk::Format::R32G32B32A32_UINT,
        vk::Format::R32G32B32A32_SINT,
        vk::Format::R32G32B32A32_SFLOAT,
        vk::Format::R64_UINT,
        vk::Format::R64_SINT,
        vk::Format::R64_SFLOAT,
        vk::Format::R64G64_UINT,
        vk::Format::R64G64_SINT,
        vk::Format::R64G64_SFLOAT,
        vk::Format::R64G64B64_UINT,
        vk::Format::R64G64B64_SINT,
        vk::Format::R64G64B64_SFLOAT,
        vk::Format::R64G64B64A64_UINT,
        vk::Format::R64G64B64A64_SINT,
        vk::Format::R64G64B64A64_SFLOAT,
        vk::Format::B10G11R11_UFLOAT_PACK32,
        vk::Format::E5B9G9R9_UFLOAT_PACK32,
        vk::Format::D16_UNORM,
        vk::Format::X8_D24_UNORM_PACK32,
        vk::Format::D32_SFLOAT,
        vk::Format::S8_UINT,
        vk::Format::D16_UNORM_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::BC1_RGB_UNORM_BLOCK,
        vk::Format::BC1_RGB_SRGB_BLOCK,
        vk::Format::BC1_RGBA_UNORM_BLOCK,
        vk::Format::BC1_RGBA_SRGB_BLOCK,
        vk::Format::BC2_UNORM_BLOCK,
        vk::Format::BC2_SRGB_BLOCK,
        vk::Format::BC3_UNORM_BLOCK,
        vk::Format::BC3_SRGB_BLOCK,
        vk::Format::BC4_UNORM_BLOCK,
        vk::Format::BC4_SNORM_BLOCK,
        vk::Format::BC5_UNORM_BLOCK,
        vk::Format::BC5_SNORM_BLOCK,
        vk::Format::BC6H_UFLOAT_BLOCK,
        vk::Format::BC6H_SFLOAT_BLOCK,
        vk::Format::BC7_UNORM_BLOCK,
        vk::Format::BC7_SRGB_BLOCK,
        vk::Format::ETC2_R8G8B8_UNORM_BLOCK,
        vk::Format::ETC2_R8G8B8_SRGB_BLOCK,
        vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK,
        vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK,
        vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK,
        vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK,
        vk::Format::EAC_R11_UNORM_BLOCK,
        vk::Format::EAC_R11_SNORM_BLOCK,
        vk::Format::EAC_R11G11_UNORM_BLOCK,
        vk::Format::EAC_R11G11_SNORM_BLOCK,
        vk::Format::ASTC_4X4_UNORM_BLOCK,
        vk::Format::ASTC_4X4_SRGB_BLOCK,
        vk::Format::ASTC_5X4_UNORM_BLOCK,
        vk::Format::ASTC_5X4_SRGB_BLOCK,
        vk::Format::ASTC_5X5_UNORM_BLOCK,
        vk::Format::ASTC_5X5_SRGB_BLOCK,
        vk::Format::ASTC_6X5_UNORM_BLOCK,
        vk::Format::ASTC_6X5_SRGB_BLOCK,
        vk::Format::ASTC_6X6_UNORM_BLOCK,
        vk::Format::ASTC_6X6_SRGB_BLOCK,
        vk::Format::ASTC_8X5_UNORM_BLOCK,
        vk::Format::ASTC_8X5_SRGB_BLOCK,
        vk::Format::ASTC_8X6_UNORM_BLOCK,
        vk::Format::ASTC_8X6_SRGB_BLOCK,
        vk::Format::ASTC_8X8_UNORM_BLOCK,
        vk::Format::ASTC_8X8_SRGB_BLOCK,
        vk::Format::ASTC_10X5_UNORM_BLOCK,
        vk::Format::ASTC_10X5_SRGB_BLOCK,
        vk::Format::ASTC_10X6_UNORM_BLOCK,
        vk::Format::ASTC_10X6_SRGB_BLOCK,
        vk::Format::ASTC_10X8_UNORM_BLOCK,
        vk::Format::ASTC_10X8_SRGB_BLOCK,
        vk::Format::ASTC_10X10_UNORM_BLOCK,
        vk::Format::ASTC_10X10_SRGB_BLOCK,
        vk::Format::ASTC_12X10_UNORM_BLOCK,
        vk::Format::ASTC_12X10_SRGB_BLOCK,
        vk::Format::ASTC_12X12_UNORM_BLOCK,
        vk::Format::ASTC_12X12_SRGB_BLOCK,
    ];

    for &format in &all_formats {
        let props =
            unsafe { instance.get_physical_device_format_properties(physical_device, format) };

        if props.optimal_tiling_features.contains(
            vk::FormatFeatureFlags::COLOR_ATTACHMENT
                | vk::FormatFeatureFlags::COLOR_ATTACHMENT_BLEND,
        ) {
            log::info!("selected screenshot format: {:?}", format);
            return Ok(format);
        }
    }

    Err(String::from("failed to get screenshot format"))
}
