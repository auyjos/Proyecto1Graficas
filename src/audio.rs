use raylib::prelude::*;

pub struct AudioManager {
    music_volume: f32,
    is_music_enabled: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        AudioManager {
            music_volume: 0.5,
            is_music_enabled: true,
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn is_music_enabled(&self) -> bool {
        self.is_music_enabled
    }

    pub fn set_music_enabled(&mut self, enabled: bool) {
        self.is_music_enabled = enabled;
    }

    pub fn toggle_music(&mut self) {
        self.is_music_enabled = !self.is_music_enabled;
    }
}
