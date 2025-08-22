// player.rs

use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;
use crate::audio::AudioManager;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32, // field of view
    pub mouse_sensitivity: f32,
}

fn check_collision(maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
    if x < 0.0 || y < 0.0 {
        return true; // Out of bounds
    }
    
    let i = (x as usize) / block_size;
    let j = (y as usize) / block_size;
    
    if j >= maze.len() || i >= maze[0].len() {
        return true; // Out of bounds
    }
    
    // Treat 'p' (player spawn) as walkable space like ' '
    let cell = maze[j][i];
    cell != ' ' && cell != 'p' // Return true if it's a wall
}

pub fn process_events(
    player: &mut Player, 
    rl: &RaylibHandle, 
    maze: &Maze, 
    block_size: usize, 
    window_width: i32, 
    window_height: i32,
    audio_manager: &AudioManager,
    walking_sound: &Option<Sound>
) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;

    let mut is_moving = false;

    // Mouse camera control (always enabled)
    let mouse_pos = rl.get_mouse_position();
    let center_x = window_width as f32 / 2.0;
    let center_y = window_height as f32 / 2.0;
    
    let mouse_delta_x = mouse_pos.x - center_x;
    
    if mouse_delta_x.abs() > 1.0 {
        player.a += mouse_delta_x * player.mouse_sensitivity;
        // Reset mouse to center to prevent drift
        unsafe {
            raylib::ffi::SetMousePosition(center_x as i32, center_y as i32);
        }
    }

    // WASD movement
    if rl.is_key_down(KeyboardKey::KEY_W) {
        // Move forward
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
        if !check_collision(maze, new_x, new_y, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            is_moving = true;
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        // Move backward
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
        if !check_collision(maze, new_x, new_y, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            is_moving = true;
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        // Strafe left (perpendicular to current direction)
        let strafe_angle = player.a - PI / 2.0;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        if !check_collision(maze, new_x, new_y, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            is_moving = true;
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        // Strafe right (perpendicular to current direction)
        let strafe_angle = player.a + PI / 2.0;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        if !check_collision(maze, new_x, new_y, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            is_moving = true;
        }
    }

    // Keep arrow key controls for backwards compatibility
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
        if !check_collision(maze, new_x, new_y, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            is_moving = true;
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
        if !check_collision(maze, new_x, new_y, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            is_moving = true;
        }
    }

    // Handle walking sound based on movement
    if let Some(sound) = walking_sound {
        if is_moving {
            // Start playing sound if not already playing
            if !sound.is_playing() {
                audio_manager.play_footstep(sound);
            }
        } else {
            // Stop sound if playing and player stopped moving
            if sound.is_playing() {
                sound.stop();
            }
        }
    }
}
