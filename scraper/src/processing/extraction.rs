pub fn get_paragraphs(src: &str) -> String {
    let p_start: Vec<_> = src.match_indices("<P>").map(|(i, _)|i).collect();
    if p_start.len() <= 0 { panic!("get_patagraphs: no paragraphs detected in text") }
    let p_end: Vec<_> = src.match_indices("</P>").map(|(i, _)|i).collect();
    let mut para = String::new();
    if p_start.len() == p_end.len() {
        for i in 0..p_start.len() {
            for p in src[p_start[i]+3..p_end[i]].chars() { para.push(p); }
            para.push('\n');
        }
    }
    else {
        for i in 1..p_start.len() {
            for p in src[p_start[i-1]+3..p_start[i]].chars() { para.push(p); }
            para.push('\n');
        }
    }
    para
}

/*pub fn rm_tags_paragraph(src: &str) -> String {
    let iso639_1: Vec<String> = [
*/

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

