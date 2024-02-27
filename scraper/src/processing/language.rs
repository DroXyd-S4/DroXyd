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
    
}



