#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, Read, Write};
//use std::io::prelude::*;
use std::path::Path;
use std::net::TcpStream;


pub fn get_filename(file_path: &str) {
    let path = Path::new(file_path);
    let filename = path.file_name().unwrap();
    println!("{}", filename.to_str().unwrap());
}

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

fn IMCP_ping() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("209.85.233.101:80")?;
    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}

pub fn URL_to_String(url: &str) -> Option<String> {
    match IMCP_ping() {
        Ok(_) => println!("Connexion detected"),
        Err(_) => println!("URL_to_String: no ping detected")
    };
    let body = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    Some(body)
}
