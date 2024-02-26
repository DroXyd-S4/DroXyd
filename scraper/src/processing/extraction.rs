pub fn extraction(src: String) -> String {
    String::new()


}

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

/*pub fn is_valid_lang(src: &str) -> bool {
    
}*/


pub fn get_urls(src: &str) -> (bool, Vec<String>) {
    
    let href_indices: Vec<_> = src.match_indices("href").map(|(i, _)|i).collect();
    if href_indices.len() <= 0 { return (false, Vec::new()) }
    let mut urls_vec: Vec<String> = vec![String::new(); href_indices.len()];
    let mut i_in_vec: usize = 0;
    for i in href_indices {
        let mut found: bool = false;
        //let mut url_tmp: String = String::new(); 
        for c in src[i..].chars() {
            if found {
                if c == '"' { break }
                urls_vec[i_in_vec].push(c);
            }
            else if c == '"' { found = true; }
        };
        //urls_vec[i_in_vec] = url_tmp;
        i_in_vec += 1;
    }
    (true, urls_vec)
}


pub fn valid_urls(urls: (bool, Vec<String>)) -> Vec<String> {
    if !urls.0 { panic!("valid_urls: no urls detected") }
    let mut valid_urls: Vec<String> = Vec::new();
    for i in urls.1 {
        match i.find("http") {
            Some(_) => valid_urls.push(i),
            None => continue,
        }
    }
    valid_urls
}

