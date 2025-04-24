use crate::color::Color;

#[derive(PartialEq)]
pub enum ProgramMode {
   CharByChar,
   LineByLine
}

pub struct ProgramState {
   pub angle: i16,
   pub colors: Vec<Color>,
   pub mode: ProgramMode,
   pub animation_delay: u64,
   pub align_vertically: bool,
   pub align_horizontally: bool
}

impl ProgramState {
   pub fn new(angle: i16, colors: Vec<Color>, mode: ProgramMode, animation_delay: u64, align_vertically: bool, align_horizontally: bool) -> ProgramState {
      return ProgramState { angle, colors, mode, animation_delay, align_horizontally, align_vertically };
   }
}