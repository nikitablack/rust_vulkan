use crate::MemImage;
use ash::vk;

pub fn create_image_2d(
    device: &ash::Device,
    allocator: &mut gpu_allocator::vulkan::Allocator,
    format: vk::Format,
    extent: vk::Extent3D,
    usage: vk::ImageUsageFlags,
    view_subresource_range_aspect: vk::ImageAspectFlags,
    object_name: &str,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<MemImage, String> {
    // image
    log::info!("{}: creating", object_name);

    let image_sg = {
        let image_create_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(extent)
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .build();

        let image = unsafe {
            device
                .create_image(&image_create_info, None)
                .map_err(|_| format!("{}: failed to create", object_name))?
        };

        scopeguard::guard(image, |image| {
            log::warn!("{} scopeguard", object_name);
            unsafe {
                device.destroy_image(image, None);
            }
        })
    };

    log::info!("{}: created", object_name);

    // allocation
    log::info!("{}: allocating memory", object_name);

    let allocation_sg = {
        let memory_requirements = unsafe { device.get_image_memory_requirements(*image_sg) };

        let allocation_create_desc = gpu_allocator::vulkan::AllocationCreateDesc {
            name: object_name,
            requirements: memory_requirements,
            location: gpu_allocator::MemoryLocation::GpuOnly,
            linear: false,
        };

        let allocation = allocator
            .allocate(&allocation_create_desc)
            .map_err(|_| format!("{}: failed to allocate memory", object_name))?;

        scopeguard::guard(allocation, |allocation| {
            log::warn!("{} allocation scopeguard", object_name);
            let _ = allocator.free(allocation);
        })
    };

    log::info!("{}: memory allocated", object_name);

    // binding
    log::info!("{}: binding memory", object_name);

    unsafe {
        device
            .bind_image_memory(*image_sg, allocation_sg.memory(), allocation_sg.offset())
            .map_err(|_| format!("{}: failed to bind memory", object_name))?
    };

    log::info!("{}: memory bound", object_name);

    // view
    log::info!("{}: creating view", object_name);

    let image_view_sg = {
        let view_create_info = vk::ImageViewCreateInfo::builder()
            .image(*image_sg)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(vk::ComponentMapping {
                r: vk::ComponentSwizzle::R,
                g: vk::ComponentSwizzle::G,
                b: vk::ComponentSwizzle::B,
                a: vk::ComponentSwizzle::A,
            })
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: view_subresource_range_aspect,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .build();

        let view = unsafe {
            device
                .create_image_view(&view_create_info, None)
                .map_err(|_| format!("{}: failed to allocate memory", object_name))?
        };

        scopeguard::guard(view, |view| {
            log::warn!("{} image view scopeguard", object_name);
            unsafe {
                device.destroy_image_view(view, None);
            }
        })
    };

    log::info!("{}: image view created", object_name);

    crate::set_debug_utils_object_name(debug_utils_loader, device.handle(), *image_sg, object_name);

    crate::set_debug_utils_object_name(
        &debug_utils_loader,
        device.handle(),
        unsafe { allocation_sg.memory() },
        &format!("{} memory", object_name),
    );

    crate::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        *image_view_sg,
        object_name,
    );

    Ok(MemImage {
        image: scopeguard::ScopeGuard::into_inner(image_sg),
        view: scopeguard::ScopeGuard::into_inner(image_view_sg),
        extent,
        allocation: scopeguard::ScopeGuard::into_inner(allocation_sg),
    })
}

pub fn destroy_mem_image(
    device: &ash::Device,
    allocator: &mut gpu_allocator::vulkan::Allocator,
    mem_image: MemImage,
) {
    unsafe {
        device.destroy_image(mem_image.image, None);
        device.destroy_image_view(mem_image.view, None);
        let _ = allocator.free(mem_image.allocation);
    }
}
