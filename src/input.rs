use std::{f32::consts::PI, io::Error, ops::{AddAssign, SubAssign}, process::exit};

use crate::{leave_iterative_mode, parser::{parse_message, parse_program_colors, ControlMessageAction, ParsedControlMessage, ParsedMessage}, state::ProgramState, terminal::clear_terminal};

pub 
fn handle_input(input: char, state: &mut ProgramState, keep_rendering: &mut bool, width: usize, height: usize) -> bool {
   let redraw;

   if input == ',' || input == '.' {
      let step = get_adjusted_angle_step(state.angle as f32, 5 as f32, width as f32, height as f32);

      if input == '.' {
         state.angle.add_assign(step);
      }

      if input == ',' {
         state.angle.sub_assign(step);
      }

      if state.angle < 0.0 {
         state.angle = 360.0 + state.angle;
      } else if state.angle > 360.0 {
         state.angle = state.angle % 360.0;
      }

      redraw = true;      
   } else {
      redraw = false;
      *keep_rendering = false;
   }

   return redraw;
}

pub fn read_message_file(filename: &str) -> Result<String, Error> {
   let read_result = std::fs::read_to_string(filename);

   return read_result;
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

pub fn handle_network_input(input: ParsedControlMessage, state: &mut ProgramState, message: &mut ParsedMessage) -> Result<(), String> {
   match input.action {
      ControlMessageAction::SetAngle => {
         handle_network_set_angle(input.value, state);
      },
      ControlMessageAction::SetColor => {
         handle_network_set_color(input.value, state)?;
      },
      ControlMessageAction::SetMessage => {
         handle_network_set_message(input.value, message);
      },
      ControlMessageAction::RotateLeft => {
         handle_network_rotation(state, true, message);
      },
      ControlMessageAction::RotateRight => {
         handle_network_rotation(state, false, message);
      }
   }

   Ok(())
}


fn handle_network_rotation(state: &mut ProgramState, rotate_left: bool, message: &mut ParsedMessage) {
   let mut keep_rendering_mock = false;

   if rotate_left == true {
      handle_input(',', state, &mut keep_rendering_mock, message.max_line_size, message.lines.len());
   } else {
      handle_input('.', state, &mut keep_rendering_mock, message.max_line_size, message.lines.len());
   }
}

fn handle_network_set_angle(angle: String, state: &mut ProgramState) {
   state.angle = angle.parse::<f32>().unwrap();
}

fn handle_network_set_color(colors: String, state: &mut ProgramState) -> Result<(), String>{
   let colors_vector: Vec<String> = colors.split(" ")
                                                   .collect::<Vec<&str>>().iter()
                                                   .map(| str | str.to_string())
                                                   .collect();
   
   match parse_program_colors(&colors_vector) {
      Ok(colors) => {
         state.colors = colors;

         Ok(())
      },
      Err(error) => {
         Err(error)
      }
   }
}

fn handle_network_set_message(incoming_message: String, program_message: &mut ParsedMessage) {
   clear_terminal().unwrap_or_else(| _ | {
      leave_iterative_mode();
      println!("Failed to clear the screen after the message was updated.");

      exit(1);
   });

   *program_message = parse_message(incoming_message);
}