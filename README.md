# 🎮 Raycaster Game Engine - Advanced 3D FPS Experience

A sophisticated first-person raycaster game built in Rust using Raylib, featuring advanced graphics, dynamic enemy AI, immersive audio, and comprehensive controller support.

## 🌟 Project Overview

This project is a fully-featured 3D raycaster game engine that demonstrates advanced game development concepts including real-time 3D rendering, complex AI systems, spatial audio, and modern input handling. Built from the ground up using Rust and Raylib 5.5.1.

## ✨ Key Features

### 🎯 **Core Gameplay**
- **First-Person 3D Raycasting**: Advanced raycasting engine with textured walls and perspective-correct rendering
- **Dynamic Enemy System**: 31 enemies per level with 4 different AI behavior types
- **Combat System**: Sword-based melee combat with visual and audio feedback
- **Multiple Map Support**: Three distinct maze layouts with automatic map progression
- **Goal-Based Progression**: Reach the goal to advance to the next level

### 🎮 **Input & Controls**
- **Dual Input Support**: Full keyboard + mouse and gamepad support
- **PS5 Controller Integration**: Native PlayStation 5 controller support with haptic feedback
- **Configurable Controls**: Customizable key bindings and sensitivity settings
- **Smooth Movement**: Delta-time based movement for consistent performance across framerates

### 🎵 **Advanced Audio System**
- **Map-Specific Music**: Unique background tracks for each level
  - Map 1: `ghosts.mp3` - Atmospheric horror ambience
  - Map 2: `behelit.mp3` - Dark fantasy soundtrack  
  - Map 3: `blood_guts.mp3` - Intense combat music
- **3D Spatial Audio**: Positional walking sounds with proper volume control
- **Combat Audio Feedback**: 
  - Sword swing sounds when attacking
  - Impact sounds when hitting enemies
  - Death sounds when enemies are defeated
- **Dynamic Volume Control**: Separate music and SFX volume controls

### 🤖 **Intelligent Enemy AI**
Four distinct enemy types with unique behaviors:
- **Patrol Enemies**: Follow predefined routes between waypoints
- **Wandering Enemies**: Random movement within defined radius areas
- **Chase Enemies**: Actively pursue the player when in range
- **Guard Enemies**: Stationary sentries protecting key areas

### 🎨 **Visual Systems**
- **Texture Management**: Advanced texture loading with RGBA format support
- **Animated Sprites**: Multi-frame enemy animations (idle, walking, attack, death)
- **Dynamic Weapon Display**: Always-visible sword with attack animations
- **Performance Modes**: Quality vs. performance rendering options
- **Minimap System**: Optional overhead view for navigation
- **Debug Overlays**: Real-time performance and game state information

### 🔧 **Technical Architecture**

#### **Modular Code Structure**
```
src/
├── main.rs          # Core game loop and rendering pipeline
├── player.rs        # Player state and movement systems
├── enemy.rs         # Enemy AI and behavior logic
├── maze.rs          # Level generation and collision detection
├── textures.rs      # Texture loading and management
├── audio.rs         # Audio system and sound management
├── framebuffer.rs   # Low-level rendering buffer
├── caster.rs        # Raycasting algorithm implementation
└── line.rs          # Line drawing utilities
```

#### **Performance Features**
- **Optimized Raycasting**: Efficient wall detection with early termination
- **Dynamic Enemy Culling**: Only render enemies within player's field of view
- **Texture Caching**: Smart texture loading and memory management
- **Delta-Time Movement**: Frame-rate independent physics
- **Configurable Quality**: Adjustable rendering quality for different hardware

## 🛠️ **Build Requirements**

### **Dependencies**
- **Rust** (latest stable version)
- **Raylib 5.5.1** (automatically handled by Cargo)
- **ImageMagick** (for texture conversion)

### **System Requirements**
- **OS**: Linux, Windows, macOS
- **Memory**: 2GB RAM minimum
- **Graphics**: OpenGL 3.3+ compatible GPU
- **Audio**: Standard audio device for sound output
- **Controllers**: PS5 controller support (optional)

## 🚀 **Installation & Running**

### **Quick Start**
```bash
# Clone the repository
git clone <repository-url>
cd Proyecto1

# Build and run (debug mode)
cargo run

# Build and run (optimized release mode)
cargo run --release

# Build only
cargo build --release
```

### **Texture Setup**
Convert any new textures to RGBA format:
```bash
# Convert single texture
magick input_texture.png -alpha on output_texture_rgba.png

# Convert all textures using provided script
./convert_to_rgba.sh
```

## 🎮 **Controls**

### **Keyboard + Mouse**
- **W, A, S, D**: Movement (forward, strafe left, backward, strafe right)
- **Mouse**: Look around / Camera rotation
- **Left Click**: Attack with sword
- **M**: Toggle minimap
- **ESC**: Pause menu
- **Plus/Minus**: Adjust music volume
- **Tab**: Toggle performance mode

### **PS5 Controller**
- **Left Stick**: Movement
- **Right Stick**: Camera rotation  
- **R2 Trigger**: Attack with sword
- **Options Button**: Pause menu
- **TouchPad**: Toggle minimap
- **D-Pad Up/Down**: Adjust volume

## 🎯 **Gameplay Mechanics**

### **Combat System**
- **Sword Range**: 150-unit attack radius with 30° cone
- **Attack Timing**: Attacks have cooldown periods to prevent spam
- **Visual Feedback**: Sword position adjusts during attacks (left/down movement)
- **Audio Feedback**: Different sounds for successful hits vs. missed attacks

### **Enemy Behavior**
- **Dynamic Positioning**: Enemies spawn in valid maze locations
- **Collision Avoidance**: Smart pathfinding around walls
- **State Management**: Idle, walking, attacking, and death animations
- **Player Interaction**: Enemies react to player proximity with aggressive behavior

### **Level Progression**
- **Goal System**: Find and reach the goal marker ('g') in each maze
- **Automatic Advancement**: Seamless transition between levels
- **Increasing Difficulty**: Larger mazes and more complex enemy patterns

## 📁 **Asset Structure**

```
assets/
├── textures/
│   ├── elements/          # Wall and environmental textures
│   ├── metals/           # Metallic surface textures
│   └── large_door_rgba.png # Special door textures
├── sounds/
│   ├── music/            # Background music tracks
│   │   ├── ghosts.mp3
│   │   ├── behelit.mp3
│   │   └── blood_guts.mp3
│   ├── walk.mp3          # Footstep sounds
│   ├── sword_sound.mp3   # Combat audio
│   ├── splat.mp3         # Hit effects
│   └── death.mp3         # Enemy death sounds
├── sprite1_rgba.png      # Enemy sprite texture
├── sprite_sheet_rgba.png # Animated enemy frames
└── sword2.png            # Weapon texture
```

## 🎬 **Video Demonstration**

### **Controller Functionality Demo**
🎥 **[Link to Video Demonstration]** - (https://youtube.com/shorts/y-aDI8vax6c)

*This section will contain a link to a comprehensive video showing:*
- *Complete PS5 controller integration*
- *All movement and combat controls*
- *Audio system demonstration*
- *Enemy AI behaviors*
- *Level progression and map transitions*
- *Performance features and quality settings*

## 🔧 **Development Features**

### **Debug Tools**
- **Real-time Performance Metrics**: FPS, frame timing, memory usage
- **Enemy State Display**: AI behavior and position debugging
- **Audio Volume Controls**: Live adjustment of all audio levels
- **Collision Visualization**: Debug rendering of hit boxes and ray casts

### **Extensibility**
- **Modular Design**: Easy to add new enemy types, weapons, or levels
- **Configurable Systems**: Adjustable game parameters without recompilation
- **Asset Pipeline**: Automated texture conversion and loading
- **Cross-Platform**: Builds and runs on multiple operating systems

## 🏆 **Technical Achievements**

### **Graphics Programming**
- ✅ **Custom Raycasting Engine**: Built from scratch for optimal performance
- ✅ **Texture Mapping**: Perspective-correct wall texturing with RGBA support
- ✅ **Sprite Rendering**: 3D positioned sprites with distance-based scaling
- ✅ **Animation System**: Multi-frame character animations with state management

### **Game Systems**
- ✅ **Advanced AI**: Multiple behavior patterns with pathfinding
- ✅ **Physics Integration**: Collision detection and response systems
- ✅ **Audio Engine**: 3D spatial audio with dynamic mixing
- ✅ **Input Abstraction**: Unified input handling for multiple device types

### **Performance Optimization**
- ✅ **Efficient Algorithms**: Optimized raycasting with early exit conditions
- ✅ **Memory Management**: Smart caching and resource pooling
- ✅ **Scalable Rendering**: Adjustable quality settings for different hardware
- ✅ **Frame-Rate Independence**: Consistent gameplay across varying performance

## 🧩 **Code Highlights**

### **Raycasting Algorithm**
The core rendering uses a sophisticated raycasting implementation with:
- DDA (Digital Differential Analyzer) for efficient grid traversal
- Texture coordinate calculation for realistic wall rendering
- Distance-based fog effects for atmospheric depth

### **Enemy AI System**
```rust
// Example: Chase enemy behavior
if distance_to_player < chase_range {
    let direction = (player_pos - enemy_pos).normalize();
    enemy.move_towards(direction * chase_speed * delta_time);
    enemy.set_animation(AnimationState::Attack);
}
```

### **Audio Management**
```rust
// Dynamic audio mixing based on game state
audio_manager.play_footstep_sound(player_velocity);
audio_manager.set_music_volume(user_preference);
audio_manager.trigger_combat_sound(attack_type);
```

## 🎯 **Future Development**

### **Planned Features**
- [ ] **Multiplayer Support**: Network-based cooperative gameplay
- [ ] **Level Editor**: In-game maze creation tools
- [ ] **Additional Weapons**: Ranged combat options
- [ ] **Power-ups**: Temporary ability enhancements
- [ ] **Save System**: Progress persistence across sessions

### **Technical Improvements**
- [ ] **Enhanced Graphics**: Lighting effects and shadows
- [ ] **Advanced AI**: Machine learning-based enemy behaviors  
- [ ] **Mobile Support**: Touch-based controls for mobile devices
- [ ] **VR Integration**: Virtual reality compatibility

## 📄 **License**

This project is developed as an educational demonstration of advanced game programming concepts using Rust and modern graphics programming techniques.

## 🤝 **Contributing**

This is a educational project showcasing advanced raycasting and game development techniques. The codebase demonstrates best practices in:
- Rust game development
- Real-time graphics programming
- Audio system design
- Input handling abstraction
- Performance optimization

---

*Built with ❤️ using Rust and Raylib - Showcasing the power of systems programming for game development*
