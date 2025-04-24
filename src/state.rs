use std::process::exit;

use crate::{arguments::ProgramArguments, color::Color, parser::parse_program_colors};

#[derive(PartialEq)]
pub enum ProgramMode {
   CharByChar,
   LineByLine,
   Flash
}

pub struct ProgramState {
   pub angle: f32,
   pub colors: Vec<Color>,
   pub mode: ProgramMode,
   pub animation_delay: u64,
   pub align_vertically: bool,
   pub align_horizontally: bool
}

impl ProgramState {
   pub fn new(angle: i16, colors: Vec<Color>, mode: ProgramMode, animation_delay: u64, align_vertically: bool, align_horizontally: bool) -> ProgramState {
      return ProgramState { angle: angle as f32, colors, mode, animation_delay, align_horizontally, align_vertically };
   }
}

pub fn generate_program_state(arguments: &ProgramArguments) -> Result<ProgramState, String> {
   let colors = match &arguments.colors {
      Some(colors) => {
         let parsed_colors = parse_program_colors(colors);

         let parsed_colors = parsed_colors.unwrap_or_else(| error | {
            println!("Error while trying to parse the colors argument: {}", error);
            exit(1);
         });

         parsed_colors
      },
      None => {
         vec![Color { r: 255.0, g: 255.0, b: 255.0 }]
      }
   };

   let mut mode = ProgramMode::Flash;
   let mut delay = 0;

   if let Some(line_by_line_delay) = arguments.line_by_line_mode {
      mode =  ProgramMode::LineByLine;
      delay = line_by_line_delay;
   }

   if let Some(char_by_char_delay) = arguments.char_by_char_mode {
      mode = ProgramMode::CharByChar;
      delay = char_by_char_delay;
   }

   let mut gradient_angle = 0;

   if let Some(angle) = arguments.angle {
      gradient_angle = angle;
   }

   let state: ProgramState = ProgramState::new(gradient_angle as i16, colors, mode, delay.into(), arguments.align_vertically, arguments.align_horizontally);

   Ok(state)
}