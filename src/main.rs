use std::{process::exit, thread::sleep, time::Duration};
use arguments::ProgramArguments;
use clap::Parser;
use crossterm::event::{self, KeyCode};
use display::{print_message, reset_display_colors};
use input::{handle_input, read_message_file};
use parser::parse_message;
use state::generate_program_state;
use terminal::{alternate_screen, change_cursor_visibility, clear_terminal, get_terminal_size, set_stdin_raw_mode, AlternateScreenAction, CursorVisibilityAction, SetStdinRawModeAction, TerminalSize};

mod terminal;
mod arguments;
mod color;
mod input;
mod display;
mod parser;
mod state;

fn main() {
   let mut keep_rendering: bool = true;
   let mut dimensions: TerminalSize;
   let arguments = ProgramArguments::parse();
   let read_result = read_message_file(&arguments.message_file);

   let message = read_result.unwrap_or_else(| error |  {
      println!("Error while trying to read the message file: {}", error);
      exit(1);
   });

   let parsed_message = parse_message(message);

   let mut state = generate_program_state(&arguments).unwrap_or_else(| error | {
      println!("{}", error);
      exit(1);
   });

   dimensions = get_terminal_size().unwrap_or_else(| _ | {
      println!("Couldn't get the terminal dimensions. Exiting.");
      exit(1);
   });

   if arguments.disable_iterative == false {
      enter_iterative_mode();

      while keep_rendering {
         print_message(&parsed_message, &state, &dimensions, false, false);

         let read_result = event::read();

         if let Ok(some_event) = read_result {

            if let event::Event::Key(key_event) = some_event {

               if let KeyCode::Char(c) = key_event.code {
                  handle_input(c, &mut state, &mut keep_rendering, parsed_message.max_line_size, parsed_message.lines.len());
               } else {
                  keep_rendering = false
               }

            }

            if let event::Event::Resize(columns, rows) = some_event{
               dimensions.update(columns, rows);
               
               clear_terminal().unwrap_or_else(| _ | {
                  leave_iterative_mode();
                  println!("Couldn't clear the screen after a terminal resize");

                  exit(1);
               });
            }

         }

         sleep(Duration::from_millis(20));
      }

      leave_iterative_mode();
   }

   if arguments.disable_stdout == false {
      print_message(&parsed_message, &state, &dimensions, false, true);
   }

   reset_display_colors().unwrap_or_else(| _ | {
      println!("Couldn't reset the terminal colors");
   });

}

fn enter_iterative_mode() {
   alternate_screen(AlternateScreenAction::Enter).unwrap_or_else(| _ | {
      println!("Couldn't enter in alternate screen. Exiting.");

      exit(1);
   });

   set_stdin_raw_mode(SetStdinRawModeAction::Enable).unwrap_or_else(| _ | {
      alternate_screen(AlternateScreenAction::Leave).unwrap_or(());
      println!("Couldn't set the stdin raw mode. Exiting.");
      
      exit(1);
   });

   change_cursor_visibility(CursorVisibilityAction::Hide).unwrap_or(());
}

fn leave_iterative_mode() {
   alternate_screen(AlternateScreenAction::Leave).unwrap_or_else(| _ | {
      println!("Couldn't leave the alternate screen mode.");
   });

   set_stdin_raw_mode(SetStdinRawModeAction::Disable).unwrap_or_else(| _ | {
      print!("Couldn't disable the stdin raw mode.");
   });

   change_cursor_visibility(CursorVisibilityAction::Show).unwrap_or_else(| _ | {
      println!("Couldn't change the cursor visibility to visibile.");
   });
}