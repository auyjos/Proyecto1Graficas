// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod textures;

use line::line;
use maze::{Maze, load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};
use textures::TextureManager;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
mod enemy;
use enemy::{Enemy};

const TRANSPARENT_COLOR: Color = Color::new(152, 0, 136, 255);

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    // Calculate angle from player to enemy
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);

    // Normalize angle difference to [-PI, PI]
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > std::f32::consts::PI {
        angle_diff -= 2.0 * std::f32::consts::PI;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += 2.0 * std::f32::consts::PI;
    }

    // If enemy is outside player's FOV, skip drawing
    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    // Distance from player to enemy
    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    // Calculate sprite size on screen (scale inversely proportional to distance)
    let sprite_size = (screen_height / sprite_d) * 70.0;

    // Calculate horizontal screen position (centered)
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    // Calculate top-left corner of sprite on screen
    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;

    let sprite_size_usize = sprite_size as usize;

    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            // Map screen pixel to texture coordinates (assuming 128x128 texture)
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color('e', tx, ty);

            // Skip transparent pixels
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}


fn draw_cell(
  framebuffer: &mut Framebuffer,
  xo: usize,
  yo: usize,
  block_size: usize,
  cell: char,
) {
  if cell == ' ' {
    return;
  }
  framebuffer.set_current_color(Color::WHITE);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.set_pixel(x as u32, y as u32);
    }
  }
}

pub fn render_maze(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  for (row_index, row) in maze.iter().enumerate() {
    for (col_index, &cell) in row.iter().enumerate() {
      let xo = col_index * block_size;
      let yo = row_index * block_size;
      draw_cell(framebuffer, xo, yo, block_size, cell);
    }
  }

  framebuffer.set_current_color(Color::WHITESMOKE);

  let num_rays = 5;
  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32;
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    cast_ray(framebuffer, &maze, &player, a, block_size, true);
  }
}

fn render_world(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
  texture_cache: &TextureManager,
) {
  let num_rays = framebuffer.width;
  let hh = framebuffer.height as f32 / 2.0;

  // Draw sky and floor
  for i in 0..framebuffer.width {
    framebuffer.set_current_color(Color::SKYBLUE);
    for j in 0..(framebuffer.height / 2) {
      framebuffer.set_pixel(i, j);
    }
    framebuffer.set_current_color(Color::GAINSBORO);
    for j in (framebuffer.height / 2)..framebuffer.height {
      framebuffer.set_pixel(i, j);
    }
  }

  framebuffer.set_current_color(Color::WHITESMOKE);

  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32;
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    let distance_to_wall = intersect.distance;
    let distance_to_projection_plane = 70.0;
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;

    for y in stake_top..stake_bottom {
      let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0;

      let color = texture_cache.get_pixel_color(intersect.impact, intersect.tx as u32, ty as u32);
      framebuffer.set_current_color(color);
      framebuffer.set_pixel(i, y as u32);
    }
  }
}

fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, texture_cache: &TextureManager) {
  let enemies = vec![
    Enemy::new(250.0, 250.0, 'e'),
    // Enemy::new(450.0, 450.0, 'e'),
    // Enemy::new(650.0, 650.0, 'e'),
  ];

  for enemy in &enemies {
    draw_sprite(framebuffer, &player, enemy, texture_cache);
  }
}

fn render_minimap(
  d: &mut RaylibDrawHandle,
  maze: &Maze,
  player: &Player,
  block_size: usize,
  screen_width: i32,
  screen_height: i32,
) {
  let minimap_size = 200; // Size of the minimap in pixels
  let minimap_scale = 8;  // Each maze cell will be 8x8 pixels in the minimap
  
  // Position minimap in lower middle of screen
  let minimap_x = (screen_width - minimap_size) / 2;
  let minimap_y = screen_height - minimap_size - 20; // 20 pixels from bottom
  
  // Draw semi-transparent background for minimap
  d.draw_rectangle(minimap_x - 5, minimap_y - 5, minimap_size + 10, minimap_size + 10, Color::new(0, 0, 0, 180));
  d.draw_rectangle_lines(minimap_x - 5, minimap_y - 5, minimap_size + 10, minimap_size + 10, Color::WHITE);
  
  // Calculate which part of the maze to show (centered on player)
  let player_maze_x = (player.pos.x / block_size as f32) as i32;
  let player_maze_y = (player.pos.y / block_size as f32) as i32;
  
  let minimap_cells = minimap_size / minimap_scale; // How many maze cells fit in minimap
  let half_cells = minimap_cells / 2;
  
  // Draw maze cells
  for dy in -half_cells..half_cells {
    for dx in -half_cells..half_cells {
      let maze_x = player_maze_x + dx;
      let maze_y = player_maze_y + dy;
      
      // Check bounds
      if maze_y >= 0 && maze_y < maze.len() as i32 && 
         maze_x >= 0 && maze_x < maze[0].len() as i32 {
        
        let cell = maze[maze_y as usize][maze_x as usize];
        let color = match cell {
          ' ' => Color::new(40, 40, 40, 255),   // Floor - dark gray
          _ => Color::new(100, 100, 100, 255),  // Wall - light gray
        };
        
        let pixel_x = minimap_x + (dx + half_cells) * minimap_scale;
        let pixel_y = minimap_y + (dy + half_cells) * minimap_scale;
        
        d.draw_rectangle(pixel_x, pixel_y, minimap_scale, minimap_scale, color);
      }
    }
  }
  
  // Draw player position as a red dot in the center
  let player_pixel_x = minimap_x + minimap_size / 2;
  let player_pixel_y = minimap_y + minimap_size / 2;
  d.draw_circle(player_pixel_x, player_pixel_y, 3.0, Color::RED);
  
  // Draw player direction as a line
  let direction_length = 8.0;
  let end_x = player_pixel_x as f32 + direction_length * player.a.cos();
  let end_y = player_pixel_y as f32 + direction_length * player.a.sin();
  d.draw_line_ex(
    Vector2::new(player_pixel_x as f32, player_pixel_y as f32),
    Vector2::new(end_x, end_y),
    2.0,
    Color::YELLOW
  );
  
  // Add minimap label
  d.draw_text("MINIMAP", minimap_x, minimap_y - 25, 16, Color::WHITE);
}

fn main() {
  // Use your actual screen resolution
  let mut window_width = 1980;
  let mut window_height = 1200;
  let block_size = 100;

  let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Raycaster Example")
    .log_level(TraceLogLevel::LOG_WARNING)
    .resizable()
    .vsync()
    .build();

  // Start in fullscreen mode and get the actual screen dimensions
  window.toggle_fullscreen();
  
  // Wait a frame for fullscreen to take effect
  std::thread::sleep(std::time::Duration::from_millis(100));
  
  // Check what raylib reports vs what we know is correct
  let reported_width = window.get_screen_width();
  let reported_height = window.get_screen_height();
  
  println!("Your actual screen: 1980x1200");
  println!("Raylib reports: {}x{}", reported_width, reported_height);
  
  // Use your actual screen dimensions regardless of what raylib reports
  window_width = 1920;
  window_height = 1080;

  let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
  framebuffer.set_background_color(Color::new(50, 50, 100, 255));

  let maze = load_maze("maze.txt");
  let mut player = Player {
    pos: Vector2::new(150.0, 150.0),
    a: PI / 3.0,
    fov: PI / 3.0,
    mouse_sensitivity: 0.01, // Increased sensitivity for more noticeable movement
  };

  // Enable mouse cursor hiding and capturing for FPS-style controls
  window.disable_cursor();
  window.set_mouse_position(Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0));

  // Initialize texture cache once
  let texture_cache = TextureManager::new(&mut window, &raylib_thread);

  let mut mouse_control_enabled = true;
  let mut show_minimap = false; // Toggle for minimap display

  //window.set_target_fps(60); // Set target FPS to 60 for smoother rendering

  while !window.window_should_close() {
    // Always ensure framebuffer matches current window size
    let current_width = window.get_screen_width();
    let current_height = window.get_screen_height();
    if current_width != window_width || current_height != window_height || 
       framebuffer.width != current_width as u32 || framebuffer.height != current_height as u32 {
      window_width = current_width;
      window_height = current_height;
      framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
      framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    }

    framebuffer.clear();

    // Toggle fullscreen with F11
    if window.is_key_pressed(KeyboardKey::KEY_F11) {
      window.toggle_fullscreen();
      window_width = window.get_screen_width();
      window_height = window.get_screen_height();
      framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
      framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    }

    // Toggle mouse control with ESC key
    if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
      mouse_control_enabled = !mouse_control_enabled;
      if mouse_control_enabled {
        window.disable_cursor();
      } else {
        window.enable_cursor();
      }
    }

    // Process events with mouse control state
    process_events(&mut player, &window, &maze, block_size, mouse_control_enabled, window_width, window_height);

    // Toggle minimap with M key
    if window.is_key_pressed(KeyboardKey::KEY_M) {
      show_minimap = !show_minimap;
    }

    // Always render in 3D mode
    render_world(&mut framebuffer, &maze, block_size, &player, &texture_cache);
    render_enemies(&mut framebuffer, &player, &texture_cache);

        // Create texture from framebuffer
    if let Ok(framebuffer_texture) = framebuffer.get_texture(&mut window, &raylib_thread) {
        // Get raylib reported dimensions before drawing
        let raylib_width = window.get_screen_width();
        let raylib_height = window.get_screen_height();
        
        // Single drawing operation to reduce flickering
        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::BLACK);
        
        // Draw texture to exactly fill the screen
        d.draw_texture_ex(&framebuffer_texture, Vector2::zero(), 0.0, 1.0, Color::WHITE);
        
        // Draw UI elements on top
        d.draw_text(&format!("FPS: {}", d.get_fps()), 10, 10, 20, Color::WHITE);
        d.draw_text("ESC: Toggle mouse control", 10, 35, 16, Color::WHITE);
        d.draw_text("M: Toggle minimap", 10, 55, 16, Color::WHITE);
        d.draw_text("F11: Toggle fullscreen", 10, 75, 16, Color::WHITE);
        d.draw_text(&format!("Minimap: {}", if show_minimap { "ON" } else { "OFF" }), 10, 95, 16, Color::WHITE);
        d.draw_text(&format!("Using: {}x{}, FB: {}x{}, Raylib: {}x{}", 
                   window_width, window_height, framebuffer.width, framebuffer.height,
                   raylib_width, raylib_height), 10, 115, 16, Color::YELLOW);
        
        // Render minimap if enabled
        if show_minimap {
          render_minimap(&mut d, &maze, &player, block_size, window_width, window_height);
        }
    }
  }
}
