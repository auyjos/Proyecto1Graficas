// textures.rs

use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
    sprite_sheets: HashMap<char, SpriteSheet>, // Store sprite sheet data
}

#[derive(Clone)]
pub struct SpriteSheet {
    pub image: Image,
    pub frame_width: u32,
    pub frame_height: u32,
    pub columns: u32,
    pub rows: u32,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

      
          let texture_files = vec![
            // Dark medieval stone for main structure
            ('+', "assets/textures/elements/Elements_06-128x128_rgba.png"), // Dark stone corners
            ('-', "assets/textures/metals/Metal_07-128x128_rgba.png"),      // Rusty metal horizontals
            ('|', "assets/textures/elements/Elements_08-128x128_rgba.png"), // Weathered stone verticals
            ('g', "assets/textures/large_door_rgba.png"),                   // Large imposing door
            ('#', "assets/Horror_Metal_03-128x128_rgba.png"),               // Horror metal for variety
            ('e', "assets/sprite1_rgba.png"),                               // Enemy sprite
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

        // Initialize sprite sheets
        let mut sprite_sheets = HashMap::new();
        
        // Load sprite sheet for animated enemies (assuming 4x3 grid: 4 columns, 3 rows)
        // Save your sprite sheet as "assets/sprite_sheet.png" 
        println!("Attempting to load sprite sheet: assets/sprite_sheet_rgba.png");
        if let Ok(sprite_image) = Image::load_image("assets/sprite_sheet_rgba.png") {
            println!("Successfully loaded sprite_sheet_rgba.png ({}x{})", sprite_image.width, sprite_image.height);
            let sprite_sheet = SpriteSheet {
                frame_width: sprite_image.width as u32 / 4, // 4 columns
                frame_height: sprite_image.height as u32 / 3, // 3 rows  
                columns: 4,
                rows: 3,
                image: sprite_image,
            };
            println!("Created sprite sheet with frame size: {}x{}", sprite_sheet.frame_width, sprite_sheet.frame_height);
            sprite_sheets.insert('a', sprite_sheet); // 'a' for animated sprite
        } else {
            println!("Warning: Could not load sprite_sheet_rgba.png - using fallback for animations");
            // Create a simple fallback sprite sheet
            let fallback_sprite = Image::gen_image_color(128, 96, Color::BLUE); // 4x3 * 32x32 frames
            let sprite_sheet = SpriteSheet {
                frame_width: 32,
                frame_height: 32,
                columns: 4,
                rows: 3,
                image: fallback_sprite,
            };
            sprite_sheets.insert('a', sprite_sheet);
        }

        TextureManager { images, textures, sprite_sheets }
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

    pub fn get_sprite_frame_color(&self, ch: char, frame_x: usize, frame_y: usize, tx: u32, ty: u32) -> Color {
        if let Some(sprite_sheet) = self.sprite_sheets.get(&ch) {
            // Calculate the pixel position within the sprite sheet
            let pixel_x = (frame_x as u32 * sprite_sheet.frame_width + tx).min(sprite_sheet.image.width as u32 - 1);
            let pixel_y = (frame_y as u32 * sprite_sheet.frame_height + ty).min(sprite_sheet.image.height as u32 - 1);
            
            get_pixel_color(&sprite_sheet.image, pixel_x as i32, pixel_y as i32)
        } else {
            // Fallback to regular texture if no sprite sheet found
            self.get_pixel_color(ch, tx, ty)
        }
    }

    pub fn has_sprite_sheet(&self, ch: char) -> bool {
        self.sprite_sheets.contains_key(&ch)
    }

    pub fn get_sprite_frame_size(&self, ch: char) -> Option<(u32, u32)> {
        self.sprite_sheets.get(&ch).map(|sheet| (sheet.frame_width, sheet.frame_height))
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
