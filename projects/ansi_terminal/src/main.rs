use ansi_term::{Colour,Style};

fn main() {
    println!("this is {} in color, {} in color, {} is bold, and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Style::new().bold().paint("this is bold"),
        Colour::Green.bold().paint("green")
        );
}
