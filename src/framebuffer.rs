// framebuffer.rs

use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub depth_buffer: Vec<f32>, // Add depth buffer for z-testing
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        let depth_buffer = vec![f32::INFINITY; (width * height) as usize]; // Initialize with max depth
        Framebuffer {
            width,
            height,
            color_buffer,
            depth_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
        // Faster depth buffer clear using fill
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    // New method: set pixel with depth testing
    pub fn set_pixel_with_depth(&mut self, x: u32, y: u32, depth: f32) -> bool {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if depth < self.depth_buffer[index] {
                self.depth_buffer[index] = depth;
                self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
                return true;
            }
        }
        false
    }

    // Get depth at pixel (for sprite rendering)
    pub fn get_depth(&self, x: u32, y: u32) -> f32 {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.depth_buffer[index]
        } else {
            f32::INFINITY
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn _render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn get_texture(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) -> Result<Texture2D, String> {
        window.load_texture_from_image(raylib_thread, &self.color_buffer)
            .map_err(|_| "Failed to create texture from image".to_string())
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
