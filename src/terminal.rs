use std::io::{stdout, Error};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::ClearType::All;
use crossterm::{cursor::MoveRight, terminal::Clear, ExecutableCommand};

pub enum AlternateScreenAction {
   Enter,
   Leave
}

pub enum SetStdinRawModeAction {
   Enable,
   Disable
}

pub enum CursorVisibilityAction {
   Hide,
   Show
}

pub struct TerminalSize {
   pub width: u16,
   pub height: u16
}

impl TerminalSize {
   pub fn update(&mut self, width: u16, height: u16) {
      self.width = width;
      self.height = height;
   }
}

pub fn move_cursor_right(columns: u16) -> Result<(), Error>{
   let mut stdout = stdout();

   match stdout.execute(MoveRight(columns)) {
      Ok(_) => Ok(()),
      Err(error) => Err(error)
   }
}

pub fn clear_terminal() -> Result<(), Error>{
   let mut stdout = stdout();

   match stdout.execute(Clear(All)) {
      Ok(_) => Ok(()),
      Err(error) => Err(error)
   }
}

pub fn change_cursor_visibility(action: CursorVisibilityAction) -> Result<(), Error> {
   let mut stdout = stdout();

   let execute_result;

   match action {
      CursorVisibilityAction::Hide => {
         execute_result = stdout.execute(Hide);
      },
      CursorVisibilityAction::Show => {
         execute_result = stdout.execute(Show);
      }
   }

   match execute_result {
      Ok(_) => Ok(()), 
      Err(error) => Err(error)
   }
}

pub fn get_terminal_size() -> Result<TerminalSize, Error> {
   let terminal_size = size();

   match terminal_size {
      Ok(size) => {
         Ok(TerminalSize {
            width: size.0,
            height: size.1
         })
      },
      Err(error) => {
         Err(error)
      }
   }
}

pub fn alternate_screen(action: AlternateScreenAction) -> Result<(), Error> {
   let mut stdout = stdout();

   let execute_result;

   match action {
      AlternateScreenAction::Enter => {
         execute_result = stdout.execute(EnterAlternateScreen);
      },
      AlternateScreenAction::Leave => {
         execute_result = stdout.execute(LeaveAlternateScreen);
      }
   }

   match execute_result {
      Ok(_) => Ok(()),
      Err(error) => Err(error)
   }
}

pub fn set_stdin_raw_mode(action: SetStdinRawModeAction) -> Result<(), Error> {
   match action {
      SetStdinRawModeAction::Enable => {
         enable_raw_mode()
      },
      SetStdinRawModeAction::Disable => {
        disable_raw_mode()
      }
   }
}

pub fn move_cursor_to(column: u16, row: u16) -> Result<(), Error> {
   let mut stdout = stdout();

   match stdout.execute(MoveTo(column, row)) {
      Ok(_) => Ok(()),
      Err(error) => Err(error)
   }
}

