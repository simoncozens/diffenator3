use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;

macro_rules! include_script {
    ($var:ident, $path:literal ) => {
        lazy_static! {
            pub static ref $var: Vec<u8> = {
                let mut input: Vec<u8> = Vec::new();
                let compressed = include_bytes!($path);
                brotli::BrotliDecompress(&mut compressed.as_ref(), &mut input)
                    .expect("Could not decompress");
                input
            };
        }
    };
}

include_script!(ADLAM, "../../test-data/Adlam.txt.br");
include_script!(ARABIC, "../../test-data/Arabic.txt.br");
include_script!(ARMENIAN, "../../test-data/Armenian.txt.br");
include_script!(AVESTAN, "../../test-data/Avestan.txt.br");
include_script!(BENGALI, "../../test-data/Bengali.txt.br");
include_script!(BOPOMOFO, "../../test-data/Bopomofo.txt.br");
include_script!(
    CANADIAN_ABORIGINAL,
    "../../test-data/Canadian_Aboriginal.txt.br"
);
include_script!(CHAKMA, "../../test-data/Chakma.txt.br");
include_script!(CHEROKEE, "../../test-data/Cherokee.txt.br");
include_script!(COMMON, "../../test-data/Common.txt.br");
include_script!(CYRILLIC, "../../test-data/Cyrillic.txt.br");
include_script!(DEVANAGARI, "../../test-data/Devanagari.txt.br");
include_script!(ETHIOPIC, "../../test-data/Ethiopic.txt.br");
include_script!(GEORGIAN, "../../test-data/Georgian.txt.br");
include_script!(GRANTHA, "../../test-data/Grantha.txt.br");
include_script!(GREEK, "../../test-data/Greek.txt.br");
include_script!(GUJARATI, "../../test-data/Gujarati.txt.br");
include_script!(GURMUKHI, "../../test-data/Gurmukhi.txt.br");
include_script!(HEBREW, "../../test-data/Hebrew.txt.br");
include_script!(HIRAGANA, "../../test-data/Hiragana.txt.br");
include_script!(JAPANESE, "../../test-data/Japanese.txt.br");
include_script!(KANNADA, "../../test-data/Kannada.txt.br");
include_script!(KATAKANA, "../../test-data/Katakana.txt.br");
include_script!(KHMER, "../../test-data/Khmer.txt.br");
include_script!(LAO, "../../test-data/Lao.txt.br");
include_script!(LATIN, "../../test-data/Latin.txt.br");
include_script!(LISU, "../../test-data/Lisu.txt.br");
include_script!(MALAYALAM, "../../test-data/Malayalam.txt.br");
include_script!(MONGOLIAN, "../../test-data/Mongolian.txt.br");
include_script!(MYANMAR, "../../test-data/Myanmar.txt.br");
include_script!(OL_CHIKI, "../../test-data/Ol_Chiki.txt.br");
include_script!(ORIYA, "../../test-data/Oriya.txt.br");
include_script!(OSAGE, "../../test-data/Osage.txt.br");
include_script!(SINHALA, "../../test-data/Sinhala.txt.br");
include_script!(SYRIAC, "../../test-data/Syriac.txt.br");
include_script!(TAMIL, "../../test-data/Tamil.txt.br");
include_script!(TELUGU, "../../test-data/Telugu.txt.br");
include_script!(THAI, "../../test-data/Thai.txt.br");
include_script!(THANAA, "../../test-data/Thanaa.txt.br");
include_script!(TIBETAN, "../../test-data/Tibetan.txt.br");
include_script!(TIFINAGH, "../../test-data/Tifinagh.txt.br");
include_script!(VAI, "../../test-data/Vai.txt.br");

pub(crate) fn get_wordlist(script: &str) -> Option<Vec<String>> {
    let compressed = match script {
        "Adlam" => ADLAM.as_slice(),
        "Arabic" => ARABIC.as_slice(),
        "Armenian" => ARMENIAN.as_slice(),
        "Avestan" => AVESTAN.as_slice(),
        "Bengali" => BENGALI.as_slice(),
        "Bopomofo" => BOPOMOFO.as_slice(),
        "Canadian_Aboriginal" => CANADIAN_ABORIGINAL.as_slice(),
        "Chakma" => CHAKMA.as_slice(),
        "Cherokee" => CHEROKEE.as_slice(),
        "Common" => COMMON.as_slice(),
        "Cyrillic" => CYRILLIC.as_slice(),
        "Devanagari" => DEVANAGARI.as_slice(),
        "Ethiopic" => ETHIOPIC.as_slice(),
        "Georgian" => GEORGIAN.as_slice(),
        "Grantha" => GRANTHA.as_slice(),
        "Greek" => GREEK.as_slice(),
        "Gujarati" => GUJARATI.as_slice(),
        "Gurmukhi" => GURMUKHI.as_slice(),
        "Hebrew" => HEBREW.as_slice(),
        "Hiragana" => HIRAGANA.as_slice(),
        "Japanese" => JAPANESE.as_slice(),
        "Kannada" => KANNADA.as_slice(),
        "Katakana" => KATAKANA.as_slice(),
        "Khmer" => KHMER.as_slice(),
        "Lao" => LAO.as_slice(),
        "Latin" => LATIN.as_slice(),
        "Lisu" => LISU.as_slice(),
        "Malayalam" => MALAYALAM.as_slice(),
        "Mongolian" => MONGOLIAN.as_slice(),
        "Myanmar" => MYANMAR.as_slice(),
        "Ol_Chiki" => OL_CHIKI.as_slice(),
        "Oriya" => ORIYA.as_slice(),
        "Osage" => OSAGE.as_slice(),
        "Sinhala" => SINHALA.as_slice(),
        "Syriac" => SYRIAC.as_slice(),
        "Tamil" => TAMIL.as_slice(),
        "Telugu" => TELUGU.as_slice(),
        "Thai" => THAI.as_slice(),
        "Thanaa" => THANAA.as_slice(),
        "Tibetan" => TIBETAN.as_slice(),
        "Tifinagh" => TIFINAGH.as_slice(),
        "Vai" => VAI.as_slice(),

        _ => return None,
    };
    let buf = BufReader::new(compressed);
    let wordlist: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    Some(wordlist)
}


pub(crate) fn get_script_tag(script: &str) -> Option<rustybuzz::Script> {
    match script {
        "Adlam" => Some(rustybuzz::script::ADLAM),
        "Arabic" => Some(rustybuzz::script::ARABIC),
        "Armenian" => Some(rustybuzz::script::ARMENIAN),
        "Avestan" => Some(rustybuzz::script::AVESTAN),
        "Bengali" => Some(rustybuzz::script::BENGALI),
        "Bopomofo" => Some(rustybuzz::script::BOPOMOFO),
        "Canadian_Aboriginal" => Some(rustybuzz::script::CANADIAN_SYLLABICS),
        "Chakma" => Some(rustybuzz::script::CHAKMA),
        "Cherokee" => Some(rustybuzz::script::CHEROKEE),
        "Common" => Some(rustybuzz::script::COMMON),
        "Cyrillic" => Some(rustybuzz::script::CYRILLIC),
        "Devanagari" => Some(rustybuzz::script::DEVANAGARI),
        "Ethiopic" => Some(rustybuzz::script::ETHIOPIC),
        "Georgian" => Some(rustybuzz::script::GEORGIAN),
        "Grantha" => Some(rustybuzz::script::GRANTHA),
        "Greek" => Some(rustybuzz::script::GREEK),
        "Gujarati" => Some(rustybuzz::script::GUJARATI),
        "Gurmukhi" => Some(rustybuzz::script::GURMUKHI),
        "Hebrew" => Some(rustybuzz::script::HEBREW),
        "Hiragana" => Some(rustybuzz::script::HIRAGANA),
        "Kannada" => Some(rustybuzz::script::KANNADA),
        "Katakana" => Some(rustybuzz::script::KATAKANA),
        "Khmer" => Some(rustybuzz::script::KHMER),
        "Lao" => Some(rustybuzz::script::LAO),
        "Latin" => Some(rustybuzz::script::LATIN),
        "Lisu" => Some(rustybuzz::script::LISU),
        "Malayalam" => Some(rustybuzz::script::MALAYALAM),
        "Mongolian" => Some(rustybuzz::script::MONGOLIAN),
        "Myanmar" => Some(rustybuzz::script::MYANMAR),
        "Ol_Chiki" => Some(rustybuzz::script::OL_CHIKI),
        "Oriya" => Some(rustybuzz::script::ORIYA),
        "Osage" => Some(rustybuzz::script::OSAGE),
        "Sinhala" => Some(rustybuzz::script::SINHALA),
        "Syriac" => Some(rustybuzz::script::SYRIAC),
        "Tamil" => Some(rustybuzz::script::TAMIL),
        "Telugu" => Some(rustybuzz::script::TELUGU),
        "Thai" => Some(rustybuzz::script::THAI),
        "Tibetan" => Some(rustybuzz::script::TIBETAN),
        "Tifinagh" => Some(rustybuzz::script::TIFINAGH),
        "Vai" => Some(rustybuzz::script::VAI),
        _ =>  None,
    }
}

pub(crate) fn get_script_direction(script: &str) -> rustybuzz::Direction {
    match script {
        "Arabic" => rustybuzz::Direction::RightToLeft,
        "Avestan" => rustybuzz::Direction::RightToLeft,
        "Hebrew" => rustybuzz::Direction::RightToLeft,
        "Syriac" => rustybuzz::Direction::RightToLeft,
        "Thanaa" => rustybuzz::Direction::RightToLeft,
        _ => rustybuzz::Direction::LeftToRight,
    }
}