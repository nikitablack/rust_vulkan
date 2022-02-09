use ash::vk;

pub trait ColorReader {
    fn read(&self, data: &[u8], offset: usize) -> image::Rgba<u8>;
}

pub struct ReaderB8G8R8A8;
pub struct ReaderR8G8B8A8;

impl ColorReader for ReaderB8G8R8A8 {
    fn read(&self, data: &[u8], offset: usize) -> image::Rgba<u8> {
        let b = data[offset + 0];
        let g = data[offset + 1];
        let r = data[offset + 2];
        let a = data[offset + 3];

        image::Rgba([r, g, b, a])
    }
}

impl ColorReader for ReaderR8G8B8A8 {
    fn read(&self, data: &[u8], offset: usize) -> image::Rgba<u8> {
        let r = data[offset + 0];
        let g = data[offset + 1];
        let b = data[offset + 2];
        let a = data[offset + 3];

        image::Rgba([r, g, b, a])
    }
}

pub fn get_format_block_size(format: vk::Format) -> u8 {
    match format {
        vk::Format::B8G8R8A8_SRGB | vk::Format::R8G8B8A8_UNORM /* | TODO */ => 4,
        _ => panic!("implement `get_format_texel_size` for {:?}", format),
    }
}

pub fn get_color_reader(format: vk::Format) -> Box<dyn ColorReader> {
    match format {
        vk::Format::B8G8R8A8_SRGB => Box::new(ReaderB8G8R8A8),
        vk::Format::R8G8B8A8_UNORM => Box::new(ReaderR8G8B8A8),
        _ => panic!("implement `get_color_reader` for {:?}", format),
    }
}
