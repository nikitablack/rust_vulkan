use std::cell::RefCell;

use crate::screenshot_data;
use crate::vulkan;
use ash::vk;
use vulkan_base::VulkanBase;
use vulkan_utils::MemImage;

pub struct VulkanData {
    pub vertex_shader_module: vk::ShaderModule,
    pub fragment_shader_module: vk::ShaderModule,
    pub positions_mem_buffer: vulkan_utils::MemBuffer,
    pub colors_mem_buffer: vulkan_utils::MemBuffer,
    pub indices_count: u32,
    pub indices_mem_buffer: vulkan_utils::MemBuffer,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub render_pass: vk::RenderPass,
    pub pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub should_resize: bool,
    pub image_available_semaphore: vk::Semaphore,
    pub rendering_finished_semaphore: vk::Semaphore,
    pub fences: Vec<vk::Fence>,
    pub command_pools: Vec<vk::CommandPool>,
    pub descriptor_pools: Vec<vk::DescriptorPool>,
    pub available_command_buffers: Vec<Vec<vk::CommandBuffer>>,
    pub used_command_buffers: Vec<Vec<vk::CommandBuffer>>,
    pub curr_resource_index: u32,
    pub screenshot_format: vk::Format,
    pub screenshot_render_pass: vk::RenderPass,
    pub screenshot_mem_image: MemImage,
    pub screenshot_framebuffer: vk::Framebuffer,
}

impl VulkanData {
    pub fn new(vulkan_base: &mut VulkanBase) -> Result<Self, String> {
        let allocator_rc = RefCell::new(&mut vulkan_base.allocator);

        let vertex_shader_module = vulkan_utils::create_shader_module(
            &vulkan_base.device,
            std::path::Path::new("shaders/shader.vert.spv"),
            &vulkan_base.debug_utils_loader,
            "vertex shader",
        )?;

        let fragment_shader_module = vulkan_utils::create_shader_module(
            &vulkan_base.device,
            std::path::Path::new("shaders/shader.frag.spv"),
            &vulkan_base.debug_utils_loader,
            "fragment shader",
        )?;

        let screenshot_data = screenshot_data::ScreenshotData::new();

        let positions_mem_buffer = vulkan_utils::create_gpu_buffer_init(
            &vulkan_base.device,
            *allocator_rc.borrow_mut(),
            &vulkan_base.debug_utils_loader,
            vulkan_base.queue_family,
            vulkan_base.queue,
            screenshot_data.get_positions_slice(),
            vk::BufferUsageFlags::STORAGE_BUFFER,
            vk::AccessFlags::SHADER_READ,
            vk::PipelineStageFlags::VERTEX_SHADER,
            "positions buffer",
        )?;

        let colors_mem_buffer = vulkan_utils::create_gpu_buffer_init(
            &vulkan_base.device,
            *allocator_rc.borrow_mut(),
            &vulkan_base.debug_utils_loader,
            vulkan_base.queue_family,
            vulkan_base.queue,
            screenshot_data.get_colors_slice(),
            vk::BufferUsageFlags::STORAGE_BUFFER,
            vk::AccessFlags::SHADER_READ,
            vk::PipelineStageFlags::VERTEX_SHADER,
            "colors buffer",
        )?;

        let indices_count = screenshot_data.get_indices_count();

        let indices_mem_buffer = vulkan_utils::create_gpu_buffer_init(
            &vulkan_base.device,
            *allocator_rc.borrow_mut(),
            &vulkan_base.debug_utils_loader,
            vulkan_base.queue_family,
            vulkan_base.queue,
            screenshot_data.get_indices_slice(),
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::AccessFlags::INDEX_READ,
            vk::PipelineStageFlags::VERTEX_INPUT,
            "indices buffer",
        )?;

        let descriptor_set_layout = vulkan::create_descriptor_set_layout(
            &vulkan_base.device,
            &vulkan_base.debug_utils_loader,
        )?;

        let pipeline_layout = vulkan::create_pipeline_layout(
            &vulkan_base.device,
            descriptor_set_layout,
            &vulkan_base.debug_utils_loader,
        )?;

        /*let all_formats = vec![
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
            let props = unsafe {
                vulkan_base
                    .instance
                    .get_physical_device_format_properties(vulkan_base.physical_device, format)
            };

            if props.optimal_tiling_features.contains(
                vk::FormatFeatureFlags::COLOR_ATTACHMENT
                    | vk::FormatFeatureFlags::COLOR_ATTACHMENT_BLEND,
            ) {
                log::info!("AAAAAAAAAAAAAAAAAAAAa format: {:?}", format);
            }
        }*/

        let render_pass = vulkan::create_render_pass(
            &vulkan_base.device,
            vulkan_base.surface_format.format,
            vulkan_base.depth_format,
            "render pass",
            &vulkan_base.debug_utils_loader,
        )?;

        let pipeline = vulkan::create_pipeline(
            &vulkan_base.device,
            vertex_shader_module,
            fragment_shader_module,
            pipeline_layout,
            render_pass,
            &vulkan_base.debug_utils_loader,
        )?;

        let framebuffers = vulkan::create_framebuffers(
            &vulkan_base.device,
            &vulkan_base.swapchain_image_views,
            render_pass,
            vulkan_base.surface_extent,
            vulkan_base.depth_buffer_mem_image.view,
            &vulkan_base.debug_utils_loader,
        )?;

        let image_available_semaphore = vulkan_utils::create_semaphore(
            &vulkan_base.device,
            &vulkan_base.debug_utils_loader,
            "image available semaphore",
        )?;

        let rendering_finished_semaphore = vulkan_utils::create_semaphore(
            &vulkan_base.device,
            &vulkan_base.debug_utils_loader,
            "rendering finished semaphore",
        )?;

        let fences = vulkan::create_fences(&vulkan_base.device, &vulkan_base.debug_utils_loader)?;

        let command_pools = vulkan::create_command_pools(
            &vulkan_base.device,
            vulkan_base.queue_family,
            &vulkan_base.debug_utils_loader,
        )?;

        let descriptor_pools =
            vulkan::create_descriptor_pools(&vulkan_base.device, &vulkan_base.debug_utils_loader)?;

        let screenshot_format = vk::Format::B8G8R8A8_SRGB;
        let screenshot_render_pass = vulkan::create_render_pass(
            &vulkan_base.device,
            screenshot_format,
            vulkan_base.depth_format,
            "screenshot render pass",
            &vulkan_base.debug_utils_loader,
        )?;

        let screenshot_mem_image = vulkan::create_screenshot_mem_image(
            &vulkan_base.device,
            *allocator_rc.borrow_mut(),
            screenshot_format,
            &vulkan_base.surface_extent,
            &vulkan_base.debug_utils_loader,
        )?;

        let screenshot_framebuffer = vulkan::create_framebuffers(
            &vulkan_base.device,
            &vec![screenshot_mem_image.view],
            screenshot_render_pass,
            vulkan_base.surface_extent,
            vulkan_base.depth_buffer_mem_image.view,
            &vulkan_base.debug_utils_loader,
        )?[0];

        Ok(VulkanData {
            vertex_shader_module,
            fragment_shader_module,
            positions_mem_buffer,
            colors_mem_buffer,
            indices_count,
            indices_mem_buffer,
            descriptor_set_layout,
            pipeline_layout,
            render_pass,
            pipeline,
            framebuffers,
            should_resize: false,
            image_available_semaphore,
            rendering_finished_semaphore,
            fences,
            command_pools,
            descriptor_pools,
            available_command_buffers: vec![vec![]; crate::CONCURRENT_RESOURCE_COUNT as usize],
            used_command_buffers: vec![vec![]; crate::CONCURRENT_RESOURCE_COUNT as usize],
            curr_resource_index: 0,
            screenshot_format,
            screenshot_render_pass,
            screenshot_mem_image,
            screenshot_framebuffer,
        })
    }

    pub fn resize(&mut self, vulkan_base: &mut VulkanBase) -> Result<(), String> {
        // framebuffer
        unsafe {
            for &framebuffer in &self.framebuffers {
                vulkan_base.device.destroy_framebuffer(framebuffer, None);
            }
        }

        self.framebuffers = vulkan::create_framebuffers(
            &vulkan_base.device,
            &vulkan_base.swapchain_image_views,
            self.render_pass,
            vulkan_base.surface_extent,
            vulkan_base.depth_buffer_mem_image.view,
            &vulkan_base.debug_utils_loader,
        )?;

        // screenshot mem image
        let mut tmp_screenshot_mem_image = vulkan_utils::MemImage {
            image: vk::Image::null(),
            view: vk::ImageView::null(),
            extent: vk::Extent3D::default(),
            allocation: gpu_allocator::vulkan::Allocation::default(),
        };

        std::mem::swap(
            &mut tmp_screenshot_mem_image,
            &mut self.screenshot_mem_image,
        );

        vulkan_utils::destroy_mem_image(
            &vulkan_base.device,
            &mut vulkan_base.allocator,
            tmp_screenshot_mem_image,
        );

        self.screenshot_mem_image = vulkan::create_screenshot_mem_image(
            &vulkan_base.device,
            &mut vulkan_base.allocator,
            self.screenshot_format,
            &vulkan_base.surface_extent,
            &vulkan_base.debug_utils_loader,
        )?;

        // screenshot framebuffer
        unsafe {
            vulkan_base
                .device
                .destroy_framebuffer(self.screenshot_framebuffer, None);
        }

        self.screenshot_framebuffer = vulkan::create_framebuffers(
            &vulkan_base.device,
            &vec![self.screenshot_mem_image.view],
            self.screenshot_render_pass,
            vulkan_base.surface_extent,
            vulkan_base.depth_buffer_mem_image.view,
            &vulkan_base.debug_utils_loader,
        )?[0];

        Ok(())
    }

    pub fn clean(self, vulkan_base: &mut VulkanBase) {
        log::info!("cleaning vulkan data");

        unsafe {
            let device = &vulkan_base.device;
            let allocator = &mut vulkan_base.allocator;

            device.destroy_shader_module(self.vertex_shader_module, None);
            device.destroy_shader_module(self.fragment_shader_module, None);

            device.destroy_buffer(self.positions_mem_buffer.buffer, None);
            let _ = allocator.free(self.positions_mem_buffer.allocation);

            device.destroy_buffer(self.colors_mem_buffer.buffer, None);
            let _ = allocator.free(self.colors_mem_buffer.allocation);

            device.destroy_buffer(self.indices_mem_buffer.buffer, None);
            let _ = allocator.free(self.indices_mem_buffer.allocation);

            vulkan_base
                .device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            vulkan_base
                .device
                .destroy_pipeline_layout(self.pipeline_layout, None);

            vulkan_base
                .device
                .destroy_render_pass(self.render_pass, None);

            vulkan_base.device.destroy_pipeline(self.pipeline, None);

            for &framebuffer in &self.framebuffers {
                vulkan_base.device.destroy_framebuffer(framebuffer, None);
            }

            vulkan_base
                .device
                .destroy_semaphore(self.image_available_semaphore, None);

            vulkan_base
                .device
                .destroy_semaphore(self.rendering_finished_semaphore, None);

            for &fence in &self.fences {
                vulkan_base.device.destroy_fence(fence, None);
            }

            for &command_pool in &self.command_pools {
                vulkan_base.device.destroy_command_pool(command_pool, None);
            }

            for &descriptor_pool in &self.descriptor_pools {
                vulkan_base
                    .device
                    .destroy_descriptor_pool(descriptor_pool, None);
            }

            vulkan_base
                .device
                .destroy_render_pass(self.screenshot_render_pass, None);

            vulkan_utils::destroy_mem_image(
                &vulkan_base.device,
                allocator,
                self.screenshot_mem_image,
            );

            vulkan_base
                .device
                .destroy_framebuffer(self.screenshot_framebuffer, None);
        }
    }
}
