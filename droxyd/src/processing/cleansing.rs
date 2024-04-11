#![allow(non_snake_case)]

use std::collections::BTreeMap;
use crate::processing::extraction::get_sentences;

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
                    && (s.as_bytes()[c] < 48 || s.as_bytes()[c] > 57) 
                    { break; }
            else { curr_w.push(s.as_bytes()[c] as char); }
            j+=1;
        }
<<<<<<< HEAD:droxyd/src/processing/cleansing.rs
        words.push((s.matches(&curr_w).count(), curr_w.clone()));
        s = remove_str(s, curr_w);
=======
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
>>>>>>> scraper:scraper/src/processing/cleansing.rs
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


