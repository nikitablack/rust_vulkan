pub struct ScreenshotData {
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u16>,
}

impl ScreenshotData {
    pub fn new() -> Self {
        let positions = vec![
            // triangle 1
            -0.5, -1.0, 0.2, 1.0, // 1
            0.5, 1.0, 0.2, 1.0, // 2
            -1.0, 1.0, 0.2, 1.0, // 3
            // triangle 2
            0.5, -1.0, 0.1, 1.0, // 1
            1.0, 1.0, 0.1, 1.0, // 2
            -0.5, 1.0, 0.1, 1.0, // 3
        ];

        let colors = vec![
            // triangle 1
            0.5, 0.0, 0.0, 0.5, // 1
            0.5, 0.0, 0.0, 0.5, // 1
            0.5, 0.0, 0.0, 0.5, // 1
            // triangle 2
            0.0, 1.0, 0.0, 0.5, // 2
            0.0, 1.0, 0.0, 0.5, // 2
            0.0, 1.0, 0.0, 0.5, // 2
        ];

        let indices = vec![
            0, 1, 2, // triangle 1
            3, 4, 5, // triangle 2
        ];

        ScreenshotData {
            positions,
            colors,
            indices,
        }
    }

    pub fn get_positions_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.positions)
    }

    pub fn get_colors_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.colors)
    }

    pub fn get_indices_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.indices)
    }

    pub fn get_indices_count(&self) -> u32 {
        self.indices.len() as u32
    }
}
