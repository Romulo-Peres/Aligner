use clap::Parser;

#[derive(Parser, Debug)]
pub struct ProgramArguments {
   #[arg(short = 'f', long = "file")]
   pub message_file: Option<String>,

   #[arg(short = 'e', long = "text")]
   pub display_text: Option<String>,

   #[arg(short = 'v', long = "vertically")]
   pub align_vertically: bool,

   #[arg(short = 'o', long = "horizontally")]
   pub align_horizontally: bool,

   #[arg(short = 'l', long = "line-line", value_name = "milliseconds")]
   pub line_by_line_mode: Option<u32>,

   #[arg(short = 'c', long = "char-char", value_name = "milliseconds")]
   pub char_by_char_mode: Option<u32>,

   #[arg(short = 's', long = "colors", value_name = "hexadecimal colors", num_args=1..)]
   pub colors: Option<Vec<String>>,

   #[arg(short = 'a', long = "angle", value_name = "0-360")]
   pub angle: Option<u16>,

   #[arg(short = 'i', long = "disable-iterative")]
   pub disable_iterative: bool,

   #[arg(short = 't', long = "disable-stdout")]
   pub disable_stdout: bool,

   #[arg(short = 'n', long = "control-server", value_name = "address:port")]
   pub control_server: Option<String>
}