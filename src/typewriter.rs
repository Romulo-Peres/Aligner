use std::process::exit;

use clap::builder::Str;
use unicode_segmentation::UnicodeSegmentation;

use crate::parser::ParsedMessage;

const FONT_HEIGHT: u8 = 6;

#[derive(Default)]
struct ExtractedPunctuation {
   exclamation: Vec<Vec<String>>,
   hashtag: Vec<Vec<String>>,
   question: Vec<Vec<String>>,
   open_bracket: Vec<Vec<String>>,
   close_bracket: Vec<Vec<String>>,
   comma: Vec<Vec<String>>,
   period: Vec<Vec<String>>,
   open_parenthesis: Vec<Vec<String>>,
   close_parenthesis: Vec<Vec<String>>,
   minus: Vec<Vec<String>>,
   underline: Vec<Vec<String>>,
   slash: Vec<Vec<String>>
}

struct ExtractedGraphemes {
   characters: Vec<Vec<Vec<String>>>,
   numbers: Vec<Vec<Vec<String>>>,
   punctuation: ExtractedPunctuation
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

         if let Some(grapheme_lines) = get_grapheme_lines(c, &graphemes) {
            line.extend(grapheme_lines[i as usize].clone());
         }
      }

      if line.len() > max_line_size {
         max_line_size = line.len();
      }

      message.push(line);
   }

   ParsedMessage { max_line_size, lines: message }
}

fn get_grapheme_lines(c: char, graphemes: &ExtractedGraphemes) -> Option<Vec<Vec<String>>> {
   if c.is_numeric() {
      let index = c as usize - 48;

      Some(graphemes.numbers[index].clone())
   } else if c.is_alphabetic() {
      let index = c as usize - 65;

      Some(graphemes.characters[index].clone())
   } else if c.is_whitespace() {
      Some(generate_whitespace_graphemes())
   } else if c.is_ascii_punctuation() {
      get_punctuation_graphemes(c, graphemes)
   } else {
      None
   }
}

fn get_punctuation_graphemes(c: char, graphemes: &ExtractedGraphemes) -> Option<Vec<Vec<String>>> {
   let punctuation = match c {
      '!' => graphemes.punctuation.exclamation.clone(),
      '#' => graphemes.punctuation.hashtag.clone(),
      '?' => graphemes.punctuation.question.clone(),
      '[' => graphemes.punctuation.open_bracket.clone(),
      ']' => graphemes.punctuation.close_bracket.clone(),
      ',' => graphemes.punctuation.comma.clone(),
      '.' => graphemes.punctuation.period.clone(),
      '(' => graphemes.punctuation.open_parenthesis.clone(),
      ')' => graphemes.punctuation.close_parenthesis.clone(),
      '-' => graphemes.punctuation.minus.clone(),
      '_' => graphemes.punctuation.underline.clone(),
      '/' => graphemes.punctuation.slash.clone(),
      _ => return None,
  };

  Some(punctuation)
}

fn generate_whitespace_graphemes() -> Vec<Vec<String>> {
   let mut whitespace: Vec<Vec<String>> = Vec::new();
   let whitespace_graphemes: Vec<String> = "    ".graphemes(true).map(|x| x.to_string()).collect();

   for i in 0..FONT_HEIGHT {
      whitespace.push(whitespace_graphemes.clone());
   }

   whitespace
}

fn extract_font_graphemes(font: &str) -> ExtractedGraphemes {
   let font_lines: Vec<&str> = font.split("\n").collect();
   let mut characters: Vec<Vec<Vec<String>>> = Vec::new();
   let mut numbers: Vec<Vec<Vec<String>>> = Vec::new();
   let mut punctuation: ExtractedPunctuation = ExtractedPunctuation::default();

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

   for i in 0..=11 {
      let lines = font_lines[i*6+36*6..i*6+36*6+6].to_vec();

      match i {
         0 => extract_punctuation(lines, &mut punctuation.exclamation),
         1 => extract_punctuation(lines, &mut punctuation.hashtag),
         2 => extract_punctuation(lines, &mut punctuation.question),
         3 => extract_punctuation(lines, &mut punctuation.open_bracket),
         4 => extract_punctuation(lines, &mut punctuation.close_bracket),
         5 => extract_punctuation(lines, &mut punctuation.comma),
         6 => extract_punctuation(lines, &mut punctuation.period),
         7 => extract_punctuation(lines, &mut punctuation.open_parenthesis),
         8 => extract_punctuation(lines, &mut punctuation.close_parenthesis),
         9 => extract_punctuation(lines, &mut punctuation.minus),
         10 => extract_punctuation(lines, &mut punctuation.underline),
         11 => extract_punctuation(lines, &mut punctuation.slash),
         _ => {}
      }
   }

   ExtractedGraphemes { characters, numbers, punctuation }
}

fn extract_punctuation(font_slice: Vec<&str>, target: &mut Vec<Vec<String>>) {
   let mut lines: Vec<Vec<String>> = Vec::new();

   for line in font_slice {
      lines.push(line.graphemes(true).map(|x| x.to_string()).collect());
   }

   *target = lines;
}
