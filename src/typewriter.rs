use unicode_segmentation::UnicodeSegmentation;

use crate::parser::ParsedMessage;

const FONT_HEIGHT: u8 = 6;

struct ExtractedGraphemes {
   characters: Vec<Vec<Vec<String>>>,
   numbers: Vec<Vec<Vec<String>>>
}

pub fn generate_message(text: String) -> ParsedMessage {
   let builtin_font: &str = include_str!("./../builtin-font.txt");

   let graphemes = extract_font_graphemes(builtin_font);

   create_display_message(text, graphemes)
}

fn create_display_message(mut text: String, graphemes: ExtractedGraphemes) -> ParsedMessage {
   let mut message: Vec<Vec<String>> = Vec::new();
   let mut max_line_size = 0;

   text = text.to_uppercase();

   let line_characters: Vec<char> = text.chars().collect();

   for i in 0..FONT_HEIGHT {
      let mut line: Vec<String> = Vec::new();

      for j in 0..text.len() {
         let c = line_characters[j];

         let grapheme_lines = get_grapheme_lines(c, &graphemes);

         line.extend(grapheme_lines[i as usize].clone());
      }

      if line.len() > max_line_size {
         max_line_size = line.len();
      }

      message.push(line);
   }

   ParsedMessage { max_line_size, lines: message }
}

fn get_grapheme_lines(c: char, graphemes: &ExtractedGraphemes) -> Vec<Vec<String>> {
   if c.is_numeric() {
      let index = c as usize - 48;

      graphemes.numbers[index].clone()
   } else {
      let index = c as usize - 65;

      graphemes.characters[index].clone()
   }
}

fn extract_font_graphemes(font: &str) -> ExtractedGraphemes {
   let font_lines: Vec<&str> = font.split("\n").collect();
   let mut characters: Vec<Vec<Vec<String>>> = Vec::new();
   let mut numbers: Vec<Vec<Vec<String>>> = Vec::new();

   for i in 0..26 {
      let mut character_lines: Vec<Vec<String>> = Vec::new();

      for j in 0..FONT_HEIGHT {
         character_lines.push(font_lines[(i * FONT_HEIGHT + j) as usize].graphemes(true).map(|x| x.to_string()).collect());
      }

      characters.push(character_lines);
   }

   for i in 26..36 {
      let mut number_lines: Vec<Vec<String>> = Vec::new();

      for j in 0..FONT_HEIGHT {
         number_lines.push(font_lines[(i * FONT_HEIGHT + j) as usize].graphemes(true).map(|x| x.to_string()).collect());
      }

      numbers.push(number_lines);
   }

   ExtractedGraphemes { characters, numbers }
}
