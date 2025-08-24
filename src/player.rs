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
    pub is_attacking: bool,
    pub attack_timer: f32,
    pub attack_duration: f32,
    pub attack_cooldown: f32,
    pub enemy_hit_this_attack: bool, // Track if we hit an enemy during current attack
}

impl Player {
    pub fn new(pos: Vector2, a: f32, fov: f32, mouse_sensitivity: f32) -> Self {
        Player {
            pos,
            a,
            fov,
            mouse_sensitivity,
            is_attacking: false,
            attack_timer: 0.0,
            attack_duration: 0.25, // Faster attack duration for more responsive feel
            attack_cooldown: 0.0,
            enemy_hit_this_attack: false,
        }
    }

    pub fn start_attack(&mut self) {
        if !self.is_attacking && self.attack_cooldown <= 0.0 {
            self.is_attacking = true;
            self.attack_timer = self.attack_duration;
            self.attack_cooldown = 0.1; // Small cooldown to prevent spam clicking
            self.enemy_hit_this_attack = false; // Reset hit flag for new attack
        }
    }

    pub fn update_attack(&mut self, delta_time: f32) {
        if self.is_attacking {
            self.attack_timer -= delta_time;
            if self.attack_timer <= 0.0 {
                self.is_attacking = false;
                self.attack_timer = 0.0;
            }
        }
        
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= delta_time;
            if self.attack_cooldown < 0.0 {
                self.attack_cooldown = 0.0;
            }
        }
    }

    pub fn get_attack_progress(&self) -> f32 {
        if !self.is_attacking {
            return 0.0;
        }
        1.0 - (self.attack_timer / self.attack_duration)
    }
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
    walking_sound: &Option<Sound>,
    delta_time: f32
) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;
    const CONTROLLER_SENSITIVITY: f32 = 0.03; // Right stick sensitivity for looking
    const CONTROLLER_DEADZONE: f32 = 0.15; // Deadzone for analog sticks

    let mut is_moving = false;

    // Update attack state
    player.update_attack(delta_time);

    // Check if a gamepad is connected (PS5 controller)
    let gamepad_available = rl.is_gamepad_available(0);

    // Mouse camera control (only if no gamepad or gamepad right stick not being used)
    let mouse_pos = rl.get_mouse_position();
    let center_x = window_width as f32 / 2.0;
    let center_y = window_height as f32 / 2.0;
    
    let mouse_delta_x = mouse_pos.x - center_x;
    
    // Controller camera control takes priority over mouse
    if gamepad_available {
        let right_stick_x = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
        if right_stick_x.abs() > CONTROLLER_DEADZONE {
            player.a += right_stick_x * CONTROLLER_SENSITIVITY;
        } else if mouse_delta_x.abs() > 1.0 {
            // Fall back to mouse if right stick not being used
            player.a += mouse_delta_x * player.mouse_sensitivity;
            // Reset mouse to center to prevent drift
            unsafe {
                raylib::ffi::SetMousePosition(center_x as i32, center_y as i32);
            }
        }
    } else {
        // No gamepad, use mouse
        if mouse_delta_x.abs() > 1.0 {
            player.a += mouse_delta_x * player.mouse_sensitivity;
            // Reset mouse to center to prevent drift
            unsafe {
                raylib::ffi::SetMousePosition(center_x as i32, center_y as i32);
            }
        }
    }

    // Movement controls - Controller takes priority
    if gamepad_available {
        // Left stick for movement
        let left_stick_x = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
        let left_stick_y = rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
        
        // Forward/Backward (left stick Y-axis, inverted because up is negative)
        if left_stick_y.abs() > CONTROLLER_DEADZONE {
            let move_amount = -left_stick_y * MOVE_SPEED; // Negative because up should be forward
            let new_x = player.pos.x + move_amount * player.a.cos();
            let new_y = player.pos.y + move_amount * player.a.sin();
            if !check_collision(maze, new_x, new_y, block_size) {
                player.pos.x = new_x;
                player.pos.y = new_y;
                is_moving = true;
            }
        }
        
        // Strafe Left/Right (left stick X-axis)
        if left_stick_x.abs() > CONTROLLER_DEADZONE {
            let strafe_angle = player.a + PI / 2.0; // Right direction
            let move_amount = left_stick_x * MOVE_SPEED;
            let new_x = player.pos.x + move_amount * strafe_angle.cos();
            let new_y = player.pos.y + move_amount * strafe_angle.sin();
            if !check_collision(maze, new_x, new_y, block_size) {
                player.pos.x = new_x;
                player.pos.y = new_y;
                is_moving = true;
            }
        }
        
        // D-Pad as backup movement controls
        if rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) {
            // Move forward
            let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
            let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
            if !check_collision(maze, new_x, new_y, block_size) {
                player.pos.x = new_x;
                player.pos.y = new_y;
                is_moving = true;
            }
        }
        if rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
            // Move backward
            let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
            let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
            if !check_collision(maze, new_x, new_y, block_size) {
                player.pos.x = new_x;
                player.pos.y = new_y;
                is_moving = true;
            }
        }
        if rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
            // Strafe left
            let strafe_angle = player.a - PI / 2.0;
            let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
            let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
            if !check_collision(maze, new_x, new_y, block_size) {
                player.pos.x = new_x;
                player.pos.y = new_y;
                is_moving = true;
            }
        }
        if rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
            // Strafe right
            let strafe_angle = player.a + PI / 2.0;
            let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
            let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();
            if !check_collision(maze, new_x, new_y, block_size) {
                player.pos.x = new_x;
                player.pos.y = new_y;
                is_moving = true;
            }
        }
        
        // Shoulder buttons for rotation (as backup to right stick)
        if rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
            player.a -= ROTATION_SPEED;
        }
        if rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
            player.a += ROTATION_SPEED;
        }
    }

    // WASD movement (keyboard - works alongside or without controller)
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

    // Attack controls
    if gamepad_available {
        // R2 trigger (Right Trigger 2) for attack
        if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2) {
            player.start_attack();
        }
        // Alternative: Square button for attack
        if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
            player.start_attack();
        }
    }
    
    // Keyboard attack controls
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) || rl.is_key_pressed(KeyboardKey::KEY_E) {
        player.start_attack();
    }
    
    // Mouse attack control
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        player.start_attack();
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
