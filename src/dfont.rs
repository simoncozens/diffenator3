use font_types::NameId;
use read_fonts::{tables::cmap::CmapSubtable, FontRef, TableProvider};
use skrifa::MetadataProvider;
use std::{collections::HashSet, path::Path};

pub struct DFont {
    pub backing: Vec<u8>,
    variations: String,
    suffix: String,
}

impl DFont {
    pub fn new<P: AsRef<Path>>(path: P, suffix: &str) -> Self {
        let backing: Vec<u8> = std::fs::read(path).expect("Couldn't open file");
        DFont {
            backing,
            suffix: suffix.to_string(),
            variations: "".to_string(),
        }
    }

    pub fn fontref(&self) -> FontRef {
        FontRef::new(&self.backing).expect("Couldn't parse font")
    }
    pub fn family_name(&self) -> String {
        self.fontref()
            .localized_strings(NameId::FAMILY_NAME)
            .english_or_first()
            .map_or_else(|| "Unknown".to_string(), |s| s.chars().collect())
    }

    pub fn is_color(&self) -> bool {
        self.fontref()
            .table_directory
            .table_records()
            .iter()
            .any(|tr| {
                let tag = tr.tag();
                tag == "SVG " || tag == "COLR" || tag == "CBDT"
            })
    }

    pub fn is_variable(&self) -> bool {
        self.fontref()
            .table_directory
            .table_records()
            .iter()
            .any(|tr| tr.tag() == "fvar")
    }

    pub fn codepoints(&self) -> HashSet<u32> {
        let cmap = self.fontref().charmap();
        let mut points = HashSet::new();
        for (codepoint, _glyphid) in cmap.mappings() {
            points.insert(codepoint);
        }
        points
    }
}
