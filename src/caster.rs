// caster.rs

use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
  pub distance: f32,
  pub impact: char,
  pub tx: usize,
}

pub fn cast_ray(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  player: &Player,
  a: f32,
  block_size: usize,
  draw_line: bool,
) -> Intersect {
  let mut d = 0.0;

  framebuffer.set_current_color(Color::WHITESMOKE);

  loop {
    let cos = d * a.cos();
    let sin = d * a.sin();
    let ray_x = player.pos.x + cos;
    let ray_y = player.pos.y + sin;

    // Check for negative coordinates before casting to usize
    if ray_x < 0.0 || ray_y < 0.0 {
      return Intersect{
        distance: d,
        impact: '+', // Return wall character for out of bounds
        tx: 0
      };
    }

    let x = ray_x as usize;
    let y = ray_y as usize;

    let i = x / block_size;
    let j = y / block_size;

    // Add bounds checking to prevent crash
    if j >= maze.len() || i >= maze[0].len() {
      return Intersect{
        distance: d,
        impact: '+', // Return wall character for out of bounds
        tx: 0
      };
    }

    if maze[j][i] != ' ' && maze[j][i] != 'p' {
      let hitx = x - i*block_size;
      let hity = y - j*block_size;
      let mut maxhit = hity;

      if 1 < hitx && hitx < block_size - 1 {
        maxhit = hitx
      } 

      // Fix texture coordinate calculation with proper floating point math
      let tx = ((maxhit as f32 * 127.0) / block_size as f32) as usize;

      return Intersect{
        distance: d,
        impact: maze[j][i],
        tx: tx
      };
    }

    if draw_line {
      framebuffer.set_pixel(x as u32, y as u32);
    }

    d += 1.0;
  }
}



