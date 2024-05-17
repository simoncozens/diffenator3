use clap::{builder::ArgAction, Parser};
use colored::Colorize;
use diffenator3::{
    dfont::DFont,
    html::{render_output, CSSFontFace, CSSFontStyle},
    render::{test_font_glyphs, test_font_words},
    ttj::{jsondiff::Substantial, table_diff},
};
use serde_json::Map;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// If an entry is absent in one font, show the data anyway
    #[clap(long = "no-succinct", action = ArgAction::SetFalse)]
    succinct: bool,

    /// If an entry is absent in one font, just report it as absent
    #[clap(long = "succinct", overrides_with = "succinct")]
    _no_succinct: bool,

    /// Don't show diffs in font-tables
    #[clap(long = "no-tables", action = ArgAction::SetFalse)]
    tables: bool,

    /// Show diffs in font tables [default]
    #[clap(long = "tables", overrides_with = "tables")]
    _no_tables: bool,

    /// Don't show diffs in glyph images
    #[clap(long = "no-glyphs", action = ArgAction::SetFalse)]
    glyphs: bool,

    /// Show diffs in glyph images [default]
    #[clap(long = "glyphs", overrides_with = "glyphs")]
    _no_glyphs: bool,

    /// Don't show diffs in word images
    #[clap(long = "no-words", action = ArgAction::SetFalse)]
    words: bool,

    /// Show diffs in word images [default]
    #[clap(long = "words", overrides_with = "words")]
    _no_words: bool,

    /// Show diffs as JSON
    #[clap(long = "json")]
    json: bool,
    /// Show diffs as HTML
    #[clap(long = "html")]
    html: bool,

    /// Output directory for HTML
    #[clap(long = "output", default_value = "out", requires = "html")]
    output: String,

    /// Location in design space, in the form axis=123,other=456
    #[clap(long = "location")]
    location: Option<String>,
    #[clap(long = "instance", conflicts_with = "location")]
    instance: Option<String>,

    /// The first font file to compare
    font1: PathBuf,
    /// The second font file to compare
    font2: PathBuf,
}

fn die(doing: &str, err: impl Error) -> ! {
    eprintln!("Error {}: {}", doing, err);
    eprintln!();
    eprintln!("Caused by:");
    if let Some(cause) = err.source() {
        for (i, e) in std::iter::successors(Some(cause), |e| (*e).source()).enumerate() {
            eprintln!("   {}: {}", i, e);
        }
    }
    std::process::exit(1);
}

fn show_map_diff(fields: &Map<String, serde_json::Value>, indent: usize, succinct: bool) {
    for (field, diff) in fields.iter() {
        print!("{}", " ".repeat(indent * 2));
        if field == "error" {
            println!("{}", diff.as_str().unwrap().red());
            continue;
        }
        if let Some(lr) = diff.as_array() {
            let (left, right) = (&lr[0], &lr[1]);
            if succinct && (left.is_something() && !right.is_something()) {
                println!(
                    "{}: {} => {}",
                    field,
                    format!("{}", left).green(),
                    "<absent>".red().italic()
                );
            } else if succinct && (right.is_something() && !left.is_something()) {
                println!(
                    "{}: {} => {}",
                    field,
                    "<absent>".green().italic(),
                    format!("{}", right).red()
                );
            } else {
                println!(
                    "{}: {} => {}",
                    field,
                    format!("{}", left).green(),
                    format!("{}", right).red()
                );
            }
        } else if let Some(fields) = diff.as_object() {
            println!("{}:", field);
            show_map_diff(fields, indent + 1, succinct)
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let font_binary_a = std::fs::read(&cli.font1).expect("Couldn't open file");
    let font_binary_b = std::fs::read(&cli.font2).expect("Couldn't open file");

    let mut font_a = DFont::new(&font_binary_a);
    let mut font_b = DFont::new(&font_binary_b);
    if let Some(ref loc) = cli.location {
        let _hack = font_a.set_location(loc);
        let _hack = font_b.set_location(loc);
    } else if let Some(ref inst) = cli.instance {
        font_a.set_instance(inst).expect("Couldn't find instance");
        font_b.set_instance(inst).expect("Couldn't find instance");
    }
    let mut diff = Map::new();
    if cli.tables {
        let table_diff = table_diff(&font_a.fontref(), &font_b.fontref());
        if table_diff.is_something() {
            diff.insert("tables".into(), table_diff);
        }
    }
    if cli.glyphs {
        let glyph_diff = test_font_glyphs(&font_a, &font_b);
        if glyph_diff.is_something() {
            diff.insert("glyphs".into(), glyph_diff);
        }
    }
    if cli.words {
        let word_diff = test_font_words(&font_a, &font_b);
        diff.insert("words".into(), word_diff);
    }
    if cli.html {
        do_html(&cli, &font_a, &font_b, diff);
    }
    if cli.json {
        println!("{}", serde_json::to_string_pretty(&diff).expect("foo"));
        std::process::exit(0);
    }

    if diff.contains_key("tables") {
        for (table_name, diff) in diff["tables"].as_object().unwrap().iter() {
            if diff.is_something() {
                println!("\n# {}", table_name);
            }
            if let Some(lr) = diff.as_array() {
                let (left, right) = (&lr[0], &lr[1]);
                if cli.succinct && (left.is_something() && !right.is_something()) {
                    println!("Table was present in LHS but absent in RHS");
                } else if cli.succinct && (right.is_something() && !left.is_something()) {
                    println!("Table was present in RHS but absent in LHS");
                } else {
                    println!("LHS had: {}", left);
                    println!("RHS had: {}", right);
                }
            } else if let Some(fields) = diff.as_object() {
                show_map_diff(fields, 0, cli.succinct);
            } else {
                println!("Unexpected diff format: {}", diff);
            }
        }
    }
    if diff.contains_key("glyphs") {
        println!("\n# Glyphs");
        let display_glyph = |glyph: &serde_json::Value| {
            println!(
                "  - {} ({}: {}) {:.3}%",
                glyph["string"].as_str().unwrap(),
                glyph["unicode"].as_str().unwrap(),
                glyph["name"].as_str().unwrap(),
                glyph["percent"].as_f64().unwrap()
            );
        };
        let map = diff["glyphs"].as_object().unwrap();
        if map["missing"].is_something() {
            println!("\nMissing glyphs:");
            for glyph in map["missing"].as_array().unwrap() {
                display_glyph(glyph);
            }
        }
        if map["new"].is_something() {
            println!("\nNew glyphs:");
            for glyph in map["new"].as_array().unwrap() {
                display_glyph(glyph);
            }
        }
        if map["modified"].is_something() {
            println!("\nModified glyphs:");
            for glyph in map["modified"].as_array().unwrap() {
                display_glyph(glyph);
            }
        }
    }
    if diff.contains_key("words") {
        println!("# Words");
        let map = diff["words"].as_object().unwrap();
        for (script, script_diff) in map.iter() {
            println!("\n## {}", script);
            for difference in script_diff.as_array().unwrap().iter() {
                println!(
                    "  - {} ({:.3}%)",
                    difference["word"].as_str().unwrap(),
                    difference["percent"].as_f64().unwrap()
                );
            }
        }
    }
}

fn do_html(cli: &Cli, font_a: &DFont, font_b: &DFont, diff: Map<String, serde_json::Value>) -> ! {
    // Make output directory
    let output_dir = Path::new(&cli.output);
    if !output_dir.exists() {
        std::fs::create_dir(output_dir).expect("Couldn't create output directory");
    }

    // Copy old font to output/old-<existing name>
    let old_font = output_dir.join(format!(
        "old-{}",
        cli.font1.file_name().unwrap().to_str().unwrap()
    ));
    std::fs::copy(&cli.font1, &old_font).expect("Couldn't copy old font");
    let new_font = output_dir.join(format!(
        "new-{}",
        cli.font2.file_name().unwrap().to_str().unwrap()
    ));
    std::fs::copy(&cli.font2, &new_font).expect("Couldn't copy new font");

    let font_face_old = CSSFontFace::new(&old_font, "old", font_a);
    let font_face_new = CSSFontFace::new(&new_font, "new", font_b);
    let font_style_old = CSSFontStyle::new(font_a, Some("old"));
    let font_style_new = CSSFontStyle::new(font_b, Some("new"));
    let value = serde_json::to_value(&diff).unwrap_or_else(|e| {
        die("serializing diff", e);
    });
    let html = render_output(
        &value,
        font_face_old,
        font_face_new,
        font_style_old,
        font_style_new,
    )
    .unwrap_or_else(|err| die("rendering HTML", err));

    // Write output
    let output_file = output_dir.join("diffenator.html");
    println!("Writing output to {}", output_file.to_str().unwrap());
    std::fs::write(output_file, html).expect("Couldn't write output file");
    std::process::exit(0);
}
