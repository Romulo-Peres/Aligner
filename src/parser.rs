use std::process::exit;

use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

use crate::color::Color;

pub enum ControlMessageAction {
   RotateLeft,
   RotateRight,
   SetMessage,
   SetColor,
   SetAngle,
   SetText
}

pub struct ParsedMessage {
   pub max_line_size: usize,
   pub lines: Vec<Vec<String>>
}

pub struct ParsedControlMessage {
   pub action: ControlMessageAction,
   pub value: String
}

#[derive(Debug, Deserialize)]
struct ControlMessage {
   action: String,
   message: Option<String>,
   colors: Option<String>,
   angle: Option<u16>,

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

pub fn parse_program_colors(colors: &Vec<String>) -> Result<Vec<Color>, String> {
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


pub fn parse_client_control_message(message: String) -> Result<ParsedControlMessage, String> {
   if let Ok(parsed_message) = serde_json::from_str::<ControlMessage>(&message) {
      match parsed_message.action.as_str() {
         "SET_MESSAGE" => {
            if let Some(message) = parsed_message.message {

               let return_value = ParsedControlMessage {
                  action: ControlMessageAction::SetMessage,
                  value: message
               };

               return Ok(return_value);
            }
         },
         "SET_COLORS" => {
            if let Some(colors) = parsed_message.colors {

               let return_value = ParsedControlMessage {
                  action: ControlMessageAction::SetColor,
                  value: colors
               };

               return Ok(return_value);
            }
         },
         "SET_ANGLE" => {
            if let Some(angle) = parsed_message.angle {

               let return_value = ParsedControlMessage {
                  action: ControlMessageAction::SetAngle,
                  value: angle.to_string()
               };

               return Ok(return_value);
            }
         },
         "ROTATE_LEFT" => {
            let return_value = ParsedControlMessage {
               action: ControlMessageAction::RotateLeft,
               value: "".to_string()
            };

            return Ok(return_value);
         },
         "ROTATE_RIGHT" => {
            let return_value = ParsedControlMessage {
               action: ControlMessageAction::RotateRight,
               value: "".to_string()
            };

            return Ok(return_value);
         },
         "SET_TEXT" => {
            if let Some(message) = parsed_message.message {

               let return_value = ParsedControlMessage {
                  action: ControlMessageAction::SetText,
                  value: message
               };

               return Ok(return_value);
            }

         }
         _ => {
            return Err("The sent message is not a valid JSON for this application.".to_string())
         }
      }

      return Err(format!("You must provide a valid value to perform the action \'{}\'.", parsed_message.action).to_string());
   }

   Err("The sent message is not a valid JSON for this application.".to_string())
}