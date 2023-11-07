use clap::{Arg, Command};
use diffenator3::ttj::font_to_json;
use read_fonts::FontRef;

fn main() {
    let matches = Command::new("ttj")
        .about("dump a font file to json")
        .arg_required_else_help(true)
        .arg(Arg::new("font").help("Font file to dump"))
        .get_matches();

    let name = matches.get_one::<String>("font").expect("No font name?");
    let font_binary = std::fs::read(name).expect("Couldn't open file");
    let font = FontRef::new(&font_binary).expect("Can't parse");
    let json = font_to_json(&font);
    println!("{:}", serde_json::to_string_pretty(&json).unwrap());
}
