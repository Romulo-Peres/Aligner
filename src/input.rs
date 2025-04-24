use std::{f32::consts::PI, io::Error, ops::{AddAssign, SubAssign}};

use unicode_segmentation::UnicodeSegmentation;

pub fn handle_input(input: char, angle: &mut i16, keep_rendering: &mut bool, width: u16, height: u16) -> bool {
   let redraw;

   if input == ',' || input == '.' {
      let step = get_adjusted_angle_step(*angle as f32, 5 as f32, width as f32, height as f32);

      if input == '.' {
         angle.add_assign(step as i16);
      }

      if input == ',' {
         angle.sub_assign(step as i16);
      }

      if *angle < 0 {
         *angle = 360 + *angle;
      } else if *angle > 360 {
         *angle = *angle % 360;
      }

      redraw = true;      
   } else {
      redraw = false;
      *keep_rendering = false;
   }

   return redraw;
}

pub fn read_message_file(filename: &str) -> Result<Vec<Vec<String>>, Error> {
   let read_result = std::fs::read_to_string(filename);
   let mut file_lines = Vec::new();

   match read_result {
      Ok(content) => {
         for line in content.split("\n") {
            let graphemes: Vec<&str> = line.graphemes(true).collect();

            file_lines.push(graphemes);
         }

         Ok(file_lines.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect())
      }, 
      Err(error) => {
         Err(error)
      }
   }
}

pub fn get_adjusted_angle_step(angle_deg: f32, base_step_deg: f32, width: f32, height: f32) -> f32 {
   let angle_rad = angle_deg * PI / 180.0;
   let mut dir_x = angle_rad.cos();
   let mut dir_y = angle_rad.sin();

   let len = (dir_x * dir_x + dir_y * dir_y).sqrt();

   if len == 0.0 {
       return base_step_deg;
   }

   dir_x /= len;
   dir_y /= len;

   let max_x = dir_x.abs() * (width / 2.0);
   let max_y = dir_y.abs() * (height / 2.0);
   let projection_max = (max_x * max_x + max_y * max_y).sqrt();

   let max_half = width.max(height) / 2.0;
   let weight = projection_max / max_half;

   base_step_deg * weight
}