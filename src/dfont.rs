use font_types::NameId;
use read_fonts::FontRef;
use skrifa::{instance::Location, setting::VariationSetting, MetadataProvider};
use std::collections::HashSet;
use ucd::Codepoint;

pub struct DFont {
    pub backing: Vec<u8>,
    pub location: Location,
    pub codepoints: HashSet<u32>,
}

impl DFont {
    pub fn new(string: &[u8], variations: Option<&str>) -> Self {
        let backing: Vec<u8> = string.to_vec();

        let mut fnt = DFont {
            backing,
            codepoints: HashSet::new(),
            location: Location::default(),
        };
        let cmap = fnt.fontref().charmap();
        fnt.codepoints = cmap.mappings().map(|(cp, _)| cp).collect();
        if let Some(variations) = variations {
            fnt.location = fnt.parse_location(variations);
        }
        fnt
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

    fn parse_location(&self, variations: &str) -> Location {
        let mut settings: Vec<VariationSetting> = vec![];
        for variation in variations.split(',') {
            let mut parts = variation.split('=');
            let axis = parts.next().expect("No axis");
            let value = parts.next().expect("No value");
            let value = value.parse::<f32>().expect("Couldn't parse value");
            settings.push((axis, value).into());
        }
        self.fontref().axes().location(&settings)
    }

    pub fn supported_scripts(&self) -> HashSet<String> {
        let cmap = self.fontref().charmap();
        let mut strings = HashSet::new();
        for (codepoint, _glyphid) in cmap.mappings() {
            if let Some(script) = char::from_u32(codepoint).and_then(|c| c.script()) {
                strings.insert(format!("{:?}", script));
            }
        }
        strings
    }
}
