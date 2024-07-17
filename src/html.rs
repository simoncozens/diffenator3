use serde::Serialize;
use serde_json::json;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};
use walkdir::WalkDir;

use crate::dfont::DFont;

#[derive(Debug, Serialize)]
pub struct CSSFontFace {
    suffix: String,
    filename: String,
    familyname: String,
    cssfamilyname: String,
    class_name: String,
    font_weight: String,
    font_width: String,
    font_style: String,
}

impl CSSFontFace {
    pub fn new(filename: &Path, suffix: &str, dfont: &DFont) -> Self {
        let familyname = suffix.to_string() + " " + &dfont.family_name();
        let cssfamilyname = familyname.clone();
        let class_name = cssfamilyname.replace(' ', "-");
        let font_weight = "normal".to_string();
        let font_width = "normal".to_string();
        let font_style = "normal".to_string();

        CSSFontFace {
            suffix: suffix.to_string(),
            filename: filename
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or_default()
                .to_string(),
            familyname,
            cssfamilyname,
            class_name,
            font_weight,
            font_width,
            font_style,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CSSFontStyle {
    suffix: String,
    coords: HashMap<String, f32>,
    familyname: String,
    style_name: String,
    cssfamilyname: String,
    class_name: String,
    font_variation_settings: String,
}

impl CSSFontStyle {
    pub fn new(dfont: &DFont, suffix: Option<&str>) -> Self {
        let familyname = dfont.family_name();
        let stylename = dfont.style_name();
        let coords = dfont
            .location
            .iter()
            .map(|setting| (setting.selector.clone().to_string(), setting.value))
            .collect();
        let font_variation_settings = dfont
            .location
            .iter()
            .map(|setting| format!("'{}' {}", setting.selector, setting.value))
            .collect::<Vec<String>>()
            .join(", ");
        let (cssfamilyname, class_name) = if let Some(suffix) = suffix {
            (
                format!("{} {}", suffix, familyname),
                format!("{}-{}", suffix, stylename).replace(' ', "-"),
            )
        } else {
            (familyname.to_string(), stylename.replace(' ', "-"))
        };

        CSSFontStyle {
            suffix: stylename.to_string(),
            familyname: familyname.to_string(),
            style_name: stylename.to_string(),
            cssfamilyname,
            class_name,
            coords,
            font_variation_settings,
        }
    }
}

fn template_engine() -> Tera {
    let homedir = create_user_home_templates_directory();
    Tera::new(&format!("{}/*", homedir.to_str().unwrap())).unwrap_or_else(|e| {
        println!("Problem parsing templates: {:?}", e);
        std::process::exit(1)
    })
}

pub fn create_user_home_templates_directory() -> PathBuf {
    let home = homedir::my_home()
        .expect("Couldn't got home directory")
        .expect("No home directory found");
    let templates_dir = home.join(".diffenator3/templates");
    if !templates_dir.exists() {
        std::fs::create_dir_all(&templates_dir)
            .expect(format!("Couldn't create {}", templates_dir.to_str().unwrap()).as_str());
    }
    let all_templates = [
        ["_base.html", include_str!("templates/_base.html")],
        [
            "CSSFontFace.partial.html",
            include_str!("templates/CSSFontFace.partial.html"),
        ],
        [
            "CSSFontStyle.partial.html",
            include_str!("templates/CSSFontStyle.partial.html"),
        ],
        [
            "Glyph.partial.html",
            include_str!("templates/Glyph.partial.html"),
        ],
        [
            "GlyphDiff.partial.html",
            include_str!("templates/GlyphDiff.partial.html"),
        ],
        [
            "Word.partial.html",
            include_str!("templates/Word.partial.html"),
        ],
        [
            "WordDiff.partial.html",
            include_str!("templates/WordDiff.partial.html"),
        ],
        ["script.js", include_str!("templates/script.js")],
        ["style.css", include_str!("templates/style.css")],
        ["diffenator.html", include_str!("templates/diffenator.html")],
    ];
    for template in all_templates.iter() {
        let path = templates_dir.join(template[0]);
        if !path.exists() {
            std::fs::write(&path, template[1]).expect(&format!(
                "Couldn't write template file {}",
                path.to_str().unwrap()
            ));
        }
    }
    templates_dir
}

pub fn render_output(
    value: &serde_json::Value,
    font_face_old: CSSFontFace,
    font_face_new: CSSFontFace,
    font_style_old: CSSFontStyle,
    font_style_new: CSSFontStyle,
    user_templates: Option<&String>,
) -> Result<String, tera::Error> {
    let mut tera = template_engine();
    if let Some(template_dir) = user_templates {
        for entry in WalkDir::new(template_dir) {
            if entry.as_ref().is_ok_and(|e| e.file_type().is_dir()) {
                continue;
            }
            let path = entry.as_ref().unwrap().path();
            if let Err(e) =
                tera.add_template_file(path, path.strip_prefix(template_dir).unwrap().to_str())
            {
                println!("Problem adding template file: {:?}", e);
                std::process::exit(1)
            }
        }
        if let Err(e) = tera.build_inheritance_chains() {
            println!("Problem building inheritance chains: {:?}", e);
            std::process::exit(1)
        }
    }

    tera.render(
        "diffenator.html",
        &Context::from_serialize(json!({
            "diff": {
                "tables": value.get("tables").unwrap_or(&json!({})),
                "glyphs": value.get("glyphs").unwrap_or(&serde_json::Value::Null),
                "words": value.get("words").unwrap_or(&serde_json::Value::Null),
            },
            "font_faces_old": [font_face_old],
            "font_faces_new": [font_face_new],
            "font_faces": [],
            "font_styles_old": [font_style_old],
            "font_styles_new": [font_style_new],
            "font_styles": [],
            "pt_size": 40,
            "include_ui": true,
        }))?,
    )
}
