pub fn remove_str(text: String, pattern: String) -> String
{
    
    let len_t = text.chars().count();
    let len_p = pattern.chars().count();
    if len_t < 1 || len_p < 1 { panic!("remove_word: source or word incorrect") }
    let mut res = String::new();
    for mut i in 0..len_t{
        if text.as_bytes()[i] == pattern.as_bytes()[0] {
            let mut j = 0;
            while (j + i) < len_t || j < len_p {
                if j >= len_p { return res }
                if text.as_bytes()[j+i] != pattern.as_bytes()[j] { 
                    j = 0;
                    break }
                j += 1;
            }
            i += j;
        }
        else  { res.push(text.as_bytes()[i] as char); }
    }
    res
}



pub fn get_words(mut s:  String) -> Vec::<(usize, String)> {
    let (mut j, mut words) = (0, Vec::<(usize, String)>::new());
    while j < s.chars().count() {
        let mut curr_w: String = String::new();
        for c in s.chars() {
            let tmp = c as u8;
            if (tmp < 97 || tmp > 122) && (tmp < 65 || tmp > 90) {
                break;
            }
            else { curr_w.push(c); }
            j += 1;
        }
        words.push((s.matches(&curr_w).count(), curr_w));
        s = remove_str(s, curr_w);
    }
    words
}

