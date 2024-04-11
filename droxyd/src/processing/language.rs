use crate::processing::IO_helper::file_to_vec;

//Takes a &str as an entry and returns the found language.
pub fn get_lang(src: &str) -> String {
    let mut lang: String = String::new();
    let i_l: usize;
    match src.find("lang") {
        Some(i) => i_l = i,
        _ => return "no language".to_string(),
    }
    let mut found: bool = false;
    for c in src[i_l..].chars() {
        if found {
            if c == '"' { return lang }
            lang.push(c);
        }
        else if c == '"' { found = true; }
    }
    lang
}

//Takes a language &str as an entry and returns if it is a valid ISO639-1 language.
pub fn is_valid_lang(src: &str) -> bool {
    let path = "/home/thegeekeuhhhhhhh/throw/DroXyd/droxyd/src/processing/sources/iso639-1.txt";
    let iso639_1: Vec<String> = file_to_vec(&path);
    match Some(iso639_1.iter().position(|l| l == src)) {
        None => return false,
        _ => return true,
    }
}
