use std::collections::BTreeMap;
//import get text function
//



pub fn TF(s: String) -> BTreeMap<String, usize> {
    //let mut words_occ: BTreeMap<&str, &usize> = BtTreeMap::new();
    let mut tf: BTreeMap<String, usize> = BTreeMap::new();
    for _ in 0..s.chars().count() {
        let mut curr_w: String = String::new();
        for c in s.chars() {
            let tmp = c as u8;
            if (tmp < 97 || tmp > 122) && (tmp < 65 || tmp > 90) { break; }
            else { curr_w.push(c); }
        }
        let occ = s.matches(&curr_w).count()/s.chars().count();
        //words_occ.entry(curr_w).or_insert(occ);
        tf.entry(curr_w).or_insert(occ);
    }
    //words_occ
    tf
}


/*pub fn IDF(s: String) {
    let tf: BTreeMap<String, usize> = TF(s);
    let (nb, sent) = get_sentences(s);
    
}*/
