// textures.rs

use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        // Map characters to texture file paths - Enhanced Berserk style
        let texture_files = vec![
            ('+', "assets/textures/elements/Elements_02-128x128.png"),
            ('-', "assets/textures/cloth/Cloth_02-128x128.png"),
            ('|', "assets/textures/cloth/Cloth_22-128x128.png"),
            ('g', "assets/textures/large_door.png"),
            ('#', "assets/textures/cloth/Cloth_02-128x128.png"), // default/fallback
            ('e', "assets/sprite1.png"), // Keep original sprite for now
        ];

        for (ch, path) in texture_files {
            println!("Attempting to load texture: {}", path);
            match Image::load_image(path) {
                Ok(image) => {
                    match rl.load_texture(thread, path) {
                        Ok(texture) => {
                            println!("Successfully loaded texture: {} ({}x{})", path, image.width, image.height);
                            images.insert(ch, image);
                            textures.insert(ch, texture);
                        }
                        Err(e) => {
                            eprintln!("Failed to load texture {}: {:?}", path, e);
                            // Fallback to a solid color texture
                            let fallback_image = Image::gen_image_color(64, 64, Color::GRAY);
                            let fallback_texture = rl.load_texture_from_image(thread, &fallback_image).expect("Failed to create fallback texture");
                            images.insert(ch, fallback_image);
                            textures.insert(ch, fallback_texture);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load image {}: {:?}", path, e);
                    // Fallback to a solid color texture
                    let fallback_image = Image::gen_image_color(64, 64, Color::RED);
                    let fallback_texture = rl.load_texture_from_image(thread, &fallback_image).expect("Failed to create fallback texture");
                    images.insert(ch, fallback_image);
                    textures.insert(ch, fallback_texture);
                }
            }
        }

        TextureManager { images, textures }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            
            get_pixel_color(image, x, y)
        } else {
            println!("Warning: No texture found for character '{}'", ch);
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let x = (x as usize).min(width - 1);
    let y = (y as usize).min(height - 1);

    // Much safer bounds checking
    let pixel_index = y * width + x;
    let byte_index = pixel_index * 4; // RGBA = 4 bytes per pixel
    let total_bytes = width * height * 4;

    unsafe {
        // Null pointer check
        if image.data.is_null() {
            return Color::WHITE;
        }
        
        // Bounds check before creating slice
        if byte_index + 3 >= total_bytes {
            return Color::WHITE;
        }
        
        let data = slice::from_raw_parts(image.data as *const u8, total_bytes);
        
        // Final safety check
        if byte_index + 3 < data.len() {
            Color::new(
                data[byte_index],     // R
                data[byte_index + 1], // G
                data[byte_index + 2], // B
                data[byte_index + 3], // A
            )
        } else {
            Color::WHITE
        }
    }
}
