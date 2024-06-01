#![allow(non_snake_case)]

use std::collections::BTreeMap;
use crate::processing::extraction::get_sentences;
use crate::processing::IO_helper::file_to_vec;

//Takes a string containing all the texts as a paramater
//Returns a BTreeMap<String, usize> containing the keyword and its TF-IDF value.
pub fn TF_IDF(s: String) -> BTreeMap<String, f64> {
    //let mut tf: BTreeMap<String, usize> = BTreeMap::new();
    //let mut idf: BTreeMap<String, usize> = BTreeMap::new();
    let mut tf_idf: BTreeMap<String, f64> = BTreeMap::new();
    let (sent, nb) = get_sentences(&s);

    let mut j = 0;
    while j < s.chars().count() {
        let mut curr_w: String = String::new();
        for c in j..s.chars().count() {
            if (s.as_bytes()[c] < 97 || s.as_bytes()[c] > 122)
                && (s.as_bytes()[c] < 65 || s.as_bytes()[c] > 90) 
                    { break; }
            else { curr_w.push(s.as_bytes()[c] as char); }
            j+=1;
        }
        if !(curr_w == " " || curr_w == "") {
            let tf_res = (s.matches(&curr_w).count() as f64)/(s.chars().count() as f64);
            //tf.entry(curr_w).or_insert(tf_res);
            let mut in_nb_sent: f64 = 0.0;
            for sentence in sent.iter() {
                if sentence.matches(&curr_w).count() > 0 { in_nb_sent += 1.0; }
            }
            let idf_res = (in_nb_sent.log(nb as f64)).max(0.0);
            //idf.entry(curr_w).or_insert(idf_res);

            /*Calculates the TF-IDF result of the current word and add it in 
              the TF-IDF BTreeMap if it isnt already in it. */
            tf_idf.entry(curr_w).or_insert(tf_res*idf_res);
        }
        j+=1;
    }
    tf_idf
}

//takes a TF-IDF BTreeMap as an entry and returns the keywords that 
//possess a TF-IDF under the TF-IDF BTreeMap average.
pub fn keywords(tf_idf: BTreeMap<String, f64>) -> Vec<String> {
    let mut average = 0.0;
    for (_, n) in &tf_idf { average += n; }
    average = average/(tf_idf.len() as f64);
    let mut keywords = Vec::new();
    for (curr_w, n) in tf_idf {
        if n <= average { keywords.push(curr_w) }
    }
    keywords
}

//takes a TF-IDF BTreeMap as an entry and returns the keywords that are more relevant to the website (via a threshhold value)
//as well as 3 relevant words to add in the database.
pub fn keywords2(tf_idf: BTreeMap<String, f64>, mut threshhold: usize) -> (Vec<String>, String, String, String) {
    let path = "src/processing/sources/fr-determinants.txt";
    let fr = file_to_vec(&path);
    let path = "src/processing/sources/en-determinants.txt";
    let en = file_to_vec(&path);

    let mut tmp = Vec::new();
    let mut res = Vec::new();
    for (w, n) in &tf_idf {
        if fr.contains(&w) || en.contains(&w) { continue; }
        else {tmp.push((n, w)); }
    }
    tmp.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //tmp.sort();
    let (mut word1, mut word2, mut word3) = (String::new(), String::new(), String::new());
    if threshhold >= tmp.len() { threshhold = tmp.len(); }
    match tmp.len() {
        l if l < threshhold => threshhold = tmp.len(),
        l if threshhold +1 > tmp.len() => {
            word1 = tmp[tmp.len() -1 - threshhold].1.clone();
        },
        l if threshhold + 2 > tmp.len() => {
            word1 = tmp[tmp.len() -1 - threshhold].1.clone();
            word2 = tmp[tmp.len() -2 - threshhold].1.clone();
        },
        _ => {
            word1 = tmp[tmp.len() -1 - threshhold].1.clone();
            word2 = tmp[tmp.len() -2 - threshhold].1.clone();
            word3 = tmp[tmp.len() -3 - threshhold].1.clone();
        },
    };
    for i in 0..threshhold {
        //let j = tmp.len() -1 -i;
        res.push(tmp[i].1.clone());
    }
    (res, word1, word2, word3)
}
