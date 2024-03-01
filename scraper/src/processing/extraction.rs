/*pub fn get_paragraphs(src: &str) -> String {
    let p_start: Vec<_> = src.match_indices("<p>").map(|(i, _)|i).collect();
    if p_start.len() <= 0 { panic!("get_patagraphs: no paragraphs detected in text") }
    let p_end: Vec<_> = src.match_indices("</p>").map(|(i, _)|i).collect();
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
*/

pub fn get_urls(src: &str) -> (bool, Vec<String>) {
    let href_indices: Vec<_> = src.match_indices("a href").map(|(i, _)|i).collect();
    if href_indices.len() <= 0 { return (false, Vec::new()) }
    let mut urls_vec: Vec<String> = vec![String::new(); href_indices.len()];
    let mut i_in_vec: usize = 0;
    for i in href_indices {
        let mut found: bool = false;
        for c in src[i..].chars() {
            if found {
                if c == '"' { break }
                urls_vec[i_in_vec].push(c);
            }
            else if c == '"' { found = true; }
        };
        i_in_vec += 1;
    }
    (true, urls_vec)
}

pub fn tag_extraction(src: &str) -> String {
    let tag_start: Vec<_> = src.match_indices("<").map(|(i, _)|i).collect();
    if tag_start.len() <= 0 { 
        println!("tag_extraction: no tags detected");
        return String::new()
    }
    let tag_end: Vec<_> = src.match_indices(">").map(|(i, _)|i).collect();
    let mut para = String::new();

    for i in 0..tag_start.len() {
        for c in src[tag_end[i]+1..].chars() {
            if c == '<' { break }
            para.push(c);
        }
    }
    para
}





