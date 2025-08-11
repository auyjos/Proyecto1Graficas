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
                // Check depth buffer - only render if sprite is closer than existing pixel
                let current_depth = framebuffer.get_depth(x as u32, y as u32);
                if sprite_d < current_depth {
                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel_with_depth(x as u32, y as u32, sprite_d);
                }
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
  performance_mode: bool,
) {
  let num_rays = framebuffer.width;
  let hh = framebuffer.height as f32 / 2.0;

  // Draw sky and floor - use simple or detailed based on performance mode
  if performance_mode {
    // Simple, fast sky and floor for performance mode
    framebuffer.set_current_color(Color::SKYBLUE);
    for i in 0..framebuffer.width {
      for j in 0..(framebuffer.height / 2) {
        framebuffer.set_pixel_with_depth(i, j, 10000.0);
      }
    }
    framebuffer.set_current_color(Color::GAINSBORO);
    for i in 0..framebuffer.width {
      for j in (framebuffer.height / 2)..framebuffer.height {
        framebuffer.set_pixel_with_depth(i, j, 10000.0);
      }
    }
  } else {
    // Detailed gradients for quality mode
    let mut sky_colors = Vec::with_capacity((framebuffer.height / 2) as usize);
    let mut floor_colors = Vec::with_capacity((framebuffer.height / 2) as usize);
    
    for j in 0..(framebuffer.height / 2) {
      let gradient_factor = j as f32 / (framebuffer.height as f32 / 2.0);
      sky_colors.push(Color::new(
        (30.0 + gradient_factor * 105.0) as u8,
        (30.0 + gradient_factor * 176.0) as u8,
        (60.0 + gradient_factor * 175.0) as u8,
        255
      ));
    }
    
    for j in 0..(framebuffer.height / 2) {
      let distance_from_center = j as f32;
      let fog_factor = (distance_from_center / (framebuffer.height as f32 / 2.0)).min(1.0);
      floor_colors.push(Color::new(
        (40.0 + fog_factor * 40.0) as u8,
        (35.0 + fog_factor * 35.0) as u8,
        (30.0 + fog_factor * 30.0) as u8,
        255
      ));
    }

    // Draw sky and floor with pre-calculated colors
    for i in 0..framebuffer.width {
      // Sky
      for j in 0..(framebuffer.height / 2) {
        framebuffer.set_current_color(sky_colors[j as usize]);
        framebuffer.set_pixel_with_depth(i, j, 10000.0);
      }
      
      // Floor
      for j in (framebuffer.height / 2)..framebuffer.height {
        let floor_index = (j - framebuffer.height / 2) as usize;
        if floor_index < floor_colors.len() {
          framebuffer.set_current_color(floor_colors[floor_index]);
          framebuffer.set_pixel_with_depth(i, j, 10000.0);
        }
      }
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

      let mut color = texture_cache.get_pixel_color(intersect.impact, intersect.tx as u32, ty as u32);
      
      // Only apply fog in quality mode for better performance
      if !performance_mode && distance_to_wall > 200.0 {
        let fog_factor = ((distance_to_wall - 200.0) * 0.003333).min(0.7); // Pre-calculate division
        
        // Faster color blending
        let inv_fog = 1.0 - fog_factor;
        color = Color::new(
          (color.r as f32 * inv_fog + 60.0 * fog_factor) as u8,
          (color.g as f32 * inv_fog + 60.0 * fog_factor) as u8,
          (color.b as f32 * inv_fog + 90.0 * fog_factor) as u8,
          255
        );
      }
      
      framebuffer.set_current_color(color);
      framebuffer.set_pixel_with_depth(i, y as u32, distance_to_wall);
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

fn render_pause_menu(
  d: &mut RaylibDrawHandle,
  selected_option: usize,
  screen_width: i32,
  screen_height: i32,
) {
  // Draw semi-transparent overlay
  d.draw_rectangle(0, 0, screen_width, screen_height, Color::new(0, 0, 0, 180));
  
  // Calculate menu position (center of screen)
  let menu_width = 300;
  let menu_height = 200;
  let menu_x = (screen_width - menu_width) / 2;
  let menu_y = (screen_height - menu_height) / 2;
  
  // Draw menu background
  d.draw_rectangle(menu_x, menu_y, menu_width, menu_height, Color::new(40, 40, 40, 240));
  d.draw_rectangle_lines(menu_x, menu_y, menu_width, menu_height, Color::WHITE);
  
  // Draw title
  let title = "GAME PAUSED";
  let title_width = 24 * title.len() as i32 / 2; // Approximate text width
  d.draw_text(title, menu_x + (menu_width - title_width) / 2, menu_y + 30, 24, Color::WHITE);
  
  // Draw menu options
  let options = ["Resume", "Quit"];
  for (i, option) in options.iter().enumerate() {
    let y_pos = menu_y + 80 + (i as i32 * 40);
    let color = if i == selected_option { Color::YELLOW } else { Color::WHITE };
    let prefix = if i == selected_option { "> " } else { "  " };
    
    let text = format!("{}{}", prefix, option);
    let text_width = 20 * text.len() as i32 / 2; // Approximate text width
    d.draw_text(&text, menu_x + (menu_width - text_width) / 2, y_pos, 20, color);
  }
  
  // Draw controls
  d.draw_text("Use UP/DOWN or W/S to navigate", menu_x + 20, menu_y + menu_height - 40, 14, Color::LIGHTGRAY);
  d.draw_text("Press ENTER or SPACE to select", menu_x + 20, menu_y + menu_height - 20, 14, Color::LIGHTGRAY);
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

  // Disable the default ESC key for closing the window
  window.set_exit_key(None);

  // Start in fullscreen mode and get the actual screen dimensions
  window.toggle_fullscreen();
  
  // Wait a frame for fullscreen to take effect
  std::thread::sleep(std::time::Duration::from_millis(100));
  
  // Check what raylib reports vs what we know is correct
  let reported_width = window.get_screen_width();
  let reported_height = window.get_screen_height();
  
  println!("Your actual screen: 1980x1200");
  println!("Raylib reports: {}x{}", reported_width, reported_height);
  
  // Use the correct screen dimensions
  window_width = 1980;
  window_height = 1200;

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

  let mut show_minimap = false; // Toggle for minimap display
  let mut game_paused = false; // Toggle for pause menu
  let mut selected_menu_option = 0; // 0 = Resume, 1 = Quit
  let mut performance_mode = false; // Toggle for performance vs quality

  window.set_target_fps(60); // Set target FPS to 60 for consistent performance

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

    // Toggle pause menu with ESC key
    if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
      game_paused = !game_paused;
      if game_paused {
        window.enable_cursor();
      } else {
        window.disable_cursor();
        window.set_mouse_position(Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0));
      }
    }

    // Handle pause menu navigation
    if game_paused {
      if window.is_key_pressed(KeyboardKey::KEY_UP) || window.is_key_pressed(KeyboardKey::KEY_W) {
        selected_menu_option = if selected_menu_option == 0 { 1 } else { 0 };
      }
      if window.is_key_pressed(KeyboardKey::KEY_DOWN) || window.is_key_pressed(KeyboardKey::KEY_S) {
        selected_menu_option = if selected_menu_option == 1 { 0 } else { 1 };
      }
      if window.is_key_pressed(KeyboardKey::KEY_ENTER) || window.is_key_pressed(KeyboardKey::KEY_SPACE) {
        match selected_menu_option {
          0 => {
            // Resume game
            game_paused = false;
            window.disable_cursor();
            window.set_mouse_position(Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0));
          },
          1 => {
            // Quit game
            break;
          },
          _ => {}
        }
      }
    }

    // Process events only when not paused (always enable mouse control)
    if !game_paused {
      process_events(&mut player, &window, &maze, block_size, window_width, window_height);
    }

    // Toggle minimap with M key (only when not paused)
    if !game_paused && window.is_key_pressed(KeyboardKey::KEY_M) {
      show_minimap = !show_minimap;
    }

    // Toggle performance mode with P key (only when not paused)
    if !game_paused && window.is_key_pressed(KeyboardKey::KEY_P) {
      performance_mode = !performance_mode;
    }

    // Only render game when not paused
    if !game_paused {
      // Always render in 3D mode
      render_world(&mut framebuffer, &maze, block_size, &player, &texture_cache, performance_mode);
      render_enemies(&mut framebuffer, &player, &texture_cache);
    }

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
        d.draw_text("ESC: Pause menu", 10, 35, 16, Color::WHITE);
        d.draw_text("M: Toggle minimap", 10, 55, 16, Color::WHITE);
        d.draw_text("P: Toggle performance mode", 10, 75, 16, Color::WHITE);
        d.draw_text("F11: Toggle fullscreen", 10, 95, 16, Color::WHITE);
        d.draw_text(&format!("Minimap: {}", if show_minimap { "ON" } else { "OFF" }), 10, 115, 16, Color::WHITE);
        d.draw_text(&format!("Performance: {}", if performance_mode { "HIGH" } else { "QUALITY" }), 10, 135, 16, Color::WHITE);
        d.draw_text(&format!("Using: {}x{}, FB: {}x{}, Raylib: {}x{}", 
                   window_width, window_height, framebuffer.width, framebuffer.height,
                   raylib_width, raylib_height), 10, 155, 16, Color::YELLOW);
        
        // Render minimap if enabled and not paused
        if show_minimap && !game_paused {
          render_minimap(&mut d, &maze, &player, block_size, window_width, window_height);
        }
        
        // Render pause menu if game is paused
        if game_paused {
          render_pause_menu(&mut d, selected_menu_option, window_width, window_height);
        }
    }
  }
}
