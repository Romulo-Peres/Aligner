use std::{io::{stdout, Error}, ops::AddAssign, thread::sleep, time::Duration};

use crossterm::{style::{ Color, ResetColor, SetForegroundColor }, ExecutableCommand, QueueableCommand};

use crate::{ color::interpolate_multi_color, parser::ParsedMessage, state::{ProgramMode, ProgramState}, terminal::{move_cursor_right, move_cursor_to, TerminalSize}};

pub fn print_message(message: &ParsedMessage, state: &ProgramState, dimensions: TerminalSize, animate_draw: bool, stdout_print: bool) {
   if stdout_print == false {
      move_cursor_to(0, 0).expect("Failed to set the cursor position. Exiting.");
   }

   if state.align_vertically == true {
      add_top_padding(&dimensions, message.lines.len(), stdout_print);
   }

   for line in message.lines.iter().enumerate() {
      if state.align_horizontally == true {
         add_left_padding(&dimensions, line.1.len());
      }

      let color_index = if state.align_horizontally {
         (message.max_line_size - line.1.len()) / 2
      } else {
         0 
      };

      print_line(line.1, color_index, line.0, message.max_line_size, message.lines.len(), state, animate_draw);
   }

   if state.align_vertically == true && stdout_print == true {
      add_bottom_padding(&dimensions, message.lines.len());
   }
}

fn print_line(line: &Vec<String>, mut color_index: usize, line_number: usize, max_size: usize, lines_count: usize, state: &ProgramState, animate_draw: bool) {
   for grapheme in line {
      let color = interpolate_multi_color(color_index as f32, line_number as f32, max_size as f32, lines_count as f32, state.angle.into(), &state.colors);

      let mut stdout = stdout();

      stdout.queue(SetForegroundColor(Color::Rgb { r: color.r as u8, g: color.g as u8, b: color.b as u8 })).expect("Failed to set the foreground color. Exiting.");

      print!("{}", grapheme);

      if animate_draw == true && state.mode == ProgramMode::CharByChar {
         sleep(Duration::from_millis(state.animation_delay));
      }

      color_index.add_assign(1);
   }

   println!("\r");
}

fn add_top_padding(dimensions: &TerminalSize, message_lines: usize, stdout_print: bool) {
   let y_print_coordinate = ((dimensions.height / 2) - (message_lines as u16 / 2)) + 2;

   if stdout_print {
      for _ in 0..y_print_coordinate {
         println!("");
      }
   } else {
      move_cursor_to(0, y_print_coordinate).expect("Failed to set the cursor position. Exiting.")
   }
}

fn add_bottom_padding(dimensions: &TerminalSize, line_count: usize) {
   let y_print_coordinate = (dimensions.height as usize / 2) - (line_count / 2);
   let end_stuffing  = dimensions.height as usize - y_print_coordinate - line_count;

   for _ in 0..end_stuffing {
      println!("");
   }
}

fn add_left_padding(dimensions: &TerminalSize, line_length: usize) {
   let message_x_offset = dimensions.width / 2 - line_length as u16 / 2;

   move_cursor_right(message_x_offset).expect("Failed to set the cursor position. Exiting.");
}

pub fn reset_display_colors() -> Result<(), Error>{
   let mut stdout = stdout();

   match stdout.execute(ResetColor) {
      Ok(_) => Ok(()),
      Err(error) => Err(error)
   }
}