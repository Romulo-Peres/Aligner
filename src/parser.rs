use unicode_segmentation::UnicodeSegmentation;

use crate::color::Color;

pub struct ParsedMessage {
   pub max_line_size: usize,
   pub lines: Vec<Vec<String>>
}

pub fn parse_message(message: String) -> ParsedMessage {
   let mut file_lines = Vec::new();
   let mut max_line_size = 0;

   for line in message.split("\n") {
      let graphemes: Vec<&str> = line.graphemes(true).collect();

      if graphemes.len() > max_line_size {
         max_line_size = graphemes.len();
      }

      file_lines.push(graphemes);
   }

   ParsedMessage {
      lines: file_lines.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect(),
      max_line_size: max_line_size
   }
}

pub fn parse_program_colors(colors: Vec<String>) -> Result<Vec<Color>, String> {
   let mut parsed_colors = Vec::new();

   for color in colors {
      let parse_result = parse_hex_color(&color);

      match parse_result {
         Some(color) => {
            parsed_colors.push(color);
         },
         None => {
            let error = format!("\"{}\" is not a valid hexadecimal color.", color);

            return Err(error);
         }
      }
   }

   Ok(parsed_colors)
}

fn parse_hex_color(s: &str) -> Option<Color> {
   Some(Color {
      r: u8::from_str_radix(&s[1..3], 16).ok()? as f32,
      g: u8::from_str_radix(&s[3..5], 16).ok()? as f32,
      b: u8::from_str_radix(&s[5..7], 16).ok()? as f32
   })
}
