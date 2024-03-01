use std::fs::File;
use std::io::{BufReader, Read};
use hyper::Client;

pub fn file_to_str(file_path: &str) -> String
{ 
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("file_to_str: couldn't open {}: {}", file_path, why),
        Ok(file) => BufReader::new(file),
    };
    let mut s = String::new();
    file.read_to_string(&mut s).expect("file_to_str: couldn't read from file");
    s
}

pub fn file_to_vec(file_path: &str) -> Vec<String> {
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("file_to_vec: couldn't open {}: {}", file_path, why),
        Ok(file) => BufReader::new(file),
    };
    let mut s = String::new();
    file.read_to_string(&mut s).expect("file_to_vec: couldn't read from file");
    let mut v: Vec<String> = Vec::new();
    for w in s.lines() 
    {
        let mut found = false;
        let mut tmp = String::new();
        for i in w.chars() {
            if found {
                if i == '"' { break}
                tmp.push(i);
            }
            else if i == '"' { found = true;}
        }
        v.push(tmp);
    }
    v
}






fn URL_to_String(src: &str) {
    let client = Client::new();
    let mut res = client.get("http://www.bloomberg.com/")
        .send()
        .unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).expect("failed to read into string");
    body
}
