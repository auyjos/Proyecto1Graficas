use raylib::prelude::*;

pub struct AudioManager {
    music_volume: f32,
    sfx_volume: f32,
    is_music_enabled: bool,
    is_sfx_enabled: bool,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    pub fn new() -> Self {
        AudioManager {
            music_volume: 0.5,
            sfx_volume: 0.7,
            is_music_enabled: true,
            is_sfx_enabled: true,
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
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

    pub fn is_sfx_enabled(&self) -> bool {
        self.is_sfx_enabled
    }

    pub fn set_sfx_enabled(&mut self, enabled: bool) {
        self.is_sfx_enabled = enabled;
    }

    pub fn toggle_sfx(&mut self) {
        self.is_sfx_enabled = !self.is_sfx_enabled;
    }

    pub fn play_footstep(&self, sound: &Sound) {
        if self.is_sfx_enabled {
            // Direct sound playback using Sound's methods
            sound.play();
        }
    }

    pub fn set_sound_volume(&self, sound: &mut Sound, volume_multiplier: f32) {
        sound.set_volume(self.sfx_volume * volume_multiplier);
    }

    pub fn setup_walking_sound(&self, walking_sound: &mut Option<Sound>) {
        if let Some(sound) = walking_sound {
            self.set_sound_volume(sound, 0.5); // Set walking sound volume to half of SFX volume
        }
    }
}
