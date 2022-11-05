use std::env;
use colored::Colorize;
use clipboard_anywhere::set_clipboard;
use inflector::Inflector;

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let output = format!(
        "{}\n{}{}\n{}",
        "    /// -----------------------------------------------------------------------",
        "    /// ", input.to_title_case(),
        "    /// -----------------------------------------------------------------------"
    );

    println!("{}", format!("Header saved to clipboard...").yellow());

    set_clipboard(&output.to_owned());
}