use raylib::prelude::*;
use crate::textures::TextureManager;
use crate::maze::Maze;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    Attack,
    Death,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MovementPattern {
    Stationary,     // Doesn't move
    Patrol,         // Moves back and forth between two points
    Wander,         // Random movement within an area
    Chase,          // Moves toward the player when close
}

pub struct Enemy {
    pub pos: Vector2,
    pub texture_key: char, // key to fetch texture from TextureManager
    pub animation_state: AnimationState,
    pub current_frame: usize,
    pub animation_timer: f32,
    pub frame_duration: f32, // Time per frame in seconds
    pub facing_left: bool, // Direction the sprite is facing
    pub is_dead: bool, // Track if enemy is dead
    pub death_timer: f32, // How long the enemy has been dead
    
    // Movement properties
    pub movement_pattern: MovementPattern,
    pub movement_speed: f32,
    pub patrol_start: Vector2,
    pub patrol_end: Vector2,
    pub patrol_direction: f32, // 1.0 for forward, -1.0 for backward
    pub wander_center: Vector2,
    pub wander_radius: f32,
    pub movement_timer: f32,
    pub target_pos: Vector2,
}

impl Enemy {
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        Enemy {
            pos: Vector2::new(x, y),
            texture_key,
            animation_state: AnimationState::Idle,
            current_frame: 0,
            animation_timer: 0.0,
            frame_duration: 0.2, // 200ms per frame = 5 FPS animation
            facing_left: false,
            is_dead: false,
            death_timer: 0.0,
            
            // Movement defaults
            movement_pattern: MovementPattern::Stationary,
            movement_speed: 50.0, // pixels per second
            patrol_start: Vector2::new(x, y),
            patrol_end: Vector2::new(x, y),
            patrol_direction: 1.0,
            wander_center: Vector2::new(x, y),
            wander_radius: 100.0,
            movement_timer: 0.0,
            target_pos: Vector2::new(x, y),
        }
    }

    // Constructor for patrol enemies
    pub fn new_patrol(x: f32, y: f32, texture_key: char, end_x: f32, end_y: f32) -> Self {
        let mut enemy = Self::new(x, y, texture_key);
        enemy.movement_pattern = MovementPattern::Patrol;
        enemy.patrol_start = Vector2::new(x, y);
        enemy.patrol_end = Vector2::new(end_x, end_y);
        enemy.target_pos = enemy.patrol_end;
        enemy
    }

    // Constructor for wandering enemies
    pub fn new_wander(x: f32, y: f32, texture_key: char, radius: f32) -> Self {
        let mut enemy = Self::new(x, y, texture_key);
        enemy.movement_pattern = MovementPattern::Wander;
        enemy.wander_radius = radius;
        enemy
    }

    // Constructor for chasing enemies
    pub fn new_chase(x: f32, y: f32, texture_key: char) -> Self {
        let mut enemy = Self::new(x, y, texture_key);
        enemy.movement_pattern = MovementPattern::Chase;
        enemy.movement_speed = 75.0; // Slightly faster for chase
        enemy
    }

    pub fn update(&mut self, delta_time: f32, player_pos: Vector2, maze: &Maze, block_size: usize) {
        // Update death timer if dead
        if self.is_dead {
            self.death_timer += delta_time;
            // Don't move if dead
        } else {
            // Handle movement based on pattern
            self.update_movement(delta_time, player_pos, maze, block_size);
        }
        
        // Update animation timer
        self.animation_timer += delta_time;
        
        if self.animation_timer >= self.frame_duration {
            self.animation_timer = 0.0;
            
            // Determine number of frames for current animation
            let max_frames = match self.animation_state {
                AnimationState::Idle => 4,     // 4 idle frames
                AnimationState::Walking => 4,  // 4 walking frames  
                AnimationState::Attack => 4,   // 4 attack frames
                AnimationState::Death => 4,    // 4 death frames
            };
            
            // If dead, don't loop the death animation, stay on last frame
            if self.is_dead && self.animation_state == AnimationState::Death {
                self.current_frame = (self.current_frame + 1).min(max_frames - 1);
            } else {
                self.current_frame = (self.current_frame + 1) % max_frames;
            }
        }
    }

    fn update_movement(&mut self, delta_time: f32, player_pos: Vector2, maze: &Maze, block_size: usize) {
        self.movement_timer += delta_time;
        
        match self.movement_pattern {
            MovementPattern::Stationary => {
                // Don't move, just stay idle
                self.set_animation(AnimationState::Idle);
            }
            
            MovementPattern::Patrol => {
                self.update_patrol_movement(delta_time, maze, block_size);
            }
            
            MovementPattern::Wander => {
                self.update_wander_movement(delta_time, maze, block_size);
            }
            
            MovementPattern::Chase => {
                self.update_chase_movement(delta_time, player_pos, maze, block_size);
            }
        }
    }

    fn update_patrol_movement(&mut self, delta_time: f32, maze: &Maze, block_size: usize) {
        let move_distance = self.movement_speed * delta_time;
        
        // Calculate direction to target
        let dx = self.target_pos.x - self.pos.x;
        let dy = self.target_pos.y - self.pos.y;
        let distance_to_target = (dx * dx + dy * dy).sqrt();
        
        if distance_to_target < 10.0 {
            // Reached target, switch direction
            if self.target_pos.x == self.patrol_end.x && self.target_pos.y == self.patrol_end.y {
                self.target_pos = self.patrol_start;
            } else {
                self.target_pos = self.patrol_end;
            }
        } else {
            // Move toward target
            let move_x = (dx / distance_to_target) * move_distance;
            let move_y = (dy / distance_to_target) * move_distance;
            
            let new_pos = Vector2::new(self.pos.x + move_x, self.pos.y + move_y);
            
            if !self.would_collide_with_wall(new_pos, maze, block_size) {
                self.pos = new_pos;
                self.set_animation(AnimationState::Walking);
                
                // Update facing direction
                self.facing_left = move_x < 0.0;
            } else {
                self.set_animation(AnimationState::Idle);
            }
        }
    }

    fn update_wander_movement(&mut self, delta_time: f32, maze: &Maze, block_size: usize) {
        // Change direction every 2-4 seconds
        if self.movement_timer > 2.0 + (self.pos.x as i32 % 3) as f32 {
            self.movement_timer = 0.0;
            
            // Pick a random point within wander radius
            let angle = (self.pos.x + self.pos.y) * 0.01; // Pseudo-random based on position
            let distance = self.wander_radius * 0.5 + (self.wander_radius * 0.5 * angle.sin().abs());
            
            self.target_pos = Vector2::new(
                self.wander_center.x + distance * angle.cos(),
                self.wander_center.y + distance * angle.sin(),
            );
        }
        
        // Move toward current target
        let move_distance = self.movement_speed * delta_time * 0.7; // Slower wandering
        let dx = self.target_pos.x - self.pos.x;
        let dy = self.target_pos.y - self.pos.y;
        let distance_to_target = (dx * dx + dy * dy).sqrt();
        
        if distance_to_target > 5.0 {
            let move_x = (dx / distance_to_target) * move_distance;
            let move_y = (dy / distance_to_target) * move_distance;
            
            let new_pos = Vector2::new(self.pos.x + move_x, self.pos.y + move_y);
            
            if !self.would_collide_with_wall(new_pos, maze, block_size) {
                self.pos = new_pos;
                self.set_animation(AnimationState::Walking);
                self.facing_left = move_x < 0.0;
            } else {
                self.set_animation(AnimationState::Idle);
            }
        } else {
            self.set_animation(AnimationState::Idle);
        }
    }

    fn update_chase_movement(&mut self, delta_time: f32, player_pos: Vector2, maze: &Maze, block_size: usize) {
        let dx = player_pos.x - self.pos.x;
        let dy = player_pos.y - self.pos.y;
        let distance_to_player = (dx * dx + dy * dy).sqrt();
        
        // Only chase if player is within reasonable range
        if distance_to_player < 300.0 && distance_to_player > 20.0 {
            let move_distance = self.movement_speed * delta_time;
            let move_x = (dx / distance_to_player) * move_distance;
            let move_y = (dy / distance_to_player) * move_distance;
            
            let new_pos = Vector2::new(self.pos.x + move_x, self.pos.y + move_y);
            
            if !self.would_collide_with_wall(new_pos, maze, block_size) {
                self.pos = new_pos;
                self.set_animation(AnimationState::Walking);
                self.facing_left = move_x < 0.0;
            } else {
                self.set_animation(AnimationState::Idle);
            }
        } else {
            self.set_animation(AnimationState::Idle);
        }
    }

    fn would_collide_with_wall(&self, new_pos: Vector2, maze: &Maze, block_size: usize) -> bool {
        let margin = 20.0; // Collision margin around enemy
        
        // Check corners of enemy collision box
        let corners = [
            (new_pos.x - margin, new_pos.y - margin),
            (new_pos.x + margin, new_pos.y - margin),
            (new_pos.x - margin, new_pos.y + margin),
            (new_pos.x + margin, new_pos.y + margin),
        ];
        
        for (x, y) in corners.iter() {
            let maze_x = (*x / block_size as f32) as usize;
            let maze_y = (*y / block_size as f32) as usize;
            
            if maze_y < maze.len() && maze_x < maze[0].len() {
                if maze[maze_y][maze_x] != ' ' {
                    return true; // Would collide with wall
                }
            } else {
                return true; // Out of bounds
            }
        }
        
        false
    }

    pub fn kill(&mut self) {
        if !self.is_dead {
            self.is_dead = true;
            self.death_timer = 0.0;
            self.animation_state = AnimationState::Death;
            self.current_frame = 0;
            self.animation_timer = 0.0;
        }
    }

    pub fn should_despawn(&self) -> bool {
        self.is_dead && self.death_timer > 3.0 // Despawn after 3 seconds
    }

    pub fn set_animation(&mut self, new_state: AnimationState) {
        if matches!(self.animation_state, AnimationState::Death) {
            return; // Don't change animation if dead
        }
        
        if !std::mem::discriminant(&self.animation_state).eq(&std::mem::discriminant(&new_state)) {
            self.animation_state = new_state;
            self.current_frame = 0;
            self.animation_timer = 0.0;
        }
    }
}
