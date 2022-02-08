mod check_can_read_from_surface;
mod check_instance_version;
mod check_required_instance_extensions;
mod create_allocator;
mod create_debug_utils_loader;
mod create_entry;
mod create_instance;
mod create_logical_device;
mod create_surface;
mod create_surface_loader;
mod create_swapchain;
mod create_swapchain_image_views;
mod create_swapchain_loader;
mod get_depth_format;
mod get_physical_device;
mod get_physical_device_properties;
mod get_present_mode;
mod get_queue;
mod get_queue_family;
mod get_surface_capabilities;
mod get_surface_extent;
mod get_surface_format;
mod get_swapchain_images;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr;
use ash::vk;

use check_can_read_from_surface::*;
use check_instance_version::*;
use check_required_instance_extensions::*;
use create_allocator::*;
use create_debug_utils_loader::*;
use create_entry::*;
use create_instance::*;
use create_logical_device::*;
use create_surface::*;
use create_surface_loader::*;
use create_swapchain::*;
use create_swapchain_image_views::*;
use create_swapchain_loader::*;
use get_depth_format::*;
use get_physical_device::*;
use get_physical_device_properties::*;
use get_present_mode::*;
use get_queue::*;
use get_queue_family::*;
use get_surface_capabilities::*;
use get_surface_extent::*;
use get_surface_format::*;
use get_swapchain_images::*;

pub struct VulkanBase {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub surface_loader: khr::Surface,
    pub swapchain_loader: khr::Swapchain,
    pub debug_utils_loader: ash::extensions::ext::DebugUtils,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub depth_format: vk::Format,
    pub queue_family: u32,
    pub device: ash::Device,
    pub queue: vk::Queue,
    pub allocator: gpu_allocator::vulkan::Allocator,
    pub surface_capabilities: vk::SurfaceCapabilitiesKHR,
    pub surface_extent: vk::Extent2D,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub depth_buffer_mem_image: vulkan_utils::MemImage,
}

impl VulkanBase {
    pub fn new<'a, 'b>(
        window: &winit::window::Window,
        required_instance_extensions: &Vec<&'a std::ffi::CStr>,
        required_device_extensions: &Vec<&'b std::ffi::CStr>,
    ) -> Result<Self, String> {
        let entry = create_entry();
        check_instance_version(&entry)?;
        check_required_instance_extensions(&entry, required_instance_extensions)?;

        let instance = create_instance(&entry, required_instance_extensions)?;
        let debug_utils_loader = create_debug_utils_loader(&entry, &instance);
        let surface_loader = create_surface_loader(&entry, &instance);
        let surface = create_surface(&entry, &instance, window)?;
        let physical_device = get_physical_device(&instance, &required_device_extensions)?;
        let surface_capabilities =
            get_surface_capabilities(&surface_loader, physical_device, surface)?;

        check_can_read_from_surface(surface_capabilities)?;

        let physical_device_properties = get_physical_device_properties(&instance, physical_device);
        let surface_format = get_surface_format(physical_device, &surface_loader, surface)?;
        let present_mode = get_present_mode(physical_device, &surface_loader, surface)?;
        let queue_family = get_queue_family(&instance, physical_device, &surface_loader, surface)?;
        let depth_format = get_depth_format(&instance, physical_device)?;

        let device = create_logical_device(
            &instance,
            physical_device,
            queue_family,
            &required_device_extensions,
        )?;

        let queue = get_queue(&device, queue_family);
        let mut allocator = create_allocator(&instance, &device, physical_device)?;
        let swapchain_loader = create_swapchain_loader(&instance, &device);

        let resize_result = resize_internal(
            window,
            &device,
            &surface_loader,
            &swapchain_loader,
            physical_device,
            vk::SwapchainKHR::null(),
            surface,
            &surface_format,
            present_mode,
            &vec![],
            depth_format,
            &mut allocator,
            &mut vulkan_utils::MemImage {
                image: vk::Image::null(),
                view: vk::ImageView::null(),
                extent: vk::Extent3D::default(),
                allocation: gpu_allocator::vulkan::Allocation::default(),
            },
            &debug_utils_loader,
        )?;

        Ok(VulkanBase {
            entry,
            instance,
            surface,
            surface_loader,
            debug_utils_loader,
            physical_device,
            physical_device_properties,
            surface_format,
            present_mode,
            depth_format,
            queue_family,
            queue,
            allocator,
            surface_capabilities: resize_result.surface_capabilities,
            surface_extent: resize_result.surface_extent,
            swapchain: resize_result.swapchain,
            swapchain_images: resize_result.swapchain_images,
            swapchain_image_views: resize_result.swapchain_image_views,
            swapchain_loader,
            device,
            depth_buffer_mem_image: resize_result.depth_buffer_mem_image,
        })
    }

    pub fn resize(&mut self, window: &winit::window::Window) -> Result<(), String> {
        let resize_data = resize_internal(
            window,
            &self.device,
            &self.surface_loader,
            &self.swapchain_loader,
            self.physical_device,
            self.swapchain,
            self.surface,
            &self.surface_format,
            self.present_mode,
            &self.swapchain_image_views,
            self.depth_format,
            &mut self.allocator,
            &mut self.depth_buffer_mem_image,
            &self.debug_utils_loader,
        )?;

        self.surface_capabilities = resize_data.surface_capabilities;
        self.surface_extent = resize_data.surface_extent;
        self.swapchain = resize_data.swapchain;
        self.swapchain_images = resize_data.swapchain_images;
        self.swapchain_image_views = resize_data.swapchain_image_views;
        self.depth_buffer_mem_image = resize_data.depth_buffer_mem_image;

        Ok(())
    }

    pub fn clean(mut self) {
        log::info!("cleaning vulkan base");

        unsafe {
            self.device
                .destroy_image(self.depth_buffer_mem_image.image, None);
            self.device
                .destroy_image_view(self.depth_buffer_mem_image.view, None);
            let _ = self.allocator.free(self.depth_buffer_mem_image.allocation);
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            for &image_view in &self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }
            drop(self.allocator);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

struct ResizeResult {
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,
    depth_buffer_mem_image: vulkan_utils::MemImage,
}

fn resize_internal(
    window: &winit::window::Window,
    device: &ash::Device,
    surface_loader: &ash::extensions::khr::Surface,
    swapchain_loader: &ash::extensions::khr::Swapchain,
    physical_device: vk::PhysicalDevice,
    old_swapchain: vk::SwapchainKHR,
    surface: vk::SurfaceKHR,
    surface_format: &vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    old_swapchain_image_views: &Vec<vk::ImageView>,
    depth_format: vk::Format,
    allocator: &mut gpu_allocator::vulkan::Allocator,
    old_depth_buffer_mem_image: &mut vulkan_utils::MemImage,
    debug_utils_loader: &DebugUtils,
) -> Result<ResizeResult, String> {
    log::info!("resizing VulkanBase");

    unsafe {
        let _ = device.device_wait_idle();
    }

    let surface_capabilities = get_surface_capabilities(surface_loader, physical_device, surface)?;
    let surface_extent = get_surface_extent(window, &surface_capabilities);

    let swapchain = create_swapchain(
        old_swapchain,
        surface,
        &surface_capabilities,
        surface_format,
        surface_extent,
        present_mode,
        swapchain_loader,
    )?;

    // no need to explicitly destroy images. They are destroyed when the swapchain is destroyed.
    let swapchain_images = get_swapchain_images(swapchain_loader, swapchain)?;

    if !old_swapchain_image_views.is_empty() {
        log::info!("destroying old swapchain image views");
        for &image_view in old_swapchain_image_views {
            unsafe {
                device.destroy_image_view(image_view, None);
            };
        }
    }

    let swapchain_image_views =
        create_swapchain_image_views(device, &swapchain_images, surface_format)?;

    let mut tmp_depth_buffer_mem_image = vulkan_utils::MemImage {
        image: vk::Image::null(),
        view: vk::ImageView::null(),
        extent: vk::Extent3D::default(),
        allocation: gpu_allocator::vulkan::Allocation::default(),
    };

    std::mem::swap(&mut tmp_depth_buffer_mem_image, old_depth_buffer_mem_image);

    if tmp_depth_buffer_mem_image.image != vk::Image::null() {
        log::info!("destroying old depth buffer");
        unsafe {
            device.destroy_image(tmp_depth_buffer_mem_image.image, None);
            device.destroy_image_view(tmp_depth_buffer_mem_image.view, None);
        }
        let _ = allocator.free(tmp_depth_buffer_mem_image.allocation);
    }

    let depth_image_extent = vk::Extent3D {
        width: surface_extent.width,
        height: surface_extent.height,
        depth: 1,
    };

    let depth_buffer_mem_image = vulkan_utils::create_image_2d(
        device,
        allocator,
        depth_format,
        depth_image_extent,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        vk::ImageAspectFlags::DEPTH,
        "depth",
        debug_utils_loader,
    )?;

    Ok(ResizeResult {
        surface_capabilities,
        surface_extent,
        swapchain,
        swapchain_images,
        swapchain_image_views,
        depth_buffer_mem_image,
    })
}
