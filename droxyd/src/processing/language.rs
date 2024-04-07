use crate::processing::IO_helper::file_to_vec;

pub fn get_lang(src: &str) -> String {
    let mut lang: String = String::new();
    let i_l: usize = src.find("lang").unwrap();
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

pub fn is_valid_lang(src: &str) -> bool {
    let path = "processing/iso639-1.txt";
    let iso639_1: Vec<String> = file_to_vec(&path);
    match Some(iso639_1.iter().position(|l| l == src)) {
        None => return false,
        _ => return true,
    }
}



