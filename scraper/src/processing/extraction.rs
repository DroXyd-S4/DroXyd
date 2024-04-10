
//takes a HTML source code as an entry and returns all the detectable URLs in it.
pub fn get_urls(src: &str) -> Vec<String> {
    let href_indices: Vec<_> = src.match_indices("a href").map(|(i, _)| i).collect();
    if href_indices.len() <= 0 { 
        println!("urls returned cause len = 0");
        return Vec::new() }
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
    urls_vec
}

//takes a HTML source code as an entry and returns all the text/paragraphs found.
pub fn tag_extraction(src: &str) -> String {
    let tag_start: Vec<_> = src.match_indices("<").map(|(i, _)|i).collect();
    if tag_start.len() <= 0 { 
        println!("tag_extraction: no tags detected");
        return String::new()
    }
    let tag_end: Vec<_> = src.match_indices(">").map(|(i, _)|i).collect();
    let mut para = String::new();
    println!("{:?}", tag_end.len());
    if tag_start.len() != tag_end.len() { 
        println!("not same len");
        return "not same length".to_string() }
    for i in 0..tag_start.len() {
        println!("{}", tag_start[i]);
        println!("{}\n", tag_end[i]+1);
        for c in src[tag_end[i]+1..].chars() {
            if c == '<' { break }
            para.push(c);
        }
    }
    para
}

//takes a text as an entry and returns a tuple consisiting of a Vec<String> containing 
//all the sentences and the number of sentences in a usize.
pub fn get_sentences(src: &str) -> (Vec<String>, usize) {
    let mut sent: Vec<String> = Vec::new();
    let mut nb: usize = 0;
    let mut curr_sent = String::new();
    for i in src.chars() {
        curr_sent.push(i); 
        if i == '.' { 
            nb += 1;
            sent.push(curr_sent);
            curr_sent = String::new();
        }
    }
    (sent, nb)
}


