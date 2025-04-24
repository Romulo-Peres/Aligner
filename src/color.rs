use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Color {
   pub r: f32,
   pub g: f32,
   pub b: f32,
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
   a * (1.0 - t) + b * t
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
   Color {
      r: lerp(a.r, b.r, t),
      g: lerp(a.g, b.g, t),
      b: lerp(a.b, b.b, t),
   }
}

pub fn scale_color(value: u8) -> i16 {
   ((value as f32 * 1000.0 / 255.0).round()) as i16
}

pub fn interpolate_multi_color(
   x: f32,
   y: f32,
   width: f32,
   height: f32,
   angle_deg: f32,
   colors: &[Color],
) -> Color {
   if colors.len() < 2 {
      return colors[0];
   }

   let cx = width / 2.0;
   let cy = height / 2.0;
   let dx = x - cx;
   let dy = y - cy;

   let angle_rad = angle_deg * PI / 180.0;
   let dir_x = angle_rad.cos();
   let dir_y = angle_rad.sin();

   let denom = (width / 2.0) * dir_x.abs() + (height / 2.0) * dir_y.abs();
   let denom = if denom == 0.0 { 1.0 } else { denom };

   let mut t = (dx * dir_x + dy * dir_y) / denom;
   t = t * 0.5 + 0.5;
   t = t.clamp(0.0, 0.999999);

   let scaled_t = t * (colors.len() - 1) as f32;
   let index = scaled_t.floor() as usize;
   let local_t = scaled_t - index as f32;

   lerp_color(colors[index], colors[index + 1], local_t)
}
